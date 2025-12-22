//! Comprehensive test suite for the `Operand` type in the `asm::instruction` module.
//!
//! This module tests all functionality related to assembly operands including
//! register operands, immediate operands, memory operands, and label operands.
//! Constructor methods, type checking predicates, From trait implementations,
//! and Display formatting are thoroughly covered.

use jsavrs::asm::{
    GPRegister8, GPRegister16, GPRegister32, GPRegister64, Immediate, MemoryOperand, Operand, X86Register, XMMRegister,
    YMMRegister,
};

#[test]
fn test_reg64_rax() {
    let op = Operand::reg64(GPRegister64::Rax);
    assert!(matches!(op, Operand::Register(X86Register::GP64(GPRegister64::Rax))));
}

#[test]
fn test_reg64_all_registers() {
    let registers = [
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

    for reg in registers {
        let op = Operand::reg64(reg);
        if let Operand::Register(X86Register::GP64(r)) = op {
            assert_eq!(r, reg);
        } else {
            panic!("Expected GP64 register operand");
        }
    }
}

#[test]
fn test_reg32_eax() {
    let op = Operand::reg32(GPRegister32::Eax);
    assert!(matches!(op, Operand::Register(X86Register::GP32(GPRegister32::Eax))));
}

#[test]
fn test_reg32_all_registers() {
    let registers = [
        GPRegister32::Eax,
        GPRegister32::Ebx,
        GPRegister32::Ecx,
        GPRegister32::Edx,
        GPRegister32::Esi,
        GPRegister32::Edi,
        GPRegister32::Ebp,
        GPRegister32::Esp,
        GPRegister32::R8d,
        GPRegister32::R9d,
        GPRegister32::R10d,
        GPRegister32::R11d,
        GPRegister32::R12d,
        GPRegister32::R13d,
        GPRegister32::R14d,
        GPRegister32::R15d,
    ];

    for reg in registers {
        let op = Operand::reg32(reg);
        if let Operand::Register(X86Register::GP32(r)) = op {
            assert_eq!(r, reg);
        } else {
            panic!("Expected GP32 register operand");
        }
    }
}

#[test]
fn test_reg16_ax() {
    let op = Operand::reg16(GPRegister16::Ax);
    assert!(matches!(op, Operand::Register(X86Register::GP16(GPRegister16::Ax))));
}

#[test]
fn test_reg16_various() {
    let registers = [
        GPRegister16::Ax,
        GPRegister16::Bx,
        GPRegister16::Cx,
        GPRegister16::Dx,
        GPRegister16::Si,
        GPRegister16::Di,
        GPRegister16::Bp,
        GPRegister16::Sp,
    ];

    for reg in registers {
        let op = Operand::reg16(reg);
        if let Operand::Register(X86Register::GP16(r)) = op {
            assert_eq!(r, reg);
        } else {
            panic!("Expected GP16 register operand");
        }
    }
}

#[test]
fn test_reg8_al() {
    let op = Operand::reg8(GPRegister8::Al);
    assert!(matches!(op, Operand::Register(X86Register::GP8(GPRegister8::Al))));
}

#[test]
fn test_reg8_various() {
    let registers = [
        GPRegister8::Al,
        GPRegister8::Bl,
        GPRegister8::Cl,
        GPRegister8::Dl,
        GPRegister8::Ah,
        GPRegister8::Bh,
        GPRegister8::Ch,
        GPRegister8::Dh,
        GPRegister8::Sil,
        GPRegister8::Dil,
    ];

    for reg in registers {
        let op = Operand::reg8(reg);
        if let Operand::Register(X86Register::GP8(r)) = op {
            assert_eq!(r, reg);
        } else {
            panic!("Expected GP8 register operand");
        }
    }
}

#[test]
fn test_xmm0() {
    let op = Operand::xmm(XMMRegister::Xmm0);
    assert!(matches!(op, Operand::Register(X86Register::Xmm(XMMRegister::Xmm0))));
}

#[test]
fn test_xmm_all_registers() {
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

    for reg in registers {
        let op = Operand::xmm(reg);
        if let Operand::Register(X86Register::Xmm(r)) = op {
            assert_eq!(r, reg);
        } else {
            panic!("Expected XMM register operand");
        }
    }
}

#[test]
fn test_ymm0() {
    let op = Operand::ymm(YMMRegister::Ymm0);
    assert!(matches!(op, Operand::Register(X86Register::Ymm(YMMRegister::Ymm0))));
}

#[test]
fn test_imm8_positive() {
    let op = Operand::imm8(42);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(42))));
}

