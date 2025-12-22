// Extended comprehensive tests for x86_register.rs
// This file focuses on edge cases, corner cases, and exhaustive coverage

use jsavrs::asm::{
    CALLEE_SAVED_GP_SYSTEMV, CALLEE_SAVED_GP_WINDOWS, CALLEE_SAVED_XMM_WINDOWS, CALLER_SAVED_GP_SYSTEMV,
    CALLER_SAVED_GP_WINDOWS, CALLER_SAVED_XMM_SYSTEMV, CALLER_SAVED_XMM_WINDOWS, ControlRegister, DebugRegister,
    FLOAT_PARAM_REGS_SYSTEMV, FLOAT_PARAM_REGS_WINDOWS, FLOAT_RETURN_REGS_SYSTEMV, FPURegister, FlagsRegister,
    GPRegister8, GPRegister16, GPRegister32, GPRegister64, INT_PARAM_REGS_SYSTEMV, INT_PARAM_REGS_WINDOWS,
    INT_RETURN_REGS, InstructionPointer, MMXRegister, MaskRegister, Platform, SegmentRegister, X86Register,
    XMMRegister, YMMRegister, ZMMRegister,
};

// ============================================================================
// Clone, Copy, and Debug Trait Tests
// ============================================================================

#[test]
#[allow(clippy::clone_on_copy)]
fn test_x86_register_clone_and_copy() {
    // Test that X86Register implements Clone and Copy traits properly
    let original = X86Register::GP64(GPRegister64::Rax);
    let cloned = original.clone();
    let copied = original;

    assert_eq!(original, cloned);
    assert_eq!(original, copied);
    assert_eq!(cloned, copied);

    // Test with various register types
    let regs = vec![
        X86Register::GP32(GPRegister32::Ebx),
        X86Register::GP16(GPRegister16::Cx),
        X86Register::GP8(GPRegister8::Dl),
        X86Register::Fpu(FPURegister::St3),
        X86Register::Mmx(MMXRegister::Mm5),
        X86Register::Xmm(XMMRegister::Xmm7),
        X86Register::Ymm(YMMRegister::Ymm9),
        X86Register::Zmm(ZMMRegister::Zmm11),
    ];

    for reg in regs {
        let cloned = reg.clone();
        let copied = reg;
        assert_eq!(reg, cloned);
        assert_eq!(reg, copied);
    }
}

#[test]
fn test_x86_register_debug_format() {
    // Test Debug trait formatting
    let reg = X86Register::GP64(GPRegister64::Rax);
    let debug_str = format!("{reg:?}");
    assert!(debug_str.contains("GP64"));
    assert!(debug_str.contains("Rax"));

    let xmm_reg = X86Register::Xmm(XMMRegister::Xmm15);
    let xmm_debug = format!("{xmm_reg:?}");
    assert!(xmm_debug.contains("Xmm"));
    assert!(xmm_debug.contains("Xmm15"));
}

// ============================================================================
// Parameter Register Edge Cases
// ============================================================================

#[test]
fn test_is_parameter_register_out_of_bounds() {
    let platform_win = Platform::Windows;
    let platform_linux = Platform::Linux;

    // Test Windows: only 4 integer parameters (0-3)
    let rcx = X86Register::GP64(GPRegister64::Rcx);
    assert!(rcx.is_parameter_register(platform_win, 0));
    assert!(!rcx.is_parameter_register(platform_win, 1)); // RCX is not 2nd param
    assert!(!rcx.is_parameter_register(platform_win, 4)); // Out of bounds
    assert!(!rcx.is_parameter_register(platform_win, 100)); // Far out of bounds

    // Test Linux: 6 integer parameters (0-5)
    let rdi = X86Register::GP64(GPRegister64::Rdi);
    assert!(rdi.is_parameter_register(platform_linux, 0));
    assert!(!rdi.is_parameter_register(platform_linux, 6)); // Out of bounds
    assert!(!rdi.is_parameter_register(platform_linux, 1000)); // Far out of bounds

    // Test XMM parameter bounds
    // Windows: 4 float params (0-3)
    let xmm0 = X86Register::Xmm(XMMRegister::Xmm0);
    assert!(xmm0.is_parameter_register(platform_win, 0));
    assert!(!xmm0.is_parameter_register(platform_win, 4)); // Out of bounds

    // Linux: 8 float params (0-7)
    assert!(xmm0.is_parameter_register(platform_linux, 0));
    assert!(!xmm0.is_parameter_register(platform_linux, 8)); // Out of bounds
    assert!(!xmm0.is_parameter_register(platform_linux, 9)); // Out of bounds
}

