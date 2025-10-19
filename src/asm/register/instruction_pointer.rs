/// Instruction pointer register for different processor modes.
///
/// Contains offset of next instruction to execute. Modified by control flow
/// instructions (JMP, CALL, RET), not by direct MOV. RIP-relative addressing
/// in 64-bit mode enables position-independent code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionPointer {
    /// 64-bit instruction pointer. Supports RIP-relative addressing for PIC.
    Rip,
    /// 32-bit instruction pointer (Protected/Compatibility Mode).
    Eip,
    /// 16-bit instruction pointer (Real Mode). Combined with CS as CS:IP.
    Ip,
}
