//! Integration tests for x86-64 ABI (Application Binary Interface) implementations.
//!
//! Tests the Windows and System V ABI conventions including calling conventions,
//! register usage, stack alignment, and parameter passing.

use jsavrs::asm::{Abi, AbiKind, GPRegister64, Platform, X86Register, XMMRegister};

#[test]
fn test_abi_from_platform() {
    assert_eq!(Abi::from_platform(Platform::Windows), Abi::WINDOWS);
    assert_eq!(Abi::from_platform(Platform::Linux), Abi::SYSTEM_V_LINUX);
    assert_eq!(Abi::from_platform(Platform::MacOS), Abi::SYSTEM_V_MACOS);
}

#[test]
fn test_alignment() {
    assert_eq!(Abi::SYSTEM_V_LINUX.alignment(), 16);
    assert_eq!(Abi::SYSTEM_V_MACOS.alignment(), 16);
    assert_eq!(Abi::WINDOWS.alignment(), 16);
}

#[test]
fn test_red_zone() {
    assert_eq!(Abi::SYSTEM_V_LINUX.red_zone(), 128);
    assert_eq!(Abi::SYSTEM_V_MACOS.red_zone(), 128);
    assert_eq!(Abi::WINDOWS.red_zone(), 0);
}

#[test]
fn test_shadow_space() {
    assert_eq!(Abi::SYSTEM_V_LINUX.shadow_space(), 0);
    assert_eq!(Abi::SYSTEM_V_MACOS.shadow_space(), 0);
    assert_eq!(Abi::WINDOWS.shadow_space(), 32);
}

#[test]
fn test_int_param_registers() {
    assert_eq!(Abi::SYSTEM_V_LINUX.int_param_registers().len(), 6);
    assert_eq!(Abi::SYSTEM_V_MACOS.int_param_registers().len(), 6);
    assert_eq!(Abi::WINDOWS.int_param_registers().len(), 4);
    assert_eq!(Abi::SYSTEM_V_LINUX.int_param_registers()[0], GPRegister64::Rdi);
    assert_eq!(Abi::SYSTEM_V_MACOS.int_param_registers()[0], GPRegister64::Rdi);
    assert_eq!(Abi::WINDOWS.int_param_registers()[0], GPRegister64::Rcx);
}

#[test]
fn test_float_param_registers() {
    assert_eq!(Abi::SYSTEM_V_LINUX.float_param_registers().len(), 8);
    assert_eq!(Abi::SYSTEM_V_MACOS.float_param_registers().len(), 8);
    assert_eq!(Abi::WINDOWS.float_param_registers().len(), 4);
    assert_eq!(Abi::SYSTEM_V_LINUX.float_param_registers()[0], XMMRegister::Xmm0);
    assert_eq!(Abi::SYSTEM_V_MACOS.float_param_registers()[0], XMMRegister::Xmm0);
    assert_eq!(Abi::WINDOWS.float_param_registers()[0], XMMRegister::Xmm0);
}

#[test]
fn test_int_return_register() {
    assert_eq!(Abi::SYSTEM_V_LINUX.int_return_registers()[0], GPRegister64::Rax);
    assert_eq!(Abi::SYSTEM_V_MACOS.int_return_registers()[0], GPRegister64::Rax);
    assert_eq!(Abi::WINDOWS.int_return_registers()[0], GPRegister64::Rax);
}

#[test]
fn test_float_return_register() {
    assert_eq!(Abi::SYSTEM_V_LINUX.float_return_registers()[0], XMMRegister::Xmm0);
    assert_eq!(Abi::SYSTEM_V_MACOS.float_return_registers()[0], XMMRegister::Xmm0);
    assert_eq!(Abi::WINDOWS.float_return_registers()[0], XMMRegister::Xmm0);
}

#[test]
fn test_callee_saved() {
    let abi = Abi::SYSTEM_V_LINUX;
    assert!(abi.is_callee_saved(X86Register::GP64(GPRegister64::Rbx)));
    assert!(!abi.is_callee_saved(X86Register::GP64(GPRegister64::Rax)));

    let win_abi = Abi::WINDOWS;
    assert!(win_abi.is_callee_saved(X86Register::GP64(GPRegister64::Rdi)));
    assert!(win_abi.is_callee_saved(X86Register::Xmm(XMMRegister::Xmm6)));

    let mac_abi = Abi::SYSTEM_V_MACOS;
    assert!(mac_abi.is_callee_saved(X86Register::GP64(GPRegister64::Rbx)));
    assert!(!mac_abi.is_callee_saved(X86Register::GP64(GPRegister64::Rax)));
    let sysv_saved = abi.callee_saved_gp_registers();
    assert!(sysv_saved.contains(&GPRegister64::Rbx));
    assert!(sysv_saved.contains(&GPRegister64::Rbp));
    assert!(sysv_saved.contains(&GPRegister64::R12));
    assert!(!sysv_saved.contains(&GPRegister64::Rax));

    let windows_saved = win_abi.callee_saved_gp_registers();
    assert!(windows_saved.contains(&GPRegister64::Rbx));
    assert!(windows_saved.contains(&GPRegister64::Rdi));
    assert!(windows_saved.contains(&GPRegister64::Rsi));
    assert!(!windows_saved.contains(&GPRegister64::Rax));
}

#[test]
fn test_abi_kind_fmt() {
    assert_eq!(format!("{}", AbiKind::SystemV), "System V AMD64 ABI");
    assert_eq!(format!("{}", AbiKind::Windows), "Microsoft x64 Calling Convention");
}
