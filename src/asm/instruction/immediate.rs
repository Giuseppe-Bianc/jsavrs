/// Tipi di valori immediati
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Immediate {
    /// Immediato a 8-bit con segno
    Imm8(i8),
    /// Immediato a 8-bit senza segno
    Imm8u(u8),
    /// Immediato a 16-bit con segno
    Imm16(i16),
    /// Immediato a 16-bit senza segno
    Imm16u(u16),
    /// Immediato a 32-bit con segno
    Imm32(i32),
    /// Immediato a 32-bit senza segno
    Imm32u(u32),
    /// Immediato a 64-bit con segno
    Imm64(i64),
    /// Immediato a 64-bit senza segno
    Imm64u(u64),
}

impl Immediate {
    /// Ottiene la dimensione in bit dell'immediato
    pub fn size_bits(&self) -> usize {
        match self {
            Self::Imm8(_) | Self::Imm8u(_) => 8,
            Self::Imm16(_) | Self::Imm16u(_) => 16,
            Self::Imm32(_) | Self::Imm32u(_) => 32,
            Self::Imm64(_) | Self::Imm64u(_) => 64,
        }
    }

    /// Ottiene la dimensione in byte dell'immediato
    pub fn size_bytes(&self) -> usize {
        self.size_bits() / 8
    }

    /// Converte l'immediato a i64
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::Imm8(v) => *v as i64,
            Self::Imm8u(v) => *v as i64,
            Self::Imm16(v) => *v as i64,
            Self::Imm16u(v) => *v as i64,
            Self::Imm32(v) => *v as i64,
            Self::Imm32u(v) => *v as i64,
            Self::Imm64(v) => *v,
            Self::Imm64u(v) => *v as i64,
        }
    }

    /// Converte l'immediato a u64
    pub fn as_u64(&self) -> u64 {
        match self {
            Self::Imm8(v) => *v as u64,
            Self::Imm8u(v) => *v as u64,
            Self::Imm16(v) => *v as u64,
            Self::Imm16u(v) => *v as u64,
            Self::Imm32(v) => *v as u64,
            Self::Imm32u(v) => *v as u64,
            Self::Imm64(v) => *v as u64,
            Self::Imm64u(v) => *v,
        }
    }

    /// Verifica se l'immediato è con segno
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Imm8(_) | Self::Imm16(_) | Self::Imm32(_) | Self::Imm64(_))
    }

    /// Verifica se l'immediato può essere rappresentato in una dimensione più piccola
    pub fn fits_in(&self, bits: usize) -> bool {
        match bits {
            8 => {
                let val = self.as_i64();
                val >= i8::MIN as i64 && val <= i8::MAX as i64
            }
            16 => {
                let val = self.as_i64();
                val >= i16::MIN as i64 && val <= i16::MAX as i64
            }
            32 => {
                let val = self.as_i64();
                val >= i32::MIN as i64 && val <= i32::MAX as i64
            }
            64 => true,
            _ => false,
        }
    }
}

impl From<i8> for Immediate {
    fn from(v: i8) -> Self {
        Self::Imm8(v)
    }
}

impl From<u8> for Immediate {
    fn from(v: u8) -> Self {
        Self::Imm8u(v)
    }
}

impl From<i16> for Immediate {
    fn from(v: i16) -> Self {
        Self::Imm16(v)
    }
}

impl From<u16> for Immediate {
    fn from(v: u16) -> Self {
        Self::Imm16u(v)
    }
}

impl From<i32> for Immediate {
    fn from(v: i32) -> Self {
        Self::Imm32(v)
    }
}

impl From<u32> for Immediate {
    fn from(v: u32) -> Self {
        Self::Imm32u(v)
    }
}

impl From<i64> for Immediate {
    fn from(v: i64) -> Self {
        Self::Imm64(v)
    }
}

impl From<u64> for Immediate {
    fn from(v: u64) -> Self {
        Self::Imm64u(v)
    }
}

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imm8(v) => write!(f, "{}", v),
            Self::Imm8u(v) => write!(f, "0x{:02x}", v),
            Self::Imm16(v) => write!(f, "{}", v),
            Self::Imm16u(v) => write!(f, "0x{:04x}", v),
            Self::Imm32(v) => write!(f, "{}", v),
            Self::Imm32u(v) => write!(f, "0x{:08x}", v),
            Self::Imm64(v) => write!(f, "{}", v),
            Self::Imm64u(v) => write!(f, "0x{:016x}", v),
        }
    }
}