/// Registri di debug
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugRegister {
    /// Debug register 0 - Breakpoint address
    Dr0,
    /// Debug register 1 - Breakpoint address
    Dr1,
    /// Debug register 2 - Breakpoint address
    Dr2,
    /// Debug register 3 - Breakpoint address
    Dr3,
    /// Debug register 6 - Debug status
    Dr6,
    /// Debug register 7 - Debug control
    Dr7,
}
