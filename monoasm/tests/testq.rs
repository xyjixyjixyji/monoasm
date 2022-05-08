  extern crate monoasm;
  extern crate monoasm_macro;
  use std::io::Write;

  use monoasm::*;
  use monoasm_macro::monoasm;

  #[test]
  fn testq() {
      let mut jit: JitMemory = JitMemory::new();
      monoasm!(
          jit,
	testq rax, rax;
	testq rax, rcx;
	testq rax, rdx;
	testq rax, rbx;
	testq rax, rsp;
	testq rax, rbp;
	testq rax, rsi;
	testq rax, rdi;
	testq rax, r8;
	testq rax, r9;
	testq rax, r10;
	testq rax, r11;
	testq rax, r12;
	testq rax, r13;
	testq rax, r14;
	testq rax, r15;
	testq rcx, rax;
	testq rcx, rcx;
	testq rcx, rdx;
	testq rcx, rbx;
	testq rcx, rsp;
	testq rcx, rbp;
	testq rcx, rsi;
	testq rcx, rdi;
	testq rcx, r8;
	testq rcx, r9;
	testq rcx, r10;
	testq rcx, r11;
	testq rcx, r12;
	testq rcx, r13;
	testq rcx, r14;
	testq rcx, r15;
	testq rdx, rax;
	testq rdx, rcx;
	testq rdx, rdx;
	testq rdx, rbx;
	testq rdx, rsp;
	testq rdx, rbp;
	testq rdx, rsi;
	testq rdx, rdi;
	testq rdx, r8;
	testq rdx, r9;
	testq rdx, r10;
	testq rdx, r11;
	testq rdx, r12;
	testq rdx, r13;
	testq rdx, r14;
	testq rdx, r15;
	testq rbx, rax;
	testq rbx, rcx;
	testq rbx, rdx;
	testq rbx, rbx;
	testq rbx, rsp;
	testq rbx, rbp;
	testq rbx, rsi;
	testq rbx, rdi;
	testq rbx, r8;
	testq rbx, r9;
	testq rbx, r10;
	testq rbx, r11;
	testq rbx, r12;
	testq rbx, r13;
	testq rbx, r14;
	testq rbx, r15;
	testq rsp, rax;
	testq rsp, rcx;
	testq rsp, rdx;
	testq rsp, rbx;
	testq rsp, rsp;
	testq rsp, rbp;
	testq rsp, rsi;
	testq rsp, rdi;
	testq rsp, r8;
	testq rsp, r9;
	testq rsp, r10;
	testq rsp, r11;
	testq rsp, r12;
	testq rsp, r13;
	testq rsp, r14;
	testq rsp, r15;
	testq rbp, rax;
	testq rbp, rcx;
	testq rbp, rdx;
	testq rbp, rbx;
	testq rbp, rsp;
	testq rbp, rbp;
	testq rbp, rsi;
	testq rbp, rdi;
	testq rbp, r8;
	testq rbp, r9;
	testq rbp, r10;
	testq rbp, r11;
	testq rbp, r12;
	testq rbp, r13;
	testq rbp, r14;
	testq rbp, r15;
	testq rsi, rax;
	testq rsi, rcx;
	testq rsi, rdx;
	testq rsi, rbx;
	testq rsi, rsp;
	testq rsi, rbp;
	testq rsi, rsi;
	testq rsi, rdi;
	testq rsi, r8;
	testq rsi, r9;
	testq rsi, r10;
	testq rsi, r11;
	testq rsi, r12;
	testq rsi, r13;
	testq rsi, r14;
	testq rsi, r15;
	testq rdi, rax;
	testq rdi, rcx;
	testq rdi, rdx;
	testq rdi, rbx;
	testq rdi, rsp;
	testq rdi, rbp;
	testq rdi, rsi;
	testq rdi, rdi;
	testq rdi, r8;
	testq rdi, r9;
	testq rdi, r10;
	testq rdi, r11;
	testq rdi, r12;
	testq rdi, r13;
	testq rdi, r14;
	testq rdi, r15;
	testq r8, rax;
	testq r8, rcx;
	testq r8, rdx;
	testq r8, rbx;
	testq r8, rsp;
	testq r8, rbp;
	testq r8, rsi;
	testq r8, rdi;
	testq r8, r8;
	testq r8, r9;
	testq r8, r10;
	testq r8, r11;
	testq r8, r12;
	testq r8, r13;
	testq r8, r14;
	testq r8, r15;
	testq r9, rax;
	testq r9, rcx;
	testq r9, rdx;
	testq r9, rbx;
	testq r9, rsp;
	testq r9, rbp;
	testq r9, rsi;
	testq r9, rdi;
	testq r9, r8;
	testq r9, r9;
	testq r9, r10;
	testq r9, r11;
	testq r9, r12;
	testq r9, r13;
	testq r9, r14;
	testq r9, r15;
	testq r10, rax;
	testq r10, rcx;
	testq r10, rdx;
	testq r10, rbx;
	testq r10, rsp;
	testq r10, rbp;
	testq r10, rsi;
	testq r10, rdi;
	testq r10, r8;
	testq r10, r9;
	testq r10, r10;
	testq r10, r11;
	testq r10, r12;
	testq r10, r13;
	testq r10, r14;
	testq r10, r15;
	testq r11, rax;
	testq r11, rcx;
	testq r11, rdx;
	testq r11, rbx;
	testq r11, rsp;
	testq r11, rbp;
	testq r11, rsi;
	testq r11, rdi;
	testq r11, r8;
	testq r11, r9;
	testq r11, r10;
	testq r11, r11;
	testq r11, r12;
	testq r11, r13;
	testq r11, r14;
	testq r11, r15;
	testq r12, rax;
	testq r12, rcx;
	testq r12, rdx;
	testq r12, rbx;
	testq r12, rsp;
	testq r12, rbp;
	testq r12, rsi;
	testq r12, rdi;
	testq r12, r8;
	testq r12, r9;
	testq r12, r10;
	testq r12, r11;
	testq r12, r12;
	testq r12, r13;
	testq r12, r14;
	testq r12, r15;
	testq r13, rax;
	testq r13, rcx;
	testq r13, rdx;
	testq r13, rbx;
	testq r13, rsp;
	testq r13, rbp;
	testq r13, rsi;
	testq r13, rdi;
	testq r13, r8;
	testq r13, r9;
	testq r13, r10;
	testq r13, r11;
	testq r13, r12;
	testq r13, r13;
	testq r13, r14;
	testq r13, r15;
	testq r14, rax;
	testq r14, rcx;
	testq r14, rdx;
	testq r14, rbx;
	testq r14, rsp;
	testq r14, rbp;
	testq r14, rsi;
	testq r14, rdi;
	testq r14, r8;
	testq r14, r9;
	testq r14, r10;
	testq r14, r11;
	testq r14, r12;
	testq r14, r13;
	testq r14, r14;
	testq r14, r15;
	testq r15, rax;
	testq r15, rcx;
	testq r15, rdx;
	testq r15, rbx;
	testq r15, rsp;
	testq r15, rbp;
	testq r15, rsi;
	testq r15, rdi;
	testq r15, r8;
	testq r15, r9;
	testq r15, r10;
	testq r15, r11;
	testq r15, r12;
	testq r15, r13;
	testq r15, r14;
	testq r15, r15;
	testq rax, 18;
	testq rcx, 18;
	testq rdx, 18;
	testq rbx, 18;
	testq rsp, 18;
	testq rbp, 18;
	testq rsi, 18;
	testq rdi, 18;
	testq r8, 18;
	testq r9, 18;
	testq r10, 18;
	testq r11, 18;
	testq r12, 18;
	testq r13, 18;
	testq r14, 18;
	testq r15, 18;
	testq [rax], 18;
	testq [rax + 16], 18;
	testq [rax + 512], 18;
	testq [rcx], 18;
	testq [rcx + 16], 18;
	testq [rcx + 512], 18;
	testq [rdx], 18;
	testq [rdx + 16], 18;
	testq [rdx + 512], 18;
	testq [rbx], 18;
	testq [rbx + 16], 18;
	testq [rbx + 512], 18;
	testq [rsp], 18;
	testq [rsp + 16], 18;
	testq [rsp + 512], 18;
	testq [rbp], 18;
	testq [rbp + 16], 18;
	testq [rbp + 512], 18;
	testq [rsi], 18;
	testq [rsi + 16], 18;
	testq [rsi + 512], 18;
	testq [rdi], 18;
	testq [rdi + 16], 18;
	testq [rdi + 512], 18;
	testq [r8], 18;
	testq [r8 + 16], 18;
	testq [r8 + 512], 18;
	testq [r9], 18;
	testq [r9 + 16], 18;
	testq [r9 + 512], 18;
	testq [r10], 18;
	testq [r10 + 16], 18;
	testq [r10 + 512], 18;
	testq [r11], 18;
	testq [r11 + 16], 18;
	testq [r11 + 512], 18;
	testq [r12], 18;
	testq [r12 + 16], 18;
	testq [r12 + 512], 18;
	testq [r13], 18;
	testq [r13 + 16], 18;
	testq [r13 + 512], 18;
	testq [r14], 18;
	testq [r14 + 16], 18;
	testq [r14 + 512], 18;
	testq [r15], 18;
	testq [r15 + 16], 18;
	testq [r15 + 512], 18;
	testq [rip], 18;
	testq [rip + 16], 18;
	testq [rip + 512], 18;
	testq [rax], rax;
	testq [rax], rcx;
	testq [rax], rdx;
	testq [rax], rbx;
	testq [rax], rsp;
	testq [rax], rbp;
	testq [rax], rsi;
	testq [rax], rdi;
	testq [rax], r8;
	testq [rax], r9;
	testq [rax], r10;
	testq [rax], r11;
	testq [rax], r12;
	testq [rax], r13;
	testq [rax], r14;
	testq [rax], r15;
	testq [rax + 16], rax;
	testq [rax + 16], rcx;
	testq [rax + 16], rdx;
	testq [rax + 16], rbx;
	testq [rax + 16], rsp;
	testq [rax + 16], rbp;
	testq [rax + 16], rsi;
	testq [rax + 16], rdi;
	testq [rax + 16], r8;
	testq [rax + 16], r9;
	testq [rax + 16], r10;
	testq [rax + 16], r11;
	testq [rax + 16], r12;
	testq [rax + 16], r13;
	testq [rax + 16], r14;
	testq [rax + 16], r15;
	testq [rax + 512], rax;
	testq [rax + 512], rcx;
	testq [rax + 512], rdx;
	testq [rax + 512], rbx;
	testq [rax + 512], rsp;
	testq [rax + 512], rbp;
	testq [rax + 512], rsi;
	testq [rax + 512], rdi;
	testq [rax + 512], r8;
	testq [rax + 512], r9;
	testq [rax + 512], r10;
	testq [rax + 512], r11;
	testq [rax + 512], r12;
	testq [rax + 512], r13;
	testq [rax + 512], r14;
	testq [rax + 512], r15;
	testq [rcx], rax;
	testq [rcx], rcx;
	testq [rcx], rdx;
	testq [rcx], rbx;
	testq [rcx], rsp;
	testq [rcx], rbp;
	testq [rcx], rsi;
	testq [rcx], rdi;
	testq [rcx], r8;
	testq [rcx], r9;
	testq [rcx], r10;
	testq [rcx], r11;
	testq [rcx], r12;
	testq [rcx], r13;
	testq [rcx], r14;
	testq [rcx], r15;
	testq [rcx + 16], rax;
	testq [rcx + 16], rcx;
	testq [rcx + 16], rdx;
	testq [rcx + 16], rbx;
	testq [rcx + 16], rsp;
	testq [rcx + 16], rbp;
	testq [rcx + 16], rsi;
	testq [rcx + 16], rdi;
	testq [rcx + 16], r8;
	testq [rcx + 16], r9;
	testq [rcx + 16], r10;
	testq [rcx + 16], r11;
	testq [rcx + 16], r12;
	testq [rcx + 16], r13;
	testq [rcx + 16], r14;
	testq [rcx + 16], r15;
	testq [rcx + 512], rax;
	testq [rcx + 512], rcx;
	testq [rcx + 512], rdx;
	testq [rcx + 512], rbx;
	testq [rcx + 512], rsp;
	testq [rcx + 512], rbp;
	testq [rcx + 512], rsi;
	testq [rcx + 512], rdi;
	testq [rcx + 512], r8;
	testq [rcx + 512], r9;
	testq [rcx + 512], r10;
	testq [rcx + 512], r11;
	testq [rcx + 512], r12;
	testq [rcx + 512], r13;
	testq [rcx + 512], r14;
	testq [rcx + 512], r15;
	testq [rdx], rax;
	testq [rdx], rcx;
	testq [rdx], rdx;
	testq [rdx], rbx;
	testq [rdx], rsp;
	testq [rdx], rbp;
	testq [rdx], rsi;
	testq [rdx], rdi;
	testq [rdx], r8;
	testq [rdx], r9;
	testq [rdx], r10;
	testq [rdx], r11;
	testq [rdx], r12;
	testq [rdx], r13;
	testq [rdx], r14;
	testq [rdx], r15;
	testq [rdx + 16], rax;
	testq [rdx + 16], rcx;
	testq [rdx + 16], rdx;
	testq [rdx + 16], rbx;
	testq [rdx + 16], rsp;
	testq [rdx + 16], rbp;
	testq [rdx + 16], rsi;
	testq [rdx + 16], rdi;
	testq [rdx + 16], r8;
	testq [rdx + 16], r9;
	testq [rdx + 16], r10;
	testq [rdx + 16], r11;
	testq [rdx + 16], r12;
	testq [rdx + 16], r13;
	testq [rdx + 16], r14;
	testq [rdx + 16], r15;
	testq [rdx + 512], rax;
	testq [rdx + 512], rcx;
	testq [rdx + 512], rdx;
	testq [rdx + 512], rbx;
	testq [rdx + 512], rsp;
	testq [rdx + 512], rbp;
	testq [rdx + 512], rsi;
	testq [rdx + 512], rdi;
	testq [rdx + 512], r8;
	testq [rdx + 512], r9;
	testq [rdx + 512], r10;
	testq [rdx + 512], r11;
	testq [rdx + 512], r12;
	testq [rdx + 512], r13;
	testq [rdx + 512], r14;
	testq [rdx + 512], r15;
	testq [rbx], rax;
	testq [rbx], rcx;
	testq [rbx], rdx;
	testq [rbx], rbx;
	testq [rbx], rsp;
	testq [rbx], rbp;
	testq [rbx], rsi;
	testq [rbx], rdi;
	testq [rbx], r8;
	testq [rbx], r9;
	testq [rbx], r10;
	testq [rbx], r11;
	testq [rbx], r12;
	testq [rbx], r13;
	testq [rbx], r14;
	testq [rbx], r15;
	testq [rbx + 16], rax;
	testq [rbx + 16], rcx;
	testq [rbx + 16], rdx;
	testq [rbx + 16], rbx;
	testq [rbx + 16], rsp;
	testq [rbx + 16], rbp;
	testq [rbx + 16], rsi;
	testq [rbx + 16], rdi;
	testq [rbx + 16], r8;
	testq [rbx + 16], r9;
	testq [rbx + 16], r10;
	testq [rbx + 16], r11;
	testq [rbx + 16], r12;
	testq [rbx + 16], r13;
	testq [rbx + 16], r14;
	testq [rbx + 16], r15;
	testq [rbx + 512], rax;
	testq [rbx + 512], rcx;
	testq [rbx + 512], rdx;
	testq [rbx + 512], rbx;
	testq [rbx + 512], rsp;
	testq [rbx + 512], rbp;
	testq [rbx + 512], rsi;
	testq [rbx + 512], rdi;
	testq [rbx + 512], r8;
	testq [rbx + 512], r9;
	testq [rbx + 512], r10;
	testq [rbx + 512], r11;
	testq [rbx + 512], r12;
	testq [rbx + 512], r13;
	testq [rbx + 512], r14;
	testq [rbx + 512], r15;
	testq [rsp], rax;
	testq [rsp], rcx;
	testq [rsp], rdx;
	testq [rsp], rbx;
	testq [rsp], rsp;
	testq [rsp], rbp;
	testq [rsp], rsi;
	testq [rsp], rdi;
	testq [rsp], r8;
	testq [rsp], r9;
	testq [rsp], r10;
	testq [rsp], r11;
	testq [rsp], r12;
	testq [rsp], r13;
	testq [rsp], r14;
	testq [rsp], r15;
	testq [rsp + 16], rax;
	testq [rsp + 16], rcx;
	testq [rsp + 16], rdx;
	testq [rsp + 16], rbx;
	testq [rsp + 16], rsp;
	testq [rsp + 16], rbp;
	testq [rsp + 16], rsi;
	testq [rsp + 16], rdi;
	testq [rsp + 16], r8;
	testq [rsp + 16], r9;
	testq [rsp + 16], r10;
	testq [rsp + 16], r11;
	testq [rsp + 16], r12;
	testq [rsp + 16], r13;
	testq [rsp + 16], r14;
	testq [rsp + 16], r15;
	testq [rsp + 512], rax;
	testq [rsp + 512], rcx;
	testq [rsp + 512], rdx;
	testq [rsp + 512], rbx;
	testq [rsp + 512], rsp;
	testq [rsp + 512], rbp;
	testq [rsp + 512], rsi;
	testq [rsp + 512], rdi;
	testq [rsp + 512], r8;
	testq [rsp + 512], r9;
	testq [rsp + 512], r10;
	testq [rsp + 512], r11;
	testq [rsp + 512], r12;
	testq [rsp + 512], r13;
	testq [rsp + 512], r14;
	testq [rsp + 512], r15;
	testq [rbp], rax;
	testq [rbp], rcx;
	testq [rbp], rdx;
	testq [rbp], rbx;
	testq [rbp], rsp;
	testq [rbp], rbp;
	testq [rbp], rsi;
	testq [rbp], rdi;
	testq [rbp], r8;
	testq [rbp], r9;
	testq [rbp], r10;
	testq [rbp], r11;
	testq [rbp], r12;
	testq [rbp], r13;
	testq [rbp], r14;
	testq [rbp], r15;
	testq [rbp + 16], rax;
	testq [rbp + 16], rcx;
	testq [rbp + 16], rdx;
	testq [rbp + 16], rbx;
	testq [rbp + 16], rsp;
	testq [rbp + 16], rbp;
	testq [rbp + 16], rsi;
	testq [rbp + 16], rdi;
	testq [rbp + 16], r8;
	testq [rbp + 16], r9;
	testq [rbp + 16], r10;
	testq [rbp + 16], r11;
	testq [rbp + 16], r12;
	testq [rbp + 16], r13;
	testq [rbp + 16], r14;
	testq [rbp + 16], r15;
	testq [rbp + 512], rax;
	testq [rbp + 512], rcx;
	testq [rbp + 512], rdx;
	testq [rbp + 512], rbx;
	testq [rbp + 512], rsp;
	testq [rbp + 512], rbp;
	testq [rbp + 512], rsi;
	testq [rbp + 512], rdi;
	testq [rbp + 512], r8;
	testq [rbp + 512], r9;
	testq [rbp + 512], r10;
	testq [rbp + 512], r11;
	testq [rbp + 512], r12;
	testq [rbp + 512], r13;
	testq [rbp + 512], r14;
	testq [rbp + 512], r15;
	testq [rsi], rax;
	testq [rsi], rcx;
	testq [rsi], rdx;
	testq [rsi], rbx;
	testq [rsi], rsp;
	testq [rsi], rbp;
	testq [rsi], rsi;
	testq [rsi], rdi;
	testq [rsi], r8;
	testq [rsi], r9;
	testq [rsi], r10;
	testq [rsi], r11;
	testq [rsi], r12;
	testq [rsi], r13;
	testq [rsi], r14;
	testq [rsi], r15;
	testq [rsi + 16], rax;
	testq [rsi + 16], rcx;
	testq [rsi + 16], rdx;
	testq [rsi + 16], rbx;
	testq [rsi + 16], rsp;
	testq [rsi + 16], rbp;
	testq [rsi + 16], rsi;
	testq [rsi + 16], rdi;
	testq [rsi + 16], r8;
	testq [rsi + 16], r9;
	testq [rsi + 16], r10;
	testq [rsi + 16], r11;
	testq [rsi + 16], r12;
	testq [rsi + 16], r13;
	testq [rsi + 16], r14;
	testq [rsi + 16], r15;
	testq [rsi + 512], rax;
	testq [rsi + 512], rcx;
	testq [rsi + 512], rdx;
	testq [rsi + 512], rbx;
	testq [rsi + 512], rsp;
	testq [rsi + 512], rbp;
	testq [rsi + 512], rsi;
	testq [rsi + 512], rdi;
	testq [rsi + 512], r8;
	testq [rsi + 512], r9;
	testq [rsi + 512], r10;
	testq [rsi + 512], r11;
	testq [rsi + 512], r12;
	testq [rsi + 512], r13;
	testq [rsi + 512], r14;
	testq [rsi + 512], r15;
	testq [rdi], rax;
	testq [rdi], rcx;
	testq [rdi], rdx;
	testq [rdi], rbx;
	testq [rdi], rsp;
	testq [rdi], rbp;
	testq [rdi], rsi;
	testq [rdi], rdi;
	testq [rdi], r8;
	testq [rdi], r9;
	testq [rdi], r10;
	testq [rdi], r11;
	testq [rdi], r12;
	testq [rdi], r13;
	testq [rdi], r14;
	testq [rdi], r15;
	testq [rdi + 16], rax;
	testq [rdi + 16], rcx;
	testq [rdi + 16], rdx;
	testq [rdi + 16], rbx;
	testq [rdi + 16], rsp;
	testq [rdi + 16], rbp;
	testq [rdi + 16], rsi;
	testq [rdi + 16], rdi;
	testq [rdi + 16], r8;
	testq [rdi + 16], r9;
	testq [rdi + 16], r10;
	testq [rdi + 16], r11;
	testq [rdi + 16], r12;
	testq [rdi + 16], r13;
	testq [rdi + 16], r14;
	testq [rdi + 16], r15;
	testq [rdi + 512], rax;
	testq [rdi + 512], rcx;
	testq [rdi + 512], rdx;
	testq [rdi + 512], rbx;
	testq [rdi + 512], rsp;
	testq [rdi + 512], rbp;
	testq [rdi + 512], rsi;
	testq [rdi + 512], rdi;
	testq [rdi + 512], r8;
	testq [rdi + 512], r9;
	testq [rdi + 512], r10;
	testq [rdi + 512], r11;
	testq [rdi + 512], r12;
	testq [rdi + 512], r13;
	testq [rdi + 512], r14;
	testq [rdi + 512], r15;
	testq [r8], rax;
	testq [r8], rcx;
	testq [r8], rdx;
	testq [r8], rbx;
	testq [r8], rsp;
	testq [r8], rbp;
	testq [r8], rsi;
	testq [r8], rdi;
	testq [r8], r8;
	testq [r8], r9;
	testq [r8], r10;
	testq [r8], r11;
	testq [r8], r12;
	testq [r8], r13;
	testq [r8], r14;
	testq [r8], r15;
	testq [r8 + 16], rax;
	testq [r8 + 16], rcx;
	testq [r8 + 16], rdx;
	testq [r8 + 16], rbx;
	testq [r8 + 16], rsp;
	testq [r8 + 16], rbp;
	testq [r8 + 16], rsi;
	testq [r8 + 16], rdi;
	testq [r8 + 16], r8;
	testq [r8 + 16], r9;
	testq [r8 + 16], r10;
	testq [r8 + 16], r11;
	testq [r8 + 16], r12;
	testq [r8 + 16], r13;
	testq [r8 + 16], r14;
	testq [r8 + 16], r15;
	testq [r8 + 512], rax;
	testq [r8 + 512], rcx;
	testq [r8 + 512], rdx;
	testq [r8 + 512], rbx;
	testq [r8 + 512], rsp;
	testq [r8 + 512], rbp;
	testq [r8 + 512], rsi;
	testq [r8 + 512], rdi;
	testq [r8 + 512], r8;
	testq [r8 + 512], r9;
	testq [r8 + 512], r10;
	testq [r8 + 512], r11;
	testq [r8 + 512], r12;
	testq [r8 + 512], r13;
	testq [r8 + 512], r14;
	testq [r8 + 512], r15;
	testq [r9], rax;
	testq [r9], rcx;
	testq [r9], rdx;
	testq [r9], rbx;
	testq [r9], rsp;
	testq [r9], rbp;
	testq [r9], rsi;
	testq [r9], rdi;
	testq [r9], r8;
	testq [r9], r9;
	testq [r9], r10;
	testq [r9], r11;
	testq [r9], r12;
	testq [r9], r13;
	testq [r9], r14;
	testq [r9], r15;
	testq [r9 + 16], rax;
	testq [r9 + 16], rcx;
	testq [r9 + 16], rdx;
	testq [r9 + 16], rbx;
	testq [r9 + 16], rsp;
	testq [r9 + 16], rbp;
	testq [r9 + 16], rsi;
	testq [r9 + 16], rdi;
	testq [r9 + 16], r8;
	testq [r9 + 16], r9;
	testq [r9 + 16], r10;
	testq [r9 + 16], r11;
	testq [r9 + 16], r12;
	testq [r9 + 16], r13;
	testq [r9 + 16], r14;
	testq [r9 + 16], r15;
	testq [r9 + 512], rax;
	testq [r9 + 512], rcx;
	testq [r9 + 512], rdx;
	testq [r9 + 512], rbx;
	testq [r9 + 512], rsp;
	testq [r9 + 512], rbp;
	testq [r9 + 512], rsi;
	testq [r9 + 512], rdi;
	testq [r9 + 512], r8;
	testq [r9 + 512], r9;
	testq [r9 + 512], r10;
	testq [r9 + 512], r11;
	testq [r9 + 512], r12;
	testq [r9 + 512], r13;
	testq [r9 + 512], r14;
	testq [r9 + 512], r15;
	testq [r10], rax;
	testq [r10], rcx;
	testq [r10], rdx;
	testq [r10], rbx;
	testq [r10], rsp;
	testq [r10], rbp;
	testq [r10], rsi;
	testq [r10], rdi;
	testq [r10], r8;
	testq [r10], r9;
	testq [r10], r10;
	testq [r10], r11;
	testq [r10], r12;
	testq [r10], r13;
	testq [r10], r14;
	testq [r10], r15;
	testq [r10 + 16], rax;
	testq [r10 + 16], rcx;
	testq [r10 + 16], rdx;
	testq [r10 + 16], rbx;
	testq [r10 + 16], rsp;
	testq [r10 + 16], rbp;
	testq [r10 + 16], rsi;
	testq [r10 + 16], rdi;
	testq [r10 + 16], r8;
	testq [r10 + 16], r9;
	testq [r10 + 16], r10;
	testq [r10 + 16], r11;
	testq [r10 + 16], r12;
	testq [r10 + 16], r13;
	testq [r10 + 16], r14;
	testq [r10 + 16], r15;
	testq [r10 + 512], rax;
	testq [r10 + 512], rcx;
	testq [r10 + 512], rdx;
	testq [r10 + 512], rbx;
	testq [r10 + 512], rsp;
	testq [r10 + 512], rbp;
	testq [r10 + 512], rsi;
	testq [r10 + 512], rdi;
	testq [r10 + 512], r8;
	testq [r10 + 512], r9;
	testq [r10 + 512], r10;
	testq [r10 + 512], r11;
	testq [r10 + 512], r12;
	testq [r10 + 512], r13;
	testq [r10 + 512], r14;
	testq [r10 + 512], r15;
	testq [r11], rax;
	testq [r11], rcx;
	testq [r11], rdx;
	testq [r11], rbx;
	testq [r11], rsp;
	testq [r11], rbp;
	testq [r11], rsi;
	testq [r11], rdi;
	testq [r11], r8;
	testq [r11], r9;
	testq [r11], r10;
	testq [r11], r11;
	testq [r11], r12;
	testq [r11], r13;
	testq [r11], r14;
	testq [r11], r15;
	testq [r11 + 16], rax;
	testq [r11 + 16], rcx;
	testq [r11 + 16], rdx;
	testq [r11 + 16], rbx;
	testq [r11 + 16], rsp;
	testq [r11 + 16], rbp;
	testq [r11 + 16], rsi;
	testq [r11 + 16], rdi;
	testq [r11 + 16], r8;
	testq [r11 + 16], r9;
	testq [r11 + 16], r10;
	testq [r11 + 16], r11;
	testq [r11 + 16], r12;
	testq [r11 + 16], r13;
	testq [r11 + 16], r14;
	testq [r11 + 16], r15;
	testq [r11 + 512], rax;
	testq [r11 + 512], rcx;
	testq [r11 + 512], rdx;
	testq [r11 + 512], rbx;
	testq [r11 + 512], rsp;
	testq [r11 + 512], rbp;
	testq [r11 + 512], rsi;
	testq [r11 + 512], rdi;
	testq [r11 + 512], r8;
	testq [r11 + 512], r9;
	testq [r11 + 512], r10;
	testq [r11 + 512], r11;
	testq [r11 + 512], r12;
	testq [r11 + 512], r13;
	testq [r11 + 512], r14;
	testq [r11 + 512], r15;
	testq [r12], rax;
	testq [r12], rcx;
	testq [r12], rdx;
	testq [r12], rbx;
	testq [r12], rsp;
	testq [r12], rbp;
	testq [r12], rsi;
	testq [r12], rdi;
	testq [r12], r8;
	testq [r12], r9;
	testq [r12], r10;
	testq [r12], r11;
	testq [r12], r12;
	testq [r12], r13;
	testq [r12], r14;
	testq [r12], r15;
	testq [r12 + 16], rax;
	testq [r12 + 16], rcx;
	testq [r12 + 16], rdx;
	testq [r12 + 16], rbx;
	testq [r12 + 16], rsp;
	testq [r12 + 16], rbp;
	testq [r12 + 16], rsi;
	testq [r12 + 16], rdi;
	testq [r12 + 16], r8;
	testq [r12 + 16], r9;
	testq [r12 + 16], r10;
	testq [r12 + 16], r11;
	testq [r12 + 16], r12;
	testq [r12 + 16], r13;
	testq [r12 + 16], r14;
	testq [r12 + 16], r15;
	testq [r12 + 512], rax;
	testq [r12 + 512], rcx;
	testq [r12 + 512], rdx;
	testq [r12 + 512], rbx;
	testq [r12 + 512], rsp;
	testq [r12 + 512], rbp;
	testq [r12 + 512], rsi;
	testq [r12 + 512], rdi;
	testq [r12 + 512], r8;
	testq [r12 + 512], r9;
	testq [r12 + 512], r10;
	testq [r12 + 512], r11;
	testq [r12 + 512], r12;
	testq [r12 + 512], r13;
	testq [r12 + 512], r14;
	testq [r12 + 512], r15;
	testq [r13], rax;
	testq [r13], rcx;
	testq [r13], rdx;
	testq [r13], rbx;
	testq [r13], rsp;
	testq [r13], rbp;
	testq [r13], rsi;
	testq [r13], rdi;
	testq [r13], r8;
	testq [r13], r9;
	testq [r13], r10;
	testq [r13], r11;
	testq [r13], r12;
	testq [r13], r13;
	testq [r13], r14;
	testq [r13], r15;
	testq [r13 + 16], rax;
	testq [r13 + 16], rcx;
	testq [r13 + 16], rdx;
	testq [r13 + 16], rbx;
	testq [r13 + 16], rsp;
	testq [r13 + 16], rbp;
	testq [r13 + 16], rsi;
	testq [r13 + 16], rdi;
	testq [r13 + 16], r8;
	testq [r13 + 16], r9;
	testq [r13 + 16], r10;
	testq [r13 + 16], r11;
	testq [r13 + 16], r12;
	testq [r13 + 16], r13;
	testq [r13 + 16], r14;
	testq [r13 + 16], r15;
	testq [r13 + 512], rax;
	testq [r13 + 512], rcx;
	testq [r13 + 512], rdx;
	testq [r13 + 512], rbx;
	testq [r13 + 512], rsp;
	testq [r13 + 512], rbp;
	testq [r13 + 512], rsi;
	testq [r13 + 512], rdi;
	testq [r13 + 512], r8;
	testq [r13 + 512], r9;
	testq [r13 + 512], r10;
	testq [r13 + 512], r11;
	testq [r13 + 512], r12;
	testq [r13 + 512], r13;
	testq [r13 + 512], r14;
	testq [r13 + 512], r15;
	testq [r14], rax;
	testq [r14], rcx;
	testq [r14], rdx;
	testq [r14], rbx;
	testq [r14], rsp;
	testq [r14], rbp;
	testq [r14], rsi;
	testq [r14], rdi;
	testq [r14], r8;
	testq [r14], r9;
	testq [r14], r10;
	testq [r14], r11;
	testq [r14], r12;
	testq [r14], r13;
	testq [r14], r14;
	testq [r14], r15;
	testq [r14 + 16], rax;
	testq [r14 + 16], rcx;
	testq [r14 + 16], rdx;
	testq [r14 + 16], rbx;
	testq [r14 + 16], rsp;
	testq [r14 + 16], rbp;
	testq [r14 + 16], rsi;
	testq [r14 + 16], rdi;
	testq [r14 + 16], r8;
	testq [r14 + 16], r9;
	testq [r14 + 16], r10;
	testq [r14 + 16], r11;
	testq [r14 + 16], r12;
	testq [r14 + 16], r13;
	testq [r14 + 16], r14;
	testq [r14 + 16], r15;
	testq [r14 + 512], rax;
	testq [r14 + 512], rcx;
	testq [r14 + 512], rdx;
	testq [r14 + 512], rbx;
	testq [r14 + 512], rsp;
	testq [r14 + 512], rbp;
	testq [r14 + 512], rsi;
	testq [r14 + 512], rdi;
	testq [r14 + 512], r8;
	testq [r14 + 512], r9;
	testq [r14 + 512], r10;
	testq [r14 + 512], r11;
	testq [r14 + 512], r12;
	testq [r14 + 512], r13;
	testq [r14 + 512], r14;
	testq [r14 + 512], r15;
	testq [r15], rax;
	testq [r15], rcx;
	testq [r15], rdx;
	testq [r15], rbx;
	testq [r15], rsp;
	testq [r15], rbp;
	testq [r15], rsi;
	testq [r15], rdi;
	testq [r15], r8;
	testq [r15], r9;
	testq [r15], r10;
	testq [r15], r11;
	testq [r15], r12;
	testq [r15], r13;
	testq [r15], r14;
	testq [r15], r15;
	testq [r15 + 16], rax;
	testq [r15 + 16], rcx;
	testq [r15 + 16], rdx;
	testq [r15 + 16], rbx;
	testq [r15 + 16], rsp;
	testq [r15 + 16], rbp;
	testq [r15 + 16], rsi;
	testq [r15 + 16], rdi;
	testq [r15 + 16], r8;
	testq [r15 + 16], r9;
	testq [r15 + 16], r10;
	testq [r15 + 16], r11;
	testq [r15 + 16], r12;
	testq [r15 + 16], r13;
	testq [r15 + 16], r14;
	testq [r15 + 16], r15;
	testq [r15 + 512], rax;
	testq [r15 + 512], rcx;
	testq [r15 + 512], rdx;
	testq [r15 + 512], rbx;
	testq [r15 + 512], rsp;
	testq [r15 + 512], rbp;
	testq [r15 + 512], rsi;
	testq [r15 + 512], rdi;
	testq [r15 + 512], r8;
	testq [r15 + 512], r9;
	testq [r15 + 512], r10;
	testq [r15 + 512], r11;
	testq [r15 + 512], r12;
	testq [r15 + 512], r13;
	testq [r15 + 512], r14;
	testq [r15 + 512], r15;
	testq [rip], rax;
	testq [rip], rcx;
	testq [rip], rdx;
	testq [rip], rbx;
	testq [rip], rsp;
	testq [rip], rbp;
	testq [rip], rsi;
	testq [rip], rdi;
	testq [rip], r8;
	testq [rip], r9;
	testq [rip], r10;
	testq [rip], r11;
	testq [rip], r12;
	testq [rip], r13;
	testq [rip], r14;
	testq [rip], r15;
	testq [rip + 16], rax;
	testq [rip + 16], rcx;
	testq [rip + 16], rdx;
	testq [rip + 16], rbx;
	testq [rip + 16], rsp;
	testq [rip + 16], rbp;
	testq [rip + 16], rsi;
	testq [rip + 16], rdi;
	testq [rip + 16], r8;
	testq [rip + 16], r9;
	testq [rip + 16], r10;
	testq [rip + 16], r11;
	testq [rip + 16], r12;
	testq [rip + 16], r13;
	testq [rip + 16], r14;
	testq [rip + 16], r15;
	testq [rip + 512], rax;
	testq [rip + 512], rcx;
	testq [rip + 512], rdx;
	testq [rip + 512], rbx;
	testq [rip + 512], rsp;
	testq [rip + 512], rbp;
	testq [rip + 512], rsi;
	testq [rip + 512], rdi;
	testq [rip + 512], r8;
	testq [rip + 512], r9;
	testq [rip + 512], r10;
	testq [rip + 512], r11;
	testq [rip + 512], r12;
	testq [rip + 512], r13;
	testq [rip + 512], r14;
	testq [rip + 512], r15;
      );
      jit.finalize();
      let mut buf = std::fs::File::create("tests/testq_monoasm.bin").unwrap();
      buf.write_all(jit.as_slice()).unwrap();
  }