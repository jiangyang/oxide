use column::{ColumnRef};
use matches::{Match};

#[derive(Debug)]
pub enum Pattern<'a> {
    Single(&'a ColumnRef<'a>, &'a Match<'a>),
    And(Box<Pattern<'a>>, Box<Pattern<'a>>),
    Or(Box<Pattern<'a>>, Box<Pattern<'a>>)
}

impl<'a> Pattern<'a> {
    pub fn new(refc: &'a ColumnRef<'a>, refm: &'a Match<'a>) -> Pattern<'a> {
        Pattern::Single(refc, refm)
    }
    
    pub fn and(self, rhs: Pattern<'a>) -> Pattern<'a>  {
        Pattern::And(Box::new(self), Box::new(rhs))
    }
    
    pub fn or(self, rhs: Pattern<'a>) -> Pattern<'a> {
        Pattern::Or(Box::new(self), Box::new(rhs))
    }
}