#[test]
fn test_imm8_negative() {
    let op = Operand::imm8(-42);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(-42))));
}

#[test]
fn test_imm8_max() {
    let op = Operand::imm8(i8::MAX);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(127))));
}

#[test]
fn test_imm8_min() {
    let op = Operand::imm8(i8::MIN);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(-128))));
}

#[test]
fn test_imm16_positive() {
    let op = Operand::imm16(1000);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16(1000))));
}

#[test]
fn test_imm16_negative() {
    let op = Operand::imm16(-1000);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16(-1000))));
}

#[test]
fn test_imm16_max() {
    let op = Operand::imm16(i16::MAX);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16(32767))));
}

#[test]
fn test_imm16_min() {
    let op = Operand::imm16(i16::MIN);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16(-32768))));
}

#[test]
fn test_imm32_positive() {
    let op = Operand::imm32(100_000);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm32(100_000))));
}

#[test]
fn test_imm32_negative() {
    let op = Operand::imm32(-100_000);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm32(-100_000))));
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_imm32_max() {
    let op = Operand::imm32(i32::MAX);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm32(2147483647))));
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_imm32_min() {
    let op = Operand::imm32(i32::MIN);
    assert!(matches!(op, Operand::Immediate(Immediate::Imm32(-2147483648))));
}

#[test]
fn test_imm64_positive() {
    let op = Operand::imm64(10_000_000_000);
    if let Operand::Immediate(Immediate::Imm64(v)) = op {
        assert_eq!(v, 10_000_000_000);
    } else {
        panic!("Expected Imm64");
    }
}

#[test]
fn test_imm64_negative() {
    let op = Operand::imm64(-10_000_000_000);
    if let Operand::Immediate(Immediate::Imm64(v)) = op {
        assert_eq!(v, -10_000_000_000);
    } else {
        panic!("Expected Imm64");
    }
}

#[test]
fn test_imm64_max() {
    let op = Operand::imm64(i64::MAX);
    if let Operand::Immediate(Immediate::Imm64(v)) = op {
        assert_eq!(v, i64::MAX);
    } else {
        panic!("Expected Imm64");
    }
}

#[test]
fn test_imm64_min() {
    let op = Operand::imm64(i64::MIN);
    if let Operand::Immediate(Immediate::Imm64(v)) = op {
        assert_eq!(v, i64::MIN);
    } else {
        panic!("Expected Imm64");
    }
}

#[test]
fn test_mem_basic() {
    let op = Operand::mem(GPRegister64::Rax);

    if let Operand::Memory(mem) = op {
        assert_eq!(mem.base, Some(GPRegister64::Rax));
        assert_eq!(mem.index, None);
        assert_eq!(mem.displacement, 0);
    } else {
        panic!("Expected Memory operand");
    }
}

#[test]
fn test_mem_various_bases() {
    let registers = [
        GPRegister64::Rax,
        GPRegister64::Rbx,
        GPRegister64::Rcx,
        GPRegister64::Rdx,
        GPRegister64::Rsp,
        GPRegister64::Rbp,
        GPRegister64::Rsi,
        GPRegister64::Rdi,
    ];

    for reg in registers {
        let op = Operand::mem(reg);
        if let Operand::Memory(mem) = op {
            assert_eq!(mem.base, Some(reg));
        } else {
            panic!("Expected Memory operand");
        }
    }
}

#[test]
fn test_mem_disp_positive() {
    let op = Operand::mem_disp(GPRegister64::Rbp, 16);

    if let Operand::Memory(mem) = op {
        assert_eq!(mem.base, Some(GPRegister64::Rbp));
        assert_eq!(mem.displacement, 16);
    } else {
        panic!("Expected Memory operand");
    }
}

#[test]
fn test_mem_disp_negative() {
    let op = Operand::mem_disp(GPRegister64::Rbp, -16);

    if let Operand::Memory(mem) = op {
        assert_eq!(mem.base, Some(GPRegister64::Rbp));
        assert_eq!(mem.displacement, -16);
    } else {
        panic!("Expected Memory operand");
    }
}

