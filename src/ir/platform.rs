// src/ir/platform.rs
//! Platform detection and configuration for target-specific compilation.
//!
//! This module provides automatic host-OS detection to set correct `DataLayout`
//! and `TargetTriple` values for `Module` construction. It exposes a pure,
//! deterministic mapping function (`platform_config_for`) for testability and
//! a thin wrapper (`detect_host_platform`) that reads `std::env::consts`.
//!
//! # Architecture
//!
//! ```text
//! platform_config_for(os, arch)   ← pure, no side effects
//!        ↑
//! platform_config_with_warnings(os, arch)  ← emits eprintln! warnings
//!        ↑
//! detect_host_platform()          ← reads std::env::consts
//!        ↑
//! Module::new()                   ← consumer
//! ```

use super::data_layout::DataLayout;
use super::module::TargetTriple;

/// A consistent pair of `DataLayout` and `TargetTriple` for a single platform.
///
/// This struct is always constructed via [`platform_config_for()`] or
/// [`detect_host_platform()`], which guarantee internal consistency
/// (i.e., the `DataLayout`'s mangling mode always matches the `TargetTriple`'s platform).
///
/// # Examples
///
/// ```
/// use jsavrs::ir::platform_config_for;
/// use jsavrs::ir::{DataLayout, TargetTriple};
///
/// let config = platform_config_for("linux", "x86_64");
/// assert_eq!(config.data_layout, DataLayout::LinuxX86_64);
/// assert_eq!(config.target_triple, TargetTriple::X86_64UnknownLinuxGnu);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlatformConfig {
    /// Platform-specific data layout specification.
    pub data_layout: DataLayout,
    /// Platform identifier triple.
    pub target_triple: TargetTriple,
}

/// Returns the platform configuration for the given OS and architecture strings.
///
/// This is a pure, deterministic function with no side effects. It maps
/// OS names (as returned by [`std::env::consts::OS`]) and architecture names
/// (as returned by [`std::env::consts::ARCH`]) to consistent
/// (`DataLayout`, `TargetTriple`) pairs.
///
/// # Supported Platforms
///
/// | OS | Arch | `DataLayout` | `TargetTriple` |
/// |---|---|---|---|
/// | `"windows"` | `"x86_64"` | `WindowsX86_64` | `X86_64PcWindowsGnu` |
/// | `"linux"` | `"x86_64"` | `LinuxX86_64` | `X86_64UnknownLinuxGnu` |
/// | `"macos"` | `"x86_64"` | `MacOSX86_64` | `X86_64AppleDarwin` |
///
/// # Fallback Behavior
///
/// - Recognized OS with non-`"x86_64"` arch: Returns x86\_64 config for that OS.
/// - Unrecognized OS: Returns Linux x86\_64 config.
///
/// Note: This function does **not** emit warnings. Warnings are emitted by
/// [`platform_config_with_warnings()`] which wraps this function.
///
/// # Examples
///
/// ```
/// use jsavrs::ir::platform_config_for;
/// use jsavrs::ir::{DataLayout, TargetTriple};
///
/// let config = platform_config_for("windows", "x86_64");
/// assert_eq!(config.data_layout, DataLayout::WindowsX86_64);
/// assert_eq!(config.target_triple, TargetTriple::X86_64PcWindowsGnu);
/// ```
#[must_use]
pub fn platform_config_for(os: &str, arch: &str) -> PlatformConfig {
    // `arch` is accepted for API consistency with `platform_config_with_warnings`
    // but currently unused — all supported platforms map to their x86_64 variant.
    let _ = arch;

    match os {
        "windows" => {
            PlatformConfig { data_layout: DataLayout::WindowsX86_64, target_triple: TargetTriple::X86_64PcWindowsGnu }
        }
        "macos" => {
            PlatformConfig { data_layout: DataLayout::MacOSX86_64, target_triple: TargetTriple::X86_64AppleDarwin }
        }
        // "linux" and any unrecognized OS fall back to Linux x86_64 configuration
        _ => {
            PlatformConfig { data_layout: DataLayout::LinuxX86_64, target_triple: TargetTriple::X86_64UnknownLinuxGnu }
        }
    }
}

/// Returns the platform configuration for the given OS and architecture strings,
/// emitting `eprintln!` warnings for unsupported combinations.
///
/// This function combines the pure mapping logic of [`platform_config_for()`] with
/// warning emission for unsupported platforms. It exists to make warning behavior
/// deterministically testable from any host OS.
///
/// # Warnings
///
/// - Emits `eprintln!("warning: unsupported host OS '{}', ...")`
///   when `os` is not `"windows"`, `"linux"`, or `"macos"`.
/// - Emits `eprintln!("warning: unsupported host architecture '{}', ...")`
///   when `arch` is not `"x86_64"`.
///
/// # Examples
///
/// ```
/// use jsavrs::ir::platform::platform_config_with_warnings;
/// use jsavrs::ir::{DataLayout, TargetTriple};
///
/// // On any host, this returns Linux x86_64 config and emits a warning:
/// let config = platform_config_with_warnings("freebsd", "x86_64");
/// assert_eq!(config.data_layout, DataLayout::LinuxX86_64);
/// ```
#[must_use]
pub fn platform_config_with_warnings(os: &str, arch: &str) -> PlatformConfig {
    let is_supported_os = matches!(os, "windows" | "linux" | "macos");

    if !is_supported_os {
        eprintln!("warning: unsupported host OS '{os}', falling back to Linux x86_64 configuration");
    }

    if arch != "x86_64" {
        eprintln!("warning: unsupported host architecture '{arch}', using x86_64 configuration");
    }

    platform_config_for(os, arch)
}

/// Detects the host platform and returns the corresponding configuration.
///
/// Thin wrapper: calls [`platform_config_with_warnings()`] with
/// [`std::env::consts::OS`] and [`std::env::consts::ARCH`].
///
/// # Examples
///
/// ```
/// use jsavrs::ir::platform::detect_host_platform;
///
/// let config = detect_host_platform();
/// // Returns platform-appropriate config for the current host
/// ```
#[must_use]
pub fn detect_host_platform() -> PlatformConfig {
    platform_config_with_warnings(std::env::consts::OS, std::env::consts::ARCH)
}
