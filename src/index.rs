extern crate roaring;
use roaring::RoaringBitmap;

use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;

use value::{Column, Value};
use matches::{Match};

#[derive(Debug)]
pub enum Index<'a> {
    UInt(HashMap<usize, RoaringBitmap<usize>>),
    Boolean(HashMap<bool, RoaringBitmap<usize>>),
    Str(HashMap<&'a str, RoaringBitmap<usize>>)
}

impl<'a> Index<'a> {
    pub fn new_by_column(col: &Column) -> Index<'a> {
        match *col {
            Column::UInt => Index::UInt(HashMap::new()),
            Column::Boolean => Index::Boolean(HashMap::new()),
            Column::Str => Index::Str(HashMap::new()),
        }
    }

    pub fn insert(&mut self, val: &Value<'a>, id: usize) {
        match (self, val) {
            (&mut Index::UInt(ref mut m), &Value::UInt(u)) => {
                ensure_bitmap(m, u);
                if let Some(idx) = m.get_mut(&u) {
                    idx.insert(id);
                }
            },
            (&mut Index::Boolean(ref mut m), &Value::Boolean(tf)) => {
                ensure_bitmap(m, tf);
                if let Some(idx) = m.get_mut(&tf) {
                    idx.insert(id);
                }
            },
            (&mut Index::Str(ref mut m), &Value::Str(s)) => {
                ensure_bitmap(m, s);
                if let Some(idx) = m.get_mut(s) {
                    idx.insert(id);
                }
            },
            _ => unreachable!(),
        }
    }

    pub fn get_matching_index(&self, pattern: &Match) -> Option<&RoaringBitmap<usize>> {
        match (self, pattern) {
            (&Index::UInt(ref m), &Match::UInt(u)) => m.get(&u),
            (&Index::Boolean(ref m), &Match::Boolean(tf)) => m.get(&tf),
            (&Index::Str(ref m), &Match::Str(s)) => m.get(s),
            _ => unreachable!(),
        }
    }
}

fn ensure_bitmap<T: Eq + Hash>(m: &mut HashMap<T, RoaringBitmap<usize>>, key: T) {
    if let None = m.get(&key) {
        let idx: RoaringBitmap<usize> = RoaringBitmap::new();
        m.insert(key, idx);
    }
}
