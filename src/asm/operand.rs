//! # Operand Representation
//!
//! Defines operand types for x86-64 assembly instructions.
use crate::asm::register::Register;
use std::fmt;

/// Immediate value with size information
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImmediateValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(u32), // IEEE 754 binary representation
    Float64(u64), // IEEE 754 binary representation
}

impl fmt::Display for ImmediateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImmediateValue::Int8(v) => write!(f, "{}", v),
            ImmediateValue::Int16(v) => write!(f, "{}", v),
            ImmediateValue::Int32(v) => write!(f, "{}", v),
            ImmediateValue::Int64(v) => write!(f, "{}", v),
            ImmediateValue::UInt8(v) => write!(f, "{}", v),
            ImmediateValue::UInt16(v) => write!(f, "{}", v),
            ImmediateValue::UInt32(v) => write!(f, "{}", v),
            ImmediateValue::UInt64(v) => write!(f, "{}", v),
            ImmediateValue::Float32(v) => write!(f, "{}", v),
            ImmediateValue::Float64(v) => write!(f, "{}", v),
        }
    }
}

impl From<i8> for ImmediateValue {
    fn from(value: i8) -> Self {
        ImmediateValue::Int8(value)
    }
}

impl From<i16> for ImmediateValue {
    fn from(value: i16) -> Self {
        ImmediateValue::Int16(value)
    }
}

impl From<i32> for ImmediateValue {
    fn from(value: i32) -> Self {
        ImmediateValue::Int32(value)
    }
}

impl From<i64> for ImmediateValue {
    fn from(value: i64) -> Self {
        ImmediateValue::Int64(value)
    }
}

impl From<u8> for ImmediateValue {
    fn from(value: u8) -> Self {
        ImmediateValue::UInt8(value)
    }
}

impl From<u16> for ImmediateValue {
    fn from(value: u16) -> Self {
        ImmediateValue::UInt16(value)
    }
}

impl From<u32> for ImmediateValue {
    fn from(value: u32) -> Self {
        ImmediateValue::UInt32(value)
    }
}

impl From<u64> for ImmediateValue {
    fn from(value: u64) -> Self {
        ImmediateValue::UInt64(value)
    }
}

/// Represents different types of operands in x86-64 instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// Register operand (e.g., RAX, RBX, XMM0)
    Register(Register),
    /// Immediate value (e.g., 42, 0x100)
    Immediate(ImmediateValue),
    /// Memory operand with base register, displacement, and optional index/scale
    Memory(MemoryOperand),
    /// Label or symbol reference
    Label(String),
    /// Address operand
    Address(u64),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "{}", reg),
            Operand::Immediate(value) => {
                write!(f, "{}", value)
            }
            Operand::Memory(mem_op) => {
                write!(f, "{}", mem_op)
            }
            Operand::Label(label) => write!(f, "{}", label),
            Operand::Address(addr) => write!(f, "0x{:x}", addr),
        }
    }
}

impl fmt::Display for MemoryOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        if let Some(base) = self.base {
            write!(f, "{}", base)?;
        }

        if let Some(index) = self.index {
            if self.base.is_some() {
                write!(f, "+{}", index)?;
            } else {
                write!(f, "{}", index)?;
            }
            if self.scale != 1 {
                write!(f, "*{}", self.scale)?;
            }
        }

        if self.displacement != 0 {
            if self.displacement > 0 {
                if self.base.is_some() || self.index.is_some() {
                    write!(f, "+{}", self.displacement)?;
                } else {
                    write!(f, "{}", self.displacement)?;
                }
            } else {
                write!(f, "{}", self.displacement)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}

/// Represents memory operands with addressing modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    /// Base register (e.g., RBP, RSP)
    pub base: Option<Register>,
    /// Index register (e.g., RAX, RBX) - for scaled indexing
    pub index: Option<Register>,
    /// Scale factor (1, 2, 4, or 8)
    pub scale: u8,
    /// Displacement/offset value
    pub displacement: i32,
}

impl MemoryOperand {
    /// Creates a new memory operand with base register and displacement
    pub fn new(base: Register, displacement: i32) -> Self {
        MemoryOperand {
            base: Some(base),
            index: None,
            scale: 1,
            displacement,
        }
    }

    /// Creates a memory operand with base, index, scale, and displacement
    pub fn with_index(base: Register, index: Register, scale: u8, displacement: i32) -> Self {
        MemoryOperand {
            base: Some(base),
            index: Some(index),
            scale,
            displacement,
        }
    }

    /// Creates a memory operand with displacement only (absolute addressing)
    pub fn absolute(displacement: i32) -> Self {
        MemoryOperand {
            base: None,
            index: None,
            scale: 1,
            displacement,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::register::{GPRegister, Register};

    #[test]
    fn test_register_operand() {
        let reg_rax = Register::GP(GPRegister::RAX);
        let op = Operand::Register(reg_rax);

        assert_eq!(op, Operand::Register(reg_rax));
    }

    #[test]
    fn test_immediate_operand() {
        let op = Operand::Immediate(ImmediateValue::Int32(42));
        assert_eq!(op, Operand::Immediate(ImmediateValue::Int32(42)));
    }

    #[test]
    fn test_memory_operand() {
        let reg_rbp = Register::GP(GPRegister::RBP);
        let mem_op = MemoryOperand::new(reg_rbp, 8);

        assert_eq!(mem_op.base, Some(reg_rbp));
        assert_eq!(mem_op.displacement, 8);
        assert_eq!(mem_op.index, None);
        assert_eq!(mem_op.scale, 1);
    }
}
