//! # Type Promotion Engine
//!
//! This module provides the core logic for analyzing and implementing type promotions in binary operations.
//! The engine uses a promotion matrix to determine the correct target type for operations involving
//! operands of different types, and generates appropriate cast instructions to ensure type safety
//! and mathematical correctness.
//!
//! ## Responsibilities
//!
//! - Analyze binary operations for required type promotions (`analyze_binary_promotion`)
//! - Insert necessary cast instructions (`insert_promotion_casts`)
//! - Generate appropriate warnings for precision loss or overflow
//! - Handle special cases like signed/unsigned mixing
//!
//! ## Key Functions
//!
//! - `analyze_binary_promotion`: Determines the target type and required casts for a binary operation
//! - `insert_promotion_casts`: Inserts the necessary cast instructions into the IR
//!
//! ## Example Usage
//!
//! ```rust
//! use jsavrs::ir::type_promotion_engine::TypePromotionEngine;
//! use jsavrs::ir::types::IrType;
//! use jsavrs::ir::instruction::IrBinaryOp;
//!
//! let engine = TypePromotionEngine::new();
//! // Use engine to analyze and promote types in binary operations
//! ```

use super::{
    CastKind, Instruction, InstructionKind, IrBinaryOp, IrType, PromotionMatrix, PromotionResult, PromotionWarning,
    TypePromotion, Value,
};
use crate::ir::generator::IrGenerator;
use crate::location::source_span::SourceSpan;
//use once_cell::sync::Lazy;
use std::sync::LazyLock;

// Global singleton PromotionMatrix - initialized once and reused across all operations
// This eliminates ~4.5MB of allocations for repeated matrix initialization
static GLOBAL_PROMOTION_MATRIX: LazyLock<PromotionMatrix> = LazyLock::new(PromotionMatrix::new);

#[derive(Debug, Clone, Default)]
pub struct TypePromotionEngine;

impl TypePromotionEngine {
    /// Creates a new TypePromotionEngine that uses the global singleton PromotionMatrix
    pub const fn new() -> Self {
        Self
    }

    /// Returns a reference to the global promotion matrix singleton
    fn get_promotion_matrix(&self) -> &PromotionMatrix {
        &GLOBAL_PROMOTION_MATRIX
    }

