use std::iter::Peekable;

use crate::token::{Token, TokenKind};
use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AstErrorKind {
    UnclosedParenth,
    NotPatternMatching,
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
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AstErrorKind::*;
        match self.val {
            UnclosedParenth => write!(f, "Unclosed"),
            NotPatternMatching => write!(f, "Not Pattern"),
        }
    }
}

impl error::Error for AstError {}

#[derive(Debug, Clone, Copy)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Ast {
    Num(i32),
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

    // expr    = mul ("+" mul | "-" mul) *
    // mul     = primary ("*" primary | "/" primary)*
    // primary = num | "(" expr ")"
    pub fn expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   mul ("+" mul | "-" mul) *
        // ^
        let mut l_ast = Ast::mul(tokens)?;
        loop {
            //   mul ("+" mul | "-" mul) *
            //     ^
            match tokens.next().unwrap() {
                match_token!(TokenKind::RParen, pos) => {
                    //   mul ("+" mul | "-" mul) *
                    //         ^
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Plus, pos) => {
                            //   mul ("+" mul | "-" mul) *
                            //         ^
                            let r_ast = Ast::mul(tokens)?;
                            //   mul ("+" mul | "-" mul) *
                            //              ^
                            l_ast = Ast::node(NodeKind::Add, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Minus, pos) => {
                            //   mul ("+" mul | "-" mul) *
                            //                   ^
                            let r_ast = Ast::primary(tokens)?;
                            //   mul ("+" mul | "-" mul) *
                            //                        ^
                            l_ast = Ast::node(NodeKind::Sub, l_ast, r_ast);
                        },
                        match_token_nothing!(pos) => return Err(AstError::not_pattern_matching(pos)),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn mul<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   primary ("*" primary | "/" primary)*
        // ^
        let mut l_ast = Ast::primary(tokens)?;
        loop {
            // primary ("*" primary | "/" primary)*
            //       ^
            match tokens.next().unwrap() {
                match_token!(TokenKind::RParen, pos) => {
                    // primary ("*" primary | "/" primary)
                    //         ^
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Asterisk, pos) => {
                            // primary ("*" primary | "/" primary)
                            //           ^
                            let r_ast = Ast::primary(tokens)?;
                            // primary ("*" primary | "/" primary)
                            //                    ^
                            l_ast = Ast::node(NodeKind::Mul, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Slash, pos) => {
                            // primary ("*" primary | "/" primary)
                            //                         ^
                            let r_ast = Ast::primary(tokens)?;
                            // primary ("*" primary | "/" primary)
                            //                                  ^
                            l_ast = Ast::node(NodeKind::Div, l_ast, r_ast);
                        },
                        match_token_nothing!(pos) => return Err(AstError::not_pattern_matching(pos)),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn primary<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //  num | "("
        // ^
        match tokens.next().unwrap() {
            match_token_num!(num) => Ok(Ast::num(num)),
            match_token!(TokenKind::LParen, pos) => {
                // "(" epxr ")"
                //  ^
                let ex = Ast::expr(tokens)?;
                // "(" epxr ")"
                //        ^
                match tokens.next().unwrap() {
                    match_token!(TokenKind::RParen, pos) => Ok(ex),
                    match_token_nothing!(pos) => Err(AstError::unclosed_parenth(pos)),
                }
            },
            match_token_nothing!(pos) => Err(AstError::not_pattern_matching(pos)),
        }
    }


}
