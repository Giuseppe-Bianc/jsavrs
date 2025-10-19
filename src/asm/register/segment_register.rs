/// Segment registers for x86 memory segmentation.
///
/// Store segment selectors referencing GDT/LDT descriptors. In 64-bit mode,
/// CS/DS/ES/SS largely ignored (flat memory). FS/GS remain useful for
/// thread-local storage (TLS) and per-CPU data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentRegister {
    /// Code Segment - Executable code. Modified via far JMP/CALL/IRET only.
    Cs,
    /// Data Segment - Default for most memory accesses.
    Ds,
    /// Extra Segment - Additional data, often string operation destination.
    Es,
    /// FS Segment - Thread-local storage (Linux). Base via WRFSBASE/MSR.
    Fs,
    /// GS Segment - TLS (Windows user), per-CPU (kernel). Base via WRGSBASE/SWAPGS.
    Gs,
    /// Stack Segment - Default for stack operations with RSP/ESP/SP.
    Ss,
}
