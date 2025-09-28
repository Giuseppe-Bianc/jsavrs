//! Calling convention implementations for different platforms
//!
//! Provides platform-specific ABI implementations including Windows x64
//! and System V ABI as specified in the data model and contract specifications.

pub mod windows_x64;
pub mod system_v;

use crate::asm::register::{GPRegister, XMMRegister, Register};
use crate::asm::instruction::X86Instruction;
use crate::asm::generator::FunctionContext;

/// Trait defining calling convention behavior
pub trait CallingConvention: Send + Sync {
    /// Registers used for integer/pointer parameters
    fn integer_param_registers(&self) -> &[GPRegister];
    
    /// Registers used for floating-point parameters
    fn float_param_registers(&self) -> &[XMMRegister];
    
    /// Register for return values
    fn return_registers(&self) -> (Option<GPRegister>, Option<XMMRegister>);
    
    /// Caller-saved (volatile) registers
    fn caller_saved_registers(&self) -> &[Register];
    
    /// Callee-saved (non-volatile) registers
    fn callee_saved_registers(&self) -> &[Register];
    
    /// Required stack alignment in bytes
    fn stack_alignment(&self) -> u32;
    
    /// Shadow space size (Windows-specific)
    fn shadow_space_size(&self) -> u32;
    
    /// Generate function prologue
    fn generate_prologue(&self, ctx: &FunctionContext) -> Vec<X86Instruction>;
    
    /// Generate function epilogue
    fn generate_epilogue(&self, ctx: &FunctionContext) -> Vec<X86Instruction>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConventionType {
    WindowsX64,
    SystemV,
}