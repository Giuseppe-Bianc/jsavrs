use crate::asm::register::*;
use std::fmt;

/// Operando di memoria
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    /// Registro base
    pub base: Option<GPRegister64>,
    /// Registro indice
    pub index: Option<GPRegister64>,
    /// Scala (1, 2, 4, 8)
    pub scale: u8,
    /// Displacement
    pub displacement: i32,
    /// Dimensione dell'operando in byte
    pub size: usize,
}

impl MemoryOperand {
    pub fn new(base: Option<GPRegister64>) -> Self {
        Self { base, index: None, scale: 1, displacement: 0, size: 8 }
    }

    pub fn with_displacement(mut self, disp: i32) -> Self {
        self.displacement = disp;
        self
    }

    pub fn with_index(mut self, index: GPRegister64, scale: u8) -> Self {
        self.index = Some(index);
        self.scale = scale;
        self
    }

    pub fn with_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
}


impl fmt::Display for MemoryOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determina il prefisso di dimensione
        let size_prefix = match self.size {
            1 => "BYTE PTR ",
            2 => "WORD PTR ",
            4 => "DWORD PTR ",
            8 => "QWORD PTR ",
            16 => "XMMWORD PTR ",
            32 => "YMMWORD PTR ",
            _ => "",
        };

        write!(f, "{}", size_prefix)?;
        write!(f, "[")?;

        let mut has_component = false;

        // Base register
        if let Some(base) = &self.base {
            write!(f, "{}", base)?;
            has_component = true;
        }

        // Index register con scala
        if let Some(index) = &self.index {
            if has_component {
                write!(f, " + ")?;
            }
            write!(f, "{}", index)?;
            if self.scale != 1 {
                write!(f, "*{}", self.scale)?;
            }
            has_component = true;
        }

        // Displacement
        if self.displacement != 0 {
            if has_component {
                if self.displacement > 0 {
                    write!(f, " + {}", self.displacement)?;
                } else {
                    write!(f, " - {}", -self.displacement)?;
                }
            } else {
                write!(f, "{}", self.displacement)?;
            }
        } else if !has_component {
            write!(f, "0")?;
        }

        write!(f, "]")
    }
}