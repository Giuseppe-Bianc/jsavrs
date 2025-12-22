//! Comprehensive tests for uncovered Instruction Display and helper method edge cases.
//!
//! This test suite specifically targets code paths that are not covered by existing tests,
//! including:
//! - IMUL instruction with all operand combinations
//! - All jump instruction variants (conditional and unconditional)
//! - Return instructions (with and without immediate)
//! - AVX three-operand instructions
//! - FPU instructions with optional operands
//! - String instructions
//! - `SETcc` instructions
//! - Bit manipulation instructions
//! - Conditional move instructions

use jsavrs::asm::Instruction;
use jsavrs::asm::Operand;
use jsavrs::asm::{GPRegister8, GPRegister32, GPRegister64, XMMRegister, YMMRegister};

// ============================================================================
// IMUL Instruction Display Tests - All Variants
// ============================================================================

#[test]
fn test_imul_single_operand_display() {
    // Test IMUL with single operand (implicit destination)
    let instr = Instruction::Imul { dest: None, src1: Operand::reg64(GPRegister64::Rcx), src2: None };
    assert_eq!(instr.to_string(), "imul rcx");
}

#[test]
fn test_imul_two_operand_display() {
    // Test IMUL with two operands (dest and src)
    let instr = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rax)),
        src1: Operand::reg64(GPRegister64::Rbx),
        src2: None,
    };
    assert_eq!(instr.to_string(), "imul rax, rbx");
}

#[test]
fn test_imul_three_operand_display() {
    // Test IMUL with three operands (dest, src1, src2)
    let instr = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rax)),
        src1: Operand::reg64(GPRegister64::Rbx),
        src2: Some(Operand::imm32(42)),
    };
    assert_eq!(instr.to_string(), "imul rax, rbx, 42");
}

#[test]
fn test_imul_with_immediate_src1() {
    // Test IMUL with immediate as src1
    let instr =
        Instruction::Imul { dest: Some(Operand::reg32(GPRegister32::Eax)), src1: Operand::imm32(100), src2: None };
    assert_eq!(instr.to_string(), "imul eax, 100");
}

#[test]
fn test_imul_with_memory_operand() {
    // Test IMUL with memory operand
    let instr = Instruction::Imul { dest: None, src1: Operand::mem_disp(GPRegister64::Rbp, -8), src2: None };
    assert_eq!(instr.to_string(), "imul QWORD PTR [rbp - 8]");
}

#[test]
fn test_imul_negative_immediate() {
    // Test IMUL with negative immediate value
    let instr = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rdx)),
        src1: Operand::reg64(GPRegister64::Rsi),
        src2: Some(Operand::imm32(-10)),
    };
    assert_eq!(instr.to_string(), "imul rdx, rsi, -10");
}

// ============================================================================
// Jump Instructions Display Tests - All Conditional Variants
// ============================================================================

#[test]
fn test_jmp_unconditional_display() {
    // Test unconditional jump
    let instr = Instruction::Jmp { target: Operand::label("loop_start") };
    assert_eq!(instr.to_string(), "jmp loop_start");
    assert!(instr.is_jump());
    assert!(!instr.is_call());
    assert!(!instr.is_return());
}

#[test]
fn test_je_display() {
    // Test jump if equal
    let instr = Instruction::Je { target: Operand::label("equal_label") };
    assert_eq!(instr.to_string(), "je equal_label");
    assert!(instr.is_jump());
}

#[test]
fn test_jne_display() {
    // Test jump if not equal
    let instr = Instruction::Jne { target: Operand::label("not_equal") };
    assert_eq!(instr.to_string(), "jne not_equal");
    assert!(instr.is_jump());
}

#[test]
fn test_jz_display() {
    // Test jump if zero
    let instr = Instruction::Jz { target: Operand::label("zero_label") };
    assert_eq!(instr.to_string(), "jz zero_label");
    assert!(instr.is_jump());
}

#[test]
fn test_jnz_display() {
    // Test jump if not zero
    let instr = Instruction::Jnz { target: Operand::label("not_zero") };
    assert_eq!(instr.to_string(), "jnz not_zero");
    assert!(instr.is_jump());
}

#[test]
fn test_jg_display() {
    // Test jump if greater (signed)
    let instr = Instruction::Jg { target: Operand::label("greater") };
    assert_eq!(instr.to_string(), "jg greater");
    assert!(instr.is_jump());
}

#[test]
fn test_jge_display() {
    // Test jump if greater or equal (signed)
    let instr = Instruction::Jge { target: Operand::label("greater_equal") };
    assert_eq!(instr.to_string(), "jge greater_equal");
    assert!(instr.is_jump());
}

#[test]
fn test_jl_display() {
    // Test jump if less (signed)
    let instr = Instruction::Jl { target: Operand::label("less") };
    assert_eq!(instr.to_string(), "jl less");
    assert!(instr.is_jump());
}

#[test]
fn test_jle_display() {
    // Test jump if less or equal (signed)
    let instr = Instruction::Jle { target: Operand::label("less_equal") };
    assert_eq!(instr.to_string(), "jle less_equal");
    assert!(instr.is_jump());
}

#[test]
fn test_ja_display() {
    // Test jump if above (unsigned)
    let instr = Instruction::Ja { target: Operand::label("above") };
    assert_eq!(instr.to_string(), "ja above");
    assert!(instr.is_jump());
}

#[test]
fn test_jae_display() {
    // Test jump if above or equal (unsigned)
    let instr = Instruction::Jae { target: Operand::label("above_equal") };
    assert_eq!(instr.to_string(), "jae above_equal");
    assert!(instr.is_jump());
}

#[test]
fn test_jb_display() {
    // Test jump if below (unsigned)
    let instr = Instruction::Jb { target: Operand::label("below") };
    assert_eq!(instr.to_string(), "jb below");
    assert!(instr.is_jump());
}

#[test]
fn test_jbe_display() {
    // Test jump if below or equal (unsigned)
    let instr = Instruction::Jbe { target: Operand::label("below_equal") };
    assert_eq!(instr.to_string(), "jbe below_equal");
    assert!(instr.is_jump());
}

#[test]
fn test_js_display() {
    // Test jump if sign (SF=1)
    let instr = Instruction::Js { target: Operand::label("sign_set") };
    assert_eq!(instr.to_string(), "js sign_set");
    assert!(instr.is_jump());
}

#[test]
fn test_jns_display() {
    // Test jump if not sign (SF=0)
    let instr = Instruction::Jns { target: Operand::label("sign_clear") };
    assert_eq!(instr.to_string(), "jns sign_clear");
    assert!(instr.is_jump());
}

#[test]
fn test_jo_display() {
    // Test jump if overflow (OF=1)
    let instr = Instruction::Jo { target: Operand::label("overflow") };
    assert_eq!(instr.to_string(), "jo overflow");
    assert!(instr.is_jump());
}

#[test]
fn test_jno_display() {
    // Test jump if not overflow (OF=0)
    let instr = Instruction::Jno { target: Operand::label("no_overflow") };
    assert_eq!(instr.to_string(), "jno no_overflow");
    assert!(instr.is_jump());
}

#[test]
fn test_jp_display() {
    // Test jump if parity (PF=1)
    let instr = Instruction::Jp { target: Operand::label("parity") };
    assert_eq!(instr.to_string(), "jp parity");
    assert!(instr.is_jump());
}

#[test]
fn test_jnp_display() {
    // Test jump if not parity (PF=0)
    let instr = Instruction::Jnp { target: Operand::label("no_parity") };
    assert_eq!(instr.to_string(), "jnp no_parity");
    assert!(instr.is_jump());
}

// ============================================================================
// Call and Return Instructions Tests
// ============================================================================

#[test]
fn test_call_display() {
    // Test call instruction
    let instr = Instruction::Call { target: Operand::label("function") };
    assert_eq!(instr.to_string(), "call function");
    assert!(instr.is_call());
    assert!(!instr.is_jump());
    assert!(!instr.is_return());
}

#[test]
fn test_ret_plain_display() {
    // Test plain return instruction
    let instr = Instruction::Ret;
    assert_eq!(instr.to_string(), "ret");
    assert!(instr.is_return());
    assert!(!instr.is_jump());
    assert!(!instr.is_call());
}

#[test]
fn test_ret_with_immediate_display() {
    // Test return with immediate value (stack cleanup)
    let instr = Instruction::RetImm { imm: 16 };
    assert_eq!(instr.to_string(), "ret 16");
    assert!(instr.is_return());
}

#[test]
fn test_ret_with_zero_immediate() {
    // Test return with zero immediate (edge case)
    let instr = Instruction::RetImm { imm: 0 };
    assert_eq!(instr.to_string(), "ret 0");
    assert!(instr.is_return());
}

#[test]
fn test_ret_with_max_immediate() {
    // Test return with maximum u16 immediate
    let instr = Instruction::RetImm { imm: u16::MAX };
    assert_eq!(instr.to_string(), format!("ret {}", u16::MAX));
    assert!(instr.is_return());
}

// ============================================================================
// AVX Three-Operand Instructions Display Tests
// ============================================================================

#[test]
fn test_vaddps_display() {
    // Test AVX packed single-precision float addition
    let instr = Instruction::Vaddps {
        dest: Operand::xmm(XMMRegister::Xmm0),
        src1: Operand::xmm(XMMRegister::Xmm1),
        src2: Operand::xmm(XMMRegister::Xmm2),
    };
    assert_eq!(instr.to_string(), "vaddps xmm0, xmm1, xmm2");
}

