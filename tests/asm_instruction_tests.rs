use jsavrs::asm::*;

#[test]
fn test_immediate_creation() {
    let imm8 = Immediate::Imm8(42);
    let imm32 = Immediate::Imm32(-100);
    let imm64 = Immediate::Imm64(1234567890);

    match imm8 {
        Immediate::Imm8(val) => assert_eq!(val, 42),
        _ => panic!("Expected Imm8"),
    }

    match imm32 {
        Immediate::Imm32(val) => assert_eq!(val, -100),
        _ => panic!("Expected Imm32"),
    }

    match imm64 {
        Immediate::Imm64(val) => assert_eq!(val, 1234567890),
        _ => panic!("Expected Imm64"),
    }
}

#[test]
fn test_immediate_size() {
    let imm8 = Immediate::Imm8(42);
    assert_eq!(imm8.size_bits(), 8);
    assert_eq!(imm8.size_bytes(), 1);

    let imm16 = Immediate::Imm16(1000);
    assert_eq!(imm16.size_bits(), 16);
    assert_eq!(imm16.size_bytes(), 2);

    let imm32 = Immediate::Imm32(100000);
    assert_eq!(imm32.size_bits(), 32);
    assert_eq!(imm32.size_bytes(), 4);

    let imm64 = Immediate::Imm64(1000000000);
    assert_eq!(imm64.size_bits(), 64);
    assert_eq!(imm64.size_bytes(), 8);
}

#[test]
fn test_immediate_conversion() {
    let imm32 = Immediate::Imm32(-100);
    assert_eq!(imm32.as_i64(), -100);
    assert_eq!(imm32.as_u64(), -100i64 as u64);
    assert!(imm32.is_signed());

    let imm32u = Immediate::Imm32u(100);
    assert_eq!(imm32u.as_i64(), 100);
    assert_eq!(imm32u.as_u64(), 100);
    assert!(!imm32u.is_signed());
}

#[test]
fn test_immediate_fits_in() {
    let imm64 = Immediate::Imm64(100);
    assert!(imm64.fits_in(8));
    assert!(imm64.fits_in(16));
    assert!(imm64.fits_in(32));
    assert!(imm64.fits_in(64));

    let imm_neg = Immediate::Imm64(-1000);
    assert!(!imm_neg.fits_in(8));
    assert!(imm_neg.fits_in(16));
    assert!(imm_neg.fits_in(32));
    assert!(imm_neg.fits_in(64));

    let imm_large = Immediate::Imm64(i32::MAX as i64 + 1);
    assert!(!imm_large.fits_in(8));
    assert!(!imm_large.fits_in(16));
    assert!(!imm_large.fits_in(32));
    assert!(imm_large.fits_in(64));
}

#[test]
fn test_immediate_from_conversions() {
    let imm8: Immediate = 42i8.into();
    assert!(matches!(imm8, Immediate::Imm8(42)));

    let imm32: Immediate = 123i32.into();
    assert!(matches!(imm32, Immediate::Imm32(123)));

    let imm64: Immediate = 1234567890i64.into();
    assert!(matches!(imm64, Immediate::Imm64(1234567890)));
}

#[test]
fn test_immediate_display() {
    assert_eq!(format!("{}", Immediate::Imm8(42)), "42");
    assert_eq!(format!("{}", Immediate::Imm8u(255)), "0xff");
    assert_eq!(format!("{}", Immediate::Imm16(-1000)), "-1000");
    assert_eq!(format!("{}", Immediate::Imm32(100000)), "100000");
    assert_eq!(format!("{}", Immediate::Imm64(1000000000000)), "1000000000000");
    assert_eq!(format!("{}", Immediate::Imm64u(18446744073709551615)), "0xffffffffffffffff");
}

#[test]
fn test_memory_operand_creation() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax));
    assert_eq!(mem.base, Some(GPRegister64::Rax));
    assert_eq!(mem.index, None);
    assert_eq!(mem.scale, 1);
    assert_eq!(mem.displacement, 0);
    assert_eq!(mem.size, 8);

    let mem_with_disp = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(32);
    assert_eq!(mem_with_disp.displacement, 32);

    let mem_with_index = MemoryOperand::new(Some(GPRegister64::Rcx)).with_index(GPRegister64::Rdx, 4);
    assert_eq!(mem_with_index.index, Some(GPRegister64::Rdx));
    assert_eq!(mem_with_index.scale, 4);

    let mem_with_size = MemoryOperand::new(Some(GPRegister64::Rsi)).with_size(4);
    assert_eq!(mem_with_size.size, 4);
}

#[test]
fn test_memory_operand_display() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax));
    assert_eq!(format!("{}", mem), "QWORD PTR [rax]");

    let mem_disp = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(10);
    assert_eq!(format!("{}", mem_disp), "QWORD PTR [rbx + 10]");

    let mem_disp_neg = MemoryOperand::new(Some(GPRegister64::Rcx)).with_displacement(-5);
    assert_eq!(format!("{}", mem_disp_neg), "QWORD PTR [rcx - 5]");

    let mem_index = MemoryOperand::new(Some(GPRegister64::Rsi)).with_index(GPRegister64::Rdi, 2).with_size(4);
    assert_eq!(format!("{}", mem_index), "DWORD PTR [rsi + rdi*2]");

    let mem_all =
        MemoryOperand::new(Some(GPRegister64::Rbp)).with_index(GPRegister64::R12, 8).with_displacement(16).with_size(1);
    assert_eq!(format!("{}", mem_all), "BYTE PTR [rbp + r12*8 + 16]");
}

#[test]
fn test_operand_creation() {
    let reg_op = Operand::reg64(GPRegister64::Rax);
    assert!(matches!(reg_op, Operand::Register(X86Register::GP64(GPRegister64::Rax))));

    let imm_op = Operand::imm32(42);
    assert!(matches!(imm_op, Operand::Immediate(Immediate::Imm32(42))));

    let mem_op = Operand::mem(GPRegister64::Rcx);
    assert!(matches!(mem_op, Operand::Memory(_)));

    let label_op = Operand::label("my_label");
    assert!(matches!(label_op, Operand::Label(ref s) if s == "my_label"));
}

#[test]
fn test_operand_type_checking() {
    let reg_op = Operand::reg64(GPRegister64::Rax);
    assert!(reg_op.is_register());
    assert!(!reg_op.is_immediate());
    assert!(!reg_op.is_memory());
    assert!(!reg_op.is_label());

    let imm_op = Operand::imm32(42);
    assert!(!imm_op.is_register());
    assert!(imm_op.is_immediate());
    assert!(!imm_op.is_memory());
    assert!(!imm_op.is_label());

    let mem_op = Operand::mem(GPRegister64::Rcx);
    assert!(!mem_op.is_register());
    assert!(!mem_op.is_immediate());
    assert!(mem_op.is_memory());
    assert!(!mem_op.is_label());

    let label_op = Operand::label("my_label");
    assert!(!label_op.is_register());
    assert!(!label_op.is_immediate());
    assert!(!label_op.is_memory());
    assert!(label_op.is_label());
}

#[test]
fn test_operand_display() {
    assert_eq!(format!("{}", Operand::reg64(GPRegister64::Rax)), "rax");
    assert_eq!(format!("{}", Operand::imm32(42)), "42");
    assert_eq!(format!("{}", Operand::mem(GPRegister64::Rbx)), "QWORD PTR [rbx]");
    assert_eq!(format!("{}", Operand::label("start")), "start");
}

