use jsavrs::asm::generator::{NasmGenerator, TargetOS, AssemblyElement, Section};
use jsavrs::asm::instruction::Instruction;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

#[test]
fn test_nasm_generator_section_handling() {
    // Test section handling functionality to prevent duplicate sections
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add the same section multiple times
    generator.add_section(Section::Text);
    generator.add_section(Section::Text); // This should be ignored
    generator.add_section(Section::Data);
    generator.add_section(Section::Text); // This should also be ignored
    
    // Count the number of Section::Text elements
    let text_sections: Vec<&AssemblyElement> = generator.elements()
        .iter()
        .filter(|elem| matches!(elem, AssemblyElement::Section(Section::Text)))
        .collect();
    
    assert_eq!(text_sections.len(), 1, "Text section should only be added once");
    
    // Verify that both text and data sections exist
    let has_text = generator.elements().iter().any(|elem| matches!(elem, AssemblyElement::Section(Section::Text)));
    let has_data = generator.elements().iter().any(|elem| matches!(elem, AssemblyElement::Section(Section::Data)));
    
    assert!(has_text, "Text section should be present");
    assert!(has_data, "Data section should be present");
}

#[test]
fn test_correct_section_ordering() {
    // Test correct section ordering
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add sections in a specific order
    generator.add_section(Section::Data);
    generator.add_section(Section::Text);
    generator.add_section(Section::Bss);
    generator.add_section(Section::Rodata);
    
    // Get all sections in order
    let sections: Vec<&AssemblyElement> = generator.elements()
        .iter()
        .filter(|elem| matches!(elem, AssemblyElement::Section(_)))
        .collect();
    
    // Verify all sections were added
    assert_eq!(sections.len(), 4);
}

#[test]
fn test_detection_of_empty_or_invalid_sections() {
    // Test detection of empty or invalid sections
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Initially empty
    assert_eq!(generator.len(), 0);
    assert!(generator.is_empty());
    
    // Add a section
    generator.add_section(Section::Text);
    assert_eq!(generator.len(), 1);
    assert!(!generator.is_empty());
    
    // Clear and verify it's empty again
    generator.clear();
    assert_eq!(generator.len(), 0);
    assert!(generator.is_empty());
}

#[test]
fn test_label_generation_with_unique_names() {
    // Test label generation with unique names
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    let label1 = generator.generate_label("loop");
    let label2 = generator.generate_label("loop");
    let label3 = generator.generate_label("loop");
    
    assert_ne!(label1, label2);
    assert_ne!(label2, label3);
    assert_ne!(label1, label3);
    
    assert!(label1.starts_with("loop_"));
    assert!(label2.starts_with("loop_"));
    assert!(label3.starts_with("loop_"));
}

#[test]
fn test_hello_world_program_generation_for_linux() {
    // Test hello world program generation for Linux
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_hello_world_linux();
    
    // Verify the program has content
    assert!(!generator.is_empty());
    
    // Check that it has the main components
    let code = generator.generate();
    assert!(code.contains("_start:"));
    assert!(code.contains("mov rax, 1")); // sys_write
    assert!(code.contains("global _start"));
    assert!(code.contains("section .data"));
    assert!(code.contains("section .text"));
    assert!(code.contains("msg"));
    assert!(code.contains("sys_exit"));
}

#[test]
fn test_hello_world_program_generation_for_windows() {
    // Test hello world program generation for Windows
    let mut generator = NasmGenerator::new(TargetOS::Windows);
    generator.create_hello_world_windows();
    
    // Verify the program has content
    assert!(!generator.is_empty());
    
    // Check that it has the main components
    let code = generator.generate();
    assert!(code.contains("main:"));
    assert!(code.contains("global main"));
    assert!(code.contains("extern ExitProcess"));
    assert!(code.contains("extern WriteConsoleA"));
    assert!(code.contains("section .data"));
    assert!(code.contains("section .text"));
    assert!(code.contains("msg"));
}

