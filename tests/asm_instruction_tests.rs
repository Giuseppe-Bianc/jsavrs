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
