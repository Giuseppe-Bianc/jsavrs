/// MMX registers for legacy 64-bit SIMD operations.
///
/// Eight 64-bit registers (MM0-MM7) for integer SIMD. Aliased to x87 FPU
/// register mantissas; using MMX requires EMMS before FPU operations.
/// Obsolete; modern code should use XMM/YMM/ZMM registers instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MMXRegister {
    Mm0,
    Mm1,
    Mm2,
    Mm3,
    Mm4,
    Mm5,
    Mm6,
    Mm7,
}
