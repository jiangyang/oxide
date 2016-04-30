use token::Token;

#[derive(Debug)]
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
pub struct ColumnRef<'a> {
    pub id: usize,
    pub t: Token,
    pub r: &'a Column,
}
