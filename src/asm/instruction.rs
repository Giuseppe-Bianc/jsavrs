#![allow(dead_code)]

use super::register::*;

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

/// Enumerazione principale per tutte le istruzioni x86_64
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // === Istruzioni Aritmetiche ===
    Add { dest: Operand, src: Operand },
    Sub { dest: Operand, src: Operand },
    Mul { src: Operand },
    Imul { dest: Option<Operand>, src1: Operand, src2: Option<Operand> },
    Div { src: Operand },
    Idiv { src: Operand },
    Inc { dest: Operand },
    Dec { dest: Operand },
    Neg { dest: Operand },
    Adc { dest: Operand, src: Operand },
    Sbb { dest: Operand, src: Operand },

    // === Istruzioni Logiche ===
    And { dest: Operand, src: Operand },
    Or { dest: Operand, src: Operand },
    Xor { dest: Operand, src: Operand },
    Not { dest: Operand },
    Test { op1: Operand, op2: Operand },

    // === Istruzioni di Shift e Rotate ===
    Shl { dest: Operand, count: Operand },
    Shr { dest: Operand, count: Operand },
    Sar { dest: Operand, count: Operand },
    Sal { dest: Operand, count: Operand },
    Rol { dest: Operand, count: Operand },
    Ror { dest: Operand, count: Operand },
    Rcl { dest: Operand, count: Operand },
    Rcr { dest: Operand, count: Operand },

    // === Istruzioni di Movimento ===
    Mov { dest: Operand, src: Operand },
    Movsx { dest: Operand, src: Operand },
    Movsxd { dest: Operand, src: Operand },
    Movzx { dest: Operand, src: Operand },
    Lea { dest: Operand, src: Operand },
    Push { src: Operand },
    Pop { dest: Operand },
    Xchg { op1: Operand, op2: Operand },

    // === Istruzioni di Confronto ===
    Cmp { op1: Operand, op2: Operand },

    // === Istruzioni di Salto ===
    Jmp { target: Operand },
    Je { target: Operand },  // Jump if Equal (ZF=1)
    Jne { target: Operand }, // Jump if Not Equal (ZF=0)
    Jz { target: Operand },  // Jump if Zero (ZF=1)
    Jnz { target: Operand }, // Jump if Not Zero (ZF=0)
    Jg { target: Operand },  // Jump if Greater (signed)
    Jge { target: Operand }, // Jump if Greater or Equal (signed)
    Jl { target: Operand },  // Jump if Less (signed)
    Jle { target: Operand }, // Jump if Less or Equal (signed)
    Ja { target: Operand },  // Jump if Above (unsigned)
    Jae { target: Operand }, // Jump if Above or Equal (unsigned)
    Jb { target: Operand },  // Jump if Below (unsigned)
    Jbe { target: Operand }, // Jump if Below or Equal (unsigned)
    Js { target: Operand },  // Jump if Sign (SF=1)
    Jns { target: Operand }, // Jump if Not Sign (SF=0)
    Jo { target: Operand },  // Jump if Overflow (OF=1)
    Jno { target: Operand }, // Jump if Not Overflow (OF=0)
    Jp { target: Operand },  // Jump if Parity (PF=1)
    Jnp { target: Operand }, // Jump if Not Parity (PF=0)

    // === Istruzioni di Call e Return ===
    Call { target: Operand },
    Ret,
    RetImm { imm: u16 },

    // === Istruzioni SSE/AVX - Movimento ===
    Movaps { dest: Operand, src: Operand },
    Movapd { dest: Operand, src: Operand },
    Movups { dest: Operand, src: Operand },
    Movupd { dest: Operand, src: Operand },
    Movss { dest: Operand, src: Operand },
    Movsd { dest: Operand, src: Operand },
    Movdqa { dest: Operand, src: Operand },
    Movdqu { dest: Operand, src: Operand },

    // === Istruzioni SSE/AVX - Aritmetiche ===
    Addps { dest: Operand, src: Operand },
    Addpd { dest: Operand, src: Operand },
    Addss { dest: Operand, src: Operand },
    Addsd { dest: Operand, src: Operand },
    Subps { dest: Operand, src: Operand },
    Subpd { dest: Operand, src: Operand },
    Subss { dest: Operand, src: Operand },
    Subsd { dest: Operand, src: Operand },
    Mulps { dest: Operand, src: Operand },
    Mulpd { dest: Operand, src: Operand },
    Mulss { dest: Operand, src: Operand },
    Mulsd { dest: Operand, src: Operand },
    Divps { dest: Operand, src: Operand },
    Divpd { dest: Operand, src: Operand },
    Divss { dest: Operand, src: Operand },
    Divsd { dest: Operand, src: Operand },

    // === Istruzioni SSE/AVX - Logiche ===
    Andps { dest: Operand, src: Operand },
    Andpd { dest: Operand, src: Operand },
    Andnps { dest: Operand, src: Operand },
    Andnpd { dest: Operand, src: Operand },
    Orps { dest: Operand, src: Operand },
    Orpd { dest: Operand, src: Operand },
    Xorps { dest: Operand, src: Operand },
    Xorpd { dest: Operand, src: Operand },

    // === Istruzioni SSE/AVX - Conversione ===
    Cvtss2sd { dest: Operand, src: Operand },
    Cvtsd2ss { dest: Operand, src: Operand },
    Cvttss2si { dest: Operand, src: Operand },
    Cvttsd2si { dest: Operand, src: Operand },
    Cvtsi2ss { dest: Operand, src: Operand },
    Cvtsi2sd { dest: Operand, src: Operand },

    // === Istruzioni AVX ===
    Vaddps { dest: Operand, src1: Operand, src2: Operand },
    Vaddpd { dest: Operand, src1: Operand, src2: Operand },
    Vaddss { dest: Operand, src1: Operand, src2: Operand },
    Vaddsd { dest: Operand, src1: Operand, src2: Operand },
    Vsubps { dest: Operand, src1: Operand, src2: Operand },
    Vsubpd { dest: Operand, src1: Operand, src2: Operand },
    Vmulps { dest: Operand, src1: Operand, src2: Operand },
    Vmulpd { dest: Operand, src1: Operand, src2: Operand },
    Vdivps { dest: Operand, src1: Operand, src2: Operand },
    Vdivpd { dest: Operand, src1: Operand, src2: Operand },

    // === Istruzioni FPU x87 ===
    Fld { src: Operand },
    Fst { dest: Operand },
    Fstp { dest: Operand },
    Fadd { src: Option<Operand> },
    Faddp { src: Option<Operand> },
    Fsub { src: Option<Operand> },
    Fsubp { src: Option<Operand> },
    Fmul { src: Option<Operand> },
    Fmulp { src: Option<Operand> },
    Fdiv { src: Option<Operand> },
    Fdivp { src: Option<Operand> },

    // === Istruzioni di Bit Manipulation ===
    Bsf { dest: Operand, src: Operand },
    Bsr { dest: Operand, src: Operand },
    Bt { dest: Operand, src: Operand },
    Btc { dest: Operand, src: Operand },
    Btr { dest: Operand, src: Operand },
    Bts { dest: Operand, src: Operand },
    Popcnt { dest: Operand, src: Operand },
    Lzcnt { dest: Operand, src: Operand },
    Tzcnt { dest: Operand, src: Operand },

    // === Istruzioni CMOVcc (Conditional Move) ===
    Cmove { dest: Operand, src: Operand },
    Cmovne { dest: Operand, src: Operand },
    Cmovg { dest: Operand, src: Operand },
    Cmovge { dest: Operand, src: Operand },
    Cmovl { dest: Operand, src: Operand },
    Cmovle { dest: Operand, src: Operand },
    Cmova { dest: Operand, src: Operand },
    Cmovae { dest: Operand, src: Operand },
    Cmovb { dest: Operand, src: Operand },
    Cmovbe { dest: Operand, src: Operand },

    // === Istruzioni SETcc (Set Byte on Condition) ===
    Sete { dest: Operand },
    Setne { dest: Operand },
    Setg { dest: Operand },
    Setge { dest: Operand },
    Setl { dest: Operand },
    Setle { dest: Operand },
    Seta { dest: Operand },
    Setae { dest: Operand },
    Setb { dest: Operand },
    Setbe { dest: Operand },

    // === Istruzioni di Controllo ===
    Nop,
    Hlt,
    Cpuid,
    Pause,

    // === Istruzioni di String ===
    Movsb,
    Movsw,
    MovsdString,
    Movsq,
    Stosb,
    Stosw,
    Stosd,
    Stosq,

    // === Istruzioni Speciali ===
    Cqo, // Convert Quadword to Octword
    Cdq, // Convert Doubleword to Quadword
    Syscall,
    Sysret,
}