    /// Analyzes binary operation for proper type promotion
    #[must_use]
    pub fn analyze_binary_promotion(
        &self, left_type: &IrType, right_type: &IrType, operation: IrBinaryOp, span: SourceSpan,
    ) -> PromotionResult {
        // Use global singleton promotion matrix
        let promotion_matrix = self.get_promotion_matrix();

        // Compute the target result type based on the promotion matrix
        let result_type =
            promotion_matrix.compute_common_type(left_type, right_type).unwrap_or_else(|| left_type.clone()); // fallback to left type if no promotion found

        let mut warnings = Vec::new();
        let mut left_cast = None;
        let mut right_cast = None;

        // Check if left operand needs casting
        if left_type != &result_type
            && let Some(rule) = promotion_matrix.get_promotion_rule(left_type, &result_type)
            && let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule
        {
            left_cast = Some(TypePromotion {
                from_type: left_type.clone(),
                to_type: result_type.clone(),
                cast_kind: *cast_kind,
                may_lose_precision: *may_lose_precision,
                may_overflow: *may_overflow,
                source_span: span.clone(),
            });

            // Add warnings if applicable
            if *may_lose_precision {
                warnings.push(PromotionWarning::PrecisionLoss {
                    from_type: left_type.clone(),
                    to_type: result_type.clone(),
                    estimated_loss: PrecisionLossEstimate::None, // Determine more precisely as needed
                });
            }
            if *may_overflow {
                warnings.push(PromotionWarning::PotentialOverflow {
                    from_type: left_type.clone(),
                    to_type: result_type.clone(),
                    operation,
                });
            }
        }

        // Check if right operand needs casting
        if right_type != &result_type
            && let Some(rule) = promotion_matrix.get_promotion_rule(right_type, &result_type)
            && let PromotionRule::Direct { cast_kind, may_lose_precision, may_overflow, .. } = rule
        {
            right_cast = Some(TypePromotion {
                from_type: right_type.clone(),
                to_type: result_type.clone(),
                cast_kind: *cast_kind,
                may_lose_precision: *may_lose_precision,
                may_overflow: *may_overflow,
                source_span: span,
            });

            // Add warnings if applicable
            if *may_lose_precision {
                warnings.push(PromotionWarning::PrecisionLoss {
                    from_type: right_type.clone(),
                    to_type: result_type.clone(),
                    estimated_loss: PrecisionLossEstimate::None, // Determine more precisely as needed
                });
            }
            if *may_overflow {
                warnings.push(PromotionWarning::PotentialOverflow {
                    from_type: right_type.clone(),
                    to_type: result_type.clone(),
                    operation,
                });
            }
        }

        // Check for signedness changes between operands of same width
        if (left_type.is_signed_integer()
            && right_type.is_unsigned_integer()
            && left_type.get_bit_width() == right_type.get_bit_width())
            || (left_type.is_unsigned_integer()
                && right_type.is_signed_integer()
                && left_type.get_bit_width() == right_type.get_bit_width())
        {
            warnings.push(PromotionWarning::SignednessChange {
                from_signed: left_type.is_signed_integer(),
                to_signed: result_type.is_signed_integer(),
                may_affect_comparisons: true,
            });
        }

        PromotionResult {
            result_type,
            left_cast,
            right_cast,
            warnings: warnings.clone(),
            is_sound: warnings.is_empty(), // Simplified - in reality, presence of warning doesn't mean promotion is unsound
                                           /*is_sound: !warnings.iter().any(|w| matches!(w, PromotionWarning::PrecisionLoss { .. } | PromotionWarning::PotentialOverflow { .. } | PromotionWarning::SignednessChange { .. })),*/
        }
    }

    /// Inserts promotion casts for binary operations
    pub fn insert_promotion_casts(
        &self,
        generator: &mut IrGenerator,
        func: &mut super::function::Function, // Function is now used in the implementation
        left_value: Value,
        right_value: Value,
        promotion_result: &PromotionResult,
        span: SourceSpan,
    ) -> (Value, Value) {
        let mut new_left_value = left_value;
        let mut new_right_value = right_value;

        // Insert cast for left operand if needed
        if let Some(ref left_cast_info) = promotion_result.left_cast {
            new_left_value = self.insert_cast_instruction(
                generator,
                func,
                new_left_value,
                &left_cast_info.to_type,
                left_cast_info.cast_kind,
                span.clone(),
            );
        }

        // Insert cast for right operand if needed
        if let Some(ref right_cast_info) = promotion_result.right_cast {
            new_right_value = self.insert_cast_instruction(
                generator,
                func,
                new_right_value,
                &right_cast_info.to_type,
                right_cast_info.cast_kind,
                span,
            );
        }

        (new_left_value, new_right_value)
    }

    /// Helper function to insert a cast instruction
    fn insert_cast_instruction(
        &self,
        generator: &mut IrGenerator,
        _func: &mut super::function::Function, // Using _ to indicate it's intentionally unused for now
        value: Value,
        to_type: &IrType,
        cast_kind: CastKind,
        span: SourceSpan,
    ) -> Value {
        let temp_id = generator.new_temp();
        let result_value = Value::new_temporary(temp_id, to_type.clone()).with_debug_info(None, span.clone());

        let cast_inst = Instruction::new(
            InstructionKind::Cast { kind: cast_kind, value: value.clone(), from_ty: value.ty, to_ty: to_type.clone() },
            span,
        )
        .with_result(result_value.clone());

        generator.add_instruction(cast_inst);

        result_value
    }
}

// Helper methods for IrType that might not exist
use super::{PrecisionLossEstimate, PromotionRule};
