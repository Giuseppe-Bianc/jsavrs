use jsavrs::asm::{
    ControlRegister, DebugRegister, FPURegister, FlagsRegister, GPRegister8, GPRegister16, GPRegister32, GPRegister64,
    InstructionPointer, SegmentRegister, XMMRegister, YMMRegister, ZMMRegister,
};

#[test]
fn test_gp_register64_variants() {
    let variants = [
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

    assert_eq!(variants.len(), 16);

    // Test display formatting
    assert_eq!(format!("{}", GPRegister64::Rax), "rax");
    assert_eq!(format!("{}", GPRegister64::Rsp), "rsp");
    assert_eq!(format!("{}", GPRegister64::R15), "r15");
}

#[test]
fn test_gp_register32_variants() {
    let variants = [
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

    assert_eq!(variants.len(), 16);

    // Test display formatting
    assert_eq!(format!("{}", GPRegister32::Eax), "eax");
    assert_eq!(format!("{}", GPRegister32::Esp), "esp");
    assert_eq!(format!("{}", GPRegister32::R15d), "r15d");
}

#[test]
fn test_gp_register16_variants() {
    let variants = [
        GPRegister16::Ax,
        GPRegister16::Bx,
        GPRegister16::Cx,
        GPRegister16::Dx,
        GPRegister16::Si,
        GPRegister16::Di,
        GPRegister16::Bp,
        GPRegister16::Sp,
        GPRegister16::R8w,
        GPRegister16::R9w,
        GPRegister16::R10w,
        GPRegister16::R11w,
        GPRegister16::R12w,
        GPRegister16::R13w,
        GPRegister16::R14w,
        GPRegister16::R15w,
    ];

    assert_eq!(variants.len(), 16);

    // Test display formatting
    assert_eq!(format!("{}", GPRegister16::Ax), "ax");
    assert_eq!(format!("{}", GPRegister16::Sp), "sp");
    assert_eq!(format!("{}", GPRegister16::R15w), "r15w");
}

#[test]
fn test_gp_register8_variants() {
    let variants = [
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

    assert_eq!(variants.len(), 20);

    // Test display formatting
    assert_eq!(format!("{}", GPRegister8::Al), "al");
    assert_eq!(format!("{}", GPRegister8::Ah), "ah");
    assert_eq!(format!("{}", GPRegister8::R15b), "r15b");
}

#[test]
fn test_xmm_register_variants() {
    // Test that XMM registers have the expected number
    // Since we can't enumerate them easily, we'll test a few
    assert_eq!(format!("{}", XMMRegister::Xmm0), "xmm0");
    assert_eq!(format!("{}", XMMRegister::Xmm1), "xmm1");
    assert_eq!(format!("{}", XMMRegister::Xmm15), "xmm15");
}

#[test]
fn test_ymm_register_variants() {
    // Test a few YMM registers
    assert_eq!(format!("{}", YMMRegister::Ymm0), "ymm0");
    assert_eq!(format!("{}", YMMRegister::Ymm1), "ymm1");
    assert_eq!(format!("{}", YMMRegister::Ymm15), "ymm15");
}

#[test]
fn test_zmm_register_variants() {
    // Test a few ZMM registers
    assert_eq!(format!("{}", ZMMRegister::Zmm0), "zmm0");
    assert_eq!(format!("{}", ZMMRegister::Zmm1), "zmm1");
    assert_eq!(format!("{}", ZMMRegister::Zmm15), "zmm15");
}

#[test]
fn test_segment_register_variants() {
    assert_eq!(format!("{}", SegmentRegister::Cs), "cs");
    assert_eq!(format!("{}", SegmentRegister::Ds), "ds");
    assert_eq!(format!("{}", SegmentRegister::Es), "es");
    assert_eq!(format!("{}", SegmentRegister::Fs), "fs");
    assert_eq!(format!("{}", SegmentRegister::Gs), "gs");
    assert_eq!(format!("{}", SegmentRegister::Ss), "ss");
}

#[test]
fn test_control_register_variants() {
    assert_eq!(format!("{}", ControlRegister::Cr0), "cr0");
    assert_eq!(format!("{}", ControlRegister::Cr2), "cr2");
    assert_eq!(format!("{}", ControlRegister::Cr3), "cr3");
    assert_eq!(format!("{}", ControlRegister::Cr4), "cr4");
}

#[test]
fn test_debug_register_variants() {
    assert_eq!(format!("{}", DebugRegister::Dr0), "dr0");
    assert_eq!(format!("{}", DebugRegister::Dr1), "dr1");
    assert_eq!(format!("{}", DebugRegister::Dr2), "dr2");
    assert_eq!(format!("{}", DebugRegister::Dr3), "dr3");
}

#[test]
fn test_flags_register_variants() {
    assert_eq!(format!("{}", FlagsRegister::Rflags), "rflags");
    assert_eq!(format!("{}", FlagsRegister::Eflags), "eflags");
    assert_eq!(format!("{}", FlagsRegister::Flags), "flags");
}

#[test]
fn test_instruction_pointer_variants() {
    assert_eq!(format!("{}", InstructionPointer::Rip), "rip");
    assert_eq!(format!("{}", InstructionPointer::Eip), "eip");
    assert_eq!(format!("{}", InstructionPointer::Ip), "ip");
}

#[test]
fn test_fpu_register_variants() {
    // Test all FPU register variants to ensure complete Display trait coverage
    assert_eq!(format!("{}", FPURegister::St0), "st0");
    assert_eq!(format!("{}", FPURegister::St1), "st1");
    assert_eq!(format!("{}", FPURegister::St2), "st2");
    assert_eq!(format!("{}", FPURegister::St3), "st3");
    assert_eq!(format!("{}", FPURegister::St4), "st4");
    assert_eq!(format!("{}", FPURegister::St5), "st5");
    assert_eq!(format!("{}", FPURegister::St6), "st6");
    assert_eq!(format!("{}", FPURegister::St7), "st7");
}

#[test]
fn test_fpu_register_display_consistency() {
    // Test that all FPU registers follow the 'st' + index pattern
    let fpu_registers = [
        (FPURegister::St0, "st0"),
        (FPURegister::St1, "st1"),
        (FPURegister::St2, "st2"),
        (FPURegister::St3, "st3"),
        (FPURegister::St4, "st4"),
        (FPURegister::St5, "st5"),
        (FPURegister::St6, "st6"),
        (FPURegister::St7, "st7"),
    ];

    for (reg, expected) in &fpu_registers {
        assert_eq!(format!("{reg}"), *expected);
    }
}

#[test]
fn test_register_equality() {
    assert_eq!(GPRegister64::Rax, GPRegister64::Rax);
    assert_ne!(GPRegister64::Rax, GPRegister64::Rbx);

    assert_eq!(GPRegister32::Ecx, GPRegister32::Ecx);
    assert_ne!(GPRegister32::Ecx, GPRegister32::Edx);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_register_clone() {
    let reg = GPRegister64::Rax;
    let cloned_reg = reg.clone();
    assert_eq!(reg, cloned_reg);
}

#[test]
fn test_register_debug() {
    let reg = GPRegister64::Rbx;
    let debug_str = format!("{reg:?}");
    assert!(debug_str.contains("Rbx"));
}

#[test]
fn test_register_display_consistency() {
    // All register types should have lowercase display representations
    assert_eq!(format!("{}", GPRegister64::Rax), "rax");
    assert_eq!(format!("{}", GPRegister32::Eax), "eax");
    assert_eq!(format!("{}", GPRegister16::Ax), "ax");
    assert_eq!(format!("{}", GPRegister8::Al), "al");
    assert_eq!(format!("{}", XMMRegister::Xmm0), "xmm0");
    assert_eq!(format!("{}", YMMRegister::Ymm0), "ymm0");
    assert_eq!(format!("{}", ZMMRegister::Zmm0), "zmm0");
    assert_eq!(format!("{}", SegmentRegister::Ds), "ds");
    assert_eq!(format!("{}", ControlRegister::Cr0), "cr0");
    assert_eq!(format!("{}", DebugRegister::Dr0), "dr0");
    assert_eq!(format!("{}", FlagsRegister::Rflags), "rflags");
    assert_eq!(format!("{}", InstructionPointer::Rip), "rip");
}

#[test]
fn test_all_gp_registers_display() {
    // Test a few more register displays to ensure consistency
    assert_eq!(format!("{}", GPRegister64::R8), "r8");
    assert_eq!(format!("{}", GPRegister64::R15), "r15");

    assert_eq!(format!("{}", GPRegister32::R8d), "r8d");
    assert_eq!(format!("{}", GPRegister32::R15d), "r15d");

    assert_eq!(format!("{}", GPRegister16::R8w), "r8w");
    assert_eq!(format!("{}", GPRegister16::R15w), "r15w");

    assert_eq!(format!("{}", GPRegister8::R8b), "r8b");
    assert_eq!(format!("{}", GPRegister8::R15b), "r15b");
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_register_variant_properties() {
    // All registers should support cloning
    let reg1 = GPRegister64::Rax;
    let reg1_clone = reg1.clone();
    assert_eq!(reg1, reg1_clone);

    let reg2 = XMMRegister::Xmm5;
    let reg2_clone = reg2.clone();
    assert_eq!(reg2, reg2_clone);

    // All registers should support Debug formatting
    let reg_debug = format!("{:?}", GPRegister64::Rcx);
    assert!(reg_debug.contains("Rcx"));
}
