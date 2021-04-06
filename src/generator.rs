use std::error;
use std::fmt;

use crate::node::{Ast, NodeKind};

#[derive(Debug, Clone, Copy)]
enum GeneratorErrorKind {
    NotLeftValue,
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratorError {
    val: GeneratorErrorKind
}

impl GeneratorError {
    fn new(val: GeneratorErrorKind) -> Self {
        GeneratorError {
            val
        }
    }

    fn not_left_value() -> Self {
        Self::new(GeneratorErrorKind::NotLeftValue)
    }
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GeneratorErrorKind::*;
        match self.val {
            NotLeftValue => write!(f, "Not Left Value"),
        }
    }
}

impl error::Error for GeneratorError {}

fn gen_left_value(ast: Ast) -> Result<(), GeneratorError>{
    match ast {
        Ast::Ident(String, usize) => {
            // 変数のアドレスをraxに代入
            println!("  mov rax, rbp");
            println!("  sub rax, {}", (usize + 1) * 8);
            println!("  push rax");
            Ok(())
        },
        _ => Err(GeneratorError::not_left_value()),
    }
}
pub fn gen(ast: Ast) -> Result<(), GeneratorError> {
    match ast {
        Ast::Num(num) => {
            println!("  push {}", num);
            Ok(())
        },
        Ast::Ident(_, _) => {
            gen_left_value(ast)?;
            // raxには変数のアドレスが格納
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            // 変数の値がstackに積まれる
            Ok(())
        }
        Ast::Node {
            node_kind,
            lhs,
            rhs,
        } => {
            match node_kind {
                NodeKind::Substitution => {
                    gen_left_value(*lhs)?;
                    gen(*rhs)?;
                },
                _ => {
                    gen(*lhs)?;
                    gen(*rhs)?;
                }
            }
            println!("  pop rdi");
            println!("  pop rax");
            match node_kind {
                NodeKind::Add => println!("  add rax, rdi"),
                NodeKind::Sub => println!("  sub rax, rdi"),
                NodeKind::Mul => println!("  imul rax, rdi"),
                NodeKind::Div => {
                    println!("  cqo");
                    println!("  idiv rdi");
                },
                NodeKind::Substitution => {
                    // raxに変数のアドレス
                    // rdiに値
                    println!("  mov [rax], rdi");
                },
                // 比較演算子では真なら1, 偽なら0が
                // raxに格納されstackに積まれる
                NodeKind::Small => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al");
                },
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
            match node_kind {
                // 代入の時は右辺値がstackに積まれる
                NodeKind::Substitution => println!("  push rdi"),
                // それ以外は計算結果がstackに積まれる
                _ =>  println!("  push rax"),
            }
            Ok(())
        }
    }
}
