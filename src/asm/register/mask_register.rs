/// AVX-512 mask registers for per-element predication.
///
/// Eight 64-bit mask registers (k0-k7) control which vector elements are
/// processed. Each bit enables/disables corresponding element. Supports
/// zeroing (masked elements = 0) and merging (masked elements preserved).
/// K0 is special: cannot be used as write mask (always all-ones).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaskRegister {
    /// K0 - Cannot be used as write mask, treated as all-ones when source.
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
}
