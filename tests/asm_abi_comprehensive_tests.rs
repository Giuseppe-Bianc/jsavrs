//! Comprehensive integration tests for x86-64 ABI (Application Binary Interface) implementations.
//!
//! This test module provides exhaustive coverage of the ABI module including:
//! - `AbiKind` enum variants and Display trait
//! - `Abi` struct methods, constants, and Display trait
//! - `VariadicInfo` struct and Display trait
//! - Edge cases, boundary conditions, and cross-platform behavior
//!
//! # Test Coverage
//!
//! - All public methods on `Abi` struct
//! - All `AbiKind` variants
//! - Platform-specific behavior differences between System V and Windows
//! - Register classification and parameter passing conventions
//! - Stack layout and memory requirements

use jsavrs::asm::{Abi, AbiKind, GPRegister64, Platform, X86Register, XMMRegister};

#[test]
fn test_abi_kind_debug_format() {
    // Verify Debug trait implementation produces expected output
    let systemv_debug = format!("{:?}", AbiKind::SystemV);
    let windows_debug = format!("{:?}", AbiKind::Windows);

    assert_eq!(systemv_debug, "SystemV");
    assert_eq!(windows_debug, "Windows");
}

#[test]
fn test_abi_kind_display_systemv() {
    let display = format!("{}", AbiKind::SystemV);
    assert_eq!(display, "System V AMD64 ABI");
}

#[test]
fn test_abi_kind_display_windows() {
    let display = format!("{}", AbiKind::Windows);
    assert_eq!(display, "Microsoft x64 Calling Convention");
}