#[test]
fn test_mem_disp_zero() {
    let op = Operand::mem_disp(GPRegister64::Rsp, 0);

    if let Operand::Memory(mem) = op {
        assert_eq!(mem.base, Some(GPRegister64::Rsp));
        assert_eq!(mem.displacement, 0);
    } else {
        panic!("Expected Memory operand");
    }
}

#[test]
fn test_mem_disp_max() {
    let op = Operand::mem_disp(GPRegister64::R12, i32::MAX);

    if let Operand::Memory(mem) = op {
        assert_eq!(mem.displacement, i32::MAX);
    } else {
        panic!("Expected Memory operand");
    }
}

#[test]
fn test_mem_disp_min() {
    let op = Operand::mem_disp(GPRegister64::R13, i32::MIN);

    if let Operand::Memory(mem) = op {
        assert_eq!(mem.displacement, i32::MIN);
    } else {
        panic!("Expected Memory operand");
    }
}

#[test]
fn test_label_from_str() {
    let op = Operand::label("main");
    assert!(matches!(op, Operand::Label(s) if s == "main"));
}

#[test]
fn test_label_from_string() {
    let op = Operand::label(String::from("_start"));
    assert!(matches!(op, Operand::Label(s) if s == "_start"));
}

#[test]
fn test_label_empty() {
    let op = Operand::label("");
    assert!(matches!(op, Operand::Label(s) if s.is_empty()));
}

#[test]
fn test_label_with_underscore() {
    let op = Operand::label("__my_label__");
    assert!(matches!(op, Operand::Label(s) if s == "__my_label__"));
}

#[test]
fn test_label_with_numbers() {
    let op = Operand::label("label123");
    assert!(matches!(op, Operand::Label(s) if s == "label123"));
}

#[test]
fn test_label_with_dots() {
    let op = Operand::label(".L0");
    assert!(matches!(op, Operand::Label(s) if s == ".L0"));
}

#[test]
fn test_is_register_true_for_reg64() {
    let op = Operand::reg64(GPRegister64::Rax);
    assert!(op.is_register());
}

#[test]
fn test_is_register_true_for_reg32() {
    let op = Operand::reg32(GPRegister32::Eax);
    assert!(op.is_register());
}

#[test]
fn test_is_register_true_for_xmm() {
    let op = Operand::xmm(XMMRegister::Xmm0);
    assert!(op.is_register());
}

#[test]
fn test_is_register_false_for_immediate() {
    let op = Operand::imm32(42);
    assert!(!op.is_register());
}

#[test]
fn test_is_register_false_for_memory() {
    let op = Operand::mem(GPRegister64::Rax);
    assert!(!op.is_register());
}

#[test]
fn test_is_register_false_for_label() {
    let op = Operand::label("test");
    assert!(!op.is_register());
}

#[test]
fn test_is_immediate_true_for_imm8() {
    let op = Operand::imm8(10);
    assert!(op.is_immediate());
}

#[test]
fn test_is_immediate_true_for_imm16() {
    let op = Operand::imm16(1000);
    assert!(op.is_immediate());
}

#[test]
fn test_is_immediate_true_for_imm32() {
    let op = Operand::imm32(100_000);
    assert!(op.is_immediate());
}

#[test]
fn test_is_immediate_true_for_imm64() {
    let op = Operand::imm64(10_000_000_000);
    assert!(op.is_immediate());
}

#[test]
fn test_is_immediate_false_for_register() {
    let op = Operand::reg64(GPRegister64::Rax);
    assert!(!op.is_immediate());
}

#[test]
fn test_is_immediate_false_for_memory() {
    let op = Operand::mem(GPRegister64::Rbx);
    assert!(!op.is_immediate());
}

#[test]
fn test_is_immediate_false_for_label() {
    let op = Operand::label("func");
    assert!(!op.is_immediate());
}

#[test]
fn test_is_memory_true_for_mem() {
    let op = Operand::mem(GPRegister64::Rax);
    assert!(op.is_memory());
}

#[test]
fn test_is_memory_true_for_mem_disp() {
    let op = Operand::mem_disp(GPRegister64::Rbp, -8);
    assert!(op.is_memory());
}

