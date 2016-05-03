extern crate roaring;
use roaring::RoaringBitmap;

use std::fmt;
use std::ops::Deref;
use std::slice::IterMut;
use std::sync::{Mutex, LockResult, MutexGuard};

use errs::Error;
use token::Token;
use column::{Column, ColumnBuilder, ColumnRef};
use value::{Value, ValueStore};
use matches::{Match, MatchResults};
use pattern::Pattern;
use index::{Index, IndexStats};

#[derive(Debug)]
pub struct BucketStats {
    pub columns: usize,
    pub inserts: usize,
    pub deletes: usize,
    pub rows: usize,
    pub index_stats: Vec<IndexStats>,
}

impl fmt::Display for BucketStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "-----------------------------------"));
        try!(writeln!(f, "columns: {:>}", self.columns));
        try!(writeln!(f, "inserts: {:>}", self.inserts));
        try!(writeln!(f, "deletes: {:>}", self.deletes));
        try!(writeln!(f, "rows   : {:>}", self.rows));
        try!(writeln!(f, "---------------Indices-------------"));
        for i in self.index_stats.iter() {
            try!(write!(f, "{}", i));
        }
        write!(f, "")
    }
}

pub struct Bucket<'b> {
    write_lock: Mutex<bool>,
    token: Token,
    columns: Vec<Column>,
    indices: Vec<Index<'b>>,
    deleted: RoaringBitmap<usize>,
    values: ValueStore<'b>,
    stats: BucketStats,
}

impl<'b> Bucket<'b> {
    pub fn new(cols: Vec<ColumnBuilder>) -> Result<Self, Error> {
        let l = cols.len();
        if l == 0 {
            return Err(Error::NoColumn);
        }
        let col_vec: Vec<Column> = cols.into_iter()
                                       .map(|cb| {
                                           match cb {
                                               ColumnBuilder::UInt => Column::UInt,
                                               ColumnBuilder::Int => Column::Int,
                                               ColumnBuilder::Boolean => Column::Boolean,
                                               ColumnBuilder::Str => Column::Str,
                                               ColumnBuilder::OwnedStr => Column::OwnedStr,
                                           }
                                       })
                                       .collect();
        let mut b = Bucket {
            write_lock: Mutex::new(true),
            token: Token::new(),
            columns: col_vec,
            indices: Vec::new(),
            deleted: RoaringBitmap::new(),
            values: ValueStore::new(l),
            stats: BucketStats {
                columns: l,
                inserts: 0,
                deletes: 0,
                rows: 0,
                index_stats: Vec::new(),
            },
        };
        for col in &b.columns {
            b.indices.push(Index::new_by_column(col));
        }
        Ok(b)
    }

    pub fn rows(&self) -> usize {
        let all: RoaringBitmap<usize> = (0..self.values.rows()).collect();
        (all ^ &self.deleted).len()
    }

    pub fn stats(&self) -> BucketStats {
        let mut is = Vec::<IndexStats>::new();
        for i in self.indices.iter() {
            is.push(i.stats());
        }
        BucketStats {
            columns: self.stats.columns,
            inserts: self.stats.inserts,
            deletes: self.stats.deletes,
            rows: self.stats.inserts - self.stats.deletes,
            index_stats: is,
        }
    }

    pub fn write(&mut self) -> LockResult<MutexGuard<bool>> {
        self.write_lock.lock()
    }

