use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod token;
mod node;
mod generator;

use token::{Token, TokenKind};
use node::Ast;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./9cc <code>");
        process::exit(1);
    }

    let mut tokens: Vec<Token> = Vec::new();

    for (line_num, code) in BufReader::new(File::open(&args[1]).unwrap()).lines().enumerate() {
        let input  = code.unwrap();

        match Token::tokenize(&input.as_bytes(), line_num) {
            Ok(mut tk) => tokens.append(&mut tk),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
    let eof_pos = tokens.last().unwrap().pos.1 + 1;
    let eof_line_num = tokens.last().unwrap().line_num;
    tokens.push(Token::new(TokenKind::EOF, token::Location(eof_pos, eof_pos), eof_line_num));

    let mut token = tokens.into_iter().peekable();
    let asts = match Ast::program(&mut token) {
        Ok(ast) => ast,
        Err(e) => {
            println!("{}", e);
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    println!(".intel_syntax noprefix");
    println!(".global main");

    for ast in asts.into_iter() {
        match generator::gen(ast) {
            Ok(_) => println!(""),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}