#[test]
fn test_is_memory_false_for_register() {
    let op = Operand::reg64(GPRegister64::Rcx);
    assert!(!op.is_memory());
}

#[test]
fn test_is_memory_false_for_immediate() {
    let op = Operand::imm32(0);
    assert!(!op.is_memory());
}

#[test]
fn test_is_memory_false_for_label() {
    let op = Operand::label("data");
    assert!(!op.is_memory());
}

#[test]
fn test_is_label_true() {
    let op = Operand::label("loop_start");
    assert!(op.is_label());
}

#[test]
fn test_is_label_true_empty() {
    let op = Operand::label("");
    assert!(op.is_label());
}

#[test]
fn test_is_label_false_for_register() {
    let op = Operand::reg32(GPRegister32::Edx);
    assert!(!op.is_label());
}

#[test]
fn test_is_label_false_for_immediate() {
    let op = Operand::imm16(256);
    assert!(!op.is_label());
}

#[test]
fn test_is_label_false_for_memory() {
    let op = Operand::mem(GPRegister64::Rdi);
    assert!(!op.is_label());
}

#[test]
fn test_from_i8_positive() {
    let op: Operand = 42_i8.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(42))));
}

#[test]
fn test_from_i8_negative() {
    let op: Operand = (-42_i8).into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(-42))));
}

#[test]
fn test_from_i8_zero() {
    let op: Operand = 0_i8.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8(0))));
}

#[test]
fn test_from_u8() {
    let op: Operand = 200_u8.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8u(200))));
}

#[test]
fn test_from_u8_max() {
    let op: Operand = u8::MAX.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm8u(255))));
}

#[test]
fn test_from_i16_positive() {
    let op: Operand = 1000_i16.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16(1000))));
}

#[test]
fn test_from_i16_negative() {
    let op: Operand = (-1000_i16).into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16(-1000))));
}

#[test]
fn test_from_u16() {
    let op: Operand = 50000_u16.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16u(50000))));
}

#[test]
fn test_from_u16_max() {
    let op: Operand = u16::MAX.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm16u(65535))));
}

#[test]
fn test_from_i32_positive() {
    let op: Operand = 100_000_i32.into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm32(100_000))));
}

#[test]
fn test_from_i32_negative() {
    let op: Operand = (-100_000_i32).into();
    assert!(matches!(op, Operand::Immediate(Immediate::Imm32(-100_000))));
}

#[test]
fn test_from_u32() {
    let op: Operand = 3_000_000_000_u32.into();
    if let Operand::Immediate(Immediate::Imm32u(v)) = op {
        assert_eq!(v, 3_000_000_000);
    } else {
        panic!("Expected Imm32u");
    }
}

#[test]
fn test_from_i64() {
    let op: Operand = 10_000_000_000_i64.into();
    if let Operand::Immediate(Immediate::Imm64(v)) = op {
        assert_eq!(v, 10_000_000_000);
    } else {
        panic!("Expected Imm64");
    }
}

#[test]
fn test_from_u64() {
    let op: Operand = 10_000_000_000_u64.into();
    if let Operand::Immediate(Immediate::Imm64u(v)) = op {
        assert_eq!(v, 10_000_000_000);
    } else {
        panic!("Expected Imm64u");
    }
}

#[test]
fn test_from_u64_max() {
    let op: Operand = u64::MAX.into();
    if let Operand::Immediate(Immediate::Imm64u(v)) = op {
        assert_eq!(v, u64::MAX);
    } else {
        panic!("Expected Imm64u");
    }
}

#[test]
fn test_from_x86register_gp64() {
    let reg = X86Register::GP64(GPRegister64::Rax);
    let op: Operand = reg.into();
    assert!(matches!(op, Operand::Register(X86Register::GP64(GPRegister64::Rax))));
}

#[test]
fn test_from_x86register_gp32() {
    let reg = X86Register::GP32(GPRegister32::Eax);
    let op: Operand = reg.into();
    assert!(matches!(op, Operand::Register(X86Register::GP32(GPRegister32::Eax))));
}

#[test]
fn test_from_x86register_xmm() {
    let reg = X86Register::Xmm(XMMRegister::Xmm5);
    let op: Operand = reg.into();
    assert!(matches!(op, Operand::Register(X86Register::Xmm(XMMRegister::Xmm5))));
}

