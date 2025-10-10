//! Type definitions for the type promotion system.
//!
//! This module contains all the core type definitions used throughout the type promotion system,
//! including rules, warnings, behaviors, and result types.

use crate::ir::{CastKind, IrBinaryOp, IrType};
use crate::location::source_span::SourceSpan;

/// Represents a single type promotion operation
#[derive(Debug, Clone, PartialEq)]
pub struct TypePromotion {
    pub from_type: IrType,
    pub to_type: IrType,
    pub cast_kind: CastKind,
    pub may_lose_precision: bool,
    pub may_overflow: bool,
    pub source_span: SourceSpan,
}

impl TypePromotion {
    /// Creates a new type promotion
    pub fn new(from_type: IrType, to_type: IrType, cast_kind: CastKind, source_span: SourceSpan) -> Self {
        Self { from_type, to_type, cast_kind, may_lose_precision: false, may_overflow: false, source_span }
    }

    /// Creates a new type promotion with all parameters
    pub fn with_flags(
        from_type: IrType, to_type: IrType, cast_kind: CastKind, may_lose_precision: bool, may_overflow: bool,
        source_span: SourceSpan,
    ) -> Self {
        Self { from_type, to_type, cast_kind, may_lose_precision, may_overflow, source_span }
    }

    /// Returns true if this is a widening conversion (lossless)
    pub fn is_widening(&self) -> bool {
        matches!(self.cast_kind, CastKind::IntZeroExtend | CastKind::IntSignExtend | CastKind::FloatExtend)
    }

    /// Returns true if this is a narrowing conversion (may lose precision)
    pub fn is_narrowing(&self) -> bool {
        matches!(self.cast_kind, CastKind::IntTruncate | CastKind::FloatTruncate)
    }
}

/// Defines specific promotion behavior between two types
#[derive(Debug, Clone, PartialEq)]
pub enum PromotionRule {
    /// Direct promotion without intermediate steps
    Direct {
        cast_kind: CastKind,
        may_lose_precision: bool,
        may_overflow: bool,
        requires_runtime_support: bool,
        requires_validation: bool,
        precision_loss_estimate: Option<PrecisionLossEstimate>,
    },
    /// Promotion through intermediate type
    Indirect { intermediate_type: IrType, first_cast: CastKind, second_cast: CastKind, requires_runtime_support: bool },
    /// Promotion not allowed
    Forbidden { reason: String },
}

/// Groups types by mathematical properties for promotion ordering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeGroup {
    SignedIntegers(Vec<IrType>),
    UnsignedIntegers(Vec<IrType>),
    FloatingPoint(Vec<IrType>),
    Boolean,
    Character,
}

/// Contains the result of type promotion analysis
#[derive(Debug, Clone, PartialEq)]
pub struct PromotionResult {
    /// The target type for the operation result
    pub result_type: IrType,
    /// Cast required for left operand (if any)
    pub left_cast: Option<TypePromotion>,
    /// Cast required for right operand (if any)
    pub right_cast: Option<TypePromotion>,
    /// Warnings generated during promotion analysis
    pub warnings: Vec<PromotionWarning>,
    /// Whether the promotion is mathematically sound
    pub is_sound: bool,
}

/// Represents warnings generated during type promotion
#[derive(Debug, Clone, PartialEq)]
pub enum PromotionWarning {
    PrecisionLoss {
        from_type: IrType,
        to_type: IrType,
        estimated_loss: PrecisionLossEstimate,
    },
    PotentialOverflow {
        from_type: IrType,
        to_type: IrType,
        operation: IrBinaryOp,
    },
    SignednessChange {
        from_signed: bool,
        to_signed: bool,
        may_affect_comparisons: bool,
    },
    /// Float special values in type conversions
    FloatSpecialValues {
        value_type: FloatSpecialValueType,
        source_type: IrType,
        target_type: IrType,
        applied_behavior: OverflowBehavior,
        source_span: SourceSpan,
    },
    /// Invalid string conversion (unparseable)
    InvalidStringConversion {
        string_value: Option<String>,
        target_type: IrType,
        reason: String,
    },
    /// Invalid Unicode code point for char
    InvalidUnicodeCodePoint {
        value: u32,
        reason: String,
    },
}

/// Helper enum for float special value types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSpecialValueType {
    NaN,
    PositiveInfinity,
    NegativeInfinity,
}

/// Configuration for runtime behavior on numeric overflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverflowBehavior {
    Wrap,
    Saturate,
    Trap,
    CompileError,
}

/// Quantifies potential precision loss in type conversions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecisionLossEstimate {
    None,
    FractionalPart,
    SignificantDigits { lost_bits: u32 },
    ValueRange { from_bits: u32, to_bits: u32 },
}

/// Binary operation promotion information
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperationPromotion {
    pub operation: IrBinaryOp,
    pub left_type: IrType,
    pub right_type: IrType,
    pub result: PromotionResult,
}
