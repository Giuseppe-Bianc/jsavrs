//! x86-64 register definitions
use std::fmt;

/// x86-64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Register {
    // 64-bit registers
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP,
    R8, R9, R10, R11, R12, R13, R14, R15,
    // 32-bit registers
    EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP,
    R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D,
    // 16-bit registers
    AX, BX, CX, DX, SI, DI, BP, SP,
    R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W,
    // 8-bit registers
    AL, BL, CL, DL, SIL, DIL, BPL, SPL,
    R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Register::RAX => "rax", Register::RBX => "rbx", Register::RCX => "rcx", Register::RDX => "rdx",
            Register::RSI => "rsi", Register::RDI => "rdi", Register::RBP => "rbp", Register::RSP => "rsp",
            Register::R8 => "r8", Register::R9 => "r9", Register::R10 => "r10", Register::R11 => "r11",
            Register::R12 => "r12", Register::R13 => "r13", Register::R14 => "r14", Register::R15 => "r15",
            Register::EAX => "eax", Register::EBX => "ebx", Register::ECX => "ecx", Register::EDX => "edx",
            Register::ESI => "esi", Register::EDI => "edi", Register::EBP => "ebp", Register::ESP => "esp",
            Register::R8D => "r8d", Register::R9D => "r9d", Register::R10D => "r10d", Register::R11D => "r11d",
            Register::R12D => "r12d", Register::R13D => "r13d", Register::R14D => "r14d", Register::R15D => "r15d",
            Register::AX => "ax", Register::BX => "bx", Register::CX => "cx", Register::DX => "dx",
            Register::SI => "si", Register::DI => "di", Register::BP => "bp", Register::SP => "sp",
            Register::R8W => "r8w", Register::R9W => "r9w", Register::R10W => "r10w", Register::R11W => "r11w",
            Register::R12W => "r12w", Register::R13W => "r13w", Register::R14W => "r14w", Register::R15W => "r15w",
            Register::AL => "al", Register::BL => "bl", Register::CL => "cl", Register::DL => "dl",
            Register::SIL => "sil", Register::DIL => "dil", Register::BPL => "bpl", Register::SPL => "spl",
            Register::R8B => "r8b", Register::R9B => "r9b", Register::R10B => "r10b", Register::R11B => "r11b",
            Register::R12B => "r12b", Register::R13B => "r13b", Register::R14B => "r14b", Register::R15B => "r15b",
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
        }
    }
    
    /// Get the size of the register in bits
    pub fn size(&self) -> u8 {
        match self {
            Register::RAX | Register::RBX | Register::RCX | Register::RDX | 
            Register::RSI | Register::RDI | Register::RBP | Register::RSP |
            Register::R8 | Register::R9 | Register::R10 | Register::R11 |
            Register::R12 | Register::R13 | Register::R14 | Register::R15 => 64,
            
            Register::EAX | Register::EBX | Register::ECX | Register::EDX |
            Register::ESI | Register::EDI | Register::EBP | Register::ESP |
            Register::R8D | Register::R9D | Register::R10D | Register::R11D |
            Register::R12D | Register::R13D | Register::R14D | Register::R15D => 32,
            
            Register::AX | Register::BX | Register::CX | Register::DX |
            Register::SI | Register::DI | Register::BP | Register::SP |
            Register::R8W | Register::R9W | Register::R10W | Register::R11W |
            Register::R12W | Register::R13W | Register::R14W | Register::R15W => 16,
            
            Register::AL | Register::BL | Register::CL | Register::DL |
            Register::SIL | Register::DIL | Register::BPL | Register::SPL |
            Register::R8B | Register::R9B | Register::R10B | Register::R11B |
            Register::R12B | Register::R13B | Register::R14B | Register::R15B => 8,
        }
    }
}