#[test]
fn test_vaddpd_display() {
    // Test AVX packed double-precision float addition
    let instr = Instruction::Vaddpd {
        dest: Operand::ymm(YMMRegister::Ymm0),
        src1: Operand::ymm(YMMRegister::Ymm1),
        src2: Operand::ymm(YMMRegister::Ymm2),
    };
    assert_eq!(instr.to_string(), "vaddpd ymm0, ymm1, ymm2");
}

#[test]
fn test_vaddss_display() {
    // Test AVX scalar single-precision float addition
    let instr = Instruction::Vaddss {
        dest: Operand::xmm(XMMRegister::Xmm3),
        src1: Operand::xmm(XMMRegister::Xmm4),
        src2: Operand::xmm(XMMRegister::Xmm5),
    };
    assert_eq!(instr.to_string(), "vaddss xmm3, xmm4, xmm5");
}

#[test]
fn test_vaddsd_display() {
    // Test AVX scalar double-precision float addition
    let instr = Instruction::Vaddsd {
        dest: Operand::xmm(XMMRegister::Xmm6),
        src1: Operand::xmm(XMMRegister::Xmm7),
        src2: Operand::xmm(XMMRegister::Xmm8),
    };
    assert_eq!(instr.to_string(), "vaddsd xmm6, xmm7, xmm8");
}

#[test]
fn test_vsubps_display() {
    // Test AVX packed single-precision float subtraction
    let instr = Instruction::Vsubps {
        dest: Operand::ymm(YMMRegister::Ymm3),
        src1: Operand::ymm(YMMRegister::Ymm4),
        src2: Operand::ymm(YMMRegister::Ymm5),
    };
    assert_eq!(instr.to_string(), "vsubps ymm3, ymm4, ymm5");
}

#[test]
fn test_vsubpd_display() {
    // Test AVX packed double-precision float subtraction
    let instr = Instruction::Vsubpd {
        dest: Operand::xmm(XMMRegister::Xmm9),
        src1: Operand::xmm(XMMRegister::Xmm10),
        src2: Operand::xmm(XMMRegister::Xmm11),
    };
    assert_eq!(instr.to_string(), "vsubpd xmm9, xmm10, xmm11");
}

#[test]
fn test_vmulps_display() {
    // Test AVX packed single-precision float multiplication
    let instr = Instruction::Vmulps {
        dest: Operand::ymm(YMMRegister::Ymm6),
        src1: Operand::ymm(YMMRegister::Ymm7),
        src2: Operand::ymm(YMMRegister::Ymm8),
    };
    assert_eq!(instr.to_string(), "vmulps ymm6, ymm7, ymm8");
}

#[test]
fn test_vmulpd_display() {
    // Test AVX packed double-precision float multiplication
    let instr = Instruction::Vmulpd {
        dest: Operand::xmm(XMMRegister::Xmm12),
        src1: Operand::xmm(XMMRegister::Xmm13),
        src2: Operand::xmm(XMMRegister::Xmm14),
    };
    assert_eq!(instr.to_string(), "vmulpd xmm12, xmm13, xmm14");
}

#[test]
fn test_vdivps_display() {
    // Test AVX packed single-precision float division
    let instr = Instruction::Vdivps {
        dest: Operand::ymm(YMMRegister::Ymm9),
        src1: Operand::ymm(YMMRegister::Ymm10),
        src2: Operand::ymm(YMMRegister::Ymm11),
    };
    assert_eq!(instr.to_string(), "vdivps ymm9, ymm10, ymm11");
}

#[test]
fn test_vdivpd_display() {
    // Test AVX packed double-precision float division
    let instr = Instruction::Vdivpd {
        dest: Operand::xmm(XMMRegister::Xmm15),
        src1: Operand::xmm(XMMRegister::Xmm0),
        src2: Operand::xmm(XMMRegister::Xmm1),
    };
    assert_eq!(instr.to_string(), "vdivpd xmm15, xmm0, xmm1");
}

#[test]
fn test_avx_with_memory_operand() {
    // Test AVX instruction with memory operand as src2
    let instr = Instruction::Vaddps {
        dest: Operand::ymm(YMMRegister::Ymm0),
        src1: Operand::ymm(YMMRegister::Ymm1),
        src2: Operand::mem(GPRegister64::Rax),
    };
    assert_eq!(instr.to_string(), "vaddps ymm0, ymm1, QWORD PTR [rax]");
}

// ============================================================================
// FPU Instructions with Optional Operands Tests
// ============================================================================

#[test]
fn test_fadd_with_operand_display() {
    // Test FPU add with explicit operand
    let instr = Instruction::Fadd { src: Some(Operand::mem_disp(GPRegister64::Rbp, -16)) };
    assert_eq!(instr.to_string(), "fadd QWORD PTR [rbp - 16]");
}

#[test]
fn test_fadd_without_operand_display() {
    // Test FPU add without operand (ST(0) + ST(1) -> ST(0))
    let instr = Instruction::Fadd { src: None };
    assert_eq!(instr.to_string(), "fadd");
}

#[test]
fn test_faddp_with_operand_display() {
    // Test FPU add and pop with operand
    let instr = Instruction::Faddp { src: Some(Operand::mem(GPRegister64::Rsi)) };
    assert_eq!(instr.to_string(), "faddp QWORD PTR [rsi]");
}

#[test]
fn test_faddp_without_operand_display() {
    // Test FPU add and pop without operand
    let instr = Instruction::Faddp { src: None };
    assert_eq!(instr.to_string(), "faddp");
}

#[test]
fn test_fsub_with_operand_display() {
    // Test FPU subtract with operand
    let instr = Instruction::Fsub { src: Some(Operand::mem_disp(GPRegister64::Rdi, 8)) };
    assert_eq!(instr.to_string(), "fsub QWORD PTR [rdi + 8]");
}

#[test]
fn test_fsub_without_operand_display() {
    // Test FPU subtract without operand
    let instr = Instruction::Fsub { src: None };
    assert_eq!(instr.to_string(), "fsub");
}

#[test]
fn test_fsubp_with_operand_display() {
    // Test FPU subtract and pop with operand
    let instr = Instruction::Fsubp { src: Some(Operand::mem_disp(GPRegister64::Rsp, 8)) };
    assert_eq!(instr.to_string(), "fsubp QWORD PTR [rsp + 8]");
}

#[test]
fn test_fsubp_without_operand_display() {
    // Test FPU subtract and pop without operand
    let instr = Instruction::Fsubp { src: None };
    assert_eq!(instr.to_string(), "fsubp");
}

#[test]
fn test_fmul_with_operand_display() {
    // Test FPU multiply with operand
    let instr = Instruction::Fmul { src: Some(Operand::mem(GPRegister64::Rbx)) };
    assert_eq!(instr.to_string(), "fmul QWORD PTR [rbx]");
}

#[test]
fn test_fmul_without_operand_display() {
    // Test FPU multiply without operand
    let instr = Instruction::Fmul { src: None };
    assert_eq!(instr.to_string(), "fmul");
}

#[test]
fn test_fmulp_with_operand_display() {
    // Test FPU multiply and pop with operand
    let instr = Instruction::Fmulp { src: Some(Operand::mem_disp(GPRegister64::R10, 32)) };
    assert_eq!(instr.to_string(), "fmulp QWORD PTR [r10 + 32]");
}

#[test]
fn test_fmulp_without_operand_display() {
    // Test FPU multiply and pop without operand
    let instr = Instruction::Fmulp { src: None };
    assert_eq!(instr.to_string(), "fmulp");
}

#[test]
fn test_fdiv_with_operand_display() {
    // Test FPU divide with operand
    let instr = Instruction::Fdiv { src: Some(Operand::mem_disp(GPRegister64::R11, -24)) };
    assert_eq!(instr.to_string(), "fdiv QWORD PTR [r11 - 24]");
}

#[test]
fn test_fdiv_without_operand_display() {
    // Test FPU divide without operand
    let instr = Instruction::Fdiv { src: None };
    assert_eq!(instr.to_string(), "fdiv");
}

#[test]
fn test_fdivp_with_operand_display() {
    // Test FPU divide and pop with operand
    let instr = Instruction::Fdivp { src: Some(Operand::mem(GPRegister64::R12)) };
    assert_eq!(instr.to_string(), "fdivp QWORD PTR [r12]");
}

#[test]
fn test_fdivp_without_operand_display() {
    // Test FPU divide and pop without operand
    let instr = Instruction::Fdivp { src: None };
    assert_eq!(instr.to_string(), "fdivp");
}

// ============================================================================
// String Instructions Display Tests
// ============================================================================

#[test]
fn test_movsb_display() {
    // Test move string byte
    let instr = Instruction::Movsb;
    assert_eq!(instr.to_string(), "movsb");
    assert_eq!(instr.mnemonic(), "movsb");
}

#[test]
fn test_movsw_display() {
    // Test move string word
    let instr = Instruction::Movsw;
    assert_eq!(instr.to_string(), "movsw");
    assert_eq!(instr.mnemonic(), "movsw");
}

#[test]
fn test_movsd_string_display() {
    // Test move string doubleword
    let instr = Instruction::MovsdString;
    assert_eq!(instr.to_string(), "movsd");
    assert_eq!(instr.mnemonic(), "movsd");
}

