// src/tokens/parsers/mod.rs
//! Numeric literal parsing modules.
//!
//! This module contains all parsing logic for converting string representations
//! of numeric literals into structured `Number` types during lexical analysis.
//!
//! # Overview
//!
//! The parsers module provides three complementary submodules for parsing numeric
//! literals: base detection, suffix parsing, and numeric value extraction.
//! 
//! # Submodules
//!
//! - [`numeric`]: Core numeric literal parsing logic
//! - [`suffix`]: Type suffix parsing (e.g., `u32`, `f64`)
//! - [`base`]: Numeric base detection (binary, octal, decimal, hexadecimal)
pub mod numeric;
pub mod suffix;
pub mod base;