#[test]
fn test_instruction_mnemonics() {
    assert_eq!(Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "add");
    assert_eq!(Instruction::Sub { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(1) }.mnemonic(), "sub");
    assert_eq!(Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(42) }.mnemonic(), "mov");
    assert_eq!(Instruction::Call { target: Operand::label("func".to_string()) }.mnemonic(), "call");
    assert_eq!(Instruction::Ret.mnemonic(), "ret");
    assert_eq!(Instruction::Nop.mnemonic(), "nop");
}

#[test]
fn test_instruction_type_checking() {
    assert!(Instruction::Jmp { target: Operand::label("label".to_string()) }.is_jump());
    assert!(!Instruction::Jmp { target: Operand::label("label".to_string()) }.is_call());
    assert!(!Instruction::Jmp { target: Operand::label("label".to_string()) }.is_return());

    assert!(Instruction::Je { target: Operand::label("label".to_string()) }.is_jump());
    assert!(Instruction::Jne { target: Operand::label("label".to_string()) }.is_jump());
    assert!(Instruction::Jz { target: Operand::label("label".to_string()) }.is_jump());

    assert!(Instruction::Call { target: Operand::label("func".to_string()) }.is_call());
    assert!(!Instruction::Call { target: Operand::label("func".to_string()) }.is_jump());
    assert!(!Instruction::Call { target: Operand::label("func".to_string()) }.is_return());

    assert!(Instruction::Ret.is_return());
    assert!(Instruction::RetImm { imm: 8 }.is_return());
    assert!(!Instruction::Ret.is_call());
    assert!(!Instruction::Ret.is_jump());
}

#[test]
fn test_instruction_display() {
    let add_instr =
        Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(format!("{}", add_instr), "add rax, rbx");

    let mov_instr = Instruction::Mov { dest: Operand::reg64(GPRegister64::Rcx), src: Operand::imm32(42) };
    assert_eq!(format!("{}", mov_instr), "mov rcx, 42");

    let call_instr = Instruction::Call { target: Operand::label("my_function".to_string()) };
    assert_eq!(format!("{}", call_instr), "call my_function");

    let ret_instr = Instruction::Ret;
    assert_eq!(format!("{}", ret_instr), "ret");

    let nop_instr = Instruction::Nop;
    assert_eq!(format!("{}", nop_instr), "nop");
}

#[test]
fn test_imul_instruction_variants() {
    // imul reg
    let instr1 = Instruction::Imul { dest: None, src1: Operand::reg64(GPRegister64::Rax), src2: None };
    assert_eq!(format!("{}", instr1), "imul rax");

    // imul dest, src
    let instr2 = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rbx)),
        src1: Operand::reg64(GPRegister64::Rcx),
        src2: None,
    };
    assert_eq!(format!("{}", instr2), "imul rbx, rcx");

    // imul dest, src1, src2
    let instr3 = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rdx)),
        src1: Operand::reg64(GPRegister64::Rsi),
        src2: Some(Operand::imm32(5)),
    };
    assert_eq!(format!("{}", instr3), "imul rdx, rsi, 5");
}

#[test]
fn test_immediate_edge_values() {
    let min_i8 = Immediate::Imm8(i8::MIN);
    let max_i8 = Immediate::Imm8(i8::MAX);
    assert_eq!(min_i8.as_i64(), i8::MIN as i64);
    assert_eq!(max_i8.as_i64(), i8::MAX as i64);

    let min_i32 = Immediate::Imm32(i32::MIN);
    let max_i32 = Immediate::Imm32(i32::MAX);
    assert_eq!(min_i32.as_i64(), i32::MIN as i64);
    assert_eq!(max_i32.as_i64(), i32::MAX as i64);

    let min_i64 = Immediate::Imm64(i64::MIN);
    let max_i64 = Immediate::Imm64(i64::MAX);
    assert_eq!(min_i64.as_i64(), i64::MIN);
    assert_eq!(max_i64.as_i64(), i64::MAX);
}

#[test]
fn test_memory_operand_edge_cases() {
    // Memory with no base register (absolute addressing)
    let mem_no_base = MemoryOperand::new(None).with_displacement(0x12345678);
    assert_eq!(format!("{}", mem_no_base), "QWORD PTR [305419896]");

    // Memory with all components
    let mem_all = MemoryOperand::new(Some(GPRegister64::Rbp))
        .with_index(GPRegister64::R15, 8)
        .with_displacement(-1000)
        .with_size(16);
    assert_eq!(format!("{}", mem_all), "XMMWORD PTR [rbp + r15*8 - 1000]");
}

#[test]
fn test_operand_from_conversions() {
    let imm8: Operand = 42i8.into();
    assert!(matches!(imm8, Operand::Immediate(Immediate::Imm8(42))));

    let reg: Operand = X86Register::GP64(GPRegister64::Rax).into();
    assert!(matches!(reg, Operand::Register(X86Register::GP64(GPRegister64::Rax))));
}

#[test]
fn test_instruction_complex_operands() {
    let mem_op = Operand::Memory(MemoryOperand::new(Some(GPRegister64::Rbp)).with_displacement(-8).with_size(4));

    let mov_instr = Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: mem_op };

    assert_eq!(format!("{}", mov_instr), "mov rax, DWORD PTR [rbp - 8]");
}

#[test]
fn test_instruction_imul_edge_cases() {
    // The case that would lead to unreachable!() in the match
    // We'll test the two valid patterns instead
    let instr1 = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rax)),
        src1: Operand::reg64(GPRegister64::Rbx),
        src2: None,
    };
    assert_eq!(format!("{}", instr1), "imul rax, rbx");

    let instr2 = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rcx)),
        src1: Operand::reg64(GPRegister64::Rdx),
        src2: Some(Operand::imm32(7)),
    };
    assert_eq!(format!("{}", instr2), "imul rcx, rdx, 7");
}

#[test]
fn test_large_immediate_display() {
    let large_imm = Immediate::Imm64(i64::MAX);
    let display = format!("{}", large_imm);
    assert!(display.contains(&i64::MAX.to_string()));
}

#[test]
fn test_instruction_clone() {
    let original = Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(42) };
    let cloned = original.clone();
    assert_eq!(format!("{}", original), format!("{}", cloned));
}

