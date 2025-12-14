use super::immediate::Immediate;
use super::memory_operand::MemoryOperand;
use crate::asm::register::{
    GPRegister8, GPRegister16, GPRegister32, GPRegister64, X86Register, XMMRegister, YMMRegister,
};
use std::fmt;

/// An operand used by `x86_64` instructions.
///
/// Variants:
/// - `Register(X86Register)`: a hardware register (GP/XMM/YMM, etc.).
/// - `Immediate(Immediate)`: an immediate constant of a specific width.
/// - `Memory(MemoryOperand)`: a memory addressing expression.
/// - `Label(String)`: a symbolic label used by jumps and calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// Register operand.
    Register(X86Register),
    /// Immediate constant operand.
    Immediate(Immediate),
    /// Memory reference operand.
    Memory(MemoryOperand),
    /// Label operand (for branches and calls).
    Label(String),
}

impl Operand {
    /// Create a 64-bit general-purpose register operand.
    ///
    /// Inputs: `reg: GPRegister64`.
    /// Outputs: `Operand::Register(X86Register::GP64(reg))`.
    /// Side effects: none.
    #[must_use]
    pub const fn reg64(reg: GPRegister64) -> Self {
        Self::Register(X86Register::GP64(reg))
    }

    /// Create a 32-bit general-purpose register operand.
    #[must_use]
    pub const fn reg32(reg: GPRegister32) -> Self {
        Self::Register(X86Register::GP32(reg))
    }

    /// Create a 16-bit general-purpose register operand.
    #[must_use]
    pub const fn reg16(reg: GPRegister16) -> Self {
        Self::Register(X86Register::GP16(reg))
    }

    /// Create an 8-bit general-purpose register operand.
    #[must_use]
    pub const fn reg8(reg: GPRegister8) -> Self {
        Self::Register(X86Register::GP8(reg))
    }

    /// Create an XMM register operand.
    #[must_use]
    pub const fn xmm(reg: XMMRegister) -> Self {
        Self::Register(X86Register::Xmm(reg))
    }

    /// Create a YMM register operand.
    #[must_use]
    pub const fn ymm(reg: YMMRegister) -> Self {
        Self::Register(X86Register::Ymm(reg))
    }

    /// Create an 8-bit immediate operand.
    ///
    /// Inputs: `val: i8`.
    /// Outputs: `Operand::Immediate(Immediate::Imm8(val))`.
    #[must_use]
    pub const fn imm8(val: i8) -> Self {
        Self::Immediate(Immediate::Imm8(val))
    }

    /// Create a 16-bit immediate operand.
    #[must_use]
    pub const fn imm16(val: i16) -> Self {
        Self::Immediate(Immediate::Imm16(val))
    }

    /// Create a 32-bit immediate operand.
    #[must_use]
    pub const fn imm32(val: i32) -> Self {
        Self::Immediate(Immediate::Imm32(val))
    }

    /// Create a 64-bit immediate operand.
    #[must_use]
    pub const fn imm64(val: i64) -> Self {
        Self::Immediate(Immediate::Imm64(val))
    }

    /// Create a memory operand with the given base register.
    ///
    /// Inputs: `base: GPRegister64`.
    /// Outputs: `Operand::Memory(MemoryOperand::new(Some(base)))`.
    #[must_use]
    pub const fn mem(base: GPRegister64) -> Self {
        Self::Memory(MemoryOperand::new(Some(base)))
    }

    /// Create a memory operand with base and displacement (base + disp).
    #[must_use]
    pub const fn mem_disp(base: GPRegister64, disp: i32) -> Self {
        Self::Memory(MemoryOperand::new(Some(base)).with_displacement(disp))
    }

    /// Create a label operand from any type convertible to `String`.
    ///
    /// Inputs: `name: impl Into<String>`.
    /// Outputs: `Operand::Label(name.into())`.
    pub fn label(name: impl Into<String>) -> Self {
        Self::Label(name.into())
    }

    /// Returns true if the operand is a register.
    #[must_use]
    pub const fn is_register(&self) -> bool {
        matches!(self, Self::Register(_))
    }

    /// Returns true if the operand is an immediate.
    #[must_use]
    pub const fn is_immediate(&self) -> bool {
        matches!(self, Self::Immediate(_))
    }

    /// Returns true if the operand is a memory reference.
    #[must_use]
    pub const fn is_memory(&self) -> bool {
        matches!(self, Self::Memory(_))
    }

    /// Returns true if the operand is a label.
    #[must_use]
    pub const fn is_label(&self) -> bool {
        matches!(self, Self::Label(_))
    }
}

/// Convert an `i8` into an `Operand::Immediate(Imm8)`.
///
/// Inputs: `v: i8` — value to wrap.
/// Outputs: `Operand::Immediate(Immediate::Imm8(v))`.
/// Side effects: none.
impl From<i8> for Operand {
    fn from(v: i8) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert a `u8` into an `Operand::Immediate(Imm8u)`.
///
/// Inputs: `v: u8` — value to wrap.
/// Outputs: `Operand::Immediate(Immediate::Imm8u(v))`.
/// Side effects: none.
impl From<u8> for Operand {
    fn from(v: u8) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert an `i16` into an `Operand::Immediate(Imm16)`.
impl From<i16> for Operand {
    fn from(v: i16) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert a `u16` into an `Operand::Immediate(Imm16u)`.
impl From<u16> for Operand {
    fn from(v: u16) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert an `i32` into an `Operand::Immediate(Imm32)`.
impl From<i32> for Operand {
    fn from(v: i32) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert a `u32` into an `Operand::Immediate(Imm32u)`.
impl From<u32> for Operand {
    fn from(v: u32) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert an `i64` into an `Operand::Immediate(Imm64)`.
impl From<i64> for Operand {
    fn from(v: i64) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert a `u64` into an `Operand::Immediate(Imm64u)`.
impl From<u64> for Operand {
    fn from(v: u64) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

/// Convert an `X86Register` directly into an `Operand::Register`.
///
/// Inputs: `reg: X86Register`.
/// Outputs: `Operand::Register(reg)`.
/// Side effects: none.
impl From<X86Register> for Operand {
    fn from(reg: X86Register) -> Self {
        Self::Register(reg)
    }
}

impl fmt::Display for Operand {
    /// Format the operand for human-readable assembly output.
    ///
    /// Registers, immediates, memory operands and labels are delegated to
    /// their respective `Display` implementations.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{reg}"),
            Self::Immediate(imm) => write!(f, "{imm}"),
            Self::Memory(mem) => write!(f, "{mem}"),
            Self::Label(label) => write!(f, "{label}"),
        }
    }
}
