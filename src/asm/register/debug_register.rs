/// Debug registers for hardware breakpoints and debugging.
///
/// Provides hardware-level debugging support: DR0-DR3 store breakpoint
/// addresses, DR6 reports debug status, DR7 configures breakpoint conditions.
/// Supports execution, read, write, and I/O breakpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugRegister {
    /// Debug register 0 - Linear address for first hardware breakpoint.
    Dr0,
    /// Debug register 1 - Linear address for second hardware breakpoint.
    Dr1,
    /// Debug register 2 - Linear address for third hardware breakpoint.
    Dr2,
    /// Debug register 3 - Linear address for fourth hardware breakpoint.
    Dr3,
    
    /// Debug register 6 - Debug status register.
    /// Reports which breakpoint triggered (B0-B3), single-step (BS), etc.
    /// Software must manually clear status bits.
    Dr6,
    
    /// Debug register 7 - Debug control register.
    /// Configures breakpoint enable (L0-L3, G0-G3), conditions (RW), and
    /// length (LEN) for all hardware breakpoints.
    Dr7,
}
