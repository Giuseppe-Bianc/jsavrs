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

// NEW TESTS FOR MISSING FUNCTIONALITY

#[test]
fn test_register_to_32bit_conversion() {
    // Test 64-bit to 32-bit conversion
    assert_eq!(Register::RAX.to_32bit(), Register::EAX);
    assert_eq!(Register::RBX.to_32bit(), Register::EBX);
    assert_eq!(Register::RCX.to_32bit(), Register::ECX);
    assert_eq!(Register::RDX.to_32bit(), Register::EDX);
    
    // Test 32-bit registers stay the same
    assert_eq!(Register::EAX.to_32bit(), Register::EAX);
    assert_eq!(Register::EBX.to_32bit(), Register::EBX);
    
    // Test 16-bit to 32-bit conversion
    assert_eq!(Register::AX.to_32bit(), Register::EAX);
    assert_eq!(Register::BX.to_32bit(), Register::EBX);
    
    // Test 8-bit to 32-bit conversion
    assert_eq!(Register::AL.to_32bit(), Register::EAX);
    assert_eq!(Register::BL.to_32bit(), Register::EBX);
}

#[test]
fn test_register_to_16bit_conversion() {
    // Test 64-bit to 16-bit conversion
    assert_eq!(Register::RAX.to_16bit(), Register::AX);
    assert_eq!(Register::RBX.to_16bit(), Register::BX);
    
    // Test 32-bit to 16-bit conversion
    assert_eq!(Register::EAX.to_16bit(), Register::AX);
    assert_eq!(Register::EBX.to_16bit(), Register::BX);
    
    // Test 16-bit registers stay the same
    assert_eq!(Register::AX.to_16bit(), Register::AX);
    assert_eq!(Register::BX.to_16bit(), Register::BX);
    
    // Test 8-bit to 16-bit conversion
    assert_eq!(Register::AL.to_16bit(), Register::AX);
    assert_eq!(Register::BL.to_16bit(), Register::BX);
}

#[test]
fn test_register_to_8bit_conversion() {
    // Test 64-bit to 8-bit conversion
    assert_eq!(Register::RAX.to_8bit(), Register::AL);
    assert_eq!(Register::RBX.to_8bit(), Register::BL);
    
    // Test 32-bit to 8-bit conversion
    assert_eq!(Register::EAX.to_8bit(), Register::AL);
    assert_eq!(Register::EBX.to_8bit(), Register::BL);
    
    // Test 16-bit to 8-bit conversion
    assert_eq!(Register::AX.to_8bit(), Register::AL);
    assert_eq!(Register::BX.to_8bit(), Register::BL);
    
    // Test 8-bit registers stay the same
    assert_eq!(Register::AL.to_8bit(), Register::AL);
    assert_eq!(Register::BL.to_8bit(), Register::BL);
}