#[test]
fn test_movsq_display() {
    // Test move string quadword
    let instr = Instruction::Movsq;
    assert_eq!(instr.to_string(), "movsq");
    assert_eq!(instr.mnemonic(), "movsq");
}

#[test]
fn test_stosb_display() {
    // Test store string byte
    let instr = Instruction::Stosb;
    assert_eq!(instr.to_string(), "stosb");
    assert_eq!(instr.mnemonic(), "stosb");
}

#[test]
fn test_stosw_display() {
    // Test store string word
    let instr = Instruction::Stosw;
    assert_eq!(instr.to_string(), "stosw");
    assert_eq!(instr.mnemonic(), "stosw");
}

#[test]
fn test_stosd_display() {
    // Test store string doubleword
    let instr = Instruction::Stosd;
    assert_eq!(instr.to_string(), "stosd");
    assert_eq!(instr.mnemonic(), "stosd");
}

#[test]
fn test_stosq_display() {
    // Test store string quadword
    let instr = Instruction::Stosq;
    assert_eq!(instr.to_string(), "stosq");
    assert_eq!(instr.mnemonic(), "stosq");
}

// ============================================================================
// SETcc Instructions Display Tests
// ============================================================================

#[test]
fn test_sete_display() {
    // Test set byte on equal
    let instr = Instruction::Sete { dest: Operand::reg8(GPRegister8::Al) };
    assert_eq!(instr.to_string(), "sete al");
}

#[test]
fn test_setne_display() {
    // Test set byte on not equal
    let instr = Instruction::Setne { dest: Operand::reg8(GPRegister8::Bl) };
    assert_eq!(instr.to_string(), "setne bl");
}

#[test]
fn test_setg_display() {
    // Test set byte on greater (signed)
    let instr = Instruction::Setg { dest: Operand::reg8(GPRegister8::Cl) };
    assert_eq!(instr.to_string(), "setg cl");
}

#[test]
fn test_setge_display() {
    // Test set byte on greater or equal (signed)
    let instr = Instruction::Setge { dest: Operand::reg8(GPRegister8::Dl) };
    assert_eq!(instr.to_string(), "setge dl");
}

#[test]
fn test_setl_display() {
    // Test set byte on less (signed)
    let instr = Instruction::Setl { dest: Operand::reg8(GPRegister8::Sil) };
    assert_eq!(instr.to_string(), "setl sil");
}

#[test]
fn test_setle_display() {
    // Test set byte on less or equal (signed)
    let instr = Instruction::Setle { dest: Operand::reg8(GPRegister8::Dil) };
    assert_eq!(instr.to_string(), "setle dil");
}

#[test]
fn test_seta_display() {
    // Test set byte on above (unsigned)
    let instr = Instruction::Seta { dest: Operand::reg8(GPRegister8::Bpl) };
    assert_eq!(instr.to_string(), "seta bpl");
}

#[test]
fn test_setae_display() {
    // Test set byte on above or equal (unsigned)
    let instr = Instruction::Setae { dest: Operand::reg8(GPRegister8::Spl) };
    assert_eq!(instr.to_string(), "setae spl");
}

#[test]
fn test_setb_display() {
    // Test set byte on below (unsigned)
    let instr = Instruction::Setb { dest: Operand::reg8(GPRegister8::R8b) };
    assert_eq!(instr.to_string(), "setb r8b");
}

#[test]
fn test_setbe_display() {
    // Test set byte on below or equal (unsigned)
    let instr = Instruction::Setbe { dest: Operand::reg8(GPRegister8::R9b) };
    assert_eq!(instr.to_string(), "setbe r9b");
}

#[test]
fn test_setcc_with_memory_dest() {
    // Test SETcc with memory destination
    let instr = Instruction::Sete { dest: Operand::mem_disp(GPRegister64::Rcx, 4) };
    assert_eq!(instr.to_string(), "sete QWORD PTR [rcx + 4]");
}

// ============================================================================
// Bit Manipulation Instructions Display Tests
// ============================================================================

#[test]
fn test_bsf_display() {
    // Test bit scan forward
    let instr = Instruction::Bsf { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(instr.to_string(), "bsf rax, rbx");
}

#[test]
fn test_bsr_display() {
    // Test bit scan reverse
    let instr = Instruction::Bsr { dest: Operand::reg32(GPRegister32::Ecx), src: Operand::reg32(GPRegister32::Edx) };
    assert_eq!(instr.to_string(), "bsr ecx, edx");
}

#[test]
fn test_bt_display() {
    // Test bit test
    let instr = Instruction::Bt { dest: Operand::reg64(GPRegister64::R8), src: Operand::imm8(15) };
    assert_eq!(instr.to_string(), "bt r8, 15");
}

#[test]
fn test_btc_display() {
    // Test bit test and complement
    let instr = Instruction::Btc { dest: Operand::reg32(GPRegister32::R9d), src: Operand::reg32(GPRegister32::R10d) };
    assert_eq!(instr.to_string(), "btc r9d, r10d");
}

#[test]
fn test_btr_display() {
    // Test bit test and reset
    let instr = Instruction::Btr { dest: Operand::reg64(GPRegister64::R11), src: Operand::imm8(31) };
    assert_eq!(instr.to_string(), "btr r11, 31");
}

#[test]
fn test_bts_display() {
    // Test bit test and set
    let instr = Instruction::Bts { dest: Operand::reg64(GPRegister64::R12), src: Operand::reg64(GPRegister64::R13) };
    assert_eq!(instr.to_string(), "bts r12, r13");
}

#[test]
fn test_popcnt_display() {
    // Test population count (count set bits)
    let instr = Instruction::Popcnt { dest: Operand::reg64(GPRegister64::R14), src: Operand::reg64(GPRegister64::R15) };
    assert_eq!(instr.to_string(), "popcnt r14, r15");
}

#[test]
fn test_lzcnt_display() {
    // Test leading zero count
    let instr = Instruction::Lzcnt { dest: Operand::reg32(GPRegister32::Eax), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(instr.to_string(), "lzcnt eax, QWORD PTR [rbx]");
}

#[test]
#[allow(clippy::unreadable_literal, clippy::cast_possible_wrap)]
fn test_tzcnt_display() {
    // Test trailing zero count
    let instr =
        Instruction::Tzcnt { dest: Operand::reg64(GPRegister64::Rcx), src: Operand::imm32(0xFF00FF00_u32 as i32) };
    assert_eq!(instr.to_string(), "tzcnt rcx, -16711936");
}

// ============================================================================
// Conditional Move (CMOVcc) Instructions Display Tests
// ============================================================================

#[test]
fn test_cmove_display() {
    // Test conditional move if equal
    let instr = Instruction::Cmove { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(instr.to_string(), "cmove rax, rbx");
}

#[test]
fn test_cmovne_display() {
    // Test conditional move if not equal
    let instr =
        Instruction::Cmovne { dest: Operand::reg32(GPRegister32::Ecx), src: Operand::mem_disp(GPRegister64::Rdx, 8) };
    assert_eq!(instr.to_string(), "cmovne ecx, QWORD PTR [rdx + 8]");
}

#[test]
fn test_cmovg_display() {
    // Test conditional move if greater (signed)
    let instr = Instruction::Cmovg { dest: Operand::reg64(GPRegister64::Rsi), src: Operand::reg64(GPRegister64::Rdi) };
    assert_eq!(instr.to_string(), "cmovg rsi, rdi");
}

#[test]
fn test_cmovge_display() {
    // Test conditional move if greater or equal (signed)
    let instr = Instruction::Cmovge { dest: Operand::reg64(GPRegister64::R8), src: Operand::reg64(GPRegister64::R9) };
    assert_eq!(instr.to_string(), "cmovge r8, r9");
}

#[test]
fn test_cmovl_display() {
    // Test conditional move if less (signed)
    let instr =
        Instruction::Cmovl { dest: Operand::reg32(GPRegister32::R10d), src: Operand::reg32(GPRegister32::R11d) };
    assert_eq!(instr.to_string(), "cmovl r10d, r11d");
}

#[test]
fn test_cmovle_display() {
    // Test conditional move if less or equal (signed)
    let instr = Instruction::Cmovle { dest: Operand::reg64(GPRegister64::R12), src: Operand::mem(GPRegister64::R13) };
    assert_eq!(instr.to_string(), "cmovle r12, QWORD PTR [r13]");
}

#[test]
fn test_cmova_display() {
    // Test conditional move if above (unsigned)
    let instr = Instruction::Cmova { dest: Operand::reg64(GPRegister64::R14), src: Operand::reg64(GPRegister64::R15) };
    assert_eq!(instr.to_string(), "cmova r14, r15");
}

#[test]
fn test_cmovae_display() {
    // Test conditional move if above or equal (unsigned)
    let instr = Instruction::Cmovae { dest: Operand::reg32(GPRegister32::Ebp), src: Operand::reg32(GPRegister32::Esp) };
    assert_eq!(instr.to_string(), "cmovae ebp, esp");
}

#[test]
fn test_cmovb_display() {
    // Test conditional move if below (unsigned)
    let instr = Instruction::Cmovb { dest: Operand::reg64(GPRegister64::Rbp), src: Operand::reg64(GPRegister64::Rsp) };
    assert_eq!(instr.to_string(), "cmovb rbp, rsp");
}

#[test]
fn test_cmovbe_display() {
    // Test conditional move if below or equal (unsigned)
    let instr =
        Instruction::Cmovbe { dest: Operand::reg64(GPRegister64::Rax), src: Operand::mem_disp(GPRegister64::Rbx, 16) };
    assert_eq!(instr.to_string(), "cmovbe rax, QWORD PTR [rbx + 16]");
}

// ============================================================================
// Helper Method Tests - is_jump, is_call, is_return
// ============================================================================

#[test]
fn test_is_jump_for_all_jump_variants() {
    // Test that all jump instructions return true for is_jump()
    let jumps = vec![
        Instruction::Jmp { target: Operand::label("l") },
        Instruction::Je { target: Operand::label("l") },
        Instruction::Jne { target: Operand::label("l") },
        Instruction::Jz { target: Operand::label("l") },
        Instruction::Jnz { target: Operand::label("l") },
        Instruction::Jg { target: Operand::label("l") },
        Instruction::Jge { target: Operand::label("l") },
        Instruction::Jl { target: Operand::label("l") },
        Instruction::Jle { target: Operand::label("l") },
        Instruction::Ja { target: Operand::label("l") },
        Instruction::Jae { target: Operand::label("l") },
        Instruction::Jb { target: Operand::label("l") },
        Instruction::Jbe { target: Operand::label("l") },
        Instruction::Js { target: Operand::label("l") },
        Instruction::Jns { target: Operand::label("l") },
        Instruction::Jo { target: Operand::label("l") },
        Instruction::Jno { target: Operand::label("l") },
        Instruction::Jp { target: Operand::label("l") },
        Instruction::Jnp { target: Operand::label("l") },
    ];

    for jump in jumps {
        assert!(jump.is_jump(), "{} should return true for is_jump()", jump.mnemonic());
        assert!(!jump.is_call(), "{} should return false for is_call()", jump.mnemonic());
        assert!(!jump.is_return(), "{} should return false for is_return()", jump.mnemonic());
    }
}

#[test]
fn test_non_jump_instructions_return_false() {
    // Test that non-jump instructions return false for is_jump()
    let non_jumps = vec![
        Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) },
        Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(5) },
        Instruction::Call { target: Operand::label("func") },
        Instruction::Ret,
        Instruction::Nop,
    ];

    for instr in non_jumps {
        assert!(!instr.is_jump(), "{} should return false for is_jump()", instr.mnemonic());
    }
}