#[test]
fn test_immediate_clone() {
    let original = Immediate::Imm32(123);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_operand_clone() {
    let original = Operand::reg64(GPRegister64::Rax);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_memory_operand_clone() {
    let original = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(100);
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_all_immediate_types() {
    // Test all signed immediate types
    let imm8 = Immediate::Imm8(-128);
    assert_eq!(imm8.size_bits(), 8);
    assert_eq!(imm8.size_bytes(), 1);
    assert_eq!(imm8.as_i64(), -128);
    assert_eq!(imm8.as_u64(), -128i8 as u64);
    assert!(imm8.is_signed());

    let imm16 = Immediate::Imm16(-32768);
    assert_eq!(imm16.size_bits(), 16);
    assert_eq!(imm16.size_bytes(), 2);
    assert_eq!(imm16.as_i64(), -32768);
    assert_eq!(imm16.as_u64(), -32768i16 as u64);
    assert!(imm16.is_signed());

    let imm32 = Immediate::Imm32(-2147483648);
    assert_eq!(imm32.size_bits(), 32);
    assert_eq!(imm32.size_bytes(), 4);
    assert_eq!(imm32.as_i64(), -2147483648);
    assert_eq!(imm32.as_u64(), -2147483648i32 as u64);
    assert!(imm32.is_signed());

    let imm64 = Immediate::Imm64(-9223372036854775808);
    assert_eq!(imm64.size_bits(), 64);
    assert_eq!(imm64.size_bytes(), 8);
    assert_eq!(imm64.as_i64(), -9223372036854775808);
    assert_eq!(imm64.as_u64(), -9223372036854775808i64 as u64);
    assert!(imm64.is_signed());

    // Test all unsigned immediate types
    let imm8u = Immediate::Imm8u(255);
    assert_eq!(imm8u.size_bits(), 8);
    assert_eq!(imm8u.size_bytes(), 1);
    assert_eq!(imm8u.as_i64(), 255);
    assert_eq!(imm8u.as_u64(), 255);
    assert!(!imm8u.is_signed());

    let imm16u = Immediate::Imm16u(65535);
    assert_eq!(imm16u.size_bits(), 16);
    assert_eq!(imm16u.size_bytes(), 2);
    assert_eq!(imm16u.as_i64(), 65535);
    assert_eq!(imm16u.as_u64(), 65535);
    assert!(!imm16u.is_signed());

    let imm32u = Immediate::Imm32u(4294967295);
    assert_eq!(imm32u.size_bits(), 32);
    assert_eq!(imm32u.size_bytes(), 4);
    assert_eq!(imm32u.as_i64(), 4294967295);
    assert_eq!(imm32u.as_u64(), 4294967295);
    assert!(!imm32u.is_signed());

    let imm64u = Immediate::Imm64u(18446744073709551615);
    assert_eq!(imm64u.size_bits(), 64);
    assert_eq!(imm64u.size_bytes(), 8);
    assert_eq!(imm64u.as_i64(), 18446744073709551615u64 as i64);
    assert_eq!(imm64u.as_u64(), 18446744073709551615);
    assert!(!imm64u.is_signed());
}

#[test]
fn test_immediate_from_all_types() {
    let imm8: Immediate = i8::MAX.into();
    assert!(matches!(imm8, Immediate::Imm8(v) if v == i8::MAX));

    let imm8u: Immediate = u8::MAX.into();
    assert!(matches!(imm8u, Immediate::Imm8u(v) if v == u8::MAX));

    let imm16: Immediate = i16::MAX.into();
    assert!(matches!(imm16, Immediate::Imm16(v) if v == i16::MAX));

    let imm16u: Immediate = u16::MAX.into();
    assert!(matches!(imm16u, Immediate::Imm16u(v) if v == u16::MAX));

    let imm32: Immediate = i32::MAX.into();
    assert!(matches!(imm32, Immediate::Imm32(v) if v == i32::MAX));

    let imm32u: Immediate = u32::MAX.into();
    assert!(matches!(imm32u, Immediate::Imm32u(v) if v == u32::MAX));

    let imm64: Immediate = i64::MAX.into();
    assert!(matches!(imm64, Immediate::Imm64(v) if v == i64::MAX));

    let imm64u: Immediate = u64::MAX.into();
    assert!(matches!(imm64u, Immediate::Imm64u(v) if v == u64::MAX));
}

#[test]
fn test_immediate_display_all_types() {
    assert_eq!(format!("{}", Immediate::Imm8(i8::MIN)), "-128");
    assert_eq!(format!("{}", Immediate::Imm8(i8::MAX)), "127");
    assert_eq!(format!("{}", Immediate::Imm8u(0)), "0x00");
    assert_eq!(format!("{}", Immediate::Imm8u(u8::MAX)), "0xff");

    assert_eq!(format!("{}", Immediate::Imm16(i16::MIN)), "-32768");
    assert_eq!(format!("{}", Immediate::Imm16(i16::MAX)), "32767");
    assert_eq!(format!("{}", Immediate::Imm16u(0)), "0x0000");
    assert_eq!(format!("{}", Immediate::Imm16u(u16::MAX)), "0xffff");

    assert_eq!(format!("{}", Immediate::Imm32(i32::MIN)), "-2147483648");
    assert_eq!(format!("{}", Immediate::Imm32(i32::MAX)), "2147483647");
    assert_eq!(format!("{}", Immediate::Imm32u(0)), "0x00000000");
    assert_eq!(format!("{}", Immediate::Imm32u(u32::MAX)), "0xffffffff");

    assert_eq!(format!("{}", Immediate::Imm64(i64::MIN)), "-9223372036854775808");
    assert_eq!(format!("{}", Immediate::Imm64(i64::MAX)), "9223372036854775807");
    assert_eq!(format!("{}", Immediate::Imm64u(0)), "0x0000000000000000");
    assert_eq!(format!("{}", Immediate::Imm64u(u64::MAX)), "0xffffffffffffffff");
}

#[test]
fn test_immediate_fits_in_all_sizes() {
    let imm8 = Immediate::Imm8(100);
    assert!(imm8.fits_in(8));
    assert!(imm8.fits_in(16));
    assert!(imm8.fits_in(32));
    assert!(imm8.fits_in(64));

    let imm8_min = Immediate::Imm8(i8::MIN);
    assert!(imm8_min.fits_in(8));
    assert!(imm8_min.fits_in(16));
    assert!(imm8_min.fits_in(32));
    assert!(imm8_min.fits_in(64));

    let imm8_max = Immediate::Imm8(i8::MAX);
    assert!(imm8_max.fits_in(8));
    assert!(imm8_max.fits_in(16));
    assert!(imm8_max.fits_in(32));
    assert!(imm8_max.fits_in(64));

    let imm16 = Immediate::Imm16(20000);
    assert!(!imm16.fits_in(8));
    assert!(imm16.fits_in(16));
    assert!(imm16.fits_in(32));
    assert!(imm16.fits_in(64));

    let imm32 = Immediate::Imm32(1000000000);
    assert!(!imm32.fits_in(8));
    assert!(!imm32.fits_in(16));
    assert!(imm32.fits_in(32));
    assert!(imm32.fits_in(64));

    let imm64 = Immediate::Imm64(1000000000000000000);
    assert!(!imm64.fits_in(8));
    assert!(!imm64.fits_in(16));
    assert!(!imm64.fits_in(32));
    assert!(imm64.fits_in(64));

    // Test invalid bits
    let imm = Immediate::Imm8(42);
    assert!(!imm.fits_in(0));
    assert!(!imm.fits_in(7));
    assert!(!imm.fits_in(15));
    assert!(!imm.fits_in(17));
    assert!(!imm.fits_in(31));
    assert!(!imm.fits_in(33));
    assert!(!imm.fits_in(63));
    assert!(!imm.fits_in(65));
}

#[test]
fn test_arithmetic_instructions() {
    // Add instruction
    let add_instr =
        Instruction::Add { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(add_instr.mnemonic(), "add");
    assert_eq!(format!("{}", add_instr), "add rax, rbx");

    // Sub instruction
    let sub_instr = Instruction::Sub { dest: Operand::reg64(GPRegister64::Rcx), src: Operand::imm32(100) };
    assert_eq!(sub_instr.mnemonic(), "sub");
    assert_eq!(format!("{}", sub_instr), "sub rcx, 100");

    // Mul instruction
    let mul_instr = Instruction::Mul { src: Operand::reg64(GPRegister64::Rdx) };
    assert_eq!(mul_instr.mnemonic(), "mul");
    assert_eq!(format!("{}", mul_instr), "mul rdx");

    // Imul instruction with all variants
    let imul1 = Instruction::Imul { dest: None, src1: Operand::reg64(GPRegister64::Rsi), src2: None };
    assert_eq!(imul1.mnemonic(), "imul");
    assert_eq!(format!("{}", imul1), "imul rsi");

    let imul2 = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::Rdi)),
        src1: Operand::reg64(GPRegister64::R8),
        src2: None,
    };
    assert_eq!(imul2.mnemonic(), "imul");
    assert_eq!(format!("{}", imul2), "imul rdi, r8");

    let imul3 = Instruction::Imul {
        dest: Some(Operand::reg64(GPRegister64::R9)),
        src1: Operand::reg64(GPRegister64::R10),
        src2: Some(Operand::imm32(42)),
    };
    assert_eq!(imul3.mnemonic(), "imul");
    assert_eq!(format!("{}", imul3), "imul r9, r10, 42");

    // Div instruction
    let div_instr = Instruction::Div { src: Operand::reg64(GPRegister64::R11) };
    assert_eq!(div_instr.mnemonic(), "div");
    assert_eq!(format!("{}", div_instr), "div r11");

    // Idiv instruction
    let idiv_instr = Instruction::Idiv { src: Operand::mem(GPRegister64::Rsp) };
    assert_eq!(idiv_instr.mnemonic(), "idiv");
    assert_eq!(format!("{}", idiv_instr), "idiv QWORD PTR [rsp]");

    // Inc instruction
    let inc_instr = Instruction::Inc { dest: Operand::reg64(GPRegister64::Rax) };
    assert_eq!(inc_instr.mnemonic(), "inc");
    assert_eq!(format!("{}", inc_instr), "inc rax");

    // Dec instruction
    let dec_instr = Instruction::Dec { dest: Operand::reg32(GPRegister32::Ebx) };
    assert_eq!(dec_instr.mnemonic(), "dec");
    assert_eq!(format!("{}", dec_instr), "dec ebx");

    // Neg instruction
    let neg_instr = Instruction::Neg { dest: Operand::reg16(GPRegister16::Cx) };
    assert_eq!(neg_instr.mnemonic(), "neg");
    assert_eq!(format!("{}", neg_instr), "neg cx");

    // Adc instruction
    let adc_instr = Instruction::Adc { dest: Operand::reg8(GPRegister8::Al), src: Operand::imm8(5) };
    assert_eq!(adc_instr.mnemonic(), "adc");
    assert_eq!(format!("{}", adc_instr), "adc al, 5");

    // Sbb instruction
    let sbb_instr =
        Instruction::Sbb { dest: Operand::reg64(GPRegister64::R12), src: Operand::reg64(GPRegister64::R13) };
    assert_eq!(sbb_instr.mnemonic(), "sbb");
    assert_eq!(format!("{}", sbb_instr), "sbb r12, r13");
}

#[test]
fn test_logical_instructions() {
    // And instruction
    let and_instr =
        Instruction::And { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(and_instr.mnemonic(), "and");
    assert_eq!(format!("{}", and_instr), "and rax, rbx");

    // Or instruction
    let or_instr = Instruction::Or { dest: Operand::reg64(GPRegister64::Rcx), src: Operand::imm32(255) };
    assert_eq!(or_instr.mnemonic(), "or");
    assert_eq!(format!("{}", or_instr), "or rcx, 255");

    // Xor instruction
    let xor_instr =
        Instruction::Xor { dest: Operand::reg64(GPRegister64::Rdx), src: Operand::reg64(GPRegister64::Rsi) };
    assert_eq!(xor_instr.mnemonic(), "xor");
    assert_eq!(format!("{}", xor_instr), "xor rdx, rsi");

    // Not instruction
    let not_instr = Instruction::Not { dest: Operand::reg64(GPRegister64::Rdi) };
    assert_eq!(not_instr.mnemonic(), "not");
    assert_eq!(format!("{}", not_instr), "not rdi");

    // Test with different operand types
    let and_mem_imm = Instruction::And { dest: Operand::mem(GPRegister64::Rsp), src: Operand::imm32(0xFF) };
    assert_eq!(format!("{}", and_mem_imm), "and QWORD PTR [rsp], 255");

    let or_reg_mem = Instruction::Or { dest: Operand::reg32(GPRegister32::Eax), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(format!("{}", or_reg_mem), "or eax, QWORD PTR [rbx]");

    let xor_with_label = Instruction::Xor { dest: Operand::reg8(GPRegister8::Al), src: Operand::imm8(-1) };
    assert_eq!(format!("{}", xor_with_label), "xor al, -1");

    // Test Test instruction
    let test_instr = Instruction::Test { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::imm32(1) };
    assert_eq!(test_instr.mnemonic(), "test");
    assert_eq!(format!("{}", test_instr), "test rax, 1");
}

#[test]
fn test_shift_rotate_instructions() {
    // Shl instruction
    let shl_instr = Instruction::Shl { dest: Operand::reg64(GPRegister64::Rax), count: Operand::imm8(1) };
    assert_eq!(shl_instr.mnemonic(), "shl");
    assert_eq!(format!("{}", shl_instr), "shl rax, 1");

    // Shr instruction
    let shr_instr = Instruction::Shr { dest: Operand::reg32(GPRegister32::Ebx), count: Operand::reg8(GPRegister8::Cl) };
    assert_eq!(shr_instr.mnemonic(), "shr");
    assert_eq!(format!("{}", shr_instr), "shr ebx, cl");

    // Sar instruction
    let sar_instr = Instruction::Sar { dest: Operand::reg16(GPRegister16::Dx), count: Operand::imm8(2) };
    assert_eq!(sar_instr.mnemonic(), "sar");
    assert_eq!(format!("{}", sar_instr), "sar dx, 2");

    // Sal instruction (same as Shl)
    let sal_instr = Instruction::Sal { dest: Operand::reg8(GPRegister8::Al), count: Operand::imm8(3) };
    assert_eq!(sal_instr.mnemonic(), "sal");
    assert_eq!(format!("{}", sal_instr), "sal al, 3");

    // Rol instruction
    let rol_instr = Instruction::Rol { dest: Operand::reg64(GPRegister64::Rcx), count: Operand::reg8(GPRegister8::Cl) };
    assert_eq!(rol_instr.mnemonic(), "rol");
    assert_eq!(format!("{}", rol_instr), "rol rcx, cl");

    // Ror instruction
    let ror_instr = Instruction::Ror { dest: Operand::reg64(GPRegister64::Rdx), count: Operand::imm8(4) };
    assert_eq!(ror_instr.mnemonic(), "ror");
    assert_eq!(format!("{}", ror_instr), "ror rdx, 4");

    // Rcl instruction
    let rcl_instr = Instruction::Rcl { dest: Operand::reg32(GPRegister32::Esi), count: Operand::imm8(1) };
    assert_eq!(rcl_instr.mnemonic(), "rcl");
    assert_eq!(format!("{}", rcl_instr), "rcl esi, 1");

    // Rcr instruction
    let rcr_instr = Instruction::Rcr { dest: Operand::reg32(GPRegister32::Edi), count: Operand::reg8(GPRegister8::Cl) };
    assert_eq!(rcr_instr.mnemonic(), "rcr");
    assert_eq!(format!("{}", rcr_instr), "rcr edi, cl");

    // Test with memory operand
    let shl_mem = Instruction::Shl { dest: Operand::mem(GPRegister64::Rsp), count: Operand::imm8(1) };
    assert_eq!(format!("{}", shl_mem), "shl QWORD PTR [rsp], 1");
}

#[test]
fn test_movement_instructions() {
    // Mov instruction
    let mov_instr =
        Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(mov_instr.mnemonic(), "mov");
    assert_eq!(format!("{}", mov_instr), "mov rax, rbx");

    // Movsx instruction
    let movsx_instr =
        Instruction::Movsx { dest: Operand::reg64(GPRegister64::Rcx), src: Operand::reg32(GPRegister32::Edx) };
    assert_eq!(movsx_instr.mnemonic(), "movsx");
    assert_eq!(format!("{}", movsx_instr), "movsx rcx, edx");

    // Movsxd instruction
    let movsxd_instr =
        Instruction::Movsxd { dest: Operand::reg64(GPRegister64::Rsi), src: Operand::reg32(GPRegister32::Eax) };
    assert_eq!(movsxd_instr.mnemonic(), "movsxd");
    assert_eq!(format!("{}", movsxd_instr), "movsxd rsi, eax");

    // Movzx instruction
    let movzx_instr =
        Instruction::Movzx { dest: Operand::reg64(GPRegister64::Rdi), src: Operand::reg8(GPRegister8::Al) };
    assert_eq!(movzx_instr.mnemonic(), "movzx");
    assert_eq!(format!("{}", movzx_instr), "movzx rdi, al");

    // Lea instruction
    let lea_instr = Instruction::Lea { dest: Operand::reg64(GPRegister64::R8), src: Operand::mem(GPRegister64::Rsp) };
    assert_eq!(lea_instr.mnemonic(), "lea");
    assert_eq!(format!("{}", lea_instr), "lea r8, QWORD PTR [rsp]");

    // Push instruction
    let push_instr = Instruction::Push { src: Operand::reg64(GPRegister64::R9) };
    assert_eq!(push_instr.mnemonic(), "push");
    assert_eq!(format!("{}", push_instr), "push r9");

    // Pop instruction
    let pop_instr = Instruction::Pop { dest: Operand::reg64(GPRegister64::R10) };
    assert_eq!(pop_instr.mnemonic(), "pop");
    assert_eq!(format!("{}", pop_instr), "pop r10");

    // Xchg instruction
    let xchg_instr =
        Instruction::Xchg { op1: Operand::reg64(GPRegister64::R11), op2: Operand::reg64(GPRegister64::R12) };
    assert_eq!(xchg_instr.mnemonic(), "xchg");
    assert_eq!(format!("{}", xchg_instr), "xchg r11, r12");

    // Test different operand combinations
    let mov_reg_mem =
        Instruction::Mov { dest: Operand::reg32(GPRegister32::Eax), src: Operand::mem_disp(GPRegister64::Rbp, -8) };
    assert_eq!(format!("{}", mov_reg_mem), "mov eax, QWORD PTR [rbp - 8]");

    let mov_mem_reg =
        Instruction::Mov { dest: Operand::mem(GPRegister64::Rsp), src: Operand::reg64(GPRegister64::Rax) };
    assert_eq!(format!("{}", mov_mem_reg), "mov QWORD PTR [rsp], rax");

    let mov_reg_imm = Instruction::Mov { dest: Operand::reg16(GPRegister16::Ax), src: Operand::imm16(0x1234) };
    assert_eq!(format!("{}", mov_reg_imm), "mov ax, 4660");

    let lea_complex = Instruction::Lea {
        dest: Operand::reg64(GPRegister64::R15),
        src: Operand::Memory(
            MemoryOperand::new(Some(GPRegister64::Rax))
                .with_index(GPRegister64::Rbx, 4)
                .with_displacement(16)
                .with_size(8),
        ),
    };
    assert_eq!(format!("{}", lea_complex), "lea r15, QWORD PTR [rax + rbx*4 + 16]");
}

#[test]
fn test_comparison_instructions() {
    // Cmp instruction
    let cmp_instr = Instruction::Cmp { op1: Operand::reg64(GPRegister64::Rax), op2: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(cmp_instr.mnemonic(), "cmp");
    assert_eq!(format!("{}", cmp_instr), "cmp rax, rbx");

    // Test with different operand types
    let cmp_mem_imm = Instruction::Cmp { op1: Operand::mem(GPRegister64::Rsp), op2: Operand::imm32(42) };
    assert_eq!(format!("{}", cmp_mem_imm), "cmp QWORD PTR [rsp], 42");

    let cmp_reg_mem =
        Instruction::Cmp { op1: Operand::reg32(GPRegister32::Eax), op2: Operand::mem_disp(GPRegister64::Rbp, 16) };
    assert_eq!(format!("{}", cmp_reg_mem), "cmp eax, QWORD PTR [rbp + 16]");

    let cmp_reg_imm = Instruction::Cmp { op1: Operand::reg16(GPRegister16::Cx), op2: Operand::imm16(-100) };
    assert_eq!(format!("{}", cmp_reg_imm), "cmp cx, -100");

    let cmp_reg_label = Instruction::Cmp { op1: Operand::reg8(GPRegister8::Dl), op2: Operand::imm8(0xF) };
    assert_eq!(format!("{}", cmp_reg_label), "cmp dl, 15");
}

#[test]
fn test_jump_instructions() {
    // Jmp instruction
    let jmp_instr = Instruction::Jmp { target: Operand::label("start") };
    assert_eq!(jmp_instr.mnemonic(), "jmp");
    assert_eq!(format!("{}", jmp_instr), "jmp start");
    assert!(jmp_instr.is_jump());

    // Conditional jumps
    let je_instr = Instruction::Je { target: Operand::label("equal") };
    assert_eq!(je_instr.mnemonic(), "je");
    assert_eq!(format!("{}", je_instr), "je equal");
    assert!(je_instr.is_jump());

    let jne_instr = Instruction::Jne { target: Operand::label("not_equal") };
    assert_eq!(jne_instr.mnemonic(), "jne");
    assert_eq!(format!("{}", jne_instr), "jne not_equal");
    assert!(jne_instr.is_jump());

    let jz_instr = Instruction::Jz { target: Operand::label("zero") };
    assert_eq!(jz_instr.mnemonic(), "jz");
    assert_eq!(format!("{}", jz_instr), "jz zero");
    assert!(jz_instr.is_jump());

    let jnz_instr = Instruction::Jnz { target: Operand::label("not_zero") };
    assert_eq!(jnz_instr.mnemonic(), "jnz");
    assert_eq!(format!("{}", jnz_instr), "jnz not_zero");
    assert!(jnz_instr.is_jump());

    let jg_instr = Instruction::Jg { target: Operand::label("greater") };
    assert_eq!(jg_instr.mnemonic(), "jg");
    assert_eq!(format!("{}", jg_instr), "jg greater");
    assert!(jg_instr.is_jump());

    let jge_instr = Instruction::Jge { target: Operand::label("greater_equal") };
    assert_eq!(jge_instr.mnemonic(), "jge");
    assert_eq!(format!("{}", jge_instr), "jge greater_equal");
    assert!(jge_instr.is_jump());

    let jl_instr = Instruction::Jl { target: Operand::label("less") };
    assert_eq!(jl_instr.mnemonic(), "jl");
    assert_eq!(format!("{}", jl_instr), "jl less");
    assert!(jl_instr.is_jump());

    let jle_instr = Instruction::Jle { target: Operand::label("less_equal") };
    assert_eq!(jle_instr.mnemonic(), "jle");
    assert_eq!(format!("{}", jle_instr), "jle less_equal");
    assert!(jle_instr.is_jump());

    let ja_instr = Instruction::Ja { target: Operand::label("above") };
    assert_eq!(ja_instr.mnemonic(), "ja");
    assert_eq!(format!("{}", ja_instr), "ja above");
    assert!(ja_instr.is_jump());

    let jae_instr = Instruction::Jae { target: Operand::label("above_equal") };
    assert_eq!(jae_instr.mnemonic(), "jae");
    assert_eq!(format!("{}", jae_instr), "jae above_equal");
    assert!(jae_instr.is_jump());

    let jb_instr = Instruction::Jb { target: Operand::label("below") };
    assert_eq!(jb_instr.mnemonic(), "jb");
    assert_eq!(format!("{}", jb_instr), "jb below");
    assert!(jb_instr.is_jump());

    let jbe_instr = Instruction::Jbe { target: Operand::label("below_equal") };
    assert_eq!(jbe_instr.mnemonic(), "jbe");
    assert_eq!(format!("{}", jbe_instr), "jbe below_equal");
    assert!(jbe_instr.is_jump());

    let js_instr = Instruction::Js { target: Operand::label("sign") };
    assert_eq!(js_instr.mnemonic(), "js");
    assert_eq!(format!("{}", js_instr), "js sign");
    assert!(js_instr.is_jump());

    let jns_instr = Instruction::Jns { target: Operand::label("no_sign") };
    assert_eq!(jns_instr.mnemonic(), "jns");
    assert_eq!(format!("{}", jns_instr), "jns no_sign");
    assert!(jns_instr.is_jump());

    let jo_instr = Instruction::Jo { target: Operand::label("overflow") };
    assert_eq!(jo_instr.mnemonic(), "jo");
    assert_eq!(format!("{}", jo_instr), "jo overflow");
    assert!(jo_instr.is_jump());

    let jno_instr = Instruction::Jno { target: Operand::label("no_overflow") };
    assert_eq!(jno_instr.mnemonic(), "jno");
    assert_eq!(format!("{}", jno_instr), "jno no_overflow");
    assert!(jno_instr.is_jump());

    let jp_instr = Instruction::Jp { target: Operand::label("parity") };
    assert_eq!(jp_instr.mnemonic(), "jp");
    assert_eq!(format!("{}", jp_instr), "jp parity");
    assert!(jp_instr.is_jump());

    let jnp_instr = Instruction::Jnp { target: Operand::label("no_parity") };
    assert_eq!(jnp_instr.mnemonic(), "jnp");
    assert_eq!(format!("{}", jnp_instr), "jnp no_parity");
    assert!(jnp_instr.is_jump());

    // Test with memory operand as target (though this is uncommon)
    let jmp_mem = Instruction::Jmp { target: Operand::mem(GPRegister64::Rax) };
    assert_eq!(format!("{}", jmp_mem), "jmp QWORD PTR [rax]");

    // Test with register operand as target
    let jmp_reg = Instruction::Jmp { target: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(format!("{}", jmp_reg), "jmp rbx");
}

#[test]
fn test_sse_avx_instructions() {
    // SSE movement instructions
    let movaps_instr =
        Instruction::Movaps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(movaps_instr.mnemonic(), "movaps");
    assert_eq!(format!("{}", movaps_instr), "movaps xmm0, xmm1");

    let movapd_instr =
        Instruction::Movapd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(movapd_instr.mnemonic(), "movapd");
    assert_eq!(format!("{}", movapd_instr), "movapd xmm2, QWORD PTR [rax]");

    let movups_instr =
        Instruction::Movups { dest: Operand::mem(GPRegister64::Rbx), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_eq!(movups_instr.mnemonic(), "movups");
    assert_eq!(format!("{}", movups_instr), "movups QWORD PTR [rbx], xmm3");

    let movupd_instr =
        Instruction::Movupd { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) };
    assert_eq!(movupd_instr.mnemonic(), "movupd");
    assert_eq!(format!("{}", movupd_instr), "movupd xmm4, xmm5");

    let movss_instr =
        Instruction::Movss { dest: Operand::xmm(XMMRegister::Xmm6), src: Operand::xmm(XMMRegister::Xmm7) };
    assert_eq!(movss_instr.mnemonic(), "movss");
    assert_eq!(format!("{}", movss_instr), "movss xmm6, xmm7");

    let movsd_instr =
        Instruction::Movsd { dest: Operand::xmm(XMMRegister::Xmm8), src: Operand::mem(GPRegister64::Rcx) };
    assert_eq!(movsd_instr.mnemonic(), "movsd");
    assert_eq!(format!("{}", movsd_instr), "movsd xmm8, QWORD PTR [rcx]");

    let movdqa_instr =
        Instruction::Movdqa { dest: Operand::xmm(XMMRegister::Xmm9), src: Operand::xmm(XMMRegister::Xmm10) };
    assert_eq!(movdqa_instr.mnemonic(), "movdqa");
    assert_eq!(format!("{}", movdqa_instr), "movdqa xmm9, xmm10");

    let movdqu_instr =
        Instruction::Movdqu { dest: Operand::mem(GPRegister64::Rdx), src: Operand::xmm(XMMRegister::Xmm11) };
    assert_eq!(movdqu_instr.mnemonic(), "movdqu");
    assert_eq!(format!("{}", movdqu_instr), "movdqu QWORD PTR [rdx], xmm11");

    // SSE arithmetic instructions
    let addps_instr =
        Instruction::Addps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(addps_instr.mnemonic(), "addps");
    assert_eq!(format!("{}", addps_instr), "addps xmm0, xmm1");

    let addpd_instr =
        Instruction::Addpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(addpd_instr.mnemonic(), "addpd");
    assert_eq!(format!("{}", addpd_instr), "addpd xmm2, QWORD PTR [rax]");

    let addss_instr =
        Instruction::Addss { dest: Operand::xmm(XMMRegister::Xmm3), src: Operand::xmm(XMMRegister::Xmm4) };
    assert_eq!(addss_instr.mnemonic(), "addss");
    assert_eq!(format!("{}", addss_instr), "addss xmm3, xmm4");

    let addsd_instr =
        Instruction::Addsd { dest: Operand::xmm(XMMRegister::Xmm5), src: Operand::xmm(XMMRegister::Xmm6) };
    assert_eq!(addsd_instr.mnemonic(), "addsd");
    assert_eq!(format!("{}", addsd_instr), "addsd xmm5, xmm6");

    let subps_instr =
        Instruction::Subps { dest: Operand::xmm(XMMRegister::Xmm7), src: Operand::xmm(XMMRegister::Xmm8) };
    assert_eq!(subps_instr.mnemonic(), "subps");
    assert_eq!(format!("{}", subps_instr), "subps xmm7, xmm8");

    let subpd_instr =
        Instruction::Subpd { dest: Operand::xmm(XMMRegister::Xmm9), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(subpd_instr.mnemonic(), "subpd");
    assert_eq!(format!("{}", subpd_instr), "subpd xmm9, QWORD PTR [rbx]");

    let mulps_instr =
        Instruction::Mulps { dest: Operand::xmm(XMMRegister::Xmm10), src: Operand::xmm(XMMRegister::Xmm11) };
    assert_eq!(mulps_instr.mnemonic(), "mulps");
    assert_eq!(format!("{}", mulps_instr), "mulps xmm10, xmm11");

    let mulpd_instr =
        Instruction::Mulpd { dest: Operand::xmm(XMMRegister::Xmm12), src: Operand::xmm(XMMRegister::Xmm13) };
    assert_eq!(mulpd_instr.mnemonic(), "mulpd");
    assert_eq!(format!("{}", mulpd_instr), "mulpd xmm12, xmm13");

    let divps_instr =
        Instruction::Divps { dest: Operand::xmm(XMMRegister::Xmm14), src: Operand::mem(GPRegister64::Rcx) };
    assert_eq!(divps_instr.mnemonic(), "divps");
    assert_eq!(format!("{}", divps_instr), "divps xmm14, QWORD PTR [rcx]");

    let divpd_instr =
        Instruction::Divpd { dest: Operand::xmm(XMMRegister::Xmm15), src: Operand::xmm(XMMRegister::Xmm0) };
    assert_eq!(divpd_instr.mnemonic(), "divpd");
    assert_eq!(format!("{}", divpd_instr), "divpd xmm15, xmm0");

    // SSE logical instructions
    let andps_instr =
        Instruction::Andps { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(andps_instr.mnemonic(), "andps");
    assert_eq!(format!("{}", andps_instr), "andps xmm0, xmm1");

    let andpd_instr =
        Instruction::Andpd { dest: Operand::xmm(XMMRegister::Xmm2), src: Operand::xmm(XMMRegister::Xmm3) };
    assert_eq!(andpd_instr.mnemonic(), "andpd");
    assert_eq!(format!("{}", andpd_instr), "andpd xmm2, xmm3");

    let orps_instr = Instruction::Orps { dest: Operand::xmm(XMMRegister::Xmm4), src: Operand::xmm(XMMRegister::Xmm5) };
    assert_eq!(orps_instr.mnemonic(), "orps");
    assert_eq!(format!("{}", orps_instr), "orps xmm4, xmm5");

    let orpd_instr = Instruction::Orpd { dest: Operand::xmm(XMMRegister::Xmm6), src: Operand::xmm(XMMRegister::Xmm7) };
    assert_eq!(orpd_instr.mnemonic(), "orpd");
    assert_eq!(format!("{}", orpd_instr), "orpd xmm6, xmm7");

    let xorps_instr =
        Instruction::Xorps { dest: Operand::xmm(XMMRegister::Xmm8), src: Operand::xmm(XMMRegister::Xmm9) };
    assert_eq!(xorps_instr.mnemonic(), "xorps");
    assert_eq!(format!("{}", xorps_instr), "xorps xmm8, xmm9");

    let xorpd_instr =
        Instruction::Xorpd { dest: Operand::xmm(XMMRegister::Xmm10), src: Operand::xmm(XMMRegister::Xmm11) };
    assert_eq!(xorpd_instr.mnemonic(), "xorpd");
    assert_eq!(format!("{}", xorpd_instr), "xorpd xmm10, xmm11");

    // Conversion instructions
    let cvtss2sd_instr =
        Instruction::Cvtss2sd { dest: Operand::xmm(XMMRegister::Xmm0), src: Operand::xmm(XMMRegister::Xmm1) };
    assert_eq!(cvtss2sd_instr.mnemonic(), "cvtss2sd");
    assert_eq!(format!("{}", cvtss2sd_instr), "cvtss2sd xmm0, xmm1");

    // AVX instructions
    let vaddps_instr = Instruction::Vaddps {
        dest: Operand::xmm(XMMRegister::Xmm0),
        src1: Operand::xmm(XMMRegister::Xmm1),
        src2: Operand::xmm(XMMRegister::Xmm2),
    };
    assert_eq!(vaddps_instr.mnemonic(), "vaddps");
    assert_eq!(format!("{}", vaddps_instr), "vaddps xmm0, xmm1, xmm2");

    let vsubpd_instr = Instruction::Vsubpd {
        dest: Operand::xmm(XMMRegister::Xmm3),
        src1: Operand::xmm(XMMRegister::Xmm4),
        src2: Operand::mem(GPRegister64::Rax),
    };
    assert_eq!(vsubpd_instr.mnemonic(), "vsubpd");
    assert_eq!(format!("{}", vsubpd_instr), "vsubpd xmm3, xmm4, QWORD PTR [rax]");

    let vmulps_instr = Instruction::Vmulps {
        dest: Operand::xmm(XMMRegister::Xmm5),
        src1: Operand::xmm(XMMRegister::Xmm6),
        src2: Operand::xmm(XMMRegister::Xmm7),
    };
    assert_eq!(vmulps_instr.mnemonic(), "vmulps");
    assert_eq!(format!("{}", vmulps_instr), "vmulps xmm5, xmm6, xmm7");

    let vdivpd_instr = Instruction::Vdivpd {
        dest: Operand::xmm(XMMRegister::Xmm8),
        src1: Operand::mem(GPRegister64::Rbx),
        src2: Operand::xmm(XMMRegister::Xmm9),
    };
    assert_eq!(vdivpd_instr.mnemonic(), "vdivpd");
    assert_eq!(format!("{}", vdivpd_instr), "vdivpd xmm8, QWORD PTR [rbx], xmm9");
}

#[test]
fn test_fpu_instructions() {
    // FLD instruction
    let fld_mem = Instruction::Fld { src: Operand::mem(GPRegister64::Rax) };
    assert_eq!(fld_mem.mnemonic(), "fld");
    assert_eq!(format!("{}", fld_mem), "fld QWORD PTR [rax]");

    let fld_st = Instruction::Fld {
        src: Operand::reg64(GPRegister64::Rax)  // Using a register for demonstration
    };
    assert_eq!(fld_st.mnemonic(), "fld");
    assert_eq!(format!("{}", fld_st), "fld rax");

    // FST instruction
    let fst_mem = Instruction::Fst { dest: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(fst_mem.mnemonic(), "fst");
    assert_eq!(format!("{}", fst_mem), "fst QWORD PTR [rbx]");

    // FSTP instruction
    let fstp_mem = Instruction::Fstp { dest: Operand::mem(GPRegister64::Rcx) };
    assert_eq!(fstp_mem.mnemonic(), "fstp");
    assert_eq!(format!("{}", fstp_mem), "fstp QWORD PTR [rcx]");

    // FADD instruction
    let fadd_instr = Instruction::Fadd { src: Some(Operand::mem(GPRegister64::Rdx)) };
    assert_eq!(fadd_instr.mnemonic(), "fadd");
    assert_eq!(format!("{}", fadd_instr), "fadd QWORD PTR [rdx]");

    let fadd_no_src = Instruction::Fadd { src: None };
    assert_eq!(fadd_no_src.mnemonic(), "fadd");
    assert_eq!(format!("{}", fadd_no_src), "fadd");

    // FADDP instruction
    let faddp_instr = Instruction::Faddp { src: Some(Operand::reg64(GPRegister64::Rsi)) };
    assert_eq!(faddp_instr.mnemonic(), "faddp");
    assert_eq!(format!("{}", faddp_instr), "faddp rsi");

    // FSUB instruction
    let fsub_instr = Instruction::Fsub { src: Some(Operand::mem(GPRegister64::Rdi)) };
    assert_eq!(fsub_instr.mnemonic(), "fsub");
    assert_eq!(format!("{}", fsub_instr), "fsub QWORD PTR [rdi]");

    // FSUBP instruction
    let fsubp_instr = Instruction::Fsubp { src: Some(Operand::reg64(GPRegister64::R8)) };
    assert_eq!(fsubp_instr.mnemonic(), "fsubp");
    assert_eq!(format!("{}", fsubp_instr), "fsubp r8");

    // FMUL instruction
    let fmul_instr = Instruction::Fmul { src: Some(Operand::mem(GPRegister64::R9)) };
    assert_eq!(fmul_instr.mnemonic(), "fmul");
    assert_eq!(format!("{}", fmul_instr), "fmul QWORD PTR [r9]");

    // FMULP instruction
    let fmulp_instr = Instruction::Fmulp { src: Some(Operand::reg64(GPRegister64::R10)) };
    assert_eq!(fmulp_instr.mnemonic(), "fmulp");
    assert_eq!(format!("{}", fmulp_instr), "fmulp r10");

    // FDIV instruction
    let fdiv_instr = Instruction::Fdiv { src: Some(Operand::mem(GPRegister64::R11)) };
    assert_eq!(fdiv_instr.mnemonic(), "fdiv");
    assert_eq!(format!("{}", fdiv_instr), "fdiv QWORD PTR [r11]");

    // FDIVP instruction
    let fdivp_instr = Instruction::Fdivp { src: Some(Operand::reg64(GPRegister64::R12)) };
    assert_eq!(fdivp_instr.mnemonic(), "fdivp");
    assert_eq!(format!("{}", fdivp_instr), "fdivp r12");

    // Test with different register types for FPU
    let fadd_reg = Instruction::Fadd {
        src: Some(Operand::xmm(XMMRegister::Xmm0)), // Using XMM register for demonstration
    };
    assert_eq!(format!("{}", fadd_reg), "fadd xmm0");
}

#[test]
fn test_bit_manipulation_instructions() {
    // BSF instruction
    let bsf_instr =
        Instruction::Bsf { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(bsf_instr.mnemonic(), "bsf");
    assert_eq!(format!("{}", bsf_instr), "bsf rax, rbx");

    // BSR instruction
    let bsr_instr = Instruction::Bsr { dest: Operand::reg32(GPRegister32::Ecx), src: Operand::mem(GPRegister64::Rdx) };
    assert_eq!(bsr_instr.mnemonic(), "bsr");
    assert_eq!(format!("{}", bsr_instr), "bsr ecx, QWORD PTR [rdx]");

    // BT instruction
    let bt_instr = Instruction::Bt { dest: Operand::reg16(GPRegister16::Ax), src: Operand::imm8(5) };
    assert_eq!(bt_instr.mnemonic(), "bt");
    assert_eq!(format!("{}", bt_instr), "bt ax, 5");

    // BTC instruction
    let btc_instr =
        Instruction::Btc { dest: Operand::reg64(GPRegister64::Rsi), src: Operand::reg64(GPRegister64::Rdi) };
    assert_eq!(btc_instr.mnemonic(), "btc");
    assert_eq!(format!("{}", btc_instr), "btc rsi, rdi");

    // BTR instruction
    let btr_instr = Instruction::Btr { dest: Operand::reg32(GPRegister32::Edx), src: Operand::imm8(3) };
    assert_eq!(btr_instr.mnemonic(), "btr");
    assert_eq!(format!("{}", btr_instr), "btr edx, 3");

    // BTS instruction
    let bts_instr = Instruction::Bts { dest: Operand::mem(GPRegister64::Rsp), src: Operand::reg64(GPRegister64::Rax) };
    assert_eq!(bts_instr.mnemonic(), "bts");
    assert_eq!(format!("{}", bts_instr), "bts QWORD PTR [rsp], rax");

    // POPCNT instruction
    let popcnt_instr =
        Instruction::Popcnt { dest: Operand::reg64(GPRegister64::Rcx), src: Operand::reg64(GPRegister64::Rdx) };
    assert_eq!(popcnt_instr.mnemonic(), "popcnt");
    assert_eq!(format!("{}", popcnt_instr), "popcnt rcx, rdx");

    // LZCNT instruction
    let lzcnt_instr =
        Instruction::Lzcnt { dest: Operand::reg32(GPRegister32::Esi), src: Operand::mem(GPRegister64::Rbx) };
    assert_eq!(lzcnt_instr.mnemonic(), "lzcnt");
    assert_eq!(format!("{}", lzcnt_instr), "lzcnt esi, QWORD PTR [rbx]");

    // TZCNT instruction
    let tzcnt_instr =
        Instruction::Tzcnt { dest: Operand::reg16(GPRegister16::Dx), src: Operand::reg16(GPRegister16::Ax) };
    assert_eq!(tzcnt_instr.mnemonic(), "tzcnt");
    assert_eq!(format!("{}", tzcnt_instr), "tzcnt dx, ax");

    // Test with different combinations
    let bt_mem_imm = Instruction::Bt { dest: Operand::mem(GPRegister64::Rbp), src: Operand::imm16(16) };
    assert_eq!(format!("{}", bt_mem_imm), "bt QWORD PTR [rbp], 16");

    let popcnt_mem =
        Instruction::Popcnt { dest: Operand::reg64(GPRegister64::R8), src: Operand::mem(GPRegister64::R9) };
    assert_eq!(format!("{}", popcnt_mem), "popcnt r8, QWORD PTR [r9]");
}

#[test]
fn test_cmovcc_instructions() {
    // CMOVE instruction
    let cmove_instr =
        Instruction::Cmove { dest: Operand::reg64(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(cmove_instr.mnemonic(), "cmove");
    assert_eq!(format!("{}", cmove_instr), "cmove rax, rbx");

    // CMOVNE instruction
    let cmovne_instr =
        Instruction::Cmovne { dest: Operand::reg32(GPRegister32::Ecx), src: Operand::mem(GPRegister64::Rdx) };
    assert_eq!(cmovne_instr.mnemonic(), "cmovne");
    assert_eq!(format!("{}", cmovne_instr), "cmovne ecx, QWORD PTR [rdx]");

    // CMOVG instruction
    let cmovg_instr =
        Instruction::Cmovg { dest: Operand::reg64(GPRegister64::Rsi), src: Operand::reg64(GPRegister64::Rdi) };
    assert_eq!(cmovg_instr.mnemonic(), "cmovg");
    assert_eq!(format!("{}", cmovg_instr), "cmovg rsi, rdi");

    // CMOVGE instruction
    let cmovge_instr =
        Instruction::Cmovge { dest: Operand::reg16(GPRegister16::Ax), src: Operand::reg16(GPRegister16::Bx) };
    assert_eq!(cmovge_instr.mnemonic(), "cmovge");
    assert_eq!(format!("{}", cmovge_instr), "cmovge ax, bx");

    // CMOVL instruction
    let cmovl_instr =
        Instruction::Cmovl { dest: Operand::reg32(GPRegister32::Edx), src: Operand::mem(GPRegister64::Rsp) };
    assert_eq!(cmovl_instr.mnemonic(), "cmovl");
    assert_eq!(format!("{}", cmovl_instr), "cmovl edx, QWORD PTR [rsp]");

    // CMOVLE instruction
    let cmovle_instr =
        Instruction::Cmovle { dest: Operand::reg64(GPRegister64::R8), src: Operand::reg64(GPRegister64::R9) };
    assert_eq!(cmovle_instr.mnemonic(), "cmovle");
    assert_eq!(format!("{}", cmovle_instr), "cmovle r8, r9");

    // CMOVA instruction
    let cmova_instr =
        Instruction::Cmova { dest: Operand::reg16(GPRegister16::Cx), src: Operand::reg16(GPRegister16::Dx) };
    assert_eq!(cmova_instr.mnemonic(), "cmova");
    assert_eq!(format!("{}", cmova_instr), "cmova cx, dx");

    // CMOVAE instruction
    let cmovae_instr =
        Instruction::Cmovae { dest: Operand::reg64(GPRegister64::R10), src: Operand::mem(GPRegister64::Rbp) };
    assert_eq!(cmovae_instr.mnemonic(), "cmovae");
    assert_eq!(format!("{}", cmovae_instr), "cmovae r10, QWORD PTR [rbp]");

    // CMOVB instruction
    let cmovb_instr = Instruction::Cmovb { dest: Operand::reg8(GPRegister8::Al), src: Operand::reg8(GPRegister8::Bl) };
    assert_eq!(cmovb_instr.mnemonic(), "cmovb");
    assert_eq!(format!("{}", cmovb_instr), "cmovb al, bl");

    // CMOVBE instruction
    let cmovbe_instr =
        Instruction::Cmovbe { dest: Operand::reg64(GPRegister64::R11), src: Operand::reg64(GPRegister64::R12) };
    assert_eq!(cmovbe_instr.mnemonic(), "cmovbe");
    assert_eq!(format!("{}", cmovbe_instr), "cmovbe r11, r12");

    // Test with different operand combinations
    let cmov_mem_reg =
        Instruction::Cmove { dest: Operand::mem(GPRegister64::Rax), src: Operand::reg64(GPRegister64::Rbx) };
    assert_eq!(format!("{}", cmov_mem_reg), "cmove QWORD PTR [rax], rbx");

    let cmov_reg_mem =
        Instruction::Cmovne { dest: Operand::reg32(GPRegister32::Esi), src: Operand::mem(GPRegister64::Rdi) };
    assert_eq!(format!("{}", cmov_reg_mem), "cmovne esi, QWORD PTR [rdi]");
}
