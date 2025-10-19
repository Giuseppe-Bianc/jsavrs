/// XMM registers for SSE operations (128-bit SIMD).
///
/// 16 registers (XMM0-XMM15, 8 in 32-bit mode) for packed integers and
/// floats. Supports 16×8-bit, 8×16-bit, 4×32-bit, 2×64-bit elements.
/// Lower 128 bits of YMM registers; writing zeros upper YMM/ZMM bits.
///
/// Calling conventions:
/// - Windows: XMM0-XMM5 volatile, XMM6-XMM15 non-volatile.
/// - System V: All volatile. XMM0-XMM7 for float params, XMM0-XMM1 for returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XMMRegister {
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
    Xmm8,
    Xmm9,
    Xmm10,
    Xmm11,
    Xmm12,
    Xmm13,
    Xmm14,
    Xmm15,
}