#[test]
fn test_is_call_true_only_for_call() {
    // Test is_call() for call and non-call instructions
    let call = Instruction::Call { target: Operand::label("function") };
    assert!(call.is_call());
    assert!(!call.is_jump());
    assert!(!call.is_return());

    let non_call = Instruction::Jmp { target: Operand::label("label") };
    assert!(!non_call.is_call());
}

#[test]
fn test_is_return_for_both_ret_variants() {
    // Test is_return() for both return variants
    let ret_plain = Instruction::Ret;
    assert!(ret_plain.is_return());
    assert!(!ret_plain.is_jump());
    assert!(!ret_plain.is_call());

    let ret_imm = Instruction::RetImm { imm: 8 };
    assert!(ret_imm.is_return());
    assert!(!ret_imm.is_jump());
    assert!(!ret_imm.is_call());
}

// ============================================================================
// Mnemonic Tests for All Instruction Variants
// ============================================================================

#[test]
fn test_mnemonic_for_all_jumps() {
    // Test that mnemonic() returns correct string for all jump variants
    assert_eq!(Instruction::Jmp { target: Operand::label("l") }.mnemonic(), "jmp");
    assert_eq!(Instruction::Je { target: Operand::label("l") }.mnemonic(), "je");
    assert_eq!(Instruction::Jne { target: Operand::label("l") }.mnemonic(), "jne");
    assert_eq!(Instruction::Jz { target: Operand::label("l") }.mnemonic(), "jz");
    assert_eq!(Instruction::Jnz { target: Operand::label("l") }.mnemonic(), "jnz");
    assert_eq!(Instruction::Jg { target: Operand::label("l") }.mnemonic(), "jg");
    assert_eq!(Instruction::Jge { target: Operand::label("l") }.mnemonic(), "jge");
    assert_eq!(Instruction::Jl { target: Operand::label("l") }.mnemonic(), "jl");
    assert_eq!(Instruction::Jle { target: Operand::label("l") }.mnemonic(), "jle");
    assert_eq!(Instruction::Ja { target: Operand::label("l") }.mnemonic(), "ja");
    assert_eq!(Instruction::Jae { target: Operand::label("l") }.mnemonic(), "jae");
    assert_eq!(Instruction::Jb { target: Operand::label("l") }.mnemonic(), "jb");
    assert_eq!(Instruction::Jbe { target: Operand::label("l") }.mnemonic(), "jbe");
    assert_eq!(Instruction::Js { target: Operand::label("l") }.mnemonic(), "js");
    assert_eq!(Instruction::Jns { target: Operand::label("l") }.mnemonic(), "jns");
    assert_eq!(Instruction::Jo { target: Operand::label("l") }.mnemonic(), "jo");
    assert_eq!(Instruction::Jno { target: Operand::label("l") }.mnemonic(), "jno");
    assert_eq!(Instruction::Jp { target: Operand::label("l") }.mnemonic(), "jp");
    assert_eq!(Instruction::Jnp { target: Operand::label("l") }.mnemonic(), "jnp");
}

