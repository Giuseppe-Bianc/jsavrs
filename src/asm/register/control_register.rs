/// x86-64 control register definitions.
///
/// Control registers configure processor operating modes and enable/disable
/// processor features. They require privileged mode (ring 0) access via
/// special MOV instructions. CR1, CR5, CR6, CR7 are reserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlRegister {
    /// Control Register 0: Processor operating mode and state flags.
    /// Key bits: PE (protection enable), PG (paging), WP (write protect).
    Cr0,
    
    /// Control Register 2: Page fault linear address.
    /// Automatically loaded with the faulting address on page faults.
    Cr2,
    
    /// Control Register 3: Physical address of page directory base.
    /// Writing to CR3 flushes the TLB. Used for process context switching.
    Cr3,
    
    /// Control Register 4: Processor feature control flags.
    /// Enables PSE (page size extension), PAE, PGE, OSFXSR, OSXSAVE, etc.
    Cr4,
    
    /// Control Register 8: Task priority register (64-bit mode only).
    /// Controls interrupt priority levels via TPR access.
    Cr8,
}