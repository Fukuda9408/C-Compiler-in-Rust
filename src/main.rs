use std::env;
use std::process;

mod token;
mod node;

fn main() {
    use token::Token;
    use node::Ast;
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
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut token = tokens.into_iter().peekable();
    let ast = match Ast::expr(&mut token) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    Ast::gen(ast);

    println!("  pop rax");
    println!("  ret");
}
