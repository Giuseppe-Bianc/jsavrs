//! x86-64 register definitions
use std::fmt;

/// x86-64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Register {
    // 64-bit registers
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    // 32-bit registers
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    EBP,
    ESP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,
    // 16-bit registers
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    BP,
    SP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,
    // 8-bit registers
    AL,
    BL,
    CL,
    DL,
    SIL,
    DIL,
    BPL,
    SPL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B,
    // XMM registers (128-bit)
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
    // YMM registers (256-bit)
    YMM0,
    YMM1,
    YMM2,
    YMM3,
    YMM4,
    YMM5,
    YMM6,
    YMM7,
    YMM8,
    YMM9,
    YMM10,
    YMM11,
    YMM12,
    YMM13,
    YMM14,
    YMM15,
    // ZMM registers (512-bit)
    ZMM0,
    ZMM1,
    ZMM2,
    ZMM3,
    ZMM4,
    ZMM5,
    ZMM6,
    ZMM7,
    ZMM8,
    ZMM9,
    ZMM10,
    ZMM11,
    ZMM12,
    ZMM13,
    ZMM14,
    ZMM15,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Register::RAX => "rax",
            Register::RBX => "rbx",
            Register::RCX => "rcx",
            Register::RDX => "rdx",
            Register::RSI => "rsi",
            Register::RDI => "rdi",
            Register::RBP => "rbp",
            Register::RSP => "rsp",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
            Register::EAX => "eax",
            Register::EBX => "ebx",
            Register::ECX => "ecx",
            Register::EDX => "edx",
            Register::ESI => "esi",
            Register::EDI => "edi",
            Register::EBP => "ebp",
            Register::ESP => "esp",
            Register::R8D => "r8d",
            Register::R9D => "r9d",
            Register::R10D => "r10d",
            Register::R11D => "r11d",
            Register::R12D => "r12d",
            Register::R13D => "r13d",
            Register::R14D => "r14d",
            Register::R15D => "r15d",
            Register::AX => "ax",
            Register::BX => "bx",
            Register::CX => "cx",
            Register::DX => "dx",
            Register::SI => "si",
            Register::DI => "di",
            Register::BP => "bp",
            Register::SP => "sp",
            Register::R8W => "r8w",
            Register::R9W => "r9w",
            Register::R10W => "r10w",
            Register::R11W => "r11w",
            Register::R12W => "r12w",
            Register::R13W => "r13w",
            Register::R14W => "r14w",
            Register::R15W => "r15w",
            Register::AL => "al",
            Register::BL => "bl",
            Register::CL => "cl",
            Register::DL => "dl",
            Register::SIL => "sil",
            Register::DIL => "dil",
            Register::BPL => "bpl",
            Register::SPL => "spl",
            Register::R8B => "r8b",
            Register::R9B => "r9b",
            Register::R10B => "r10b",
            Register::R11B => "r11b",
            Register::R12B => "r12b",
            Register::R13B => "r13b",
            Register::R14B => "r14b",
            Register::R15B => "r15b",
            // XMM registers
            Register::XMM0 => "xmm0",
            Register::XMM1 => "xmm1",
            Register::XMM2 => "xmm2",
            Register::XMM3 => "xmm3",
            Register::XMM4 => "xmm4",
            Register::XMM5 => "xmm5",
            Register::XMM6 => "xmm6",
            Register::XMM7 => "xmm7",
            Register::XMM8 => "xmm8",
            Register::XMM9 => "xmm9",
            Register::XMM10 => "xmm10",
            Register::XMM11 => "xmm11",
            Register::XMM12 => "xmm12",
            Register::XMM13 => "xmm13",
            Register::XMM14 => "xmm14",
            Register::XMM15 => "xmm15",
            // YMM registers
            Register::YMM0 => "ymm0",
            Register::YMM1 => "ymm1",
            Register::YMM2 => "ymm2",
            Register::YMM3 => "ymm3",
            Register::YMM4 => "ymm4",
            Register::YMM5 => "ymm5",
            Register::YMM6 => "ymm6",
            Register::YMM7 => "ymm7",
            Register::YMM8 => "ymm8",
            Register::YMM9 => "ymm9",
            Register::YMM10 => "ymm10",
            Register::YMM11 => "ymm11",
            Register::YMM12 => "ymm12",
            Register::YMM13 => "ymm13",
            Register::YMM14 => "ymm14",
            Register::YMM15 => "ymm15",
            // ZMM registers
            Register::ZMM0 => "zmm0",
            Register::ZMM1 => "zmm1",
            Register::ZMM2 => "zmm2",
            Register::ZMM3 => "zmm3",
            Register::ZMM4 => "zmm4",
            Register::ZMM5 => "zmm5",
            Register::ZMM6 => "zmm6",
            Register::ZMM7 => "zmm7",
            Register::ZMM8 => "zmm8",
            Register::ZMM9 => "zmm9",
            Register::ZMM10 => "zmm10",
            Register::ZMM11 => "zmm11",
            Register::ZMM12 => "zmm12",
            Register::ZMM13 => "zmm13",
            Register::ZMM14 => "zmm14",
            Register::ZMM15 => "zmm15",
        };
        write!(f, "{}", name)
    }
}

