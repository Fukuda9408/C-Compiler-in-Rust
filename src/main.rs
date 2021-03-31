use std::env;
use std::process;
use std::error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./9cc <code>");
        process::exit(1);
    }

    let input = args[1].as_bytes();
    let mut pos = 0;
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // 最初の数字を取得
    let (num, new_pos) = get_number(input, pos).unwrap();
    pos = new_pos;
    println!("  mov rax, {}", num);
    while pos < input.len() {
        match input[pos] {
            b'+' => {
                pos += 1;
                let (num, new_pos) = get_number(input, pos).unwrap_or_else(|err| {
                    eprintln!("Parse Error: {}", err);
                    process::exit(1);
                });
                pos = new_pos;
                println!("  add rax, {}", num);
            },
            b'-' => {
                pos += 1;
                let (num, new_pos) = get_number(input, pos).unwrap_or_else(|err| {
                    eprintln!("Parse Error: {}", err);
                    process::exit(1);
                });
                pos = new_pos;
                println!("  sub rax, {}", num);
            }
            _ => eprintln!("Unexpected Character: {}", input[pos]),
        }
    }
    println!("  ret")
}

fn get_number(input: &[u8], mut pos: usize) -> Result<(i32, usize) , Box<dyn error::Error>> {
    let start = pos;
    while pos < input.len() {
        match input[pos] {
            b'0'..=b'9' => pos += 1,
            _ => break,
        }
    }
    let str = String::from_utf8(input[start..pos].to_vec())?;
    let num: i32 = str.parse()?;

    Ok((num, pos))
}
