use std::iter::Peekable;
use std::collections::HashMap;

use crate::token::{Token, TokenKind};
use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AstErrorKind {
    UnclosedParenth,
    NotPatternMatching,
    RequirSemicolon,
    EoF,
}

#[derive(Debug)]
pub struct AstError {
    val: AstErrorKind,
    pos: usize,
}

impl AstError {
    fn new(val: AstErrorKind, pos: usize) -> Self {
        AstError {
            val,
            pos
        }
    }

    pub fn unclosed_parenth(pos: usize) -> Self{
        Self::new(AstErrorKind::UnclosedParenth, pos)
    }

    pub fn not_pattern_matching(pos: usize) -> Self {
        Self::new(AstErrorKind::NotPatternMatching, pos)
    }

    pub fn eof(pos: usize) -> Self {
        Self::new(AstErrorKind::EoF, pos)
    }

    pub fn require_semicolon(pos: usize) -> Self {
        Self::new(AstErrorKind::RequirSemicolon, pos)
    }
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AstErrorKind::*;
        match self.val {
            UnclosedParenth => write!(f, "Unclosed"),
            NotPatternMatching => write!(f, "Not Pattern"),
            RequirSemicolon => write!(f, "Require Semicolon"),
            EoF => write!(f, "EoF in imcomplement position"),
        }
    }
}

impl error::Error for AstError {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Small,      // <
    EqualSmall, // <=
    Equal,      // ==
    NotEqual,   // !=
    Substitution,   // =
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OneNodeKind {
    Return,
}

#[derive(Debug)]
pub enum Ast {
    Num(i32),
    Ident(String, usize),
    OneNode {
        node_kind: OneNodeKind,
        hs: Box<Ast>,
    },
    Node {
        node_kind: NodeKind,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    }
}

macro_rules! match_token {
    ($token_kind:path, $pos:ident) => {
        Token {
            val: $token_kind,
            pos: $pos
        }
    };
}

macro_rules! match_token_num {
    ($num:ident) => {
        Token {
            val: TokenKind::Num($num),
            pos: pos
        }
    };
}

macro_rules! match_token_ident {
    ($str:ident) => {
        Token {
            val: TokenKind::Ident($str),
            pos: pos
        }
    };
}

macro_rules! match_token_nothing {
    ($pos:ident) => {
        Token {
            val: _,
            pos: $pos
        }
    };
}

impl Ast {
    fn num(num: i32) -> Self {
        Ast::Num(num)
    }