impl Instruction {
    /// Restituisce il mnemonic dell'istruzione
    pub fn mnemonic(&self) -> &str {
        match self {
            Self::Add { .. } => "add",
            Self::Sub { .. } => "sub",
            Self::Mul { .. } => "mul",
            Self::Imul { .. } => "imul",
            Self::Div { .. } => "div",
            Self::Idiv { .. } => "idiv",
            Self::Inc { .. } => "inc",
            Self::Dec { .. } => "dec",
            Self::Neg { .. } => "neg",
            Self::Adc { .. } => "adc",
            Self::Sbb { .. } => "sbb",
            Self::And { .. } => "and",
            Self::Or { .. } => "or",
            Self::Xor { .. } => "xor",
            Self::Not { .. } => "not",
            Self::Test { .. } => "test",
            Self::Shl { .. } => "shl",
            Self::Shr { .. } => "shr",
            Self::Sar { .. } => "sar",
            Self::Sal { .. } => "sal",
            Self::Rol { .. } => "rol",
            Self::Ror { .. } => "ror",
            Self::Rcl { .. } => "rcl",
            Self::Rcr { .. } => "rcr",
            Self::Mov { .. } => "mov",
            Self::Movsx { .. } => "movsx",
            Self::Movsxd { .. } => "movsxd",
            Self::Movzx { .. } => "movzx",
            Self::Lea { .. } => "lea",
            Self::Push { .. } => "push",
            Self::Pop { .. } => "pop",
            Self::Xchg { .. } => "xchg",
            Self::Cmp { .. } => "cmp",
            Self::Jmp { .. } => "jmp",
            Self::Je { .. } => "je",
            Self::Jne { .. } => "jne",
            Self::Jz { .. } => "jz",
            Self::Jnz { .. } => "jnz",
            Self::Jg { .. } => "jg",
            Self::Jge { .. } => "jge",
            Self::Jl { .. } => "jl",
            Self::Jle { .. } => "jle",
            Self::Ja { .. } => "ja",
            Self::Jae { .. } => "jae",
            Self::Jb { .. } => "jb",
            Self::Jbe { .. } => "jbe",
            Self::Js { .. } => "js",
            Self::Jns { .. } => "jns",
            Self::Jo { .. } => "jo",
            Self::Jno { .. } => "jno",
            Self::Jp { .. } => "jp",
            Self::Jnp { .. } => "jnp",
            Self::Call { .. } => "call",
            Self::Ret => "ret",
            Self::RetImm { .. } => "ret",
            Self::Nop => "nop",
            Self::Hlt => "hlt",
            Self::Cpuid => "cpuid",
            Self::Pause => "pause",
            Self::Cqo => "cqo",
            Self::Cdq => "cdq",
            Self::Syscall => "syscall",
            Self::Sysret => "sysret",
            Self::Movaps { .. } => "movaps",
            Self::Movapd { .. } => "movapd",
            Self::Movups { .. } => "movups",
            Self::Movupd { .. } => "movupd",
            Self::Movss { .. } => "movss",
            Self::Movsd { .. } => "movsd",
            Self::Movdqa { .. } => "movdqa",
            Self::Movdqu { .. } => "movdqu",
            Self::Addps { .. } => "addps",
            Self::Addpd { .. } => "addpd",
            Self::Addss { .. } => "addss",
            Self::Addsd { .. } => "addsd",
            Self::Subps { .. } => "subps",
            Self::Subpd { .. } => "subpd",
            Self::Subss { .. } => "subss",
            Self::Subsd { .. } => "subsd",
            Self::Mulps { .. } => "mulps",
            Self::Mulpd { .. } => "mulpd",
            Self::Mulss { .. } => "mulss",
            Self::Mulsd { .. } => "mulsd",
            Self::Divps { .. } => "divps",
            Self::Divpd { .. } => "divpd",
            Self::Divss { .. } => "divss",
            Self::Divsd { .. } => "divsd",
            Self::Andps { .. } => "andps",
            Self::Andpd { .. } => "andpd",
            Self::Andnps { .. } => "andnps",
            Self::Andnpd { .. } => "andnpd",
            Self::Orps { .. } => "orps",
            Self::Orpd { .. } => "orpd",
            Self::Xorps { .. } => "xorps",
            Self::Xorpd { .. } => "xorpd",
            Self::Cvtss2sd { .. } => "cvtss2sd",
            Self::Cvtsd2ss { .. } => "cvtsd2ss",
            Self::Cvttss2si { .. } => "cvttss2si",
            Self::Cvttsd2si { .. } => "cvttsd2si",
            Self::Cvtsi2ss { .. } => "cvtsi2ss",
            Self::Cvtsi2sd { .. } => "cvtsi2sd",
            Self::Vaddps { .. } => "vaddps",
            Self::Vaddpd { .. } => "vaddpd",
            Self::Vaddss { .. } => "vaddss",
            Self::Vaddsd { .. } => "vaddsd",
            Self::Vsubps { .. } => "vsubps",
            Self::Vsubpd { .. } => "vsubpd",
            Self::Vmulps { .. } => "vmulps",
            Self::Vmulpd { .. } => "vmulpd",
            Self::Vdivps { .. } => "vdivps",
            Self::Vdivpd { .. } => "vdivpd",
            Self::Fld { .. } => "fld",
            Self::Fst { .. } => "fst",
            Self::Fstp { .. } => "fstp",
            Self::Fadd { .. } => "fadd",
            Self::Faddp { .. } => "faddp",
            Self::Fsub { .. } => "fsub",
            Self::Fsubp { .. } => "fsubp",
            Self::Fmul { .. } => "fmul",
            Self::Fmulp { .. } => "fmulp",
            Self::Fdiv { .. } => "fdiv",
            Self::Fdivp { .. } => "fdivp",
            Self::Bsf { .. } => "bsf",
            Self::Bsr { .. } => "bsr",
            Self::Bt { .. } => "bt",
            Self::Btc { .. } => "btc",
            Self::Btr { .. } => "btr",
            Self::Bts { .. } => "bts",
            Self::Popcnt { .. } => "popcnt",
            Self::Lzcnt { .. } => "lzcnt",
            Self::Tzcnt { .. } => "tzcnt",
            Self::Cmove { .. } => "cmove",
            Self::Cmovne { .. } => "cmovne",
            Self::Cmovg { .. } => "cmovg",
            Self::Cmovge { .. } => "cmovge",
            Self::Cmovl { .. } => "cmovl",
            Self::Cmovle { .. } => "cmovle",
            Self::Cmova { .. } => "cmova",
            Self::Cmovae { .. } => "cmovae",
            Self::Cmovb { .. } => "cmovb",
            Self::Cmovbe { .. } => "cmovbe",
            Self::Sete { .. } => "sete",
            Self::Setne { .. } => "setne",
            Self::Setg { .. } => "setg",
            Self::Setge { .. } => "setge",
            Self::Setl { .. } => "setl",
            Self::Setle { .. } => "setle",
            Self::Seta { .. } => "seta",
            Self::Setae { .. } => "setae",
            Self::Setb { .. } => "setb",
            Self::Setbe { .. } => "setbe",
            Self::Movsb => "movsb",
            Self::Movsw => "movsw",
            Self::MovsdString => "movsd",
            Self::Movsq => "movsq",
            Self::Stosb => "stosb",
            Self::Stosw => "stosw",
            Self::Stosd => "stosd",
            Self::Stosq => "stosq",
        }
    }

