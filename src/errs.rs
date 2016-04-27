#[derive(Debug)]
pub enum Error {
    NoColumn,
    InvalidBucket,
    InvalidColumn,
    InvalidColumnMatch,
    WrongNumberOfValues(usize, usize),
    WrongValueType(usize),
    WrongNumberOfMatches(usize, usize),
    WrongMatchType(usize),
}