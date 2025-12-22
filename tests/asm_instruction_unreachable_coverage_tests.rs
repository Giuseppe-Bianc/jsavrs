//! Comprehensive tests to achieve 100% code coverage for Instruction Display implementation.
//!
//! This test suite specifically targets the unreachable code path and all edge cases
//! in the instruction.rs file, particularly focusing on:
//! - Line 547: The unreachable!() case for IMUL with (None, Some(_)) pattern
//! - All instruction variant combinations for Display trait
//! - Mnemonic method coverage for all instruction types
//! - Helper methods (`is_jump`, `is_call`, `is_return`) for all variants
//! - Edge cases in operand formatting

use jsavrs::asm::Instruction;
use jsavrs::asm::Operand;
use jsavrs::asm::{GPRegister8, GPRegister16, GPRegister32, GPRegister64};
use jsavrs::asm::{XMMRegister, YMMRegister};

// ============================================================================
// IMUL Unreachable Case Test - Line 547 Coverage
// ============================================================================

#[test]
#[should_panic(expected = "internal error: entered unreachable code")]
fn test_imul_unreachable_case_none_dest_some_src2() {
    // This tests the unreachable case at line 547 in instruction.rs
    // The pattern (None, Some(_)) in IMUL should never occur in valid assembly,
    // but we test it to achieve 100% coverage.
    //
    // In x86_64 assembly, IMUL has these valid forms:
    // 1. imul src                  => (None, None)
    // 2. imul dest, src            => (Some, None)
    // 3. imul dest, src1, imm      => (Some, Some)
    //
    // The pattern (None, Some(_)) is semantically impossible because you cannot
    // have an immediate operand (src2) without specifying a destination register.
    let instr =
        Instruction::Imul { dest: None, src1: Operand::reg64(GPRegister64::Rax), src2: Some(Operand::imm32(42)) };

    // This should panic with unreachable!()
    let _ = format!("{instr}");
}

// ============================================================================
// Complete Mnemonic Coverage Tests
// ============================================================================

