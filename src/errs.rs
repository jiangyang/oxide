#[derive(Debug)]
pub enum Error {
    InvalidBucket,
    WrongNumberOfValues(usize, usize),
    WrongValueType(usize),
    WrongNumberOfMatches(usize, usize),
    WrongMatchType(usize),
}