#[test]
fn test_display_register_gp64() {
    let op = Operand::reg64(GPRegister64::Rax);
    let output = format!("{op}");
    assert_eq!(output, "rax");
}

#[test]
fn test_display_register_gp32() {
    let op = Operand::reg32(GPRegister32::Ebx);
    let output = format!("{op}");
    assert_eq!(output, "ebx");
}

#[test]
fn test_display_register_gp16() {
    let op = Operand::reg16(GPRegister16::Cx);
    let output = format!("{op}");
    assert_eq!(output, "cx");
}

#[test]
fn test_display_register_gp8() {
    let op = Operand::reg8(GPRegister8::Dl);
    let output = format!("{op}");
    assert_eq!(output, "dl");
}

#[test]
fn test_display_register_xmm() {
    let op = Operand::xmm(XMMRegister::Xmm0);
    let output = format!("{op}");
    assert_eq!(output, "xmm0");
}

#[test]
fn test_display_immediate_signed() {
    let op = Operand::imm32(42);
    let output = format!("{op}");
    assert_eq!(output, "42");
}

#[test]
fn test_display_immediate_negative() {
    let op = Operand::imm32(-100);
    let output = format!("{op}");
    assert_eq!(output, "-100");
}

#[test]
fn test_display_memory_simple() {
    let op = Operand::Memory(MemoryOperand::new(Some(GPRegister64::Rax)).with_size(8));
    let output = format!("{op}");
    assert_eq!(output, "QWORD PTR [rax]");
}

#[test]
fn test_display_memory_with_displacement() {
    let op = Operand::Memory(MemoryOperand::new(Some(GPRegister64::Rbp)).with_displacement(-16).with_size(4));
    let output = format!("{op}");
    assert_eq!(output, "DWORD PTR [rbp - 16]");
}

#[test]
fn test_display_label() {
    let op = Operand::label("my_function");
    let output = format!("{op}");
    assert_eq!(output, "my_function");
}

#[test]
fn test_display_label_empty() {
    let op = Operand::label("");
    let output = format!("{op}");
    assert_eq!(output, "");
}

