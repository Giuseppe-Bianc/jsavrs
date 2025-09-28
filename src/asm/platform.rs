//! Platform-specific configuration for code generation
//!
//! Defines target platforms, operating systems, and ABI specifications
//! as outlined in the data model and contract specifications.

use std::fmt;

/// Platform-specific configuration for code generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetPlatform {
    /// Operating system (Windows, Linux, macOS)
    pub os: TargetOS,
    /// Processor architecture (x86_64)
    pub arch: TargetArch,
    /// ABI specification for function calls
    pub abi: ABISpec,
}

impl TargetPlatform {
    /// Create Windows x64 target configuration
    pub fn windows_x64() -> Self {
        TargetPlatform {
            os: TargetOS::Windows,
            arch: TargetArch::X86_64,
            abi: ABISpec::WindowsX64,
        }
    }
    
    /// Create Linux x64 target configuration
    pub fn linux_x64() -> Self {
        TargetPlatform {
            os: TargetOS::Linux,
            arch: TargetArch::X86_64,
            abi: ABISpec::SystemV,
        }
    }
    
    /// Create macOS x64 target configuration
    pub fn macos_x64() -> Self {
        TargetPlatform {
            os: TargetOS::MacOS,
            arch: TargetArch::X86_64,
            abi: ABISpec::SystemV,
        }
    }
    
    /// Validate platform configuration
    pub fn validate(&self) -> Result<(), PlatformError> {
        match self.arch {
            TargetArch::X86_64 => {},
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
}

impl fmt::Display for TargetOS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetOS::Windows => write!(f, "windows"),
            TargetOS::Linux => write!(f, "linux"),
            TargetOS::MacOS => write!(f, "macos"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetArch {
    X86_64,
}

impl fmt::Display for TargetArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetArch::X86_64 => write!(f, "x86_64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABISpec {
    /// Windows x64 calling convention
    WindowsX64,
    /// System V ABI (Linux/macOS)
    SystemV,
}

impl fmt::Display for ABISpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ABISpec::WindowsX64 => write!(f, "windows_x64"),
            ABISpec::SystemV => write!(f, "systemv"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Unsupported architecture: {arch}")]
    UnsupportedArchitecture { arch: TargetArch },
    #[error("Incompatible OS and ABI combination")]
    IncompatibleOSABI,
}