    pub fn find<'a>(&self, matches: &[Match<'a>]) -> Result<Option<MatchResults>, Error> {
        let found = try!(self.find_id(matches));
        if let Some(ref ids) = found {
            Ok(Some(self.get_by_ids(ids)))
        } else {
            Ok(None)
        }
    }

    pub fn get_column_ref(&self, col_num: usize) -> Option<ColumnRef> {
        if col_num < self.columns.len() {
            Some(ColumnRef {
                id: col_num,
                t: self.token,
                r: self.columns[col_num].clone(),
            })
        } else {
            None
        }
    }

    pub fn find_pattern<'c>(&self, pattern: &Pattern<'c>) -> Result<Option<MatchResults>, Error> {
        match self.find_pattern_internal(pattern) {
            Ok(Some(ref ids)) => Ok(Some(self.get_by_ids(ids))),
            Err(e) => Err(e),
            _ => Ok(None),
        }
    }

    fn insert(&mut self, vals: Vec<Value<'b>>) -> Result<(), Error> {
        try!(validate_insert_value(&self.columns, &vals));
        try!(self.values.insert(&vals));
        self.stats.inserts += 1;
        let cur_id = self.values.next_id() - 1;
        for index_and_val in self.indices.iter_mut().zip(vals.iter()) {
            let (i, v) = index_and_val;
            i.insert(v, cur_id);
        }
        Ok(())
    }

    fn insert_unique(&mut self, vals: Vec<Value<'b>>) -> Result<bool, Error> {
        try!(validate_insert_value(&self.columns, &vals));
        let ms: Vec<Match> = vals.iter()
                                 .map(|v| {
                                     match *v {
                                         Value::UInt(u) => Match::UInt(u),
                                         Value::Int(i) => Match::Int(i),
                                         Value::Boolean(b) => Match::Boolean(b),
                                         Value::Str(s) => Match::Str(s),
                                         Value::OwnedStr(ref s) => Match::OwnedStr(s.clone()),
                                     }
                                 })
                                 .collect();
        if let Ok(Some(_)) = self.find(&ms) {
            return Ok(false);
        }

        try!(self.values.insert(&vals));
        self.stats.inserts += 1;
        let cur_id = self.values.next_id() - 1;
        for index_and_val in self.indices.iter_mut().zip(vals.iter()) {
            let (i, v) = index_and_val;
            i.insert(v, cur_id);
        }
        Ok(true)
    }

    fn get_by_ids(&self, ids: &[usize]) -> MatchResults {
        let mut out: Vec<&[Value]> = Vec::new();
        let w = self.values.width();
        // println!("row width is {}", w);
        for id in ids.iter() {
            // println!("id {} is in the result", id);
            out.push(self.values.slice_at(id * w, id * w + w));
        }
        MatchResults::new(out)
    }

    fn delete_by_ids(&mut self, ids: &[usize]) -> usize {
        let mut c = 0_usize;
        for id in ids.iter() {
            self.deleted.insert(*id);
            c += 1;
        }
        self.stats.deletes += c;
        c
    }

    fn find_id<'a>(&self, matches: &[Match<'a>]) -> Result<Option<Vec<usize>>, Error> {
        try!(validate_find_simple_pattern(&self.columns, matches));
        let mut indices_to_match: Vec<&RoaringBitmap<usize>> = Vec::new();
        for index_and_match in self.indices.iter().zip(matches.iter()) {
            let (idx, match_) = index_and_match;
            if let &Match::Any = match_ {
                continue;
            }
            if let Some(t) = idx.get_match_index(match_) {
                indices_to_match.push(t);
            } else {
                return Ok(None);
            }
        }

        if indices_to_match.len() == 0 {
            return Ok(None);
        }

        indices_to_match.sort_by(|lhs, rhs| lhs.len().cmp(&rhs.len()));
        // println!("checking {} indices", indices_to_match.len());
        // for i in indices_to_match.iter() {
        //     println!("index has length {}", i.len());
        // }

        let init = indices_to_match[0].clone();
        let matches: RoaringBitmap<usize> = indices_to_match.iter()
                                                            .skip(1)
                                                            .fold(init, |acc, &i| acc & i);
        // println!("out length {}", matches.len());
        if matches.len() == 0 {
            return Ok(None);
        }
        // for i in indices_to_match.iter() {
        //     println!("index has length {}", i.len());
        // }

        let mut out: Vec<usize> = Vec::new();
        for id in matches.iter() {
            if !self.deleted.contains(id) {
                out.push(id);
            }
        }
        if out.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(out))
        }
    }

    fn delete<'a>(&mut self, matches: &[Match<'a>]) -> Result<usize, Error> {
        if let Ok(Some(ref ids)) = self.find_id(matches) {
            Ok(self.delete_by_ids(ids))
        } else {
            Ok(0)
        }
    }

    fn walk_pattern<'a>(&self, pattern: &Pattern<'a>) -> Result<RoaringBitmap<usize>, Error> {
        match *pattern {
            Pattern::Single(refcr, refv) => {
                let &ColumnRef { id: col_id, t: token, r: ref refcol } = refcr;
                if self.token != token || col_id >= self.columns.len() {
                    return Err(Error::InvalidColumnRef);
                }
                // column and match type should match
                try!(single_pattern_type_match(refcol, refv));
                if let Some(b) = self.indices[col_id].get_value_index(refv) {
                    Ok(b.clone())
                } else {
                    Ok(RoaringBitmap::new())
                }
            }
            Pattern::And(ref left, ref right) => {
                match (self.walk_pattern(left), self.walk_pattern(right)) {
                    (Ok(bl), Ok(br)) => Ok(bl & br),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
            Pattern::Or(ref left, ref right) => {
                match (self.walk_pattern(left), self.walk_pattern(right)) {
                    (Ok(bl), Ok(br)) => Ok(bl | br),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
        }
    }

    fn find_pattern_internal<'a>(&self,
                                 pattern: &Pattern<'a>)
                                 -> Result<Option<Vec<usize>>, Error> {
        match self.walk_pattern(pattern) {
            Ok(b) => {
                let mut out: Vec<usize> = Vec::new();
                for id in b.iter() {
                    if !self.deleted.contains(id) {
                        out.push(id);
                    }
                }
                if out.len() == 0 {
                    Ok(None)
                } else {
                    Ok(Some(out))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn index_iter_mut<'c>(&'c mut self) -> IterMut<'c, Index<'b>> {
        self.indices.iter_mut()
    }
}

fn value_type_eq(l: &Column, r: &Value) -> bool {
    match (l, r) {
        (&Column::UInt, &Value::UInt(_)) => true,
        (&Column::Int, &Value::Int(_)) => true,
        (&Column::Boolean, &Value::Boolean(_)) => true,
        (&Column::Str, &Value::Str(_)) => true,
        (&Column::OwnedStr, &Value::OwnedStr(_)) => true,
        _ => false,
    }
}

fn validate_insert_value(cols: &Vec<Column>, vals: &[Value]) -> Result<(), Error> {
    if cols.len() != vals.len() {
        return Err(Error::WrongNumberOfValues(cols.len(), vals.len()));
    }
    for (i, col) in cols.iter().enumerate() {
        if !value_type_eq(&col, &vals[i]) {
            return Err(Error::WrongValueType(i));
        }
    }
    Ok(())
}

fn match_simple_type_eq(l: &Column, r: &Match) -> bool {
    match (l, r) {
        (&Column::UInt, &Match::UInt(_)) => true,
        (&Column::Int, &Match::Int(_)) => true,
        (&Column::Boolean, &Match::Boolean(_)) => true,
        (&Column::Str, &Match::Str(_)) => true,
        (&Column::OwnedStr, &Match::OwnedStr(_)) => true,
        (_, &Match::Any) => true,
        _ => false,
    }
}

fn validate_find_simple_pattern(cols: &Vec<Column>, matches: &[Match]) -> Result<(), Error> {
    if cols.len() != matches.len() {
        return Err(Error::WrongNumberOfMatches(cols.len(), matches.len()));
    }
    if let None = matches.iter().find(|m| {
        match m {
            &&Match::Any => false,
            _ => true,
        }
    }) {
        return Err(Error::NothingToMatch);
    }
    for (i, col) in cols.iter().enumerate() {
        if !match_simple_type_eq(&col, &matches[i]) {
            return Err(Error::WrongMatchType(i));
        }
    }
    Ok(())
}

fn single_pattern_type_match<'a>(refcol: &Column, refv: &Value<'a>) -> Result<(), Error> {
    match (refcol, refv) {
        (&Column::UInt, &Value::UInt(_)) => Ok(()),
        (&Column::Int, &Value::Int(_)) => Ok(()),
        (&Column::Boolean, &Value::Boolean(_)) => Ok(()),
        (&Column::Str, &Value::Str(_)) => Ok(()),
        (&Column::OwnedStr, &Value::OwnedStr(_)) => Ok(()),
        _ => Err(Error::InvalidColumnMatch),
    }
}

pub struct ReadHandle<'a, 'b: 'a> {
    b: &'a Bucket<'b>,
}

impl<'a, 'b: 'a> ReadHandle<'a, 'b> {
    pub fn new(refb: &'a Bucket<'b>) -> Self {
        ReadHandle { b: refb }
    }
}

impl<'a, 'b: 'a> Deref for ReadHandle<'a, 'b> {
    type Target = Bucket<'b>;

    fn deref(&self) -> &Bucket<'b> {
        self.b
    }
}

pub struct WriteHandle<'a, 'b: 'a> {
    b: &'a mut Bucket<'b>,
}