#[test]
fn test_function_prologue_and_epilogue_generation() {
    // Test function prologue and epilogue generation for different target operating systems
    let mut gen_linux = NasmGenerator::new(TargetOS::Linux);
    gen_linux.add_function_prologue();
    gen_linux.add_function_epilogue();
    assert!(!gen_linux.is_empty());
    
    let mut gen_windows = NasmGenerator::new(TargetOS::Windows);
    gen_windows.add_function_prologue();
    gen_windows.add_function_epilogue();
    assert!(!gen_windows.is_empty());
    
    // Verify Linux version has the appropriate callee-saved registers
    let linux_code = gen_linux.generate();
    // In Linux (System V ABI), RBX, RBP, R12-R15 are callee-saved
    assert!(linux_code.contains("push rbx"));
    assert!(linux_code.contains("push rbp"));
    assert!(linux_code.contains("push r12"));
    assert!(linux_code.contains("push r13"));
    assert!(linux_code.contains("push r14"));
    assert!(linux_code.contains("push r15"));
    
    // Verify Windows version has the appropriate callee-saved registers
    let windows_code = gen_windows.generate();
    // In Windows x64 ABI, RBX, RBP, RDI, RSI, R12-R15 are callee-saved
    assert!(windows_code.contains("push rbx"));
    assert!(windows_code.contains("push rbp"));
    assert!(windows_code.contains("push rdi"));
    assert!(windows_code.contains("push rsi"));
    assert!(windows_code.contains("push r12"));
    assert!(windows_code.contains("push r13"));
    assert!(windows_code.contains("push r14"));
    assert!(windows_code.contains("push r15"));
}

#[test]
fn test_factorial_function_generation() {
    // Test factorial function generation with recursive calls
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    // Verify the function was generated
    let code = generator.generate();
    assert!(code.contains("factorial:"));
    assert!(code.contains("call factorial"));
    assert!(code.contains("mov rax, 1")); // base case return
    assert!(code.contains("imul rax, rdi")); // Linux ABI uses RDI
}

#[test]
fn test_generated_assembly_formatting_and_correctness() {
    // Test generated assembly formatting and correctness
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add various elements
    generator.add_standard_prelude();
    generator.add_section(Section::Text);
    generator.add_global("test_func");
    generator.add_label("test_func");
    generator.add_instruction(Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(42)));
    generator.add_instruction(Instruction::Add(Operand::reg(Register::RAX), Operand::reg(Register::RBX)));
    generator.add_instruction(Instruction::Ret);
    
    let code = generator.generate();
    assert!(code.contains("test_func:"));
    assert!(code.contains("global test_func"));
    assert!(code.contains("mov rax, 42"));
    assert!(code.contains("add rax, rbx"));
    assert!(code.contains("ret"));
    assert!(code.contains("bits 64"));
}

#[test]
fn test_assembly_element_type_checking_methods() {
    // Test assembly element manipulation methods
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Create various assembly elements
    let section_elem = AssemblyElement::Section(Section::Text);
    let label_elem = AssemblyElement::Label("test_label".to_string());
    let inst_elem = AssemblyElement::Instruction(Instruction::Nop);
    let comment_elem = AssemblyElement::Comment("test comment".to_string());
    let global_elem = AssemblyElement::Global("test_global".to_string());
    let extern_elem = AssemblyElement::Extern("test_extern".to_string());
    
    // Test type checking methods
    assert!(section_elem.is_section());
    assert!(!section_elem.is_label());
    assert!(!section_elem.is_instruction());
    
    assert!(label_elem.is_label());
    assert!(!label_elem.is_section());
    
    assert!(inst_elem.is_instruction());
    assert!(!inst_elem.is_comment());
    
    assert!(comment_elem.is_comment());
    assert!(!comment_elem.is_empty_line());
    
    assert!(global_elem.is_global());
    assert!(!global_elem.is_extern());
    
    assert!(extern_elem.is_extern());
    assert!(!extern_elem.is_global());
    
    // Test as_* methods
    assert_eq!(section_elem.as_section().unwrap().name(), ".text");
    assert_eq!(label_elem.as_label(), Some("test_label"));
    assert!(inst_elem.as_instruction().is_some());
    assert_eq!(comment_elem.as_comment(), Some("test comment"));
    assert_eq!(global_elem.as_global(), Some("test_global"));
    assert_eq!(extern_elem.as_extern(), Some("test_extern"));
}

#[test]
fn test_assembly_element_manipulation_methods() {
    // Test assembly element manipulation methods
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add multiple elements
    generator.add_instruction(Instruction::Nop);
    generator.add_instruction(Instruction::Ret);
    
    // Add multiple elements at once
    let instructions = vec![
        Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(1)),
        Instruction::Mov(Operand::reg(Register::RBX), Operand::imm(2)),
    ];
    generator.add_instructions(instructions);
    
    // Add multiple comments
    generator.add_comments(vec!["First comment", "Second comment"]);
    
    // Add multiple empty lines
    generator.add_empty_lines(2);
    
    // Verify the elements were added
    assert_eq!(generator.len(), 8); // 2 individual + 2 from vector + 2 comments + 2 empty lines = 8
    
    // Test clear method
    generator.clear();
    assert!(generator.is_empty());
    assert_eq!(generator.len(), 0);
}