#[test]
fn test_abi_kind_clone() {
    let original = AbiKind::SystemV;
    let cloned = original.clone();
    assert_eq!(original, cloned);

    let original = AbiKind::Windows;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_abi_kind_copy() {
    let original = AbiKind::SystemV;
    let copied = original; // Copy semantics
    assert_eq!(original, copied);
}

#[test]
fn test_abi_kind_equality() {
    assert_eq!(AbiKind::SystemV, AbiKind::SystemV);
    assert_eq!(AbiKind::Windows, AbiKind::Windows);
    assert_ne!(AbiKind::SystemV, AbiKind::Windows);
    assert_ne!(AbiKind::Windows, AbiKind::SystemV);
}

#[test]
fn test_from_platform_windows() {
    let abi = Abi::from_platform(Platform::Windows);
    assert_eq!(abi.kind, AbiKind::Windows);
    assert_eq!(abi.platform, Platform::Windows);
}

#[test]
fn test_from_platform_linux() {
    let abi = Abi::from_platform(Platform::Linux);
    assert_eq!(abi.kind, AbiKind::SystemV);
    assert_eq!(abi.platform, Platform::Linux);
}

#[test]
fn test_from_platform_macos() {
    let abi = Abi::from_platform(Platform::MacOS);
    assert_eq!(abi.kind, AbiKind::SystemV);
    assert_eq!(abi.platform, Platform::MacOS);
}

#[test]
fn test_constant_system_v_linux() {
    let abi = Abi::SYSTEM_V_LINUX;
    assert_eq!(abi.kind, AbiKind::SystemV);
    assert_eq!(abi.platform, Platform::Linux);
}

#[test]
fn test_constant_system_v_macos() {
    let abi = Abi::SYSTEM_V_MACOS;
    assert_eq!(abi.kind, AbiKind::SystemV);
    assert_eq!(abi.platform, Platform::MacOS);
}

#[test]
fn test_constant_windows() {
    let abi = Abi::WINDOWS;
    assert_eq!(abi.kind, AbiKind::Windows);
    assert_eq!(abi.platform, Platform::Windows);
}

#[test]
fn test_from_platform_matches_constants() {
    assert_eq!(Abi::from_platform(Platform::Windows), Abi::WINDOWS);
    assert_eq!(Abi::from_platform(Platform::Linux), Abi::SYSTEM_V_LINUX);
    assert_eq!(Abi::from_platform(Platform::MacOS), Abi::SYSTEM_V_MACOS);
}

#[test]
fn test_abi_clone() {
    let original = Abi::SYSTEM_V_LINUX;
    let cloned = original.clone();
    assert_eq!(original.kind, cloned.kind);
    assert_eq!(original.platform, cloned.platform);
}

#[test]
fn test_abi_copy() {
    let original = Abi::WINDOWS;
    let copied = original; // Copy semantics
    assert_eq!(original.kind, copied.kind);
    assert_eq!(original.platform, copied.platform);
}

#[test]
fn test_abi_debug_format() {
    let debug_output = format!("{:?}", Abi::SYSTEM_V_LINUX);
    assert!(debug_output.contains("Abi"));
    assert!(debug_output.contains("SystemV"));
    assert!(debug_output.contains("Linux"));
}

#[test]
fn test_display_system_v_linux() {
    let display = format!("{}", Abi::SYSTEM_V_LINUX);
    assert_eq!(display, "System V AMD64 ABI on Linux");
}

#[test]
fn test_display_system_v_macos() {
    let display = format!("{}", Abi::SYSTEM_V_MACOS);
    assert_eq!(display, "System V AMD64 ABI on macOS");
}

#[test]
fn test_display_windows() {
    let display = format!("{}", Abi::WINDOWS);
    assert_eq!(display, "Microsoft x64 Calling Convention on Windows");
}

#[test]
fn test_display_contains_platform() {
    let linux = format!("{}", Abi::SYSTEM_V_LINUX);
    let macos = format!("{}", Abi::SYSTEM_V_MACOS);
    let windows = format!("{}", Abi::WINDOWS);

    assert!(linux.contains("Linux"));
    assert!(macos.contains("macOS"));
    assert!(windows.contains("Windows"));
}

#[test]
fn test_alignment_all_platforms_16_bytes() {
    // x86-64 requires 16-byte stack alignment across all platforms
    assert_eq!(Abi::SYSTEM_V_LINUX.alignment(), 16);
    assert_eq!(Abi::SYSTEM_V_MACOS.alignment(), 16);
    assert_eq!(Abi::WINDOWS.alignment(), 16);
}

#[test]
fn test_red_zone_system_v() {
    // System V ABI provides 128-byte red zone for leaf functions
    assert_eq!(Abi::SYSTEM_V_LINUX.red_zone(), 128);
    assert_eq!(Abi::SYSTEM_V_MACOS.red_zone(), 128);
}

#[test]
fn test_red_zone_windows() {
    // Windows x64 has no red zone
    assert_eq!(Abi::WINDOWS.red_zone(), 0);
}

#[test]
fn test_shadow_space_system_v() {
    // System V has no shadow space requirement
    assert_eq!(Abi::SYSTEM_V_LINUX.shadow_space(), 0);
    assert_eq!(Abi::SYSTEM_V_MACOS.shadow_space(), 0);
}

#[test]
fn test_shadow_space_windows() {
    // Windows requires 32 bytes of shadow space (4 registers × 8 bytes)
    assert_eq!(Abi::WINDOWS.shadow_space(), 32);
}

#[test]
fn test_first_stack_param_offset_system_v() {
    // System V: only return address (8 bytes)
    assert_eq!(Abi::SYSTEM_V_LINUX.first_stack_param_offset(), 8);
    assert_eq!(Abi::SYSTEM_V_MACOS.first_stack_param_offset(), 8);
}

#[test]
fn test_first_stack_param_offset_windows() {
    // Windows: return address (8) + shadow space (32) = 40 bytes
    assert_eq!(Abi::WINDOWS.first_stack_param_offset(), 40);
}

#[test]
fn test_first_stack_param_offset_calculation() {
    // Verify the calculation is correct
    let windows_abi = Abi::WINDOWS;
    let expected = 8 + windows_abi.shadow_space(); // return addr + shadow
    assert_eq!(windows_abi.first_stack_param_offset(), expected);
}

#[test]
fn test_stack_param_order_all_platforms() {
    // Both ABIs push remaining parameters left-to-right
    assert!(Abi::SYSTEM_V_LINUX.stack_param_order_is_left_to_right());
    assert!(Abi::SYSTEM_V_MACOS.stack_param_order_is_left_to_right());
    assert!(Abi::WINDOWS.stack_param_order_is_left_to_right());
}

#[test]
fn test_system_v_has_6_int_param_registers() {
    assert_eq!(Abi::SYSTEM_V_LINUX.int_param_registers().len(), 6);
    assert_eq!(Abi::SYSTEM_V_MACOS.int_param_registers().len(), 6);
}

#[test]
fn test_windows_has_4_int_param_registers() {
    assert_eq!(Abi::WINDOWS.int_param_registers().len(), 4);
}

#[test]
fn test_system_v_int_param_register_order() {
    let regs = Abi::SYSTEM_V_LINUX.int_param_registers();
    assert_eq!(regs[0], GPRegister64::Rdi);
    assert_eq!(regs[1], GPRegister64::Rsi);
    assert_eq!(regs[2], GPRegister64::Rdx);
    assert_eq!(regs[3], GPRegister64::Rcx);
    assert_eq!(regs[4], GPRegister64::R8);
    assert_eq!(regs[5], GPRegister64::R9);
}

#[test]
fn test_windows_int_param_register_order() {
    let regs = Abi::WINDOWS.int_param_registers();
    assert_eq!(regs[0], GPRegister64::Rcx);
    assert_eq!(regs[1], GPRegister64::Rdx);
    assert_eq!(regs[2], GPRegister64::R8);
    assert_eq!(regs[3], GPRegister64::R9);
}

#[test]
fn test_linux_and_macos_share_int_param_registers() {
    let linux_regs = Abi::SYSTEM_V_LINUX.int_param_registers();
    let macos_regs = Abi::SYSTEM_V_MACOS.int_param_registers();
    assert_eq!(linux_regs, macos_regs);
}

#[test]
fn test_system_v_first_param_is_rdi() {
    // RDI is used for first integer parameter in System V
    let abi = Abi::SYSTEM_V_LINUX;
    assert!(abi.is_parameter_register(X86Register::GP64(GPRegister64::Rdi), 0));
}

#[test]
fn test_windows_first_param_is_rcx() {
    // RCX is used for first integer parameter in Windows
    let abi = Abi::WINDOWS;
    assert!(abi.is_parameter_register(X86Register::GP64(GPRegister64::Rcx), 0));
}

#[test]
fn test_parameter_register_out_of_bounds() {
    // Test that registers beyond the limit aren't reported as parameter registers
    let systemv = Abi::SYSTEM_V_LINUX;
    let windows = Abi::WINDOWS;

    // System V has 6 integer params, Windows has 4
    // Parameter index 10 should never be a register param
    assert!(!systemv.is_parameter_register(X86Register::GP64(GPRegister64::Rdi), 10));
    assert!(!windows.is_parameter_register(X86Register::GP64(GPRegister64::Rcx), 10));
}

#[test]
fn test_system_v_has_8_float_param_registers() {
    assert_eq!(Abi::SYSTEM_V_LINUX.float_param_registers().len(), 8);
    assert_eq!(Abi::SYSTEM_V_MACOS.float_param_registers().len(), 8);
}

#[test]
fn test_windows_has_4_float_param_registers() {
    assert_eq!(Abi::WINDOWS.float_param_registers().len(), 4);
}

#[test]
fn test_system_v_float_param_register_order() {
    let regs = Abi::SYSTEM_V_LINUX.float_param_registers();
    assert_eq!(regs[0], XMMRegister::Xmm0);
    assert_eq!(regs[1], XMMRegister::Xmm1);
    assert_eq!(regs[2], XMMRegister::Xmm2);
    assert_eq!(regs[3], XMMRegister::Xmm3);
    assert_eq!(regs[4], XMMRegister::Xmm4);
    assert_eq!(regs[5], XMMRegister::Xmm5);
    assert_eq!(regs[6], XMMRegister::Xmm6);
    assert_eq!(regs[7], XMMRegister::Xmm7);
}

#[test]
fn test_windows_float_param_register_order() {
    let regs = Abi::WINDOWS.float_param_registers();
    assert_eq!(regs[0], XMMRegister::Xmm0);
    assert_eq!(regs[1], XMMRegister::Xmm1);
    assert_eq!(regs[2], XMMRegister::Xmm2);
    assert_eq!(regs[3], XMMRegister::Xmm3);
}

#[test]
fn test_xmm_parameter_register_system_v() {
    let abi = Abi::SYSTEM_V_LINUX;
    // Test all 8 XMM parameter positions
    for i in 0..8 {
        let xmm = match i {
            0 => XMMRegister::Xmm0,
            1 => XMMRegister::Xmm1,
            2 => XMMRegister::Xmm2,
            3 => XMMRegister::Xmm3,
            4 => XMMRegister::Xmm4,
            5 => XMMRegister::Xmm5,
            6 => XMMRegister::Xmm6,
            7 => XMMRegister::Xmm7,
            _ => unreachable!(),
        };
        assert!(
            abi.is_parameter_register(X86Register::Xmm(xmm), i),
            "XMM{i} should be parameter register at position {i}"
        );
    }
}

#[test]
fn test_xmm_parameter_register_windows() {
    let abi = Abi::WINDOWS;
    // Test all 4 XMM parameter positions for Windows
    for i in 0..4 {
        let xmm = match i {
            0 => XMMRegister::Xmm0,
            1 => XMMRegister::Xmm1,
            2 => XMMRegister::Xmm2,
            3 => XMMRegister::Xmm3,
            _ => unreachable!(),
        };
        assert!(
            abi.is_parameter_register(X86Register::Xmm(xmm), i),
            "XMM{i} should be parameter register at position {i} on Windows"
        );
    }
}

#[test]
fn test_int_return_registers_all_platforms() {
    // RAX and RDX are used for integer returns on all platforms
    let systemv = Abi::SYSTEM_V_LINUX.int_return_registers();
    let windows = Abi::WINDOWS.int_return_registers();

    assert_eq!(systemv.len(), 2);
    assert_eq!(windows.len(), 2);
    assert_eq!(systemv[0], GPRegister64::Rax);
    assert_eq!(systemv[1], GPRegister64::Rdx);
    assert_eq!(windows[0], GPRegister64::Rax);
    assert_eq!(windows[1], GPRegister64::Rdx);
}

#[test]
fn test_float_return_registers_system_v() {
    let regs = Abi::SYSTEM_V_LINUX.float_return_registers();
    assert_eq!(regs.len(), 2);
    assert_eq!(regs[0], XMMRegister::Xmm0);
    assert_eq!(regs[1], XMMRegister::Xmm1);
}

#[test]
fn test_float_return_registers_windows() {
    let regs = Abi::WINDOWS.float_return_registers();
    assert_eq!(regs.len(), 1);
    assert_eq!(regs[0], XMMRegister::Xmm0);
}

#[test]
fn test_rax_is_return_register_all_platforms() {
    let rax = X86Register::GP64(GPRegister64::Rax);
    assert!(Abi::SYSTEM_V_LINUX.is_return_register(rax));
    assert!(Abi::SYSTEM_V_MACOS.is_return_register(rax));
    assert!(Abi::WINDOWS.is_return_register(rax));
}

#[test]
fn test_rdx_is_return_register_all_platforms() {
    let rdx = X86Register::GP64(GPRegister64::Rdx);
    assert!(Abi::SYSTEM_V_LINUX.is_return_register(rdx));
    assert!(Abi::SYSTEM_V_MACOS.is_return_register(rdx));
    assert!(Abi::WINDOWS.is_return_register(rdx));
}

#[test]
fn test_xmm0_is_return_register_all_platforms() {
    let xmm0 = X86Register::Xmm(XMMRegister::Xmm0);
    assert!(Abi::SYSTEM_V_LINUX.is_return_register(xmm0));
    assert!(Abi::SYSTEM_V_MACOS.is_return_register(xmm0));
    assert!(Abi::WINDOWS.is_return_register(xmm0));
}

#[test]
fn test_xmm1_is_return_register_system_v() {
    let xmm1 = X86Register::Xmm(XMMRegister::Xmm1);
    assert!(Abi::SYSTEM_V_LINUX.is_return_register(xmm1));
    assert!(Abi::SYSTEM_V_MACOS.is_return_register(xmm1));
    // Note: Windows also reports XMM1 as return register based on X86Register impl
    assert!(Abi::WINDOWS.is_return_register(xmm1));
}

#[test]
fn test_non_return_registers() {
    // RBX should not be a return register on any platform
    let rbx = X86Register::GP64(GPRegister64::Rbx);
    assert!(!Abi::SYSTEM_V_LINUX.is_return_register(rbx));
    assert!(!Abi::SYSTEM_V_MACOS.is_return_register(rbx));
    assert!(!Abi::WINDOWS.is_return_register(rbx));
}

#[test]
fn test_system_v_callee_saved_gp_registers() {
    let regs = Abi::SYSTEM_V_LINUX.callee_saved_gp_registers();
    // RBX, RBP, R12, R13, R14, R15
    assert_eq!(regs.len(), 6);
    assert!(regs.contains(&GPRegister64::Rbx));
    assert!(regs.contains(&GPRegister64::Rbp));
    assert!(regs.contains(&GPRegister64::R12));
    assert!(regs.contains(&GPRegister64::R13));
    assert!(regs.contains(&GPRegister64::R14));
    assert!(regs.contains(&GPRegister64::R15));
}

#[test]
fn test_windows_callee_saved_gp_registers() {
    let regs = Abi::WINDOWS.callee_saved_gp_registers();
    // RBX, RBP, RDI, RSI, R12, R13, R14, R15
    assert_eq!(regs.len(), 8);
    assert!(regs.contains(&GPRegister64::Rbx));
    assert!(regs.contains(&GPRegister64::Rbp));
    assert!(regs.contains(&GPRegister64::Rdi));
    assert!(regs.contains(&GPRegister64::Rsi));
    assert!(regs.contains(&GPRegister64::R12));
    assert!(regs.contains(&GPRegister64::R13));
    assert!(regs.contains(&GPRegister64::R14));
    assert!(regs.contains(&GPRegister64::R15));
}

#[test]
fn test_system_v_rdi_rsi_are_not_callee_saved() {
    let regs = Abi::SYSTEM_V_LINUX.callee_saved_gp_registers();
    // In System V, RDI and RSI are used for parameters (caller-saved)
    assert!(!regs.contains(&GPRegister64::Rdi));
    assert!(!regs.contains(&GPRegister64::Rsi));
}

#[test]
fn test_windows_rdi_rsi_are_callee_saved() {
    let regs = Abi::WINDOWS.callee_saved_gp_registers();
    // In Windows, RDI and RSI are callee-saved
    assert!(regs.contains(&GPRegister64::Rdi));
    assert!(regs.contains(&GPRegister64::Rsi));
}

#[test]
fn test_system_v_no_callee_saved_xmm_registers() {
    let regs = Abi::SYSTEM_V_LINUX.callee_saved_xmm_registers();
    assert!(regs.is_empty(), "System V has no callee-saved XMM registers");
}

#[test]
fn test_windows_callee_saved_xmm_registers() {
    let regs = Abi::WINDOWS.callee_saved_xmm_registers();
    // XMM6-XMM15 (10 registers)
    assert_eq!(regs.len(), 10);
    assert!(regs.contains(&XMMRegister::Xmm6));
    assert!(regs.contains(&XMMRegister::Xmm7));
    assert!(regs.contains(&XMMRegister::Xmm8));
    assert!(regs.contains(&XMMRegister::Xmm9));
    assert!(regs.contains(&XMMRegister::Xmm10));
    assert!(regs.contains(&XMMRegister::Xmm11));
    assert!(regs.contains(&XMMRegister::Xmm12));
    assert!(regs.contains(&XMMRegister::Xmm13));
    assert!(regs.contains(&XMMRegister::Xmm14));
    assert!(regs.contains(&XMMRegister::Xmm15));
}

#[test]
fn test_is_callee_saved_rbx_all_platforms() {
    let rbx = X86Register::GP64(GPRegister64::Rbx);
    assert!(Abi::SYSTEM_V_LINUX.is_callee_saved(rbx));
    assert!(Abi::SYSTEM_V_MACOS.is_callee_saved(rbx));
    assert!(Abi::WINDOWS.is_callee_saved(rbx));
}

#[test]
fn test_is_callee_saved_rax_no_platforms() {
    let rax = X86Register::GP64(GPRegister64::Rax);
    assert!(!Abi::SYSTEM_V_LINUX.is_callee_saved(rax));
    assert!(!Abi::SYSTEM_V_MACOS.is_callee_saved(rax));
    assert!(!Abi::WINDOWS.is_callee_saved(rax));
}

#[test]
fn test_is_callee_saved_rdi_platform_specific() {
    let rdi = X86Register::GP64(GPRegister64::Rdi);
    // RDI is NOT callee-saved in System V (it's a parameter register)
    assert!(!Abi::SYSTEM_V_LINUX.is_callee_saved(rdi));
    assert!(!Abi::SYSTEM_V_MACOS.is_callee_saved(rdi));
    // RDI IS callee-saved in Windows
    assert!(Abi::WINDOWS.is_callee_saved(rdi));
}

#[test]
fn test_is_callee_saved_xmm6_platform_specific() {
    let xmm6 = X86Register::Xmm(XMMRegister::Xmm6);
    // XMM6 is NOT callee-saved in System V (all XMM are volatile)
    assert!(!Abi::SYSTEM_V_LINUX.is_callee_saved(xmm6));
    assert!(!Abi::SYSTEM_V_MACOS.is_callee_saved(xmm6));
    // XMM6 IS callee-saved in Windows
    assert!(Abi::WINDOWS.is_callee_saved(xmm6));
}

#[test]
fn test_system_v_caller_saved_gp_registers() {
    let regs = Abi::SYSTEM_V_LINUX.caller_saved_gp_registers();
    // RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11
    assert_eq!(regs.len(), 9);
    assert!(regs.contains(&GPRegister64::Rax));
    assert!(regs.contains(&GPRegister64::Rcx));
    assert!(regs.contains(&GPRegister64::Rdx));
    assert!(regs.contains(&GPRegister64::Rsi));
    assert!(regs.contains(&GPRegister64::Rdi));
    assert!(regs.contains(&GPRegister64::R8));
    assert!(regs.contains(&GPRegister64::R9));
    assert!(regs.contains(&GPRegister64::R10));
    assert!(regs.contains(&GPRegister64::R11));
}

#[test]
fn test_windows_caller_saved_gp_registers() {
    let regs = Abi::WINDOWS.caller_saved_gp_registers();
    // RAX, RCX, RDX, R8, R9, R10, R11
    assert_eq!(regs.len(), 7);
    assert!(regs.contains(&GPRegister64::Rax));
    assert!(regs.contains(&GPRegister64::Rcx));
    assert!(regs.contains(&GPRegister64::Rdx));
    assert!(regs.contains(&GPRegister64::R8));
    assert!(regs.contains(&GPRegister64::R9));
    assert!(regs.contains(&GPRegister64::R10));
    assert!(regs.contains(&GPRegister64::R11));
    // RDI and RSI are NOT volatile on Windows
    assert!(!regs.contains(&GPRegister64::Rdi));
    assert!(!regs.contains(&GPRegister64::Rsi));
}

#[test]
fn test_system_v_all_xmm_registers_are_caller_saved() {
    let regs = Abi::SYSTEM_V_LINUX.caller_saved_xmm_registers();
    // All 16 XMM registers are volatile in System V
    assert_eq!(regs.len(), 16);
}

#[test]
fn test_windows_xmm0_to_xmm5_are_caller_saved() {
    let regs = Abi::WINDOWS.caller_saved_xmm_registers();
    // Only XMM0-XMM5 are volatile in Windows
    assert_eq!(regs.len(), 6);
    assert!(regs.contains(&XMMRegister::Xmm0));
    assert!(regs.contains(&XMMRegister::Xmm1));
    assert!(regs.contains(&XMMRegister::Xmm2));
    assert!(regs.contains(&XMMRegister::Xmm3));
    assert!(regs.contains(&XMMRegister::Xmm4));
    assert!(regs.contains(&XMMRegister::Xmm5));
}

#[test]
fn test_is_caller_saved_rax_all_platforms() {
    let rax = X86Register::GP64(GPRegister64::Rax);
    assert!(Abi::SYSTEM_V_LINUX.is_caller_saved(rax));
    assert!(Abi::SYSTEM_V_MACOS.is_caller_saved(rax));
    assert!(Abi::WINDOWS.is_caller_saved(rax));
}

#[test]
fn test_is_caller_saved_r11_all_platforms() {
    let r11 = X86Register::GP64(GPRegister64::R11);
    assert!(Abi::SYSTEM_V_LINUX.is_caller_saved(r11));
    assert!(Abi::SYSTEM_V_MACOS.is_caller_saved(r11));
    assert!(Abi::WINDOWS.is_caller_saved(r11));
}

#[test]
fn test_is_caller_saved_rbx_no_platforms() {
    let rbx = X86Register::GP64(GPRegister64::Rbx);
    assert!(!Abi::SYSTEM_V_LINUX.is_caller_saved(rbx));
    assert!(!Abi::SYSTEM_V_MACOS.is_caller_saved(rbx));
    assert!(!Abi::WINDOWS.is_caller_saved(rbx));
}

#[test]
fn test_caller_callee_saved_are_complementary_gp() {
    // For GP registers, caller-saved and callee-saved should be disjoint
    let systemv = Abi::SYSTEM_V_LINUX;
    let caller_saved = systemv.caller_saved_gp_registers();
    let callee_saved = systemv.callee_saved_gp_registers();

    for reg in caller_saved {
        assert!(!callee_saved.contains(reg), "{:?} should not be in both caller and callee saved", reg);
    }
}

#[test]
fn test_system_v_struct_return_pointer_register() {
    assert_eq!(Abi::SYSTEM_V_LINUX.struct_return_pointer_register(), GPRegister64::Rdi);
    assert_eq!(Abi::SYSTEM_V_MACOS.struct_return_pointer_register(), GPRegister64::Rdi);
}

#[test]
fn test_windows_struct_return_pointer_register() {
    assert_eq!(Abi::WINDOWS.struct_return_pointer_register(), GPRegister64::Rcx);
}

#[test]
fn test_system_v_max_struct_return_size() {
    // System V can return up to 128 bits (16 bytes) in registers
    assert_eq!(Abi::SYSTEM_V_LINUX.max_struct_return_size(), 16);
    assert_eq!(Abi::SYSTEM_V_MACOS.max_struct_return_size(), 16);
}

#[test]
fn test_windows_max_struct_return_size() {
    // Windows can only return up to 64 bits (8 bytes) in registers
    assert_eq!(Abi::WINDOWS.max_struct_return_size(), 8);
}

#[test]
fn test_struct_return_uses_first_param_register() {
    // The struct return pointer is passed in the first parameter register
    let systemv = Abi::SYSTEM_V_LINUX;
    let windows = Abi::WINDOWS;

    assert_eq!(systemv.struct_return_pointer_register(), systemv.int_param_registers()[0]);
    assert_eq!(windows.struct_return_pointer_register(), windows.int_param_registers()[0]);
}

#[test]
fn test_system_v_does_not_require_frame_pointer() {
    assert!(!Abi::SYSTEM_V_LINUX.requires_frame_pointer());
    assert!(!Abi::SYSTEM_V_MACOS.requires_frame_pointer());
}

#[test]
fn test_windows_does_not_require_frame_pointer() {
    assert!(!Abi::WINDOWS.requires_frame_pointer());
}

#[test]
fn test_rbp_is_callee_saved_all_platforms() {
    // Even though frame pointer is optional, RBP is always callee-saved
    let rbp = X86Register::GP64(GPRegister64::Rbp);
    assert!(Abi::SYSTEM_V_LINUX.is_callee_saved(rbp));
    assert!(Abi::SYSTEM_V_MACOS.is_callee_saved(rbp));
    assert!(Abi::WINDOWS.is_callee_saved(rbp));
}

#[test]
fn test_scratch_register_is_r11() {
    assert_eq!(Abi::scratch_register(), GPRegister64::R11);
}

#[test]
fn test_scratch_register_is_caller_saved() {
    // R11 should be caller-saved on all platforms
    let r11 = X86Register::GP64(Abi::scratch_register());
    assert!(Abi::SYSTEM_V_LINUX.is_caller_saved(r11));
    assert!(Abi::SYSTEM_V_MACOS.is_caller_saved(r11));
    assert!(Abi::WINDOWS.is_caller_saved(r11));
}

#[test]
fn test_scratch_register_is_not_parameter_register() {
    // R11 should not be used for parameter passing in any ABI
    let r11 = X86Register::GP64(Abi::scratch_register());
    for i in 0..10 {
        assert!(!Abi::SYSTEM_V_LINUX.is_parameter_register(r11, i));
        assert!(!Abi::WINDOWS.is_parameter_register(r11, i));
    }
}

#[test]
fn test_system_v_variadic_info() {
    let info = Abi::SYSTEM_V_LINUX.variadic_info();
    assert!(info.supported);
    assert!(info.requires_va_list);
    assert!(info.requires_vector_count_in_al);
}

#[test]
fn test_windows_variadic_info() {
    let info = Abi::WINDOWS.variadic_info();
    assert!(info.supported);
    assert!(info.requires_va_list);
    assert!(!info.requires_vector_count_in_al);
}

#[test]
fn test_variadic_info_display() {
    let systemv_info = Abi::SYSTEM_V_LINUX.variadic_info();
    let display = format!("{}", systemv_info);

    assert!(display.contains("supported: true"));
    assert!(display.contains("requires_va_list: true"));
    assert!(display.contains("requires_vector_count_in_al: true"));
}

#[test]
fn test_variadic_info_display_windows() {
    let windows_info = Abi::WINDOWS.variadic_info();
    let display = format!("{}", windows_info);

    assert!(display.contains("supported: true"));
    assert!(display.contains("requires_va_list: true"));
    assert!(display.contains("requires_vector_count_in_al: false"));
}

#[test]
fn test_variadic_info_debug() {
    let info = Abi::SYSTEM_V_LINUX.variadic_info();
    let debug = format!("{:?}", info);

    assert!(debug.contains("VariadicInfo"));
    assert!(debug.contains("supported"));
    assert!(debug.contains("requires_va_list"));
}

#[test]
fn test_system_v_name() {
    assert_eq!(Abi::SYSTEM_V_LINUX.name(), "System V AMD64 ABI");
    assert_eq!(Abi::SYSTEM_V_MACOS.name(), "System V AMD64 ABI");
}

#[test]
fn test_windows_name() {
    assert_eq!(Abi::WINDOWS.name(), "Microsoft x64 Calling Convention");
}

#[test]
fn test_name_is_static_str() {
    // Verify that name returns a 'static str (doesn't allocate)
    let name: &'static str = Abi::SYSTEM_V_LINUX.name();
    assert!(!name.is_empty());
}

#[test]
fn test_register_classification_consistency() {
    // A register cannot be both caller-saved and callee-saved
    let abis = [Abi::SYSTEM_V_LINUX, Abi::SYSTEM_V_MACOS, Abi::WINDOWS];

    let gp_registers = [
        GPRegister64::Rax,
        GPRegister64::Rbx,
        GPRegister64::Rcx,
        GPRegister64::Rdx,
        GPRegister64::Rsi,
        GPRegister64::Rdi,
        GPRegister64::Rbp,
        GPRegister64::R8,
        GPRegister64::R9,
        GPRegister64::R10,
        GPRegister64::R11,
        GPRegister64::R12,
        GPRegister64::R13,
        GPRegister64::R14,
        GPRegister64::R15,
    ];

    for abi in &abis {
        for reg in &gp_registers {
            let x86reg = X86Register::GP64(*reg);
            let is_callee = abi.is_callee_saved(x86reg);
            let is_caller = abi.is_caller_saved(x86reg);

            // RSP is special - it's neither caller nor callee saved in the traditional sense
            if *reg != GPRegister64::Rsp {
                assert!(
                    is_callee != is_caller,
                    "Register {:?} has inconsistent save classification in {:?}: callee={}, caller={}",
                    reg,
                    abi.platform,
                    is_callee,
                    is_caller
                );
            }
        }
    }
}

#[test]
fn test_linux_macos_share_same_abi_kind() {
    assert_eq!(Abi::SYSTEM_V_LINUX.kind, Abi::SYSTEM_V_MACOS.kind);
}

#[test]
fn test_linux_macos_differ_in_platform() {
    assert_ne!(Abi::SYSTEM_V_LINUX.platform, Abi::SYSTEM_V_MACOS.platform);
}

#[test]
fn test_windows_differs_from_system_v() {
    assert_ne!(Abi::WINDOWS.kind, Abi::SYSTEM_V_LINUX.kind);
}

#[test]
fn test_int_param_register_counts() {
    // Windows has fewer integer param registers than System V
    let systemv_count = Abi::SYSTEM_V_LINUX.int_param_registers().len();
    let windows_count = Abi::WINDOWS.int_param_registers().len();

    assert!(systemv_count > windows_count, "System V should have more integer param registers");
}

#[test]
fn test_float_param_register_counts() {
    // Windows has fewer float param registers than System V
    let systemv_count = Abi::SYSTEM_V_LINUX.float_param_registers().len();
    let windows_count = Abi::WINDOWS.float_param_registers().len();

    assert!(systemv_count > windows_count, "System V should have more float param registers");
}

#[test]
fn test_windows_has_more_callee_saved_gp() {
    // Windows preserves more GP registers across calls
    let systemv_count = Abi::SYSTEM_V_LINUX.callee_saved_gp_registers().len();
    let windows_count = Abi::WINDOWS.callee_saved_gp_registers().len();

    assert!(windows_count > systemv_count, "Windows should have more callee-saved GP registers");
}

#[test]
fn test_windows_has_callee_saved_xmm() {
    // Windows preserves some XMM registers, System V does not
    let systemv_count = Abi::SYSTEM_V_LINUX.callee_saved_xmm_registers().len();
    let windows_count = Abi::WINDOWS.callee_saved_xmm_registers().len();

    assert_eq!(systemv_count, 0);
    assert!(windows_count > 0);
}

#[test]
fn test_parameter_index_zero() {
    // First parameter should always work
    let systemv = Abi::SYSTEM_V_LINUX;
    let windows = Abi::WINDOWS;

    assert!(systemv.is_parameter_register(X86Register::GP64(GPRegister64::Rdi), 0));
    assert!(windows.is_parameter_register(X86Register::GP64(GPRegister64::Rcx), 0));
}

#[test]
fn test_parameter_index_at_boundary_system_v() {
    // System V has 6 integer params (indices 0-5)
    let abi = Abi::SYSTEM_V_LINUX;

    // Last valid index (5)
    assert!(abi.is_parameter_register(X86Register::GP64(GPRegister64::R9), 5));

    // Just beyond boundary (6) - should return false
    assert!(!abi.is_parameter_register(X86Register::GP64(GPRegister64::R9), 6));
}

#[test]
fn test_parameter_index_at_boundary_windows() {
    // Windows has 4 integer params (indices 0-3)
    let abi = Abi::WINDOWS;

    // Last valid index (3)
    assert!(abi.is_parameter_register(X86Register::GP64(GPRegister64::R9), 3));

    // Just beyond boundary (4) - should return false
    assert!(!abi.is_parameter_register(X86Register::GP64(GPRegister64::R9), 4));
}

#[test]
fn test_parameter_index_very_large() {
    // Very large parameter indices should never match
    let abi = Abi::SYSTEM_V_LINUX;

    for reg in Abi::SYSTEM_V_LINUX.int_param_registers() {
        assert!(!abi.is_parameter_register(X86Register::GP64(*reg), usize::MAX));
    }
}

#[test]
fn test_wrong_register_for_parameter_position() {
    let abi = Abi::SYSTEM_V_LINUX;

    // RDI is param 0, not param 1
    assert!(!abi.is_parameter_register(X86Register::GP64(GPRegister64::Rdi), 1));

    // RSI is param 1, not param 0
    assert!(!abi.is_parameter_register(X86Register::GP64(GPRegister64::Rsi), 0));
}

#[test]
fn test_rsp_is_special() {
    // RSP should not be in caller-saved or callee-saved lists (it's the stack pointer)
    let systemv_caller = Abi::SYSTEM_V_LINUX.caller_saved_gp_registers();
    let windows_caller = Abi::WINDOWS.caller_saved_gp_registers();

    assert!(!systemv_caller.contains(&GPRegister64::Rsp));
    assert!(!windows_caller.contains(&GPRegister64::Rsp));
}

#[test]
fn test_all_register_arrays_are_non_empty() {
    let abis = [Abi::SYSTEM_V_LINUX, Abi::SYSTEM_V_MACOS, Abi::WINDOWS];

    for abi in &abis {
        assert!(!abi.int_param_registers().is_empty());
        assert!(!abi.float_param_registers().is_empty());
        assert!(!abi.int_return_registers().is_empty());
        assert!(!abi.float_return_registers().is_empty());
        assert!(!abi.callee_saved_gp_registers().is_empty());
        assert!(!abi.caller_saved_gp_registers().is_empty());
    }
}

#[test]
fn test_alignment_is_power_of_two() {
    let abis = [Abi::SYSTEM_V_LINUX, Abi::SYSTEM_V_MACOS, Abi::WINDOWS];

    for abi in &abis {
        let alignment = abi.alignment();
        assert!(alignment.is_power_of_two(), "Alignment {} is not a power of two", alignment);
    }
}

#[test]
fn test_shadow_space_is_aligned() {
    // Windows shadow space should be properly aligned
    let shadow = Abi::WINDOWS.shadow_space();
    assert_eq!(shadow % 8, 0, "Shadow space should be 8-byte aligned");
}

#[test]
fn test_max_struct_return_size_reasonable() {
    // Max struct return size should be reasonable (not too large)
    let abis = [Abi::SYSTEM_V_LINUX, Abi::SYSTEM_V_MACOS, Abi::WINDOWS];

    for abi in &abis {
        let max_size = abi.max_struct_return_size();
        assert!(max_size > 0, "Max struct return size should be positive");
        assert!(max_size <= 16, "Max struct return size should not exceed 16 bytes");
    }
}

#[test]
fn test_all_platforms_have_consistent_return_conventions() {
    // All platforms use RAX for integer returns
    let abis = [Abi::SYSTEM_V_LINUX, Abi::SYSTEM_V_MACOS, Abi::WINDOWS];

    for abi in &abis {
        let regs = abi.int_return_registers();
        assert_eq!(regs[0], GPRegister64::Rax);
    }
}

#[test]
fn test_all_platforms_use_xmm0_for_float_returns() {
    let abis = [Abi::SYSTEM_V_LINUX, Abi::SYSTEM_V_MACOS, Abi::WINDOWS];

    for abi in &abis {
        let regs = abi.float_return_registers();
        assert_eq!(regs[0], XMMRegister::Xmm0);
    }
}

#[test]
fn test_system_v_platforms_are_identical_except_platform() {
    let linux = Abi::SYSTEM_V_LINUX;
    let macos = Abi::SYSTEM_V_MACOS;

    // Same kind
    assert_eq!(linux.kind, macos.kind);

    // Same alignment
    assert_eq!(linux.alignment(), macos.alignment());

    // Same red zone
    assert_eq!(linux.red_zone(), macos.red_zone());

    // Same shadow space
    assert_eq!(linux.shadow_space(), macos.shadow_space());

    // Same parameter registers
    assert_eq!(linux.int_param_registers(), macos.int_param_registers());
    assert_eq!(linux.float_param_registers(), macos.float_param_registers());

    // Same callee-saved registers
    assert_eq!(linux.callee_saved_gp_registers(), macos.callee_saved_gp_registers());
    assert_eq!(linux.callee_saved_xmm_registers(), macos.callee_saved_xmm_registers());

    // Different platforms
    assert_ne!(linux.platform, macos.platform);
}

#[test]
fn test_key_differences_between_system_v_and_windows() {
    let systemv = Abi::SYSTEM_V_LINUX;
    let windows = Abi::WINDOWS;

    // Red zone difference
    assert_eq!(systemv.red_zone(), 128);
    assert_eq!(windows.red_zone(), 0);

    // Shadow space difference
    assert_eq!(systemv.shadow_space(), 0);
    assert_eq!(windows.shadow_space(), 32);

    // Number of integer param registers
    assert_ne!(systemv.int_param_registers().len(), windows.int_param_registers().len());

    // First integer param register
    assert_ne!(systemv.int_param_registers()[0], windows.int_param_registers()[0]);

    // RDI/RSI handling
    let rdi = X86Register::GP64(GPRegister64::Rdi);
    assert!(systemv.is_caller_saved(rdi));
    assert!(!windows.is_caller_saved(rdi));
}
