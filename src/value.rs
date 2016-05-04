use std::convert;
use std::fmt;

use errs::Error;

#[derive(Clone, Debug)]
pub enum Value<'a> {
    UInt(usize),
    Int(isize),
    Boolean(bool),
    Str(&'a str),
    OwnedStr(String),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::UInt(u) => write!(f, "{}", u),
            &Value::Int(i) => write!(f, "{}", i),
            &Value::Boolean(b) => write!(f, "{}", b),
            &Value::Str(s) => write!(f, "{}", s),
            &Value::OwnedStr(ref s) => write!(f, "{}", s),
        }
    }
}

pub struct ValueStore<'v> {
    vals: Vec<Value<'v>>,
    width: usize,
    next_id: usize,
}

impl<'v> ValueStore<'v> {
    pub fn new(width: usize) -> Self {
        ValueStore {
            vals: Vec::new(),
            width: width,
            next_id: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn next_id(&self) -> usize {
        self.next_id
    }

    pub fn slice_at(&self, start: usize, end: usize) -> &[Value] {
        &self.vals[start..end]
    }

    pub fn insert(&mut self, vals: &Vec<Value<'v>>) -> Result<(), Error> {
        self.vals.extend(vals.iter().cloned());
        self.next_id += 1;
        Ok(())
    }

    pub fn rows(&self) -> usize {
        self.vals.len() / self.width
    }
}

impl<'a> convert::Into<Value<'a>> for usize {
    fn into(self) -> Value<'a> {
        Value::UInt(self)
    }
}

impl<'a> convert::Into<Value<'a>> for isize {
    fn into(self) -> Value<'a> {
        Value::Int(self)
    }
}

impl<'a> convert::Into<Value<'a>> for bool {
    fn into(self) -> Value<'a> {
        Value::Boolean(self)
    }
}

impl<'a> convert::Into<Value<'a>> for &'a str {
    fn into(self) -> Value<'a> {
        Value::Str(self)
    }
}

impl<'a> convert::Into<Value<'a>> for String {
    fn into(self) -> Value<'a> {
        Value::OwnedStr(self)
    }
}
