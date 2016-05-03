use std::slice::Iter;

use value::Value;

#[derive(Clone, Debug)]
pub enum Match<'a> {
    UInt(usize),
    Int(isize),
    Boolean(bool),
    Str(&'a str),
    OwnedStr(String),
    Any,
}

#[derive(Debug)]
pub struct MatchResults<'a, 'b: 'a> {
    data: Vec<&'a [Value<'b>]>,
}

impl<'a, 'b: 'a> MatchResults<'a, 'b> {
    pub fn new(d: Vec<&'a [Value<'b>]>) -> Self {
        MatchResults {
            data: d
        }
    }

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
