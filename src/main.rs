use std::env;
use std::process;

mod token;
mod node;
mod generator;

fn main() {
    use token::Token;
    use node::Ast;
    use generator;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./9cc <code>");
        process::exit(1);
    }

    let input = args[1].as_bytes();
    let tokens = match Token::tokenize(input) {
        Ok(tk) => tk,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    println!("{:?}", tokens);
    let mut token = tokens.into_iter().peekable();
    let asts = match Ast::program(&mut token) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let variable_num = asts.1;
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // プロローグ
    // 変数の個数はvariable_numに格納
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", variable_num * 8);
    println!("{:?}", asts.0);
    for ast in asts.0.into_iter() {
        match generator::gen(ast) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        // 最終的な値がstackに残っているため
        println!("  pop rax");
    }

    // エピローグ
    // 最後の式の値がraxに格納されており、それが返り値
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
