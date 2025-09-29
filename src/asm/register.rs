//! # Register Management
//!
//! Defines register types and allocation strategies for the assembly generator.

use std::collections::HashMap;
use std::fmt;

/// General-purpose 64-bit registers for x86-64 architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPRegister {
    RAX, RBX, RCX, RDX, RSI, RDI, RSP, RBP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

impl fmt::Display for GPRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// XMM (SSE) registers for floating-point operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XMMRegister {
    XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7,
    XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15,
}

impl fmt::Display for XMMRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Unified register enum that includes both general-purpose and XMM registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    GP(GPRegister),
    XMM(XMMRegister),
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::GP(gp_reg) => write!(f, "{}", gp_reg),
            Register::XMM(xmm_reg) => write!(f, "{}", xmm_reg),
        }
    }
}

/// Public interface for register allocation information
pub trait RegisterInfo {
    /// Get available general-purpose registers for allocation
    fn available_gp_registers(&self) -> Vec<GPRegister>;
    
    /// Get available XMM registers for allocation
    fn available_xmm_registers(&self) -> Vec<XMMRegister>;
    
    /// Check if register is caller-saved (volatile)
    fn is_caller_saved(&self, register: Register) -> bool;
    
    /// Check if register is callee-saved (non-volatile)
    fn is_callee_saved(&self, register: Register) -> bool;
}

/// Register allocation statistics for debugging
#[derive(Debug, Clone)]
pub struct AllocationStats {
    pub registers_used: usize,
    pub registers_spilled: usize,
    pub stack_bytes_used: u32,
    pub allocation_pressure: f64, // 0.0 to 1.0
}

/// Strategy for allocating registers during assembly generation.
pub struct RegisterAllocator {
    /// List of currently available GP registers
    pub available_gp_registers: Vec<GPRegister>,
    /// List of currently available XMM registers
    pub available_xmm_registers: Vec<XMMRegister>,
    /// Mapping between IR values and physical registers
    pub allocated_map: HashMap<String, Register>,
    /// Next stack location for spilling registers
    pub spill_location: usize,
    /// Statistics for allocation
    pub stats: AllocationStats,
}

impl RegisterAllocator {
    /// Creates a new RegisterAllocator with default available registers
    pub fn new() -> Self {
        // Add general-purpose registers (caller-saved and callee-saved)
        let available_gp_registers = vec![
            GPRegister::RAX, GPRegister::RCX, GPRegister::RDX,
            GPRegister::RSI, GPRegister::RDI, GPRegister::R8,
            GPRegister::R9, GPRegister::R10, GPRegister::R11,
        ];
        
        // Add XMM registers (caller-saved)
        let available_xmm_registers = vec![
            XMMRegister::XMM0, XMMRegister::XMM1, XMMRegister::XMM2,
            XMMRegister::XMM3, XMMRegister::XMM4, XMMRegister::XMM5,
        ];
        
        RegisterAllocator {
            available_gp_registers,
            available_xmm_registers,
            allocated_map: HashMap::new(),
            spill_location: 0,
            stats: AllocationStats {
                registers_used: 0,
                registers_spilled: 0,
                stack_bytes_used: 0,
                allocation_pressure: 0.0,
            },
        }
    }

    /// Assigns a physical register to an IR value (deprecated - use type-specific methods)
    pub fn allocate_register(&mut self, ir_value: &str) -> Option<Register> {
        // For backward compatibility, allocate a GP register by default
        self.allocate_gp_register(ir_value).map(Register::GP)
    }

    /// Assigns a general-purpose register to an IR value
    pub fn allocate_gp_register(&mut self, ir_value: &str) -> Option<GPRegister> {
        if let Some(reg) = self.available_gp_registers.pop() {
            self.allocated_map.insert(ir_value.to_string(), Register::GP(reg));
            self.stats.registers_used += 1;
            Some(reg)
        } else {
            None
        }
    }

    /// Assigns an XMM register to an IR value
    pub fn allocate_xmm_register(&mut self, ir_value: &str) -> Option<XMMRegister> {
        if let Some(reg) = self.available_xmm_registers.pop() {
            self.allocated_map.insert(ir_value.to_string(), Register::XMM(reg));
            self.stats.registers_used += 1;
            Some(reg)
        } else {
            None
        }
    }

    /// Moves a value from register to stack when needed
    pub fn spill_to_stack(&mut self, ir_value: &str) -> usize {
        if let Some(reg) = self.allocated_map.remove(ir_value) {
            match reg {
                Register::GP(gp_reg) => self.available_gp_registers.push(gp_reg),
                Register::XMM(xmm_reg) => self.available_xmm_registers.push(xmm_reg),
            }
            self.stats.registers_spilled += 1;
        }
        let location = self.spill_location;
        self.spill_location += 1;
        location
    }

    /// Marks a register as available - doesn't remove from mapping (as we need to track which register holds what value)
    /// To remove a mapping between a value and register, use the spill_to_stack method
    pub fn free_register(&mut self, reg: Register) {
        match reg {
            Register::GP(gp_reg) => self.available_gp_registers.push(gp_reg),
            Register::XMM(xmm_reg) => self.available_xmm_registers.push(xmm_reg),
        }
    }
    
    /// Get allocation statistics
    pub fn get_stats(&self) -> &AllocationStats {
        &self.stats
    }
}

impl RegisterInfo for RegisterAllocator {
    /// Get available general-purpose registers for allocation
    fn available_gp_registers(&self) -> Vec<GPRegister> {
        self.available_gp_registers.clone()
    }
    
    /// Get available XMM registers for allocation
    fn available_xmm_registers(&self) -> Vec<XMMRegister> {
        self.available_xmm_registers.clone()
    }
    
    /// Check if register is caller-saved (volatile)
    fn is_caller_saved(&self, register: Register) -> bool {
        match register {
            Register::GP(gp_reg) => {
                matches!(gp_reg, 
                    GPRegister::RAX | GPRegister::RCX | GPRegister::RDX | 
                    GPRegister::RSI | GPRegister::RDI | GPRegister::R8 | 
                    GPRegister::R9 | GPRegister::R10 | GPRegister::R11)
            },
            Register::XMM(xmm_reg) => {
                matches!(xmm_reg,
                    XMMRegister::XMM0 | XMMRegister::XMM1 | XMMRegister::XMM2 |
                    XMMRegister::XMM3 | XMMRegister::XMM4 | XMMRegister::XMM5 |
                    XMMRegister::XMM6 | XMMRegister::XMM7 | XMMRegister::XMM8 |
                    XMMRegister::XMM9 | XMMRegister::XMM10 | XMMRegister::XMM11 |
                    XMMRegister::XMM12 | XMMRegister::XMM13 | XMMRegister::XMM14 |
                    XMMRegister::XMM15)
            }
        }
    }
    
    /// Check if register is callee-saved (non-volatile)
    fn is_callee_saved(&self, register: Register) -> bool {
        match register {
            Register::GP(gp_reg) => {
                matches!(gp_reg,
                    GPRegister::RBX | GPRegister::RBP | GPRegister::R12 |
                    GPRegister::R13 | GPRegister::R14 | GPRegister::R15)
            },
            Register::XMM(_) => false, // XMM registers are caller-saved in most ABIs
        }
    }
}