#[test]
fn test_mnemonic_for_avx_instructions() {
    // Test that AVX instructions return correct mnemonics
    assert_eq!(
        Instruction::Vaddps {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vaddps"
    );
    assert_eq!(
        Instruction::Vaddpd {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vaddpd"
    );
    assert_eq!(
        Instruction::Vaddss {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vaddss"
    );
    assert_eq!(
        Instruction::Vaddsd {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vaddsd"
    );
    assert_eq!(
        Instruction::Vsubps {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vsubps"
    );
    assert_eq!(
        Instruction::Vsubpd {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vsubpd"
    );
    assert_eq!(
        Instruction::Vmulps {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vmulps"
    );
    assert_eq!(
        Instruction::Vmulpd {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vmulpd"
    );
    assert_eq!(
        Instruction::Vdivps {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vdivps"
    );
    assert_eq!(
        Instruction::Vdivpd {
            dest: Operand::xmm(XMMRegister::Xmm0),
            src1: Operand::xmm(XMMRegister::Xmm1),
            src2: Operand::xmm(XMMRegister::Xmm2)
        }
        .mnemonic(),
        "vdivpd"
    );
}

#[test]
fn test_mnemonic_for_string_instructions() {
    // Test that string instructions return correct mnemonics
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
fn test_mnemonic_for_special_instructions() {
    // Test that special instructions return correct mnemonics (lines 273-279)
    assert_eq!(Instruction::Cdq.mnemonic(), "cdq");
    assert_eq!(Instruction::Syscall.mnemonic(), "syscall");
    assert_eq!(Instruction::Sysret.mnemonic(), "sysret");
}

#[test]
fn test_mnemonic_for_sse_move_instructions() {
    // Test SSE move instructions mnemonics (lines 276-279)
    assert_eq!(
        Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) }.mnemonic(),
        "movaps"
    );
    assert_eq!(
        Instruction::Movapd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) }.mnemonic(),
        "movapd"
    );
    assert_eq!(
        Instruction::Movups { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) }.mnemonic(),
        "movups"
    );
    assert_eq!(
        Instruction::Movupd { dest: Operand::xmm(XMMRegister::Xmm6), src: Operand::xmm(XMMRegister::Xmm7) }.mnemonic(),
        "movupd"
    );
}

// ============================================================================
// Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_special_instruction_display() {
    // Test display formatting for special instructions (Cdq, Syscall, Sysret)
    let cdq = Instruction::Cdq;
    assert_eq!(cdq.to_string(), "cdq");
    assert_eq!(cdq.mnemonic(), "cdq");

    let syscall = Instruction::Syscall;
    assert_eq!(syscall.to_string(), "syscall");
    assert_eq!(syscall.mnemonic(), "syscall");

    let sysret = Instruction::Sysret;
    assert_eq!(sysret.to_string(), "sysret");
    assert_eq!(sysret.mnemonic(), "sysret");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_aligned_move_display() {
    // Test SSE aligned move instructions display
    let movaps = Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(movaps.to_string(), "movaps xmm0, QWORD PTR [rax]");
    assert_eq!(movaps.mnemonic(), "movaps");

    let movapd = Instruction::Movapd { dest: Operand::xmm(XMMRegister::Xmm1), src: Operand::xmm(XMMRegister::Xmm2) };
    assert_eq!(movapd.to_string(), "movapd xmm1, xmm2");
    assert_eq!(movapd.mnemonic(), "movapd");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_unaligned_move_display() {
    // Test SSE unaligned move instructions display
    let movups =
        Instruction::Movups { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::mem_disp(GPRegister64::Rbx, 16) };
    assert_eq!(movups.to_string(), "movups xmm3, QWORD PTR [rbx + 16]");
    assert_eq!(movups.mnemonic(), "movups");

    let movupd =
        Instruction::Movupd { dest: Operand::mem_disp(GPRegister64::Rcx, -8), src: Operand::xmm(XMMRegister::Xmm4) };
    assert_eq!(movupd.to_string(), "movupd QWORD PTR [rcx - 8], xmm4");
    assert_eq!(movupd.mnemonic(), "movupd");
}

#[test]
fn test_instruction_clone() {
    // Test that instructions can be cloned properly
    let original = Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(42) };
    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(original.to_string(), cloned.to_string());
}

#[test]
fn test_instruction_debug() {
    // Test that Debug formatting works
    let instr = Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    let debug_str = format!("{instr:?}");
    assert!(debug_str.contains("Mov"));
}

#[test]
fn test_instruction_equality() {
    // Test PartialEq implementation
    let instr1 = Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(5) };
    let instr2 = Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(5) };
    let instr3 = Instruction::Add { dest: Operand::reg64(GPRegister64::Rbx), src: Operand::imm32(5) };

    assert_eq!(instr1, instr2);
    assert_ne!(instr1, instr3);
}

#[test]
fn test_display_with_complex_memory_operands() {
    // Test instruction display with complex memory addressing modes
    let instr = Instruction::Add { dest: Operand::mem_disp(GPRegister64::Rbp, -16), src: Operand::imm32(100) };
    assert_eq!(instr.to_string(), "add QWORD PTR [rbp - 16], 100");
}

// ============================================================================
// Additional Coverage Tests for Lines 273-276
// ============================================================================

#[test]
fn test_cdq_mnemonic_direct() {
    // Direct test for Cdq mnemonic (line 273)
    let cdq = Instruction::Cdq;
    let mnemonic = cdq.mnemonic();
    assert_eq!(mnemonic, "cdq");
    assert_eq!(cdq.to_string(), "cdq");
    // Verify it's not a jump, call, or return
    assert!(!cdq.is_jump());
    assert!(!cdq.is_call());
    assert!(!cdq.is_return());
}

#[test]
fn test_syscall_mnemonic_direct() {
    // Direct test for Syscall mnemonic (line 274)
    let syscall = Instruction::Syscall;
    let mnemonic = syscall.mnemonic();
    assert_eq!(mnemonic, "syscall");
    assert_eq!(syscall.to_string(), "syscall");
    // Verify it's not a jump, call, or return
    assert!(!syscall.is_jump());
    assert!(!syscall.is_call());
    assert!(!syscall.is_return());
}

#[test]
fn test_sysret_mnemonic_direct() {
    // Direct test for Sysret mnemonic (line 275)
    let sysret = Instruction::Sysret;
    let mnemonic = sysret.mnemonic();
    assert_eq!(mnemonic, "sysret");
    assert_eq!(sysret.to_string(), "sysret");
    // Verify it's not a jump, call, or return
    assert!(!sysret.is_jump());
    assert!(!sysret.is_call());
    assert!(!sysret.is_return());
}

#[test]
fn test_movaps_mnemonic_direct() {
    // Direct test for Movaps mnemonic (line 276)
    let movaps = Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    let mnemonic = movaps.mnemonic();
    assert_eq!(mnemonic, "movaps");
    assert_eq!(movaps.to_string(), "movaps xmm0, xmm1");
    // Verify it's not a jump, call, or return
    assert!(!movaps.is_jump());
    assert!(!movaps.is_call());
    assert!(!movaps.is_return());
}

#[test]
fn test_special_instructions_batch_mnemonic() {
    // Batch test for lines 273-276 to ensure all are covered
    let instructions = vec![
        (Instruction::Cdq, "cdq"),
        (Instruction::Syscall, "syscall"),
        (Instruction::Sysret, "sysret"),
        (
            Instruction::Movaps {
                dest: Operand::xmm(XMMRegister::Xmm15),
                src: Operand::mem_disp(GPRegister64::R15, 64),
            },
            "movaps",
        ),
    ];

    for (instr, expected_mnemonic) in instructions {
        assert_eq!(instr.mnemonic(), expected_mnemonic);
        assert!(!instr.is_jump());
        assert!(!instr.is_call());
        assert!(!instr.is_return());
    }
}

#[test]
fn test_cdq_display_formatting() {
    // Comprehensive test for Cdq display and formatting
    let cdq = Instruction::Cdq;

    // Test mnemonic
    assert_eq!(cdq.mnemonic(), "cdq");

    // Test Display trait
    let display_str = format!("{cdq}");
    assert_eq!(display_str, "cdq");

    // Test Debug trait
    let debug_str = format!("{cdq:?}");
    assert!(debug_str.contains("Cdq"));

    // Test clone
    let cloned = cdq.clone();
    assert_eq!(cdq, cloned);
}

#[test]
fn test_syscall_sysret_display_formatting() {
    // Comprehensive test for Syscall and Sysret
    let syscall = Instruction::Syscall;
    let sysret = Instruction::Sysret;

    // Test mnemonics
    assert_eq!(syscall.mnemonic(), "syscall");
    assert_eq!(sysret.mnemonic(), "sysret");

    // Test Display trait
    assert_eq!(format!("{syscall}"), "syscall");
    assert_eq!(format!("{sysret}"), "sysret");

    // Test inequality
    assert_ne!(syscall, sysret);

    // Test clone and equality
    let syscall_clone = syscall.clone();
    let sysret_clone = sysret.clone();
    assert_eq!(syscall, syscall_clone);
    assert_eq!(sysret, sysret_clone);
}

#[test]
fn test_movaps_various_operands() {
    // Test Movaps with various operand combinations to ensure line 276 coverage

    // Register to register
    let movaps1 = Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm15) };
    assert_eq!(movaps1.mnemonic(), "movaps");
    assert_eq!(movaps1.to_string(), "movaps xmm0, xmm15");

    // Memory to register
    let movaps2 = Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm7), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(movaps2.mnemonic(), "movaps");
    assert_eq!(movaps2.to_string(), "movaps xmm7, QWORD PTR [rbx]");

    // Register to memory with displacement
    let movaps3 =
        Instruction::Movaps { dest: Operand::mem_disp(GPRegister64::Rsp, 128), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_eq!(movaps3.mnemonic(), "movaps");
    assert_eq!(movaps3.to_string(), "movaps QWORD PTR [rsp + 128], xmm3");

    // Negative displacement
    let movaps4 =
        Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm14), src: Operand::mem_disp(GPRegister64::Rbp, -32) };
    assert_eq!(movaps4.mnemonic(), "movaps");
    assert_eq!(movaps4.to_string(), "movaps xmm14, QWORD PTR [rbp - 32]");
}

// ============================================================================
// Control Instructions - Hlt, Cpuid, Pause (Lines 273-275)
// ============================================================================

#[test]
fn test_hlt_instruction_display() {
    // Test HLT (Halt) instruction display and mnemonic.
    // HLT stops instruction execution and places the processor in a HALT state.
    // This is a privileged instruction typically used in operating systems.
    let hlt = Instruction::Hlt;
    assert_eq!(hlt.to_string(), "hlt", "HLT display should be 'hlt'");
    assert_eq!(hlt.mnemonic(), "hlt", "HLT mnemonic should be 'hlt'");
}

#[test]
fn test_cpuid_instruction_display() {
    // Test CPUID instruction display and mnemonic.
    // CPUID provides processor identification and feature information.
    // It uses EAX as input and returns information in EAX, EBX, ECX, and EDX.
    let cpuid = Instruction::Cpuid;
    assert_eq!(cpuid.to_string(), "cpuid", "CPUID display should be 'cpuid'");
    assert_eq!(cpuid.mnemonic(), "cpuid", "CPUID mnemonic should be 'cpuid'");
}

#[test]
fn test_pause_instruction_display() {
    // Test PAUSE instruction display and mnemonic.
    // PAUSE improves performance in spin-wait loops by hinting the processor
    // that the code sequence is a spin-wait loop. This reduces power consumption
    // and improves performance on simultaneous multithreading processors.
    let pause = Instruction::Pause;
    assert_eq!(pause.to_string(), "pause", "PAUSE display should be 'pause'");
    assert_eq!(pause.mnemonic(), "pause", "PAUSE mnemonic should be 'pause'");
}

#[test]
fn test_cqo_instruction_display() {
    // Test CQO (Convert Quadword to Octword) instruction display and mnemonic.
    // CQO sign-extends RAX into RDX:RAX (used before signed division).
    // This is the 64-bit version of the sign extension instructions.
    let cqo = Instruction::Cqo;
    assert_eq!(cqo.to_string(), "cqo", "CQO display should be 'cqo'");
    assert_eq!(cqo.mnemonic(), "cqo", "CQO mnemonic should be 'cqo'");
}

// ============================================================================
// Control Instructions - Comprehensive Edge Cases
// ============================================================================

#[test]
fn test_control_instructions_mnemonic_consistency() {
    // Verify that all control instructions have consistent mnemonic() results
    // with their Display implementation for reliable code generation.
    let instructions = vec![
        (Instruction::Hlt, "hlt"),
        (Instruction::Cpuid, "cpuid"),
        (Instruction::Pause, "pause"),
        (Instruction::Cqo, "cqo"),
        (Instruction::Nop, "nop"),
    ];

    for (instr, expected_mnemonic) in instructions {
        assert_eq!(instr.mnemonic(), expected_mnemonic, "Mnemonic mismatch for {instr:?}");
        assert_eq!(instr.to_string(), expected_mnemonic, "Display mismatch for {instr:?}");
    }
}

