//! Special type promotion rules for Bool, Char, and String types.
//!
//! This module contains promotion rules for non-numeric types that require
//! special handling, including Unicode validation and string parsing.

use crate::ir::{CastKind, IrType};

use super::matrix::PromotionMatrix;
use super::types::PromotionRule;
use std::cmp::Ordering;

impl PromotionMatrix {
    /// Add all boolean conversion rules (24 rules total)
    ///
    /// Boolean conversions:
    /// - Bool → Integers (8 rules): Bool to I8, I16, I32, I64, U8, U16, U32, U64
    /// - Integers → Bool (8 rules): zero test (0 → false, non-zero → true)
    /// - Bool ↔ Floats (4 rules): Bool ↔ F32, Bool ↔ F64
    /// - Bool ↔ String (2 rules): "true"/"false" conversions
    /// - Bool ↔ Char (2 rules): Indirect via U32
    pub(super) fn add_boolean_promotions(&mut self) {
        let int_types =
            vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];

        // Bool → Integers (8 rules)
        for int_ty in &int_types {
            self.add_promotion_rule(
                IrType::Bool,
                int_ty.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::BoolToInt,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }

        // Integers → Bool (8 rules)
        for int_ty in &int_types {
            self.add_promotion_rule(
                int_ty.clone(),
                IrType::Bool,
                PromotionRule::Direct {
                    cast_kind: CastKind::IntToBool,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }

        // Bool → Floats (2 rules)
        for float_ty in &[IrType::F32, IrType::F64] {
            self.add_promotion_rule(
                IrType::Bool,
                float_ty.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::BoolToFloat,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }

        // Floats → Bool (2 rules)
        for float_ty in &[IrType::F32, IrType::F64] {
            self.add_promotion_rule(
                float_ty.clone(),
                IrType::Bool,
                PromotionRule::Direct {
                    cast_kind: CastKind::FloatToBool,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }

        // Bool ↔ String (2 rules)
        self.add_promotion_rule(
            IrType::Bool,
            IrType::String,
            PromotionRule::Direct {
                cast_kind: CastKind::BoolToString,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: true,
                requires_validation: false,
                precision_loss_estimate: None,
            },
        );

        self.add_promotion_rule(
            IrType::String,
            IrType::Bool,
            PromotionRule::Direct {
                cast_kind: CastKind::StringToBool,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: true,
                requires_validation: true,
                precision_loss_estimate: None,
            },
        );
    }

    /// Add all character conversion rules (21 rules total)
    ///
    /// Character conversions:
    /// - Char ↔ U32: Direct (2 rules)
    /// - Char ↔ I32: Direct with validation (2 rules)
    /// - Char ↔ String: Runtime support (2 rules)
    /// - Char ↔ Other integers: Indirect via U32 (12 rules)
    /// - Char ↔ Floats: Indirect via U32 (4 rules)
    /// - Char ↔ Bool: Indirect via U32 (2 rules)
    #[allow(clippy::too_many_lines)]
    pub(super) fn add_character_promotions(&mut self) {
        // Char ↔ U32: Direct Unicode scalar value conversion (2 rules)
        self.add_promotion_rule(
            IrType::Char,
            IrType::U32,
            PromotionRule::Direct {
                cast_kind: CastKind::CharToInt,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
                precision_loss_estimate: None,
            },
        );

        self.add_promotion_rule(
            IrType::U32,
            IrType::Char,
            PromotionRule::Direct {
                cast_kind: CastKind::IntToChar,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: true, // Unicode scalar validation
                precision_loss_estimate: None,
            },
        );

        // Char ↔ I32: Direct with validation (2 rules)
        self.add_promotion_rule(
            IrType::Char,
            IrType::I32,
            PromotionRule::Direct {
                cast_kind: CastKind::CharToInt,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
                precision_loss_estimate: None,
            },
        );

        self.add_promotion_rule(
            IrType::I32,
            IrType::Char,
            PromotionRule::Direct {
                cast_kind: CastKind::IntToChar,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: true, // Non-negative + Unicode validation
                precision_loss_estimate: None,
            },
        );

        // Char ↔ String: Runtime support (2 rules)
        self.add_promotion_rule(
            IrType::Char,
            IrType::String,
            PromotionRule::Direct {
                cast_kind: CastKind::CharToString,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: true,
                requires_validation: false,
                precision_loss_estimate: None,
            },
        );

        self.add_promotion_rule(
            IrType::String,
            IrType::Char,
            PromotionRule::Direct {
                cast_kind: CastKind::StringToChar,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: true,
                requires_validation: true, // Length = 1 validation
                precision_loss_estimate: None,
            },
        );

        // Char ↔ Other integers: Indirect via U32 (12 rules)
        let other_int_types = vec![IrType::I8, IrType::I16, IrType::I64, IrType::U8, IrType::U16, IrType::U64];

        for int_ty in &other_int_types {
            // Char → Other Int
            self.add_promotion_rule(
                IrType::Char,
                int_ty.clone(),
                PromotionRule::Indirect {
                    intermediate_type: IrType::U32,
                    first_cast: CastKind::CharToInt,
                    second_cast: self.infer_cast_kind_for_conversion(&IrType::U32, int_ty),
                    requires_runtime_support: false,
                },
            );

            // Other Int → Char
            self.add_promotion_rule(
                int_ty.clone(),
                IrType::Char,
                PromotionRule::Indirect {
                    intermediate_type: IrType::U32,
                    first_cast: self.infer_cast_kind_for_conversion(int_ty, &IrType::U32),
                    second_cast: CastKind::IntToChar,
                    requires_runtime_support: false,
                },
            );
        }

        // Char ↔ Float: Indirect via U32 (4 rules)
        for float_ty in &[IrType::F32, IrType::F64] {
            // Char → Float
            self.add_promotion_rule(
                IrType::Char,
                float_ty.clone(),
                PromotionRule::Indirect {
                    intermediate_type: IrType::U32,
                    first_cast: CastKind::CharToInt,
                    second_cast: CastKind::IntToFloat,
                    requires_runtime_support: false,
                },
            );

            // Float → Char
            self.add_promotion_rule(
                float_ty.clone(),
                IrType::Char,
                PromotionRule::Indirect {
                    intermediate_type: IrType::U32,
                    first_cast: CastKind::FloatToInt,
                    second_cast: CastKind::IntToChar,
                    requires_runtime_support: false,
                },
            );
        }

        // Char ↔ Bool: Indirect via U32 (2 rules)
        self.add_promotion_rule(
            IrType::Bool,
            IrType::Char,
            PromotionRule::Indirect {
                intermediate_type: IrType::U32,
                first_cast: CastKind::BoolToInt,
                second_cast: CastKind::IntToChar,
                requires_runtime_support: false,
            },
        );

        self.add_promotion_rule(
            IrType::Char,
            IrType::Bool,
            PromotionRule::Indirect {
                intermediate_type: IrType::U32,
                first_cast: CastKind::CharToInt,
                second_cast: CastKind::IntToBool,
                requires_runtime_support: false,
            },
        );

        // Char → Char: Identity (1 rule)
        self.add_identity_promotion(IrType::Char);
    }

    /// Add all string conversion rules (25 rules)
    ///
    /// String conversions:
    /// 1. Primitive → String (12 rules): Always succeed, require runtime formatting
    /// 2. String → Primitive (12 rules): May fail, require runtime parsing + validation
    /// 3. String → String (1 rule): Identity/no-op
    pub(super) fn add_string_promotions(&mut self) {
        let int_types =
            [IrType::I8, IrType::I16, IrType::I32, IrType::I64, IrType::U8, IrType::U16, IrType::U32, IrType::U64];

        // Integers → String (8 rules)
        for int_ty in &int_types {
            self.add_promotion_rule(
                int_ty.clone(),
                IrType::String,
                PromotionRule::Direct {
                    cast_kind: CastKind::IntToString,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: true,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }

        // String → Integers (8 rules)
        for int_ty in &int_types {
            self.add_promotion_rule(
                IrType::String,
                int_ty.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::StringToInt,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: true,
                    requires_validation: true,
                    precision_loss_estimate: None,
                },
            );
        }

        // Floats → String (2 rules)
        for float_ty in &[IrType::F32, IrType::F64] {
            self.add_promotion_rule(
                float_ty.clone(),
                IrType::String,
                PromotionRule::Direct {
                    cast_kind: CastKind::FloatToString,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: true,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }

        // String → Floats (2 rules)
        for float_ty in &[IrType::F32, IrType::F64] {
            self.add_promotion_rule(
                IrType::String,
                float_ty.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::StringToFloat,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: true,
                    requires_validation: true,
                    precision_loss_estimate: None,
                },
            );
        }

        // String → String: Identity (1 rule)
        self.add_promotion_rule(
            IrType::String,
            IrType::String,
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
                precision_loss_estimate: None,
            },
        );
    }

    /// Helper method to infer `CastKind` for integer-to-integer conversions
    #[allow(clippy::unused_self)]
    pub(super) fn infer_cast_kind_for_conversion(&self, from: &IrType, to: &IrType) -> CastKind {
        use IrType::{I8, I16, I32, I64, U8, U16, U32, U64};

        match (from, to) {
            // Same type → Bitcast
            (I8, I8) | (I16, I16) | (I32, I32) | (I64, I64) | (U8, U8) | (U16, U16) | (U32, U32) | (U64, U64) => {
                CastKind::Bitcast
            }

            // Widening signed
            (I8, I16 | I32 | I64) | (I16, I32 | I64) | (I32, I64) => CastKind::IntSignExtend,

            // Widening unsigned
            (U8, U16 | U32 | U64) | (U16, U32 | U64) | (U32, U64) => CastKind::IntZeroExtend,

            // Narrowing signed
            // Narrowing unsigned
            (I64, I32 | I16 | I8)
            | (I32, I16 | I8)
            | (I16, I8)
            | (U64, U32 | U16 | U8)
            | (U32, U16 | U8)
            | (U16, U8) => CastKind::IntTruncate,

            // Cross-signedness same width
            (I8, U8) | (U8, I8) | (I16, U16) | (U16, I16) | (I32, U32) | (U32, I32) | (I64, U64) | (U64, I64) => {
                CastKind::IntBitcast
            }

            // Cross-signedness different widths
            (I8 | I16 | I32 | I64, U8 | U16 | U32 | U64) | (U8 | U16 | U32 | U64, I8 | I16 | I32 | I64) => {
                let from_bits = Self::get_int_bit_width(from);
                let to_bits = Self::get_int_bit_width(to);
                match from_bits.cmp(&to_bits) {
                    Ordering::Greater => CastKind::IntTruncate,
                    Ordering::Less => CastKind::IntZeroExtend,
                    Ordering::Equal => CastKind::IntBitcast,
                }
            }

            _ => CastKind::Bitcast,
        }
    }

    /// Helper to get integer bit width
    const fn get_int_bit_width(ty: &IrType) -> u32 {
        match ty {
            IrType::I8 | IrType::U8 => 8,
            IrType::I16 | IrType::U16 => 16,
            IrType::I32 | IrType::U32 => 32,
            IrType::I64 | IrType::U64 => 64,
            _ => 0,
        }
    }
}