#[test]
fn test_non_parameter_registers() {
    // Test that non-parameter GP registers return false for all indices
    let non_param_regs = vec![
        GPRegister64::Rax, // Return register, not parameter
        GPRegister64::Rbx,
        GPRegister64::Rbp,
        GPRegister64::Rsp,
        GPRegister64::R10,
        GPRegister64::R11,
        GPRegister64::R12,
        GPRegister64::R13,
        GPRegister64::R14,
        GPRegister64::R15,
    ];

    for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
        for reg in &non_param_regs {
            let x86_reg = X86Register::GP64(*reg);
            for i in 0..10 {
                assert!(
                    !x86_reg.is_parameter_register(platform, i),
                    "{reg:?} should not be parameter {i} on {platform:?}"
                );
            }
        }
    }
}

#[test]
fn test_non_parameter_xmm_registers() {
    // Test XMM registers that are never parameter registers
    let xmm8 = X86Register::Xmm(XMMRegister::Xmm8);
    let xmm15 = X86Register::Xmm(XMMRegister::Xmm15);

    for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
        for i in 0..20 {
            assert!(!xmm8.is_parameter_register(platform, i));
            assert!(!xmm15.is_parameter_register(platform, i));
        }
    }
}

#[test]
fn test_non_gp_non_xmm_parameter_registers() {
    // Registers that should never be parameter registers
    let test_regs = vec![
        X86Register::Fpu(FPURegister::St0),
        X86Register::Mmx(MMXRegister::Mm0),
        X86Register::Ymm(YMMRegister::Ymm0),
        X86Register::Zmm(ZMMRegister::Zmm0),
        X86Register::Mask(MaskRegister::K0),
        X86Register::Segment(SegmentRegister::Ds),
        X86Register::Control(ControlRegister::Cr0),
        X86Register::Debug(DebugRegister::Dr0),
        X86Register::Flags(FlagsRegister::Rflags),
        X86Register::InstructionPointer(InstructionPointer::Rip),
    ];

    for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
        for reg in &test_regs {
            for i in 0..10 {
                assert!(!reg.is_parameter_register(platform, i), "{reg:?} should never be parameter register");
            }
        }
    }
}

// ============================================================================
// Exhaustive GP8 Register Tests
// ============================================================================

#[test]
fn test_all_gp8_registers_size() {
    let gp8_regs = vec![
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
        GPRegister8::Bpl,
        GPRegister8::Spl,
        GPRegister8::R8b,
        GPRegister8::R9b,
        GPRegister8::R10b,
        GPRegister8::R11b,
        GPRegister8::R12b,
        GPRegister8::R13b,
        GPRegister8::R14b,
        GPRegister8::R15b,
    ];

    for reg in gp8_regs {
        let x86_reg = X86Register::GP8(reg);
        assert_eq!(x86_reg.size_bits(), 8, "{reg:?} should be 8 bits");
        assert_eq!(x86_reg.size_bytes(), 1, "{reg:?} should be 1 byte");
        assert!(x86_reg.is_8bit());
        assert!(x86_reg.is_gp());
        assert!(!x86_reg.is_simd());
        assert!(!x86_reg.is_float());
        assert!(!x86_reg.is_special());
    }
}

#[test]
fn test_gp8_legacy_high_byte_registers() {
    // Test the legacy high-byte registers (AH, BH, CH, DH)
    // These are incompatible with REX prefix on x86-64
    let high_byte_regs =
        vec![(GPRegister8::Ah, "ah"), (GPRegister8::Bh, "bh"), (GPRegister8::Ch, "ch"), (GPRegister8::Dh, "dh")];

    for (reg, expected_name) in high_byte_regs {
        let x86_reg = X86Register::GP8(reg);
        assert_eq!(x86_reg.nasm_name(), expected_name);
        assert_eq!(x86_reg.size_bits(), 8);
        assert!(x86_reg.is_gp());
    }
}

#[test]
fn test_gp8_rex_required_registers() {
    // Test registers that require REX prefix (SIL, DIL, BPL, SPL)
    let rex_regs = vec![
        (GPRegister8::Sil, "sil"),
        (GPRegister8::Dil, "dil"),
        (GPRegister8::Bpl, "bpl"),
        (GPRegister8::Spl, "spl"),
    ];

    for (reg, expected_name) in rex_regs {
        let x86_reg = X86Register::GP8(reg);
        assert_eq!(x86_reg.nasm_name(), expected_name);
        assert_eq!(x86_reg.size_bits(), 8);
        assert!(x86_reg.is_gp());
    }
}

// ============================================================================
// Exhaustive YMM and ZMM Register Tests
// ============================================================================

