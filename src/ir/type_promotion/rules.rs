//! Rule initialization methods for the promotion matrix.
//!
//! This module contains all the rule builder methods that populate the promotion matrix
//! with conversion rules for different type categories.

use crate::ir::{CastKind, IrType};

use super::matrix::PromotionMatrix;
use super::types::{PrecisionLossEstimate, PromotionRule};

impl PromotionMatrix {
    /// Initialize all default promotion rules
    pub(super) fn initialize_all_rules(&mut self) {
        // Floating point promotions
        self.add_float_promotions();

        // Integer widening and narrowing
        self.add_integer_widening_promotions_signed();
        self.add_integer_widening_promotions_unsigned();
        self.add_integer_narrowing_promotions();

        // Float-Integer conversions
        self.add_float_integer_promotions();

        // Cross-signedness conversions
        self.add_cross_signedness_promotions();
        self.add_cross_signedness_different_width_promotions();

        // Boolean promotions
        self.add_boolean_promotions();

        // Character promotions
        self.add_character_promotions();

        // String promotions
        self.add_string_promotions();

        // Identity promotions
        self.add_identity_promotions();
    }

    /// Add floating-point conversion rules (F32 ↔ F64)
    fn add_float_promotions(&mut self) {
        // F64 → F32: Narrowing with precision loss
        self.add_promotion_rule(
            IrType::F64,
            IrType::F32,
            PromotionRule::Direct {
                cast_kind: CastKind::FloatTruncate,
                may_lose_precision: true,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
                precision_loss_estimate: Some(PrecisionLossEstimate::SignificantDigits { lost_bits: 29 }),
            },
        );

        // F32 → F64: Widening, no precision loss
        self.add_promotion_rule(
            IrType::F32,
            IrType::F64,
            PromotionRule::Direct {
                cast_kind: CastKind::FloatExtend,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
                precision_loss_estimate: None,
            },
        );
    }

    /// Add signed integer widening promotions
    fn add_integer_widening_promotions_signed(&mut self) {
        let types = [(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)];
        self.add_integer_widening_promotions(&types, CastKind::IntSignExtend);
    }

    /// Add unsigned integer widening promotions
    fn add_integer_widening_promotions_unsigned(&mut self) {
        let types = [(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)];
        self.add_integer_widening_promotions(&types, CastKind::IntZeroExtend);
    }

