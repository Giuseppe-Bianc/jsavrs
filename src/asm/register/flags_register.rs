/// Registro dei flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagsRegister {
    Rflags, // 64-bit
    Eflags, // 32-bit
    Flags,  // 16-bit
}
