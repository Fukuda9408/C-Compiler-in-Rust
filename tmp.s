.intel_syntax noprefix
.global main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 24
# Substitution start
# Adress Read start
  mov rax, rbp 
  sub rax, 8
  push rax
# Adress Read finish
# Num Push
  push 0
  pop rdi
  pop rax
  mov [rax], rdi
# Substitution finish
# For start
# Substitution start
# Adress Read start
  mov rax, rbp 
  sub rax, 16
  push rax
# Adress Read finish
# Num Push
  push 0
  pop rdi
  pop rax
  mov [rax], rdi
# Substitution finish
.Lbegin0:
# Arithmetic start
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 16
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
# Num Push
  push 4
  pop rdi
  pop rax
  cmp rax, rdi
  setl al
  movzb rax, al
  push rax
# Arithmetic finish
  pop rax
  cmp rax, 0
  je .Lend0
# For start
# Substitution start
# Adress Read start
  mov rax, rbp 
  sub rax, 24
  push rax
# Adress Read finish
# Num Push
  push 0
  pop rdi
  pop rax
  mov [rax], rdi
# Substitution finish
.Lbegin1:
# Arithmetic start
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 24
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
# Num Push
  push 4
  pop rdi
  pop rax
  cmp rax, rdi
  setl al
  movzb rax, al
  push rax
# Arithmetic finish
  pop rax
  cmp rax, 0
  je .Lend1
# Substitution start
# Adress Read start
  mov rax, rbp 
  sub rax, 8
  push rax
# Adress Read finish
# Arithmetic start
# Arithmetic start
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 8
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 16
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
  pop rdi
  pop rax
  add rax, rdi
  push rax
# Arithmetic finish
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 24
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
  pop rdi
  pop rax
  add rax, rdi
  push rax
# Arithmetic finish
  pop rdi
  pop rax
  mov [rax], rdi
# Substitution finish
# Substitution start
# Adress Read start
  mov rax, rbp 
  sub rax, 24
  push rax
# Adress Read finish
# Arithmetic start
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 24
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
# Num Push
  push 1
  pop rdi
  pop rax
  add rax, rdi
  push rax
# Arithmetic finish
  pop rdi
  pop rax
  mov [rax], rdi
# Substitution finish
  jmp .Lbegin1
.Lend1:
# For finish
# Substitution start
# Adress Read start
  mov rax, rbp 
  sub rax, 16
  push rax
# Adress Read finish
# Arithmetic start
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 16
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
# Num Push
  push 1
  pop rdi
  pop rax
  add rax, rdi
  push rax
# Arithmetic finish
  pop rdi
  pop rax
  mov [rax], rdi
# Substitution finish
  jmp .Lbegin0
.Lend0:
# For finish
# Return start
# Local Variable Read start
# Adress Read start
  mov rax, rbp 
  sub rax, 8
  push rax
# Adress Read finish
  pop rax
  mov rax, [rax]
  push rax
# Local Variable Read finish
  pop rax
  mov rsp, rbp
  pop rbp
  ret
# Return finish
  mov rsp, rbp
  pop rbp
  ret