#[test]
fn test_register_all_variants_display() {
    // Test all register variants can be displayed properly
    assert_eq!(format!("{}", Register::RAX), "rax");
    assert_eq!(format!("{}", Register::RBX), "rbx");
    assert_eq!(format!("{}", Register::RCX), "rcx");
    assert_eq!(format!("{}", Register::RDX), "rdx");
    assert_eq!(format!("{}", Register::RSI), "rsi");
    assert_eq!(format!("{}", Register::RDI), "rdi");
    assert_eq!(format!("{}", Register::RBP), "rbp");
    assert_eq!(format!("{}", Register::RSP), "rsp");
    assert_eq!(format!("{}", Register::R8), "r8");
    assert_eq!(format!("{}", Register::R9), "r9");
    assert_eq!(format!("{}", Register::R10), "r10");
    assert_eq!(format!("{}", Register::R11), "r11");
    assert_eq!(format!("{}", Register::R12), "r12");
    assert_eq!(format!("{}", Register::R13), "r13");
    assert_eq!(format!("{}", Register::R14), "r14");
    assert_eq!(format!("{}", Register::R15), "r15");
    
    assert_eq!(format!("{}", Register::EAX), "eax");
    assert_eq!(format!("{}", Register::EBX), "ebx");
    assert_eq!(format!("{}", Register::ECX), "ecx");
    assert_eq!(format!("{}", Register::EDX), "edx");
    assert_eq!(format!("{}", Register::ESI), "esi");
    assert_eq!(format!("{}", Register::EDI), "edi");
    assert_eq!(format!("{}", Register::EBP), "ebp");
    assert_eq!(format!("{}", Register::ESP), "esp");
    assert_eq!(format!("{}", Register::R8D), "r8d");
    assert_eq!(format!("{}", Register::R9D), "r9d");
    assert_eq!(format!("{}", Register::R10D), "r10d");
    assert_eq!(format!("{}", Register::R11D), "r11d");
    assert_eq!(format!("{}", Register::R12D), "r12d");
    assert_eq!(format!("{}", Register::R13D), "r13d");
    assert_eq!(format!("{}", Register::R14D), "r14d");
    assert_eq!(format!("{}", Register::R15D), "r15d");
    
    assert_eq!(format!("{}", Register::AX), "ax");
    assert_eq!(format!("{}", Register::BX), "bx");
    assert_eq!(format!("{}", Register::CX), "cx");
    assert_eq!(format!("{}", Register::DX), "dx");
    assert_eq!(format!("{}", Register::SI), "si");
    assert_eq!(format!("{}", Register::DI), "di");
    assert_eq!(format!("{}", Register::BP), "bp");
    assert_eq!(format!("{}", Register::SP), "sp");
    assert_eq!(format!("{}", Register::R8W), "r8w");
    assert_eq!(format!("{}", Register::R9W), "r9w");
    assert_eq!(format!("{}", Register::R10W), "r10w");
    assert_eq!(format!("{}", Register::R11W), "r11w");
    assert_eq!(format!("{}", Register::R12W), "r12w");
    assert_eq!(format!("{}", Register::R13W), "r13w");
    assert_eq!(format!("{}", Register::R14W), "r14w");
    assert_eq!(format!("{}", Register::R15W), "r15w");
    
    assert_eq!(format!("{}", Register::AL), "al");
    assert_eq!(format!("{}", Register::BL), "bl");
    assert_eq!(format!("{}", Register::CL), "cl");
    assert_eq!(format!("{}", Register::DL), "dl");
    assert_eq!(format!("{}", Register::SIL), "sil");
    assert_eq!(format!("{}", Register::DIL), "dil");
    assert_eq!(format!("{}", Register::BPL), "bpl");
    assert_eq!(format!("{}", Register::SPL), "spl");
    assert_eq!(format!("{}", Register::R8B), "r8b");
    assert_eq!(format!("{}", Register::R9B), "r9b");
    assert_eq!(format!("{}", Register::R10B), "r10b");
    assert_eq!(format!("{}", Register::R11B), "r11b");
    assert_eq!(format!("{}", Register::R12B), "r12b");
    assert_eq!(format!("{}", Register::R13B), "r13b");
    assert_eq!(format!("{}", Register::R14B), "r14b");
    assert_eq!(format!("{}", Register::R15B), "r15b");
}

#[test]
fn test_windows_caller_saved_registers() {
    // Test Windows ABI caller-saved registers
    assert!(Register::RAX.is_windows_caller_saved());
    assert!(Register::RCX.is_windows_caller_saved());
    assert!(Register::RDX.is_windows_caller_saved());
    assert!(Register::R8.is_windows_caller_saved());
    assert!(Register::R9.is_windows_caller_saved());
    assert!(Register::R10.is_windows_caller_saved());
    assert!(Register::R11.is_windows_caller_saved());
    
    // Test Windows ABI callee-saved registers (should not be caller-saved)
    assert!(!Register::RBX.is_windows_caller_saved());
    assert!(!Register::RBP.is_windows_caller_saved());
    assert!(!Register::RSI.is_windows_caller_saved());
    assert!(!Register::RDI.is_windows_caller_saved());
    assert!(!Register::R12.is_windows_caller_saved());
    assert!(!Register::R13.is_windows_caller_saved());
    assert!(!Register::R14.is_windows_caller_saved());
    assert!(!Register::R15.is_windows_caller_saved());
}

