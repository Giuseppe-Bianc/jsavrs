//! Application Binary Interface conventions for floating-point parameters

use super::register::Register;

/// ABI calling convention variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABIConvention {
    /// Windows x64 calling convention
    WindowsX64,
    /// System V ABI (Linux, macOS, Unix)
    SystemV,
}

impl std::fmt::Display for ABIConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowsX64 => write!(f, "windows_x64"),
            Self::SystemV => write!(f, "system_v"),
        }
    }
}

impl Default for ABIConvention {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        return Self::WindowsX64;
        
        #[cfg(not(target_os = "windows"))]
        return Self::SystemV;
    }
}

impl ABIConvention {
    /// Get the floating-point registers used for parameter passing
    pub fn get_float_parameter_registers(&self) -> Vec<Register> {
        match self {
            Self::WindowsX64 => {
                // Windows x64: XMM0-XMM3 for first 4 float parameters
                vec![
                    Register::XMM0, Register::XMM1, Register::XMM2, Register::XMM3
                ]
            },
            Self::SystemV => {
                // System V: XMM0-XMM7 for float parameters
                vec![
                    Register::XMM0, Register::XMM1, Register::XMM2, Register::XMM3,
                    Register::XMM4, Register::XMM5, Register::XMM6, Register::XMM7
                ]
            }
        }
    }

    /// Get the floating-point register for return values
    pub fn get_float_return_register(&self) -> Register {
        // Both ABIs use XMM0 for floating-point return values
        Register::XMM0
    }

    /// Check if a register is caller-saved (volatile)
    pub fn is_caller_saved(&self, register: Register) -> bool {
        match self {
            Self::WindowsX64 => {
                // Windows x64: XMM0-XMM5 are caller-saved, XMM6-XMM15 are callee-saved
                matches!(register, 
                    Register::XMM0 | Register::XMM1 | Register::XMM2 | Register::XMM3 |
                    Register::XMM4 | Register::XMM5)
            },
            Self::SystemV => {
                // System V: All XMM registers are caller-saved
                matches!(register,
                    Register::XMM0 | Register::XMM1 | Register::XMM2 | Register::XMM3 |
                    Register::XMM4 | Register::XMM5 | Register::XMM6 | Register::XMM7 |
                    Register::XMM8 | Register::XMM9 | Register::XMM10 | Register::XMM11 |
                    Register::XMM12 | Register::XMM13 | Register::XMM14 | Register::XMM15)
            }
        }
    }

    /// Get the nth floating-point parameter register for this ABI
    pub fn get_float_param_register(&self, n: usize) -> Option<Register> {
        self.get_float_parameter_registers().get(n).copied()
    }

    /// Check if a register is a floating-point parameter register for this ABI
    pub fn is_float_param_register(&self, register: Register) -> bool {
        self.get_float_parameter_registers().contains(&register)
    }

    /// Get the maximum number of floating-point parameters that can be passed in registers
    pub fn max_float_param_registers(&self) -> usize {
        match self {
            Self::WindowsX64 => 4,  // XMM0-XMM3
            Self::SystemV => 8,     // XMM0-XMM7
        }
    }

    /// Get all floating-point register save locations for the ABI (for saving across function calls)
    pub fn get_float_save_registers(&self) -> Vec<Register> {
        match self {
            Self::WindowsX64 => {
                // Windows x64: XMM6-XMM15 are callee-saved (need to be preserved)
                vec![
                    Register::XMM6, Register::XMM7, Register::XMM8, Register::XMM9,
                    Register::XMM10, Register::XMM11, Register::XMM12, Register::XMM13,
                    Register::XMM14, Register::XMM15
                ]
            },
            Self::SystemV => {
                // System V: No XMM registers are callee-saved, all are caller-saved
                vec![]
            }
        }
    }

    /// Get the correct register for the nth floating-point argument
    pub fn float_argument_register(&self, index: usize) -> Option<Register> {
        self.get_float_param_register(index)
    }

    /// Get the correct register for floating-point return value for single precision
    pub fn float_return_register_single(&self) -> Register {
        Register::XMM0
    }

    /// Get the correct register for floating-point return value for double precision
    pub fn float_return_register_double(&self) -> Register {
        Register::XMM0
    }

    /// Get the correct register for pair of floating-point return values (for complex operations)
    pub fn float_return_registers_pair(&self) -> (Register, Register) {
        match self {
            Self::WindowsX64 => (Register::XMM0, Register::XMM1),
            Self::SystemV => (Register::XMM0, Register::XMM1),
        }
    }
}