    fn node(node_kind: NodeKind, lhs: Ast, rhs: Ast) -> Self {
        Ast::Node {
            node_kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn one_node(node_kind: OneNodeKind, hs: Ast) -> Self {
        Ast::OneNode {
            node_kind,
            hs: Box::new(hs),
        }
    }
    // program      = stmt*
    // stmt         = expr ";" | "return" expr ";"
    // expr         = assign
    // assign       = equality ("=" assign)?
    // equality     = relational ("==" relational | "!=" relational)*
    // relationl    = add ("<" add | ">" add | "<=" add | ">=" add)*
    // add           = mul ("+" mul | "-" mul) *
    // mul          = unary ("*" unary | "/" unary)*
    // unary        = ("+" | "-")? primary
    // primary      = num | ident | "(" expr ")"
    pub fn program<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<(Vec<Ast>, usize), AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let mut ast_vec = Vec::new();
        let mut variable_list: HashMap<String, usize> = HashMap::new();
        while tokens.peek().unwrap().val != TokenKind::EOF {
            let ast = Ast::stmt(tokens, &mut variable_list)?;
            ast_vec.push(ast);
        }
        Ok((ast_vec, variable_list.len()))
    }

    fn stmt<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        match tokens.peek().unwrap() {
            match_token!(TokenKind::Return, pos) => {
                match tokens.next().unwrap() {
                    match_token!(TokenKind::Return, pos) => {
                        let expr = Ast::expr(tokens, variable_list)?;
                        match tokens.next().unwrap() {
                            match_token!(TokenKind::SemiColon, pos) => Ok(Ast::one_node(OneNodeKind::Return, expr)),
                            match_token_nothing!(pos) => Err(AstError::require_semicolon(pos)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            _ => {
                let expr = Ast::expr(tokens, variable_list);
                match tokens.next().unwrap() {
                    match_token!(TokenKind::SemiColon, pos) => expr,
                    match_token_nothing!(pos) => Err(AstError::require_semicolon(pos)),
                }
            }
        }
    }

    fn expr<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        Ast::assign(tokens, variable_list)
    }

    fn assign<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let l_ast = Ast::equality(tokens, variable_list)?;
        match tokens.peek().unwrap().val {
            TokenKind::Substitution => {
                match tokens.next().unwrap() {
                    match_token!(TokenKind::Substitution, pos) => {
                        let r_ast = Ast::assign(tokens, variable_list)?;
                        Ok(Ast::node(NodeKind::Substitution, l_ast, r_ast))
                    },
                    _ => unreachable!(),
                }
            }
            _ => return Ok(l_ast)
        }
    }

    fn equality<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let mut l_ast = Ast::relational(tokens, variable_list)?;
        loop {
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Equal, pos) | match_token!(TokenKind::NotEqual, pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Equal, pos) => {
                            let r_ast = Ast::relational(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::Equal, l_ast, r_ast);
                        },
                        match_token!(TokenKind::NotEqual, pos) => {
                            let r_ast = Ast::relational(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::NotEqual, l_ast, r_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn relational<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let mut l_ast = Ast::add(tokens, variable_list)?;
        loop {
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Small, pos) | match_token!(TokenKind::Large, pos)
                | match_token!(TokenKind::EqualSmall, pos) | match_token!(TokenKind::EqualLarge, pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Small, pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::Small, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Large, pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            // l_ast = Ast::node(NodeKind::Large, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::Small, r_ast, l_ast);
                        },
                        match_token!(TokenKind::EqualSmall, pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::EqualSmall, l_ast, r_ast);
                        },
                        match_token!(TokenKind::EqualLarge, pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            // l_ast = Ast::node(NodeKind::EqualLarge, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::EqualSmall, r_ast, l_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn add<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   mul ("+" mul | "-" mul) *
        // ^
        let mut l_ast = Ast::mul(tokens, variable_list)?;
        loop {
            //   mul ("+" mul | "-" mul) *
            //     ^
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Plus, pos) | match_token!(TokenKind::Minus, pos) => {
                    //   mul ("+" mul | "-" mul) *
                    //         ^
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Plus, pos) => {
                            //   mul ("+" mul | "-" mul) *
                            //         ^
                            let r_ast = Ast::mul(tokens, variable_list)?;
                            //   mul ("+" mul | "-" mul) *
                            //              ^
                            l_ast = Ast::node(NodeKind::Add, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Minus, pos) => {
                            //   mul ("+" mul | "-" mul) *
                            //                   ^
                            let r_ast = Ast::mul(tokens, variable_list)?;
                            //   mul ("+" mul | "-" mul) *
                            //                        ^
                            l_ast = Ast::node(NodeKind::Sub, l_ast, r_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn mul<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   unary ("*" unary | "/" unary)*
        //  ^
        let mut l_ast = Ast::unary(tokens, variable_list)?;
        loop {
            // unary ("*" unary | "/" unary)*
            //     ^
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Asterisk, pos) | match_token!(TokenKind::Slash, pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Asterisk, pos) => {
                            // unary ("*" unary | "/" unary)
                            //         ^
                            let r_ast = Ast::unary(tokens, variable_list)?;
                            // unary ("*" unary | "/" unary)
                            //                ^
                            l_ast = Ast::node(NodeKind::Mul, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Slash, pos) => {
                            // unary ("*" unary | "/" unary)
                            //                     ^
                            let r_ast = Ast::unary(tokens, variable_list)?;
                            // unary ("*" unary | "/" unary)
                            //                            ^
                            l_ast = Ast::node(NodeKind::Div, l_ast, r_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn unary<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   ("+" | "-")? primary
        //  ^
        match tokens.peek().unwrap().val {
            TokenKind::Plus | TokenKind::Minus => {
                match tokens.next().unwrap() {
                    //   ("+" | "-")? primary
                    //     ^
                    match_token!(TokenKind::Plus, pos) => return Ast::primary(tokens, variable_list),
                    //   ("+" | "-")? primary
                    //           ^
                    match_token!(TokenKind::Minus, pos) => {
                        let l_ast = Ast::num(0);
                        let r_ast = Ast::primary(tokens, variable_list)?;
                    //   ("+" | "-")? primary
                    //                      ^
                        Ok(Ast::node(NodeKind::Sub, l_ast, r_ast))
                    },
                    _ => unreachable!(),
                }
            },
            _ => return Ast::primary(tokens, variable_list),
        }
    }

    fn primary<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //  num | "("
        // ^
        match tokens.next().unwrap() {
            match_token_num!(num) => Ok(Ast::num(num)),
            match_token_ident!(str) => {
                let ident_str = str.clone();
                let pos = match variable_list.get(&str) {
                    Some(&pos) => pos,
                    None => {
                        let variable_list_len = variable_list.len();
                        variable_list.insert(str, variable_list_len + 1);
                        variable_list_len + 1
                    }
                };
                Ok(Ast::Ident(ident_str, pos))
            },
            match_token!(TokenKind::LParen, pos) => {
                // "(" epxr ")"
                //  ^
                let ex = Ast::expr(tokens, variable_list)?;
                // "(" epxr ")"
                //        ^
                match tokens.next().unwrap() {
                    match_token!(TokenKind::RParen, pos) => Ok(ex),
                    match_token!(TokenKind::EOF, pos) => Err(AstError::eof(pos)),
                    match_token_nothing!(pos) => Err(AstError::unclosed_parenth(pos)),
                }
            },
            match_token!(TokenKind::EOF, pos) => Err(AstError::eof(pos)),
            match_token_nothing!(pos) => Err(AstError::not_pattern_matching(pos)),
        }
    }
}