#[test]
fn test_all_ymm_registers_complete() {
    let ymm_regs = vec![
        YMMRegister::Ymm0,
        YMMRegister::Ymm1,
        YMMRegister::Ymm2,
        YMMRegister::Ymm3,
        YMMRegister::Ymm4,
        YMMRegister::Ymm5,
        YMMRegister::Ymm6,
        YMMRegister::Ymm7,
        YMMRegister::Ymm8,
        YMMRegister::Ymm9,
        YMMRegister::Ymm10,
        YMMRegister::Ymm11,
        YMMRegister::Ymm12,
        YMMRegister::Ymm13,
        YMMRegister::Ymm14,
        YMMRegister::Ymm15,
    ];

    for (idx, reg) in ymm_regs.iter().enumerate() {
        let x86_reg = X86Register::Ymm(*reg);
        assert_eq!(x86_reg.size_bits(), 256, "YMM{idx} should be 256 bits");
        assert_eq!(x86_reg.size_bytes(), 32, "YMM{idx} should be 32 bytes");
        assert!(x86_reg.is_simd());
        assert!(x86_reg.is_float());
        assert!(!x86_reg.is_gp());
        assert!(!x86_reg.is_special());
        assert_eq!(x86_reg.nasm_name(), format!("ymm{idx}"));
    }
}

#[test]
fn test_all_zmm_registers_complete() {
    let zmm_regs = vec![
        ZMMRegister::Zmm0,
        ZMMRegister::Zmm1,
        ZMMRegister::Zmm2,
        ZMMRegister::Zmm3,
        ZMMRegister::Zmm4,
        ZMMRegister::Zmm5,
        ZMMRegister::Zmm6,
        ZMMRegister::Zmm7,
        ZMMRegister::Zmm8,
        ZMMRegister::Zmm9,
        ZMMRegister::Zmm10,
        ZMMRegister::Zmm11,
        ZMMRegister::Zmm12,
        ZMMRegister::Zmm13,
        ZMMRegister::Zmm14,
        ZMMRegister::Zmm15,
        ZMMRegister::Zmm16,
        ZMMRegister::Zmm17,
        ZMMRegister::Zmm18,
        ZMMRegister::Zmm19,
        ZMMRegister::Zmm20,
        ZMMRegister::Zmm21,
        ZMMRegister::Zmm22,
        ZMMRegister::Zmm23,
        ZMMRegister::Zmm24,
        ZMMRegister::Zmm25,
        ZMMRegister::Zmm26,
        ZMMRegister::Zmm27,
        ZMMRegister::Zmm28,
        ZMMRegister::Zmm29,
        ZMMRegister::Zmm30,
        ZMMRegister::Zmm31,
    ];

    for (idx, reg) in zmm_regs.iter().enumerate() {
        let x86_reg = X86Register::Zmm(*reg);
        assert_eq!(x86_reg.size_bits(), 512, "ZMM{idx} should be 512 bits");
        assert_eq!(x86_reg.size_bytes(), 64, "ZMM{idx} should be 64 bytes");
        assert!(x86_reg.is_simd());
        assert!(x86_reg.is_float());
        assert!(!x86_reg.is_gp());
        assert!(!x86_reg.is_special());
        assert_eq!(x86_reg.nasm_name(), format!("zmm{idx}"));
    }
}

#[test]
fn test_ymm_volatility_all_registers() {
    // Test volatility for all 16 YMM registers on Windows
    for i in 0..16 {
        let ymm = match i {
            0 => YMMRegister::Ymm0,
            1 => YMMRegister::Ymm1,
            2 => YMMRegister::Ymm2,
            3 => YMMRegister::Ymm3,
            4 => YMMRegister::Ymm4,
            5 => YMMRegister::Ymm5,
            6 => YMMRegister::Ymm6,
            7 => YMMRegister::Ymm7,
            8 => YMMRegister::Ymm8,
            9 => YMMRegister::Ymm9,
            10 => YMMRegister::Ymm10,
            11 => YMMRegister::Ymm11,
            12 => YMMRegister::Ymm12,
            13 => YMMRegister::Ymm13,
            14 => YMMRegister::Ymm14,
            15 => YMMRegister::Ymm15,
            _ => unreachable!(),
        };

        let x86_reg = X86Register::Ymm(ymm);

        // On Windows, YMM0-YMM5 are volatile
        if i < 6 {
            assert!(x86_reg.is_volatile(Platform::Windows), "YMM{i} should be volatile on Windows");
            assert!(!x86_reg.is_callee_saved(Platform::Windows), "YMM{i} should not be callee-saved on Windows");
        } else {
            assert!(!x86_reg.is_volatile(Platform::Windows), "YMM{i} should not be volatile on Windows");
        }

        // On Linux/MacOS, all YMM are volatile
        assert!(x86_reg.is_volatile(Platform::Linux), "YMM{i} should be volatile on Linux");
        assert!(x86_reg.is_volatile(Platform::MacOS), "YMM{i} should be volatile on MacOS");
    }
}