#[test]
fn test_windows_callee_saved_registers() {
    // Test Windows ABI callee-saved registers
    assert!(Register::RBX.is_windows_callee_saved());
    assert!(Register::RBP.is_windows_callee_saved());
    assert!(Register::RSI.is_windows_callee_saved());
    assert!(Register::RDI.is_windows_callee_saved());
    assert!(Register::R12.is_windows_callee_saved());
    assert!(Register::R13.is_windows_callee_saved());
    assert!(Register::R14.is_windows_callee_saved());
    assert!(Register::R15.is_windows_callee_saved());
    
    // Test Windows ABI caller-saved registers (should not be callee-saved)
    assert!(!Register::RAX.is_windows_callee_saved());
    assert!(!Register::RCX.is_windows_callee_saved());
    assert!(!Register::RDX.is_windows_callee_saved());
    assert!(!Register::R8.is_windows_callee_saved());
    assert!(!Register::R9.is_windows_callee_saved());
    assert!(!Register::R10.is_windows_callee_saved());
    assert!(!Register::R11.is_windows_callee_saved());
}

#[test]
fn test_systemv_caller_saved_registers() {
    // Test System V ABI caller-saved registers
    assert!(Register::RAX.is_systemv_caller_saved());
    assert!(Register::RCX.is_systemv_caller_saved());
    assert!(Register::RDX.is_systemv_caller_saved());
    assert!(Register::RSI.is_systemv_caller_saved());
    assert!(Register::RDI.is_systemv_caller_saved());
    assert!(Register::R8.is_systemv_caller_saved());
    assert!(Register::R9.is_systemv_caller_saved());
    assert!(Register::R10.is_systemv_caller_saved());
    assert!(Register::R11.is_systemv_caller_saved());
    
    // Test System V ABI callee-saved registers (should not be caller-saved)
    assert!(!Register::RBX.is_systemv_caller_saved());
    assert!(!Register::RBP.is_systemv_caller_saved());
    assert!(!Register::R12.is_systemv_caller_saved());
    assert!(!Register::R13.is_systemv_caller_saved());
    assert!(!Register::R14.is_systemv_caller_saved());
    assert!(!Register::R15.is_systemv_caller_saved());
}

#[test]
fn test_systemv_callee_saved_registers() {
    // Test System V ABI callee-saved registers
    assert!(Register::RBX.is_systemv_callee_saved());
    assert!(Register::RBP.is_systemv_callee_saved());
    assert!(Register::R12.is_systemv_callee_saved());
    assert!(Register::R13.is_systemv_callee_saved());
    assert!(Register::R14.is_systemv_callee_saved());
    assert!(Register::R15.is_systemv_callee_saved());
    
    // Test System V ABI caller-saved registers (should not be callee-saved)
    assert!(!Register::RAX.is_systemv_callee_saved());
    assert!(!Register::RCX.is_systemv_callee_saved());
    assert!(!Register::RDX.is_systemv_callee_saved());
    assert!(!Register::RSI.is_systemv_callee_saved());
    assert!(!Register::RDI.is_systemv_callee_saved());
    assert!(!Register::R8.is_systemv_callee_saved());
    assert!(!Register::R9.is_systemv_callee_saved());
    assert!(!Register::R10.is_systemv_callee_saved());
    assert!(!Register::R11.is_systemv_callee_saved());
}

#[test]
fn test_general_purpose_registers_by_size() {
    // Test getting all registers of a specific size
    let sixty_four_bit_regs = Register::general_purpose_registers(64);
    let thirty_two_bit_regs = Register::general_purpose_registers(32);
    let sixteen_bit_regs = Register::general_purpose_registers(16);
    let eight_bit_regs = Register::general_purpose_registers(8);
    
    // Verify each list has the expected count
    assert_eq!(sixty_four_bit_regs.len(), 16); // RAX...R15 (16 registers)
    assert_eq!(thirty_two_bit_regs.len(), 16); // EAX...R15D (16 registers) 
    assert_eq!(sixteen_bit_regs.len(), 16);     // AX...R15W (16 registers)
    assert_eq!(eight_bit_regs.len(), 16);      // AL...R15B (16 registers)
    
    // Verify first and last of each size
    assert!(sixty_four_bit_regs.contains(&Register::RAX));
    assert!(sixty_four_bit_regs.contains(&Register::R15));
    
    assert!(thirty_two_bit_regs.contains(&Register::EAX));
    assert!(thirty_two_bit_regs.contains(&Register::R15D));
    
    assert!(sixteen_bit_regs.contains(&Register::AX));
    assert!(sixteen_bit_regs.contains(&Register::R15W));
    
    assert!(eight_bit_regs.contains(&Register::AL));
    assert!(eight_bit_regs.contains(&Register::R15B));
    
    // Test invalid size returns empty vector
    let invalid_size_regs = Register::general_purpose_registers(128);
    assert!(invalid_size_regs.is_empty());
}

