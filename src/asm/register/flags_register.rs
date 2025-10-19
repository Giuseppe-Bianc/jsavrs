/// CPU flags register for different operand sizes.
///
/// Contains status flags (CF, ZF, SF, OF, etc.), control flags (DF, IF),
/// and system flags (IOPL, NT, RF, VM). Size variant depends on processor
/// mode and instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagsRegister {
    /// 64-bit flags register (Long Mode). Upper 32 bits reserved.
    Rflags,
    /// 32-bit flags register (Protected Mode).
    Eflags,
    /// 16-bit flags register (Real Mode). Lower 16 bits of EFLAGS/RFLAGS.
    Flags,
}