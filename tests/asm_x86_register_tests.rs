use jsavrs::asm::*;

#[test]
fn test_x86_register_size() {
    // 64-bit registers
    assert_eq!(X86Register::GP64(GPRegister64::Rax).size_bits(), 64);
    assert_eq!(X86Register::GP64(GPRegister64::Rax).size_bytes(), 8);

    // 32-bit registers
    assert_eq!(X86Register::GP32(GPRegister32::Eax).size_bits(), 32);
    assert_eq!(X86Register::GP32(GPRegister32::Eax).size_bytes(), 4);

    // 16-bit registers
    assert_eq!(X86Register::GP16(GPRegister16::Ax).size_bits(), 16);
    assert_eq!(X86Register::GP16(GPRegister16::Ax).size_bytes(), 2);

    // 8-bit registers
    assert_eq!(X86Register::GP8(GPRegister8::Al).size_bits(), 8);
    assert_eq!(X86Register::GP8(GPRegister8::Al).size_bytes(), 1);

    // XMM registers
    assert_eq!(X86Register::Xmm(XMMRegister::Xmm0).size_bits(), 128);
    assert_eq!(X86Register::Xmm(XMMRegister::Xmm0).size_bytes(), 16);

    // YMM registers
    assert_eq!(X86Register::Ymm(YMMRegister::Ymm0).size_bits(), 256);
    assert_eq!(X86Register::Ymm(YMMRegister::Ymm0).size_bytes(), 32);

    // ZMM registers
    assert_eq!(X86Register::Zmm(ZMMRegister::Zmm0).size_bits(), 512);
    assert_eq!(X86Register::Zmm(ZMMRegister::Zmm0).size_bytes(), 64);

    // FPU register
    assert_eq!(X86Register::Fpu(FPURegister::St0).size_bits(), 80);
    assert_eq!(X86Register::Fpu(FPURegister::St0).size_bytes(), 10);

    // Instruction pointer
    assert_eq!(X86Register::InstructionPointer(InstructionPointer::Rip).size_bits(), 64);
    assert_eq!(X86Register::InstructionPointer(InstructionPointer::Rip).size_bytes(), 8);
}

#[test]
fn test_x86_register_type_checking() {
    // General purpose
    assert!(X86Register::GP64(GPRegister64::Rax).is_gp());
    assert!(X86Register::GP32(GPRegister32::Eax).is_gp());
    assert!(X86Register::GP16(GPRegister16::Ax).is_gp());
    assert!(X86Register::GP8(GPRegister8::Al).is_gp());
    assert!(!X86Register::Xmm(XMMRegister::Xmm0).is_gp());

    // SIMD
    assert!(X86Register::Xmm(XMMRegister::Xmm0).is_simd());
    assert!(X86Register::Ymm(YMMRegister::Ymm0).is_simd());
    assert!(X86Register::Zmm(ZMMRegister::Zmm0).is_simd());
    assert!(!X86Register::GP64(GPRegister64::Rax).is_simd());

    // Float
    assert!(X86Register::Fpu(FPURegister::St0).is_float());
    assert!(X86Register::Xmm(XMMRegister::Xmm0).is_float());
    assert!(X86Register::Ymm(YMMRegister::Ymm0).is_float());
    assert!(X86Register::Zmm(ZMMRegister::Zmm0).is_float());
    assert!(!X86Register::GP64(GPRegister64::Rax).is_float());

    // Special
    assert!(X86Register::Segment(SegmentRegister::Cs).is_special());
    assert!(X86Register::Control(ControlRegister::Cr0).is_special());
    assert!(X86Register::Debug(DebugRegister::Dr0).is_special());
    assert!(X86Register::Flags(FlagsRegister::Rflags).is_special());
    assert!(X86Register::InstructionPointer(InstructionPointer::Rip).is_special());
    assert!(!X86Register::GP64(GPRegister64::Rax).is_special());
}

#[test]
fn test_x86_register_bit_size_checking() {
    assert!(X86Register::GP64(GPRegister64::Rax).is_64bit());
    assert!(!X86Register::GP64(GPRegister64::Rax).is_32bit());
    assert!(!X86Register::GP64(GPRegister64::Rax).is_16bit());
    assert!(!X86Register::GP64(GPRegister64::Rax).is_8bit());

    assert!(!X86Register::GP32(GPRegister32::Eax).is_64bit());
    assert!(X86Register::GP32(GPRegister32::Eax).is_32bit());
    assert!(!X86Register::GP32(GPRegister32::Eax).is_16bit());
    assert!(!X86Register::GP32(GPRegister32::Eax).is_8bit());

    assert!(!X86Register::GP16(GPRegister16::Ax).is_64bit());
    assert!(!X86Register::GP16(GPRegister16::Ax).is_32bit());
    assert!(X86Register::GP16(GPRegister16::Ax).is_16bit());
    assert!(!X86Register::GP16(GPRegister16::Ax).is_8bit());

    assert!(!X86Register::GP8(GPRegister8::Al).is_64bit());
    assert!(!X86Register::GP8(GPRegister8::Al).is_32bit());
    assert!(!X86Register::GP8(GPRegister8::Al).is_16bit());
    assert!(X86Register::GP8(GPRegister8::Al).is_8bit());
}

