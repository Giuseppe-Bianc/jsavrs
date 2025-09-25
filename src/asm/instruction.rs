//! Assembly instructions
use std::fmt;
use super::operand::Operand;
use super::register::Register;

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

/// Floating-point instructions following IEEE 754 standards
#[derive(Debug, Clone)]
pub enum FloatingPointInstruction {
    // Single-precision arithmetic
    AddSS { dst: Register, src1: Operand, src2: Operand },
    SubSS { dst: Register, src1: Operand, src2: Operand },
    MulSS { dst: Register, src1: Operand, src2: Operand },
    DivSS { dst: Register, src1: Operand, src2: Operand },
    SqrtSS { dst: Register, src: Operand },
    RcpSS { dst: Register, src: Operand },      // Reciprocal
    RsqrtSS { dst: Register, src: Operand },    // Reciprocal square root
    
    // Double-precision arithmetic
    AddSD { dst: Register, src1: Operand, src2: Operand },
    SubSD { dst: Register, src1: Operand, src2: Operand },
    MulSD { dst: Register, src1: Operand, src2: Operand },
    DivSD { dst: Register, src1: Operand, src2: Operand },
    SqrtSD { dst: Register, src: Operand },
    RcpSD { dst: Register, src: Operand },      // Reciprocal
    RsqrtSD { dst: Register, src: Operand },    // Reciprocal square root
    
    // Vector operations (SSE)
    AddPS { dst: Register, src1: Operand, src2: Operand },  // Add packed single
    SubPS { dst: Register, src1: Operand, src2: Operand },
    MulPS { dst: Register, src1: Operand, src2: Operand },
    DivPS { dst: Register, src1: Operand, src2: Operand },
    SqrtPS { dst: Register, src: Operand },
    
    AddPD { dst: Register, src1: Operand, src2: Operand },  // Add packed double
    SubPD { dst: Register, src1: Operand, src2: Operand },
    MulPD { dst: Register, src1: Operand, src2: Operand },
    DivPD { dst: Register, src1: Operand, src2: Operand },
    SqrtPD { dst: Register, src: Operand },
    
    // Comparison operations (single precision)
    ComiSS { src1: Operand, src2: Operand },    // Compare scalar ordered single
    UComiSS { src1: Operand, src2: Operand },   // Compare scalar unordered single
    CmpSS { dst: Register, src1: Operand, src2: Operand, predicate: u8 }, // Compare packed single
    
    // Comparison operations (double precision)
    ComiSD { src1: Operand, src2: Operand },    // Compare scalar ordered double
    UComiSD { src1: Operand, src2: Operand },   // Compare scalar unordered double
    CmpSD { dst: Register, src1: Operand, src2: Operand, predicate: u8 }, // Compare packed double
    
    // Conversion operations
    CvttSS2SI { dst: Register, src: Operand },  // Convert with truncation scalar single to signed integer
    CvtSS2SI { dst: Register, src: Operand },   // Convert scalar single to signed integer
    CvttSD2SI { dst: Register, src: Operand },  // Convert with truncation scalar double to signed integer
    CvtSD2SI { dst: Register, src: Operand },   // Convert scalar double to signed integer
    CvttSD2SIQ { dst: Register, src: Operand }, // Convert scalar double to signed 64-bit integer
    CvtSS2SD { dst: Register, src: Operand },   // Convert scalar single to scalar double
    CvtSD2SS { dst: Register, src: Operand },   // Convert scalar double to scalar single
    CvtSI2SS { dst: Register, src: Operand },   // Convert signed integer to scalar single
    CvtSI2SD { dst: Register, src: Operand },   // Convert signed integer to scalar double
    CvtSIQ2SS { dst: Register, src: Operand },  // Convert signed 64-bit integer to scalar single
    CvtSIQ2SD { dst: Register, src: Operand },  // Convert signed 64-bit integer to scalar double
    
    // Move operations
    MovSS { dst: Register, src: Operand },      // Move scalar single
    MovSD { dst: Register, src: Operand },      // Move scalar double
    MovAPS { dst: Register, src: Operand },     // Move aligned packed single
    MovUPS { dst: Register, src: Operand },     // Move unaligned packed single
    MovAPD { dst: Register, src: Operand },     // Move aligned packed double
    MovUPD { dst: Register, src: Operand },     // Move unaligned packed double
    
    // Min/Max operations
    MinSS { dst: Register, src1: Operand, src2: Operand },  // Minimum scalar single
    MaxSS { dst: Register, src1: Operand, src2: Operand },  // Maximum scalar single
    MinSD { dst: Register, src1: Operand, src2: Operand },  // Minimum scalar double
    MaxSD { dst: Register, src1: Operand, src2: Operand },  // Maximum scalar double
}

