//! # Code Generation Module
//!
//! The `codegen` module is responsible for transforming intermediate representations (IR)
//! into target-specific assembly or machine code. It serves as the backend of the
//! compilation pipeline, converting high-level abstractions into executable instructions.
//
//! ## Overview
//!
//! This module provides the core code generation infrastructure, including:
//!
//! - **Assembly Generation** ([`asmgen`]): Low-level assembly code emission for supported
//!   target architectures.
//!
//! ## Key Responsibilities
//!
//! 1. Instruction selection and lowering from IR to target instructions.
//! 2. Register allocation and stack frame management.
//! 3. Assembly code emission with proper formatting and directives.
//! 4. Target-specific optimizations and code scheduling.
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use crate::codegen::asmgen::AsmGenerator;
//!
//! // Create an assembly generator for the target architecture
//! let generator = AsmGenerator::new(target_config);
//!
//! // Generate assembly from the IR module
//! let assembly_output = generator.generate(&ir_module)?;
//! ```
//!
//! ## Integration Guidelines
//!
//! The `codegen` module typically receives input from:
//! - The IR/optimization passes that produce lowered, target-ready IR.
//!
//! Output is consumed by:
//! - Assemblers or object file writers for final binary generation.
//!
//! ## Performance Characteristics
//!
//! Code generation complexity is generally O(n) with respect to the number of IR
//! instructions, though certain optimizations (e.g., register allocation) may have
//! higher complexity depending on the algorithm used.
//!
//! ## Related Modules
//!
//! - `ir`: Intermediate representation that serves as input to code generation.
//! - `target`: Target architecture specifications and configurations.
//!

/// Assembly code generation submodule.
///
/// Provides infrastructure for emitting assembly instructions, managing labels,
/// and formatting output for various target architectures.
pub mod asmgen;