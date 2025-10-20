//! # Assembly Module
//!
//! The assembly module handles the generation of assembly code from intermediate 
//! representation. This module is responsible for translating IR into specific 
//! assembly instructions for the target architecture.
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Sets up target architecture specifications and register allocation
//! * Runtime: Translates IR operations to appropriate assembly instructions
//! * Termination: Finalizes assembly with proper linking information and directives
mod abi;
mod assembly_file;
mod data_directive;
mod instruction;
mod platform;
mod register;
mod section;

#[allow(unused_imports)]
pub use abi::*;
pub use assembly_file::*;
#[allow(unused_imports)]
pub use data_directive::*;
pub use instruction::*;
pub use platform::*;
pub use register::*;
pub use section::*;