// ============================================================================
// Consistency Tests: is_volatile vs is_callee_saved
// ============================================================================

#[test]
fn test_volatile_callee_saved_consistency_gp64() {
    // For GP64 registers, a register cannot be both volatile and callee-saved
    // (except for RSP which is special)
    let all_gp64 = vec![
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

    for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
        for reg in &all_gp64 {
            let x86_reg = X86Register::GP64(*reg);
            let is_vol = x86_reg.is_volatile(platform);
            let is_callee = x86_reg.is_callee_saved(platform);

            // RSP is special - it's callee-saved but not considered volatile
            if *reg != GPRegister64::Rsp {
                // For normal registers, they should be either volatile OR callee-saved, not both
                assert!(!(is_vol && is_callee), "{reg:?} cannot be both volatile and callee-saved on {platform:?}");
            }
        }
    }
}

#[test]
fn test_volatile_callee_saved_consistency_xmm() {
    // For XMM registers, check consistency across platforms
    let all_xmm = vec![
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

    for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
        for reg in &all_xmm {
            let x86_reg = X86Register::Xmm(*reg);
            let is_vol = x86_reg.is_volatile(platform);
            let is_callee = x86_reg.is_callee_saved(platform);

            // Cannot be both volatile and callee-saved
            assert!(!(is_vol && is_callee), "{reg:?} cannot be both volatile and callee-saved on {platform:?}");
        }
    }
}

// ============================================================================
// Special Register Tests
// ============================================================================

#[test]
fn test_all_control_registers() {
    let control_regs = vec![
        (ControlRegister::Cr0, "cr0"),
        (ControlRegister::Cr2, "cr2"),
        (ControlRegister::Cr3, "cr3"),
        (ControlRegister::Cr4, "cr4"),
        (ControlRegister::Cr8, "cr8"),
    ];

    for (reg, name) in control_regs {
        let x86_reg = X86Register::Control(reg);
        assert_eq!(x86_reg.size_bits(), 64);
        assert_eq!(x86_reg.size_bytes(), 8);
        assert!(x86_reg.is_special());
        assert!(!x86_reg.is_gp());
        assert!(!x86_reg.is_simd());
        assert!(!x86_reg.is_float());
        assert_eq!(x86_reg.nasm_name(), name);

        // Control registers should never be volatile or callee-saved
        assert!(!x86_reg.is_volatile(Platform::Windows));
        assert!(!x86_reg.is_volatile(Platform::Linux));
        assert!(!x86_reg.is_callee_saved(Platform::Windows));
        assert!(!x86_reg.is_callee_saved(Platform::Linux));
    }
}

#[test]
fn test_all_debug_registers() {
    let debug_regs = vec![
        (DebugRegister::Dr0, "dr0"),
        (DebugRegister::Dr1, "dr1"),
        (DebugRegister::Dr2, "dr2"),
        (DebugRegister::Dr3, "dr3"),
        (DebugRegister::Dr6, "dr6"),
        (DebugRegister::Dr7, "dr7"),
    ];

    for (reg, name) in debug_regs {
        let x86_reg = X86Register::Debug(reg);
        assert_eq!(x86_reg.size_bits(), 64);
        assert_eq!(x86_reg.size_bytes(), 8);
        assert!(x86_reg.is_special());
        assert!(!x86_reg.is_gp());
        assert_eq!(x86_reg.nasm_name(), name);

        // Debug registers should never be volatile or callee-saved
        assert!(!x86_reg.is_volatile(Platform::Windows));
        assert!(!x86_reg.is_callee_saved(Platform::Windows));
    }
}

#[test]
fn test_all_segment_registers() {
    let segment_regs = vec![
        (SegmentRegister::Cs, "cs"),
        (SegmentRegister::Ds, "ds"),
        (SegmentRegister::Es, "es"),
        (SegmentRegister::Fs, "fs"),
        (SegmentRegister::Gs, "gs"),
        (SegmentRegister::Ss, "ss"),
    ];

    for (reg, name) in segment_regs {
        let x86_reg = X86Register::Segment(reg);
        assert_eq!(x86_reg.size_bits(), 16);
        assert_eq!(x86_reg.size_bytes(), 2);
        assert!(x86_reg.is_special());
        assert!(x86_reg.is_16bit());
        assert!(!x86_reg.is_gp());
        assert_eq!(x86_reg.nasm_name(), name);

        // Segment registers should never be volatile or callee-saved
        assert!(!x86_reg.is_volatile(Platform::Windows));
        assert!(!x86_reg.is_callee_saved(Platform::Windows));
    }
}

