use crate::asm::{
    ControlRegister, DebugRegister, FPURegister, FlagsRegister, GPRegister8, GPRegister16, GPRegister32, GPRegister64,
    InstructionPointer, MMXRegister, MaskRegister, SegmentRegister, XMMRegister, YMMRegister, ZMMRegister,
    platform::Platform,
};
use std::fmt;

/// Enumerazione principale che raggruppa tutti i tipi di registri
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Register {
    GP64(GPRegister64),
    GP32(GPRegister32),
    GP16(GPRegister16),
    GP8(GPRegister8),
    Fpu(FPURegister),
    Mmx(MMXRegister),
    Xmm(XMMRegister),
    Ymm(YMMRegister),
    Zmm(ZMMRegister),
    Mask(MaskRegister),
    Segment(SegmentRegister),
    Control(ControlRegister),
    Debug(DebugRegister),
    Flags(FlagsRegister),
    InstructionPointer(InstructionPointer),
}

// Shared static register lists used by ABI logic. Defining them here
// avoids duplicating the same arrays in multiple places.
pub const INT_PARAM_REGS_SYSTEMV: &[GPRegister64] =
    &[GPRegister64::Rdi, GPRegister64::Rsi, GPRegister64::Rdx, GPRegister64::Rcx, GPRegister64::R8, GPRegister64::R9];

pub const INT_PARAM_REGS_WINDOWS: &[GPRegister64] =
    &[GPRegister64::Rcx, GPRegister64::Rdx, GPRegister64::R8, GPRegister64::R9];

pub const FLOAT_PARAM_REGS_SYSTEMV: &[XMMRegister] = &[
    XMMRegister::Xmm0,
    XMMRegister::Xmm1,
    XMMRegister::Xmm2,
    XMMRegister::Xmm3,
    XMMRegister::Xmm4,
    XMMRegister::Xmm5,
    XMMRegister::Xmm6,
    XMMRegister::Xmm7,
];

pub const FLOAT_PARAM_REGS_WINDOWS: &[XMMRegister] =
    &[XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2, XMMRegister::Xmm3];

pub const INT_RETURN_REGS: &[GPRegister64] = &[GPRegister64::Rax, GPRegister64::Rdx];

pub const FLOAT_RETURN_REGS_SYSTEMV: &[XMMRegister] = &[XMMRegister::Xmm0, XMMRegister::Xmm1];
pub const FLOAT_RETURN_REGS_WINDOWS: &[XMMRegister] = &[XMMRegister::Xmm0];

pub const CALLEE_SAVED_GP_SYSTEMV: &[GPRegister64] =
    &[GPRegister64::Rbx, GPRegister64::Rbp, GPRegister64::R12, GPRegister64::R13, GPRegister64::R14, GPRegister64::R15];

pub const CALLEE_SAVED_GP_WINDOWS: &[GPRegister64] = &[
    GPRegister64::Rbx,
    GPRegister64::Rbp,
    GPRegister64::Rdi,
    GPRegister64::Rsi,
    GPRegister64::R12,
    GPRegister64::R13,
    GPRegister64::R14,
    GPRegister64::R15,
];

pub const CALLEE_SAVED_XMM_WINDOWS: &[XMMRegister] = &[
    XMMRegister::Xmm6,
    XMMRegister::Xmm7,
    XMMRegister::Xmm8,
    XMMRegister::Xmm9,
    XMMRegister::Xmm10,
    XMMRegister::Xmm11,
    XMMRegister::Xmm12,
    XMMRegister::Xmm13,
    XMMRegister::Xmm14,
    XMMRegister::Xmm15,
];

pub const CALLER_SAVED_GP_SYSTEMV: &[GPRegister64] = &[
    GPRegister64::Rax,
    GPRegister64::Rcx,
    GPRegister64::Rdx,
    GPRegister64::Rsi,
    GPRegister64::Rdi,
    GPRegister64::R8,
    GPRegister64::R9,
    GPRegister64::R10,
    GPRegister64::R11,
];

pub const CALLER_SAVED_GP_WINDOWS: &[GPRegister64] = &[
    GPRegister64::Rax,
    GPRegister64::Rcx,
    GPRegister64::Rdx,
    GPRegister64::R8,
    GPRegister64::R9,
    GPRegister64::R10,
    GPRegister64::R11,
];

pub const CALLER_SAVED_XMM_SYSTEMV: &[XMMRegister] = &[
    XMMRegister::Xmm0,
    XMMRegister::Xmm1,
    XMMRegister::Xmm2,
    XMMRegister::Xmm3,
    XMMRegister::Xmm4,
    XMMRegister::Xmm5,
    XMMRegister::Xmm6,
    XMMRegister::Xmm7,
    XMMRegister::Xmm8,
    XMMRegister::Xmm9,
    XMMRegister::Xmm10,
    XMMRegister::Xmm11,
    XMMRegister::Xmm12,
    XMMRegister::Xmm13,
    XMMRegister::Xmm14,
    XMMRegister::Xmm15,
];

