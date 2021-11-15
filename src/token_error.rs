use std::fmt;
use std::error;

#[derive(Debug, Clone, Copy)]
pub enum TokenizeErrorKind {
    InvalidChar(char),
}

#[derive(Debug)]
pub struct TokenizeError<'a> {
    val: TokenizeErrorKind,
    pos: usize,
    str: &'a str,
}

impl<'a> TokenizeError<'a> {
    fn new(val: TokenizeErrorKind, pos: usize, str: &'a str) -> Self {
        Self {
            val,
            pos,
            str
        }
    }

    pub fn invalid_char(c: char, pos: usize, str: &'a str) -> Self {
        Self::new(TokenizeErrorKind::InvalidChar(c), pos, str)
    }
}

impl<'a> fmt::Display for TokenizeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenizeErrorKind::*;
        let space = " ".repeat(self.pos);
        match self.val {
            InvalidChar(c) => write!(f, "{}\n{}^ Invalid char '{}'", self.str, space, c),
        }
    }
}

impl<'a> error::Error for TokenizeError<'a> {}
