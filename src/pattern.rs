use std::ops;

use column::ColumnRef;
use value::Value;

#[derive(Debug)]
pub enum Pattern<'a> {
    Single(&'a ColumnRef, &'a Value<'a>),
    And(Box<Pattern<'a>>, Box<Pattern<'a>>),
    Or(Box<Pattern<'a>>, Box<Pattern<'a>>),
}

impl<'a> Pattern<'a> {
    pub fn new(refc: &'a ColumnRef, refv: &'a Value<'a>) -> Pattern<'a> {
        Pattern::Single(refc, refv)
    }

    pub fn and(self, rhs: Pattern<'a>) -> Pattern<'a> {
        Pattern::And(Box::new(self), Box::new(rhs))
    }

    pub fn or(self, rhs: Pattern<'a>) -> Pattern<'a> {
        Pattern::Or(Box::new(self), Box::new(rhs))
    }
}

impl<'a> ops::BitAnd for Pattern<'a> {
    type Output = Pattern<'a>;

    fn bitand(self, rhs: Pattern<'a>) -> Pattern<'a> {
        Pattern::And(Box::new(self), Box::new(rhs))
    }
}

impl<'a> ops::BitOr for Pattern<'a> {
    type Output = Pattern<'a>;

    fn bitor(self, rhs: Pattern<'a>) -> Pattern<'a> {
        Pattern::Or(Box::new(self), Box::new(rhs))
    }
}
