use jsavrs::asm::operand::Operand;
use jsavrs::asm::register::Register;

#[test]
fn test_operand_type_variants() {
    // Test all operand type variants
    let reg_op = Operand::reg(Register::RAX);
    assert!(reg_op.is_register());
    
    let imm_op = Operand::imm(42);
    assert!(imm_op.is_immediate());
    
    let label_op = Operand::label("test_label");
    assert!(label_op.is_label());
    
    let mem_op = Operand::mem("rax+8");
    assert!(mem_op.is_memory());
    
    let mem_ref_op = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 4, 16);
    assert!(mem_ref_op.is_memory());
}

#[test]
fn test_operand_display() {
    // Test operand display implementations
    assert_eq!(format!("{}", Operand::reg(Register::RAX)), "rax");
    assert_eq!(format!("{}", Operand::imm(42)), "42");
    assert_eq!(format!("{}", Operand::label("loop_start")), "loop_start");
    assert_eq!(format!("{}", Operand::mem("rax+8")), "[rax+8]");
    
    // Test memory reference display
    let mem_ref = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 4, 16);
    assert_eq!(format!("{}", mem_ref), "[rax+rbx*4+16]");
    
    let rip_relative = Operand::rip_relative(32);
    assert_eq!(format!("{}", rip_relative), "[32]");
}

#[test]
fn test_immediate_operand_boundary_values() {
    // Test boundary value tests for immediate operands
    assert_eq!(format!("{}", Operand::imm(i64::MIN)), format!("{}", i64::MIN));
    assert_eq!(format!("{}", Operand::imm(i64::MAX)), format!("{}", i64::MAX));
    assert_eq!(format!("{}", Operand::imm(0)), "0");
    assert_eq!(format!("{}", Operand::imm(1)), "1");
    assert_eq!(format!("{}", Operand::imm(-1)), "-1");
}

#[test]
fn test_complex_addressing_modes() {
    // Test complex addressing modes including RIP-relative addressing
    let mem_ref = Operand::mem_base_index_scale_disp(Register::RAX, Register::RBX, 2, 8);
    assert_eq!(format!("{}", mem_ref), "[rax+rbx*2+8]");
    
    let rip_rel = Operand::rip_relative(-4);
    assert_eq!(format!("{}", rip_rel), "[-4]");
    
    let base_only = Operand::mem_base(Register::RCX);
    assert_eq!(format!("{}", base_only), "[rcx]");
    
    let base_disp = Operand::mem_base_disp(Register::RDX, 32);
    assert_eq!(format!("{}", base_disp), "[rdx+32]");
    
    let base_index = Operand::mem_base_index(Register::RSI, Register::RDI);
    assert_eq!(format!("{}", base_index), "[rsi+rdi]");
    
    let base_index_scale = Operand::mem_base_index_scale(Register::R8, Register::R9, 8);
    assert_eq!(format!("{}", base_index_scale), "[r8+r9*8]");
}

#[test]
fn test_operand_formatting_edge_cases() {
    // Test operand formatting with edge cases
    let mem_ref_zero_disp = Operand::mem_ref(Some(Register::RAX), None, 1, 0);
    assert_eq!(format!("{}", mem_ref_zero_disp), "[rax]");
    
    let mem_ref_no_base = Operand::mem_ref(None, Some(Register::RBX), 2, 0);
    assert_eq!(format!("{}", mem_ref_no_base), "[rbx*2]");
    
    let mem_ref_neg_disp = Operand::mem_base_disp(Register::RSP, -8);
    assert_eq!(format!("{}", mem_ref_neg_disp), "[rsp-8]");
}

#[test]
fn test_operand_utility_methods() {
    // Test operand utility methods
    let reg_op = Operand::reg(Register::RBX);
    assert!(reg_op.is_register());
    assert!(!reg_op.is_immediate());
    assert!(!reg_op.is_label());
    assert!(!reg_op.is_memory());
    
    let imm_op = Operand::imm(100);
    assert!(imm_op.is_immediate());
    assert_eq!(imm_op.as_immediate(), Some(100));
    
    let label_op = Operand::label("start");
    assert!(label_op.is_label());
    assert_eq!(label_op.as_label(), Some("start"));
    
    let mem_op = Operand::mem("rsp+8");
    assert!(mem_op.is_memory());
    assert_eq!(mem_op.as_memory(), Some("rsp+8"));
    
    let mem_ref_op = Operand::mem_base_index(Register::RAX, Register::RBX);
    assert!(mem_ref_op.is_memory());
    if let Some((base, index, scale, disp)) = mem_ref_op.as_memory_ref() {
        assert_eq!(base, &Some(Register::RAX));
        assert_eq!(index, &Some(Register::RBX));
        assert_eq!(scale, &1);
        assert_eq!(disp, &0);
    }
}

