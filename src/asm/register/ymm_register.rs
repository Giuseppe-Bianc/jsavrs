/// YMM registers for AVX operations (256-bit SIMD).
///
/// 16 registers extending XMM to 256 bits. Lower 128 bits are corresponding
/// XMM registers. Supports 32×8-bit, 16×16-bit, 8×32-bit, 4×64-bit elements.
/// Use VZEROUPPER before SSE code to avoid performance penalties. Volatility
/// follows XMM rules; upper 128 bits require preservation where XMM is non-volatile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YMMRegister {
    Ymm0,
    Ymm1,
    Ymm2,
    Ymm3,
    Ymm4,
    Ymm5,
    Ymm6,
    Ymm7,
    Ymm8,
    Ymm9,
    Ymm10,
    Ymm11,
    Ymm12,
    Ymm13,
    Ymm14,
    Ymm15,
}
