//! Assembly instructions
use std::fmt;
use super::operand::Operand;

/// Assembly instructions
#[derive(Debug, Clone)]
pub enum Instruction {
    // Data movement
    Mov(Operand, Operand),
    Movzx(Operand, Operand),  // Move with zero extension
    Movsx(Operand, Operand),  // Move with sign extension
    Push(Operand),
    Pop(Operand),
    Lea(Operand, Operand),    // Load effective address
    
    // Arithmetic
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand),             // Unsigned multiply
    Imul(Operand, Option<Operand>, Option<Operand>), // Signed multiply
    Div(Operand),             // Unsigned divide
    Idiv(Operand),            // Signed divide
    Inc(Operand),
    Dec(Operand),
    Neg(Operand),
    
    // Logical
    And(Operand, Operand),
    Or(Operand, Operand),
    Xor(Operand, Operand),
    Not(Operand),
    Shl(Operand, Operand),    // Shift left
    Shr(Operand, Operand),    // Shift right
    Sal(Operand, Operand),    // Arithmetic left shift
    Sar(Operand, Operand),    // Arithmetic right shift
    
    // Comparison
    Cmp(Operand, Operand),
    Test(Operand, Operand),
    
    // Control flow
    Jmp(String),              // Unconditional jump
    Je(String),               // Jump if equal
    Jne(String),              // Jump if not equal
    Jl(String),               // Jump if less
    Jle(String),              // Jump if less or equal
    Jg(String),               // Jump if greater
    Jge(String),              // Jump if greater or equal
    Jz(String),               // Jump if zero
    Jnz(String),              // Jump if not zero
    Ja(String),               // Jump if above (unsigned)
    Jb(String),               // Jump if below (unsigned)
    Jae(String),              // Jump if above or equal (unsigned)
    Jbe(String),              // Jump if below or equal (unsigned)
    Jo(String),               // Jump if overflow
    Jno(String),              // Jump if not overflow
    Js(String),               // Jump if sign
    Jns(String),              // Jump if not sign
    Loop(String),             // Loop with RCX counter
    Call(String),             // Function call
    Ret,                      // Return from function
    Retn(u16),                // Return with immediate
    
    // System
    Syscall,                  // System call (Linux)
    Int(u8),                  // Interrupt
    Hlt,                      // Halt processor
    
    // Control
    Nop,                      // No operation
    Cdq,                      // Convert doubleword to quadword
    Cqo,                      // Convert quadword to octword
    
    // Conditional moves
    Cmove(Operand, Operand),  // Conditional move if equal
    Cmovne(Operand, Operand), // Conditional move if not equal
    Cmovl(Operand, Operand),  // Conditional move if less
    Cmovle(Operand, Operand), // Conditional move if less or equal
    Cmovg(Operand, Operand),  // Conditional move if greater
    Cmovge(Operand, Operand), // Conditional move if greater or equal
    
    // Conditional set
    Sete(Operand),            // Set byte if equal
    Setne(Operand),           // Set byte if not equal
    Setl(Operand),            // Set byte if less
    Setle(Operand),           // Set byte if less or equal
    Setg(Operand),            // Set byte if greater
    Setge(Operand),           // Set byte if greater or equal
    Sets(Operand),            // Set byte if sign
    Setns(Operand),           // Set byte if not sign
    Seta(Operand),            // Set byte if above
    Setb(Operand),            // Set byte if below
    Setae(Operand),           // Set byte if above or equal
    Setbe(Operand),           // Set byte if below or equal
    Seto(Operand),            // Set byte if overflow
    Setno(Operand),           // Set byte if not overflow
    Setz(Operand),            // Set byte if zero
    Setnz(Operand),           // Set byte if not zero
    
    // Additional instructions for completeness
    Bt(Operand, Operand),     // Bit test
    Bts(Operand, Operand),    // Bit test and set
    Btr(Operand, Operand),    // Bit test and reset
    Btc(Operand, Operand),    // Bit test and complement
    Bsf(Operand, Operand),    // Bit scan forward
    Bsr(Operand, Operand),    // Bit scan reverse
    Bswap(Operand),           // Byte swap
    Xchg(Operand, Operand),   // Exchange
    Lock,                     // Lock prefix for atomic operations
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Mov(dst, src) => write!(f, "    mov {}, {}", dst, src),
            Instruction::Movzx(dst, src) => write!(f, "    movzx {}, {}", dst, src),
            Instruction::Movsx(dst, src) => write!(f, "    movsx {}, {}", dst, src),
            Instruction::Push(op) => write!(f, "    push {}", op),
            Instruction::Pop(op) => write!(f, "    pop {}", op),
            Instruction::Lea(dst, src) => write!(f, "    lea {}, {}", dst, src),
            
            Instruction::Add(dst, src) => write!(f, "    add {}, {}", dst, src),
            Instruction::Sub(dst, src) => write!(f, "    sub {}, {}", dst, src),
            Instruction::Mul(op) => write!(f, "    mul {}", op),
            Instruction::Imul(dst, src, third) => match (src, third) {
                (Some(src), Some(third)) => write!(f, "    imul {}, {}, {}", dst, src, third),
                (Some(src), None) => write!(f, "    imul {}, {}", dst, src),
                (None, _) => write!(f, "    imul {}", dst),
            },
            Instruction::Div(op) => write!(f, "    div {}", op),
            Instruction::Idiv(op) => write!(f, "    idiv {}", op),
            Instruction::Inc(op) => write!(f, "    inc {}", op),
            Instruction::Dec(op) => write!(f, "    dec {}", op),
            Instruction::Neg(op) => write!(f, "    neg {}", op),
            
            Instruction::And(dst, src) => write!(f, "    and {}, {}", dst, src),
            Instruction::Or(dst, src) => write!(f, "    or {}, {}", dst, src),
            Instruction::Xor(dst, src) => write!(f, "    xor {}, {}", dst, src),
            Instruction::Not(op) => write!(f, "    not {}", op),
            Instruction::Shl(dst, src) => write!(f, "    shl {}, {}", dst, src),
            Instruction::Shr(dst, src) => write!(f, "    shr {}, {}", dst, src),
            Instruction::Sal(dst, src) => write!(f, "    sal {}, {}", dst, src),
            Instruction::Sar(dst, src) => write!(f, "    sar {}, {}", dst, src),
            
            Instruction::Cmp(op1, op2) => write!(f, "    cmp {}, {}", op1, op2),
            Instruction::Test(op1, op2) => write!(f, "    test {}, {}", op1, op2),
            
            Instruction::Jmp(label) => write!(f, "    jmp {}", label),
            Instruction::Je(label) => write!(f, "    je {}", label),
            Instruction::Jne(label) => write!(f, "    jne {}", label),
            Instruction::Jl(label) => write!(f, "    jl {}", label),
            Instruction::Jle(label) => write!(f, "    jle {}", label),
            Instruction::Jg(label) => write!(f, "    jg {}", label),
            Instruction::Jge(label) => write!(f, "    jge {}", label),
            Instruction::Jz(label) => write!(f, "    jz {}", label),
            Instruction::Jnz(label) => write!(f, "    jnz {}", label),
            Instruction::Ja(label) => write!(f, "    ja {}", label),
            Instruction::Jb(label) => write!(f, "    jb {}", label),
            Instruction::Jae(label) => write!(f, "    jae {}", label),
            Instruction::Jbe(label) => write!(f, "    jbe {}", label),
            Instruction::Jo(label) => write!(f, "    jo {}", label),
            Instruction::Jno(label) => write!(f, "    jno {}", label),
            Instruction::Js(label) => write!(f, "    js {}", label),
            Instruction::Jns(label) => write!(f, "    jns {}", label),
            Instruction::Loop(label) => write!(f, "    loop {}", label),
            Instruction::Call(func) => write!(f, "    call {}", func),
            Instruction::Ret => write!(f, "    ret"),
            Instruction::Retn(val) => write!(f, "    ret {}", val),
            
            Instruction::Syscall => write!(f, "    syscall"),
            Instruction::Int(num) => write!(f, "    int 0x{:x}", num),
            Instruction::Hlt => write!(f, "    hlt"),
            
            Instruction::Nop => write!(f, "    nop"),
            Instruction::Cdq => write!(f, "    cdq"),
            Instruction::Cqo => write!(f, "    cqo"),
            
            Instruction::Cmove(dst, src) => write!(f, "    cmove {}, {}", dst, src),
            Instruction::Cmovne(dst, src) => write!(f, "    cmovne {}, {}", dst, src),
            Instruction::Cmovl(dst, src) => write!(f, "    cmovl {}, {}", dst, src),
            Instruction::Cmovle(dst, src) => write!(f, "    cmovle {}, {}", dst, src),
            Instruction::Cmovg(dst, src) => write!(f, "    cmovg {}, {}", dst, src),
            Instruction::Cmovge(dst, src) => write!(f, "    cmovge {}, {}", dst, src),
            
            Instruction::Sete(op) => write!(f, "    sete {}", op),
            Instruction::Setne(op) => write!(f, "    setne {}", op),
            Instruction::Setl(op) => write!(f, "    setl {}", op),
            Instruction::Setle(op) => write!(f, "    setle {}", op),
            Instruction::Setg(op) => write!(f, "    setg {}", op),
            Instruction::Setge(op) => write!(f, "    setge {}", op),
            Instruction::Sets(op) => write!(f, "    sets {}", op),
            Instruction::Setns(op) => write!(f, "    setns {}", op),
            Instruction::Seta(op) => write!(f, "    seta {}", op),
            Instruction::Setb(op) => write!(f, "    setb {}", op),
            Instruction::Setae(op) => write!(f, "    setae {}", op),
            Instruction::Setbe(op) => write!(f, "    setbe {}", op),
            Instruction::Seto(op) => write!(f, "    seto {}", op),
            Instruction::Setno(op) => write!(f, "    setno {}", op),
            Instruction::Setz(op) => write!(f, "    setz {}", op),
            Instruction::Setnz(op) => write!(f, "    setnz {}", op),
            
            // Additional instructions
            Instruction::Bt(op1, op2) => write!(f, "    bt {}, {}", op1, op2),
            Instruction::Bts(op1, op2) => write!(f, "    bts {}, {}", op1, op2),
            Instruction::Btr(op1, op2) => write!(f, "    btr {}, {}", op1, op2),
            Instruction::Btc(op1, op2) => write!(f, "    btc {}, {}", op1, op2),
            Instruction::Bsf(op1, op2) => write!(f, "    bsf {}, {}", op1, op2),
            Instruction::Bsr(op1, op2) => write!(f, "    bsr {}, {}", op1, op2),
            Instruction::Bswap(op) => write!(f, "    bswap {}", op),
            Instruction::Xchg(op1, op2) => write!(f, "    xchg {}, {}", op1, op2),
            Instruction::Lock => write!(f, "    lock"),
        }
    }
}