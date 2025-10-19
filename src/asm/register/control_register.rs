/// x86-64 control register definitions.
///
/// This module provides an enumeration of control registers used in x86-64
/// architecture for system-level operations such as memory management,
/// protection, and processor feature control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlRegister {
    /// Control Register 0: Controls processor operating mode and state flags.
    Cr0,
    /// Control Register 2: Contains the page fault linear address.
    Cr2,
    /// Control Register 3: Contains the physical address of the page directory base.
    Cr3,
    /// Control Register 4: Controls various processor features and extensions.
    Cr4,
    /// Control Register 8: Task priority register (available in 64-bit mode only).
    Cr8,
}
