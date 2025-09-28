//! # Assembly Code Generation Module for jsavrs
//!
//! This module provides functionality for translating jsavrs intermediate representation (IR)
//! into x86-64 assembly code compatible with NASM syntax. The implementation focuses on:
//!
//! - Semantic preservation between IR and generated assembly
//! - Cross-platform ABI compliance (Windows x64, System V)
//! - Type-safe register management
//! - Efficient instruction encoding using iced-x86
//!
//! This module implements the core assembly generation pipeline that processes jsavrs IR
//! and produces NASM-compatible x86-64 assembly code.
//!
//! ## Example Usage
//!
//! ```rust
//! use jsavrs::asm::{AssemblyGenerator, TargetPlatform};
//! use jsavrs::ir::module::Module;
//!
//! // Create an assembly generator for Linux x86-64
//! let mut generator = AssemblyGenerator::new(TargetPlatform::linux_x64())
//!     .expect("Failed to create generator");
//!
//! // Generate assembly for an IR module
//! // let ir_module = /* your IR module */;
//! // let assembly = generator.generate_assembly(ir_module)
//! //     .expect("Assembly generation failed");
//!
//! // The resulting assembly string can be saved to a file and assembled with NASM
//! ```

pub mod generator;
pub mod register;
pub mod instruction;
pub mod operand;
pub mod platform;
pub mod options;
pub mod error;
pub mod abi;

// Re-export key types at the module level
pub use generator::AssemblyGenerator;
pub use register::{Register, GPRegister, XMMRegister};
pub use platform::TargetPlatform;
pub use error::CodeGenError;