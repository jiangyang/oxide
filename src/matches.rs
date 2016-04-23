use std::slice::Iter;

use value::{Value};

#[derive(Clone, Copy, Debug)]
pub enum Match<'a> {
    UInt(usize),
    Boolean(bool),
    Str(&'a str),
    Any,
}

#[derive(Debug)]
pub struct MatchResults<'a, 'b: 'a> {
    pub data: Vec<&'a [Value<'b>]>
}

impl<'a, 'b: 'a> MatchResults<'a, 'b> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn iter<'c>(&self) -> Iter<'c, &[Value<'b>]> {
        self.data.iter()
    }
}