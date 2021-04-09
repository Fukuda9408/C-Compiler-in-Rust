use std::error;
use std::fmt;

use crate::node::{Ast, NodeKind, OneNodeKind};

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
        Ast::Ident(_, usize) => {
            // 変数のアドレスをraxに代入
            println!("; Adress Read start");
            println!("  mov rax, rbp ");
            println!("  sub rax, {}", usize * 8);
            println!("  push rax");
            println!("; Adress Read finish");
            Ok(())
        },
        _ => Err(GeneratorError::not_left_value()),
    }
}
pub fn gen(ast: Ast) -> Result<(), GeneratorError> {
    match ast {
        Ast::Num(num) => {
            println!("; Num Push");
            println!("  push {}", num);
            Ok(())
        },
        Ast::Ident(_, _) => {
            println!("; Local Variable Read start");
            gen_left_value(ast)?;
            // raxには変数のアドレスが格納
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            // 変数の値がstackに積まれる
            println!("; Local Variable Read finish");
            Ok(())
        }
        Ast::OneNode {
            node_kind,
            hs,
        } => {
            match node_kind {
                OneNodeKind::Return => {
                    println!("; Return start");
                    gen(*hs)?;
                    // stackにexprの値が積まれている
                    println!("  pop rax");
                    println!("  mov rsp, rbp");
                    println!("  pop rbp");
                    println!("  ret");
                    println!("; Return finish");
                },
                OneNodeKind::For(num) => todo!(),
            }
            Ok(())
        },
        Ast::Node {
            node_kind,
            lhs,
            rhs,
        } => {
            match node_kind {
                NodeKind::Substitution => {
                    println!("; Substitution start");
                    gen_left_value(*lhs)?;
                    gen(*rhs)?;
                    println!("  pop rdi");
                    println!("  pop rax");
                    // raxに変数のアドレス
                    // rdiに値
                    println!("  mov [rax], rdi");
                    // 右辺値がstackに積まれる
                    println!("  push rdi");
                    println!("; Substitution finish");
                },
                NodeKind::If(_) | NodeKind::IfElse | NodeKind::Else(_) | NodeKind::While(_) => {
                    match node_kind {
                        NodeKind::If(num) => {
                            println!("; If start");
                            gen(*lhs)?;
                            println!("  pop rax");      // 結果がstackに積まれている
                            println!("  cmp rax, 0");   // 偽: 0, 真: 1
                            println!("  je .Lend{}", num);
                            gen(*rhs)?;
                            println!(".Lend{}:", num);
                            println!("; If finish");
                        },
                        NodeKind::IfElse => {
                            println!("; If Else start");
                            gen(*lhs)?;
                            println!("  pop rax");      // 結果がstackに積まれている
                            println!("  cmp rax, 0");   // 偽: 0, 真: 1
                            gen(*rhs)?;         // Else Nodeを呼び出す
                        },
                        NodeKind::Else(num) => {
                            println!("  je .Lelse{}", num);
                            gen(*lhs)?;
                            println!("  jmp .Lend{}", num);
                            println!(".Lelse{}:", num);
                            gen(*rhs)?;
                            println!(".Lend{}:", num);
                            println!("; If Else finish");
                        },
                        NodeKind::While(num) => {
                            println!("; While start");
                            println!(".Lbegin{}:", num);
                            gen(*lhs)?;                 // 結果がraxに格納されている
                            println!("  cmp rax, 0");   // 偽: 0, 真: 1
                            println!("  je .Lend{}", num);
                            gen(*rhs)?;
                            println!("  jmp .Lbegin{}", num);
                            println!(".Lend{}:", num);
                            println!("; While finish");
                        }
                        _ => unreachable!(),
                    }
                },
                _ => {
                    println!("; Arithmetic start");
                    gen(*lhs)?;
                    gen(*rhs)?;
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
                        NodeKind::Substitution => unreachable!(),
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
                        NodeKind::If(_) => {},
                        NodeKind::IfElse => unreachable!(),
                        NodeKind::Else(_) => unreachable!(),
                        NodeKind::While(_) => unreachable!(),
                    }
                    // 計算結果がstackに積まれる
                    println!("  push rax");
                    println!("; Arithmetic finish");
                }
            }
            Ok(())
        },
        Ast::ForNode {
            for_num,
            initial,
            condition,
            change,
            stmt,
        }=> {
            println!("; For start");
            if let Some(Ast::OneNode {
                node_kind: _,
                hs,
            }) = *initial{
                gen(*hs)?;
            }
            println!(".Lbegin{}:", for_num);
            if let Some(Ast::OneNode {
                node_kind: _,
                hs,
            }) = *condition {
                gen(*hs)?;                  // 結果はraxに格納されている
                println!("  cmp rax, 0");
            }
            println!("  je .Lend{}", for_num);
            if let Ast::OneNode {
                node_kind: _,
                hs,
            } = *stmt {
                gen(*hs)?;
            }
            if let Some(Ast::OneNode {
                node_kind: _,
                hs,
            }) = *change{
                gen(*hs)?;
            }
            println!("  jmp .Lbegin{}", for_num);
            println!(".Lend{}:", for_num);
            println!("; For finish");
            Ok(())
        }
    }
}
