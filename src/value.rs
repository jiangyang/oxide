use errs::Error;

#[derive(Clone, Copy, Debug)]
pub enum Value<'a> {
    UInt(usize),
    Boolean(bool),
    Str(&'a str),
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
        self.vals.extend(vals);
        self.next_id += 1;
        Ok(())
    }
}
