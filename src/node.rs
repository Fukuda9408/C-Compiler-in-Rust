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

#[derive(Debug)]
pub enum OneNodeKind {
    Return,
    Block,
}

#[derive(Debug)]
pub enum Ast {
    Num(i32),
    Ident(String, usize),
    ReturnNode {
        node_kind: OneNodeKind,
        hs: Box<Ast>,
    },
    BlockNode {
        node_kind: OneNodeKind,
        hs: Box<Vec<Ast>>,
    },
    Node {
        node_kind: NodeKind,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    },
    ForNode {
        for_num: usize,
        initial: Box<Option<Ast>>,
        condition: Box<Option<Ast>>,
        change: Box<Option<Ast>>,
        stmt: Box<Ast>,
    },
    IfNode {
        for_num: usize,
        condition: Box<Ast>,
        stmt: Box<Ast>,
    },
    IfElseNode {
        for_num: usize,
        condition: Box<Ast>,
        stmt_1: Box<Ast>,
        stmt_2: Box<Ast>,
    },
    WhileNode {
        for_num: usize,
        condition: Box<Ast>,
        stmt: Box<Ast>,
    },
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
            pos: _pos
        }
    };
}

macro_rules! match_token_ident {
    ($str:ident) => {
        Token {
            val: TokenKind::Ident($str),
            pos: _pos
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

struct ControlVal {
    val_if_else: usize,
    val_while: usize,
    val_for : usize,
}

impl ControlVal {
    fn new() -> Self {
        ControlVal {
            val_if_else: 0,
            val_while: 0,
            val_for: 0,
        }
    }

    fn val_if_else(&mut self) -> usize {
        let res = self.val_if_else;
        self.val_if_else += 1;
        res
    }

    fn val_while(&mut self) -> usize {
        let res =  self.val_while;
        self.val_while += 1;
        res
    }

    fn val_for(&mut self) -> usize {
        let res = self.val_for;
        self.val_for += 1;
        res
    }
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

    fn return_node(node_kind: OneNodeKind, hs: Ast) -> Self {
        Ast::ReturnNode {
            node_kind,
            hs: Box::new(hs),
        }
    }


    fn block_node(node_kind: OneNodeKind, hs: Vec<Ast>) -> Self {
        Ast::BlockNode {
            node_kind,
            hs: Box::new(hs),
        }
    }

    fn for_node(for_num: usize, initilal: Option<Ast>, condtion: Option<Ast>, change: Option<Ast>, stmt: Ast) -> Self {
        Ast::ForNode {
            for_num,
            initial: Box::new(initilal),
            condition: Box::new(condtion),
            change: Box::new(change),
            stmt: Box::new(stmt),
        }
    }

    fn if_node(for_num: usize, condtion: Ast, stmt: Ast) -> Self {
        Ast::IfNode {
            for_num,
            condition: Box::new(condtion),
            stmt: Box::new(stmt),
        }
    }

    fn if_else_node(for_num: usize, condtion: Ast, stmt_1: Ast, stmt_2: Ast) -> Self {
        Ast::IfElseNode {
            for_num,
            condition: Box::new(condtion),
            stmt_1: Box::new(stmt_1),
            stmt_2: Box::new(stmt_2),
        }
    }

    fn while_node(for_num: usize, condtion: Ast, stmt: Ast) -> Self {
        Ast::WhileNode {
            for_num,
            condition: Box::new(condtion),
            stmt: Box::new(stmt),
        }
    }
    // program      = stmt*
    // stmt         = expr ";"
    //              | "{" stmt* "}"
    //              | "if" "(" expr ")" stmt ("else" stmt)?
    //              | "while" "(" expr ")" stmt
    //              | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    //              |return" expr ";"
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
        let mut control_val = ControlVal::new();
        let mut variable_list: HashMap<String, usize> = HashMap::new();
        while tokens.peek().unwrap().val != TokenKind::EOF {
            let ast = Ast::stmt(tokens, &mut variable_list, &mut control_val)?;
            ast_vec.push(ast);
        }
        Ok((ast_vec, variable_list.len()))
    }

    fn stmt<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>, control_val: &mut ControlVal) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        match tokens.peek().unwrap() {
            match_token!(TokenKind::Return,_pos) => {
                match tokens.next().unwrap() {
                    match_token!(TokenKind::Return,_pos) => {
                        let expr = Ast::expr(tokens, variable_list)?;
                        match tokens.next().unwrap() {
                            match_token!(TokenKind::SemiColon, _pos) => Ok(Ast::return_node(OneNodeKind::Return, expr)),
                            match_token_nothing!(pos) => Err(AstError::require_semicolon(pos)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token!(TokenKind::If,_pos) => {
                match tokens.next().unwrap() {
                    match_token!(TokenKind::If,pos) => {
                        match tokens.next().unwrap() {
                            match_token!(TokenKind::LParen,_pos) => {
                                let expr = Ast::expr(tokens, variable_list)?;
                                match tokens.next().unwrap() {
                                    match_token!(TokenKind::RParen,_pos) => {
                                        let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                        match tokens.peek().unwrap() {
                                            match_token!(TokenKind::Else,_pos) => {
                                                match tokens.next().unwrap() {
                                                    match_token!(TokenKind::Else,_pos) => {
                                                        let stmt_second = Ast::stmt(tokens, variable_list, control_val)?;
                                                        Ok(Ast::if_else_node(control_val.val_if_else(), expr, stmt, stmt_second))
                                                    },
                                                    _ => unreachable!(),
                                                }
                                            }
                                            _ => Ok(Ast::if_node(control_val.val_if_else(), expr, stmt))
                                        }
                                    },
                                    _ => Err(AstError::unclosed_parenth(pos)),
                                }
                            },
                            _ => Err(AstError::not_pattern_matching(pos)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token!(TokenKind::While,_pos) => {
                match tokens.next().unwrap() {
                    match_token!(TokenKind::While,pos) => {
                        match tokens.next().unwrap() {
                            match_token!(TokenKind::LParen,_pos) => {
                                let expr = Ast::expr(tokens, variable_list)?;
                                match tokens.next().unwrap() {
                                    match_token!(TokenKind::RParen,_pos) => {
                                        let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                        // Ok(Ast::node(NodeKind::While(control_val.val_while()), expr, stmt))
                                        Ok(Ast::while_node(control_val.val_while(), expr, stmt))
                                    },
                                    _ => Err(AstError::unclosed_parenth(pos)),
                                }
                            },
                            _ => Err(AstError::not_pattern_matching(pos)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token!(TokenKind::For,_pos) => {
                let for_num = control_val.val_for();
                match tokens.next().unwrap() {
                    match_token!(TokenKind::For,pos) => {
                        match tokens.next().unwrap() {
                            match_token!(TokenKind::LParen,_pos) => {
                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                //        ^
                                match tokens.peek().unwrap() {
                                    match_token!(TokenKind::SemiColon,_pos) => {
                                        match tokens.next().unwrap() {
                                            match_token!(TokenKind::SemiColon,_pos) => {
                                            // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                            //                  ^
                                                match tokens.peek().unwrap() {
                                                    match_token!(TokenKind::SemiColon,_pos) => {
                                                        match tokens.next().unwrap() {
                                                            match_token!(TokenKind::SemiColon,_pos) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token!(TokenKind::RParen,_pos) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, None, None, None, stmt))
                                                                            },
                                                                            _ => unreachable!(),
                                                                        }
                                                                    },
                                                                    _ => {
                                                                        let expr_third = Ast::expr(tokens, variable_list)?;
                                                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                        //                                   ^
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, None, None, Some(expr_third), stmt))
                                                                            },
                                                                            _ => Err(AstError::not_pattern_matching(pos))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                    _ => {
                                                        let expr_second = Ast::expr(tokens, variable_list)?;
                                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                        //                         ^
                                                        match tokens.next().unwrap() {
                                                            match_token!(TokenKind::SemiColon,_pos) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token!(TokenKind::RParen,_pos) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, None, Some(expr_second), None, stmt))
                                                                            },
                                                                            _ => unreachable!(),
                                                                        }
                                                                    },
                                                                    _ => {
                                                                        let expr_third = Ast::expr(tokens, variable_list)?;
                                                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                        //                                   ^
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, None, Some(expr_second), Some(expr_third), stmt))
                                                                            },
                                                                            _ => Err(AstError::not_pattern_matching(pos))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                }
                                            },
                                            _ => unreachable!(),
                                        }
                                    },
                                    _ => {
                                        let expr_first = Ast::expr(tokens, variable_list)?;
                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                        //               ^
                                        match tokens.next().unwrap() {
                                            match_token!(TokenKind::SemiColon,_pos) => {
                                            // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                            //                  ^
                                                match tokens.peek().unwrap() {
                                                    match_token!(TokenKind::SemiColon,_pos) => {
                                                        match tokens.next().unwrap() {
                                                            match_token!(TokenKind::SemiColon,_pos) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token!(TokenKind::RParen,_pos) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, Some(expr_first), None, None, stmt))
                                                                            },
                                                                            _ => unreachable!(),
                                                                        }
                                                                    },
                                                                    _ => {
                                                                        let expr_third = Ast::expr(tokens, variable_list)?;
                                                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                        //                                   ^
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, Some(expr_first), None, Some(expr_third), stmt))
                                                                            },
                                                                            _ => Err(AstError::not_pattern_matching(pos))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                    _ => {
                                                        let expr_second = Ast::expr(tokens, variable_list)?;
                                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                        //                         ^
                                                        match tokens.next().unwrap() {
                                                            match_token!(TokenKind::SemiColon,_pos) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token!(TokenKind::RParen,_pos) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, Some(expr_first), Some(expr_second), None, stmt))
                                                                            },
                                                                            _ => unreachable!(),
                                                                        }
                                                                    },
                                                                    _ => {
                                                                        let expr_third = Ast::expr(tokens, variable_list)?;
                                                                        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                        //                                   ^
                                                                        match tokens.next().unwrap() {
                                                                            match_token!(TokenKind::RParen,_pos) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, Some(expr_first), Some(expr_second), Some(expr_third), stmt))
                                                                            },
                                                                            _ => Err(AstError::not_pattern_matching(pos))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                }
                                            },
                                            _ => Err(AstError::not_pattern_matching(pos)),
                                        }
                                    }
                                }
                            },
                            _ => Err(AstError::not_pattern_matching(pos)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token!(TokenKind::LCuryBra, _pos) => {
                match tokens.next().unwrap() {
                    match_token!(TokenKind::LCuryBra, _pos) => {
                        let mut res_stmt: Vec<Ast> = Vec::new();
                        loop {
                            let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                            res_stmt.push(stmt);
                            // エラー処理 todo
                            if tokens.peek().unwrap().val == TokenKind::RCuryBra {
                                match tokens.next().unwrap() {
                                    match_token!(TokenKind::RCuryBra, _pos) => break,
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Ok(Ast::block_node(OneNodeKind::Block, res_stmt))
                    }
                    _ => unreachable!(),
                }
            },
            _ => {
                let expr = Ast::expr(tokens, variable_list);
                match tokens.next().unwrap() {
                    match_token!(TokenKind::SemiColon,_pos) => Ok(expr?),
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
                    match_token!(TokenKind::Substitution,_pos) => {
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
                match_token!(TokenKind::Equal, _pos) | match_token!(TokenKind::NotEqual, _pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Equal,_pos) => {
                            let r_ast = Ast::relational(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::Equal, l_ast, r_ast);
                        },
                        match_token!(TokenKind::NotEqual,_pos) => {
                            let r_ast = Ast::relational(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::NotEqual, l_ast, r_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(_pos) => return Ok(l_ast)
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
                match_token!(TokenKind::Small, _pos) | match_token!(TokenKind::Large,_pos)
                | match_token!(TokenKind::EqualSmall, _pos) | match_token!(TokenKind::EqualLarge,_pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Small,_pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::Small, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Large,_pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            // l_ast = Ast::node(NodeKind::Large, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::Small, r_ast, l_ast);
                        },
                        match_token!(TokenKind::EqualSmall,_pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::EqualSmall, l_ast, r_ast);
                        },
                        match_token!(TokenKind::EqualLarge,_pos) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            // l_ast = Ast::node(NodeKind::EqualLarge, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::EqualSmall, r_ast, l_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                match_token_nothing!(_pos) => return Ok(l_ast)
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
                match_token!(TokenKind::Plus, _pos) | match_token!(TokenKind::Minus,_pos) => {
                    //   mul ("+" mul | "-" mul) *
                    //         ^
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Plus,_pos) => {
                            //   mul ("+" mul | "-" mul) *
                            //         ^
                            let r_ast = Ast::mul(tokens, variable_list)?;
                            //   mul ("+" mul | "-" mul) *
                            //              ^
                            l_ast = Ast::node(NodeKind::Add, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Minus,_pos) => {
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
                match_token_nothing!(_pos) => return Ok(l_ast)
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
                match_token!(TokenKind::Asterisk, _pos) | match_token!(TokenKind::Slash,_pos) => {
                    match tokens.next().unwrap() {
                        match_token!(TokenKind::Asterisk,_pos) => {
                            // unary ("*" unary | "/" unary)
                            //         ^
                            let r_ast = Ast::unary(tokens, variable_list)?;
                            // unary ("*" unary | "/" unary)
                            //                ^
                            l_ast = Ast::node(NodeKind::Mul, l_ast, r_ast);
                        },
                        match_token!(TokenKind::Slash,_pos) => {
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
                match_token_nothing!(_pos) => return Ok(l_ast)
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
                    match_token!(TokenKind::Plus,_pos) => return Ast::primary(tokens, variable_list),
                    //   ("+" | "-")? primary
                    //           ^
                    match_token!(TokenKind::Minus,_pos) => {
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
            match_token!(TokenKind::LParen,pos) => {
                // "(" epxr ")"
                //  ^
                let ex = Ast::expr(tokens, variable_list)?;
                // "(" epxr ")"
                //        ^
                match tokens.next().unwrap() {
                    match_token!(TokenKind::RParen,_pos) => Ok(ex),
                    match_token!(TokenKind::EOF,_pos) => Err(AstError::eof(pos)),
                    match_token_nothing!(pos) => Err(AstError::unclosed_parenth(pos)),
                }
            },
            match_token!(TokenKind::EOF,pos) => Err(AstError::eof(pos)),
            match_token_nothing!(pos) => Err(AstError::not_pattern_matching(pos)),
        }
    }
}
