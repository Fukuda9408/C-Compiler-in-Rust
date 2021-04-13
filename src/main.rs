use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod token;
mod node;
mod generator;

fn main() {
    use token::{Token, TokenKind};
    // use node::Ast;

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./9cc <code>");
        process::exit(1);
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut program = Vec::new();

    let mut last_line_len = 0;
    let mut last_line_num = 0;
    for (line_num, result) in BufReader::new(File::open(args[1].clone()).unwrap()).lines().enumerate() {
        let l = result.unwrap();
        // 後でprogramを参照するために1行ずつStringをpush
        program.push(l.clone());

        let input = l.as_bytes();
        // 最後にEOFをtokensにpushするために必要な情報
        last_line_len = input.len();
        last_line_num = line_num;

        match Token::tokenize(input, line_num) {
            Ok(mut tk) => tokens.append(&mut tk),
            Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
            }
        }
    }
    tokens.push(Token::new(TokenKind::EOF, token::Location(last_line_len, last_line_len), last_line_num));
    println!("{:?}", tokens);
    // let mut token = tokens.into_iter().peekable();
    // let asts = match Ast::program(&mut token) {
    //     Ok(ast) => ast,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         process::exit(1);
    //     }
    // };
    // println!(".intel_syntax noprefix");
    // println!(".global main");

    // for ast in asts.into_iter() {
    //     match generator::gen(ast) {
    //         Ok(_) => println!(""),
    //         Err(e) => {
    //             eprintln!("{}", e);
    //             process::exit(1);
    //         }
    //     }
    // }
    // Ok(())
}
