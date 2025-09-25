//! Floating-point validation functions

use super::register::Register;
use super::operand::Operand;

/// Validation functions for floating-point operations
pub struct FloatingPointValidator;

impl FloatingPointValidator {
    /// Validate that a register is a valid floating-point register
    pub fn validate_fp_register(register: &Register) -> Result<(), String> {
        match register {
            Register::XMM0 | Register::XMM1 | Register::XMM2 | Register::XMM3 |
            Register::XMM4 | Register::XMM5 | Register::XMM6 | Register::XMM7 |
            Register::XMM8 | Register::XMM9 | Register::XMM10 | Register::XMM11 |
            Register::XMM12 | Register::XMM13 | Register::XMM14 | Register::XMM15 |
            Register::YMM0 | Register::YMM1 | Register::YMM2 | Register::YMM3 |
            Register::YMM4 | Register::YMM5 | Register::YMM6 | Register::YMM7 |
            Register::YMM8 | Register::YMM9 | Register::YMM10 | Register::YMM11 |
            Register::YMM12 | Register::YMM13 | Register::YMM14 | Register::YMM15 |
            Register::ZMM0 | Register::ZMM1 | Register::ZMM2 | Register::ZMM3 |
            Register::ZMM4 | Register::ZMM5 | Register::ZMM6 | Register::ZMM7 |
            Register::ZMM8 | Register::ZMM9 | Register::ZMM10 | Register::ZMM11 |
            Register::ZMM12 | Register::ZMM13 | Register::ZMM14 | Register::ZMM15 => Ok(()),
            _ => Err(format!("Register {:?} is not a valid floating-point register", register))
        }
    }

    /// Validate that an operand is valid for floating-point operations
    pub fn validate_fp_operand(operand: &Operand) -> Result<(), String> {
        match operand {
            Operand::Register(reg) => Self::validate_fp_register(reg),
            Operand::FloatImmediate(_) => Ok(()),
            Operand::MemoryRef { base, index, scale, displacement: _ } => {
                if let Some(base_reg) = base {
                    // Base can be general-purpose register for memory addressing
                    Self::validate_general_purpose_register(base_reg)?;
                }
                if let Some(index_reg) = index {
                    // Index can be general-purpose register
                    Self::validate_general_purpose_register(index_reg)?;
                }
                if ![1, 2, 4, 8].contains(scale) {
                    return Err(format!("Invalid scale factor: {}", scale));
                }
                Ok(())
            }
            _ => Err("Invalid operand type for floating-point operation".to_string())
        }
    }

    /// Validate that a register is a general-purpose register (for memory addressing)
    fn validate_general_purpose_register(register: &Register) -> Result<(), String> {
        match register {
            Register::RAX | Register::RBX | Register::RCX | Register::RDX |
            Register::RSI | Register::RDI | Register::RBP | Register::RSP |
            Register::R8 | Register::R9 | Register::R10 | Register::R11 |
            Register::R12 | Register::R13 | Register::R14 | Register::R15 => Ok(()),
            _ => Err(format!("Register {:?} cannot be used for memory addressing", register))
        }
    }

    /// Validate floating-point immediate value
    pub fn validate_fp_immediate(value: f64) -> Result<(), String> {
        if value.is_finite() || value.is_nan() || value.is_infinite() {
            Ok(())
        } else {
            Err("Invalid floating-point immediate value".to_string())
        }
    }
}