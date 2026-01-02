//! # Error Module
//!
//! The error module handles error definition and reporting throughout the compilation
//! process. It provides structured error types, error codes, and reporting mechanisms
//! for all compiler phases.
//!
//! ## Components
//!
//! * [`compile_error`]: Main error type enum for all compilation errors
//! * [`error_code`]: Standardized error codes for identification and documentation
//! * [`error_reporter`]: Formatted error output with source context
//!
//! ## Phase-specific responsibilities:
//! * Initialization: Sets up error type definitions and reporting infrastructure
//! * Runtime: Captures and categorizes errors from different compilation phases
//! * Termination: Provides comprehensive error reporting before compilation failure
//!
//! ## Error Code System
//!
//! The error code system provides unique identifiers for each error type:
//!
//! | Range | Phase | Description |
//! |-------|-------|-------------|
//! | E0001-E0999 | Lexical Analysis | Token recognition, literals, comments |
//! | E1001-E1999 | Parsing | Syntax structure, grammar violations |
//! | E2001-E2999 | Semantic Analysis | Types, scopes, declarations |
//! | E3001-E3999 | IR Generation | CFG, SSA, control flow |
//! | E4001-E4999 | Code Generation | Assembly, ABI, registers |
//! | E5001-E5999 | I/O & System | File operations, CLI |
//!
//! ## Example
//!
//! ```rust
//! use jsavrs::error::error_code::{ErrorCode, Severity};
//!
//! let code = ErrorCode::E2023;
//! println!("Error {}: {}", code.code(), code.message());
//! assert_eq!(code.severity(), Severity::Error);
//! ```
pub mod compile_error;
pub mod error_code;
pub mod error_reporter;
