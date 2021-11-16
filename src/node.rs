use std::iter::Peekable;
use std::collections::HashMap;

use crate::token::{Token, TokenKind, Location};
use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum AstErrorKind {
    UnclosedParenth,
    NotPatternMatching,
    RequirSemicolon,
    RequireComma,
    RequireLeftParenth,
    UndeclaredFunction,
}

#[derive(Debug)]
pub struct AstError {
    val: AstErrorKind,
    pos: Location,
    pub line_num: usize,
}

impl AstError {
    fn new(val: AstErrorKind, pos: Location, line_num: usize) -> Self {
        AstError {
            val,
            pos,
            line_num
        }
    }

    pub fn unclosed_parenth(pos: Location, line_num: usize) -> Self{
        Self::new(AstErrorKind::UnclosedParenth, pos, line_num)
    }

    pub fn not_pattern_matching(pos: Location, line_num: usize) -> Self {
        Self::new(AstErrorKind::NotPatternMatching, pos, line_num)
    }

    pub fn require_semicolon(pos: Location, line_num: usize) -> Self {
        Self::new(AstErrorKind::RequirSemicolon, pos, line_num)
    }

    pub fn require_left_parenth(pos: Location, line_num: usize) -> Self {
        Self::new(AstErrorKind::RequireLeftParenth, pos, line_num)
    }

    pub fn require_commma(pos: Location, line_num: usize) -> Self {
        Self::new(AstErrorKind::RequireComma, pos, line_num)
    }

