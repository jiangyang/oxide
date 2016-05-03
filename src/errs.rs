use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoColumn,
    InvalidBucket,
    InvalidColumnRef,
    InvalidColumnMatch,
    WrongNumberOfValues(usize, usize),
    WrongValueType(usize),
    WrongNumberOfMatches(usize, usize),
    WrongMatchType(usize),
    NothingToMatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoColumn => {
                write!(f, "bucket has no column defined.")
            },
            Error::InvalidBucket => { 
                write!(f, "bucket does not exist.")
            },
            Error::InvalidColumnRef => {
                write!(f, "column ref is not valid.")
            },
            Error::InvalidColumnMatch => {
                write!(f, "column type and value does not match in pattern.")
            },
            Error::WrongNumberOfValues(expected, actual) => {
                write!(f, "wrong number of values, expected: {}, actual: {}.", expected, actual)
            },
            Error::WrongValueType(idx) => {
                write!(f, "wrong value type at column index: {}", idx)
            },
            Error::WrongNumberOfMatches(expected, actual) => {
                write!(f, "wrong number of matches, expected: {}, actual: {}.", expected, actual)
            },
            Error::WrongMatchType(idx) => {
                write!(f, "wrong match type at column index: {}", idx)
            },
            Error::NothingToMatch => {
                write!(f, "nothing to match, perhaps try some match that is not Any ?")
            },
        }
    }
}