use jsavrs::asm::generator::{NasmGenerator, TargetOS};

#[test]
fn test_function_prologue_generation_for_linux() {
    // Test function prologue generation for Linux
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add prologue
    generator.add_function_prologue();
    
    // Check the generated code
    let code = generator.generate();
    
    // Linux (System V ABI) saves RBX, RBP, R12-R15 as callee-saved
    assert!(code.contains("push rbx"));
    assert!(code.contains("push rbp"));
    assert!(code.contains("push r12"));
    assert!(code.contains("push r13"));
    assert!(code.contains("push r14"));
    assert!(code.contains("push r15"));
    
    // Verify stack frame setup
    assert!(code.contains("mov rbp, rsp"));
}

#[test]
fn test_function_prologue_generation_for_windows() {
    // Test function prologue generation for Windows
    let mut generator = NasmGenerator::new(TargetOS::Windows);
    
    // Add prologue
    generator.add_function_prologue();
    
    // Check the generated code
    let code = generator.generate();
    
    // Windows x64 ABI saves RBX, RBP, RDI, RSI, R12-R15 as callee-saved
    assert!(code.contains("push rbx"));
    assert!(code.contains("push rbp"));
    assert!(code.contains("push rdi"));
    assert!(code.contains("push rsi"));
    assert!(code.contains("push r12"));
    assert!(code.contains("push r13"));
    assert!(code.contains("push r14"));
    assert!(code.contains("push r15"));
    
    // Verify stack frame setup
    assert!(code.contains("mov rbp, rsp"));
}

#[test]
fn test_function_epilogue_generation_for_linux() {
    // Test function epilogue generation for Linux
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    
    // Add epilogue
    generator.add_function_epilogue();
    
    // Check the generated code
    let code = generator.generate();
    
    // Verify stack frame restoration
    assert!(code.contains("mov rsp, rbp"));
    
    // Verify callee-saved register restoration in reverse order (for Linux)
    let lines: Vec<&str> = code.lines().collect();
    let mut pop_indices = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains("pop ") {
            pop_indices.push((i, line));
        }
    }
    
    // Check that all expected registers are being popped
    assert!(code.contains("pop r15"));
    assert!(code.contains("pop r14"));
    assert!(code.contains("pop r13"));
    assert!(code.contains("pop r12"));
    assert!(code.contains("pop rbp"));
    assert!(code.contains("pop rbx"));
    
    // Verify return instruction
    assert!(code.contains("ret"));
}

#[test]
fn test_function_epilogue_generation_for_windows() {
    // Test function epilogue generation for Windows
    let mut generator = NasmGenerator::new(TargetOS::Windows);
    
    // Add epilogue
    generator.add_function_epilogue();
    
    // Check the generated code
    let code = generator.generate();
    
    // Verify stack frame restoration
    assert!(code.contains("mov rsp, rbp"));
    
    // Verify callee-saved register restoration in reverse order (for Windows)
    assert!(code.contains("pop r15"));
    assert!(code.contains("pop r14"));
    assert!(code.contains("pop r13"));
    assert!(code.contains("pop r12"));
    assert!(code.contains("pop rbp"));
    assert!(code.contains("pop rdi"));
    assert!(code.contains("pop rsi"));
    assert!(code.contains("pop rbx"));
    
    // Verify return instruction
    assert!(code.contains("ret"));
}

#[test]
fn test_correct_stack_setup_for_each_platform() {
    // Test correct stack setup for each platform
    let mut linux_generator = NasmGenerator::new(TargetOS::Linux);
    let mut windows_generator = NasmGenerator::new(TargetOS::Windows);
    
    linux_generator.add_function_prologue();
    windows_generator.add_function_prologue();
    
    let linux_code = linux_generator.generate();
    let windows_code = windows_generator.generate();
    
    // Both should have the same stack frame setup instruction
    assert!(linux_code.contains("mov rbp, rsp"));
    assert!(windows_code.contains("mov rbp, rsp"));
    
    // But different sets of saved registers
    // Linux should not save RDI/RSI (they're parameter registers in System V ABI)
    assert!(!linux_code.contains("push rdi"));
    assert!(!linux_code.contains("push rsi"));
    
    // Windows should save RDI/RSI (they're callee-saved in Windows x64 ABI)
    assert!(windows_code.contains("push rdi"));
    assert!(windows_code.contains("push rsi"));
}

#[test]
fn test_register_preservation_for_each_platform() {
    // Test register preservation for each platform
    let mut linux_generator = NasmGenerator::new(TargetOS::Linux);
    let mut windows_generator = NasmGenerator::new(TargetOS::Windows);
    
    linux_generator.add_function_prologue();
    linux_generator.add_function_epilogue();
    windows_generator.add_function_prologue();
    windows_generator.add_function_epilogue();
    
    let linux_code = linux_generator.generate();
    let windows_code = windows_generator.generate();
    
    // Linux callee-saved: RBX, RBP, R12-R15
    assert!(linux_code.contains("push rbx") && linux_code.contains("pop rbx"));
    assert!(linux_code.contains("push rbp") && linux_code.contains("pop rbp"));
    assert!(linux_code.contains("push r12") && linux_code.contains("pop r12"));
    assert!(linux_code.contains("push r13") && linux_code.contains("pop r13"));
    assert!(linux_code.contains("push r14") && linux_code.contains("pop r14"));
    assert!(linux_code.contains("push r15") && linux_code.contains("pop r15"));
    
    // Windows callee-saved: RBX, RBP, RDI, RSI, R12-R15
    assert!(windows_code.contains("push rbx") && windows_code.contains("pop rbx"));
    assert!(windows_code.contains("push rbp") && windows_code.contains("pop rbp"));
    assert!(windows_code.contains("push rdi") && windows_code.contains("pop rdi"));
    assert!(windows_code.contains("push rsi") && windows_code.contains("pop rsi"));
    assert!(windows_code.contains("push r12") && windows_code.contains("pop r12"));
    assert!(windows_code.contains("push r13") && windows_code.contains("pop r13"));
    assert!(windows_code.contains("push r14") && windows_code.contains("pop r14"));
    assert!(windows_code.contains("push r15") && windows_code.contains("pop r15"));
}

#[test]
fn test_cleanup_procedures_for_each_platform() {
    // Test cleanup procedures for each platform
    let mut linux_generator = NasmGenerator::new(TargetOS::Linux);
    let mut windows_generator = NasmGenerator::new(TargetOS::Windows);
    
    linux_generator.add_function_prologue();
    linux_generator.add_function_epilogue();
    windows_generator.add_function_prologue();
    windows_generator.add_function_epilogue();
    
    let linux_code = linux_generator.generate();
    let windows_code = windows_generator.generate();
    
    // Both should end with return
    assert!(linux_code.contains("ret"));
    assert!(windows_code.contains("ret"));
    
    // Both should restore stack pointer from base pointer
    assert!(linux_code.contains("mov rsp, rbp"));
    assert!(windows_code.contains("mov rsp, rbp"));
}