#[test]
fn test_control_instructions_clone_and_equality() {
    // Test that control instructions correctly implement Clone and PartialEq.
    // This ensures they can be used in collections and compared reliably.
    let hlt1 = Instruction::Hlt;
    let hlt2 = hlt1.clone();
    assert_eq!(hlt1, hlt2, "Cloned HLT should equal original");

    let cpuid1 = Instruction::Cpuid;
    let cpuid2 = cpuid1.clone();
    assert_eq!(cpuid1, cpuid2, "Cloned CPUID should equal original");

    let pause1 = Instruction::Pause;
    let pause2 = pause1.clone();
    assert_eq!(pause1, pause2, "Cloned PAUSE should equal original");

    let cqo1 = Instruction::Cqo;
    let cqo2 = cqo1.clone();
    assert_eq!(cqo1, cqo2, "Cloned CQO should equal original");
}

#[test]
fn test_control_instructions_not_jump_call_return() {
    // Verify that control instructions (Hlt, Cpuid, Pause, Cqo) are not
    // classified as jumps, calls, or returns.
    let instructions = vec![Instruction::Hlt, Instruction::Cpuid, Instruction::Pause, Instruction::Cqo];

    for instr in instructions {
        assert!(!instr.is_jump(), "{instr:?} should not be a jump");
        assert!(!instr.is_call(), "{instr:?} should not be a call");
        assert!(!instr.is_return(), "{instr:?} should not be a return");
    }
}

#[test]
fn test_control_instructions_debug_formatting() {
    // Test that control instructions have proper Debug trait implementation.
    // This is essential for debugging and logging purposes.
    let hlt = Instruction::Hlt;
    let debug_str = format!("{hlt:?}");
    assert!(debug_str.contains("Hlt"), "Debug output should contain 'Hlt'");

    let cpuid = Instruction::Cpuid;
    let debug_str = format!("{cpuid:?}");
    assert!(debug_str.contains("Cpuid"), "Debug output should contain 'Cpuid'");

    let pause = Instruction::Pause;
    let debug_str = format!("{pause:?}");
    assert!(debug_str.contains("Pause"), "Debug output should contain 'Pause'");

    let cqo = Instruction::Cqo;
    let debug_str = format!("{cqo:?}");
    assert!(debug_str.contains("Cqo"), "Debug output should contain 'Cqo'");
}

#[test]
fn test_control_instructions_in_vectors() {
    // Test that control instructions can be stored in collections.
    // This verifies they work correctly in common Rust data structures.
    let mut instructions =
        vec![Instruction::Nop, Instruction::Hlt, Instruction::Cpuid, Instruction::Pause, Instruction::Cqo];

    assert_eq!(instructions.len(), 5);
    assert_eq!(instructions[1], Instruction::Hlt);
    assert_eq!(instructions[2], Instruction::Cpuid);
    assert_eq!(instructions[3], Instruction::Pause);
    assert_eq!(instructions[4], Instruction::Cqo);

    // Test that we can search for control instructions
    assert!(instructions.contains(&Instruction::Hlt));
    assert!(instructions.contains(&Instruction::Cpuid));
    assert!(instructions.contains(&Instruction::Pause));
    assert!(instructions.contains(&Instruction::Cqo));

    // Test removal
    instructions.retain(|i| !matches!(i, Instruction::Hlt));
    assert!(!instructions.contains(&Instruction::Hlt));
    assert_eq!(instructions.len(), 4);
}

#[test]
fn test_control_instructions_match_patterns() {
    // Test that control instructions can be matched correctly in pattern matching.
    // This ensures proper enum variant discrimination.
    let test_instruction = |instr: Instruction| -> &str {
        match instr {
            Instruction::Hlt => "halt",
            Instruction::Cpuid => "cpu_info",
            Instruction::Pause => "spin_wait",
            Instruction::Cqo => "sign_extend",
            _ => "other",
        }
    };

    assert_eq!(test_instruction(Instruction::Hlt), "halt");
    assert_eq!(test_instruction(Instruction::Cpuid), "cpu_info");
    assert_eq!(test_instruction(Instruction::Pause), "spin_wait");
    assert_eq!(test_instruction(Instruction::Cqo), "sign_extend");
    assert_eq!(test_instruction(Instruction::Nop), "other");
}

// ============================================================================
// Control Instructions - Semantic Correctness Tests
// ============================================================================

#[test]
fn test_hlt_semantic_properties() {
    // HLT instruction should halt execution - verify it's recognized correctly.
    // In a real system, HLT requires CPL=0 (kernel mode).
    let hlt = Instruction::Hlt;

    // Verify basic properties
    assert_eq!(hlt.mnemonic(), "hlt");

    // HLT should not be confused with other control flow instructions
    assert!(!hlt.is_jump());
    assert!(!hlt.is_call());
    assert!(!hlt.is_return());

    // HLT is a terminating instruction (conceptually)
    // but not in the same category as RET
    assert_ne!(hlt, Instruction::Ret);
}

#[test]
fn test_cpuid_semantic_properties() {
    // CPUID provides CPU identification and feature information.
    // It's a special instruction that accesses processor-specific data.
    let cpuid = Instruction::Cpuid;

    assert_eq!(cpuid.mnemonic(), "cpuid");

    // CPUID is not a control flow instruction
    assert!(!cpuid.is_jump());
    assert!(!cpuid.is_call());
    assert!(!cpuid.is_return());

    // CPUID is a data query instruction, not a control instruction
    assert_ne!(cpuid, Instruction::Nop);
    assert_ne!(cpuid, Instruction::Hlt);
}

#[test]
fn test_pause_semantic_properties() {
    // PAUSE is a hint for spin-wait loops in multithreaded code.
    // It improves processor efficiency and reduces power consumption.
    let pause = Instruction::Pause;

    assert_eq!(pause.mnemonic(), "pause");

    // PAUSE does not affect control flow
    assert!(!pause.is_jump());
    assert!(!pause.is_call());
    assert!(!pause.is_return());

    // PAUSE is distinct from NOP although both are "hint" instructions
    assert_ne!(pause, Instruction::Nop);
}

#[test]
fn test_cqo_semantic_properties() {
    // CQO sign-extends RAX into RDX:RAX for signed 64-bit division.
    // It's the 64-bit equivalent of CDQ (32-bit) and CWD (16-bit).
    let cqo = Instruction::Cqo;

    assert_eq!(cqo.mnemonic(), "cqo");

    // CQO is an arithmetic setup instruction, not control flow
    assert!(!cqo.is_jump());
    assert!(!cqo.is_call());
    assert!(!cqo.is_return());

    // CQO is related to but distinct from CDQ
    assert_ne!(cqo, Instruction::Cdq);
}

#[test]
fn test_control_instructions_formatting_consistency() {
    // Ensure that Display and mnemonic() always produce identical output
    // for control instructions, as they have no operands.
    let instructions = vec![Instruction::Hlt, Instruction::Cpuid, Instruction::Pause, Instruction::Cqo];

    for instr in instructions {
        let display = instr.to_string();
        let mnemonic = instr.mnemonic();
        assert_eq!(display, mnemonic, "Display and mnemonic should be identical for {instr:?}");

        // Verify lowercase (x86 convention)
        assert_eq!(display, display.to_lowercase(), "Mnemonic should be lowercase for {instr:?}");
    }
}

// ============================================================================
// SSE Scalar Subtraction Instructions - Subss/Subsd (Lines 294-295)
// ============================================================================

#[test]
fn test_subss_instruction_display() {
    // Test SUBSS (Subtract Scalar Single-Precision Floating-Point) instruction.
    // SUBSS subtracts a single-precision floating-point value from another.
    let instr = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(instr.to_string(), "subss xmm0, xmm1", "SUBSS register-to-register display");
    assert_eq!(instr.mnemonic(), "subss", "SUBSS mnemonic should be 'subss'");
}

#[test]
fn test_subss_with_memory_operand() {
    // Test SUBSS with memory operand as source.
    let instr = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(instr.to_string(), "subss xmm2, QWORD PTR [rax]", "SUBSS with memory source");
    assert_eq!(instr.mnemonic(), "subss");
}

#[test]
fn test_subss_with_memory_displacement() {
    // Test SUBSS with memory operand using displacement.
    let instr =
        Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm7), src: Operand::mem_disp(GPRegister64::Rbp, -16) };
    assert_eq!(instr.to_string(), "subss xmm7, QWORD PTR [rbp - 16]", "SUBSS with negative displacement");
}

#[test]
fn test_subss_with_positive_displacement() {
    // Test SUBSS with positive memory displacement.
    let instr =
        Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm15), src: Operand::mem_disp(GPRegister64::Rsp, 64) };
    assert_eq!(instr.to_string(), "subss xmm15, QWORD PTR [rsp + 64]", "SUBSS with positive displacement");
}

#[test]
fn test_subsd_instruction_display() {
    // Test SUBSD (Subtract Scalar Double-Precision Floating-Point) instruction.
    // SUBSD subtracts a double-precision floating-point value from another.
    let instr = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::xmm(XMMRegister::Xmm4) };
    assert_eq!(instr.to_string(), "subsd xmm3, xmm4", "SUBSD register-to-register display");
    assert_eq!(instr.mnemonic(), "subsd", "SUBSD mnemonic should be 'subsd'");
}