    pub fn undecrlared_function(pos: Location, line_num: usize) -> Self {
        Self::new(AstErrorKind::UndeclaredFunction, pos, line_num)
    }
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AstErrorKind::*;
        let space = " ".repeat(self.pos.0);
        let hat = "^".repeat(self.pos.1 - self.pos.0 + 1);
        match self.val {
            UnclosedParenth => write!(f, "{}{}Unclosed Parenth", space, hat),
            NotPatternMatching => write!(f, "{}{}Not Pattern", space, hat),
            RequirSemicolon => write!(f, "{}{}Require Semicolon", space, hat),
            RequireComma => write!(f, "{}{}Require Comma", space, hat),
            RequireLeftParenth => write!(f, "{}{}Require Left Parenth", space, hat),
            UndeclaredFunction => write!(f, "{}{}Undeclared Function", space, hat),
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
pub enum Ast {
    Num(u64),
    Ident(String, usize),
    Func(String),
    CallFuncNode {
        func_name: String,
        hs: Box<Vec<Ast>>,
    },
    ReturnNode {
        hs: Box<Ast>,
    },
    BlockNode {
        hs: Box<Vec<Ast>>,
    },
    AddrNode {
        hs: Box<Ast>,
    },
    DerefNode {
        hs: Box<Ast>,
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
    FuncNode {
        argument_num: usize,        // Argument
        local_variable_num: usize,
        func_name: String,
        stmt_block: Box<Ast>,
    }
}

macro_rules! match_token_ok {
    ($token_kind:path) => {
        Token {
            val: $token_kind,
            pos: _,
            line_num: _,
        }
    };
}

macro_rules! match_token_num {
    ($num:ident, $pos:ident, $line_num:ident) => {
        Token {
            val: TokenKind::Num($num),
            pos: $pos,
            line_num: $line_num
        }
    };
}

macro_rules! match_token_ident {
    ($str:ident) => {
        Token {
            val: TokenKind::Ident($str),
            pos: _,
            line_num: _
        }
    };
}

macro_rules! match_token_nothing {
    ($pos:ident, $line_num:ident) => {
        Token {
            val: _,
            pos: $pos,
            line_num: $line_num
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
    fn num(num: u64) -> Self {
        Ast::Num(num)
    }

    fn node(node_kind: NodeKind, lhs: Ast, rhs: Ast) -> Self {
        Ast::Node {
            node_kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn return_node(hs: Ast) -> Self {
        Ast::ReturnNode {
            hs: Box::new(hs),
        }
    }


    fn block_node(hs: Vec<Ast>) -> Self {
        Ast::BlockNode {
            hs: Box::new(hs),
        }
    }

    fn addr_node(hs: Ast) -> Self {
        Ast::AddrNode {
            hs: Box::new(hs)
        }
    }

    fn deref_node(hs: Ast) -> Self {
        Ast::DerefNode {
            hs: Box::new(hs)
        }
    }

    fn call_func_node(func_name: String, hs: Vec<Ast>) -> Self {
        Ast::CallFuncNode {
            func_name,
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

    fn func_node(argument_num: usize, local_variable_num: usize, func_name: String, stmt_block: Ast) -> Self {
        Ast::FuncNode {
            argument_num,
            local_variable_num,
            func_name,
            stmt_block: Box::new(stmt_block),
        }
    }
    // program      = func*
    // func         = ident ( "(" ( ident ",")* ident? ")" ) "{" stmt* "}"
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
    // unary        = "+"? primary | "-"? primary | "*" primary | "&" primary
    // primary      = num | ident ( "(" (unary ",")* unary? ")" )? | "(" expr ")"
    // 本当はunaryのところは符号付数字であるが、これでも構文解析はできるためこれで行く
    pub fn program<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Vec<Ast>, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        // 一つのfuncごとにvariable_listを持つ
        // control_val(ラベルのための連番)はprogramで一つで問題なし
        let mut func_list = Vec::new();
        let mut control_val = ControlVal::new();
        while tokens.peek().unwrap().val != TokenKind::EOF {
            let mut variable_list: HashMap<String, usize> = HashMap::new();
            let func = Ast::func(tokens, &mut variable_list, &mut control_val)?;
            func_list.push(func);
        }
        Ok(func_list)
    }

    fn func<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>, control_val: &mut ControlVal) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        match tokens.next().unwrap() {
            match_token_ident!(str) => {
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::LParen) => {
                        let mut argument_num = 0;
                        loop {
                            match tokens.peek().unwrap() {
                                match_token_ident!(_argument) => {
                                    match tokens.next().unwrap() {
                                        match_token_ident!(argument) => {
                                            // argument_list(arugmentの参照に使用)
                                            argument_num += 1;
                                            // variable_list(ローカル変数の参照に使用, argumentもローカル変数として使用するため追加)
                                            let variable_list_len = variable_list.len();
                                            variable_list.insert(argument, variable_list_len + 1);

                                            match tokens.peek().unwrap() {
                                                match_token_ok!(TokenKind::Comma) => {
                                                    tokens.next();
                                                    continue;
                                                }
                                                match_token_ok!(TokenKind::RParen) => {
                                                    tokens.next();
                                                    break;
                                                }
                                                match_token_nothing!(pos, line_num) => return Err(AstError::not_pattern_matching(*pos, *line_num))
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                },
                                _ => {
                                    match tokens.next().unwrap() {
                                        match_token_ok!(TokenKind::RParen) => break,
                                        match_token_nothing!(pos, line_num) => return Err(AstError::unclosed_parenth(pos, line_num))
                                    }
                                }
                            }
                        }
                        // func()
                        //      ^
                        // ここまで構文解析が完了
                        let mut res_stmt: Vec<Ast> = Vec::new();
                        match tokens.next().unwrap() {
                            match_token_ok!(TokenKind::LCuryBra) => {
                                loop {
                                    let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                    res_stmt.push(stmt);
                                    match tokens.peek().unwrap() {
                                        match_token_ok!(TokenKind::RCuryBra) => {
                                            match tokens.next().unwrap() {
                                                match_token_ok!(TokenKind::RCuryBra) => break,
                                                _ => unreachable!(),
                                            }
                                        }
                                        _ => continue,
                                    }
                                    // // エラー処理 todo
                                    // if tokens.peek().unwrap().val == TokenKind::RCuryBra {
                                    //     match tokens.next().unwrap() {
                                    //         match_token!(TokenKind::RCuryBra, _pos) => break,
                                    //         _ => unreachable!(),
                                    //     }
                                    // }
                                }
                            }
                            match_token_nothing!(pos, line_num) => return Err(AstError::require_left_parenth(pos, line_num))
                        }
                        Ok(Ast::func_node(argument_num, variable_list.len(), str, Ast::block_node(res_stmt)))
                    },
                    match_token_nothing!(pos, line_num) => Err(AstError::require_left_parenth(pos, line_num))
                }
            },
            match_token_nothing!(pos, line_num) => Err(AstError::undecrlared_function(pos, line_num)),
        }
    }

    fn stmt<Tokens>(tokens: &mut Peekable<Tokens>, variable_list: &mut HashMap<String, usize>, control_val: &mut ControlVal) -> Result<Ast, AstError>
    where
        Tokens: Iterator<Item = Token>,
    {
        match tokens.peek().unwrap() {
            match_token_ok!(TokenKind::Return) => {
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::Return) => {
                        let expr = Ast::expr(tokens, variable_list)?;
                        match tokens.next().unwrap() {
                            match_token_ok!(TokenKind::SemiColon) => Ok(Ast::return_node(expr)),
                            match_token_nothing!(pos, line_num) => Err(AstError::require_semicolon(pos, line_num)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token_ok!(TokenKind::If) => {
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::If) => {
                        match tokens.next().unwrap() {
                            match_token_ok!(TokenKind::LParen) => {
                                let expr = Ast::expr(tokens, variable_list)?;
                                match tokens.next().unwrap() {
                                    match_token_ok!(TokenKind::RParen) => {
                                        let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                        match tokens.peek().unwrap() {
                                            match_token_ok!(TokenKind::Else) => {
                                                match tokens.next().unwrap() {
                                                    match_token_ok!(TokenKind::Else) => {
                                                        let stmt_second = Ast::stmt(tokens, variable_list, control_val)?;
                                                        Ok(Ast::if_else_node(control_val.val_if_else(), expr, stmt, stmt_second))
                                                    },
                                                    _ => unreachable!(),
                                                }
                                            }
                                            _ => Ok(Ast::if_node(control_val.val_if_else(), expr, stmt))
                                        }
                                    },
                                    match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num)),
                                }
                            },
                            match_token_nothing!(pos, line_num) => Err(AstError::require_left_parenth(pos, line_num)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token_ok!(TokenKind::While) => {
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::While) => {
                        match tokens.next().unwrap() {
                            match_token_ok!(TokenKind::LParen) => {
                                let expr = Ast::expr(tokens, variable_list)?;
                                match tokens.next().unwrap() {
                                    match_token_ok!(TokenKind::RParen) => {
                                        let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                        Ok(Ast::while_node(control_val.val_while(), expr, stmt))
                                    },
                                    match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num)),
                                }
                            },
                            match_token_nothing!(pos, line_num) => Err(AstError::require_left_parenth(pos, line_num)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token_ok!(TokenKind::For) => {
                let for_num = control_val.val_for();
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::For) => {
                        match tokens.next().unwrap() {
                            match_token_ok!(TokenKind::LParen) => {
                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                //        ^
                                match tokens.peek().unwrap() {
                                    match_token_ok!(TokenKind::SemiColon) => {
                                        match tokens.next().unwrap() {
                                            match_token_ok!(TokenKind::SemiColon) => {
                                            // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                            //                  ^
                                                match tokens.peek().unwrap() {
                                                    match_token_ok!(TokenKind::SemiColon) => {
                                                        match tokens.next().unwrap() {
                                                            match_token_ok!(TokenKind::SemiColon) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token_ok!(TokenKind::RParen) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token_ok!(TokenKind::RParen) => {
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
                                                                            match_token_ok!(TokenKind::RParen) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, None, None, Some(expr_third), stmt))
                                                                            },
                                                                            match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num))
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
                                                            match_token_ok!(TokenKind::SemiColon) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token_ok!(TokenKind::RParen) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token_ok!(TokenKind::RParen) => {
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
                                                                            match_token_ok!(TokenKind::RParen) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, None, Some(expr_second), Some(expr_third), stmt))
                                                                            },
                                                                            match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num))
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
                                            match_token_ok!(TokenKind::SemiColon) => {
                                            // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                            //                  ^
                                                match tokens.peek().unwrap() {
                                                    match_token_ok!(TokenKind::SemiColon) => {
                                                        match tokens.next().unwrap() {
                                                            match_token_ok!(TokenKind::SemiColon) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token_ok!(TokenKind::RParen) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token_ok!(TokenKind::RParen) => {
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
                                                                            match_token_ok!(TokenKind::RParen) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, Some(expr_first), None, Some(expr_third), stmt))
                                                                            },
                                                                            match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num))
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
                                                            match_token_ok!(TokenKind::SemiColon) => {
                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                //                            ^
                                                                match tokens.peek().unwrap() {
                                                                    match_token_ok!(TokenKind::RParen) => {
                                                                        match tokens.next().unwrap() {
                                                                            match_token_ok!(TokenKind::RParen) => {
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
                                                                            match_token_ok!(TokenKind::RParen) => {
                                                                                // "for" "(" expr? ";" expr? ";" expr? ")" stmt
                                                                                //                                      ^
                                                                                let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                                                                                Ok(Ast::for_node(for_num, Some(expr_first), Some(expr_second), Some(expr_third), stmt))
                                                                            },
                                                                            match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                }
                                            },
                                            match_token_nothing!(pos, line_num) => Err(AstError::require_semicolon(pos, line_num)),
                                        }
                                    }
                                }
                            },
                            match_token_nothing!(pos, line_num) => Err(AstError::require_left_parenth(pos, line_num)),
                        }
                    },
                    _ => unreachable!(),
                }
            },
            match_token_ok!(TokenKind::LCuryBra) => {
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::LCuryBra) => {
                        let mut res_stmt: Vec<Ast> = Vec::new();
                        loop {
                            let stmt = Ast::stmt(tokens, variable_list, control_val)?;
                            res_stmt.push(stmt);
                            match tokens.peek().unwrap() {
                                match_token_ok!(TokenKind::RCuryBra) => {
                                    match tokens.next().unwrap() {
                                        match_token_ok!(TokenKind::RCuryBra) => break,
                                        _ => unreachable!(),
                                    }
                                }
                                _ => continue,
                            }
                            // エラー処理 todo
                            // if tokens.peek().unwrap().val == TokenKind::RCuryBra {
                            //     match tokens.next().unwrap() {
                            //         match_token!(TokenKind::RCuryBra, _pos) => break,
                            //         _ => unreachable!(),
                            //     }
                            // }
                        }
                        Ok(Ast::block_node(res_stmt))
                    }
                    _ => unreachable!(),
                }
            },
            _ => {
                let expr = Ast::expr(tokens, variable_list)?;
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::SemiColon) => Ok(expr),
                    match_token_nothing!(pos, line_num) => Err(AstError::require_semicolon(pos, line_num)),
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
                    match_token_ok!(TokenKind::Substitution) => {
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
                match_token_ok!(TokenKind::Equal) | match_token_ok!(TokenKind::NotEqual) => {
                    match tokens.next().unwrap() {
                        match_token_ok!(TokenKind::Equal) => {
                            let r_ast = Ast::relational(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::Equal, l_ast, r_ast);
                        },
                        match_token_ok!(TokenKind::NotEqual) => {
                            let r_ast = Ast::relational(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::NotEqual, l_ast, r_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                _ => return Ok(l_ast)
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
                match_token_ok!(TokenKind::Small) | match_token_ok!(TokenKind::Large)
                | match_token_ok!(TokenKind::EqualSmall) | match_token_ok!(TokenKind::EqualLarge) => {
                    match tokens.next().unwrap() {
                        match_token_ok!(TokenKind::Small) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::Small, l_ast, r_ast);
                        },
                        match_token_ok!(TokenKind::Large) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            // l_ast = Ast::node(NodeKind::Large, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::Small, r_ast, l_ast);
                        },
                        match_token_ok!(TokenKind::EqualSmall) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            l_ast = Ast::node(NodeKind::EqualSmall, l_ast, r_ast);
                        },
                        match_token_ok!(TokenKind::EqualLarge) => {
                            let r_ast = Ast::add(tokens, variable_list)?;
                            // l_ast = Ast::node(NodeKind::EqualLarge, l_ast, r_ast);
                            l_ast = Ast::node(NodeKind::EqualSmall, r_ast, l_ast);
                        },
                        _ => unreachable!(),
                    }
                },
                _ => return Ok(l_ast)
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
                match_token_ok!(TokenKind::Plus) | match_token_ok!(TokenKind::Minus) => {
                    //   mul ("+" mul | "-" mul) *
                    //         ^
                    match tokens.next().unwrap() {
                        match_token_ok!(TokenKind::Plus) => {
                            //   mul ("+" mul | "-" mul) *
                            //         ^
                            let r_ast = Ast::mul(tokens, variable_list)?;
                            //   mul ("+" mul | "-" mul) *
                            //              ^
                            l_ast = Ast::node(NodeKind::Add, l_ast, r_ast);
                        },
                        match_token_ok!(TokenKind::Minus) => {
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
                _ => return Ok(l_ast)
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
                match_token_ok!(TokenKind::Asterisk) | match_token_ok!(TokenKind::Slash) => {
                    match tokens.next().unwrap() {
                        match_token_ok!(TokenKind::Asterisk) => {
                            // unary ("*" unary | "/" unary)
                            //         ^
                            let r_ast = Ast::unary(tokens, variable_list)?;
                            // unary ("*" unary | "/" unary)
                            //                ^
                            l_ast = Ast::node(NodeKind::Mul, l_ast, r_ast);
                        },
                        match_token_ok!(TokenKind::Slash) => {
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
                _ => return Ok(l_ast)
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
                    match_token_ok!(TokenKind::Plus) => return Ast::primary(tokens, variable_list),
                    //   ("+" | "-")? primary
                    //           ^
                    match_token_ok!(TokenKind::Minus) => {
                        let l_ast = Ast::num(0);
                        let r_ast = Ast::primary(tokens, variable_list)?;
                    //   ("+" | "-")? primary
                    //                      ^
                        Ok(Ast::node(NodeKind::Sub, l_ast, r_ast))
                    },
                    _ => unreachable!(),
                }
            },
            TokenKind::Asterisk | TokenKind::Ampersand => {
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::Asterisk) => {
                        let hs = Ast::primary(tokens, variable_list)?;
                        Ok(Ast::deref_node(hs))
                    },
                    match_token_ok!(TokenKind::Ampersand) => {
                        let hs = Ast::primary(tokens, variable_list)?;
                        Ok(Ast::addr_node(hs))
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
            match_token_num!(num, _pos, _line_num) => Ok(Ast::num(num)),
            match_token_ident!(str) => {
                match tokens.peek().unwrap() {
                    match_token_ok!(TokenKind::LParen) => {
                        match tokens.next().unwrap() {
                            match_token_ok!(TokenKind::LParen) => {
                                match tokens.peek().unwrap() {
                                    match_token_ok!(TokenKind::RParen) => {
                                        tokens.next();
                                        Ok(Ast::Func(str))
                                    }
                                    _ => {
                                        let mut argument_list = Vec::new();
                                        loop {
                                            let unary = Ast::unary(tokens, variable_list)?;
                                            argument_list.push(unary);
                                            match tokens.next().unwrap() {
                                                match_token_ok!(TokenKind::Comma) => continue,
                                                match_token_ok!(TokenKind::RParen) => break,
                                                match_token_nothing!(pos, line_num) => return Err(AstError::require_commma(pos, line_num)),
                                            }
                                        }
                                        Ok(Ast::call_func_node(str, argument_list))
                                    },
                                }
                            }
                            _ => unreachable!(),
                        }
                    },
                    // 変数
                    _ => {
                        let ident_str = str.clone();
                        let offset = match variable_list.get(&str) {
                            Some(&offset) => offset,
                            None => {
                                let variable_list_len = variable_list.len();
                                variable_list.insert(str, variable_list_len + 1);
                                variable_list_len + 1
                            }
                        };
                        Ok(Ast::Ident(ident_str, offset))
                    }
                }
            },
            match_token_ok!(TokenKind::LParen) => {
                // "(" epxr ")"
                //  ^
                let ex = Ast::expr(tokens, variable_list)?;
                // "(" epxr ")"
                //        ^
                match tokens.next().unwrap() {
                    match_token_ok!(TokenKind::RParen) => Ok(ex),
                    match_token_nothing!(pos, line_num) => Err(AstError::unclosed_parenth(pos, line_num)),
                }
            },
            match_token_nothing!(pos, line_num) => Err(AstError::not_pattern_matching(pos, line_num)),
        }
    }
}