#[test]
fn test_all_mask_registers() {
    let mask_regs = vec![
        (MaskRegister::K0, "k0"),
        (MaskRegister::K1, "k1"),
        (MaskRegister::K2, "k2"),
        (MaskRegister::K3, "k3"),
        (MaskRegister::K4, "k4"),
        (MaskRegister::K5, "k5"),
        (MaskRegister::K6, "k6"),
        (MaskRegister::K7, "k7"),
    ];

    for (reg, name) in mask_regs {
        let x86_reg = X86Register::Mask(reg);
        assert_eq!(x86_reg.size_bits(), 64);
        assert_eq!(x86_reg.size_bytes(), 8);
        assert!(x86_reg.is_64bit());
        assert!(!x86_reg.is_gp());
        assert!(!x86_reg.is_simd());
        assert!(!x86_reg.is_special());
        assert_eq!(x86_reg.nasm_name(), name);
    }
}

#[test]
fn test_all_mmx_registers() {
    let mmx_regs = vec![
        (MMXRegister::Mm0, "mm0"),
        (MMXRegister::Mm1, "mm1"),
        (MMXRegister::Mm2, "mm2"),
        (MMXRegister::Mm3, "mm3"),
        (MMXRegister::Mm4, "mm4"),
        (MMXRegister::Mm5, "mm5"),
        (MMXRegister::Mm6, "mm6"),
        (MMXRegister::Mm7, "mm7"),
    ];

    for (reg, name) in mmx_regs {
        let x86_reg = X86Register::Mmx(reg);
        assert_eq!(x86_reg.size_bits(), 64);
        assert_eq!(x86_reg.size_bytes(), 8);
        assert!(x86_reg.is_64bit());
        assert!(!x86_reg.is_gp());
        assert!(!x86_reg.is_simd()); // MMX is not considered SIMD in our classification
        assert!(!x86_reg.is_special());
        assert_eq!(x86_reg.nasm_name(), name);
    }
}

#[test]
fn test_all_fpu_registers() {
    let fpu_regs = vec![
        (FPURegister::St0, "st0"),
        (FPURegister::St1, "st1"),
        (FPURegister::St2, "st2"),
        (FPURegister::St3, "st3"),
        (FPURegister::St4, "st4"),
        (FPURegister::St5, "st5"),
        (FPURegister::St6, "st6"),
        (FPURegister::St7, "st7"),
    ];

    for (reg, name) in fpu_regs {
        let x86_reg = X86Register::Fpu(reg);
        assert_eq!(x86_reg.size_bits(), 80);
        assert_eq!(x86_reg.size_bytes(), 10);
        assert!(x86_reg.is_float());
        assert!(!x86_reg.is_gp());
        assert!(!x86_reg.is_simd());
        assert!(!x86_reg.is_special());
        assert_eq!(x86_reg.nasm_name(), name);
    }
}

#[test]
fn test_all_flags_registers() {
    let flags_regs = vec![
        (FlagsRegister::Rflags, "rflags", 64, 8),
        (FlagsRegister::Eflags, "eflags", 32, 4),
        (FlagsRegister::Flags, "flags", 16, 2),
    ];

    for (reg, name, bits, bytes) in flags_regs {
        let x86_reg = X86Register::Flags(reg);
        assert_eq!(x86_reg.size_bits(), bits);
        assert_eq!(x86_reg.size_bytes(), bytes);
        assert!(x86_reg.is_special());
        assert!(!x86_reg.is_gp());
        assert_eq!(x86_reg.nasm_name(), name);
    }
}

#[test]
fn test_all_instruction_pointer_registers() {
    let ip_regs = vec![
        (InstructionPointer::Rip, "rip", 64, 8),
        (InstructionPointer::Eip, "eip", 32, 4),
        (InstructionPointer::Ip, "ip", 16, 2),
    ];

    for (reg, name, bits, bytes) in ip_regs {
        let x86_reg = X86Register::InstructionPointer(reg);
        assert_eq!(x86_reg.size_bits(), bits);
        assert_eq!(x86_reg.size_bytes(), bytes);
        assert!(x86_reg.is_special());
        assert!(!x86_reg.is_gp());
        assert_eq!(x86_reg.nasm_name(), name);
    }
}

// ============================================================================
// Display and Formatting Tests
// ============================================================================

