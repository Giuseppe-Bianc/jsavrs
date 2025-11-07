//! # Tokens Module
//!
//! The tokens module defines the lexical tokens used by the lexer and parser.
//! It provides token types and classification for the language being compiled.
//!
//! ## Module Structure
//!
//! - `number`: Numeric literal type definitions
//! - `token`: Token structure combining kind and location
//! - `token_kind`: Token type enumeration and classification
//! - `parsers`: Numeric literal parsing logic
//!   - `numeric`: Core decimal number parsing
//!   - `suffix`: Type suffix detection and routing
//!   - `base`: Binary, octal, and hexadecimal parsing
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Defines all possible token types for the language
//! * Runtime: Provides token classification during lexical analysis
//! * Termination: Tokens are consumed in subsequent compilation phases
pub mod number;
pub mod parsers;
pub mod token;
pub mod token_kind;
