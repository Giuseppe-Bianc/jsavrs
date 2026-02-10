use jsavrs::asm::*;

#[test]
fn test_assembly_file_creation_system_v() {
    let abi = Abi::from_platform(Platform::Linux);
    let asm_file = AssemblyFile::new(abi);

    assert_eq!(asm_file.abi().kind, AbiKind::SystemV);
    assert!(asm_file.bss_section().is_some());
    assert!(asm_file.rodata_section().is_none());
}

#[test]
fn test_assembly_file_creation_windows() {
    let abi = Abi::from_platform(Platform::Windows);
    let asm_file = AssemblyFile::new(abi);

    assert_eq!(asm_file.abi().kind, AbiKind::Windows);
    assert!(asm_file.bss_section().is_none());
    assert!(asm_file.rodata_section().is_some());
}

#[test]
fn test_assembly_file_data_section_add_data() {
    let abi = Abi::from_platform(Platform::Linux);
    let mut asm_file = AssemblyFile::new(abi);

    let data_directive = DataDirective::new_asciz("Hello, world!");
    asm_file.data_sec_add_data("msg", data_directive);

    // Check that the data was added to the data section
    assert!(!asm_file.data_section().elements.is_empty());
    if let AssemblyElement::Data(label, _) = &asm_file.data_section().elements[0] {
        assert_eq!(label, "msg");
    } else {
        panic!("Expected Data element");
    }
}

#[test]
fn test_assembly_file_text_section_add_instruction() {
    let abi = Abi::from_platform(Platform::Linux);
    let mut asm_file = AssemblyFile::new(abi);

    let instruction = Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm32(42) };
    asm_file.text_sec_add_instruction(instruction);

    // Check that the instruction was added to the text section
    assert!(!asm_file.text_section().elements.is_empty());
    if let AssemblyElement::Instruction(_) = &asm_file.text_section().elements[0] {
        // Expected
    } else {
        panic!("Expected Instruction element");
    }
}

#[test]
fn test_assembly_file_text_section_add_label() {
    let abi = Abi::from_platform(Platform::Linux);
    let mut asm_file = AssemblyFile::new(abi);

    asm_file.text_sec_add_label("start");

    // Check that the label was added to the text section
    assert!(!asm_file.text_section().elements.is_empty());
    if let AssemblyElement::Label(label) = &asm_file.text_section().elements[0] {
        assert_eq!(label, "start");
    } else {
        panic!("Expected Label element");
    }
}

#[test]
fn test_assembly_file_text_section_add_comment() {
    let abi = Abi::from_platform(Platform::Linux);
    let mut asm_file = AssemblyFile::new(abi);

    asm_file.text_sec_add_comment("This is a test comment");

    // Check that the comment was added to the text section
    assert!(!asm_file.text_section().elements.is_empty());
    if let AssemblyElement::Comment(comment) = &asm_file.text_section().elements[0] {
        assert_eq!(comment, "This is a test comment");
    } else {
        panic!("Expected Comment element");
    }
}

#[test]
fn test_assembly_file_text_section_add_instruction_with_comment() {
    let abi = Abi::from_platform(Platform::Linux);
    let mut asm_file = AssemblyFile::new(abi);

    let instruction = Instruction::Nop;
    asm_file.text_sec_add_instruction_with_comment(instruction, "No operation");

    // Check that the instruction with comment was added to the text section
    assert!(!asm_file.text_section().elements.is_empty());
    if let AssemblyElement::InstructionWithComment(_, comment) = &asm_file.text_section().elements[0] {
        assert_eq!(comment, "No operation");
    } else {
        panic!("Expected InstructionWithComment element");
    }
}

#[test]
fn test_assembly_file_display() {
    let abi = Abi::from_platform(Platform::Linux);
    let asm_file = AssemblyFile::new(abi);

    let display_str = format!("{asm_file}");
    assert!(display_str.contains("Assembly File"));
    assert!(display_str.contains("System V AMD64 ABI"));
}

#[test]
fn test_assembly_file_clone() {
    let abi = Abi::from_platform(Platform::Linux);
    let asm_file = AssemblyFile::new(abi);
    let cloned_file = asm_file.clone();

    // Compare structural fields instead of Display output, because Display
    // calls Utc::now() on each invocation, producing different timestamps.
    assert_eq!(format!("{asm_file:?}"), format!("{cloned_file:?}"));

    // Also verify that both Display outputs are structurally identical
    // except for the timestamp line.
    let original_display = format!("{asm_file}");
    let cloned_display = format!("{cloned_file}");
    let original_lines: Vec<&str> =
        original_display.lines().filter(|l| !l.starts_with("; Generated on:")).map(str::trim).collect();
    let cloned_lines: Vec<&str> =
        cloned_display.lines().filter(|l| !l.starts_with("; Generated on:")).map(str::trim).collect();
    assert_eq!(original_lines, cloned_lines);
}

#[test]
fn test_assembly_file_debug() {
    let abi = Abi::from_platform(Platform::Linux);
    let asm_file = AssemblyFile::new(abi);

    let debug_str = format!("{asm_file:?}");
    assert!(debug_str.contains("AssemblyFile"));
}

#[test]
fn test_assembly_file_with_empty_elements() {
    let abi = Abi::from_platform(Platform::Linux);
    let asm_file = AssemblyFile::new(abi);

    // Test display for file with no elements in sections
    let display_str = format!("{asm_file}");
    assert!(!display_str.is_empty());
}

#[test]
fn test_assembly_file_multiple_operations() {
    let abi = Abi::from_platform(Platform::Linux);
    let mut asm_file = AssemblyFile::new(abi);

    // Add multiple data elements
    asm_file.data_sec_add_data("data1", DataDirective::new_asciz("test1"));
    asm_file.data_sec_add_data("data2", DataDirective::new_asciz("test2"));
    asm_file.data_sec_add_data("data3", DataDirective::Dw(vec![10, 20, 30]));

    // Add text elements
    asm_file.text_sec_add_label("start");
    asm_file.text_sec_add_instruction(Instruction::Nop);
    asm_file.text_sec_add_instruction_with_comment(Instruction::Ret, "return from function");
    asm_file.text_sec_add_comment("end of function");

    assert_eq!(asm_file.data_section().elements.len(), 3);
    assert_eq!(asm_file.text_section().elements.len(), 4);
}

#[test]
fn test_assembly_file_with_different_abi_kinds() {
    let linux_abi = Abi::from_platform(Platform::Linux);
    let windows_abi = Abi::from_platform(Platform::Windows);

    let linux_file = AssemblyFile::new(linux_abi);
    let windows_file = AssemblyFile::new(windows_abi);

    // Linux should have BSS but not rodata
    assert!(linux_file.bss_section().is_some());
    assert!(linux_file.rodata_section().is_none());

    // Windows should have rodata but not BSS
    assert!(windows_file.rodata_section().is_some());
    assert!(windows_file.bss_section().is_none());
}
