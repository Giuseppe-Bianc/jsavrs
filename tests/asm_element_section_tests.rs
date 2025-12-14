use jsavrs::asm::*;

#[test]
fn test_assembly_element_label() {
    let element = AssemblyElement::Label("start".to_string());

    if let AssemblyElement::Label(ref name) = element {
        assert_eq!(name, "start");
    } else {
        panic!("Expected Label variant");
    }

    assert_eq!(format!("{element}"), "start:");
}

#[test]
fn test_assembly_element_instruction() {
    let instr = Instruction::Nop;
    let element = AssemblyElement::Instruction(instr);

    if let AssemblyElement::Instruction(ref i) = element {
        assert!(matches!(i, Instruction::Nop));
    } else {
        panic!("Expected Instruction variant");
    }

    assert_eq!(format!("{element}"), "    nop");
}

#[test]
fn test_assembly_element_instruction_with_comment() {
    let instr = Instruction::Ret;
    let element = AssemblyElement::InstructionWithComment(instr, "return from function".to_string());

    if let AssemblyElement::InstructionWithComment(ref i, ref c) = element {
        assert!(matches!(i, Instruction::Ret));
        assert_eq!(c, "return from function");
    } else {
        panic!("Expected InstructionWithComment variant");
    }

    assert_eq!(format!("{element}"), "    ret    ; return from function");
}

#[test]
fn test_assembly_element_data() {
    let directive = DataDirective::new_asciz("Hello");
    let element = AssemblyElement::Data("msg".to_string(), directive);

    if let AssemblyElement::Data(ref label, _) = element {
        assert_eq!(label, "msg");
    } else {
        panic!("Expected Data variant");
    }

    assert_eq!(format!("{element}"), "msg db \"Hello\", 0");
}

#[test]
fn test_assembly_element_comment() {
    let element = AssemblyElement::Comment("This is a test".to_string());

    if let AssemblyElement::Comment(ref text) = element {
        assert_eq!(text, "This is a test");
    } else {
        panic!("Expected Comment variant");
    }

    assert_eq!(format!("{element}"), "; This is a test");
}

#[test]
fn test_assembly_element_empty_line() {
    let element = AssemblyElement::EmptyLine;

    if matches!(element, AssemblyElement::EmptyLine) {
        // Expected
    } else {
        panic!("Expected EmptyLine variant");
    }

    assert_eq!(format!("{element}"), "");
}

#[test]
fn test_assembly_element_debug() {
    let element = AssemblyElement::Label("start".to_string());
    let debug_str = format!("{element:?}");
    assert!(debug_str.contains("Label"));
    assert!(debug_str.contains("start"));
}

#[test]
fn test_assembly_section_creation() {
    let section = AssemblySection::new(Section::Text);
    assert_eq!(section.section, Section::Text);
    assert!(section.elements.is_empty());

    let text_section = AssemblySection::text_section();
    assert!(text_section.section.is_text());

    let data_section = AssemblySection::data_section();
    assert!(data_section.section.is_data());

    let bss_section = AssemblySection::bss_section();
    assert!(bss_section.section.is_bss());

    let rodata_section = AssemblySection::rodata_section();
    assert!(rodata_section.section.is_rodata());
}

