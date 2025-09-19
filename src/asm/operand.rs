//! Assembly operands
use std::fmt;
use super::register::Register;

/// Assembly operands
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Memory(String),
    Label(String),
    /// Memory reference with base register, index register, scale, and displacement
    MemoryRef {
        base: Option<Register>,
        index: Option<Register>,
        scale: u8,
        displacement: i32,
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Register(reg) => write!(f, "{}", reg),
            Operand::Immediate(val) => write!(f, "{}", val),
            Operand::Memory(addr) => write!(f, "[{}]", addr),
            Operand::Label(label) => write!(f, "{}", label),
            Operand::MemoryRef { base, index, scale, displacement } => {
                write!(f, "[")?;
                if let Some(reg) = base {
                    write!(f, "{}", reg)?;
                }
                if let Some(reg) = index {
                    if base.is_some() {
                        write!(f, "+")?;
                    }
                    write!(f, "{}", reg)?;
                    if *scale != 1 {
                        write!(f, "*{}", scale)?;
                    }
                }
                if *displacement != 0 {
                    if (*displacement > 0 && (base.is_some() || index.is_some())) || 
                       (*displacement < 0) {
                        write!(f, "{:+}", displacement)?;
                    } else {
                        write!(f, "{}", displacement)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Operand {
    /// Create a memory reference operand
    pub fn mem_ref(base: Option<Register>, index: Option<Register>, scale: u8, displacement: i32) -> Self {
        Operand::MemoryRef {
            base,
            index,
            scale,
            displacement,
        }
    }
    
    /// Create a register operand
    pub fn reg(r: Register) -> Self {
        Operand::Register(r)
    }
    
    /// Create an immediate operand
    pub fn imm(val: i64) -> Self {
        Operand::Immediate(val)
    }
    
    /// Create a label operand
    pub fn label(name: &str) -> Self {
        Operand::Label(name.to_string())
    }
    
    /// Create a memory operand
    pub fn mem(addr: &str) -> Self {
        Operand::Memory(addr.to_string())
    }
}