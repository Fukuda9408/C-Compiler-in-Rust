use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Num(i32),
    Ident(String),
    Plus,
    Minus,
    LParen,
    RParen,
    Asterisk,
    Slash,
    Large,      // >
    Small,      // <
    EqualSmall, // <=
    EqualLarge, // >=
    Equal,      // ==
    NotEqual,   // !=
    Substitution,   // =
    SemiColon,  // ;
    EOF,
}
#[derive(Debug, Clone, Copy)]
pub enum TokenizeErrorKind {
    InvalidChar(char),
    Eof
}

#[derive(Debug)]
pub struct TokenizeError {
    val: TokenizeErrorKind,
    pos: usize,
    str: String,
}

impl TokenizeError {
    fn new(val: TokenizeErrorKind, pos: usize, str: String) -> Self {
        Self {
            val,
            pos,
            str
        }
    }

    pub fn invalid_char(c: char, pos: usize, str: String) -> Self {
        Self::new(TokenizeErrorKind::InvalidChar(c), pos, str)
    }

    pub fn eof(pos: usize, str: String) -> Self {
        Self::new(TokenizeErrorKind::Eof, pos, str)
    }
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenizeErrorKind::*;
        let space = " ".repeat(self.pos);
        match self.val {
            InvalidChar(c) => write!(f, "{}\n{}^ Invalid char '{}'", self.str, space, c),
            Eof => write!(f, "{}\n{}^ End of File", self.str, space),
        }
    }
}

impl error::Error for TokenizeError {}

#[derive(Debug)]
pub struct Token {
    pub val: TokenKind,
    pub pos: usize,
}

impl Token {
    fn new(val: TokenKind, pos: usize) -> Self {
        Token {
            val,
            pos
        }
    }

    pub fn tokenize(str: &[u8]) -> Result<Vec<Token>, TokenizeError> {
        let mut result: Vec<Token> = Vec::new();
        let mut pos = 0;

        macro_rules! tokenize_except_num {
            ($token_kind:expr) => {
                {
                    let token = Token::new($token_kind, pos);
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
                b')' => tokenize_except_num!(TokenKind::RParen),
                b'(' => tokenize_except_num!(TokenKind::LParen),
                b'*' => tokenize_except_num!(TokenKind::Asterisk),
                b'/' => tokenize_except_num!(TokenKind::Slash),
                b'<' => {
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::EqualSmall),
                        _ => result.push(Token::new(TokenKind::Small, pos)),
                    }
                },
                b'>' => {
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::EqualLarge),
                        _ => result.push(Token::new(TokenKind::Large, pos)),
                    }
                },
                b'=' => {
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::Equal),
                        _ => result.push(Token::new(TokenKind::Substitution, pos)),
                    }
                },
                b'!' => {
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::NotEqual),
                        b => return Err(TokenizeError::invalid_char(b as char, pos, String::from_utf8(str.to_vec()).unwrap())),
                    }
                },
                b';' => tokenize_except_num!(TokenKind::SemiColon),
                b'0'..=b'9' => {
                    let (num, new_pos) = Token::tokenize_number(str, pos)?;
                    let token = Token::new(TokenKind::Num(num), new_pos);
                    result.push(token);
                    pos = new_pos;
                },
                _ => {
                    let (ident, new_pos) = Token::tokenize_ident(str, pos);
                    let token = Token::new(TokenKind::Ident(ident), new_pos);
                    result.push(token);
                    pos = new_pos;
                }
            }
        }
        result.push(Token::new(TokenKind::EOF, pos));
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

    fn tokenize_ident(input: &[u8], mut pos: usize) -> (String, usize) {
        let start = pos;
        while pos < input.len() && !b" \t\n><=!".contains(&input[pos]) {
            pos += 1;
        }
        let ident = String::from_utf8(input[start..pos].to_vec())
                    .unwrap()
                    .parse()
                    .unwrap();
        (ident, pos)
    }
}
