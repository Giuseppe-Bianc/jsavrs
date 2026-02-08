// src/lib.rs
/// # jsavrs Compiler Library
///
/// The jsavrs compiler library is a transpiler written in Rust that provides
/// a complete toolchain for compiling source code through various phases:
/// lexical analysis, parsing, semantic analysis, intermediate representation,
/// and code generation.
///
/// ## Phase-specific responsibilities:
/// * Initialization: Module system is set up and dependencies are loaded
/// * Runtime: Individual modules process source code through the compilation pipeline
/// * Termination: Compilation process completes and output is generated
///
/// ## Important modules:
/// * `lexer` - Performs lexical analysis, converting source text to tokens
/// * `parser` - Converts tokens into abstract syntax trees (AST)
/// * `semantic` - Performs semantic analysis and type checking
/// * `ir` - Manages intermediate representation of the code
/// * `printers` - Handles pretty-printing and formatted output of AST/IR for debugging
/// * `asm` - Manages assembly code generation
/// * `fmtlike` - Provides utilities for formatting and pretty-printing
/// * `codegen` - Orchestrates code generation, dispatching to backend-specific emitters (e.g., asm)
/// * `error` - Defines error types and error handling utilities
/// * `cli` - Provides command-line interface functionality
pub mod asm;
pub mod cli;
pub mod codegen;
pub mod error;
pub mod fmtlike;
pub mod ir;
pub mod lexer;
pub mod location;
pub mod parser;
pub mod printers;
pub mod semantic;
pub mod time;
pub mod tokens;
pub mod utils;