pub const CALLER_SAVED_XMM_WINDOWS: &[XMMRegister] =
    &[XMMRegister::Xmm0, XMMRegister::Xmm1, XMMRegister::Xmm2, XMMRegister::Xmm3, XMMRegister::Xmm4, XMMRegister::Xmm5];

impl X86Register {
    /// Verifica se il registro è volatile secondo la calling convention
    pub fn is_volatile(&self, platform: Platform) -> bool {
        match self {
            X86Register::GP64(r) => match platform {
                Platform::Windows => matches!(
                    r,
                    GPRegister64::Rax
                        | GPRegister64::Rcx
                        | GPRegister64::Rdx
                        | GPRegister64::R8
                        | GPRegister64::R9
                        | GPRegister64::R10
                        | GPRegister64::R11
                ),
                Platform::Linux | Platform::MacOS => matches!(
                    r,
                    GPRegister64::Rax
                        | GPRegister64::Rcx
                        | GPRegister64::Rdx
                        | GPRegister64::Rsi
                        | GPRegister64::Rdi
                        | GPRegister64::R8
                        | GPRegister64::R9
                        | GPRegister64::R10
                        | GPRegister64::R11
                ),
            },
            X86Register::Xmm(r) => match platform {
                Platform::Windows => matches!(
                    r,
                    XMMRegister::Xmm0
                        | XMMRegister::Xmm1
                        | XMMRegister::Xmm2
                        | XMMRegister::Xmm3
                        | XMMRegister::Xmm4
                        | XMMRegister::Xmm5
                ),
                Platform::Linux | Platform::MacOS => true, // Tutti volatili in System V
            },
            X86Register::Ymm(r) => match platform {
                Platform::Windows => matches!(
                    r,
                    YMMRegister::Ymm0
                        | YMMRegister::Ymm1
                        | YMMRegister::Ymm2
                        | YMMRegister::Ymm3
                        | YMMRegister::Ymm4
                        | YMMRegister::Ymm5
                ),
                Platform::Linux | Platform::MacOS => true,
            },
            _ => false,
        }
    }

    /// Verifica se il registro è non-volatile (callee-saved)
    pub fn is_callee_saved(&self, platform: Platform) -> bool {
        match self {
            X86Register::GP64(r) => match platform {
                Platform::Windows => matches!(
                    r,
                    GPRegister64::Rbx
                        | GPRegister64::Rbp
                        | GPRegister64::Rdi
                        | GPRegister64::Rsi
                        | GPRegister64::Rsp
                        | GPRegister64::R12
                        | GPRegister64::R13
                        | GPRegister64::R14
                        | GPRegister64::R15
                ),
                Platform::Linux | Platform::MacOS => matches!(
                    r,
                    GPRegister64::Rbx
                        | GPRegister64::Rbp
                        | GPRegister64::Rsp
                        | GPRegister64::R12
                        | GPRegister64::R13
                        | GPRegister64::R14
                        | GPRegister64::R15
                ),
            },
            X86Register::Xmm(r) => match platform {
                Platform::Windows => matches!(
                    r,
                    XMMRegister::Xmm6
                        | XMMRegister::Xmm7
                        | XMMRegister::Xmm8
                        | XMMRegister::Xmm9
                        | XMMRegister::Xmm10
                        | XMMRegister::Xmm11
                        | XMMRegister::Xmm12
                        | XMMRegister::Xmm13
                        | XMMRegister::Xmm14
                        | XMMRegister::Xmm15
                ),
                Platform::Linux | Platform::MacOS => false,
            },
            _ => false,
        }
    }

    /// Ottiene la dimensione del registro in bit
    pub fn size_bits(&self) -> usize {
        match self {
            X86Register::GP64(_)
            | X86Register::Flags(FlagsRegister::Rflags)
            | X86Register::InstructionPointer(InstructionPointer::Rip) => 64,
            X86Register::GP32(_)
            | X86Register::Flags(FlagsRegister::Eflags)
            | X86Register::InstructionPointer(InstructionPointer::Eip) => 32,
            X86Register::GP16(_)
            | X86Register::Flags(FlagsRegister::Flags)
            | X86Register::InstructionPointer(InstructionPointer::Ip)
            | X86Register::Segment(_) => 16,
            X86Register::GP8(_) => 8,
            X86Register::Fpu(_) => 80,
            X86Register::Mmx(_) => 64,
            X86Register::Xmm(_) => 128,
            X86Register::Ymm(_) => 256,
            X86Register::Zmm(_) => 512,
            X86Register::Mask(_) => 64,
            X86Register::Control(_) | X86Register::Debug(_) => 64,
        }
    }

