use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Num(u64),
    Ident(String),
    Plus,       // +
    Minus,      // -
    LParen,     // (
    RParen,     // )
    LCuryBra,   // {
    RCuryBra,   // }
    Asterisk,   // *
    Slash,      // /
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
    Exclamation,    // !
    Return,
    If,
    Else,
    While,
    For,
    EOF,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenizeErrorKind {
    NotNumber,
}

#[derive(Debug, PartialEq)]
pub struct TokenizeError {
    val: TokenizeErrorKind,
    pos: Location,
    str: String,
}

impl TokenizeError {
    fn new(val: TokenizeErrorKind, pos: Location, str: String) -> Self {
        Self {
            val,
            pos,
            str
        }
    }

    pub fn not_number(pos: Location, original_code: String) -> Self {
        Self::new(TokenizeErrorKind::NotNumber, pos, original_code)
    }
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenizeErrorKind::*;
        let space = " ".repeat(self.pos.0);
        let hat = "^".repeat(self.pos.1 - self.pos.0 + 1);
        match self.val {
            NotNumber => write!(f, "{}\n{}{} NotNumber", self.str, space, hat),
        }
    }
}

impl error::Error for TokenizeError {}

#[derive(Debug, Copy, Clone, PartialEq
)]
pub struct Location(pub usize, pub usize);

