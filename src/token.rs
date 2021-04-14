use std::error;
use std::fmt;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Num(i32),
    Ident(String),
    Plus,
    Minus,
    LParen,
    RParen,
    LCuryBra,
    RCuryBra,
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
    Comma,      // ,
    Ampersand,  // &
    Return,
    If,
    Else,
    While,
    For,
    EOF,
}
#[derive(Debug, Clone, Copy)]
pub enum TokenizeErrorKind {
    InvalidChar(char),
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
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenizeErrorKind::*;
        let space = " ".repeat(self.pos);
        match self.val {
            InvalidChar(c) => write!(f, "{}\n{}^ Invalid char '{}'", self.str, space, c),
        }
    }
}

impl error::Error for TokenizeError {}

#[derive(Debug, Copy, Clone)]
pub struct Location(pub usize, pub usize);

#[derive(Debug)]
pub struct Token {
    pub val: TokenKind,
    pub pos: Location,
    pub line_num : usize
}

impl Token {
    pub fn new(val: TokenKind, pos: Location, line_num: usize) -> Self {
        Token {
            val,
            pos,
            line_num
        }
    }

    pub fn tokenize(str: &[u8], line_num: usize) -> Result<Vec<Token>, TokenizeError> {
        let mut result: Vec<Token> = Vec::new();
        let mut pos = 0;

        macro_rules! tokenize_except_num {
            ($token_kind:expr) => {
                {
                    let token = Token::new($token_kind, Location(pos, pos), line_num);
                    result.push(token);
                    pos += 1;
                }
            };
        }

        macro_rules! tokenize_variable {
            () => {
                let (ident, new_pos) = Token::tokenize_ident(str, pos);
                let token = Token::new(TokenKind::Ident(ident), Location(pos, new_pos), line_num);
                result.push(token);
                pos = new_pos;
            };
        }

        while pos < str.len() {
            match str[pos] {
                b' ' | b'\t' | b'\n' => pos += 1,
                b'+' => tokenize_except_num!(TokenKind::Plus),
                b'-' => tokenize_except_num!(TokenKind::Minus),
                b')' => tokenize_except_num!(TokenKind::RParen),
                b'(' => tokenize_except_num!(TokenKind::LParen),
                b'{' => tokenize_except_num!(TokenKind::LCuryBra),
                b'}' => tokenize_except_num!(TokenKind::RCuryBra),
                b'*' => tokenize_except_num!(TokenKind::Asterisk),
                b'&' => tokenize_except_num!(TokenKind::Ampersand),
                b'/' => tokenize_except_num!(TokenKind::Slash),
                b'<' => {
                    let start_pos = pos;
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::EqualSmall),
                        _ => result.push(Token::new(TokenKind::Small, Location(start_pos, pos), line_num)),
                    }
                },
                b'>' => {
                    let start_pos = pos;
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::EqualLarge),
                        _ => result.push(Token::new(TokenKind::Large, Location(start_pos, pos), line_num)),
                    }
                },
                b'=' => {
                    let start_pos = pos;
                    pos += 1;
                    match str[pos] {
                        b'=' => tokenize_except_num!(TokenKind::Equal),
                        _ => result.push(Token::new(TokenKind::Substitution, Location(start_pos, pos), line_num)),
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
                b',' => tokenize_except_num!(TokenKind::Comma),
                b'0'..=b'9' => {
                    let (num, new_pos) = Token::tokenize_number(str, pos)?;
                    let token = Token::new(TokenKind::Num(num), Location(pos, new_pos - 1), line_num);
                    result.push(token);
                    pos = new_pos;
                },
                b'i' => {
                    let (is_contains, new_pos) = Token::tokenize_str(str, pos, "if".as_bytes());
                    if is_contains {
                        result.push(Token::new(TokenKind::If, Location(pos, new_pos - 1), line_num));
                        pos = new_pos;
                    } else {
                        tokenize_variable!();
                    }
                },
                b'f' => {
                    let (is_contains, new_pos) = Token::tokenize_str(str, pos, "for".as_bytes());
                    if is_contains {
                        result.push(Token::new(TokenKind::For, Location(pos, new_pos - 1), line_num));
                        pos = new_pos;
                    } else {
                        tokenize_variable!();
                    }
                },
                b'e' => {
                    let (is_contains, new_pos) = Token::tokenize_str(str, pos, "else".as_bytes());
                    if is_contains {
                        result.push(Token::new(TokenKind::Else, Location(pos, new_pos - 1), line_num));
                        pos = new_pos;
                    } else {
                        tokenize_variable!();
                    }
                },
                b'r' => {
                    let (is_contains, new_pos) = Token::tokenize_str(str, pos, "return".as_bytes());
                    if is_contains {
                        result.push(Token::new(TokenKind::Return, Location(pos, new_pos - 1), line_num));
                        pos = new_pos;
                    } else {
                        tokenize_variable!();
                    }
                },
                b'w' => {
                    let (is_contains, new_pos) = Token::tokenize_str(str, pos, "while".as_bytes());
                    if is_contains {
                        result.push(Token::new(TokenKind::While, Location(pos, new_pos - 1), line_num));
                        pos = new_pos;
                    } else {
                        tokenize_variable!();
                    }
                },
                _ => {
                    tokenize_variable!();
                }
            }
        }
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
        while pos < input.len() && !b" \t\n+-*/()><=!;,&".contains(&input[pos]) {
            pos += 1;
        }
        let ident = String::from_utf8(input[start..pos].to_vec())
                    .unwrap()
                    .parse()
                    .unwrap();
        (ident, pos)
    }

    fn tokenize_str(input: &[u8], mut pos: usize, search_str: &[u8]) -> (bool, usize) {
        let mut start = 0;
        let len = search_str.len();
        while pos < input.len() && start < len {
            if input[pos] != search_str[start] {
                return (false, pos)
            }
            pos += 1;
            start += 1;
        }
        if pos != input.len() {
            if CHARSET.contains(&input[pos]) {
                return (false, pos)
            }
        }
        (true, pos)
    }
}
