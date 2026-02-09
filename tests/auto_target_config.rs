// tests/auto_target_config.rs
//! Integration tests for automatic target configuration (feature 024).
//!
//! These tests validate:
//! - US1: `Module::new()` reflects host OS
//! - US2: Deterministic cross-platform testing via `platform_config_for()`
//! - US3: `DataLayout` / `TargetTriple` consistency per platform
//! - US4: Manual override after auto-detection
//! - Edge cases: unsupported OS, unsupported architecture, warnings

use jsavrs::ir::platform::{detect_host_platform, platform_config_for, platform_config_with_warnings};
use jsavrs::ir::{DataLayout, Module, TargetTriple};

// ─── US1: Module::new() reflects host OS ─────────────────────────────

#[test]
fn test_module_new_reflects_host_os() {
    let module = Module::new("auto_detect_test", None);

    let expected = detect_host_platform();
    assert_eq!(
        *module.data_layout(),
        expected.data_layout,
        "Module::new() data_layout should match detect_host_platform()"
    );
    assert_eq!(
        *module.target_triple(),
        expected.target_triple,
        "Module::new() target_triple should match detect_host_platform()"
    );

    // Also verify against std::env::consts for extra confidence
    match std::env::consts::OS {
        "windows" => {
            assert_eq!(*module.data_layout(), DataLayout::WindowsX86_64);
            assert_eq!(*module.target_triple(), TargetTriple::X86_64PcWindowsGnu);
        }
        "linux" => {
            assert_eq!(*module.data_layout(), DataLayout::LinuxX86_64);
            assert_eq!(*module.target_triple(), TargetTriple::X86_64UnknownLinuxGnu);
        }
        "macos" => {
            assert_eq!(*module.data_layout(), DataLayout::MacOSX86_64);
            assert_eq!(*module.target_triple(), TargetTriple::X86_64AppleDarwin);
        }
        _ => {
            // Unsupported OS falls back to Linux
            assert_eq!(*module.data_layout(), DataLayout::LinuxX86_64);
            assert_eq!(*module.target_triple(), TargetTriple::X86_64UnknownLinuxGnu);
        }
    }
}

// ─── US2: Deterministic cross-platform testing ───────────────────────

#[test]
fn test_platform_config_for_windows() {
    let config = platform_config_for("windows", "x86_64");
    assert_eq!(config.data_layout, DataLayout::WindowsX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64PcWindowsGnu);
}

#[test]
fn test_platform_config_for_linux() {
    let config = platform_config_for("linux", "x86_64");
    assert_eq!(config.data_layout, DataLayout::LinuxX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64UnknownLinuxGnu);
}

#[test]
fn test_platform_config_for_macos() {
    let config = platform_config_for("macos", "x86_64");
    assert_eq!(config.data_layout, DataLayout::MacOSX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64AppleDarwin);
}

// ─── US3: DataLayout / TargetTriple consistency ──────────────────────

#[test]
fn test_windows_config_consistency() {
    let config = platform_config_for("windows", "x86_64");
    assert_eq!(config.data_layout, DataLayout::WindowsX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64PcWindowsGnu);
    // Windows COFF mangling: m:w
    let dl_str = config.data_layout.to_string();
    assert!(dl_str.contains("m:w"), "Windows DataLayout should contain 'm:w' mangling, got: {dl_str}");
}

#[test]
fn test_linux_config_consistency() {
    let config = platform_config_for("linux", "x86_64");
    assert_eq!(config.data_layout, DataLayout::LinuxX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64UnknownLinuxGnu);
    // ELF mangling: m:e
    let dl_str = config.data_layout.to_string();
    assert!(dl_str.contains("m:e"), "Linux DataLayout should contain 'm:e' mangling, got: {dl_str}");
}