#[test]
fn test_x86_register_nasm_name() {
    assert_eq!(X86Register::GP64(GPRegister64::Rax).nasm_name(), "rax");
    assert_eq!(X86Register::GP64(GPRegister64::Rbx).nasm_name(), "rbx");
    assert_eq!(X86Register::GP32(GPRegister32::Ecx).nasm_name(), "ecx");
    assert_eq!(X86Register::GP16(GPRegister16::Dx).nasm_name(), "dx");
    assert_eq!(X86Register::GP8(GPRegister8::Al).nasm_name(), "al");
    assert_eq!(X86Register::Xmm(XMMRegister::Xmm0).nasm_name(), "xmm0");
    assert_eq!(X86Register::Ymm(YMMRegister::Ymm15).nasm_name(), "ymm15");
    assert_eq!(X86Register::Zmm(ZMMRegister::Zmm7).nasm_name(), "zmm7");
    assert_eq!(X86Register::Fpu(FPURegister::St0).nasm_name(), "st0");
    assert_eq!(X86Register::Segment(SegmentRegister::Ds).nasm_name(), "ds");
}

#[test]
fn test_x86_register_display() {
    assert_eq!(format!("{}", X86Register::GP64(GPRegister64::Rax)), "rax");
    assert_eq!(format!("{}", X86Register::Xmm(XMMRegister::Xmm5)), "xmm5");
    assert_eq!(format!("{}", X86Register::Fpu(FPURegister::St3)), "st3");
}

#[test]
fn test_x86_register_equality() {
    assert_eq!(X86Register::GP64(GPRegister64::Rax), X86Register::GP64(GPRegister64::Rax));
    assert_ne!(X86Register::GP64(GPRegister64::Rax), X86Register::GP64(GPRegister64::Rbx));

    assert_eq!(X86Register::Xmm(XMMRegister::Xmm0), X86Register::Xmm(XMMRegister::Xmm0));
    assert_ne!(X86Register::Xmm(XMMRegister::Xmm0), X86Register::Xmm(XMMRegister::Xmm1));
}

#[test]
fn test_x86_register_volatile_behavior() {
    // Test if registers are volatile (caller-saved) on different platforms
    let rax = X86Register::GP64(GPRegister64::Rax);
    let rbx = X86Register::GP64(GPRegister64::Rbx);

    // RAX is volatile on both platforms
    assert!(rax.is_volatile(Platform::Linux));
    assert!(rax.is_volatile(Platform::Windows));

    // RBX is not volatile (it's callee-saved) on both platforms
    assert!(!rbx.is_volatile(Platform::Linux));
    assert!(!rbx.is_volatile(Platform::Windows));
}

#[test]
fn test_x86_register_callee_saved_behavior() {
    let rax = X86Register::GP64(GPRegister64::Rax);
    let rbx = X86Register::GP64(GPRegister64::Rbx);

    // RAX is not callee-saved on both platforms
    assert!(!rax.is_callee_saved(Platform::Linux));
    assert!(!rax.is_callee_saved(Platform::Windows));

    // RBX is callee-saved on both platforms
    assert!(rbx.is_callee_saved(Platform::Linux));
    assert!(rbx.is_callee_saved(Platform::Windows));
}

#[test]
fn test_x86_register_parameter_check() {
    // On Windows, RCX is the first parameter register
    let rcx = X86Register::GP64(GPRegister64::Rcx);
    assert!(rcx.is_parameter_register(Platform::Windows, 0));
    assert!(!rcx.is_parameter_register(Platform::Windows, 1));

    // On System V, RDI is the first parameter register
    let rdi = X86Register::GP64(GPRegister64::Rdi);
    assert!(rdi.is_parameter_register(Platform::Linux, 0));
    assert!(!rdi.is_parameter_register(Platform::Linux, 1));
}

