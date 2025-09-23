use jsavrs::asm::instruction::Instruction;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

#[test]
fn test_instruction_display() {
    // Test instruction display implementations
    let mov_inst = Instruction::Mov(
        Operand::reg(Register::RAX),
        Operand::imm(42)
    );
    assert_eq!(format!("{}", mov_inst), "    mov rax, 42");
    
    let add_inst = Instruction::Add(
        Operand::reg(Register::RBX),
        Operand::reg(Register::RCX)
    );
    assert_eq!(format!("{}", add_inst), "    add rbx, rcx");
    
    let jmp_inst = Instruction::Jmp("label".to_string());
    assert_eq!(format!("{}", jmp_inst), "    jmp label");
    
    let call_inst = Instruction::Call("function".to_string());
    assert_eq!(format!("{}", call_inst), "    call function");
    
    let ret_inst = Instruction::Ret;
    assert_eq!(format!("{}", ret_inst), "    ret");
}

#[test]
fn test_instruction_type_variants() {
    // Test all instruction type variants
    let mov_inst = Instruction::Mov(
        Operand::reg(Register::RAX),
        Operand::imm(42)
    );
    
    let add_inst = Instruction::Add(
        Operand::reg(Register::RBX),
        Operand::imm(100)
    );
    
    let push_inst = Instruction::Push(Operand::reg(Register::RCX));
    let pop_inst = Instruction::Pop(Operand::reg(Register::RDX));
    let cmp_inst = Instruction::Cmp(Operand::reg(Register::RSI), Operand::reg(Register::RDI));
    
    let jmp_inst = Instruction::Jmp("test_label".to_string());
    let je_inst = Instruction::Je("equal_label".to_string());
    let call_inst = Instruction::Call("function_name".to_string());
    let ret_inst = Instruction::Ret;
    
    // Verify they can be formatted without errors
    assert!(format!("{}", mov_inst).contains("mov"));
    assert!(format!("{}", add_inst).contains("add"));
    assert!(format!("{}", push_inst).contains("push"));
    assert!(format!("{}", pop_inst).contains("pop"));
    assert!(format!("{}", cmp_inst).contains("cmp"));
    assert!(format!("{}", jmp_inst).contains("jmp"));
    assert!(format!("{}", je_inst).contains("je"));
    assert!(format!("{}", call_inst).contains("call"));
    assert!(format!("{}", ret_inst).contains("ret"));
}

#[test]
fn test_instruction_specific_operand_constraints() {
    // Test instruction-specific operand constraints (e.g., div, idiv require a single operand)
    
    // Single operand instructions
    let div_inst = Instruction::Div(Operand::reg(Register::RAX));
    assert!(format!("{}", div_inst).contains("div"));
    
    let idiv_inst = Instruction::Idiv(Operand::reg(Register::RBX));
    assert!(format!("{}", idiv_inst).contains("idiv"));
    
    let inc_inst = Instruction::Inc(Operand::reg(Register::RCX));
    assert!(format!("{}", inc_inst).contains("inc"));
    
    let dec_inst = Instruction::Dec(Operand::reg(Register::RDX));
    assert!(format!("{}", dec_inst).contains("dec"));
    
    let neg_inst = Instruction::Neg(Operand::reg(Register::RSI));
    assert!(format!("{}", neg_inst).contains("neg"));
    
    let not_inst = Instruction::Not(Operand::reg(Register::RDI));
    assert!(format!("{}", not_inst).contains("not"));
    
    // Two operand instructions
    let add_inst = Instruction::Add(Operand::reg(Register::RAX), Operand::imm(10));
    assert!(format!("{}", add_inst).contains("add"));
    
    let mov_inst = Instruction::Mov(Operand::reg(Register::RBX), Operand::reg(Register::RCX));
    assert!(format!("{}", mov_inst).contains("mov"));
    
    // Three operand instruction (imul with three operands)
    let imul_inst = Instruction::Imul(
        Operand::reg(Register::RAX), 
        Some(Operand::reg(Register::RBX)), 
        Some(Operand::imm(42))
    );
    assert!(format!("{}", imul_inst).contains("imul"));
    
    // Single operand with immediate
    let imul_two_op = Instruction::Imul(
        Operand::reg(Register::RAX), 
        Some(Operand::imm(5)), 
        None
    );
    assert!(format!("{}", imul_two_op).contains("imul"));
    
    // Single operand (just destination)
    let imul_one_op = Instruction::Imul(Operand::reg(Register::RAX), None, None);
    assert!(format!("{}", imul_one_op).contains("imul"));
}

#[test]
fn test_instruction_formatting_with_complex_operand_combinations() {
    // Test instruction formatting with complex operand combinations
    
    // Memory reference operands
    let mov_mem_inst = Instruction::Mov(
        Operand::reg(Register::RAX),
        Operand::mem_base_index_scale_disp(Register::RBX, Register::RCX, 4, 8)
    );
    assert!(format!("{}", mov_mem_inst).contains("[rbx+rcx*4+8]"));
    
    // Multiple register operations
    let add_inst = Instruction::Add(
        Operand::reg(Register::RSP),
        Operand::imm(16)
    );
    assert!(format!("{}", add_inst).contains("add"));
    
    // Comparison with memory
    let cmp_mem_inst = Instruction::Cmp(
        Operand::mem_base_disp(Register::RBP, -8),
        Operand::imm(0)
    );
    assert!(format!("{}", cmp_mem_inst).contains("[rbp-8]"));
    
    // Conditional jump
    let jne_inst = Instruction::Jne("not_equal".to_string());
    assert!(format!("{}", jne_inst).contains("jne"));
}

#[test]
fn test_instruction_utility_methods() {
    // Test if we can work with instruction variants as needed
    let mov_inst = Instruction::Mov(
        Operand::reg(Register::RAX),
        Operand::imm(42)
    );
    
    // Just verify the instruction can be created and formatted
    assert_eq!(format!("{}", mov_inst), "    mov rax, 42");
    
    let add_inst = Instruction::Add(
        Operand::reg(Register::RBX),
        Operand::reg(Register::RCX)
    );
    assert_eq!(format!("{}", add_inst), "    add rbx, rcx");
    
    let syscall_inst = Instruction::Syscall;
    assert_eq!(format!("{}", syscall_inst), "    syscall");
    
    let int_inst = Instruction::Int(0x80);
    assert_eq!(format!("{}", int_inst), "    int 0x80");
    
    let cmp_inst = Instruction::Cmp(
        Operand::reg(Register::RAX),
        Operand::reg(Register::RBX)
    );
    assert_eq!(format!("{}", cmp_inst), "    cmp rax, rbx");
}