#[test]
fn test_subsd_with_memory_operand() {
    // Test SUBSD with memory operand as source.
    let instr = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm5), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(instr.to_string(), "subsd xmm5, QWORD PTR [rbx]", "SUBSD with memory source");
    assert_eq!(instr.mnemonic(), "subsd");
}

#[test]
fn test_subsd_with_memory_displacement() {
    // Test SUBSD with memory operand using displacement.
    let instr =
        Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm8), src: Operand::mem_disp(GPRegister64::Rdx, -32) };
    assert_eq!(instr.to_string(), "subsd xmm8, QWORD PTR [rdx - 32]", "SUBSD with negative displacement");
}

#[test]
fn test_subsd_with_large_displacement() {
    // Test SUBSD with large positive memory displacement.
    let instr =
        Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm14), src: Operand::mem_disp(GPRegister64::Rsi, 256) };
    assert_eq!(instr.to_string(), "subsd xmm14, QWORD PTR [rsi + 256]", "SUBSD with large displacement");
}

// ============================================================================
// SSE Scalar Subtraction - Comprehensive Edge Cases
// ============================================================================

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_mnemonic_consistency() {
    // Verify mnemonic consistency for SSE scalar subtraction instructions.
    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(subss.mnemonic(), "subss");
    assert_eq!(subss.to_string(), "subss xmm0, xmm1");

    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_eq!(subsd.mnemonic(), "subsd");
    assert_eq!(subsd.to_string(), "subsd xmm2, xmm3");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_clone_and_equality() {
    // Test Clone and PartialEq for SSE scalar subtraction instructions.
    let subss1 = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm10), src: Operand::xmm(XMMRegister::Xmm11) };
    let subss2 = subss1.clone();
    assert_eq!(subss1, subss2, "Cloned SUBSS should equal original");

    let subsd1 = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm12), src: Operand::xmm(XMMRegister::Xmm13) };
    let subsd2 = subsd1.clone();
    assert_eq!(subsd1, subsd2, "Cloned SUBSD should equal original");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_not_control_flow() {
    // Verify SSE scalar subtraction instructions are not control flow instructions.
    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert!(!subss.is_jump(), "SUBSS should not be a jump");
    assert!(!subss.is_call(), "SUBSS should not be a call");
    assert!(!subss.is_return(), "SUBSS should not be a return");

    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert!(!subsd.is_jump(), "SUBSD should not be a jump");
    assert!(!subsd.is_call(), "SUBSD should not be a call");
    assert!(!subsd.is_return(), "SUBSD should not be a return");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_debug_formatting() {
    // Test Debug trait implementation for SSE scalar subtraction instructions.
    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) };
    let debug_str = format!("{subss:?}");
    assert!(debug_str.contains("Subss"), "Debug output should contain 'Subss'");
    assert!(debug_str.contains("Xmm4"), "Debug output should contain dest register");
    assert!(debug_str.contains("Xmm5"), "Debug output should contain src register");

    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm6), src: Operand::xmm(XMMRegister::Xmm7) };
    let debug_str = format!("{subsd:?}");
    assert!(debug_str.contains("Subsd"), "Debug output should contain 'Subsd'");
}

#[test]
fn test_sse_scalar_sub_in_collections() {
    // Test that SSE scalar subtraction instructions work in collections.
    let instructions = [
        Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) },
        Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) },
    ];
    assert_eq!(instructions.len(), 2);
    assert_eq!(instructions[0].mnemonic(), "subss");
    assert_eq!(instructions[1].mnemonic(), "subsd");

    // Test searching
    let subss_search =
        Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert!(instructions.contains(&subss_search));
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_pattern_matching() {
    // Test pattern matching for SSE scalar subtraction instructions.
    let classify = |instr: Instruction| -> &str {
        match instr {
            Instruction::Subss { .. } => "scalar_single",
            Instruction::Subsd { .. } => "scalar_double",
            _ => "other",
        }
    };

    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm8), src: Operand::xmm(XMMRegister::Xmm9) };
    assert_eq!(classify(subss), "scalar_single");

    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm10), src: Operand::xmm(XMMRegister::Xmm11) };
    assert_eq!(classify(subsd), "scalar_double");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_all_xmm_registers() {
    // Test SUBSS and SUBSD with all available XMM registers.
    // This ensures proper formatting across the entire register range.
    let xmm_regs = vec![
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

    for (i, &dest_reg) in xmm_regs.iter().enumerate() {
        let src_reg = xmm_regs[(i + 1) % xmm_regs.len()];

        // Test SUBSS
        let subss = Instruction::Subss { dest: Operand::xmm(dest_reg), src: Operand::xmm(src_reg) };
        assert_eq!(subss.mnemonic(), "subss");
        assert!(subss.to_string().starts_with("subss"));

        // Test SUBSD
        let subsd = Instruction::Subsd { dest: Operand::xmm(dest_reg), src: Operand::xmm(src_reg) };
        assert_eq!(subsd.mnemonic(), "subsd");
        assert!(subsd.to_string().starts_with("subsd"));
    }
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_semantic_correctness() {
    // Verify semantic properties of SSE scalar subtraction instructions.
    // SUBSS operates on single-precision (32-bit) floating-point values.
    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(subss.mnemonic(), "subss");
    assert!(subss.to_string().contains("subss"), "SUBSS should format with 'subss' mnemonic");

    // SUBSD operates on double-precision (64-bit) floating-point values.
    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm1), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(subsd.mnemonic(), "subsd");
    assert!(subsd.to_string().contains("subsd"), "SUBSD should format with 'subsd' mnemonic");

    // Verify they are distinct instructions
    assert_ne!(subss, subsd, "SUBSS and SUBSD should be different instructions");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_operand_ordering() {
    // Verify that operand ordering is correct: dest, src (Intel syntax).
    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    let display = subss.to_string();
    // Intel syntax: dest comes first, then source
    assert!(display.starts_with("subss xmm2,"), "Destination should come first");
    assert!(display.ends_with("xmm3"), "Source should come last");

    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) };
    let display = subsd.to_string();
    assert!(display.starts_with("subsd xmm4,"), "Destination should come first");
    assert!(display.ends_with("xmm5"), "Source should come last");
}

#[test]
fn test_sse_scalar_sub_with_all_memory_forms() {
    // Test SUBSS and SUBSD with various memory addressing modes.

    // Direct memory reference
    let subss_mem = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem(GPRegister64::Rdi) };
    assert!(subss_mem.to_string().contains("[rdi]"));

    // Memory with negative displacement
    let subss_neg =
        Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm1), src: Operand::mem_disp(GPRegister64::Rbp, -8) };
    assert!(subss_neg.to_string().contains("rbp - 8"));

    // Memory with large positive displacement
    let subsd_pos =
        Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::mem_disp(GPRegister64::Rsp, 128) };
    assert!(subsd_pos.to_string().contains("rsp + 128"));

    // Memory with zero displacement (should omit displacement)
    let subsd_zero = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::mem(GPRegister64::Rcx) };
    assert!(subsd_zero.to_string().contains("[rcx]"));
    assert!(!subsd_zero.to_string().contains("+ 0"));
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_scalar_sub_lowercase_formatting() {
    // Verify that mnemonics are lowercase (x86 convention).
    let subss = Instruction::Subss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    let display = subss.to_string();
    assert_eq!(display.split_whitespace().next().unwrap(), "subss", "Mnemonic should be lowercase");

    let subsd = Instruction::Subsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    let display = subsd.to_string();
    assert_eq!(display.split_whitespace().next().unwrap(), "subsd", "Mnemonic should be lowercase");
}

// ============================================================================
// SSE Packed Multiplication Instructions - Mulps/Mulpd (Lines 298-299)
// ============================================================================

#[test]
fn test_mulps_instruction_display() {
    // Test MULPS (Multiply Packed Single-Precision Floating-Point) instruction.
    // MULPS multiplies four single-precision floating-point values in parallel.
    let instr = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(instr.to_string(), "mulps xmm0, xmm1", "MULPS register-to-register display");
    assert_eq!(instr.mnemonic(), "mulps", "MULPS mnemonic should be 'mulps'");
}

#[test]
fn test_mulps_with_memory_operand() {
    // Test MULPS with memory operand as source.
    // Memory operand must be 128-bit aligned for optimal performance.
    let instr = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(instr.to_string(), "mulps xmm2, QWORD PTR [rax]", "MULPS with memory source");
    assert_eq!(instr.mnemonic(), "mulps");
}

#[test]
fn test_mulps_with_memory_displacement() {
    // Test MULPS with memory operand using displacement.
    let instr =
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm7), src: Operand::mem_disp(GPRegister64::Rbp, -16) };
    assert_eq!(instr.to_string(), "mulps xmm7, QWORD PTR [rbp - 16]", "MULPS with negative displacement");
}

#[test]
fn test_mulps_with_positive_displacement() {
    // Test MULPS with positive memory displacement.
    let instr =
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm15), src: Operand::mem_disp(GPRegister64::Rsp, 64) };
    assert_eq!(instr.to_string(), "mulps xmm15, QWORD PTR [rsp + 64]", "MULPS with positive displacement");
}

#[test]
fn test_mulpd_instruction_display() {
    // Test MULPD (Multiply Packed Double-Precision Floating-Point) instruction.
    // MULPD multiplies two double-precision floating-point values in parallel.
    let instr = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::xmm(XMMRegister::Xmm4) };
    assert_eq!(instr.to_string(), "mulpd xmm3, xmm4", "MULPD register-to-register display");
    assert_eq!(instr.mnemonic(), "mulpd", "MULPD mnemonic should be 'mulpd'");
}

