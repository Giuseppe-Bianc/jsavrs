//! Promotion matrix - the core lookup structure for type promotion rules.
//!
//! This module contains the `PromotionMatrix` struct which manages all promotion rules
//! and provides O(1) lookup for type conversion queries.

use std::collections::HashMap;

use crate::ir::{CastKind, IrType};

use super::types::{OverflowBehavior, PromotionRule, PromotionWarning};

/// Defines the complete type promotion lattice and rules
#[derive(Debug, Clone)]
pub struct PromotionMatrix {
    /// Hashmap for O(1) promotion rule lookups: (from_type, to_type) → rule
    pub(crate) promotion_rules: HashMap<(IrType, IrType), PromotionRule>,
    /// Type precedence ordering for promotion analysis
    //pub(crate) type_precedence: Vec<TypeGroup>,
    /// Overflow behavior configuration
    pub(crate) overflow_behavior: OverflowBehavior,
}

impl Default for PromotionMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl PromotionMatrix {
    /// Creates a new promotion matrix with default Saturate overflow behavior
    pub fn new() -> Self {
        Self::with_overflow_behavior(OverflowBehavior::Saturate)
    }

    /// Creates a new promotion matrix with specified overflow behavior
    pub fn with_overflow_behavior(overflow_behavior: OverflowBehavior) -> Self {
        let mut matrix = PromotionMatrix {
            promotion_rules: HashMap::new(),
            /*type_precedence: vec![
                TypeGroup::FloatingPoint(vec![IrType::F32, IrType::F64]),
                TypeGroup::SignedIntegers(vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64]),
                TypeGroup::UnsignedIntegers(vec![IrType::U8, IrType::U16, IrType::U32, IrType::U64]),
                TypeGroup::Boolean,
                TypeGroup::Character,
            ],*/
            overflow_behavior,
        };

        // Initialize all promotion rules
        matrix.initialize_all_rules();
        matrix
    }

    /// Gets the current overflow behavior
    pub fn get_overflow_behavior(&self) -> OverflowBehavior {
        self.overflow_behavior
    }

    /// Sets the overflow behavior
    pub fn set_overflow_behavior(&mut self, behavior: OverflowBehavior) {
        self.overflow_behavior = behavior;
    }

    /// Looks up a promotion rule for converting from one type to another
    pub fn get_promotion_rule(&self, from: &IrType, to: &IrType) -> Option<&PromotionRule> {
        self.promotion_rules.get(&(from.clone(), to.clone()))
    }

    /// Computes the common type for two types in a binary operation
    pub fn compute_common_type(&self, left: &IrType, right: &IrType) -> Option<IrType> {
        if left == right {
            return Some(left.clone());
        }

        // Check if there's a direct promotion rule
        if self.get_promotion_rule(left, right).is_some() {
            // Use the higher precedence type based on the type lattice
            return Some(self.get_higher_type(left, right));
        }

        // Default fallback: look for common promotion type
        match (left, right) {
            // Float takes precedence over integers
            (IrType::F64, _) | (_, IrType::F64) => Some(IrType::F64),
            (IrType::F32, _) | (_, IrType::F32) => Some(IrType::F32),

            // Signed/unsigned of same width promote to next size up
            (IrType::I64, IrType::U64) | (IrType::U64, IrType::I64) => Some(IrType::I64),
            (IrType::I32, IrType::U32) | (IrType::U32, IrType::I32) => Some(IrType::I64),
            (IrType::I16, IrType::U16) | (IrType::U16, IrType::I16) => Some(IrType::I32),
            (IrType::I8, IrType::U8) | (IrType::U8, IrType::I8) => Some(IrType::I16),

            // Wider types take precedence within same signedness
            (IrType::I64, _) | (_, IrType::I64) => Some(IrType::I64),
            (IrType::U64, _) | (_, IrType::U64) => Some(IrType::U64),
            (IrType::I32, _) | (_, IrType::I32) => Some(IrType::I32),
            (IrType::U32, _) | (_, IrType::U32) => Some(IrType::U32),
            (IrType::I16, _) | (_, IrType::I16) => Some(IrType::I16),
            (IrType::U16, _) | (_, IrType::U16) => Some(IrType::U16),
            (IrType::I8, _) | (_, IrType::I8) => Some(IrType::I8),
            (IrType::U8, _) | (_, IrType::U8) => Some(IrType::U8),

            // Default fallback
            _ => Some(IrType::I32),
        }
    }

    fn get_higher_type(&self, left: &IrType, right: &IrType) -> IrType {
        Self::determine_type_precedence(left, right)
    }

    /// Determines type precedence based on the type lattice
    fn determine_type_precedence(left: &IrType, right: &IrType) -> IrType {
        match (left, right) {
            // Float types take highest precedence
            (IrType::F64, _) | (_, IrType::F64) => IrType::F64,
            (IrType::F32, _) | (_, IrType::F32) => IrType::F32,

            // Same width signed/unsigned promote to next size
            (IrType::I64, IrType::U64) | (IrType::U64, IrType::I64) => IrType::I64,
            (IrType::I32, IrType::U32) | (IrType::U32, IrType::I32) => IrType::I64,
            (IrType::I16, IrType::U16) | (IrType::U16, IrType::I16) => IrType::I32,
            (IrType::I8, IrType::U8) | (IrType::U8, IrType::I8) => IrType::I16,

            // Within same type group, prefer wider type
            (IrType::I64, _) | (_, IrType::I64) => IrType::I64,
            (IrType::U64, _) | (_, IrType::U64) => IrType::U64,
            (IrType::I32, _) | (_, IrType::I32) => IrType::I32,
            (IrType::U32, _) | (_, IrType::U32) => IrType::U32,
            (IrType::I16, _) | (_, IrType::I16) => IrType::I16,
            (IrType::U16, _) | (_, IrType::U16) => IrType::U16,

            // Bool and Char promote to I32 when mixed
            (IrType::Bool, IrType::Char) | (IrType::Char, IrType::Bool) => IrType::I32,
            (IrType::Bool, _) | (_, IrType::Bool) => IrType::I32,
            (IrType::Char, _) | (_, IrType::Char) => IrType::I32,

            _ => left.clone(),
        }
    }

    /// Adds a promotion rule to the matrix
    pub(crate) fn add_promotion_rule(&mut self, from: IrType, to: IrType, rule: PromotionRule) {
        self.promotion_rules.insert((from, to), rule);
    }

    /// Adds an identity promotion rule (type to itself)
    pub(crate) fn add_identity_promotion(&mut self, ty: IrType) {
        use super::types::PrecisionLossEstimate;

        self.add_promotion_rule(
            ty.clone(),
            ty,
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
                precision_loss_estimate: Some(PrecisionLossEstimate::None),
            },
        );
    }

    /// Generate precision loss warning for a type conversion (T019)
    pub fn generate_precision_loss_warning(
        &self, from_type: &IrType, to_type: &IrType, rule: &PromotionRule,
    ) -> Option<PromotionWarning> {
        super::warnings::generate_precision_loss_warning(from_type, to_type, rule)
    }

    /// Generate signedness change warning for a type conversion (T020)
    pub fn generate_signedness_change_warning(
        &self, from_type: &IrType, to_type: &IrType, rule: &PromotionRule,
    ) -> Option<PromotionWarning> {
        super::warnings::generate_signedness_change_warning(from_type, to_type, rule)
    }

    /// Generate Unicode validation warning for integer→char conversions (T030)
    pub fn generate_unicode_validation_warning(&self, value: u32, to_type: &IrType) -> Option<PromotionWarning> {
        super::warnings::generate_unicode_validation_warning(value, to_type)
    }
}