impl Register {
    /// Get the 64-bit equivalent of a register
    pub fn to_64bit(&self) -> Register {
        match self {
            Register::RAX | Register::EAX | Register::AX | Register::AL => Register::RAX,
            Register::RBX | Register::EBX | Register::BX | Register::BL => Register::RBX,
            Register::RCX | Register::ECX | Register::CX | Register::CL => Register::RCX,
            Register::RDX | Register::EDX | Register::DX | Register::DL => Register::RDX,
            Register::RSI | Register::ESI | Register::SI | Register::SIL => Register::RSI,
            Register::RDI | Register::EDI | Register::DI | Register::DIL => Register::RDI,
            Register::RBP | Register::EBP | Register::BP | Register::BPL => Register::RBP,
            Register::RSP | Register::ESP | Register::SP | Register::SPL => Register::RSP,
            Register::R8 | Register::R8D | Register::R8W | Register::R8B => Register::R8,
            Register::R9 | Register::R9D | Register::R9W | Register::R9B => Register::R9,
            Register::R10 | Register::R10D | Register::R10W | Register::R10B => Register::R10,
            Register::R11 | Register::R11D | Register::R11W | Register::R11B => Register::R11,
            Register::R12 | Register::R12D | Register::R12W | Register::R12B => Register::R12,
            Register::R13 | Register::R13D | Register::R13W | Register::R13B => Register::R13,
            Register::R14 | Register::R14D | Register::R14W | Register::R14B => Register::R14,
            Register::R15 | Register::R15D | Register::R15W | Register::R15B => Register::R15,
            // Floating-point registers don't have 64-bit equivalents, return self
            _ => *self,
        }
    }

    /// Get the 32-bit equivalent of a register
    pub fn to_32bit(&self) -> Register {
        match self.to_64bit() {
            Register::RAX => Register::EAX,
            Register::RBX => Register::EBX,
            Register::RCX => Register::ECX,
            Register::RDX => Register::EDX,
            Register::RSI => Register::ESI,
            Register::RDI => Register::EDI,
            Register::RBP => Register::EBP,
            Register::RSP => Register::ESP,
            Register::R8 => Register::R8D,
            Register::R9 => Register::R9D,
            Register::R10 => Register::R10D,
            Register::R11 => Register::R11D,
            Register::R12 => Register::R12D,
            Register::R13 => Register::R13D,
            Register::R14 => Register::R14D,
            Register::R15 => Register::R15D,
            _ => *self, // Already a 32-bit or smaller register
        }
    }

    /// Get the 16-bit equivalent of a register
    pub fn to_16bit(&self) -> Register {
        match self.to_64bit() {
            Register::RAX => Register::AX,
            Register::RBX => Register::BX,
            Register::RCX => Register::CX,
            Register::RDX => Register::DX,
            Register::RSI => Register::SI,
            Register::RDI => Register::DI,
            Register::RBP => Register::BP,
            Register::RSP => Register::SP,
            Register::R8 => Register::R8W,
            Register::R9 => Register::R9W,
            Register::R10 => Register::R10W,
            Register::R11 => Register::R11W,
            Register::R12 => Register::R12W,
            Register::R13 => Register::R13W,
            Register::R14 => Register::R14W,
            Register::R15 => Register::R15W,
            _ => *self, // Already a 16-bit or smaller register
        }
    }

