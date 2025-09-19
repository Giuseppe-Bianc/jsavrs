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
    
    /// Check if this operand is a register
    pub fn is_register(&self) -> bool {
        matches!(self, Operand::Register(_))
    }
    
    /// Check if this operand is an immediate
    pub fn is_immediate(&self) -> bool {
        matches!(self, Operand::Immediate(_))
    }
    
    /// Check if this operand is a memory reference
    pub fn is_memory(&self) -> bool {
        matches!(self, Operand::Memory(_) | Operand::MemoryRef { .. })
    }
    
    /// Check if this operand is a label
    pub fn is_label(&self) -> bool {
        matches!(self, Operand::Label(_))
    }
    
    /// Get the register if this operand is a register
    pub fn as_register(&self) -> Option<&Register> {
        match self {
            Operand::Register(reg) => Some(reg),
            _ => None,
        }
    }
    
    /// Get the immediate value if this operand is an immediate
    pub fn as_immediate(&self) -> Option<i64> {
        match self {
            Operand::Immediate(val) => Some(*val),
            _ => None,
        }
    }
    
    /// Get the label if this operand is a label
    pub fn as_label(&self) -> Option<&str> {
        match self {
            Operand::Label(label) => Some(label),
            _ => None,
        }
    }
    
    /// Get the memory address if this operand is a memory operand
    pub fn as_memory(&self) -> Option<&str> {
        match self {
            Operand::Memory(addr) => Some(addr),
            _ => None,
        }
    }
    
    /// Get memory reference components if this operand is a memory reference
    pub fn as_memory_ref(&self) -> Option<(&Option<Register>, &Option<Register>, &u8, &i32)> {
        match self {
            Operand::MemoryRef { base, index, scale, displacement } => {
                Some((base, index, scale, displacement))
            }
            _ => None,
        }
    }
    
    /// Create a memory reference with only a base register
    pub fn mem_base(base: Register) -> Self {
        Operand::mem_ref(Some(base), None, 1, 0)
    }
    
    /// Create a memory reference with base register and displacement
    pub fn mem_base_disp(base: Register, displacement: i32) -> Self {
        Operand::mem_ref(Some(base), None, 1, displacement)
    }
    
    /// Create a memory reference with base and index registers
    pub fn mem_base_index(base: Register, index: Register) -> Self {
        Operand::mem_ref(Some(base), Some(index), 1, 0)
    }
    
    /// Create a memory reference with base, index, and scale
    pub fn mem_base_index_scale(base: Register, index: Register, scale: u8) -> Self {
        Operand::mem_ref(Some(base), Some(index), scale, 0)
    }
    
    /// Create a memory reference with base, index, scale, and displacement
    pub fn mem_base_index_scale_disp(base: Register, index: Register, scale: u8, displacement: i32) -> Self {
        Operand::mem_ref(Some(base), Some(index), scale, displacement)
    }
    
    /// Create a RIP-relative memory reference
    pub fn rip_relative(displacement: i32) -> Self {
        Operand::mem_ref(None, None, 1, displacement)
    }
}