// src/parser/mod.rs
//! # Parser Module
//!
//! The parser module handles the transformation of tokens into abstract syntax trees (AST).
//! This is the second phase of the compilation process, responsible for recognizing
//! syntactic structures according to the language grammar.
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Sets up the parsing context and grammar rules
//! * Runtime: Processes token stream to build AST nodes according to grammar
//! * Termination: Finalizes AST with proper structure and error reporting
pub mod ast;
pub mod jsav_parser;
pub mod precedence;