    /// Get the 8-bit equivalent of a register
    pub fn to_8bit(&self) -> Register {
        match self.to_64bit() {
            Register::RAX => Register::AL,
            Register::RBX => Register::BL,
            Register::RCX => Register::CL,
            Register::RDX => Register::DL,
            Register::RSI => Register::SIL,
            Register::RDI => Register::DIL,
            Register::RBP => Register::BPL,
            Register::RSP => Register::SPL,
            Register::R8 => Register::R8B,
            Register::R9 => Register::R9B,
            Register::R10 => Register::R10B,
            Register::R11 => Register::R11B,
            Register::R12 => Register::R12B,
            Register::R13 => Register::R13B,
            Register::R14 => Register::R14B,
            Register::R15 => Register::R15B,
            _ => *self, // Already an 8-bit register
        }
    }

    /// Get the size of the register in bits
    pub fn size(&self) -> u16 {
        match self {
            Register::RAX
            | Register::RBX
            | Register::RCX
            | Register::RDX
            | Register::RSI
            | Register::RDI
            | Register::RBP
            | Register::RSP
            | Register::R8
            | Register::R9
            | Register::R10
            | Register::R11
            | Register::R12
            | Register::R13
            | Register::R14
            | Register::R15 => 64,

            Register::EAX
            | Register::EBX
            | Register::ECX
            | Register::EDX
            | Register::ESI
            | Register::EDI
            | Register::EBP
            | Register::ESP
            | Register::R8D
            | Register::R9D
            | Register::R10D
            | Register::R11D
            | Register::R12D
            | Register::R13D
            | Register::R14D
            | Register::R15D => 32,

            Register::AX
            | Register::BX
            | Register::CX
            | Register::DX
            | Register::SI
            | Register::DI
            | Register::BP
            | Register::SP
            | Register::R8W
            | Register::R9W
            | Register::R10W
            | Register::R11W
            | Register::R12W
            | Register::R13W
            | Register::R14W
            | Register::R15W => 16,

            Register::AL
            | Register::BL
            | Register::CL
            | Register::DL
            | Register::SIL
            | Register::DIL
            | Register::BPL
            | Register::SPL
            | Register::R8B
            | Register::R9B
            | Register::R10B
            | Register::R11B
            | Register::R12B
            | Register::R13B
            | Register::R14B
            | Register::R15B => 8,

            // XMM registers are 128-bit
            Register::XMM0
            | Register::XMM1
            | Register::XMM2
            | Register::XMM3
            | Register::XMM4
            | Register::XMM5
            | Register::XMM6
            | Register::XMM7
            | Register::XMM8
            | Register::XMM9
            | Register::XMM10
            | Register::XMM11
            | Register::XMM12
            | Register::XMM13
            | Register::XMM14
            | Register::XMM15 => 128,

            // YMM registers are 256-bit
            Register::YMM0
            | Register::YMM1
            | Register::YMM2
            | Register::YMM3
            | Register::YMM4
            | Register::YMM5
            | Register::YMM6
            | Register::YMM7
            | Register::YMM8
            | Register::YMM9
            | Register::YMM10
            | Register::YMM11
            | Register::YMM12
            | Register::YMM13
            | Register::YMM14
            | Register::YMM15 => 256,

            // ZMM registers are 512-bit
            Register::ZMM0
            | Register::ZMM1
            | Register::ZMM2
            | Register::ZMM3
            | Register::ZMM4
            | Register::ZMM5
            | Register::ZMM6
            | Register::ZMM7
            | Register::ZMM8
            | Register::ZMM9
            | Register::ZMM10
            | Register::ZMM11
            | Register::ZMM12
            | Register::ZMM13
            | Register::ZMM14
            | Register::ZMM15 => 512,
        }
    }

    /// Check if this is a Windows x64 ABI parameter register (for integer/pointer arguments)
    /// Windows x64 ABI uses RCX, RDX, R8, R9 for the first four integer/pointer arguments
    pub fn is_windows_param_register(&self) -> bool {
        matches!(self.to_64bit(), Register::RCX | Register::RDX | Register::R8 | Register::R9)
    }

    /// Check if this is a Windows x64 ABI caller-saved register
    /// Caller-saved registers in Windows x64 ABI: RAX, RCX, RDX, R8-R11
    pub fn is_windows_caller_saved(&self) -> bool {
        matches!(
            self.to_64bit(),
            Register::RAX | Register::RCX | Register::RDX | Register::R8 | Register::R9 | Register::R10 | Register::R11
        )
    }