    /// Helper function to add widening promotions for integer types
    fn add_integer_widening_promotions(&mut self, types: &[(IrType, u32)], cast_kind: CastKind) {
        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                let (from_type, _) = &types[i];
                let (to_type, _) = &types[j];
                self.add_promotion_rule(
                    from_type.clone(),
                    to_type.clone(),
                    PromotionRule::Direct {
                        cast_kind,
                        may_lose_precision: false,
                        may_overflow: false,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: None,
                    },
                );
            }
        }
    }

    /// Add all integer narrowing conversion rules (24 rules)
    fn add_integer_narrowing_promotions(&mut self) {
        // Signed narrowing (6 rules)
        let signed_types = [(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)];
        for i in 0..signed_types.len() {
            for j in 0..i {
                let (from_type, from_bits) = &signed_types[i];
                let (to_type, to_bits) = &signed_types[j];
                self.add_promotion_rule(
                    from_type.clone(),
                    to_type.clone(),
                    PromotionRule::Direct {
                        cast_kind: CastKind::IntTruncate,
                        may_lose_precision: true,
                        may_overflow: true,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: Some(PrecisionLossEstimate::ValueRange {
                            from_bits: *from_bits,
                            to_bits: *to_bits,
                        }),
                    },
                );
            }
        }

        // Unsigned narrowing (6 rules)
        let unsigned_types = [(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)];
        for i in 0..unsigned_types.len() {
            for j in 0..i {
                let (from_type, from_bits) = &unsigned_types[i];
                let (to_type, to_bits) = &unsigned_types[j];
                self.add_promotion_rule(
                    from_type.clone(),
                    to_type.clone(),
                    PromotionRule::Direct {
                        cast_kind: CastKind::IntTruncate,
                        may_lose_precision: true,
                        may_overflow: true,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: Some(PrecisionLossEstimate::ValueRange {
                            from_bits: *from_bits,
                            to_bits: *to_bits,
                        }),
                    },
                );
            }
        }
    }

    /// Add float-integer conversion rules
    fn add_float_integer_promotions(&mut self) {
        let signed_types = [(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)];
        let unsigned_types = [(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)];

        // Process both signed and unsigned types
        for int_types in &[&signed_types[..], &unsigned_types[..]] {
            for (int_type, _) in *int_types {
                // F32 ↔ Integer
                self.add_promotion_rule(
                    IrType::F32,
                    int_type.clone(),
                    PromotionRule::Direct {
                        cast_kind: CastKind::FloatToInt,
                        may_lose_precision: true,
                        may_overflow: true,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: Some(PrecisionLossEstimate::FractionalPart),
                    },
                );
                self.add_promotion_rule(
                    int_type.clone(),
                    IrType::F32,
                    PromotionRule::Direct {
                        cast_kind: CastKind::IntToFloat,
                        may_lose_precision: false,
                        may_overflow: false,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: None,
                    },
                );

                // F64 ↔ Integer
                self.add_promotion_rule(
                    IrType::F64,
                    int_type.clone(),
                    PromotionRule::Direct {
                        cast_kind: CastKind::FloatToInt,
                        may_lose_precision: true,
                        may_overflow: true,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: Some(PrecisionLossEstimate::FractionalPart),
                    },
                );
                self.add_promotion_rule(
                    int_type.clone(),
                    IrType::F64,
                    PromotionRule::Direct {
                        cast_kind: CastKind::IntToFloat,
                        may_lose_precision: false,
                        may_overflow: false,
                        requires_runtime_support: false,
                        requires_validation: false,
                        precision_loss_estimate: None,
                    },
                );
            }
        }
    }

    /// Add cross-signedness promotion rules for same-width types
    fn add_cross_signedness_promotions(&mut self) {
        let pairs = [
            (IrType::I8, IrType::U8),
            (IrType::I16, IrType::U16),
            (IrType::I32, IrType::U32),
            (IrType::I64, IrType::U64),
        ];

        for (signed, unsigned) in &pairs {
            // Signed → Unsigned
            self.add_promotion_rule(
                signed.clone(),
                unsigned.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::IntBitcast,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );

            // Unsigned → Signed (symmetric)
            self.add_promotion_rule(
                unsigned.clone(),
                signed.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::IntBitcast,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                    precision_loss_estimate: None,
                },
            );
        }
    }

    /// Add cross-signedness conversions with different widths (24 rules via Indirect)
    fn add_cross_signedness_different_width_promotions(&mut self) {
        let signed_types = vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64];
        let unsigned_types = vec![IrType::U8, IrType::U16, IrType::U32, IrType::U64];

        // Signed → Unsigned (different widths)
        for (i, from_signed) in signed_types.iter().enumerate() {
            for (j, to_unsigned) in unsigned_types.iter().enumerate() {
                if i == j {
                    continue; // Skip same-width
                }

                let intermediate = signed_types[j].clone();
                let (first_cast, second_cast) = if i < j {
                    // Widening: I8 → U16 via I16
                    (CastKind::IntSignExtend, CastKind::Bitcast)
                } else {
                    // Narrowing: I32 → U8 via I8
                    (CastKind::IntTruncate, CastKind::Bitcast)
                };

                self.add_promotion_rule(
                    from_signed.clone(),
                    to_unsigned.clone(),
                    PromotionRule::Indirect {
                        intermediate_type: intermediate,
                        first_cast,
                        second_cast,
                        requires_runtime_support: false,
                    },
                );
            }
        }

        // Unsigned → Signed (different widths)
        for (i, from_unsigned) in unsigned_types.iter().enumerate() {
            for (j, to_signed) in signed_types.iter().enumerate() {
                if i == j {
                    continue; // Skip same-width
                }

                let intermediate = unsigned_types[j].clone();
                let (first_cast, second_cast) = if i < j {
                    // Widening: U8 → I16 via U16
                    (CastKind::IntZeroExtend, CastKind::Bitcast)
                } else {
                    // Narrowing: U32 → I8 via U8
                    (CastKind::IntTruncate, CastKind::Bitcast)
                };

                self.add_promotion_rule(
                    from_unsigned.clone(),
                    to_signed.clone(),
                    PromotionRule::Indirect {
                        intermediate_type: intermediate,
                        first_cast,
                        second_cast,
                        requires_runtime_support: false,
                    },
                );
            }
        }
    }

    /// Add identity promotions for all basic types
    pub(super) fn add_identity_promotions(&mut self) {
        let all_types = vec![
            IrType::I8,
            IrType::I16,
            IrType::I32,
            IrType::I64,
            IrType::U8,
            IrType::U16,
            IrType::U32,
            IrType::U64,
            IrType::F32,
            IrType::F64,
            IrType::Bool,
            IrType::Char,
        ];

        for ty in all_types {
            self.add_identity_promotion(ty);
        }
    }
}
