//! # Printers Module
//!
//! The printers module handles the generation of target code from the intermediate
//! representation. This is the final phase of the compilation process, responsible
//! for producing the output in various formats (e.g., assembly code, bytecode).
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Sets up code generation context and target format specifications
//! * Runtime: Processes IR nodes to generate target code with proper formatting
//! * Termination: Finalizes output with complete program representation
pub mod ast_printer;
pub mod branch_type;
