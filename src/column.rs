use token::Token;

#[derive(Debug)]
pub enum Column {
    UInt,
    Boolean,
    Str,
}

#[derive(Debug)]
pub enum ColumnBuilder {
    UInt,
    Boolean,
    Str,
}

#[derive(Debug)]
pub struct ColumnRef<'a> {
    pub id: usize,
    pub t: Token,
    pub r: &'a Column,
}
