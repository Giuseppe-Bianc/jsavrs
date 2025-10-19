/// ZMM registers for AVX-512 operations (512-bit SIMD).
///
/// 32 registers (ZMM0-ZMM31, 16 in non-64-bit modes) for maximum SIMD width.
/// Lower portions are XMM/YMM registers. Supports 64×8-bit, 32×16-bit,
/// 16×32-bit, 8×64-bit elements. Features: per-element masking, embedded
/// broadcast/rounding, compressed displacement. May reduce CPU frequency on
/// some processors. Check CPUID for AVX-512 variants (F, BW, DQ, VL, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZMMRegister {
    Zmm0,
    Zmm1,
    Zmm2,
    Zmm3,
    Zmm4,
    Zmm5,
    Zmm6,
    Zmm7,
    Zmm8,
    Zmm9,
    Zmm10,
    Zmm11,
    Zmm12,
    Zmm13,
    Zmm14,
    Zmm15,
    Zmm16,
    Zmm17,
    Zmm18,
    Zmm19,
    Zmm20,
    Zmm21,
    Zmm22,
    Zmm23,
    Zmm24,
    Zmm25,
    Zmm26,
    Zmm27,
    Zmm28,
    Zmm29,
    Zmm30,
    Zmm31,
}