impl fmt::Display for FloatingPointInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FloatingPointInstruction::AddSS { dst, src1, src2 } => write!(f, "    addss {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SubSS { dst, src1, src2 } => write!(f, "    subss {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MulSS { dst, src1, src2 } => write!(f, "    mulss {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::DivSS { dst, src1, src2 } => write!(f, "    divss {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SqrtSS { dst, src } => write!(f, "    sqrtss {}, {}", dst, src),
            FloatingPointInstruction::RcpSS { dst, src } => write!(f, "    rcpss {}, {}", dst, src),
            FloatingPointInstruction::RsqrtSS { dst, src } => write!(f, "    rsqrtss {}, {}", dst, src),
            
            FloatingPointInstruction::AddSD { dst, src1, src2 } => write!(f, "    addsd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SubSD { dst, src1, src2 } => write!(f, "    subsd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MulSD { dst, src1, src2 } => write!(f, "    mulsd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::DivSD { dst, src1, src2 } => write!(f, "    divsd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SqrtSD { dst, src } => write!(f, "    sqrtsd {}, {}", dst, src),
            FloatingPointInstruction::RcpSD { dst, src } => write!(f, "    rcpsd {}, {}", dst, src),
            FloatingPointInstruction::RsqrtSD { dst, src } => write!(f, "    rsqrtsd {}, {}", dst, src),
            
            FloatingPointInstruction::AddPS { dst, src1, src2 } => write!(f, "    addps {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SubPS { dst, src1, src2 } => write!(f, "    subps {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MulPS { dst, src1, src2 } => write!(f, "    mulps {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::DivPS { dst, src1, src2 } => write!(f, "    divps {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SqrtPS { dst, src } => write!(f, "    sqrtps {}, {}", dst, src),
            
            FloatingPointInstruction::AddPD { dst, src1, src2 } => write!(f, "    addpd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SubPD { dst, src1, src2 } => write!(f, "    subpd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MulPD { dst, src1, src2 } => write!(f, "    mulpd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::DivPD { dst, src1, src2 } => write!(f, "    divpd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::SqrtPD { dst, src } => write!(f, "    sqrtpd {}, {}", dst, src),
            
            FloatingPointInstruction::ComiSS { src1, src2 } => write!(f, "    comiss {}, {}", src1, src2),
            FloatingPointInstruction::UComiSS { src1, src2 } => write!(f, "    ucomiss {}, {}", src1, src2),
            FloatingPointInstruction::CmpSS { dst, src1, src2, predicate } => write!(f, "    cmpss {}, {}, {}, {}", dst, src1, src2, predicate),
            
            FloatingPointInstruction::ComiSD { src1, src2 } => write!(f, "    comisd {}, {}", src1, src2),
            FloatingPointInstruction::UComiSD { src1, src2 } => write!(f, "    ucomisd {}, {}", src1, src2),
            FloatingPointInstruction::CmpSD { dst, src1, src2, predicate } => write!(f, "    cmpsd {}, {}, {}, {}", dst, src1, src2, predicate),
            
            FloatingPointInstruction::CvttSS2SI { dst, src } => write!(f, "    cvttss2si {}, {}", dst, src),
            FloatingPointInstruction::CvtSS2SI { dst, src } => write!(f, "    cvtss2si {}, {}", dst, src),
            FloatingPointInstruction::CvttSD2SI { dst, src } => write!(f, "    cvttsd2si {}, {}", dst, src),
            FloatingPointInstruction::CvtSD2SI { dst, src } => write!(f, "    cvtsd2si {}, {}", dst, src),
            FloatingPointInstruction::CvttSD2SIQ { dst, src } => write!(f, "    cvttsd2siq {}, {}", dst, src),
            FloatingPointInstruction::CvtSS2SD { dst, src } => write!(f, "    cvtss2sd {}, {}", dst, src),
            FloatingPointInstruction::CvtSD2SS { dst, src } => write!(f, "    cvtsd2ss {}, {}", dst, src),
            FloatingPointInstruction::CvtSI2SS { dst, src } => write!(f, "    cvtsi2ss {}, {}", dst, src),
            FloatingPointInstruction::CvtSI2SD { dst, src } => write!(f, "    cvtsi2sd {}, {}", dst, src),
            FloatingPointInstruction::CvtSIQ2SS { dst, src } => write!(f, "    cvtsi2ssq {}, {}", dst, src),
            FloatingPointInstruction::CvtSIQ2SD { dst, src } => write!(f, "    cvtsi2sdq {}, {}", dst, src),
            
            FloatingPointInstruction::MovSS { dst, src } => write!(f, "    movss {}, {}", dst, src),
            FloatingPointInstruction::MovSD { dst, src } => write!(f, "    movsd {}, {}", dst, src),
            FloatingPointInstruction::MovAPS { dst, src } => write!(f, "    movaps {}, {}", dst, src),
            FloatingPointInstruction::MovUPS { dst, src } => write!(f, "    movups {}, {}", dst, src),
            FloatingPointInstruction::MovAPD { dst, src } => write!(f, "    movapd {}, {}", dst, src),
            FloatingPointInstruction::MovUPD { dst, src } => write!(f, "    movupd {}, {}", dst, src),
            
            FloatingPointInstruction::MinSS { dst, src1, src2 } => write!(f, "    minss {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MaxSS { dst, src1, src2 } => write!(f, "    maxss {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MinSD { dst, src1, src2 } => write!(f, "    minsd {}, {}, {}", dst, src1, src2),
            FloatingPointInstruction::MaxSD { dst, src1, src2 } => write!(f, "    maxsd {}, {}, {}", dst, src1, src2),
        }
    }
}