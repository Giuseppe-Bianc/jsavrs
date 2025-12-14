use crate::asm::{
    ControlRegister, DebugRegister, FPURegister, FlagsRegister, GPRegister8, GPRegister16, GPRegister32, GPRegister64,
    InstructionPointer, MMXRegister, MaskRegister, SegmentRegister, XMMRegister, YMMRegister, ZMMRegister,
    platform::Platform,
};
use std::fmt;

/// Unified enumeration for all x86-64 register types.
///
/// Provides type-safe unified interface for any register. Enables generic
/// register handling in assemblers and code generators with ABI-aware methods
/// for querying volatility, size, calling convention roles, and assembly names.
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
    /// Returns true if register is volatile (caller-saved) for given platform.
    #[must_use]
    pub const fn is_volatile(&self, platform: Platform) -> bool {
        match self {
            Self::GP64(r) => match platform {
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
            Self::Xmm(r) => match platform {
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
            Self::Ymm(r) => match platform {
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

    /// Returns true if register is callee-saved (non-volatile) for given platform.
    #[must_use]
    pub const fn is_callee_saved(&self, platform: Platform) -> bool {
        match self {
            Self::GP64(r) => match platform {
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
            Self::Xmm(r) => match platform {
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

    /// Returns register size in bits (8, 16, 32, 64, 80, 128, 256, 512).
    #[must_use]
    pub const fn size_bits(&self) -> usize {
        match self {
            Self::GP64(_)
            | Self::Flags(FlagsRegister::Rflags)
            | Self::InstructionPointer(InstructionPointer::Rip)
            | Self::Mmx(_)
            | Self::Mask(_)
            | Self::Control(_)
            | Self::Debug(_) => 64,
            Self::GP32(_) | Self::Flags(FlagsRegister::Eflags) | Self::InstructionPointer(InstructionPointer::Eip) => {
                32
            }
            Self::GP16(_)
            | Self::Flags(FlagsRegister::Flags)
            | Self::InstructionPointer(InstructionPointer::Ip)
            | Self::Segment(_) => 16,
            Self::GP8(_) => 8,
            Self::Fpu(_) => 80,
            Self::Xmm(_) => 128,
            Self::Ymm(_) => 256,
            Self::Zmm(_) => 512,
        }
    }

    /// Returns register size in bytes.
    #[must_use]
    pub const fn size_bytes(&self) -> usize {
        self.size_bits() / 8
    }

    /// Returns true if this is a general purpose register
    #[must_use]
    pub const fn is_gp(&self) -> bool {
        matches!(self, Self::GP64(_) | Self::GP32(_) | Self::GP16(_) | Self::GP8(_))
    }

    /// Returns true if this is a SIMD register (XMM, YMM, ZMM)
    #[must_use]
    pub const fn is_simd(&self) -> bool {
        matches!(self, Self::Xmm(_) | Self::Ymm(_) | Self::Zmm(_))
    }

    /// Returns true if this is a floating point register (FPU, XMM, YMM, ZMM)
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Fpu(_) | Self::Xmm(_) | Self::Ymm(_) | Self::Zmm(_))
    }

    /// Returns true if this is a special register (segment, control, debug, flags, IP)
    #[must_use]
    pub const fn is_special(&self) -> bool {
        matches!(
            self,
            Self::Segment(_) | Self::Control(_) | Self::Debug(_) | Self::Flags(_) | Self::InstructionPointer(_)
        )
    }

    /// Returns true if this is a 64-bit register
    #[must_use]
    pub const fn is_64bit(&self) -> bool {
        self.size_bits() == 64
    }

    /// Returns true if this is a 32-bit register
    #[must_use]
    pub const fn is_32bit(&self) -> bool {
        self.size_bits() == 32
    }

    /// Returns true if this is a 16-bit register
    #[must_use]
    pub const fn is_16bit(&self) -> bool {
        self.size_bits() == 16
    }

    /// Returns true if this is an 8-bit register
    #[must_use]
    pub const fn is_8bit(&self) -> bool {
        self.size_bits() == 8
    }

    /// Checks if register is used for Nth parameter (0-indexed) on platform.    /// Checks if register is used for Nth parameter (0-indexed) on platform.
    #[must_use]
    pub const fn is_parameter_register(&self, platform: Platform, param_index: usize) -> bool {
        match platform {
            Platform::Windows => {
                // Windows x64 calling convention
                match self {
                    Self::GP64(r) => matches!(
                        (r, param_index),
                        (GPRegister64::Rcx, 0) | (GPRegister64::Rdx, 1) | (GPRegister64::R8, 2) | (GPRegister64::R9, 3)
                    ),
                    Self::Xmm(r) => matches!(
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
                    Self::GP64(r) => matches!(
                        (r, param_index),
                        (GPRegister64::Rdi, 0)
                            | (GPRegister64::Rsi, 1)
                            | (GPRegister64::Rdx, 2)
                            | (GPRegister64::Rcx, 3)
                            | (GPRegister64::R8, 4)
                            | (GPRegister64::R9, 5)
                    ),
                    Self::Xmm(r) => {
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

    /// Checks if register is used for return values on platform.
    #[must_use]
    pub const fn is_return_register(&self, platform: Platform) -> bool {
        match platform {
            Platform::Windows | Platform::Linux | Platform::MacOS => {
                matches!(
                    self,
                    Self::GP64(GPRegister64::Rax | GPRegister64::Rdx)
                        | Self::Xmm(XMMRegister::Xmm0 | XMMRegister::Xmm1) // System V per struct
                )
            }
        }
    }

    /// Returns NASM-compatible lowercase register name.
    #[must_use]
    pub fn nasm_name(&self) -> String {
        match self {
            Self::GP64(r) => format!("{r:?}").to_lowercase(),
            Self::GP32(r) => format!("{r:?}").to_lowercase(),
            Self::GP16(r) => format!("{r:?}").to_lowercase(),
            Self::GP8(r) => format!("{r:?}").to_lowercase(),
            Self::Fpu(r) => {
                format!("{r}")
            }
            Self::Mmx(r) => format!("{r:?}").to_lowercase(),
            Self::Xmm(r) => format!("{r:?}").to_lowercase(),
            Self::Ymm(r) => format!("{r:?}").to_lowercase(),
            Self::Zmm(r) => format!("{r:?}").to_lowercase(),
            Self::Mask(r) => format!("{r:?}").to_lowercase(),
            Self::Segment(r) => format!("{r:?}").to_lowercase(),
            Self::Control(r) => format!("{r:?}").to_lowercase(),
            Self::Debug(r) => format!("{r:?}").to_lowercase(),
            Self::Flags(r) => format!("{r:?}").to_lowercase(),
            Self::InstructionPointer(r) => format!("{r:?}").to_lowercase(),
        }
    }
}

// Internal macro to implement Display for register types.
// Generates Display impls that delegate to Debug formatting (lowercase).
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

// Implement Display for register types that follow the same pattern (Debug lowercase).
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
); // Implementazione specifica per FPURegister
impl fmt::Display for FPURegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = match self {
            Self::St0 => 0,
            Self::St1 => 1,
            Self::St2 => 2,
            Self::St3 => 3,
            Self::St4 => 4,
            Self::St5 => 5,
            Self::St6 => 6,
            Self::St7 => 7,
        };
        write!(f, "st{idx}")
    }
}

impl fmt::Display for X86Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nasm_name())
    }
}