#[derive(Debug, PartialEq)]
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

        while pos < str.len() {
            match str[pos] {
                b' ' | b'\t' | b'\n' => pos += 1,
                b'+' => {result.push(Token::new(TokenKind::Plus, Location(pos, pos), line_num)); pos += 1;},
                b'-' => {result.push(Token::new(TokenKind::Minus, Location(pos, pos), line_num)); pos += 1;},
                b')' => {result.push(Token::new(TokenKind::RParen, Location(pos, pos), line_num)); pos += 1;},
                b'(' => {result.push(Token::new(TokenKind::LParen, Location(pos, pos), line_num)); pos += 1;},
                b'{' => {result.push(Token::new(TokenKind::LCuryBra, Location(pos, pos), line_num)); pos += 1;},
                b'}' => {result.push(Token::new(TokenKind::RCuryBra, Location(pos, pos), line_num)); pos += 1;},
                b'*' => {result.push(Token::new(TokenKind::Asterisk, Location(pos, pos), line_num)); pos += 1;},
                b'&' => {result.push(Token::new(TokenKind::Ampersand, Location(pos, pos), line_num)); pos += 1;},
                b'/' => {result.push(Token::new(TokenKind::Slash, Location(pos, pos), line_num)); pos += 1;},
                b'<' => {
                    let start = pos;
                    pos += 1;
                    if pos == str.len() {result.push(Token::new(TokenKind::Small, Location(start, pos - 1), line_num)); break;}
                    match str[pos] {
                        b'=' => {result.push(Token::new(TokenKind::EqualSmall, Location(start, pos), line_num)); pos += 1;},
                        _ => result.push(Token::new(TokenKind::Small, Location(start, pos), line_num)),
                    }
                },
                b'>' => {
                    let start = pos;
                    pos += 1;
                    if pos == str.len() {result.push(Token::new(TokenKind::Large, Location(start, pos - 1), line_num)); break;}
                    match str[pos] {
                        b'=' => {result.push(Token::new(TokenKind::EqualLarge, Location(start, pos), line_num)); pos += 1;},
                        _ => result.push(Token::new(TokenKind::Large, Location(start, pos), line_num)),
                    }
                },
                b'=' => {
                    let start = pos;
                    pos += 1;
                    if pos == str.len() {result.push(Token::new(TokenKind::Substitution, Location(start, pos - 1), line_num)); break;}
                    match str[pos] {
                        b'=' => {result.push(Token::new(TokenKind::Equal, Location(start, pos), line_num)); pos += 1;},
                        _ => result.push(Token::new(TokenKind::Substitution, Location(start, pos), line_num)),
                    }
                },
                b'!' => {
                    let start = pos;
                    pos += 1;
                    if pos == str.len() {result.push(Token::new(TokenKind::Exclamation, Location(start, pos - 1), line_num));break;}
                    match str[pos] {
                        b'=' => {result.push(Token::new(TokenKind::NotEqual, Location(start, pos), line_num)); pos += 1;},
                        _ => result.push(Token::new(TokenKind::Exclamation, Location(start, pos), line_num))
                    }
                },
                b';' => {result.push(Token::new(TokenKind::SemiColon, Location(pos, pos), line_num)); pos += 1;},
                b',' => {result.push(Token::new(TokenKind::Comma, Location(pos, pos), line_num)); pos += 1;},
                b'0'..=b'9' => {
                    let (num, new_pos) = Token::tokenize_number(str, pos)?;
                    let token = Token::new(TokenKind::Num(num), Location(pos, new_pos - 1), line_num);
                    result.push(token);
                    pos = new_pos;
                },
                _ => {
                    let (ident, new_pos) = Token::tokenize_ident(&str, pos);
                    match &ident[..] {
                        "if" => {
                            result.push(Token::new(TokenKind::If, Location(pos, new_pos - 1), line_num));
                        },
                        "else" => {
                            result.push(Token::new(TokenKind::Else, Location(pos, new_pos - 1), line_num));
                        },
                        "while" => {
                            result.push(Token::new(TokenKind::While, Location(pos, new_pos - 1), line_num));
                        },
                        "for" => {
                            result.push(Token::new(TokenKind::For, Location(pos, new_pos - 1), line_num));
                        },
                        "return" => {
                            result.push(Token::new(TokenKind::Return, Location(pos, new_pos - 1), line_num));
                        },
                        _ => result.push(Token::new(TokenKind::Ident(ident), Location(pos, new_pos - 1), line_num))
                    }
                    pos = new_pos
                }
            }
        }
        Ok(result)
    }

    fn tokenize_number(input: &[u8], mut pos: usize) -> Result<(u64, usize) , TokenizeError> {
        let start = pos;
        while pos < input.len() && !b" \t\n+-(){}*/><=!;.&".contains(&input[pos]) {
            pos += 1;
        }
        let num_str = String::from_utf8(input[start..pos].to_vec())
                    .unwrap()
                    .parse::<u64>();
        match num_str {
            Ok(num) => Ok((num, pos)),
            Err(_) => Err(TokenizeError::not_number(Location(start, pos -1), input[start..pos].iter().map(|&c| c as char).collect::<String>())),
        }
    }

    fn tokenize_ident(input: &[u8], mut pos: usize) -> (String, usize) {
        let start = pos;
        while pos < input.len() && !b" \t\n+-(){}*/><=!;.&".contains(&input[pos]) {
            pos += 1;
        }
        // Used in utf-8??
        let ident = String::from_utf8(input[start..pos].to_vec())
                    .unwrap()
                    .parse()
                    .unwrap();
        (ident, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_number() {
        let input = &"098".as_bytes();
        assert_eq!(Token::tokenize_number(input, 0).unwrap(), (98, 3));

        let input = &"012notnum".as_bytes();
        assert_eq!(Token::tokenize_number(input, 0), Err(TokenizeError::not_number(Location(0, 8), String::from("012notnum"))));

        let input = &"012+abc".as_bytes();
        assert_eq!(Token::tokenize_number(input, 0), Ok((12, 3)));
    }

    #[test]
    fn test_tokenize_error_diplay() {
        let tokenize_error = TokenizeError::not_number(Location(4, 5), String::from("a + 0a"));
        assert_eq!(format!("{:#}", tokenize_error), 
            String::from("a + 0a\n    ^^ NotNumber"))
    }

    #[test]
    fn test_tokenize_ident() {
        let input = "abcd".as_bytes();
        assert_eq!(Token::tokenize_ident(input, 0), ("abcd".to_string(), 4));

        let input = "abcd.efg".as_bytes();
        assert_eq!(Token::tokenize_ident(input, 0), ("abcd".to_string(), 4))
    }

    #[test]
    fn test_tokenize() {
        let input = "<=".as_bytes();
        assert_eq!(Token::tokenize(input, 0), Ok(vec![Token::new(TokenKind::EqualSmall, Location(0, 1), 0)]));

        let input = "<".as_bytes();
        assert_eq!(Token::tokenize(input, 0), Ok(vec![Token::new(TokenKind::Small, Location(0, 0), 0)]));

        let input = "ifa".as_bytes();
        assert_eq!(Token::tokenize(input, 0), Ok(vec![
            Token::new(TokenKind::Ident("ifa".to_string()), Location(0, 2), 0)
        ]));

        let input = "if 0123".as_bytes();
        assert_eq!(Token::tokenize(input, 0), Ok(vec![
            Token::new(TokenKind::If, Location(0, 1), 0),
            Token::new(TokenKind::Num(123), Location(3, 6), 0)
        ]))
    }
}