#[test]
fn test_display_matches_nasm_name() {
    // Test that Display trait output matches nasm_name() for all register types
    let test_regs = vec![
        X86Register::GP64(GPRegister64::Rax),
        X86Register::GP32(GPRegister32::Ebx),
        X86Register::GP16(GPRegister16::Cx),
        X86Register::GP8(GPRegister8::Dl),
        X86Register::Fpu(FPURegister::St3),
        X86Register::Mmx(MMXRegister::Mm5),
        X86Register::Xmm(XMMRegister::Xmm7),
        X86Register::Ymm(YMMRegister::Ymm9),
        X86Register::Zmm(ZMMRegister::Zmm15),
        X86Register::Mask(MaskRegister::K3),
        X86Register::Segment(SegmentRegister::Fs),
        X86Register::Control(ControlRegister::Cr3),
        X86Register::Debug(DebugRegister::Dr6),
        X86Register::Flags(FlagsRegister::Rflags),
        X86Register::InstructionPointer(InstructionPointer::Rip),
    ];

    for reg in test_regs {
        let display_str = format!("{reg}");
        let nasm_str = reg.nasm_name();
        assert_eq!(display_str, nasm_str, "Display and nasm_name should match for {reg:?}");
    }
}

#[test]
fn test_display_all_gp64_registers() {
    let gp64_regs = vec![
        (GPRegister64::Rax, "rax"),
        (GPRegister64::Rbx, "rbx"),
        (GPRegister64::Rcx, "rcx"),
        (GPRegister64::Rdx, "rdx"),
        (GPRegister64::Rsi, "rsi"),
        (GPRegister64::Rdi, "rdi"),
        (GPRegister64::Rbp, "rbp"),
        (GPRegister64::Rsp, "rsp"),
        (GPRegister64::R8, "r8"),
        (GPRegister64::R9, "r9"),
        (GPRegister64::R10, "r10"),
        (GPRegister64::R11, "r11"),
        (GPRegister64::R12, "r12"),
        (GPRegister64::R13, "r13"),
        (GPRegister64::R14, "r14"),
        (GPRegister64::R15, "r15"),
    ];

    for (reg, expected) in gp64_regs {
        let display = format!("{}", X86Register::GP64(reg));
        assert_eq!(display, expected, "GP64 register {reg:?} display mismatch");
    }
}

// ============================================================================
// Return Register Tests
// ============================================================================

#[test]
fn test_non_return_registers() {
    // Test registers that should never be return registers
    let non_return_regs = vec![
        X86Register::GP64(GPRegister64::Rcx),
        X86Register::GP64(GPRegister64::Rbx),
        X86Register::GP64(GPRegister64::Rsi),
        X86Register::GP64(GPRegister64::Rdi),
        X86Register::GP64(GPRegister64::R8),
        X86Register::Xmm(XMMRegister::Xmm2),
        X86Register::Xmm(XMMRegister::Xmm3),
        X86Register::Xmm(XMMRegister::Xmm15),
        X86Register::Fpu(FPURegister::St1),
        X86Register::Segment(SegmentRegister::Ds),
    ];

    for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
        for reg in &non_return_regs {
            assert!(!reg.is_return_register(platform), "{reg:?} should not be return register on {platform:?}");
        }
    }
}

// ============================================================================
// Constant Array Validation Tests
// ============================================================================

#[test]
fn test_int_param_regs_systemv_contents() {
    // Verify exact order and contents
    assert_eq!(INT_PARAM_REGS_SYSTEMV.len(), 6);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[0], GPRegister64::Rdi);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[1], GPRegister64::Rsi);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[2], GPRegister64::Rdx);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[3], GPRegister64::Rcx);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[4], GPRegister64::R8);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[5], GPRegister64::R9);
}

#[test]
fn test_int_param_regs_windows_contents() {
    // Verify exact order and contents
    assert_eq!(INT_PARAM_REGS_WINDOWS.len(), 4);
    assert_eq!(INT_PARAM_REGS_WINDOWS[0], GPRegister64::Rcx);
    assert_eq!(INT_PARAM_REGS_WINDOWS[1], GPRegister64::Rdx);
    assert_eq!(INT_PARAM_REGS_WINDOWS[2], GPRegister64::R8);
    assert_eq!(INT_PARAM_REGS_WINDOWS[3], GPRegister64::R9);
}

