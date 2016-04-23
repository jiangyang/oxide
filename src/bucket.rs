extern crate roaring;
use roaring::RoaringBitmap;

use std::slice::IterMut;
use std::cmp::Ordering;

use errs::Error;
use value::{Column, Value, ValueStore};
use matches::{Match, MatchResults};
use index::{Index};

pub struct Bucket<'b> {
    columns: Vec<Column>,
    indices: Vec<Index<'b>>,
    deleted: RoaringBitmap<usize>,
    values: ValueStore<'b>,
}

impl<'b> Bucket<'b> {
    pub fn new(cols: Vec<Column>) -> Self {
        let l = cols.len();
        let mut b = Bucket {
            columns: cols,
            indices: Vec::new(),
            deleted: RoaringBitmap::new(),
            values: ValueStore::new(l),
        };
        for col in &b.columns {
            b.indices.push(Index::new_by_column(col));
        }
        b
    }

    pub fn insert(&mut self, vals: Vec<Value<'b>>) -> Result<(), Error> {
        try!(validate_insert_value(&self.columns, &vals));
        try!(self.values.insert(&vals));
        let cur_id = self.values.next_id() - 1;
        for index_and_val in self.indices.iter_mut().zip(vals.iter()) {
            let (i, v) = index_and_val;
            i.insert(v, cur_id);
        }
        Ok(())
    }

    pub fn get_by_ids(&self, ids: &[usize]) -> MatchResults {
        let mut out: Vec<&[Value]> = Vec::new();
        let w = self.values.width();
        // println!("row width is {}", w);
        for id in ids.iter() {
            // println!("id {} is in the result", id);
            out.push(self.values.slice_at(id * w, id * w + w));
        }
        MatchResults {
            data: out
        }
    }

    pub fn delete_by_ids(&mut self, ids: &[usize]) -> usize {
        let mut c = 0_usize;
        for id in ids.iter() {
            self.deleted.insert(*id);
            c += 1;
        }
        c
    }

    pub fn match_simple<'a>(&self, pattern: &[Match<'a>]) -> Result<Option<Vec<usize>>, Error> {
        try!(validate_find_pattern(&self.columns, pattern));
        let mut indices_to_match: Vec<&RoaringBitmap<usize>> = Vec::new();
        for index_and_match in self.indices.iter().zip(pattern.iter()) {
            let (idx, match_) = index_and_match;
            if let &Match::Any = match_ {
                continue;
            }
            if let Some(t) = idx.get_matching_index(match_) {
                indices_to_match.push(t);
            } else {
                return Ok(None)
            }
        }

        if indices_to_match.len() == 0 {
            return Ok(None)
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
            return Ok(None)
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

fn pattern_type_eq(l: &Column, r: &Match) -> bool {
    match (l, r) {
        (&Column::UInt, &Match::UInt(_)) => true,
        (&Column::Boolean, &Match::Boolean(_)) => true,
        (&Column::Str, &Match::Str(_)) => true,
        (_, &Match::Any) => true,
        _ => false,
    }
}

fn validate_find_pattern(cols: &Vec<Column>, pattern: &[Match]) -> Result<(), Error> {
    if cols.len() != pattern.len() {
        return Err(Error::WrongNumberOfMatches(cols.len(), pattern.len()));
    }
    for (i, col) in cols.iter().enumerate() {
        if !pattern_type_eq(&col, &pattern[i]) {
            return Err(Error::WrongMatchType(i));
        }
    }
    Ok(())
}

pub struct BucketBuilder<'bb> {
    pub name: &'bb str,
    pub columns: Vec<Column>
}

impl<'bb> BucketBuilder<'bb> {
    pub fn new(name: &'bb str) -> Self {
        BucketBuilder {
            name: name,
            columns: Vec::new()
        }
    }

    pub fn add_column(mut self, col: Column) -> Self {
        self.columns.push(col);
        self
    }
}