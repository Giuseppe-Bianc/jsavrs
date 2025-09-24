use jsavrs::asm::generator::TargetOS;
use jsavrs::asm::register::Register;

#[test]
fn test_all_targetos_methods_param_register() {
    // Test all TargetOS methods - param_register functionality
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    let macos_os = TargetOS::MacOS;
    
    // Linux and MacOS use System V ABI
    assert_eq!(linux_os.param_register(0), Some(Register::RDI));
    assert_eq!(linux_os.param_register(1), Some(Register::RSI));
    assert_eq!(linux_os.param_register(2), Some(Register::RDX));
    assert_eq!(linux_os.param_register(3), Some(Register::RCX));
    assert_eq!(linux_os.param_register(4), Some(Register::R8));
    assert_eq!(linux_os.param_register(5), Some(Register::R9));
    assert_eq!(linux_os.param_register(6), None); // Out of bounds
    
    // Windows uses different ABI
    assert_eq!(windows_os.param_register(0), Some(Register::RCX));
    assert_eq!(windows_os.param_register(1), Some(Register::RDX));
    assert_eq!(windows_os.param_register(2), Some(Register::R8));
    assert_eq!(windows_os.param_register(3), Some(Register::R9));
    assert_eq!(windows_os.param_register(4), None); // Out of bounds
    
    // MacOS should match Linux (System V ABI)
    assert_eq!(macos_os.param_register(0), Some(Register::RDI));
    assert_eq!(macos_os.param_register(1), Some(Register::RSI));
    assert_eq!(macos_os.param_register(2), Some(Register::RDX));
    assert_eq!(macos_os.param_register(3), Some(Register::RCX));
    assert_eq!(macos_os.param_register(4), Some(Register::R8));
    assert_eq!(macos_os.param_register(5), Some(Register::R9));
    assert_eq!(macos_os.param_register(6), None); // Out of bounds
}

#[test]
fn test_all_targetos_methods_callee_saved_registers() {
    // Test all TargetOS methods - callee_saved_registers functionality
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    let macos_os = TargetOS::MacOS;
    
    // Linux and MacOS use System V ABI (same callee-saved registers)
    let linux_callee = linux_os.callee_saved_registers();
    assert!(linux_callee.contains(&Register::RBX));
    assert!(linux_callee.contains(&Register::RBP));
    assert!(linux_callee.contains(&Register::R12));
    assert!(linux_callee.contains(&Register::R13));
    assert!(linux_callee.contains(&Register::R14));
    assert!(linux_callee.contains(&Register::R15));
    // System V does NOT include RDI/RSI as callee-saved
    assert!(!linux_callee.contains(&Register::RDI));
    assert!(!linux_callee.contains(&Register::RSI));
    
    // Windows x64 ABI includes additional callee-saved registers
    let windows_callee = windows_os.callee_saved_registers();
    assert!(windows_callee.contains(&Register::RBX));
    assert!(windows_callee.contains(&Register::RBP));
    assert!(windows_callee.contains(&Register::RDI));
    assert!(windows_callee.contains(&Register::RSI));
    assert!(windows_callee.contains(&Register::R12));
    assert!(windows_callee.contains(&Register::R13));
    assert!(windows_callee.contains(&Register::R14));
    assert!(windows_callee.contains(&Register::R15));
    
    // MacOS should match Linux
    let macos_callee = macos_os.callee_saved_registers();
    assert_eq!(linux_callee, macos_callee);
}

#[test]
fn test_os_specific_parameter_register_retrieval() {
    // Test OS-specific parameter register retrieval for Linux, Windows, and MacOS
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    let macos_os = TargetOS::MacOS;
    
    // Linux and MacOS should have the same parameter registers (System V ABI)
    assert_eq!(linux_os.param_register(0), macos_os.param_register(0));
    assert_eq!(linux_os.param_register(1), macos_os.param_register(1));
    assert_eq!(linux_os.param_register(2), macos_os.param_register(2));
    assert_eq!(linux_os.param_register(3), macos_os.param_register(3));
    
    // Windows should have different parameter registers
    assert_ne!(windows_os.param_register(0), linux_os.param_register(0));
    assert_ne!(windows_os.param_register(1), linux_os.param_register(1));
    assert_ne!(windows_os.param_register(2), linux_os.param_register(2));
    assert_ne!(windows_os.param_register(3), linux_os.param_register(3));
}

#[test]
fn test_callee_saved_register_retrieval_for_all_platforms() {
    // Test callee-saved register retrieval for all platforms
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    let macos_os = TargetOS::MacOS;
    
    let linux_callee = linux_os.callee_saved_registers();
    let windows_callee = windows_os.callee_saved_registers();
    let macos_callee = macos_os.callee_saved_registers();
    
    // Linux and MacOS should have the same callee-saved registers
    assert_eq!(linux_callee, macos_callee);
    
    // Windows should have more callee-saved registers than Linux/MacOS
    assert!(windows_callee.len() > linux_callee.len());
    
    // All platforms should have RBX and RBP as callee-saved
    assert!(linux_callee.contains(&Register::RBX));
    assert!(linux_callee.contains(&Register::RBP));
    assert!(windows_callee.contains(&Register::RBX));
    assert!(windows_callee.contains(&Register::RBP));
    assert!(macos_callee.contains(&Register::RBX));
    assert!(macos_callee.contains(&Register::RBP));
}

