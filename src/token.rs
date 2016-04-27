extern crate rand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token(usize);

impl Token {
    pub fn new() -> Token {
        Token(rand::random::<usize>())
    }
}