    /// Check if this is a Windows x64 ABI callee-saved register
    /// Callee-saved registers in Windows x64 ABI: RBX, RBP, RDI, RSI, R12-R15
    pub fn is_windows_callee_saved(&self) -> bool {
        matches!(
            self.to_64bit(),
            Register::RBX
                | Register::RBP
                | Register::RDI
                | Register::RSI
                | Register::R12
                | Register::R13
                | Register::R14
                | Register::R15
        )
    }

    /// Get the nth parameter register for Windows x64 ABI
    /// Returns None if n > 3 (only 4 parameter registers in Windows x64 ABI)
    pub fn windows_param_register(n: usize) -> Option<Register> {
        match n {
            0 => Some(Register::RCX),
            1 => Some(Register::RDX),
            2 => Some(Register::R8),
            3 => Some(Register::R9),
            _ => None,
        }
    }

    /// Check if this is a System V ABI parameter register (for integer/pointer arguments)
    /// System V ABI uses RDI, RSI, RDX, RCX, R8, R9 for the first six integer/pointer arguments
    pub fn is_systemv_param_register(&self) -> bool {
        matches!(
            self.to_64bit(),
            Register::RDI | Register::RSI | Register::RDX | Register::RCX | Register::R8 | Register::R9
        )
    }

    /// Check if this is a System V ABI caller-saved register
    /// Caller-saved registers in System V ABI: RAX, RCX, RDX, RSI, RDI, R8-R11
    pub fn is_systemv_caller_saved(&self) -> bool {
        matches!(
            self.to_64bit(),
            Register::RAX
                | Register::RCX
                | Register::RDX
                | Register::RSI
                | Register::RDI
                | Register::R8
                | Register::R9
                | Register::R10
                | Register::R11
        )
    }

    /// Check if this is a System V ABI callee-saved register
    /// Callee-saved registers in System V ABI: RBX, RBP, R12-R15
    pub fn is_systemv_callee_saved(&self) -> bool {
        matches!(
            self.to_64bit(),
            Register::RBX | Register::RBP | Register::R12 | Register::R13 | Register::R14 | Register::R15
        )
    }

    /// Get the nth parameter register for System V ABI
    /// Returns None if n > 5 (only 6 parameter registers in System V ABI)
    pub fn systemv_param_register(n: usize) -> Option<Register> {
        match n {
            0 => Some(Register::RDI),
            1 => Some(Register::RSI),
            2 => Some(Register::RDX),
            3 => Some(Register::RCX),
            4 => Some(Register::R8),
            5 => Some(Register::R9),
            _ => None,
        }
    }

    /// Get all general purpose registers of this size
    pub fn general_purpose_registers(size: u8) -> Vec<Register> {
        match size {
            64 => vec![
                Register::RAX,
                Register::RBX,
                Register::RCX,
                Register::RDX,
                Register::RSI,
                Register::RDI,
                Register::RBP,
                Register::RSP,
                Register::R8,
                Register::R9,
                Register::R10,
                Register::R11,
                Register::R12,
                Register::R13,
                Register::R14,
                Register::R15,
            ],
            32 => vec![
                Register::EAX,
                Register::EBX,
                Register::ECX,
                Register::EDX,
                Register::ESI,
                Register::EDI,
                Register::EBP,
                Register::ESP,
                Register::R8D,
                Register::R9D,
                Register::R10D,
                Register::R11D,
                Register::R12D,
                Register::R13D,
                Register::R14D,
                Register::R15D,
            ],
            16 => vec![
                Register::AX,
                Register::BX,
                Register::CX,
                Register::DX,
                Register::SI,
                Register::DI,
                Register::BP,
                Register::SP,
                Register::R8W,
                Register::R9W,
                Register::R10W,
                Register::R11W,
                Register::R12W,
                Register::R13W,
                Register::R14W,
                Register::R15W,
            ],
            8 => vec![
                Register::AL,
                Register::BL,
                Register::CL,
                Register::DL,
                Register::SIL,
                Register::DIL,
                Register::BPL,
                Register::SPL,
                Register::R8B,
                Register::R9B,
                Register::R10B,
                Register::R11B,
                Register::R12B,
                Register::R13B,
                Register::R14B,
                Register::R15B,
            ],
            _ => vec![], // Invalid size
        }
    }
}