#[test]
fn test_x86_register_return_check() {
    let rax = X86Register::GP64(GPRegister64::Rax);
    assert!(rax.is_return_register(Platform::Linux));
    assert!(rax.is_return_register(Platform::Windows));

    let xmm0 = X86Register::Xmm(XMMRegister::Xmm0);
    assert!(xmm0.is_return_register(Platform::Linux));
    assert!(xmm0.is_return_register(Platform::Windows));
}

#[test]
fn test_x86_register_variants() {
    // Test all register variants can be created
    let _gp64 = X86Register::GP64(GPRegister64::Rax);
    let _gp32 = X86Register::GP32(GPRegister32::Eax);
    let _gp16 = X86Register::GP16(GPRegister16::Ax);
    let _gp8 = X86Register::GP8(GPRegister8::Al);
    let _fpu = X86Register::Fpu(FPURegister::St0);
    let _mmx = X86Register::Mmx(MMXRegister::Mm0);
    let _xmm = X86Register::Xmm(XMMRegister::Xmm0);
    let _ymm = X86Register::Ymm(YMMRegister::Ymm0);
    let _zmm = X86Register::Zmm(ZMMRegister::Zmm0);
    let _mask = X86Register::Mask(MaskRegister::K0);
    let _segment = X86Register::Segment(SegmentRegister::Cs);
    let _control = X86Register::Control(ControlRegister::Cr0);
    let _debug = X86Register::Debug(DebugRegister::Dr0);
    let _flags = X86Register::Flags(FlagsRegister::Rflags);
    let _ip = X86Register::InstructionPointer(InstructionPointer::Rip);
}

#[test]
fn test_register_cloning() {
    let reg = X86Register::GP64(GPRegister64::Rax);
    let cloned_reg = reg.clone();
    assert_eq!(reg, cloned_reg);
}

#[test]
fn test_register_debug() {
    let reg = X86Register::Xmm(XMMRegister::Xmm7);
    let debug_str = format!("{:?}", reg);
    assert!(debug_str.contains("Xmm"));
}

#[test]
fn test_fpu_register_display() {
    assert_eq!(format!("{}", FPURegister::St0), "st0");
    assert_eq!(format!("{}", FPURegister::St7), "st7");

    // Test all FPU registers
    for i in 0..8 {
        let fpu_reg = match i {
            0 => FPURegister::St0,
            1 => FPURegister::St1,
            2 => FPURegister::St2,
            3 => FPURegister::St3,
            4 => FPURegister::St4,
            5 => FPURegister::St5,
            6 => FPURegister::St6,
            7 => FPURegister::St7,
            _ => unreachable!(),
        };
        let display = format!("{}", fpu_reg);
        assert_eq!(display, format!("st{}", i));
    }
}

#[test]
fn test_register_size_edge_cases() {
    // Test with different register types
    let sizes = vec![
        (X86Register::GP8(GPRegister8::Al), 8, 1),
        (X86Register::GP16(GPRegister16::Ax), 16, 2),
        (X86Register::GP32(GPRegister32::Eax), 32, 4),
        (X86Register::GP64(GPRegister64::Rax), 64, 8),
        (X86Register::Xmm(XMMRegister::Xmm0), 128, 16),
        (X86Register::Ymm(YMMRegister::Ymm0), 256, 32),
        (X86Register::Zmm(ZMMRegister::Zmm0), 512, 64),
    ];

    for (reg, expected_bits, expected_bytes) in sizes {
        assert_eq!(reg.size_bits(), expected_bits);
        assert_eq!(reg.size_bytes(), expected_bytes);
    }
}

#[test]
fn test_static_register_arrays() {
    // Test the static arrays used by ABI
    assert_eq!(INT_PARAM_REGS_SYSTEMV.len(), 6);
    assert_eq!(INT_PARAM_REGS_WINDOWS.len(), 4);
    assert_eq!(FLOAT_PARAM_REGS_SYSTEMV.len(), 8);
    assert_eq!(FLOAT_PARAM_REGS_WINDOWS.len(), 4);
    assert_eq!(INT_RETURN_REGS.len(), 2);
    assert_eq!(FLOAT_RETURN_REGS_SYSTEMV.len(), 2);
    assert_eq!(FLOAT_RETURN_REGS_WINDOWS.len(), 1);

    // Test specific values
    assert_eq!(INT_PARAM_REGS_SYSTEMV[0], GPRegister64::Rdi);
    assert_eq!(INT_PARAM_REGS_SYSTEMV[1], GPRegister64::Rsi);
    assert_eq!(INT_PARAM_REGS_WINDOWS[0], GPRegister64::Rcx);
    assert_eq!(INT_PARAM_REGS_WINDOWS[1], GPRegister64::Rdx);
}
