extern crate fnv;
use fnv::FnvHasher;

extern crate roaring;
use roaring::RoaringBitmap;

use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt;
use std::hash::BuildHasherDefault;
use std::hash::Hash;

use column::Column;
use value::Value;
use matches::Match;

#[derive(Debug)]
pub struct IndexStats {
    pub cardinality: usize,
}

impl fmt::Display for IndexStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "cardinality: {}", self.cardinality)
    }
}

#[derive(Debug)]
pub enum Index<'a> {
    UInt(HashMap<usize, RoaringBitmap<usize>, BuildHasherDefault<FnvHasher>>),
    Int(HashMap<usize, RoaringBitmap<usize>, BuildHasherDefault<FnvHasher>>),
    Boolean(HashMap<bool, RoaringBitmap<usize>, BuildHasherDefault<FnvHasher>>),
    Str(HashMap<&'a str, RoaringBitmap<usize>, BuildHasherDefault<FnvHasher>>),
    OwnedStr(HashMap<String, RoaringBitmap<usize>, BuildHasherDefault<FnvHasher>>),
}

impl<'a> Index<'a> {
    pub fn new_by_column(col: &Column) -> Index<'a> {
        match *col {
            Column::UInt => Index::UInt(HashMap::default()),
            Column::Int => Index::Int(HashMap::default()),
            Column::Boolean => Index::Boolean(HashMap::default()),
            Column::Str => Index::Str(HashMap::default()),
            Column::OwnedStr => Index::OwnedStr(HashMap::default()),
        }
    }

    pub fn insert(&mut self, val: &Value<'a>, id: usize) {
        match (self, val) {
            (&mut Index::UInt(ref mut m), &Value::UInt(u)) => {
                ensure_bitmap(m, u);
                if let Some(idx) = m.get_mut(&u) {
                    idx.insert(id);
                }
            }
            (&mut Index::Int(ref mut m), &Value::Int(i)) => {
                let u = i as usize;
                ensure_bitmap(m, u);
                if let Some(idx) = m.get_mut(&u) {
                    idx.insert(id);
                }
            }
            (&mut Index::Boolean(ref mut m), &Value::Boolean(tf)) => {
                ensure_bitmap(m, tf);
                if let Some(idx) = m.get_mut(&tf) {
                    idx.insert(id);
                }
            }
            (&mut Index::Str(ref mut m), &Value::Str(s)) => {
                ensure_bitmap(m, s);
                if let Some(idx) = m.get_mut(s) {
                    idx.insert(id);
                }
            }
            (&mut Index::OwnedStr(ref mut m), &Value::OwnedStr(ref s)) => {
                ensure_bitmap(m, s.clone());
                if let Some(idx) = m.get_mut(s) {
                    idx.insert(id);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn get_match_index(&self, pattern: &Match) -> Option<&RoaringBitmap<usize>> {
        match (self, pattern) {
            (&Index::UInt(ref m), &Match::UInt(u)) => m.get(&u),
            (&Index::Int(ref m), &Match::Int(i)) => m.get(&(i as usize)),
            (&Index::Boolean(ref m), &Match::Boolean(tf)) => m.get(&tf),
            (&Index::Str(ref m), &Match::Str(s)) => m.get(s),
            (&Index::OwnedStr(ref m), &Match::OwnedStr(ref s)) => m.get(s),
            _ => unreachable!(),
        }
    }

    pub fn get_value_index<'b>(&self, pattern: &Value<'b>) -> Option<&RoaringBitmap<usize>> {
        match (self, pattern) {
            (&Index::UInt(ref m), &Value::UInt(u)) => m.get(&u),
            (&Index::Int(ref m), &Value::Int(i)) => m.get(&(i as usize)),
            (&Index::Boolean(ref m), &Value::Boolean(tf)) => m.get(&tf),
            (&Index::Str(ref m), &Value::Str(s)) => m.get(s),
            (&Index::OwnedStr(ref m), &Value::OwnedStr(ref s)) => m.get(s),
            _ => unreachable!(),
        }
    }

    pub fn stats(&self) -> IndexStats {
        let c = match self {
            &Index::UInt(ref m) => m.len(),
            &Index::Int(ref m) => m.len(),
            &Index::Boolean(ref m) => m.len(),
            &Index::Str(ref m) => m.len(),
            &Index::OwnedStr(ref m) => m.len(),
        };
        IndexStats { cardinality: c }
    }
}

fn ensure_bitmap<T: Eq + Hash>(m: &mut HashMap<T, RoaringBitmap<usize>, BuildHasherDefault<FnvHasher>>, key: T) {
    if let None = m.get(&key) {
        let idx: RoaringBitmap<usize> = RoaringBitmap::new();
        m.insert(key, idx);
    }
}