#[test]
fn test_all_register_sizes() {
    // Test that all register variants return the correct size
    // 64-bit registers
    assert_eq!(Register::RAX.size(), 64);
    assert_eq!(Register::RBX.size(), 64);
    assert_eq!(Register::RCX.size(), 64);
    assert_eq!(Register::RDX.size(), 64);
    assert_eq!(Register::RSI.size(), 64);
    assert_eq!(Register::RDI.size(), 64);
    assert_eq!(Register::RBP.size(), 64);
    assert_eq!(Register::RSP.size(), 64);
    assert_eq!(Register::R8.size(), 64);
    assert_eq!(Register::R9.size(), 64);
    assert_eq!(Register::R10.size(), 64);
    assert_eq!(Register::R11.size(), 64);
    assert_eq!(Register::R12.size(), 64);
    assert_eq!(Register::R13.size(), 64);
    assert_eq!(Register::R14.size(), 64);
    assert_eq!(Register::R15.size(), 64);
    
    // 32-bit registers
    assert_eq!(Register::EAX.size(), 32);
    assert_eq!(Register::EBX.size(), 32);
    assert_eq!(Register::ECX.size(), 32);
    assert_eq!(Register::EDX.size(), 32);
    assert_eq!(Register::ESI.size(), 32);
    assert_eq!(Register::EDI.size(), 32);
    assert_eq!(Register::EBP.size(), 32);
    assert_eq!(Register::ESP.size(), 32);
    assert_eq!(Register::R8D.size(), 32);
    assert_eq!(Register::R9D.size(), 32);
    assert_eq!(Register::R10D.size(), 32);
    assert_eq!(Register::R11D.size(), 32);
    assert_eq!(Register::R12D.size(), 32);
    assert_eq!(Register::R13D.size(), 32);
    assert_eq!(Register::R14D.size(), 32);
    assert_eq!(Register::R15D.size(), 32);
    
    // 16-bit registers
    assert_eq!(Register::AX.size(), 16);
    assert_eq!(Register::BX.size(), 16);
    assert_eq!(Register::CX.size(), 16);
    assert_eq!(Register::DX.size(), 16);
    assert_eq!(Register::SI.size(), 16);
    assert_eq!(Register::DI.size(), 16);
    assert_eq!(Register::BP.size(), 16);
    assert_eq!(Register::SP.size(), 16);
    assert_eq!(Register::R8W.size(), 16);
    assert_eq!(Register::R9W.size(), 16);
    assert_eq!(Register::R10W.size(), 16);
    assert_eq!(Register::R11W.size(), 16);
    assert_eq!(Register::R12W.size(), 16);
    assert_eq!(Register::R13W.size(), 16);
    assert_eq!(Register::R14W.size(), 16);
    assert_eq!(Register::R15W.size(), 16);
    
    // 8-bit registers
    assert_eq!(Register::AL.size(), 8);
    assert_eq!(Register::BL.size(), 8);
    assert_eq!(Register::CL.size(), 8);
    assert_eq!(Register::DL.size(), 8);
    assert_eq!(Register::SIL.size(), 8);
    assert_eq!(Register::DIL.size(), 8);
    assert_eq!(Register::BPL.size(), 8);
    assert_eq!(Register::SPL.size(), 8);
    assert_eq!(Register::R8B.size(), 8);
    assert_eq!(Register::R9B.size(), 8);
    assert_eq!(Register::R10B.size(), 8);
    assert_eq!(Register::R11B.size(), 8);
    assert_eq!(Register::R12B.size(), 8);
    assert_eq!(Register::R13B.size(), 8);
    assert_eq!(Register::R14B.size(), 8);
    assert_eq!(Register::R15B.size(), 8);
}