    /// Ottiene la dimensione del registro in byte
    pub fn size_bytes(&self) -> usize {
        self.size_bits() / 8
    }

    /// Verifica se il registro può essere usato per passaggio parametri
    pub fn is_parameter_register(&self, platform: Platform, param_index: usize) -> bool {
        match platform {
            Platform::Windows => {
                // Windows x64 calling convention
                match self {
                    X86Register::GP64(r) => matches!(
                        (r, param_index),
                        (GPRegister64::Rcx, 0) | (GPRegister64::Rdx, 1) | (GPRegister64::R8, 2) | (GPRegister64::R9, 3)
                    ),
                    X86Register::Xmm(r) => matches!(
                        (r, param_index),
                        (XMMRegister::Xmm0, 0)
                            | (XMMRegister::Xmm1, 1)
                            | (XMMRegister::Xmm2, 2)
                            | (XMMRegister::Xmm3, 3)
                    ),
                    _ => false,
                }
            }
            Platform::Linux | Platform::MacOS => {
                // System V AMD64 ABI
                match self {
                    X86Register::GP64(r) => matches!(
                        (r, param_index),
                        (GPRegister64::Rdi, 0)
                            | (GPRegister64::Rsi, 1)
                            | (GPRegister64::Rdx, 2)
                            | (GPRegister64::Rcx, 3)
                            | (GPRegister64::R8, 4)
                            | (GPRegister64::R9, 5)
                    ),
                    X86Register::Xmm(r) => {
                        param_index < 8
                            && matches!(
                                (r, param_index),
                                (XMMRegister::Xmm0, 0)
                                    | (XMMRegister::Xmm1, 1)
                                    | (XMMRegister::Xmm2, 2)
                                    | (XMMRegister::Xmm3, 3)
                                    | (XMMRegister::Xmm4, 4)
                                    | (XMMRegister::Xmm5, 5)
                                    | (XMMRegister::Xmm6, 6)
                                    | (XMMRegister::Xmm7, 7)
                            )
                    }
                    _ => false,
                }
            }
        }
    }

    /// Verifica se il registro viene usato per il valore di ritorno
    pub fn is_return_register(&self, platform: Platform) -> bool {
        match platform {
            Platform::Windows | Platform::Linux | Platform::MacOS => {
                matches!(
                    self,
                    X86Register::GP64(GPRegister64::Rax) |
                    X86Register::GP64(GPRegister64::Rdx) | // Per valori a 128-bit
                    X86Register::Xmm(XMMRegister::Xmm0) |
                    X86Register::Xmm(XMMRegister::Xmm1) // System V per struct
                )
            }
        }
    }

    pub fn nasm_name(&self) -> String {
        match self {
            X86Register::GP64(r) => format!("{:?}", r).to_lowercase(),
            X86Register::GP32(r) => format!("{:?}", r).to_lowercase(),
            X86Register::GP16(r) => format!("{:?}", r).to_lowercase(),
            X86Register::GP8(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Fpu(r) => {
                format!("{r}")
            }
            X86Register::Mmx(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Xmm(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Ymm(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Zmm(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Mask(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Segment(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Control(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Debug(r) => format!("{:?}", r).to_lowercase(),
            X86Register::Flags(r) => format!("{:?}", r).to_lowercase(),
            X86Register::InstructionPointer(r) => format!("{:?}", r).to_lowercase(),
        }
    }
}

// Implementazione del trait Display per tutti i tipi di registri
// Macro per implementare automaticamente il trait Display per i tipi di registri
macro_rules! impl_display_for_register {
    ($($t:ty),*) => {
        $(
            impl fmt::Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", format!("{:?}", self).to_lowercase())
                }
            }
        )*
    }
}

// Implementazione del trait Display per tutti i tipi di registri che seguono lo stesso pattern
impl_display_for_register!(
    GPRegister64,
    GPRegister32,
    GPRegister16,
    GPRegister8,
    MMXRegister,
    XMMRegister,
    YMMRegister,
    ZMMRegister,
    MaskRegister,
    SegmentRegister,
    ControlRegister,
    DebugRegister,
    FlagsRegister,
    InstructionPointer
);

// Implementazione specifica per FPURegister
impl fmt::Display for FPURegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = match self {
            FPURegister::St0 => 0,
            FPURegister::St1 => 1,
            FPURegister::St2 => 2,
            FPURegister::St3 => 3,
            FPURegister::St4 => 4,
            FPURegister::St5 => 5,
            FPURegister::St6 => 6,
            FPURegister::St7 => 7,
        };
        write!(f, "st{}", idx)
    }
}

impl fmt::Display for X86Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nasm_name())
    }
}
