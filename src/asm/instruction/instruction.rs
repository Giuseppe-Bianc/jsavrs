use super::operand::Operand;
use std::fmt;

/// Representation of all supported `x86_64` instructions.
///
/// This enum models the instruction set used by the assembler/IR. Each
/// variant corresponds to an instruction mnemonic and carries the operands
/// required by that form (registers, immediates, memory references, labels,
/// etc.). The enum is intentionally exhaustive for the subset of `x86_64`
/// targeted by this project and is used for formatting, analysis and
/// lowering/encoding phases.
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
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub const fn mnemonic(&self) -> &str {
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
            Self::Ret | Self::RetImm { .. } => "ret",
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
            Self::Movsd { .. } | Self::MovsdString => "movsd",
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
            Self::Movsq => "movsq",
            Self::Stosb => "stosb",
            Self::Stosw => "stosw",
            Self::Stosd => "stosd",
            Self::Stosq => "stosq",
        }
    }
    /// Return the textual mnemonic for this instruction.
    ///
    /// Inputs: `&self`.
    /// Outputs: `&str` — the canonical lowercase mnemonic (e.g. "mov", "add").
    /// Side effects: none.
    #[must_use]
    pub const fn is_jump(&self) -> bool {
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
    /// Return true if this instruction is a call instruction.
    ///
    /// Inputs: `&self`.
    /// Outputs: `bool`.
    /// Side effects: none.
    #[must_use]
    pub const fn is_call(&self) -> bool {
        matches!(self, Self::Call { .. })
    }

    /// Return true if this instruction is a return instruction.
    ///
    /// Covers both plain `ret` and `ret imm16` forms.
    #[must_use]
    pub const fn is_return(&self) -> bool {
        matches!(self, Self::Ret | Self::RetImm { .. })
    }
}

impl fmt::Display for Instruction {
    /// Format the instruction as human-readable assembly.
    ///
    /// The formatting uses the mnemonic followed by comma-separated
    /// operands when applicable. The specific layout mirrors common Intel
    /// assembly formatting conventions used across the project.
    ///
    /// Inputs: `&self`, `Formatter`.
    /// Outputs: `fmt::Result` after writing the textual representation.
    /// Side effects: writes to the provided formatter.
    #[allow(clippy::match_same_arms, clippy::too_many_lines)]
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
                (None, None) => write!(f, "imul {src1}"),
                (Some(d), None) => write!(f, "imul {d}, {d}, {src1}"),
                (Some(d), Some(s2)) => write!(f, "imul {d}, {d}, {src1}, {s2}"),
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
            Self::RetImm { imm } => write!(f, "ret {imm}"),

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
