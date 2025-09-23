use jsavrs::asm::register::Register;

#[test]
fn test_register_display_8bit() {
    assert_eq!(format!("{}", Register::AL), "al");
    assert_eq!(format!("{}", Register::BL), "bl");
}

#[test]
fn test_register_display_16bit() {
    assert_eq!(format!("{}", Register::AX), "ax");
    assert_eq!(format!("{}", Register::BX), "bx");
}

#[test]
fn test_register_display_32bit() {
    assert_eq!(format!("{}", Register::EAX), "eax");
    assert_eq!(format!("{}", Register::EBX), "ebx");
}

#[test]
fn test_register_display_64bit() {
    assert_eq!(format!("{}", Register::RAX), "rax");
    assert_eq!(format!("{}", Register::RBX), "rbx");
}

#[test]
fn test_register_creation_with_boundary_conditions() {
    // Test register properties with boundary conditions
    let rax = Register::RAX;
    assert_eq!(rax.size(), 64);
    
    let eax = Register::EAX;
    assert_eq!(eax.size(), 32);
    
    let ax = Register::AX;
    assert_eq!(ax.size(), 16);
    
    let al = Register::AL;
    assert_eq!(al.size(), 8);
}

#[test]
fn test_register_formatting_scenarios() {
    // Test register formatting with various scenarios
    assert_eq!(format!("{}", Register::RAX), "rax");
    assert_eq!(format!("{}", Register::RBX), "rbx");
    assert_eq!(format!("{}", Register::RCX), "rcx");
    assert_eq!(format!("{}", Register::RDX), "rdx");
}

#[test]
fn test_register_conversion_functions() {
    // Test register conversion functions
    assert_eq!(Register::AL.to_64bit(), Register::RAX);
    assert_eq!(Register::AX.to_64bit(), Register::RAX);
    assert_eq!(Register::EAX.to_64bit(), Register::RAX);
    
    assert_eq!(Register::RAX.to_32bit(), Register::EAX);
    assert_eq!(Register::RAX.to_16bit(), Register::AX);
    assert_eq!(Register::RAX.to_8bit(), Register::AL);
}

#[test]
fn test_architecture_specific_validation() {
    // Test architecture-specific validation for different x86-64 implementations
    
    // Test 64-bit registers
    assert_eq!(Register::RAX.size(), 64);
    assert_eq!(Register::RBX.size(), 64);
    assert_eq!(Register::RCX.size(), 64);
    assert_eq!(Register::RDX.size(), 64);
    
    // Test 32-bit registers
    assert_eq!(Register::EAX.size(), 32);
    assert_eq!(Register::EBX.size(), 32);
    assert_eq!(Register::ECX.size(), 32);
    assert_eq!(Register::EDX.size(), 32);
    
    // Test 16-bit registers
    assert_eq!(Register::AX.size(), 16);
    assert_eq!(Register::BX.size(), 16);
    assert_eq!(Register::CX.size(), 16);
    assert_eq!(Register::DX.size(), 16);
    
    // Test 8-bit registers
    assert_eq!(Register::AL.size(), 8);
    assert_eq!(Register::BL.size(), 8);
    assert_eq!(Register::CL.size(), 8);
    assert_eq!(Register::DL.size(), 8);
}

#[test]
fn test_register_boundary_value_testing() {
    // Test boundary value testing for all register sizes
    assert_eq!(Register::AL.size(), 8);
    assert_eq!(Register::AX.size(), 16);
    assert_eq!(Register::EAX.size(), 32);
    assert_eq!(Register::RAX.size(), 64);
}

#[test]
fn test_abi_specific_register_classification() {
    // Test ABI-specific register classification
    
    // Windows ABI parameter registers
    assert!(Register::RCX.is_windows_param_register());
    assert!(Register::RDX.is_windows_param_register());
    assert!(Register::R8.is_windows_param_register());
    assert!(Register::R9.is_windows_param_register());
    
    // System V ABI parameter registers
    assert!(Register::RDI.is_systemv_param_register());
    assert!(Register::RSI.is_systemv_param_register());
    assert!(Register::RDX.is_systemv_param_register());
    assert!(Register::RCX.is_systemv_param_register());
    assert!(Register::R8.is_systemv_param_register());
    assert!(Register::R9.is_systemv_param_register());
    
    // Test parameter register retrieval
    assert_eq!(Register::windows_param_register(0), Some(Register::RCX));
    assert_eq!(Register::windows_param_register(1), Some(Register::RDX));
    assert_eq!(Register::windows_param_register(2), Some(Register::R8));
    assert_eq!(Register::windows_param_register(3), Some(Register::R9));
    assert_eq!(Register::windows_param_register(4), None); // Out of bounds
    
    assert_eq!(Register::systemv_param_register(0), Some(Register::RDI));
    assert_eq!(Register::systemv_param_register(1), Some(Register::RSI));
    assert_eq!(Register::systemv_param_register(2), Some(Register::RDX));
    assert_eq!(Register::systemv_param_register(3), Some(Register::RCX));
    assert_eq!(Register::systemv_param_register(4), Some(Register::R8));
    assert_eq!(Register::systemv_param_register(5), Some(Register::R9));
    assert_eq!(Register::systemv_param_register(6), None); // Out of bounds
}