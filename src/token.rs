use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Annot<T> {
    val: T
}
impl<T> Annot<T> {
    fn new(val: T) -> Self {
        Annot {
            val
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenizeErrorKind {
    InvalidChar(char),
    Eof
}

type TokenizeError = Annot<TokenizeErrorKind>;

impl TokenizeError {
    fn invalid_char(c: char) -> Self {
        Self::new(TokenizeErrorKind::InvalidChar(c))
    }

    fn eof() -> Self {
        Self::new(TokenizeErrorKind::Eof)
    }
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenizeErrorKind::*;
        match self.val {
            InvalidChar(c) => write!(f, "Invalid char '{}'", c),
            Eof => write!(f, "End of File"),
        }
    }
}

impl error::Error for TokenizeError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Num(i32),
    Plus,
    Minus,
    EOF,
}

pub struct Token {
    pub val: TokenKind,
}

impl Token {
    fn new(val: TokenKind) -> Self {
        Token {
            val
        }
    }

    pub fn tokenize(str: &[u8]) -> Result<Vec<Token>, TokenizeError> {
        let mut result: Vec<Token> = Vec::new();
        let mut pos = 0;

        macro_rules! tokenize_except_num {
            ($token_kind:expr) => {
                {
                    let token = Token::new($token_kind);
                    result.push(token);
                    pos += 1;
                }
            };
        }
        
        while pos < str.len() {
            match str[pos] {
                b' ' | b'\t' | b'\n' => pos += 1,
                b'+' => tokenize_except_num!(TokenKind::Plus),
                b'-' => tokenize_except_num!(TokenKind::Minus),
                b'0'..=b'9' => {
                    let (num, new_pos) = Token::tokenize_number(str, pos)?;
                    let token = Token::new(TokenKind::Num(num));
                    result.push(token);
                    pos = new_pos;
                },
                b => return Err(TokenizeError::invalid_char(b as char)),
            }
        }
        result.push(Token::new(TokenKind::EOF));

        Ok(result)
    }

    fn tokenize_number(input: &[u8], mut pos: usize) -> Result<(i32, usize) , TokenizeError> {
        let start = pos;
        while pos < input.len() {
            match input[pos] {
                b'0'..=b'9' => pos += 1,
                _ => break,
            }
        }
        let num = String::from_utf8(input[start..pos].to_vec())
                    .unwrap()
                    .parse()
                    .unwrap();
        Ok((num, pos))
    }


}
