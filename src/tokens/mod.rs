//! # Tokens Module
//!
//! The tokens module defines the lexical tokens used by the lexer and parser.
//! It provides token types and classification for the language being compiled.
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Defines all possible token types for the language
//! * Runtime: Provides token classification during lexical analysis
//! * Termination: Tokens are consumed in subsequent compilation phases
pub mod number;
pub mod token;
pub mod token_kind;
