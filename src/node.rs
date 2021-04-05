use std::iter::Peekable;

use crate::token::{Token, TokenKind};
use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AstErrorKind {
    UnclosedParenth,
    NotPatternMatching,
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
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AstErrorKind::*;
        match self.val {
            UnclosedParenth => write!(f, "Unclosed"),
            NotPatternMatching => write!(f, "Not Pattern"),
            EoF => write!(f, "EoF in imcomplement position"),
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
    Large,      // >
    Small,      // <
    EqualSmall, // <=
    EqualLarge, // >=
    Equal,      // ==
    NotEqual,   // !=
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
    // expr         = equality
    // equality     = relational ("==" relational | "!=" relational)*
    // relationl    = add ("<" add | ">" add | "<=" add | ">=" add)*
    // add           = mul ("+" mul | "-" mul) *
    // mul          = unary ("*" unary | "/" unary)*
    // unary        = ("+" | "-")? primary
    // primary      = num | "(" expr ")"
    pub fn expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        Ast::equality(tokens)
    }

    fn equality<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let mut l_ast = Ast::relational(tokens)?;
        loop {
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Equal, pos) | match_token!(TokenKind::NotEqual, pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Equal, pos) => {
                            let r_ast = Ast::relational(tokens)?;
                            l_ast = Ast::node(NodeKind::Equal, l_ast, r_ast);
                        },
                        match_token!(TokenKind::NotEqual, pos) => {
                            let r_ast = Ast::relational(tokens)?;
                            l_ast = Ast::node(NodeKind::NotEqual, l_ast, r_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(pos) => return Ok(l_ast)
            }
        }
    }

    fn relational<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let mut l_ast = Ast::add(tokens)?;
        loop {
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Small, pos) | match_token!(TokenKind::Large, pos)
                | match_token!(TokenKind::EqualSmall, pos) | match_token!(TokenKind::EqualLarge, pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Small, pos) => {
                            let r_ast = Ast::add(tokens)?;
                            l_ast = Ast::node(NodeKind::Small, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Large, pos) => {
                            let r_ast = Ast::add(tokens)?;
                            // l_ast = Ast::node(NodeKind::Large, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::Small, r_ast, l_ast);
                        },
                        match_token!(TokenKind::EqualSmall, pos) => {
                            let r_ast = Ast::add(tokens)?;
                            l_ast = Ast::node(NodeKind::EqualSmall, l_ast, r_ast);
                        },
                        match_token!(TokenKind::EqualLarge, pos) => {
                            let r_ast = Ast::add(tokens)?;
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

    fn add<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   mul ("+" mul | "-" mul) *
        // ^
        let mut l_ast = Ast::mul(tokens)?;
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
                            let r_ast = Ast::mul(tokens)?;
                            //   mul ("+" mul | "-" mul) *
                            //              ^
                            l_ast = Ast::node(NodeKind::Add, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Minus, pos) => {
                            //   mul ("+" mul | "-" mul) *
                            //                   ^
                            let r_ast = Ast::mul(tokens)?;
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

    fn mul<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        //   unary ("*" unary | "/" unary)*
        //  ^
        let mut l_ast = Ast::unary(tokens)?;
        loop {
            // unary ("*" unary | "/" unary)*
            //     ^
            match tokens.peek().unwrap() {
                match_token!(TokenKind::Asterisk, pos) | match_token!(TokenKind::Slash, pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Asterisk, pos) => {
                            // unary ("*" unary | "/" unary)
                            //         ^
                            let r_ast = Ast::unary(tokens)?;
                            // unary ("*" unary | "/" unary)
                            //                ^
                            l_ast = Ast::node(NodeKind::Mul, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Slash, pos) => {
                            // unary ("*" unary | "/" unary)
                            //                     ^
                            let r_ast = Ast::unary(tokens)?;
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

    fn unary<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, AstError>
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
                    match_token!(TokenKind::Plus, pos) => return Ast::primary(tokens),
                    //   ("+" | "-")? primary
                    //           ^
                    match_token!(TokenKind::Minus, pos) => {
                        let l_ast = Ast::num(0);
                        let r_ast = Ast::primary(tokens)?;
                    //   ("+" | "-")? primary
                    //                      ^
                        Ok(Ast::node(NodeKind::Sub, l_ast, r_ast))
                    },
                    _ => unreachable!(),
                }
            },
            _ => return Ast::primary(tokens),
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
                    match_token!(TokenKind::EOF, pos) => Err(AstError::eof(pos)),
                    match_token_nothing!(pos) => Err(AstError::unclosed_parenth(pos)),
                }
            },
            match_token!(TokenKind::EOF, pos) => Err(AstError::eof(pos)),
            match_token_nothing!(pos) => Err(AstError::not_pattern_matching(pos)),
        }
    }

    pub fn gen(ast: Ast) {
        match ast {
            Self::Num(num) => {
                println!("  push {}", num);
            },
            Self::Node {
                node_kind,
                lhs,
                rhs,
            } => {
                Self::gen(*lhs);
                Self::gen(*rhs);
                println!("  pop rdi");
                println!("  pop rax");
                match node_kind {
                    NodeKind::Add => println!("  add rax, rdi"),
                    NodeKind::Sub => println!("  sub rax, rdi"),
                    NodeKind::Mul => println!("  imul rax, rdi"),
                    NodeKind::Div => {
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    // 比較演算子では真なら1, 偽なら0が
                    // 最終的にraxに格納される
                    NodeKind::Large => unreachable!(),
                    NodeKind::Small => {
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::EqualLarge => unreachable!(),
                    NodeKind::EqualSmall => {
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::Equal => {
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    },
                    NodeKind::NotEqual => {
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    },
                }
                println!("  push rax");
            }
        }
    }
}
