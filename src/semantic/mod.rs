//! # Semantic Analysis Module
//!
//! The semantic analysis module handles the verification of syntactic correctness
//! and type checking of the abstract syntax tree. This is the third phase of the
//! compilation process, responsible for ensuring the code follows semantic rules.
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Sets up symbol tables and type checking context
//! * Runtime: Processes AST nodes to verify types and relationships
//! * Termination: Finalizes symbol table and reports semantic errors
pub mod symbol_table;
pub mod type_checker;
