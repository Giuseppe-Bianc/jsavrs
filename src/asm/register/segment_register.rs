/// Registri di segmento
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentRegister {
    /// Code Segment register - points to the segment containing executable code
    Cs,
    /// Data Segment register - points to the segment containing data
    Ds,
    /// Extra Segment register - additional data segment
    Es,
    /// FS Segment register - general-purpose segment register
    Fs,
    /// GS Segment register - general-purpose segment register
    Gs,
    /// Stack Segment register - points to the segment containing the stack
    Ss,
}