#[test]
fn test_operand_constructor_functions() {
    // Test operand constructor functions
    let reg_op = Operand::reg(Register::RCX);
    assert_eq!(reg_op, Operand::Register(Register::RCX));
    
    let imm_op = Operand::imm(50);
    assert_eq!(imm_op, Operand::Immediate(50));
    
    let label_op = Operand::label("test");
    assert_eq!(label_op, Operand::Label("test".to_string()));
    
    let mem_op = Operand::mem("rax+16");
    assert_eq!(mem_op, Operand::Memory("rax+16".to_string()));
    
    let mem_ref_op = Operand::mem_base(Register::RSP);
    let expected = Operand::mem_ref(Some(Register::RSP), None, 1, 0);
    assert_eq!(mem_ref_op, expected);
}

#[test]
fn test_operand_as_register() {
    // Test as_register function for register operands
    let reg_op = Operand::reg(Register::RAX);
    assert_eq!(reg_op.as_register(), Some(&Register::RAX));
    
    let rbx_op = Operand::reg(Register::RBX);
    assert_eq!(rbx_op.as_register(), Some(&Register::RBX));
    
    // Test as_register function for non-register operands (should return None)
    let imm_op = Operand::imm(42);
    assert_eq!(imm_op.as_register(), None);
    
    let label_op = Operand::label("test_label");
    assert_eq!(label_op.as_register(), None);
    
    let mem_op = Operand::mem("rax+8");
    assert_eq!(mem_op.as_register(), None);
    
    let mem_ref_op = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 2, 8);
    assert_eq!(mem_ref_op.as_register(), None);
}

#[test]
fn test_operand_as_immediate() {
    // Test as_immediate function for immediate operands
    let imm_op = Operand::imm(42);
    assert_eq!(imm_op.as_immediate(), Some(42));
    
    let neg_imm_op = Operand::imm(-100);
    assert_eq!(neg_imm_op.as_immediate(), Some(-100));
    
    // Test as_immediate function for non-immediate operands (should return None)
    let reg_op = Operand::reg(Register::RAX);
    assert_eq!(reg_op.as_immediate(), None);
    
    let label_op = Operand::label("test_label");
    assert_eq!(label_op.as_immediate(), None);
    
    let mem_op = Operand::mem("rax+8");
    assert_eq!(mem_op.as_immediate(), None);
    
    let mem_ref_op = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 2, 8);
    assert_eq!(mem_ref_op.as_immediate(), None);
}

#[test]
fn test_operand_as_label() {
    // Test as_label function for label operands
    let label_op = Operand::label("test_label");
    assert_eq!(label_op.as_label(), Some("test_label"));
    
    let start_label_op = Operand::label("start");
    assert_eq!(start_label_op.as_label(), Some("start"));
    
    // Test as_label function for non-label operands (should return None)
    let reg_op = Operand::reg(Register::RAX);
    assert_eq!(reg_op.as_label(), None);
    
    let imm_op = Operand::imm(42);
    assert_eq!(imm_op.as_label(), None);
    
    let mem_op = Operand::mem("rax+8");
    assert_eq!(mem_op.as_label(), None);
    
    let mem_ref_op = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 2, 8);
    assert_eq!(mem_ref_op.as_label(), None);
}

#[test]
fn test_operand_as_memory() {
    // Test as_memory function for memory operands
    let mem_op = Operand::mem("rax+8");
    assert_eq!(mem_op.as_memory(), Some("rax+8"));
    
    let mem_addr_op = Operand::mem("rsp-4");
    assert_eq!(mem_addr_op.as_memory(), Some("rsp-4"));
    
    // Test as_memory function for non-memory operands (should return None)
    let reg_op = Operand::reg(Register::RAX);
    assert_eq!(reg_op.as_memory(), None);
    
    let imm_op = Operand::imm(42);
    assert_eq!(imm_op.as_memory(), None);
    
    let label_op = Operand::label("test_label");
    assert_eq!(label_op.as_memory(), None);
    
    // Note: MemoryRef operands are different from Memory operands
    let mem_ref_op = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 2, 8);
    assert_eq!(mem_ref_op.as_memory(), None);
}

#[test]
fn test_operand_as_memory_ref() {
    // Test as_memory_ref function for memory reference operands
    let mem_ref_op = Operand::mem_ref(Some(Register::RAX), Some(Register::RBX), 2, 8);
    if let Some((base, index, scale, disp)) = mem_ref_op.as_memory_ref() {
        assert_eq!(base, &Some(Register::RAX));
        assert_eq!(index, &Some(Register::RBX));
        assert_eq!(scale, &2);
        assert_eq!(disp, &8);
    } else {
        panic!("Expected Some for memory reference operand");
    }
    
    // Test as_memory_ref function for non-memory reference operands (should return None)
    let reg_op = Operand::reg(Register::RAX);
    assert_eq!(reg_op.as_memory_ref(), None);
    
    let imm_op = Operand::imm(42);
    assert_eq!(imm_op.as_memory_ref(), None);
    
    let label_op = Operand::label("test_label");
    assert_eq!(label_op.as_memory_ref(), None);
    
    let mem_op = Operand::mem("rax+8");
    assert_eq!(mem_op.as_memory_ref(), None);
}