    /// Verifica se l'istruzione è un salto
    pub fn is_jump(&self) -> bool {
        matches!(
            self,
            Self::Jmp { .. }
                | Self::Je { .. }
                | Self::Jne { .. }
                | Self::Jz { .. }
                | Self::Jnz { .. }
                | Self::Jg { .. }
                | Self::Jge { .. }
                | Self::Jl { .. }
                | Self::Jle { .. }
                | Self::Ja { .. }
                | Self::Jae { .. }
                | Self::Jb { .. }
                | Self::Jbe { .. }
                | Self::Js { .. }
                | Self::Jns { .. }
                | Self::Jo { .. }
                | Self::Jno { .. }
                | Self::Jp { .. }
                | Self::Jnp { .. }
        )
    }

    /// Verifica se l'istruzione è una chiamata
    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call { .. })
    }

    /// Verifica se l'istruzione è un return
    pub fn is_return(&self) -> bool {
        matches!(self, Self::Ret | Self::RetImm { .. })
    }
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

use std::fmt;

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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Istruzioni binarie (dest, src)
            Self::Add { dest, src }
            | Self::Sub { dest, src }
            | Self::Adc { dest, src }
            | Self::Sbb { dest, src }
            | Self::And { dest, src }
            | Self::Or { dest, src }
            | Self::Xor { dest, src }
            | Self::Mov { dest, src }
            | Self::Movsx { dest, src }
            | Self::Movsxd { dest, src }
            | Self::Movzx { dest, src }
            | Self::Lea { dest, src }
            | Self::Movaps { dest, src }
            | Self::Movapd { dest, src }
            | Self::Movups { dest, src }
            | Self::Movupd { dest, src }
            | Self::Movss { dest, src }
            | Self::Movsd { dest, src }
            | Self::Movdqa { dest, src }
            | Self::Movdqu { dest, src }
            | Self::Addps { dest, src }
            | Self::Addpd { dest, src }
            | Self::Addss { dest, src }
            | Self::Addsd { dest, src }
            | Self::Subps { dest, src }
            | Self::Subpd { dest, src }
            | Self::Subss { dest, src }
            | Self::Subsd { dest, src }
            | Self::Mulps { dest, src }
            | Self::Mulpd { dest, src }
            | Self::Mulss { dest, src }
            | Self::Mulsd { dest, src }
            | Self::Divps { dest, src }
            | Self::Divpd { dest, src }
            | Self::Divss { dest, src }
            | Self::Divsd { dest, src }
            | Self::Andps { dest, src }
            | Self::Andpd { dest, src }
            | Self::Andnps { dest, src }
            | Self::Andnpd { dest, src }
            | Self::Orps { dest, src }
            | Self::Orpd { dest, src }
            | Self::Xorps { dest, src }
            | Self::Xorpd { dest, src }
            | Self::Cvtss2sd { dest, src }
            | Self::Cvtsd2ss { dest, src }
            | Self::Cvttss2si { dest, src }
            | Self::Cvttsd2si { dest, src }
            | Self::Cvtsi2ss { dest, src }
            | Self::Cvtsi2sd { dest, src }
            | Self::Bsf { dest, src }
            | Self::Bsr { dest, src }
            | Self::Bt { dest, src }
            | Self::Btc { dest, src }
            | Self::Btr { dest, src }
            | Self::Bts { dest, src }
            | Self::Popcnt { dest, src }
            | Self::Lzcnt { dest, src }
            | Self::Tzcnt { dest, src }
            | Self::Cmove { dest, src }
            | Self::Cmovne { dest, src }
            | Self::Cmovg { dest, src }
            | Self::Cmovge { dest, src }
            | Self::Cmovl { dest, src }
            | Self::Cmovle { dest, src }
            | Self::Cmova { dest, src }
            | Self::Cmovae { dest, src }
            | Self::Cmovb { dest, src }
            | Self::Cmovbe { dest, src } => {
                write!(f, "{} {}, {}", self.mnemonic(), dest, src)
            }

            // Istruzioni shift/rotate
            Self::Shl { dest, count }
            | Self::Shr { dest, count }
            | Self::Sar { dest, count }
            | Self::Sal { dest, count }
            | Self::Rol { dest, count }
            | Self::Ror { dest, count }
            | Self::Rcl { dest, count }
            | Self::Rcr { dest, count } => {
                write!(f, "{} {}, {}", self.mnemonic(), dest, count)
            }

            // Istruzioni con due operandi (op1, op2)
            Self::Test { op1, op2 } | Self::Cmp { op1, op2 } | Self::Xchg { op1, op2 } => {
                write!(f, "{} {}, {}", self.mnemonic(), op1, op2)
            }

            // Istruzioni unarie
            Self::Mul { src }
            | Self::Div { src }
            | Self::Idiv { src }
            | Self::Inc { dest: src }
            | Self::Dec { dest: src }
            | Self::Neg { dest: src }
            | Self::Not { dest: src }
            | Self::Push { src }
            | Self::Pop { dest: src } => {
                write!(f, "{} {}", self.mnemonic(), src)
            }

            // IMUL con varianti multiple
            Self::Imul { dest, src1, src2 } => match (dest, src2) {
                (None, None) => write!(f, "imul {}", src1),
                (Some(d), None) => write!(f, "imul {}, {}", d, src1),
                (Some(d), Some(s2)) => write!(f, "imul {}, {}, {}", d, src1, s2),
                (None, Some(_)) => unreachable!(),
            },

            // Jump e call
            Self::Jmp { target }
            | Self::Je { target }
            | Self::Jne { target }
            | Self::Jz { target }
            | Self::Jnz { target }
            | Self::Jg { target }
            | Self::Jge { target }
            | Self::Jl { target }
            | Self::Jle { target }
            | Self::Ja { target }
            | Self::Jae { target }
            | Self::Jb { target }
            | Self::Jbe { target }
            | Self::Js { target }
            | Self::Jns { target }
            | Self::Jo { target }
            | Self::Jno { target }
            | Self::Jp { target }
            | Self::Jnp { target }
            | Self::Call { target } => {
                write!(f, "{} {}", self.mnemonic(), target)
            }

            // Return
            Self::Ret => write!(f, "ret"),
            Self::RetImm { imm } => write!(f, "ret {}", imm),

            // AVX a tre operandi
            Self::Vaddps { dest, src1, src2 }
            | Self::Vaddpd { dest, src1, src2 }
            | Self::Vaddss { dest, src1, src2 }
            | Self::Vaddsd { dest, src1, src2 }
            | Self::Vsubps { dest, src1, src2 }
            | Self::Vsubpd { dest, src1, src2 }
            | Self::Vmulps { dest, src1, src2 }
            | Self::Vmulpd { dest, src1, src2 }
            | Self::Vdivps { dest, src1, src2 }
            | Self::Vdivpd { dest, src1, src2 } => {
                write!(f, "{} {}, {}, {}", self.mnemonic(), dest, src1, src2)
            }

            // FPU
            Self::Fld { src } | Self::Fst { dest: src } | Self::Fstp { dest: src } => {
                write!(f, "{} {}", self.mnemonic(), src)
            }

            Self::Fadd { src }
            | Self::Faddp { src }
            | Self::Fsub { src }
            | Self::Fsubp { src }
            | Self::Fmul { src }
            | Self::Fmulp { src }
            | Self::Fdiv { src }
            | Self::Fdivp { src } => match src {
                Some(s) => write!(f, "{} {}", self.mnemonic(), s),
                None => write!(f, "{}", self.mnemonic()),
            },

            // SETcc
            Self::Sete { dest }
            | Self::Setne { dest }
            | Self::Setg { dest }
            | Self::Setge { dest }
            | Self::Setl { dest }
            | Self::Setle { dest }
            | Self::Seta { dest }
            | Self::Setae { dest }
            | Self::Setb { dest }
            | Self::Setbe { dest } => {
                write!(f, "{} {}", self.mnemonic(), dest)
            }

            // Istruzioni senza operandi
            Self::Nop
            | Self::Hlt
            | Self::Cpuid
            | Self::Pause
            | Self::Cqo
            | Self::Cdq
            | Self::Syscall
            | Self::Sysret
            | Self::Movsb
            | Self::Movsw
            | Self::MovsdString
            | Self::Movsq
            | Self::Stosb
            | Self::Stosw
            | Self::Stosd
            | Self::Stosq => {
                write!(f, "{}", self.mnemonic())
            }
        }
    }
}
