use token::Token;

#[derive(Debug, Clone)]
pub enum Column {
    UInt,
    Int,
    Boolean,
    Str,
    OwnedStr,
}

#[derive(Debug)]
pub enum ColumnBuilder {
    UInt,
    Int,
    Boolean,
    Str,
    OwnedStr,
}

#[derive(Debug)]
pub struct ColumnRef {
    pub id: usize,
    pub t: Token,
    pub r: Column,
}
