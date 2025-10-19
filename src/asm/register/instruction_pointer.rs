//! Instruction pointer register types.
//!
//! This module defines the `InstructionPointer` enum, which represents
//! different CPU instruction pointer register sizes used across x86 architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionPointer {
    Rip, // 64-bit
    Eip, // 32-bit
    Ip,  // 16-bit
}
