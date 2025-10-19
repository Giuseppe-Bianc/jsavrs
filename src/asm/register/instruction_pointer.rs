
/// Registro instruction pointer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionPointer {
    Rip, // 64-bit
    Eip, // 32-bit
    Ip,  // 16-bit
}