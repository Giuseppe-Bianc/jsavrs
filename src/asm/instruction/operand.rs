use std::fmt;
use crate::asm::register::*;
use super::immediate::Immediate;
use super::memory_operand::MemoryOperand;

/// Operando per le istruzioni x86_64
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// Registro
    Register(X86Register),
    /// Valore immediato
    Immediate(Immediate),
    /// Riferimento a memoria
    Memory(MemoryOperand),
    /// Etichetta (per jump e call)
    Label(String),
}


impl Operand {
    /// Crea un operando registro da un registro a 64-bit
    pub fn reg64(reg: GPRegister64) -> Self {
        Self::Register(X86Register::GP64(reg))
    }

    /// Crea un operando registro da un registro a 32-bit
    pub fn reg32(reg: GPRegister32) -> Self {
        Self::Register(X86Register::GP32(reg))
    }

    /// Crea un operando registro da un registro a 16-bit
    pub fn reg16(reg: GPRegister16) -> Self {
        Self::Register(X86Register::GP16(reg))
    }

    /// Crea un operando registro da un registro a 8-bit
    pub fn reg8(reg: GPRegister8) -> Self {
        Self::Register(X86Register::GP8(reg))
    }

    /// Crea un operando XMM
    pub fn xmm(reg: XMMRegister) -> Self {
        Self::Register(X86Register::Xmm(reg))
    }

    /// Crea un operando YMM
    pub fn ymm(reg: YMMRegister) -> Self {
        Self::Register(X86Register::Ymm(reg))
    }

    /// Crea un operando immediato a 8-bit
    pub fn imm8(val: i8) -> Self {
        Self::Immediate(Immediate::Imm8(val))
    }

    /// Crea un operando immediato a 16-bit
    pub fn imm16(val: i16) -> Self {
        Self::Immediate(Immediate::Imm16(val))
    }

    /// Crea un operando immediato a 32-bit
    pub fn imm32(val: i32) -> Self {
        Self::Immediate(Immediate::Imm32(val))
    }

    /// Crea un operando immediato a 64-bit
    pub fn imm64(val: i64) -> Self {
        Self::Immediate(Immediate::Imm64(val))
    }

    /// Crea un operando memoria semplice (base)
    pub fn mem(base: GPRegister64) -> Self {
        Self::Memory(MemoryOperand::new(Some(base)))
    }

    /// Crea un operando memoria con displacement (base + disp)
    pub fn mem_disp(base: GPRegister64, disp: i32) -> Self {
        Self::Memory(MemoryOperand::new(Some(base)).with_displacement(disp))
    }

    /// Crea un operando etichetta
    pub fn label(name: impl Into<String>) -> Self {
        Self::Label(name.into())
    }

    /// Verifica se l'operando è un registro
    pub fn is_register(&self) -> bool {
        matches!(self, Self::Register(_))
    }

    /// Verifica se l'operando è un immediato
    pub fn is_immediate(&self) -> bool {
        matches!(self, Self::Immediate(_))
    }

    /// Verifica se l'operando è memoria
    pub fn is_memory(&self) -> bool {
        matches!(self, Self::Memory(_))
    }

    /// Verifica se l'operando è un'etichetta
    pub fn is_label(&self) -> bool {
        matches!(self, Self::Label(_))
    }
}

impl From<i8> for Operand {
    fn from(v: i8) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<u8> for Operand {
    fn from(v: u8) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<i16> for Operand {
    fn from(v: i16) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<u16> for Operand {
    fn from(v: u16) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<i32> for Operand {
    fn from(v: i32) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<u32> for Operand {
    fn from(v: u32) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<i64> for Operand {
    fn from(v: i64) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<u64> for Operand {
    fn from(v: u64) -> Self {
        Self::Immediate(Immediate::from(v))
    }
}

impl From<X86Register> for Operand {
    fn from(reg: X86Register) -> Self {
        Self::Register(reg)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{}", reg),
            Self::Immediate(imm) => write!(f, "{}", imm),
            Self::Memory(mem) => write!(f, "{}", mem),
            Self::Label(label) => write!(f, "{}", label),
        }
    }
}