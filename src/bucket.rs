extern crate roaring;
use roaring::RoaringBitmap;

use std::slice::IterMut;
use std::cmp::Ordering;
use std::sync::{Mutex, LockResult, MutexGuard};

use errs::Error;
use token::Token;
use column::{Column, ColumnBuilder, ColumnRef};
use value::{Value, ValueStore};
use matches::{Match, MatchResults};
use pattern::Pattern;
use index::Index;

pub struct Bucket<'b> {
    write_lock: Mutex<bool>,
    token: Token,
    columns: Vec<Column>,
    indices: Vec<Index<'b>>,
    deleted: RoaringBitmap<usize>,
    values: ValueStore<'b>,
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
                                               ColumnBuilder::Boolean => Column::Boolean,
                                               ColumnBuilder::Str => Column::Str,
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
        };
        for col in &b.columns {
            b.indices.push(Index::new_by_column(col));
        }
        Ok(b)
    }

    pub fn write(&mut self) -> LockResult<MutexGuard<bool>> {
        self.write_lock.lock()
    }

    fn insert(&mut self, vals: Vec<Value<'b>>) -> Result<(), Error> {
        try!(validate_insert_value(&self.columns, &vals));
        try!(self.values.insert(&vals));
        let cur_id = self.values.next_id() - 1;
        for index_and_val in self.indices.iter_mut().zip(vals.iter()) {
            let (i, v) = index_and_val;
            i.insert(v, cur_id);
        }
        Ok(())
    }

    fn get_by_ids(&self, ids: &[usize]) -> MatchResults {
        let mut out: Vec<&[Value]> = Vec::new();
        let w = self.values.width();
        // println!("row width is {}", w);
        for id in ids.iter() {
            // println!("id {} is in the result", id);
            out.push(self.values.slice_at(id * w, id * w + w));
        }
        MatchResults { data: out }
    }

    fn delete_by_ids(&mut self, ids: &[usize]) -> usize {
        let mut c = 0_usize;
        for id in ids.iter() {
            self.deleted.insert(*id);
            c += 1;
        }
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
            if let Some(t) = idx.get_matching_index(match_) {
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

        let mut init = indices_to_match[0].clone();
        let mut matches: RoaringBitmap<usize> = indices_to_match.iter()
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

    fn find<'a>(&self, matches: &[Match<'a>]) -> Result<Option<MatchResults>, Error> {
        if let Ok(Some(ref ids)) = self.find_id(matches) {
            Ok(Some(self.get_by_ids(ids)))
        } else {
            Ok(None)
        }
    }

    fn delete<'a>(&mut self, matches: &[Match<'a>]) -> Result<usize, Error> {
        if let Ok(Some(ref ids)) = self.find_id(matches) {
            Ok(self.delete_by_ids(ids))
        } else {
            Ok(0)
        }
    }

    fn get_column_ref(&'b self, col_num: usize) -> Option<ColumnRef<'b>> {
        if col_num < self.columns.len() {
            Some(ColumnRef {
                id: col_num,
                t: self.token,
                r: &(self.columns[col_num]),
            })
        } else {
            None
        }
    }

    fn find_pattern_internal<'a>(&self,
                                 pattern: &Pattern<'a>)
                                 -> Result<RoaringBitmap<usize>, Error> {
        match *pattern {
            Pattern::Single(refcr, refm) => {
                let &ColumnRef { id: col_id, t: token, r: refcol } = refcr;
                if self.token != token || col_id >= self.columns.len() {
                    return Err(Error::InvalidColumn);
                }
                // should ref a column in this bucket
                let mut found = false;
                for refcol_ in self.columns.iter() {
                    if refcol_ as *const Column == refcol as *const Column {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(Error::InvalidColumn);
                }
                // column and match type should match
                try!(single_pattern_type_match(refcol, refm));
                if let Some(b) = self.indices[col_id].get_matching_index(refm) {
                    Ok(b.clone())
                } else {
                    Ok(RoaringBitmap::new())
                }
            }
            Pattern::And(ref left, ref right) => {
                match (self.find_pattern_internal(left), self.find_pattern_internal(right)) {
                    (Ok(bl), Ok(br)) => Ok(bl & br),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
            Pattern::Or(ref left, ref right) => {
                match (self.find_pattern_internal(left), self.find_pattern_internal(right)) {
                    (Ok(bl), Ok(br)) => Ok(bl | br),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
        }
    }

    fn find_pattern<'a>(&self, pattern: &Pattern<'a>) -> Result<Option<Vec<usize>>, Error> {
        match self.find_pattern_internal(pattern) {
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
        (&Column::Boolean, &Value::Boolean(_)) => true,
        (&Column::Str, &Value::Str(_)) => true,
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
        (&Column::Boolean, &Match::Boolean(_)) => true,
        (&Column::Str, &Match::Str(_)) => true,
        (_, &Match::Any) => true,
        _ => false,
    }
}

fn validate_find_simple_pattern(cols: &Vec<Column>, pattern: &[Match]) -> Result<(), Error> {
    if cols.len() != pattern.len() {
        return Err(Error::WrongNumberOfMatches(cols.len(), pattern.len()));
    }
    for (i, col) in cols.iter().enumerate() {
        if !match_simple_type_eq(&col, &pattern[i]) {
            return Err(Error::WrongMatchType(i));
        }
    }
    Ok(())
}

fn single_pattern_type_match<'a>(refcol: &Column, refm: &Match<'a>) -> Result<(), Error> {
    match (refcol, refm) {
        (&Column::UInt, &Match::UInt(_)) => Ok(()),
        (&Column::Boolean, &Match::Boolean(_)) => Ok(()),
        (&Column::Str, &Match::Str(_)) => Ok(()),
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

    pub fn find<'c>(&self, matches: &[Match<'c>]) -> Result<Option<MatchResults>, Error> {
        self.b.find(matches)
    }

    pub fn find_pattern<'c>(&self, pattern: &Pattern<'c>) -> Result<Option<MatchResults>, Error> {
        match self.b.find_pattern(pattern) {
            Ok(Some(ref ids)) => Ok(Some(self.b.get_by_ids(ids))),
            _ => Ok(None),
        }
    }
}

pub struct WriteHandle<'a, 'b: 'a> {
    b: &'a mut Bucket<'b>,
}

impl<'a, 'b: 'a> WriteHandle<'a, 'b> {
    pub fn new(refb: &'a mut Bucket<'b>) -> Self {
        WriteHandle { b: refb }
    } 

    pub fn find<'c>(&self, matches: &[Match<'c>]) -> Result<Option<MatchResults>, Error> {
        self.b.find(matches)
    }

    pub fn insert(&mut self, vals: Vec<Value<'b>>) -> Result<(), Error> {
        self.b.insert(vals)
    }
    
    pub fn delete<'c>(&mut self, matches: &[Match<'c>]) -> Result<usize, Error> {
        self.b.delete(matches)
    }

    // pub fn delete_pattern<'a>(&self,
    //                           pattern: &Pattern<'a>)
    //                           -> Result<usize, Error> {
    //     unimplemented!()
    // }
}

pub struct BucketBuilder<'bb> {
    pub name: &'bb str,
    pub columns: Vec<ColumnBuilder>,
}

impl<'bb> BucketBuilder<'bb> {
    pub fn new(name: &'bb str) -> Self {
        BucketBuilder {
            name: name,
            columns: Vec::new(),
        }
    }

    pub fn add_column(mut self, col: ColumnBuilder) -> Self {
        self.columns.push(col);
        self
    }
}
