//! Tests for the assembly generator
use jsavrs::asm::generator::{NasmGenerator, TargetOS, Section};
use jsavrs::asm::register::Register;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::instruction::Instruction;

#[test]
fn test_register_display() {
    assert_eq!(format!("{}", Register::RAX), "rax");
    assert_eq!(format!("{}", Register::EAX), "eax");
    assert_eq!(format!("{}", Register::AX), "ax");
    assert_eq!(format!("{}", Register::AL), "al");
}

#[test]
fn test_operand_display() {
    assert_eq!(format!("{}", Operand::reg(Register::RAX)), "rax");
    assert_eq!(format!("{}", Operand::imm(42)), "42");
    assert_eq!(format!("{}", Operand::label("test_label")), "test_label");
    assert_eq!(format!("{}", Operand::mem("rax")), "[rax]");
}

#[test]
fn test_instruction_display() {
    assert_eq!(
        format!("{}", Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(42))),
        "    mov rax, 42"
    );
    
    assert_eq!(
        format!("{}", Instruction::Add(Operand::reg(Register::RAX), Operand::reg(Register::RBX))),
        "    add rax, rbx"
    );
    
    assert_eq!(
        format!("{}", Instruction::Jmp("label".to_string())),
        "    jmp label"
    );
}

#[test]
fn test_generator_label_generation() {
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    let label1 = generator.generate_label("test");
    let label2 = generator.generate_label("test");
    
    assert_eq!(label1, "test_1");
    assert_eq!(label2, "test_2");
}

#[test]
fn test_hello_world_generation() {
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_hello_world_linux();
    let code = generator.generate();
    
    // Check that the code contains essential elements
    assert!(code.contains("section .text"));
    assert!(code.contains("section .data"));
    assert!(code.contains("global _start"));
    assert!(code.contains("sys_write"));
    assert!(code.contains("sys_exit"));
    assert!(code.contains("syscall"));
}

#[test]
fn test_single_text_section() {
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add the text section multiple times
    generator.add_section(Section::Text);
    generator.add_section(Section::Text);
    generator.add_section(Section::Text);
    
    // Add some instructions to the text section
    generator.add_instruction(Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(42)));
    
    let code = generator.generate();
    
    // Count occurrences of "section .text"
    let text_section_count = code.matches("section .text").count();
    
    // Should only have one occurrence of "section .text"
    assert_eq!(text_section_count, 1);
    
    // Check that the instruction is present
    assert!(code.contains("    mov rax, 42"));
}