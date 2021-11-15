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
        Ast::Ident(_, usize) => {
            // 変数のアドレスをraxに代入
            println!("# Adress Read start");
            println!("  mov rax, rbp ");
            println!("  sub rax, {}", usize * 8);
            println!("  push rax");
            println!("# Adress Read finish");
            Ok(())
        },
        _ => Err(GeneratorError::not_left_value()),
    }
}
pub fn gen(ast: Ast) -> Result<(), GeneratorError> {
    match ast {
        Ast::Num(num) => {
            println!("# Num Push");
            println!("  push {}", num);
            Ok(())
        },
        Ast::Ident(_, _) => {
            println!("# Local Variable Read start");
            gen_left_value(ast)?;
            // raxには変数のアドレスが格納
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            // 変数の値がstackに積まれる
            println!("# Local Variable Read finish");
            Ok(())
        },
        Ast::ReturnNode {
            hs,
        } => {
            println!("# Return start");
            gen(*hs)?;
            // stackにexprの値が積まれている
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            println!("# Return finish");
            Ok(())
        },
        Ast::BlockNode {
            hs,
        } => {
            for stmt in (*hs).into_iter() {
                gen(stmt)?;
            }
            Ok(())
        }
        Ast::AddrNode {
            hs,
        } => {
            gen_left_value(*hs)?;       // 変数のアドレスがstackにpush
            Ok(())
        }
        // *4はアドレス4から値を読みだすことになるので注意
        Ast::DerefNode {
            hs
        } => {
            gen(*hs)?;                  // 変数の値(addr)がstackにpush ->
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            Ok(())
        }
        Ast::Node {
            node_kind,
            lhs,
            rhs,
        } => {
            match node_kind {
                NodeKind::Substitution => {
                    println!("# Substitution start");
                    gen_left_value(*lhs)?;
                    gen(*rhs)?;
                    println!("  pop rdi");
                    println!("  pop rax");
                    // raxに変数のアドレス
                    // rdiに値
                    println!("  mov [rax], rdi");
                    // 右辺値がstackに積まれる
                    // println!("  push rdi");
                    println!("# Substitution finish");
                },
                _ => {
                    println!("# Arithmetic start");
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
                    }
                    // 計算結果がstackに積まれる
                    println!("  push rax");
                    println!("# Arithmetic finish");
                }
            }
            Ok(())
        },
        Ast::IfNode {
            for_num,
            condition,
            stmt,
        } => {
            println!("# If start");
            gen(*condition)?;
            println!("  pop rax");      // 結果がstackに積まれている
            println!("  cmp rax, 0");   // 偽: 0, 真: 1
            println!("  je .Lend{}", for_num);
            gen(*stmt)?;
            println!(".Lend{}:", for_num);
            println!("# If finish");
            Ok(())
        },
        Ast::IfElseNode {
            for_num,
            condition,
            stmt_1,
            stmt_2,
        } => {
            println!("# If Else start");
            gen(*condition)?;
            println!("  pop rax");      // 結果がstackに積まれている
            println!("  cmp rax, 0");   // 偽: 0, 真: 1
            println!("  je .Lelse{}", for_num);
            gen(*stmt_1)?;
            println!("  jmp .Lend{}", for_num);
            println!(".Lelse{}:", for_num);
            gen(*stmt_2)?;
            println!(".Lend{}:", for_num);
            println!("# If Else finish");
            Ok(())
        },
        Ast::WhileNode {
            for_num,
            condition,
            stmt,
        } => {
            println!("# While start");
            println!(".Lbegin{}:", for_num);
            gen(*condition)?;
            println!("  pop rax");      // 結果がraxに格納されている
            println!("  cmp rax, 0");   // 偽: 0, 真: 1
            println!("  je .Lend{}", for_num);
            gen(*stmt)?;
            println!("  jmp .Lbegin{}", for_num);
            println!(".Lend{}:", for_num);
            println!("# While finish");
            Ok(())
        }
        Ast::ForNode {
            for_num,
            initial,
            condition,
            change,
            stmt,
        }=> {
            // exprの結果は使用しないためpopする
            println!("# For start");
            if let Some(expr_first) = *initial {
                gen(expr_first)?;
                // println!("  pop rax");      // 結果はraxに格納されている
            };
            println!(".Lbegin{}:", for_num);
            if let Some(expr_second) = *condition {
                gen(expr_second)?;
                println!("  pop rax");      // 結果はraxに格納されている
                println!("  cmp rax, 0");
            }
            println!("  je .Lend{}", for_num);
            gen(*stmt)?;
            if let Some(expr_third) = *change{
                gen(expr_third)?;
                // println!("  pop rax");      // 結果はraxに格納されている
            }
            println!("  jmp .Lbegin{}", for_num);
            println!(".Lend{}:", for_num);
            println!("# For finish");
            Ok(())
        },
        Ast::Func(ident) => {
            println!("  call {}", ident);
            Ok(())
        },
        Ast::CallFuncNode {
            func_name,
            hs,
        } => {
            for (i, num) in (*hs).into_iter().enumerate() {
                gen(num)?;
                match i {
                    0 => println!("  pop rdi"),
                    1 => println!("  pop rsi"),
                    2 => println!("  pop rdx"),
                    3 => println!("  pop rcx"),
                    4 => println!("  pop r8"),
                    5 => println!("  pop r9"),
                    _ => unreachable!(),
                }
            }
            println!("  call {}", func_name);
            println!(" push rax");      // 関数の結果を代入する際に結果がstackに積まれている前提で行われるため
            Ok(())
        }
        Ast::FuncNode {
            argument_num,
            local_variable_num,
            func_name,
            stmt_block,
        } => {
            println!("{}:", func_name);
            // プロローグ
            // 変数の個数はlocal_variable_numに格
            println!("  push rbp");
            println!("  mov rbp, rsp");
            // ローカル変数の定義
            println!("  sub rsp, {}", local_variable_num * 8);
            // -------------
            //    r9
            // -------------
            //    r8
            // -------------
            //    rcx
            // -------------
            //    rdx
            // -------------
            //    rsi
            // -------------
            //    rdi
            // -------------   <-    今のrbp
            //    前のrbp
            // -------------
            // return addrss
            //
            for i in 0..argument_num {
                match i {
                    0 => println!("  mov [rbp - 0x08], rdi"),
                    1 => println!("  mov [rbp - 0x10], rsi"),
                    2 => println!("  mov [rbp - 0x18], rdx"),
                    3 => println!("  mov [rbp - 0x20], rcx"),
                    4 => println!("  mov [rbp - 0x28], r8"),
                    5 => println!("  mov [rbp - 0x30], r9"),
                    _ => unreachable!(),
                }
            }
            gen(*stmt_block)?;
            // エピローグ
            // 最後の式の値がraxに格納されており、それが返り値
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            Ok(())
        },
    }
}