#[test]
#[allow(clippy::needless_range_loop)]
fn test_float_param_regs_systemv_contents() {
    assert_eq!(FLOAT_PARAM_REGS_SYSTEMV.len(), 8);
    for i in 0..8 {
        let expected = match i {
            0 => XMMRegister::Xmm0,
            1 => XMMRegister::Xmm1,
            2 => XMMRegister::Xmm2,
            3 => XMMRegister::Xmm3,
            4 => XMMRegister::Xmm4,
            5 => XMMRegister::Xmm5,
            6 => XMMRegister::Xmm6,
            7 => XMMRegister::Xmm7,
            _ => unreachable!(),
        };
        assert_eq!(FLOAT_PARAM_REGS_SYSTEMV[i], expected);
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn test_float_param_regs_windows_contents() {
    assert_eq!(FLOAT_PARAM_REGS_WINDOWS.len(), 4);
    for i in 0..4 {
        let expected = match i {
            0 => XMMRegister::Xmm0,
            1 => XMMRegister::Xmm1,
            2 => XMMRegister::Xmm2,
            3 => XMMRegister::Xmm3,
            _ => unreachable!(),
        };
        assert_eq!(FLOAT_PARAM_REGS_WINDOWS[i], expected);
    }
}

#[test]
fn test_callee_saved_arrays_no_duplicates() {
    // Verify no duplicates in callee-saved arrays
    use std::collections::HashSet;

    // Check GP registers by comparing lengths (since GPRegister64 doesn't implement Hash)
    // We verify by checking if all elements are unique through iteration
    for (i, reg1) in CALLEE_SAVED_GP_SYSTEMV.iter().enumerate() {
        for (j, reg2) in CALLEE_SAVED_GP_SYSTEMV.iter().enumerate() {
            if i != j {
                assert_ne!(reg1, reg2, "Duplicate found in CALLEE_SAVED_GP_SYSTEMV at indices {i} and {j}");
            }
        }
    }

    for (i, reg1) in CALLEE_SAVED_GP_WINDOWS.iter().enumerate() {
        for (j, reg2) in CALLEE_SAVED_GP_WINDOWS.iter().enumerate() {
            if i != j {
                assert_ne!(reg1, reg2, "Duplicate found in CALLEE_SAVED_GP_WINDOWS at indices {i} and {j}");
            }
        }
    }

    // XMM registers implement Hash, so we can use HashSet
    let xmm_windows_set: HashSet<_> = CALLEE_SAVED_XMM_WINDOWS.iter().collect();
    assert_eq!(xmm_windows_set.len(), CALLEE_SAVED_XMM_WINDOWS.len(), "Duplicates in CALLEE_SAVED_XMM_WINDOWS");
}

#[test]
fn test_caller_saved_arrays_no_duplicates() {
    use std::collections::HashSet;

    // Check GP registers through iteration (GPRegister64 doesn't implement Hash)
    for (i, reg1) in CALLER_SAVED_GP_SYSTEMV.iter().enumerate() {
        for (j, reg2) in CALLER_SAVED_GP_SYSTEMV.iter().enumerate() {
            if i != j {
                assert_ne!(reg1, reg2, "Duplicate found in CALLER_SAVED_GP_SYSTEMV at indices {i} and {j}");
            }
        }
    }

    for (i, reg1) in CALLER_SAVED_GP_WINDOWS.iter().enumerate() {
        for (j, reg2) in CALLER_SAVED_GP_WINDOWS.iter().enumerate() {
            if i != j {
                assert_ne!(reg1, reg2, "Duplicate found in CALLER_SAVED_GP_WINDOWS at indices {i} and {j}");
            }
        }
    }

    // XMM registers implement Hash, so we can use HashSet
    let systemv_xmm_set: HashSet<_> = CALLER_SAVED_XMM_SYSTEMV.iter().collect();
    assert_eq!(systemv_xmm_set.len(), CALLER_SAVED_XMM_SYSTEMV.len(), "Duplicates in CALLER_SAVED_XMM_SYSTEMV");

    let windows_xmm_set: HashSet<_> = CALLER_SAVED_XMM_WINDOWS.iter().collect();
    assert_eq!(windows_xmm_set.len(), CALLER_SAVED_XMM_WINDOWS.len(), "Duplicates in CALLER_SAVED_XMM_WINDOWS");
}

// ============================================================================
// Edge Cases: Boundary Conditions
// ============================================================================

#[test]
fn test_size_calculations_dont_overflow() {
    // Ensure size calculations don't cause arithmetic overflow
    let test_regs = vec![
        X86Register::GP8(GPRegister8::Al),   // Smallest: 8 bits
        X86Register::Zmm(ZMMRegister::Zmm0), // Largest: 512 bits
        X86Register::Fpu(FPURegister::St0),  // Unusual: 80 bits
    ];

    for reg in test_regs {
        let bits = reg.size_bits();
        let bytes = reg.size_bytes();

        // Verify the relationship
        assert_eq!(bytes * 8, bits, "Size relationship mismatch for {reg:?}");

        // Ensure no overflow occurred (basic sanity check)
        assert!(bits > 0 && bits <= 512);
        assert!(bytes > 0 && bytes <= 64);
    }
}

#[test]
fn test_equality_across_different_variants() {
    // Ensure different variants of X86Register are never equal
    let rax_64 = X86Register::GP64(GPRegister64::Rax);
    let eax_32 = X86Register::GP32(GPRegister32::Eax);
    let ax_16 = X86Register::GP16(GPRegister16::Ax);
    let al_8 = X86Register::GP8(GPRegister8::Al);

    // These are all different parts of the same physical register,
    // but should not be equal in our type system
    assert_ne!(rax_64, eax_32);
    assert_ne!(eax_32, ax_16);
    assert_ne!(ax_16, al_8);
    assert_ne!(rax_64, al_8);
}

#[test]
fn test_all_platforms_covered() {
    // Ensure all platform variants work with all methods
    let platforms = vec![Platform::Windows, Platform::Linux, Platform::MacOS];
    let test_reg = X86Register::GP64(GPRegister64::Rax);

    for platform in platforms {
        // These should all execute without panicking
        let _ = test_reg.is_volatile(platform);
        let _ = test_reg.is_callee_saved(platform);
        let _ = test_reg.is_parameter_register(platform, 0);
        let _ = test_reg.is_return_register(platform);
    }
}

// ============================================================================
// Comprehensive Integration Tests
// ============================================================================

#[test]
fn test_parameter_register_indices_match_arrays() {
    // Verify that is_parameter_register matches the constant arrays

    // Test Windows integer parameters
    for (idx, &reg) in INT_PARAM_REGS_WINDOWS.iter().enumerate() {
        let x86_reg = X86Register::GP64(reg);
        assert!(x86_reg.is_parameter_register(Platform::Windows, idx), "{reg:?} should be parameter {idx} on Windows");
    }

    // Test System V integer parameters
    for (idx, &reg) in INT_PARAM_REGS_SYSTEMV.iter().enumerate() {
        let x86_reg = X86Register::GP64(reg);
        assert!(x86_reg.is_parameter_register(Platform::Linux, idx), "{reg:?} should be parameter {idx} on Linux");
    }

    // Test Windows float parameters
    for (idx, &reg) in FLOAT_PARAM_REGS_WINDOWS.iter().enumerate() {
        let x86_reg = X86Register::Xmm(reg);
        assert!(
            x86_reg.is_parameter_register(Platform::Windows, idx),
            "{reg:?} should be float parameter {idx} on Windows",
        );
    }

    // Test System V float parameters
    for (idx, &reg) in FLOAT_PARAM_REGS_SYSTEMV.iter().enumerate() {
        let x86_reg = X86Register::Xmm(reg);
        assert!(
            x86_reg.is_parameter_register(Platform::Linux, idx),
            "{reg:?} should be float parameter {idx} on Linux",
        );
    }
}

#[test]
fn test_return_registers_match_arrays() {
    // Verify that is_return_register matches the constant arrays

    for &reg in INT_RETURN_REGS {
        let x86_reg = X86Register::GP64(reg);
        for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
            assert!(x86_reg.is_return_register(platform), "{reg:?} should be return register on {platform:?}");
        }
    }

    for &reg in FLOAT_RETURN_REGS_SYSTEMV {
        let x86_reg = X86Register::Xmm(reg);
        // Based on implementation, XMM1 is also return on all platforms
        for platform in [Platform::Windows, Platform::Linux, Platform::MacOS] {
            assert!(x86_reg.is_return_register(platform), "{reg:?} should be float return register on {platform:?}");
        }
    }
}

#[test]
fn test_callee_saved_registers_match_arrays() {
    // Verify System V callee-saved GP registers
    for &reg in CALLEE_SAVED_GP_SYSTEMV {
        let x86_reg = X86Register::GP64(reg);
        assert!(x86_reg.is_callee_saved(Platform::Linux), "{reg:?} should be callee-saved on Linux");
    }

    // Verify Windows callee-saved GP registers
    for &reg in CALLEE_SAVED_GP_WINDOWS {
        let x86_reg = X86Register::GP64(reg);
        assert!(x86_reg.is_callee_saved(Platform::Windows), "{reg:?} should be callee-saved on Windows");
    }

    // Verify Windows callee-saved XMM registers
    for &reg in CALLEE_SAVED_XMM_WINDOWS {
        let x86_reg = X86Register::Xmm(reg);
        assert!(x86_reg.is_callee_saved(Platform::Windows), "{reg:?} should be callee-saved on Windows");
    }
}