impl<'a, 'b: 'a> WriteHandle<'a, 'b> {
    pub fn new(refb: &'a mut Bucket<'b>) -> Self {
        WriteHandle { b: refb }
    }

    pub fn insert(&mut self, vals: Vec<Value<'b>>) -> Result<(), Error> {
        self.b.insert(vals)
    }

    pub fn insert_unique(&mut self, vals: Vec<Value<'b>>) -> Result<bool, Error> {
        self.b.insert_unique(vals)
    }

    pub fn delete<'c>(&mut self, matches: &[Match<'c>]) -> Result<usize, Error> {
        self.b.delete(matches)
    }

    pub fn delete_pattern<'c>(&mut self, pattern: &Pattern<'c>) -> Result<usize, Error> {
        if let Ok(Some(ref ids)) = self.b.find_pattern_internal(pattern) {
            Ok(self.b.delete_by_ids(ids))
        } else {
            Ok(0)
        }
    }
}

impl<'a, 'b: 'a> Deref for WriteHandle<'a, 'b> {
    type Target = Bucket<'b>;

    fn deref(&self) -> &Bucket<'b> {
        self.b
    }
}

pub struct BucketBuilder {
    pub name: String,
    pub columns: Vec<ColumnBuilder>,
}

impl BucketBuilder {
    pub fn new<T: Into<String>>(name: T) -> Self {
        BucketBuilder {
            name: name.into(),
            columns: Vec::new(),
        }
    }

    pub fn add_column(mut self, col: ColumnBuilder) -> Self {
        self.columns.push(col);
        self
    }
}
