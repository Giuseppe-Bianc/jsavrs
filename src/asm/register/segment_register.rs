/// Registri di segmento
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentRegister {
    Cs,
    Ds,
    Es,
    Fs,
    Gs,
    Ss,
}