#[test]
fn test_error_handling_for_invalid_or_edge_case_inputs() {
    // Test error handling for invalid or edge-case inputs
    
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    
    // Test out-of-bounds parameter register access
    assert_eq!(linux_os.param_register(10), None); // Well out of bounds
    assert_eq!(windows_os.param_register(10), None); // Well out of bounds
    
    // Test valid bounds
    assert!(linux_os.param_register(0).is_some());
    assert!(linux_os.param_register(5).is_some()); // Last valid for Linux
    assert!(linux_os.param_register(6).is_none()); // First invalid for Linux
    
    assert!(windows_os.param_register(0).is_some());
    assert!(windows_os.param_register(3).is_some()); // Last valid for Windows
    assert!(windows_os.param_register(4).is_none()); // First invalid for Windows
    
    // Test is_param_register function
    assert!(linux_os.is_param_register(&Register::RDI)); // First param Linux
    assert!(windows_os.is_param_register(&Register::RCX)); // First param Windows
    
    // Test is_caller_saved and is_callee_saved functions
    // RAX is caller-saved in both ABIs
    assert!(linux_os.is_caller_saved(&Register::RAX));
    assert!(windows_os.is_caller_saved(&Register::RAX));
    
    // RBX is callee-saved in both ABIs
    assert!(linux_os.is_callee_saved(&Register::RBX));
    assert!(windows_os.is_callee_saved(&Register::RBX));
}

#[test]
fn test_consistency_across_different_os_targets() {
    // Test consistency across different OS targets
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    let macos_os = TargetOS::MacOS;
    
    // Linux and MacOS should be identical for most aspects
    assert_eq!(linux_os.is_unix(), true);
    assert_eq!(macos_os.is_unix(), true);
    assert_eq!(windows_os.is_unix(), false);
    
    assert_eq!(linux_os.is_windows(), false);
    assert_eq!(macos_os.is_windows(), false);
    assert_eq!(windows_os.is_windows(), true);
    
    // Both Unix-like systems should have same parameter register count for first 6 params
    for i in 0..6 {
        assert_eq!(
            linux_os.param_register(i).is_some(),
            macos_os.param_register(i).is_some()
        );
        
        if let (Some(lreg), Some(mreg)) = (linux_os.param_register(i), macos_os.param_register(i)) {
            assert_eq!(lreg, mreg);
        }
    }
    
    // But different from Windows
    for i in 0..4 {  // Windows only has 4 param registers
        assert_ne!(
            windows_os.param_register(i),
            linux_os.param_register(i)
        );
    }
}

#[test]
fn test_validation_results_logging_for_traceability_and_debugging() {
    // Test validation results logging for traceability and debugging
    // While the actual implementation doesn't have explicit logging,
    // we can test that the TargetOS provides consistent and traceable behavior
    
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    let macos_os = TargetOS::MacOS;
    
    // Verify consistent naming
    assert_eq!(linux_os.name(), "Linux");
    assert_eq!(windows_os.name(), "Windows");
    assert_eq!(macos_os.name(), "MacOS");
    
    // Verify the OS-specific properties are consistent
    assert_eq!(linux_os.is_unix(), true);
    assert_eq!(windows_os.is_unix(), false);
    assert_eq!(macos_os.is_unix(), true);
    
    assert_eq!(linux_os.is_windows(), false);
    assert_eq!(windows_os.is_windows(), true);
    assert_eq!(macos_os.is_windows(), false);
    
    // Verify that properties remain consistent across method calls
    assert_eq!(linux_os.name(), linux_os.name());
    assert_eq!(linux_os.is_unix(), linux_os.is_unix());
    assert_eq!(linux_os.is_windows(), linux_os.is_windows());
}

#[test]
fn test_backward_compatibility_in_api_even_if_internals_change() {
    // Test backward compatibility in the API even if internals change
    // This test verifies that the public API remains stable
    
    let os_types = [TargetOS::Linux, TargetOS::Windows, TargetOS::MacOS];
    
    for os in &os_types {
        // These methods should always be available and return valid data
        assert!(os.name().len() > 0);  // Name should not be empty
        assert!(os.is_windows() || os.is_unix());  // Must be one or the other
        
        // These methods should return valid data without panicking
        let _param_reg = os.param_register(0);
        let _callee_regs = os.callee_saved_registers();
        let _is_param = os.is_param_register(&Register::RAX);
        let _is_caller = os.is_caller_saved(&Register::RAX);
        let _is_callee = os.is_callee_saved(&Register::RAX);
    }
}

#[test]
fn test_snapshot_testing_for_detecting_changes() {
    // Test that behavior is consistent and detectable for snapshot testing
    let linux_os = TargetOS::Linux;
    let windows_os = TargetOS::Windows;
    
    // Create a simple representation of target OS properties that could be snapshotted
    let linux_params: Vec<String> = (0..6)
        .filter_map(|i| linux_os.param_register(i))
        .map(|r| format!("{:?}", r))
        .collect();
    
    let windows_params: Vec<String> = (0..4)
        .filter_map(|i| windows_os.param_register(i))
        .map(|r| format!("{:?}", r))
        .collect();
    
    // Verify we have the expected number of parameter registers for each OS
    assert_eq!(linux_params.len(), 6);  // System V ABI has 6 register params
    assert_eq!(windows_params.len(), 4);  // Windows x64 ABI has 4 register params
}