#[test]
fn test_macos_config_consistency() {
    let config = platform_config_for("macos", "x86_64");
    assert_eq!(config.data_layout, DataLayout::MacOSX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64AppleDarwin);
    // Mach-O mangling: m:o
    let dl_str = config.data_layout.to_string();
    assert!(dl_str.contains("m:o"), "macOS DataLayout should contain 'm:o' mangling, got: {dl_str}");
}

// ─── US4: Manual override after auto-detection ───────────────────────

#[test]
fn test_manual_override_after_auto_detection() {
    let mut module = Module::new("override_test", None);

    // Override to explicit Linux config regardless of host
    module.set_data_layout(DataLayout::LinuxX86_64);
    module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

    assert_eq!(*module.data_layout(), DataLayout::LinuxX86_64);
    assert_eq!(*module.target_triple(), TargetTriple::X86_64UnknownLinuxGnu);

    // Override to Windows
    module.set_data_layout(DataLayout::WindowsX86_64);
    module.set_target_triple(TargetTriple::X86_64PcWindowsGnu);

    assert_eq!(*module.data_layout(), DataLayout::WindowsX86_64);
    assert_eq!(*module.target_triple(), TargetTriple::X86_64PcWindowsGnu);
}

// ─── Edge Cases ──────────────────────────────────────────────────────

#[test]
fn test_unsupported_os_falls_back_to_linux() {
    let config = platform_config_for("freebsd", "x86_64");
    assert_eq!(config.data_layout, DataLayout::LinuxX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64UnknownLinuxGnu);
}

#[test]
fn test_unsupported_arch_uses_x86_64_config_linux() {
    let config = platform_config_for("linux", "aarch64");
    assert_eq!(config.data_layout, DataLayout::LinuxX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64UnknownLinuxGnu);
}

#[test]
fn test_unsupported_arch_uses_x86_64_config_macos() {
    let config = platform_config_for("macos", "aarch64");
    assert_eq!(config.data_layout, DataLayout::MacOSX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64AppleDarwin);
}

#[test]
fn test_unsupported_arch_uses_x86_64_config_windows() {
    let config = platform_config_for("windows", "aarch64");
    assert_eq!(config.data_layout, DataLayout::WindowsX86_64);
    assert_eq!(config.target_triple, TargetTriple::X86_64PcWindowsGnu);
}

// ─── Subprocess helpers for stderr capture (cross-platform) ──────────
//
// The `gag` crate does not reliably capture `eprintln!` on Windows.
// Instead we use the standard Rust pattern: an `#[ignore]` test that
// emits the warning is re-invoked as a subprocess; the parent test
// captures its stderr via `std::process::Command`.

#[test]
#[ignore = "invoked as subprocess by parent test"]
fn subprocess_emit_unsupported_os_warning() {
    // Invoked only as a subprocess by `test_unsupported_os_emits_stderr_warning`.
    let _ = platform_config_with_warnings("freebsd", "x86_64");
}

#[test]
fn test_unsupported_os_emits_stderr_warning() {
    let exe = std::env::current_exe().expect("failed to get test executable path");
    let output = std::process::Command::new(exe)
        .args(["subprocess_emit_unsupported_os_warning", "--exact", "--ignored", "--nocapture"])
        .output()
        .expect("failed to run subprocess");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("warning: unsupported host OS 'freebsd'"), "Expected OS warning in stderr, got: {stderr}");
}

#[test]
#[ignore = "invoked as subprocess by parent test"]
fn subprocess_emit_unsupported_arch_warning() {
    // Invoked only as a subprocess by `test_unsupported_arch_emits_stderr_warning`.
    let _ = platform_config_with_warnings("linux", "aarch64");
}

#[test]
fn test_unsupported_arch_emits_stderr_warning() {
    let exe = std::env::current_exe().expect("failed to get test executable path");
    let output = std::process::Command::new(exe)
        .args(["subprocess_emit_unsupported_arch_warning", "--exact", "--ignored", "--nocapture"])
        .output()
        .expect("failed to run subprocess");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("warning: unsupported host architecture 'aarch64'"),
        "Expected arch warning in stderr, got: {stderr}"
    );
}