#[test]
fn test_assembly_section_add_elements() {
    let mut section = AssemblySection::text_section();

    // Add a label
    section.add_label("start");
    assert_eq!(section.elements.len(), 1);
    if let AssemblyElement::Label(name) = &section.elements[0] {
        assert_eq!(name, "start");
    } else {
        panic!("Expected Label");
    }

    // Add an instruction
    section.add_instruction(Instruction::Nop);
    assert_eq!(section.elements.len(), 2);
    if let AssemblyElement::Instruction(_) = &section.elements[1] {
        // Expected
    } else {
        panic!("Expected Instruction");
    }

    // Add a data element
    section.add_data("msg", DataDirective::new_asciz("Hello"));
    assert_eq!(section.elements.len(), 3);
    if let AssemblyElement::Data(label, _) = &section.elements[2] {
        assert_eq!(label, "msg");
    } else {
        panic!("Expected Data");
    }

    // Add a comment
    section.add_comment("End of function");
    assert_eq!(section.elements.len(), 4);
    if let AssemblyElement::Comment(text) = &section.elements[3] {
        assert_eq!(text, "End of function");
    } else {
        panic!("Expected Comment");
    }

    // Add instruction with comment
    section.add_instruction_with_comment(Instruction::Ret, "return");
    assert_eq!(section.elements.len(), 5);
    if let AssemblyElement::InstructionWithComment(_, comment) = &section.elements[4] {
        assert_eq!(comment, "return");
    } else {
        panic!("Expected InstructionWithComment");
    }

    // Add empty line
    section.add_empty_line();
    assert_eq!(section.elements.len(), 6);
    if matches!(&section.elements[5], AssemblyElement::EmptyLine) {
        // Expected
    } else {
        panic!("Expected EmptyLine");
    }
}

#[test]
fn test_assembly_section_display() {
    let mut section = AssemblySection::data_section();
    section.add_data("msg", DataDirective::new_asciz("Hello, World!"));
    section.add_data("size", DataDirective::new_equ_length_of("msg"));

    let display = format!("{section}");
    assert!(display.starts_with("section .data"));
    assert!(display.contains("msg db \"Hello, World!\", 0"));
    assert!(display.contains("size equ $ - msg"));
}

#[test]
fn test_assembly_section_clone_preserves_content() {
    let mut original = AssemblySection::data_section();
    original.add_data("str", DataDirective::new_asciz("test"));
    original.add_comment("a comment");
    original.add_empty_line();
    original.add_label("label");

    let cloned = original.clone();

    assert_eq!(original.elements.len(), cloned.elements.len());
    assert_eq!(format!("{original}"), format!("{cloned}"));
}

#[test]
fn test_assembly_section_debug() {
    let section = AssemblySection::data_section();
    let debug_str = format!("{section:?}");
    assert!(debug_str.contains("AssemblySection"));
    assert!(debug_str.contains("Data"));
}

#[test]
fn test_assembly_section_multiple_elements_display() {
    let mut section = AssemblySection::text_section();
    section.add_label("start");
    section.add_instruction(Instruction::Mov { dest: Operand::reg64(GPRegister64::Rax), src: Operand::imm64(42) });
    section.add_instruction_with_comment(Instruction::Ret, "return with value in RAX");
    section.add_comment("Function implementation");

    let display = format!("{section}");
    assert!(display.contains("start:"));
    assert!(display.contains("mov rax, 42"));
    assert!(display.contains("ret    ; return with value in RAX"));
    assert!(display.contains("; Function implementation"));
}

#[test]
fn test_assembly_element_empty_strings() {
    let empty_label = AssemblyElement::Label(String::new());
    assert_eq!(format!("{empty_label}"), ":");

    let empty_comment = AssemblyElement::Comment(String::new());
    assert_eq!(format!("{empty_comment}"), "; ");
}

#[test]
fn test_assembly_element_special_characters() {
    let label_with_special = AssemblyElement::Label("loop.123_abc".to_string());
    assert_eq!(format!("{label_with_special}"), "loop.123_abc:");

    let comment_with_special = AssemblyElement::Comment("Comment with spaces and symbols!@#".to_string());
    assert_eq!(format!("{comment_with_special}"), "; Comment with spaces and symbols!@#");
}

#[test]
fn test_assembly_section_empty() {
    let section = AssemblySection::text_section();
    let display = format!("{section}");
    assert!(display.starts_with("section .text"));
    // Should only have the section declaration and no elements
}

#[test]
fn test_escape_string_function() {
    // Test the internal escape_string function behavior through the Ascii directive
    let directive = DataDirective::Ascii("Hello\nWorld\t\"Test\"\\Backslash".to_string());
    let display = format!("{directive}");
    assert!(display.contains("Hello\\\\nWorld"));
    assert!(display.contains("\\\\t\\\"Test\\\""));
    assert!(display.contains("\\\\Backslash"));
}
