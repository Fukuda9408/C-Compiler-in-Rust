use std::env;
use std::process;
use std::error;

mod token;

fn main() {
    use token::TokenKind;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./9cc <code>");
        process::exit(1);
    }

    let input = args[1].as_bytes();
    let tokens = token::Token::tokenize(input).unwrap();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut token = tokens.into_iter();
    match token.next().unwrap().val {
        TokenKind::Num(num) => {
            println!("  mov rax, {}", num);
        },
        _ => {
            eprintln!("Not number");
            process::exit(1);
        }
    }

    loop {
        match token.next().unwrap().val {
            TokenKind::Plus => {
                match token.next().unwrap().val {
                    TokenKind::Num(num) => {
                        println!("  add rax, {}", num);
                    },
                    _ => {
                        eprintln!("Not Number");
                        process::exit(1);
                    },
                }
            },
            TokenKind::Minus => {
                match token.next().unwrap().val {
                    TokenKind::Num(num) => {
                        println!("  sub rax, {}", num);
                    },
                    _ => {
                        eprintln!("Not Number");
                        process::exit(1);
                    },
                }
            },
            TokenKind::EOF => break,
            _ => {
                eprintln!("Not Number");
                process::exit(1);
            },
        }
    }
    println!("  ret")
}
