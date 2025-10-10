//! Integration tests for x86-64 ABI (Application Binary Interface) implementations.
//!
//! Tests the Windows and System V ABI conventions including calling conventions,
//! register usage, stack alignment, and parameter passing.

use jsavrs::asm::{Abi, GPRegister64, Platform, X86Register, XMMRegister};

#[test]
fn test_abi_from_platform() {
    assert_eq!(Abi::from_platform(Platform::Windows), Abi::Windows);
    assert_eq!(Abi::from_platform(Platform::Linux), Abi::SystemV);
    assert_eq!(Abi::from_platform(Platform::MacOS), Abi::SystemV);
}

#[test]
fn test_alignment() {
    assert_eq!(Abi::SystemV.alignment(), 16);
    assert_eq!(Abi::Windows.alignment(), 16);
}

#[test]
fn test_red_zone() {
    assert_eq!(Abi::SystemV.red_zone(), 128);
    assert_eq!(Abi::Windows.red_zone(), 0);
}

#[test]
fn test_shadow_space() {
    assert_eq!(Abi::SystemV.shadow_space(), 0);
    assert_eq!(Abi::Windows.shadow_space(), 32);
}

#[test]
fn test_int_param_registers() {
    assert_eq!(Abi::SystemV.int_param_registers().len(), 6);
    assert_eq!(Abi::Windows.int_param_registers().len(), 4);
    assert_eq!(Abi::SystemV.int_param_registers()[0], GPRegister64::Rdi);
    assert_eq!(Abi::Windows.int_param_registers()[0], GPRegister64::Rcx);
}

#[test]
fn test_callee_saved() {
    let abi = Abi::SystemV;
    assert!(abi.is_callee_saved(X86Register::GP64(GPRegister64::Rbx)));
    assert!(!abi.is_callee_saved(X86Register::GP64(GPRegister64::Rax)));

    let win_abi = Abi::Windows;
    assert!(win_abi.is_callee_saved(X86Register::GP64(GPRegister64::Rdi)));
    assert!(win_abi.is_callee_saved(X86Register::Xmm(XMMRegister::Xmm6)));
}