#[test]
fn test_operand_clone_register() {
    let op1 = Operand::reg64(GPRegister64::Rax);
    let op2 = op1.clone();
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_clone_immediate() {
    let op1 = Operand::imm32(12345);
    let op2 = op1.clone();
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_clone_memory() {
    let op1 = Operand::mem_disp(GPRegister64::Rbp, -8);
    let op2 = op1.clone();
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_clone_label() {
    let op1 = Operand::label("test_label");
    let op2 = op1.clone();
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_eq_same_register() {
    let op1 = Operand::reg32(GPRegister32::Ecx);
    let op2 = Operand::reg32(GPRegister32::Ecx);
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_neq_different_registers() {
    let op1 = Operand::reg32(GPRegister32::Ecx);
    let op2 = Operand::reg32(GPRegister32::Edx);
    assert_ne!(op1, op2);
}

#[test]
fn test_operand_neq_different_types() {
    let op1 = Operand::reg64(GPRegister64::Rax);
    let op2 = Operand::imm64(0);
    assert_ne!(op1, op2);
}

#[test]
fn test_operand_neq_register_vs_memory() {
    let op1 = Operand::reg64(GPRegister64::Rax);
    let op2 = Operand::mem(GPRegister64::Rax);
    assert_ne!(op1, op2);
}

#[test]
fn test_operand_eq_same_immediate_value() {
    let op1 = Operand::imm16(-500);
    let op2 = Operand::imm16(-500);
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_neq_different_immediate_size() {
    let op1 = Operand::imm8(10);
    let op2 = Operand::imm16(10);
    assert_ne!(op1, op2);
}

#[test]
fn test_operand_eq_same_label() {
    let op1 = Operand::label("same");
    let op2 = Operand::label("same");
    assert_eq!(op1, op2);
}

#[test]
fn test_operand_neq_different_labels() {
    let op1 = Operand::label("label1");
    let op2 = Operand::label("label2");
    assert_ne!(op1, op2);
}

#[test]
fn test_debug_register() {
    let op = Operand::reg64(GPRegister64::Rax);
    let debug_str = format!("{op:?}");
    assert!(debug_str.contains("Register"));
    assert!(debug_str.contains("GP64"));
    assert!(debug_str.contains("Rax"));
}

#[test]
fn test_debug_immediate() {
    let op = Operand::imm32(42);
    let debug_str = format!("{op:?}");
    assert!(debug_str.contains("Immediate"));
    assert!(debug_str.contains("Imm32"));
    assert!(debug_str.contains("42"));
}

#[test]
fn test_debug_memory() {
    let op = Operand::mem(GPRegister64::Rbx);
    let debug_str = format!("{op:?}");
    assert!(debug_str.contains("Memory"));
    assert!(debug_str.contains("MemoryOperand"));
}

#[test]
fn test_debug_label() {
    let op = Operand::label("my_label");
    let debug_str = format!("{op:?}");
    assert!(debug_str.contains("Label"));
    assert!(debug_str.contains("my_label"));
}

#[test]
fn test_zero_immediate_all_sizes() {
    assert!(Operand::imm8(0).is_immediate());
    assert!(Operand::imm16(0).is_immediate());
    assert!(Operand::imm32(0).is_immediate());
    assert!(Operand::imm64(0).is_immediate());
}

#[test]
fn test_max_values_all_immediate_sizes() {
    let _ = Operand::imm8(i8::MAX);
    let _ = Operand::imm16(i16::MAX);
    let _ = Operand::imm32(i32::MAX);
    let _ = Operand::imm64(i64::MAX);
}

#[test]
fn test_min_values_all_immediate_sizes() {
    let _ = Operand::imm8(i8::MIN);
    let _ = Operand::imm16(i16::MIN);
    let _ = Operand::imm32(i32::MIN);
    let _ = Operand::imm64(i64::MIN);
}

#[test]
fn test_label_with_special_characters() {
    let op = Operand::label("__Z5func1v");
    assert!(op.is_label());
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_very_long_label() {
    let long_label = "a".repeat(1000);
    let op = Operand::label(long_label.clone());
    if let Operand::Label(s) = op {
        assert_eq!(s.len(), 1000);
    } else {
        panic!("Expected Label");
    }
}

#[test]
fn test_unicode_label() {
    // Labels with unicode (though unusual in assembly)
    let op = Operand::label("función_αβγ");
    if let Operand::Label(s) = op {
        assert_eq!(s, "función_αβγ");
    } else {
        panic!("Expected Label");
    }
}

#[test]
fn test_mov_instruction_operands() {
    // mov rax, rbx
    let dest = Operand::reg64(GPRegister64::Rax);
    let src = Operand::reg64(GPRegister64::Rbx);

    assert!(dest.is_register());
    assert!(src.is_register());
}

#[test]
fn test_mov_immediate_operands() {
    // mov eax, 42
    let dest = Operand::reg32(GPRegister32::Eax);
    let src = Operand::imm32(42);

    assert!(dest.is_register());
    assert!(src.is_immediate());
}

#[test]
fn test_load_from_memory_operands() {
    // mov rax, [rbp - 8]
    let dest = Operand::reg64(GPRegister64::Rax);
    let src = Operand::mem_disp(GPRegister64::Rbp, -8);

    assert!(dest.is_register());
    assert!(src.is_memory());
}

#[test]
fn test_call_operand() {
    // call printf
    let target = Operand::label("printf");
    assert!(target.is_label());
}

#[test]
fn test_jmp_operand() {
    // jmp .L0
    let target = Operand::label(".L0");
    assert!(target.is_label());
}

#[test]
fn test_simd_register_operands() {
    // addps xmm0, xmm1
    let dest = Operand::xmm(XMMRegister::Xmm0);
    let src = Operand::xmm(XMMRegister::Xmm1);

    assert!(dest.is_register());
    assert!(src.is_register());
}

#[test]
fn test_push_immediate() {
    // push 0x10
    let op = Operand::imm8(0x10);
    assert!(op.is_immediate());
}

#[test]
fn test_byte_memory_access() {
    // mov al, [rsi]
    let dest = Operand::reg8(GPRegister8::Al);
    let src = Operand::mem(GPRegister64::Rsi);

    assert!(dest.is_register());
    assert!(src.is_memory());
}