#[test]
fn test_all_arithmetic_instruction_mnemonics() {
    // Test mnemonics for all arithmetic instructions
    assert_eq!(Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "add");
    assert_eq!(Instruction::Sub { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "sub");
    assert_eq!(Instruction::Mul { src: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "mul");
    assert_eq!(
        Instruction::Imul { dest: None, src1: Operand::reg64(GPRegister64::Rax), src2: None }.mnemonic(),
        "imul"
    );
    assert_eq!(Instruction::Div { src: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "div");
    assert_eq!(Instruction::Idiv { src: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "idiv");
    assert_eq!(Instruction::Inc { dest: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "inc");
    assert_eq!(Instruction::Dec { dest: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "dec");
    assert_eq!(Instruction::Neg { dest: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "neg");
    assert_eq!(Instruction::Adc { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "adc");
    assert_eq!(Instruction::Sbb { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "sbb");
}

#[test]
fn test_all_logical_instruction_mnemonics() {
    // Test mnemonics for all logical instructions
    assert_eq!(Instruction::And { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "and");
    assert_eq!(Instruction::Or { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "or");
    assert_eq!(
        Instruction::Xor { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rax) }.mnemonic(),
        "xor"
    );
    assert_eq!(Instruction::Not { dest: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "not");
    assert_eq!(Instruction::Test { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::imm32(1) }.mnemonic(), "test");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_shift_rotate_instruction_mnemonics() {
    // Test mnemonics for all shift and rotate instructions
    let count = Operand::imm8(1);
    assert_eq!(Instruction::Shl { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "shl");
    assert_eq!(Instruction::Shr { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "shr");
    assert_eq!(Instruction::Sar { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "sar");
    assert_eq!(Instruction::Sal { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "sal");
    assert_eq!(Instruction::Rol { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "rol");
    assert_eq!(Instruction::Ror { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "ror");
    assert_eq!(Instruction::Rcl { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "rcl");
    assert_eq!(Instruction::Rcr { dest: Operand::reg64(GPRegister64::Rax), count: count.clone() }.mnemonic(), "rcr");
}

#[test]
fn test_all_movement_instruction_mnemonics() {
    // Test mnemonics for all movement instructions
    assert_eq!(Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "mov");
    assert_eq!(
        Instruction::Movsx { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg32(GPRegister32::Eax) }
            .mnemonic(),
        "movsx"
    );
    assert_eq!(
        Instruction::Movsxd { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg32(GPRegister32::Eax) }
            .mnemonic(),
        "movsxd"
    );
    assert_eq!(
        Instruction::Movzx { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg8(GPRegister8::Al) }.mnemonic(),
        "movzx"
    );
    assert_eq!(
        Instruction::Lea { dest: Operand::reg64(GPRegister64::Rax), src: Operand::mem_disp(GPRegister64::Rbp, -8) }
            .mnemonic(),
        "lea"
    );
    assert_eq!(Instruction::Push { src: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "push");
    assert_eq!(Instruction::Pop { dest: Operand::reg64(GPRegister64::Rax) }.mnemonic(), "pop");
    assert_eq!(
        Instruction::Xchg { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::reg64(GPRegister64::Rbx) }.mnemonic(),
        "xchg"
    );
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_jump_instruction_mnemonics() {
    // Test mnemonics for all jump instructions
    let target = Operand::label("label");
    assert_eq!(Instruction::Jmp { target: target.clone() }.mnemonic(), "jmp");
    assert_eq!(Instruction::Je { target: target.clone() }.mnemonic(), "je");
    assert_eq!(Instruction::Jne { target: target.clone() }.mnemonic(), "jne");
    assert_eq!(Instruction::Jz { target: target.clone() }.mnemonic(), "jz");
    assert_eq!(Instruction::Jnz { target: target.clone() }.mnemonic(), "jnz");
    assert_eq!(Instruction::Jg { target: target.clone() }.mnemonic(), "jg");
    assert_eq!(Instruction::Jge { target: target.clone() }.mnemonic(), "jge");
    assert_eq!(Instruction::Jl { target: target.clone() }.mnemonic(), "jl");
    assert_eq!(Instruction::Jle { target: target.clone() }.mnemonic(), "jle");
    assert_eq!(Instruction::Ja { target: target.clone() }.mnemonic(), "ja");
    assert_eq!(Instruction::Jae { target: target.clone() }.mnemonic(), "jae");
    assert_eq!(Instruction::Jb { target: target.clone() }.mnemonic(), "jb");
    assert_eq!(Instruction::Jbe { target: target.clone() }.mnemonic(), "jbe");
    assert_eq!(Instruction::Js { target: target.clone() }.mnemonic(), "js");
    assert_eq!(Instruction::Jns { target: target.clone() }.mnemonic(), "jns");
    assert_eq!(Instruction::Jo { target: target.clone() }.mnemonic(), "jo");
    assert_eq!(Instruction::Jno { target: target.clone() }.mnemonic(), "jno");
    assert_eq!(Instruction::Jp { target: target.clone() }.mnemonic(), "jp");
    assert_eq!(Instruction::Jnp { target: target.clone() }.mnemonic(), "jnp");
}

#[test]
fn test_all_control_flow_instruction_mnemonics() {
    // Test mnemonics for call and return instructions
    assert_eq!(Instruction::Call { target: Operand::label("func") }.mnemonic(), "call");
    assert_eq!(Instruction::Ret.mnemonic(), "ret");
    assert_eq!(Instruction::RetImm { imm: 16 }.mnemonic(), "ret");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_sse_movement_instruction_mnemonics() {
    // Test mnemonics for SSE/AVX movement instructions
    let dest = Operand::xmm(XMMRegister::Xmm0);
    let src = Operand::xmm(XMMRegister::Xmm1);

    assert_eq!(Instruction::Movaps { dest: dest.clone(), src: src.clone() }.mnemonic(), "movaps");
    assert_eq!(Instruction::Movapd { dest: dest.clone(), src: src.clone() }.mnemonic(), "movapd");
    assert_eq!(Instruction::Movups { dest: dest.clone(), src: src.clone() }.mnemonic(), "movups");
    assert_eq!(Instruction::Movupd { dest: dest.clone(), src: src.clone() }.mnemonic(), "movupd");
    assert_eq!(Instruction::Movss { dest: dest.clone(), src: src.clone() }.mnemonic(), "movss");
    assert_eq!(Instruction::Movsd { dest: dest.clone(), src: src.clone() }.mnemonic(), "movsd");
    assert_eq!(Instruction::Movdqa { dest: dest.clone(), src: src.clone() }.mnemonic(), "movdqa");
    assert_eq!(Instruction::Movdqu { dest: dest.clone(), src: src.clone() }.mnemonic(), "movdqu");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_sse_arithmetic_instruction_mnemonics() {
    // Test mnemonics for SSE arithmetic instructions
    let dest = Operand::xmm(XMMRegister::Xmm0);
    let src = Operand::xmm(XMMRegister::Xmm1);

    assert_eq!(Instruction::Addps { dest: dest.clone(), src: src.clone() }.mnemonic(), "addps");
    assert_eq!(Instruction::Addpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "addpd");
    assert_eq!(Instruction::Addss { dest: dest.clone(), src: src.clone() }.mnemonic(), "addss");
    assert_eq!(Instruction::Addsd { dest: dest.clone(), src: src.clone() }.mnemonic(), "addsd");
    assert_eq!(Instruction::Subps { dest: dest.clone(), src: src.clone() }.mnemonic(), "subps");
    assert_eq!(Instruction::Subpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "subpd");
    assert_eq!(Instruction::Subss { dest: dest.clone(), src: src.clone() }.mnemonic(), "subss");
    assert_eq!(Instruction::Subsd { dest: dest.clone(), src: src.clone() }.mnemonic(), "subsd");
    assert_eq!(Instruction::Mulps { dest: dest.clone(), src: src.clone() }.mnemonic(), "mulps");
    assert_eq!(Instruction::Mulpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "mulpd");
    assert_eq!(Instruction::Mulss { dest: dest.clone(), src: src.clone() }.mnemonic(), "mulss");
    assert_eq!(Instruction::Mulsd { dest: dest.clone(), src: src.clone() }.mnemonic(), "mulsd");
    assert_eq!(Instruction::Divps { dest: dest.clone(), src: src.clone() }.mnemonic(), "divps");
    assert_eq!(Instruction::Divpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "divpd");
    assert_eq!(Instruction::Divss { dest: dest.clone(), src: src.clone() }.mnemonic(), "divss");
    assert_eq!(Instruction::Divsd { dest: dest.clone(), src: src.clone() }.mnemonic(), "divsd");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_sse_logical_instruction_mnemonics() {
    // Test mnemonics for SSE logical instructions
    let dest = Operand::xmm(XMMRegister::Xmm0);
    let src = Operand::xmm(XMMRegister::Xmm1);

    assert_eq!(Instruction::Andps { dest: dest.clone(), src: src.clone() }.mnemonic(), "andps");
    assert_eq!(Instruction::Andpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "andpd");
    assert_eq!(Instruction::Andnps { dest: dest.clone(), src: src.clone() }.mnemonic(), "andnps");
    assert_eq!(Instruction::Andnpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "andnpd");
    assert_eq!(Instruction::Orps { dest: dest.clone(), src: src.clone() }.mnemonic(), "orps");
    assert_eq!(Instruction::Orpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "orpd");
    assert_eq!(Instruction::Xorps { dest: dest.clone(), src: src.clone() }.mnemonic(), "xorps");
    assert_eq!(Instruction::Xorpd { dest: dest.clone(), src: src.clone() }.mnemonic(), "xorpd");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_sse_conversion_instruction_mnemonics() {
    // Test mnemonics for SSE conversion instructions
    let xmm = Operand::xmm(XMMRegister::Xmm0);
    let reg = Operand::reg64(GPRegister64::Rax);

    assert_eq!(Instruction::Cvtss2sd { dest: xmm.clone(), src: xmm.clone() }.mnemonic(), "cvtss2sd");
    assert_eq!(Instruction::Cvtsd2ss { dest: xmm.clone(), src: xmm.clone() }.mnemonic(), "cvtsd2ss");
    assert_eq!(Instruction::Cvttss2si { dest: reg.clone(), src: xmm.clone() }.mnemonic(), "cvttss2si");
    assert_eq!(Instruction::Cvttsd2si { dest: reg.clone(), src: xmm.clone() }.mnemonic(), "cvttsd2si");
    assert_eq!(Instruction::Cvtsi2ss { dest: xmm.clone(), src: reg.clone() }.mnemonic(), "cvtsi2ss");
    assert_eq!(Instruction::Cvtsi2sd { dest: xmm.clone(), src: reg.clone() }.mnemonic(), "cvtsi2sd");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_avx_instruction_mnemonics() {
    // Test mnemonics for AVX instructions
    let dest = Operand::ymm(YMMRegister::Ymm0);
    let src1 = Operand::ymm(YMMRegister::Ymm1);
    let src2 = Operand::ymm(YMMRegister::Ymm2);

    assert_eq!(Instruction::Vaddps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vaddps");
    assert_eq!(Instruction::Vaddpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vaddpd");
    assert_eq!(Instruction::Vaddss { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vaddss");
    assert_eq!(Instruction::Vaddsd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vaddsd");
    assert_eq!(Instruction::Vsubps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vsubps");
    assert_eq!(Instruction::Vsubpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vsubpd");
    assert_eq!(Instruction::Vmulps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vmulps");
    assert_eq!(Instruction::Vmulpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vmulpd");
    assert_eq!(Instruction::Vdivps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vdivps");
    assert_eq!(Instruction::Vdivpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }.mnemonic(), "vdivpd");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_fpu_instruction_mnemonics() {
    // Test mnemonics for FPU x87 instructions
    let mem = Operand::mem_disp(GPRegister64::Rbp, -8);

    assert_eq!(Instruction::Fld { src: mem.clone() }.mnemonic(), "fld");
    assert_eq!(Instruction::Fst { dest: mem.clone() }.mnemonic(), "fst");
    assert_eq!(Instruction::Fstp { dest: mem.clone() }.mnemonic(), "fstp");
    assert_eq!(Instruction::Fadd { src: Some(mem.clone()) }.mnemonic(), "fadd");
    assert_eq!(Instruction::Faddp { src: Some(mem.clone()) }.mnemonic(), "faddp");
    assert_eq!(Instruction::Fsub { src: Some(mem.clone()) }.mnemonic(), "fsub");
    assert_eq!(Instruction::Fsubp { src: Some(mem.clone()) }.mnemonic(), "fsubp");
    assert_eq!(Instruction::Fmul { src: Some(mem.clone()) }.mnemonic(), "fmul");
    assert_eq!(Instruction::Fmulp { src: Some(mem.clone()) }.mnemonic(), "fmulp");
    assert_eq!(Instruction::Fdiv { src: Some(mem.clone()) }.mnemonic(), "fdiv");
    assert_eq!(Instruction::Fdivp { src: Some(mem.clone()) }.mnemonic(), "fdivp");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_bit_manipulation_instruction_mnemonics() {
    // Test mnemonics for bit manipulation instructions
    let dest = Operand::reg64(GPRegister64::Rax);
    let src = Operand::reg64(GPRegister64::Rbx);

    assert_eq!(Instruction::Bsf { dest: dest.clone(), src: src.clone() }.mnemonic(), "bsf");
    assert_eq!(Instruction::Bsr { dest: dest.clone(), src: src.clone() }.mnemonic(), "bsr");
    assert_eq!(Instruction::Bt { dest: dest.clone(), src: src.clone() }.mnemonic(), "bt");
    assert_eq!(Instruction::Btc { dest: dest.clone(), src: src.clone() }.mnemonic(), "btc");
    assert_eq!(Instruction::Btr { dest: dest.clone(), src: src.clone() }.mnemonic(), "btr");
    assert_eq!(Instruction::Bts { dest: dest.clone(), src: src.clone() }.mnemonic(), "bts");
    assert_eq!(Instruction::Popcnt { dest: dest.clone(), src: src.clone() }.mnemonic(), "popcnt");
    assert_eq!(Instruction::Lzcnt { dest: dest.clone(), src: src.clone() }.mnemonic(), "lzcnt");
    assert_eq!(Instruction::Tzcnt { dest: dest.clone(), src: src.clone() }.mnemonic(), "tzcnt");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_cmov_instruction_mnemonics() {
    // Test mnemonics for conditional move instructions
    let dest = Operand::reg64(GPRegister64::Rax);
    let src = Operand::reg64(GPRegister64::Rbx);

    assert_eq!(Instruction::Cmove { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmove");
    assert_eq!(Instruction::Cmovne { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovne");
    assert_eq!(Instruction::Cmovg { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovg");
    assert_eq!(Instruction::Cmovge { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovge");
    assert_eq!(Instruction::Cmovl { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovl");
    assert_eq!(Instruction::Cmovle { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovle");
    assert_eq!(Instruction::Cmova { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmova");
    assert_eq!(Instruction::Cmovae { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovae");
    assert_eq!(Instruction::Cmovb { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovb");
    assert_eq!(Instruction::Cmovbe { dest: dest.clone(), src: src.clone() }.mnemonic(), "cmovbe");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_setcc_instruction_mnemonics() {
    // Test mnemonics for SETcc instructions
    let dest = Operand::reg8(GPRegister8::Al);

    assert_eq!(Instruction::Sete { dest: dest.clone() }.mnemonic(), "sete");
    assert_eq!(Instruction::Setne { dest: dest.clone() }.mnemonic(), "setne");
    assert_eq!(Instruction::Setg { dest: dest.clone() }.mnemonic(), "setg");
    assert_eq!(Instruction::Setge { dest: dest.clone() }.mnemonic(), "setge");
    assert_eq!(Instruction::Setl { dest: dest.clone() }.mnemonic(), "setl");
    assert_eq!(Instruction::Setle { dest: dest.clone() }.mnemonic(), "setle");
    assert_eq!(Instruction::Seta { dest: dest.clone() }.mnemonic(), "seta");
    assert_eq!(Instruction::Setae { dest: dest.clone() }.mnemonic(), "setae");
    assert_eq!(Instruction::Setb { dest: dest.clone() }.mnemonic(), "setb");
    assert_eq!(Instruction::Setbe { dest: dest.clone() }.mnemonic(), "setbe");
}

#[test]
fn test_all_control_instruction_mnemonics() {
    // Test mnemonics for control instructions
    assert_eq!(Instruction::Nop.mnemonic(), "nop");
    assert_eq!(Instruction::Hlt.mnemonic(), "hlt");
    assert_eq!(Instruction::Cpuid.mnemonic(), "cpuid");
    assert_eq!(Instruction::Pause.mnemonic(), "pause");
}

#[test]
fn test_all_string_instruction_mnemonics() {
    // Test mnemonics for string instructions
    assert_eq!(Instruction::Movsb.mnemonic(), "movsb");
    assert_eq!(Instruction::Movsw.mnemonic(), "movsw");
    assert_eq!(Instruction::MovsdString.mnemonic(), "movsd");
    assert_eq!(Instruction::Movsq.mnemonic(), "movsq");
    assert_eq!(Instruction::Stosb.mnemonic(), "stosb");
    assert_eq!(Instruction::Stosw.mnemonic(), "stosw");
    assert_eq!(Instruction::Stosd.mnemonic(), "stosd");
    assert_eq!(Instruction::Stosq.mnemonic(), "stosq");
}

#[test]
fn test_all_special_instruction_mnemonics() {
    // Test mnemonics for special instructions
    assert_eq!(Instruction::Cqo.mnemonic(), "cqo");
    assert_eq!(Instruction::Cdq.mnemonic(), "cdq");
    assert_eq!(Instruction::Syscall.mnemonic(), "syscall");
    assert_eq!(Instruction::Sysret.mnemonic(), "sysret");
}

// ============================================================================
// Complete Helper Method Coverage Tests
// ============================================================================

#[test]
#[allow(clippy::redundant_clone)]
fn test_is_jump_for_all_jump_instructions() {
    // Test is_jump() returns true for all jump instructions
    let target = Operand::label("test");

    assert!(Instruction::Jmp { target: target.clone() }.is_jump());
    assert!(Instruction::Je { target: target.clone() }.is_jump());
    assert!(Instruction::Jne { target: target.clone() }.is_jump());
    assert!(Instruction::Jz { target: target.clone() }.is_jump());
    assert!(Instruction::Jnz { target: target.clone() }.is_jump());
    assert!(Instruction::Jg { target: target.clone() }.is_jump());
    assert!(Instruction::Jge { target: target.clone() }.is_jump());
    assert!(Instruction::Jl { target: target.clone() }.is_jump());
    assert!(Instruction::Jle { target: target.clone() }.is_jump());
    assert!(Instruction::Ja { target: target.clone() }.is_jump());
    assert!(Instruction::Jae { target: target.clone() }.is_jump());
    assert!(Instruction::Jb { target: target.clone() }.is_jump());
    assert!(Instruction::Jbe { target: target.clone() }.is_jump());
    assert!(Instruction::Js { target: target.clone() }.is_jump());
    assert!(Instruction::Jns { target: target.clone() }.is_jump());
    assert!(Instruction::Jo { target: target.clone() }.is_jump());
    assert!(Instruction::Jno { target: target.clone() }.is_jump());
    assert!(Instruction::Jp { target: target.clone() }.is_jump());
    assert!(Instruction::Jnp { target: target.clone() }.is_jump());
}

#[test]
fn test_is_jump_false_for_non_jump_instructions() {
    // Test is_jump() returns false for non-jump instructions
    assert!(!Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(0) }.is_jump());
    assert!(!Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.is_jump());
    assert!(!Instruction::Call { target: Operand::label("func") }.is_jump());
    assert!(!Instruction::Ret.is_jump());
    assert!(!Instruction::Nop.is_jump());
}

#[test]
fn test_is_call_for_call_instruction() {
    // Test is_call() returns true only for call instructions
    assert!(Instruction::Call { target: Operand::label("func") }.is_call());
    assert!(Instruction::Call { target: Operand::reg64(GPRegister64::Rax) }.is_call());
}

#[test]
fn test_is_call_false_for_non_call_instructions() {
    // Test is_call() returns false for non-call instructions
    assert!(!Instruction::Jmp { target: Operand::label("label") }.is_call());
    assert!(!Instruction::Ret.is_call());
    assert!(!Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(0) }.is_call());
    assert!(!Instruction::Nop.is_call());
}

#[test]
fn test_is_return_for_return_instructions() {
    // Test is_return() returns true for both ret variants
    assert!(Instruction::Ret.is_return());
    assert!(Instruction::RetImm { imm: 16 }.is_return());
    assert!(Instruction::RetImm { imm: 0 }.is_return());
}

#[test]
fn test_is_return_false_for_non_return_instructions() {
    // Test is_return() returns false for non-return instructions
    assert!(!Instruction::Call { target: Operand::label("func") }.is_return());
    assert!(!Instruction::Jmp { target: Operand::label("label") }.is_return());
    assert!(!Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(0) }.is_return());
    assert!(!Instruction::Nop.is_return());
}

// ============================================================================
// Complete Display Formatting Coverage Tests
// ============================================================================

#[test]
#[allow(clippy::redundant_clone)]
fn test_display_all_shift_rotate_instructions() {
    // Test Display for all shift and rotate instructions with various counts
    let reg = Operand::reg64(GPRegister64::Rax);
    let count_imm = Operand::imm8(3);
    let count_cl = Operand::reg8(GPRegister8::Cl);

    assert_eq!(format!("{}", Instruction::Shl { dest: reg.clone(), count: count_imm.clone() }), "shl rax, 3");
    assert_eq!(format!("{}", Instruction::Shr { dest: reg.clone(), count: count_imm.clone() }), "shr rax, 3");
    assert_eq!(format!("{}", Instruction::Sar { dest: reg.clone(), count: count_imm.clone() }), "sar rax, 3");
    assert_eq!(format!("{}", Instruction::Sal { dest: reg.clone(), count: count_imm.clone() }), "sal rax, 3");
    assert_eq!(format!("{}", Instruction::Rol { dest: reg.clone(), count: count_cl.clone() }), "rol rax, cl");
    assert_eq!(format!("{}", Instruction::Ror { dest: reg.clone(), count: count_cl.clone() }), "ror rax, cl");
    assert_eq!(format!("{}", Instruction::Rcl { dest: reg.clone(), count: count_cl.clone() }), "rcl rax, cl");
    assert_eq!(format!("{}", Instruction::Rcr { dest: reg.clone(), count: count_cl.clone() }), "rcr rax, cl");
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_display_fpu_instructions_with_and_without_operands() {
    // Test Display for FPU instructions with optional operands
    let mem = Operand::mem_disp(GPRegister64::Rbp, -8);

    // Instructions with memory operands
    assert_eq!(format!("{}", Instruction::Fadd { src: Some(mem.clone()) }), "fadd QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Faddp { src: Some(mem.clone()) }), "faddp QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Fsub { src: Some(mem.clone()) }), "fsub QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Fsubp { src: Some(mem.clone()) }), "fsubp QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Fmul { src: Some(mem.clone()) }), "fmul QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Fmulp { src: Some(mem.clone()) }), "fmulp QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Fdiv { src: Some(mem.clone()) }), "fdiv QWORD PTR [rbp - 8]");
    assert_eq!(format!("{}", Instruction::Fdivp { src: Some(mem.clone()) }), "fdivp QWORD PTR [rbp - 8]");

    // Instructions without operands (implicit ST(0) and ST(1))
    assert_eq!(format!("{}", Instruction::Fadd { src: None }), "fadd");
    assert_eq!(format!("{}", Instruction::Faddp { src: None }), "faddp");
}
#[test]
#[allow(clippy::redundant_clone)]
fn test_display_avx_three_operand_instructions() {
    // Test Display for AVX instructions with three operands
    let dest = Operand::ymm(YMMRegister::Ymm0);
    let src1 = Operand::ymm(YMMRegister::Ymm1);
    let src2 = Operand::ymm(YMMRegister::Ymm2);

    assert_eq!(
        format!("{}", Instruction::Vaddps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vaddps ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vaddpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vaddpd ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vaddss { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vaddss ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vaddsd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vaddsd ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vsubps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vsubps ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vsubpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vsubpd ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vmulps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vmulps ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vmulpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vmulpd ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vdivps { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vdivps ymm0, ymm1, ymm2"
    );
    assert_eq!(
        format!("{}", Instruction::Vdivpd { dest: dest.clone(), src1: src1.clone(), src2: src2.clone() }),
        "vdivpd ymm0, ymm1, ymm2"
    );
}

#[test]
fn test_display_return_instructions() {
    // Test Display for both return variants
    assert_eq!(format!("{}", Instruction::Ret), "ret");
    assert_eq!(format!("{}", Instruction::RetImm { imm: 16 }), "ret 16");
    assert_eq!(format!("{}", Instruction::RetImm { imm: 0 }), "ret 0");
    assert_eq!(format!("{}", Instruction::RetImm { imm: 255 }), "ret 255");
}

#[test]
fn test_display_string_instructions() {
    // Test Display for all string instructions (no operands)
    assert_eq!(format!("{}", Instruction::Movsb), "movsb");
    assert_eq!(format!("{}", Instruction::Movsw), "movsw");
    assert_eq!(format!("{}", Instruction::MovsdString), "movsd");
    assert_eq!(format!("{}", Instruction::Movsq), "movsq");
    assert_eq!(format!("{}", Instruction::Stosb), "stosb");
    assert_eq!(format!("{}", Instruction::Stosw), "stosw");
    assert_eq!(format!("{}", Instruction::Stosd), "stosd");
    assert_eq!(format!("{}", Instruction::Stosq), "stosq");
}

#[test]
fn test_display_control_instructions() {
    // Test Display for control instructions (no operands)
    assert_eq!(format!("{}", Instruction::Nop), "nop");
    assert_eq!(format!("{}", Instruction::Hlt), "hlt");
    assert_eq!(format!("{}", Instruction::Cpuid), "cpuid");
    assert_eq!(format!("{}", Instruction::Pause), "pause");
    assert_eq!(format!("{}", Instruction::Cqo), "cqo");
    assert_eq!(format!("{}", Instruction::Cdq), "cdq");
    assert_eq!(format!("{}", Instruction::Syscall), "syscall");
    assert_eq!(format!("{}", Instruction::Sysret), "sysret");
}

// ============================================================================
// Edge Case Tests - Operand Combinations
// ============================================================================

#[test]
#[allow(clippy::unreadable_literal)]
fn test_imul_with_all_register_sizes() {
    // Test IMUL with 8-bit, 16-bit, 32-bit, and 64-bit registers
    assert_eq!(
        format!("{}", Instruction::Imul { dest: None, src1: Operand::reg16(GPRegister16::Ax), src2: None }),
        "imul ax"
    );
    assert_eq!(
        format!(
            "{}",
            Instruction::Imul {
                dest: Some(Operand::reg32(GPRegister32::Eax)),
                src1: Operand::reg32(GPRegister32::Ebx),
                src2: None
            }
        ),
        "imul eax, ebx"
    );
    assert_eq!(
        format!(
            "{}",
            Instruction::Imul {
                dest: Some(Operand::reg64(GPRegister64::Rax)),
                src1: Operand::reg64(GPRegister64::Rbx),
                src2: Some(Operand::imm32(1000000))
            }
        ),
        "imul rax, rbx, 1000000"
    );
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_shift_instructions_with_various_counts() {
    // Test shift instructions with immediate count, CL register, and 1
    let reg = Operand::reg32(GPRegister32::Ebx);

    assert_eq!(format!("{}", Instruction::Shl { dest: reg.clone(), count: Operand::imm8(1) }), "shl ebx, 1");
    assert_eq!(format!("{}", Instruction::Shr { dest: reg.clone(), count: Operand::imm8(31) }), "shr ebx, 31");
    assert_eq!(
        format!("{}", Instruction::Sar { dest: reg.clone(), count: Operand::reg8(GPRegister8::Cl) }),
        "sar ebx, cl"
    );
}

#[test]
fn test_cmp_and_test_with_various_operand_combinations() {
    // Test CMP and TEST with reg-reg, reg-imm, and reg-mem combinations
    assert_eq!(
        format!(
            "{}",
            Instruction::Cmp { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::reg64(GPRegister64::Rbx) }
        ),
        "cmp rax, rbx"
    );
    assert_eq!(
        format!("{}", Instruction::Cmp { op1: Operand::reg32(GPRegister32::Eax), op2: Operand::imm32(42) }),
        "cmp eax, 42"
    );
    assert_eq!(
        format!("{}", Instruction::Test { op1: Operand::reg8(GPRegister8::Al), op2: Operand::imm8(127) }),
        "test al, 127"
    );
}

#[test]
fn test_extreme_immediate_values() {
    // Test instructions with extreme immediate values (min/max for i32)
    assert_eq!(
        format!("{}", Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(i32::MAX) }),
        format!("mov rax, {}", i32::MAX)
    );
    assert_eq!(
        format!("{}", Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(i32::MIN) }),
        format!("add rax, {}", i32::MIN)
    );
    assert_eq!(
        format!("{}", Instruction::Cmp { op1: Operand::reg32(GPRegister32::Eax), op2: Operand::imm32(0) }),
        "cmp eax, 0"
    );
}

#[test]
fn test_memory_operands_with_various_displacements() {
    // Test instructions with memory operands using various displacement values
    assert_eq!(
        format!(
            "{}",
            Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::mem_disp(GPRegister64::Rbp, 0) }
        ),
        "mov rax, QWORD PTR [rbp]"
    );
    assert_eq!(
        format!(
            "{}",
            Instruction::Add { dest: Operand::reg32(GPRegister32::Eax), src: Operand::mem_disp(GPRegister64::Rsp, 16) }
        ),
        "add eax, QWORD PTR [rsp + 16]"
    );
    assert_eq!(
        format!(
            "{}",
            Instruction::Sub {
                dest: Operand::reg64(GPRegister64::Rdx),
                src: Operand::mem_disp(GPRegister64::Rbx, -128)
            }
        ),
        "sub rdx, QWORD PTR [rbx - 128]"
    );
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_all_setcc_instructions_display() {
    // Test Display for all SETcc instructions
    let dest = Operand::reg8(GPRegister8::Al);

    assert_eq!(format!("{}", Instruction::Sete { dest: dest.clone() }), "sete al");
    assert_eq!(format!("{}", Instruction::Setne { dest: dest.clone() }), "setne al");
    assert_eq!(format!("{}", Instruction::Setg { dest: dest.clone() }), "setg al");
    assert_eq!(format!("{}", Instruction::Setge { dest: dest.clone() }), "setge al");
    assert_eq!(format!("{}", Instruction::Setl { dest: dest.clone() }), "setl al");
    assert_eq!(format!("{}", Instruction::Setle { dest: dest.clone() }), "setle al");
    assert_eq!(format!("{}", Instruction::Seta { dest: dest.clone() }), "seta al");
    assert_eq!(format!("{}", Instruction::Setae { dest: dest.clone() }), "setae al");
    assert_eq!(format!("{}", Instruction::Setb { dest: dest.clone() }), "setb al");
    assert_eq!(format!("{}", Instruction::Setbe { dest: dest.clone() }), "setbe al");
}

#[test]
fn test_comparison_instruction_display() {
    // Test CMP instruction Display
    assert_eq!(
        format!("{}", Instruction::Cmp { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::imm32(0) }),
        "cmp rax, 0"
    );
}

#[test]
fn test_xchg_instruction_display() {
    // Test XCHG instruction Display with various register combinations
    assert_eq!(
        format!(
            "{}",
            Instruction::Xchg { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::reg64(GPRegister64::Rbx) }
        ),
        "xchg rax, rbx"
    );
    assert_eq!(
        format!(
            "{}",
            Instruction::Xchg { op1: Operand::reg32(GPRegister32::Eax), op2: Operand::reg32(GPRegister32::Ecx) }
        ),
        "xchg eax, ecx"
    );
}

// ============================================================================
// Line 470: Mulsd Instruction Coverage
// ============================================================================

#[test]
fn test_mulsd_display_xmm_to_xmm() {
    // Test MULSD with XMM register to XMM register
    // Line 470: Self::Mulsd { dest, src }
    let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(instr.to_string(), "mulsd xmm0, xmm1");
}

#[test]
fn test_mulsd_display_with_memory_operand() {
    // Test MULSD with memory operand as source
    // Line 470: Self::Mulsd { dest, src }
    let instr =
        Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm7), src: Operand::mem_disp(GPRegister64::Rbp, -16) };
    assert_eq!(instr.to_string(), "mulsd xmm7, QWORD PTR [rbp - 16]");
}

#[test]
fn test_mulsd_display_all_xmm_registers() {
    // Test MULSD with various XMM register combinations
    // Line 470: Self::Mulsd { dest, src }
    let registers = [
        XMMRegister::Xmm0,
        XMMRegister::Xmm1,
        XMMRegister::Xmm2,
        XMMRegister::Xmm3,
        XMMRegister::Xmm4,
        XMMRegister::Xmm5,
        XMMRegister::Xmm6,
        XMMRegister::Xmm7,
        XMMRegister::Xmm8,
        XMMRegister::Xmm9,
        XMMRegister::Xmm10,
        XMMRegister::Xmm11,
        XMMRegister::Xmm12,
        XMMRegister::Xmm13,
        XMMRegister::Xmm14,
        XMMRegister::Xmm15,
    ];

    for dest_reg in &registers {
        for src_reg in &registers {
            let instr = Instruction::Mulsd { dest: Operand::xmm(*dest_reg), src: Operand::xmm(*src_reg) };
            let expected =
                format!("mulsd {}, {}", format!("{dest_reg:?}").to_lowercase(), format!("{src_reg:?}").to_lowercase());
            assert_eq!(instr.to_string(), expected);
        }
    }
}

#[test]
fn test_mulsd_mnemonic() {
    // Test that mnemonic() returns "mulsd" for Mulsd instruction
    // This verifies the mnemonic method coverage for line 470
    let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(instr.mnemonic(), "mulsd");
}

#[test]
fn test_mulsd_with_memory_positive_displacement() {
    // Test MULSD with positive memory displacement
    // Line 470: Self::Mulsd { dest, src }
    let instr =
        Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::mem_disp(GPRegister64::Rsi, 128) };
    assert_eq!(instr.to_string(), "mulsd xmm3, QWORD PTR [rsi + 128]");
}

#[test]
fn test_mulsd_with_memory_zero_displacement() {
    // Test MULSD with zero memory displacement
    // Line 470: Self::Mulsd { dest, src }
    let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm15), src: Operand::mem(GPRegister64::Rdx) };
    assert_eq!(instr.to_string(), "mulsd xmm15, QWORD PTR [rdx]");
}

#[test]
fn test_mulsd_with_high_registers() {
    // Test MULSD with high XMM registers (xmm8-xmm15)
    // Line 470: Self::Mulsd { dest, src }
    let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm14), src: Operand::xmm(XMMRegister::Xmm13) };
    assert_eq!(instr.to_string(), "mulsd xmm14, xmm13");
}

// ============================================================================
// Line 471: Divps Instruction Coverage
// ============================================================================

#[test]
fn test_divps_display_xmm_to_xmm() {
    // Test DIVPS with XMM register to XMM register
    // Line 471: Self::Divps { dest, src }
    let instr = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(instr.to_string(), "divps xmm0, xmm1");
}

#[test]
fn test_divps_display_with_memory_operand() {
    // Test DIVPS with memory operand as source
    // Line 471: Self::Divps { dest, src }
    let instr =
        Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm5), src: Operand::mem_disp(GPRegister64::Rsp, 32) };
    assert_eq!(instr.to_string(), "divps xmm5, QWORD PTR [rsp + 32]");
}

#[test]
fn test_divps_display_all_xmm_registers() {
    // Test DIVPS with various XMM register combinations
    // Line 471: Self::Divps { dest, src }
    let registers = [
        XMMRegister::Xmm0,
        XMMRegister::Xmm1,
        XMMRegister::Xmm2,
        XMMRegister::Xmm3,
        XMMRegister::Xmm4,
        XMMRegister::Xmm5,
        XMMRegister::Xmm6,
        XMMRegister::Xmm7,
        XMMRegister::Xmm8,
        XMMRegister::Xmm9,
        XMMRegister::Xmm10,
        XMMRegister::Xmm11,
        XMMRegister::Xmm12,
        XMMRegister::Xmm13,
        XMMRegister::Xmm14,
        XMMRegister::Xmm15,
    ];

    for dest_reg in &registers {
        for src_reg in &registers {
            let instr = Instruction::Divps { dest: Operand::xmm(*dest_reg), src: Operand::xmm(*src_reg) };
            let expected =
                format!("divps {}, {}", format!("{dest_reg:?}").to_lowercase(), format!("{src_reg:?}").to_lowercase());
            assert_eq!(instr.to_string(), expected);
        }
    }
}

#[test]
fn test_divps_mnemonic() {
    // Test that mnemonic() returns "divps" for Divps instruction
    // This verifies the mnemonic method coverage for line 471
    let instr = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(instr.mnemonic(), "divps");
}

#[test]
fn test_divps_with_memory_negative_displacement() {
    // Test DIVPS with negative memory displacement
    // Line 471: Self::Divps { dest, src }
    let instr =
        Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm9), src: Operand::mem_disp(GPRegister64::Rbp, -256) };
    assert_eq!(instr.to_string(), "divps xmm9, QWORD PTR [rbp - 256]");
}

#[test]
fn test_divps_with_memory_large_positive_displacement() {
    // Test DIVPS with large positive memory displacement
    // Line 471: Self::Divps { dest, src }
    let instr = Instruction::Divps {
        dest: Operand::xmm(XMMRegister::Xmm12),
        src: Operand::mem_disp(GPRegister64::Rdi, 0x1000),
    };
    assert_eq!(instr.to_string(), "divps xmm12, QWORD PTR [rdi + 4096]");
}

#[test]
fn test_divps_with_high_registers() {
    // Test DIVPS with high XMM registers (xmm8-xmm15)
    // Line 471: Self::Divps { dest, src }
    let instr = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm15), src: Operand::xmm(XMMRegister::Xmm8) };
    assert_eq!(instr.to_string(), "divps xmm15, xmm8");
}

// ============================================================================
// Edge Case Tests: Comparison and Cloning
// ============================================================================

#[test]
fn test_mulsd_clone_and_equality() {
    // Test Clone and PartialEq for Mulsd instruction
    let instr1 = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    let instr2 = instr1.clone();
    assert_eq!(instr1, instr2);
    assert_eq!(format!("{instr1}"), format!("{instr2}"));
}

#[test]
fn test_divps_clone_and_equality() {
    // Test Clone and PartialEq for Divps instruction
    let instr1 = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::xmm(XMMRegister::Xmm4) };
    let instr2 = instr1.clone();
    assert_eq!(instr1, instr2);
    assert_eq!(format!("{instr1}"), format!("{instr2}"));
}

#[test]
fn test_mulsd_vs_divps_inequality() {
    // Test that Mulsd and Divps are not equal even with same operands
    let mulsd = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    let divps = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_ne!(mulsd, divps);
    assert_ne!(mulsd.mnemonic(), divps.mnemonic());
}

// ============================================================================
// Helper Methods Coverage for Lines 470-471
// ============================================================================

#[test]
fn test_mulsd_is_not_jump_call_or_return() {
    // Test that Mulsd is not classified as jump, call, or return
    let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert!(!instr.is_jump());
    assert!(!instr.is_call());
    assert!(!instr.is_return());
}

#[test]
fn test_divps_is_not_jump_call_or_return() {
    // Test that Divps is not classified as jump, call, or return
    let instr = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert!(!instr.is_jump());
    assert!(!instr.is_call());
    assert!(!instr.is_return());
}

// ============================================================================
// Debug Trait Coverage
// ============================================================================

#[test]
fn test_mulsd_debug_format() {
    // Test Debug trait for Mulsd instruction
    let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    let debug_str = format!("{instr:?}");
    assert!(debug_str.contains("Mulsd"));
}

#[test]
fn test_divps_debug_format() {
    // Test Debug trait for Divps instruction
    let instr = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm6), src: Operand::xmm(XMMRegister::Xmm7) };
    let debug_str = format!("{instr:?}");
    assert!(debug_str.contains("Divps"));
}

// ============================================================================
// Complex Operand Combinations
// ============================================================================

#[test]
fn test_mulsd_with_all_base_registers() {
    // Test MULSD with memory operands using all possible base registers
    let base_registers = [
        GPRegister64::Rax,
        GPRegister64::Rbx,
        GPRegister64::Rcx,
        GPRegister64::Rdx,
        GPRegister64::Rsi,
        GPRegister64::Rdi,
        GPRegister64::Rbp,
        GPRegister64::Rsp,
        GPRegister64::R8,
        GPRegister64::R9,
        GPRegister64::R10,
        GPRegister64::R11,
        GPRegister64::R12,
        GPRegister64::R13,
        GPRegister64::R14,
        GPRegister64::R15,
    ];

    for base_reg in &base_registers {
        let instr = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem(*base_reg) };
        let base_name = format!("{base_reg:?}").to_lowercase();
        let expected = format!("mulsd xmm0, QWORD PTR [{base_name}]");
        assert_eq!(instr.to_string(), expected);
    }
}

#[test]
fn test_divps_with_all_base_registers() {
    // Test DIVPS with memory operands using all possible base registers
    let base_registers = [
        GPRegister64::Rax,
        GPRegister64::Rbx,
        GPRegister64::Rcx,
        GPRegister64::Rdx,
        GPRegister64::Rsi,
        GPRegister64::Rdi,
        GPRegister64::Rbp,
        GPRegister64::Rsp,
        GPRegister64::R8,
        GPRegister64::R9,
        GPRegister64::R10,
        GPRegister64::R11,
        GPRegister64::R12,
        GPRegister64::R13,
        GPRegister64::R14,
        GPRegister64::R15,
    ];

    for base_reg in &base_registers {
        let instr = Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm1), src: Operand::mem(*base_reg) };
        let base_name = format!("{base_reg:?}").to_lowercase();
        let expected = format!("divps xmm1, QWORD PTR [{base_name}]");
        assert_eq!(instr.to_string(), expected);
    }
}

// ============================================================================
// Boundary Value Tests
// ============================================================================

#[test]
fn test_mulsd_with_max_positive_displacement() {
    // Test MULSD with maximum positive i32 displacement
    let instr = Instruction::Mulsd {
        dest: Operand::xmm(XMMRegister::Xmm0),
        src: Operand::mem_disp(GPRegister64::Rax, i32::MAX),
    };
    assert_eq!(instr.to_string(), format!("mulsd xmm0, QWORD PTR [rax + {}]", i32::MAX));
}

#[test]
fn test_mulsd_with_large_negative_displacement() {
    // Test MULSD with large negative displacement
    let instr =
        Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem_disp(GPRegister64::Rax, -65536) };
    assert_eq!(instr.to_string(), "mulsd xmm0, QWORD PTR [rax - 65536]");
}

#[test]
fn test_divps_with_max_positive_displacement() {
    // Test DIVPS with maximum positive i32 displacement
    let instr = Instruction::Divps {
        dest: Operand::xmm(XMMRegister::Xmm0),
        src: Operand::mem_disp(GPRegister64::Rbx, i32::MAX),
    };
    assert_eq!(instr.to_string(), format!("divps xmm0, QWORD PTR [rbx + {}]", i32::MAX));
}

#[test]
fn test_divps_with_large_negative_displacement() {
    // Test DIVPS with large negative displacement
    let instr =
        Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem_disp(GPRegister64::Rbx, -32768) };
    assert_eq!(instr.to_string(), "divps xmm0, QWORD PTR [rbx - 32768]");
}