#[test]
fn test_mulpd_with_memory_operand() {
    // Test MULPD with memory operand as source.
    let instr = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm5), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(instr.to_string(), "mulpd xmm5, QWORD PTR [rbx]", "MULPD with memory source");
    assert_eq!(instr.mnemonic(), "mulpd");
}

#[test]
fn test_mulpd_with_memory_displacement() {
    // Test MULPD with memory operand using displacement.
    let instr =
        Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm8), src: Operand::mem_disp(GPRegister64::Rdx, -32) };
    assert_eq!(instr.to_string(), "mulpd xmm8, QWORD PTR [rdx - 32]", "MULPD with negative displacement");
}

#[test]
fn test_mulpd_with_large_displacement() {
    // Test MULPD with large positive memory displacement.
    let instr =
        Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm14), src: Operand::mem_disp(GPRegister64::Rsi, 256) };
    assert_eq!(instr.to_string(), "mulpd xmm14, QWORD PTR [rsi + 256]", "MULPD with large displacement");
}

// ============================================================================
// SSE Packed Multiplication - Comprehensive Edge Cases
// ============================================================================

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_mnemonic_consistency() {
    // Verify mnemonic consistency for SSE packed multiplication instructions.
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(mulps.mnemonic(), "mulps");
    assert_eq!(mulps.to_string(), "mulps xmm0, xmm1");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_eq!(mulpd.mnemonic(), "mulpd");
    assert_eq!(mulpd.to_string(), "mulpd xmm2, xmm3");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_clone_and_equality() {
    // Test Clone and PartialEq for SSE packed multiplication instructions.
    let mulps1 = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm10), src: Operand::xmm(XMMRegister::Xmm11) };
    let mulps2 = mulps1.clone();
    assert_eq!(mulps1, mulps2, "Cloned MULPS should equal original");

    let mulpd1 = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm12), src: Operand::xmm(XMMRegister::Xmm13) };
    let mulpd2 = mulpd1.clone();
    assert_eq!(mulpd1, mulpd2, "Cloned MULPD should equal original");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_not_control_flow() {
    // Verify SSE packed multiplication instructions are not control flow instructions.
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert!(!mulps.is_jump(), "MULPS should not be a jump");
    assert!(!mulps.is_call(), "MULPS should not be a call");
    assert!(!mulps.is_return(), "MULPS should not be a return");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert!(!mulpd.is_jump(), "MULPD should not be a jump");
    assert!(!mulpd.is_call(), "MULPD should not be a call");
    assert!(!mulpd.is_return(), "MULPD should not be a return");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_debug_formatting() {
    // Test Debug trait implementation for SSE packed multiplication instructions.
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) };
    let debug_str = format!("{mulps:?}");
    assert!(debug_str.contains("Mulps"), "Debug output should contain 'Mulps'");
    assert!(debug_str.contains("Xmm4"), "Debug output should contain dest register");
    assert!(debug_str.contains("Xmm5"), "Debug output should contain src register");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm6), src: Operand::xmm(XMMRegister::Xmm7) };
    let debug_str = format!("{mulpd:?}");
    assert!(debug_str.contains("Mulpd"), "Debug output should contain 'Mulpd'");
}

#[test]
fn test_sse_packed_mul_in_collections() {
    // Test that SSE packed multiplication instructions work in collections.
    let instructions = [
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) },
        Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) },
    ];

    assert_eq!(instructions.len(), 2);
    assert_eq!(instructions[0].mnemonic(), "mulps");
    assert_eq!(instructions[1].mnemonic(), "mulpd");

    // Test searching
    let mulps_search =
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert!(instructions.contains(&mulps_search));
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_pattern_matching() {
    // Test pattern matching for SSE packed multiplication instructions.
    let classify = |instr: Instruction| -> &str {
        match instr {
            Instruction::Mulps { .. } => "packed_single",
            Instruction::Mulpd { .. } => "packed_double",
            _ => "other",
        }
    };

    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm8), src: Operand::xmm(XMMRegister::Xmm9) };
    assert_eq!(classify(mulps), "packed_single");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm10), src: Operand::xmm(XMMRegister::Xmm11) };
    assert_eq!(classify(mulpd), "packed_double");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_all_xmm_registers() {
    // Test MULPS and MULPD with all available XMM registers.
    // This ensures proper formatting across the entire register range.
    let xmm_regs = vec![
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

    for (i, &dest_reg) in xmm_regs.iter().enumerate() {
        let src_reg = xmm_regs[(i + 1) % xmm_regs.len()];

        // Test MULPS
        let mulps = Instruction::Mulps { dest: Operand::xmm(dest_reg), src: Operand::xmm(src_reg) };
        assert_eq!(mulps.mnemonic(), "mulps");
        assert!(mulps.to_string().starts_with("mulps"));

        // Test MULPD
        let mulpd = Instruction::Mulpd { dest: Operand::xmm(dest_reg), src: Operand::xmm(src_reg) };
        assert_eq!(mulpd.mnemonic(), "mulpd");
        assert!(mulpd.to_string().starts_with("mulpd"));
    }
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_semantic_correctness() {
    // Verify semantic properties of SSE packed multiplication instructions.
    // MULPS operates on 4 single-precision (32-bit) floating-point values in parallel.
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(mulps.mnemonic(), "mulps");
    assert!(mulps.to_string().contains("mulps"), "MULPS should format with 'mulps' mnemonic");

    // MULPD operates on 2 double-precision (64-bit) floating-point values in parallel.
    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm1), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(mulpd.mnemonic(), "mulpd");
    assert!(mulpd.to_string().contains("mulpd"), "MULPD should format with 'mulpd' mnemonic");

    // Verify they are distinct instructions
    assert_ne!(mulps, mulpd, "MULPS and MULPD should be different instructions");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_operand_ordering() {
    // Verify that operand ordering is correct: dest, src (Intel syntax).
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    let display = mulps.to_string();
    // Intel syntax: dest comes first, then source
    assert!(display.starts_with("mulps xmm2,"), "Destination should come first");
    assert!(display.ends_with("xmm3"), "Source should come last");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) };
    let display = mulpd.to_string();
    assert!(display.starts_with("mulpd xmm4,"), "Destination should come first");
    assert!(display.ends_with("xmm5"), "Source should come last");
}

#[test]
fn test_sse_packed_mul_with_all_memory_forms() {
    // Test MULPS and MULPD with various memory addressing modes.

    // Direct memory reference
    let mulps_mem = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::mem(GPRegister64::Rdi) };
    assert!(mulps_mem.to_string().contains("[rdi]"));

    // Memory with negative displacement
    let mulps_neg =
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm1), src: Operand::mem_disp(GPRegister64::Rbp, -8) };
    assert!(mulps_neg.to_string().contains("rbp - 8"));

    // Memory with large positive displacement
    let mulpd_pos =
        Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::mem_disp(GPRegister64::Rsp, 128) };
    assert!(mulpd_pos.to_string().contains("rsp + 128"));

    // Memory with zero displacement (should omit displacement)
    let mulpd_zero = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::mem(GPRegister64::Rcx) };
    assert!(mulpd_zero.to_string().contains("[rcx]"));
    assert!(!mulpd_zero.to_string().contains("+ 0"));
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_lowercase_formatting() {
    // Verify that mnemonics are lowercase (x86 convention).
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    let display = mulps.to_string();
    assert_eq!(display.split_whitespace().next().unwrap(), "mulps", "Mnemonic should be lowercase");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    let display = mulpd.to_string();
    assert_eq!(display.split_whitespace().next().unwrap(), "mulpd", "Mnemonic should be lowercase");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_simd_parallelism() {
    // Test that MULPS and MULPD instructions are properly represented
    // for SIMD parallel multiplication operations.
    // MULPS: 4 parallel 32-bit float multiplications
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(mulps.mnemonic(), "mulps", "MULPS for 4x32-bit parallel ops");

    // MULPD: 2 parallel 64-bit double multiplications
    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_eq!(mulpd.mnemonic(), "mulpd", "MULPD for 2x64-bit parallel ops");
}

#[test]
#[allow(clippy::similar_names)]
fn test_sse_packed_mul_vs_scalar() {
    // Verify distinction between packed (PS/PD) and scalar (SS/SD) multiplication.
    let mulps = Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    let mulss = Instruction::Mulss { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_ne!(mulps, mulss, "MULPS and MULSS are different instructions");
    assert_eq!(mulps.mnemonic(), "mulps");
    assert_eq!(mulss.mnemonic(), "mulss");

    let mulpd = Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    let mulsd = Instruction::Mulsd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_ne!(mulpd, mulsd, "MULPD and MULSD are different instructions");
    assert_eq!(mulpd.mnemonic(), "mulpd");
    assert_eq!(mulsd.mnemonic(), "mulsd");
}

#[test]
fn test_sse_packed_mul_boundary_registers() {
    // Test with boundary XMM registers (first and last).
    let mulps_first =
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm15) };
    assert_eq!(mulps_first.to_string(), "mulps xmm0, xmm15");

    let mulpd_last =
        Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm15), src: Operand::xmm(XMMRegister::Xmm0) };
    assert_eq!(mulpd_last.to_string(), "mulpd xmm15, xmm0");
}
