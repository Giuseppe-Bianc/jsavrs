//! # Type Promotion System for IR
//!
//! This module implements a comprehensive type promotion system for the jsavrs intermediate representation (IR).
//! The system addresses the issue where binary operations in the IR were incorrectly inheriting the type
//! of the left operand without proper type promotion logic.
//!
//! ## Key Components
//!
//! - `TypePromotion`: Represents a single type promotion operation with source, target, and cast information
//! - `PromotionMatrix`: Defines the complete type promotion lattice and rules
//! - `PromotionRule`: Defines specific promotion behavior between two types (direct, indirect, or forbidden)
//! - `TypeGroup`: Groups types by mathematical properties for promotion ordering
//! - `PromotionResult`: Contains the result of type promotion analysis
//! - `PromotionWarning`: Represents warnings generated during type promotion
//! - `OverflowBehavior`: Configuration for runtime behavior on numeric overflow
//! - `PrecisionLossEstimate`: Quantifies potential precision loss in type conversions
//!
//! ## Type Promotion Rules
//!
//! The system follows a well-defined type lattice hierarchy:
//!
//! 1. Float types take highest precedence (F64 > F32 > Integer types)
//! 2. For same-width signed/unsigned integers, promote to the next larger signed type
//! 3. Wider types take precedence within the same signedness category
//! 4. Special handling for precision loss and overflow scenarios
//!
//! ## Usage Example
//!
//! ```rust
//! use jsavrs::ir::type_promotion::PromotionResult;
//! use jsavrs::ir::TypePromotionEngine;
//! use jsavrs::ir::types::IrType;
//! use jsavrs::ir::instruction::IrBinaryOp;
//! use jsavrs::location::source_span::SourceSpan;
//!
//! let engine = TypePromotionEngine::new();
//! let result: PromotionResult = engine.analyze_binary_promotion(
//!     &IrType::I32,
//!     &IrType::F32,
//!     IrBinaryOp::Add,
//!     SourceSpan::default()
//! );
//! // Result will show that promotion to F32 is required with appropriate cast instructions
//! ```
//!
//! ## Design Goals
//!
//! - **Correctness**: Ensure mathematically sound type promotions
//! - **Consistency**: Same expression always produces same result type
//! - **Precision Preservation**: Maintain maximum possible precision
//! - **Standard Compliance**: Follow IEEE floating-point and integer behavior
//! - **Performance**: Efficient O(1) promotion lookups for common cases

use crate::ir::types::IrType;
use crate::location::source_span::SourceSpan;
use std::collections::HashMap;

/// Central entity managing type promotion logic and rules
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TypePromotion {
    /// The source type being promoted from
    pub from_type: IrType,
    /// The target type being promoted to  
    pub to_type: IrType,
    /// The kind of cast operation required for this promotion
    pub cast_kind: CastKind,
    /// Whether this promotion may result in precision loss
    pub may_lose_precision: bool,
    /// Whether this promotion may result in value overflow/underflow
    pub may_overflow: bool,
    /// Source location for error reporting
    pub source_span: SourceSpan,
}

/// Defines the complete type promotion lattice and rules
#[derive(Debug, Clone)]
pub struct PromotionMatrix {
    /// Matrix of promotion rules indexed by (from_type, to_type)
    promotion_rules: HashMap<(IrType, IrType), PromotionRule>,
    /// Type precedence ordering for automatic promotion
    #[allow(dead_code)]
    type_precedence: Vec<TypeGroup>,
    /// Configuration for runtime behavior on errors
    overflow_behavior: OverflowBehavior,
}

impl Default for PromotionMatrix {
    fn default() -> Self {
        Self::new()
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
        requires_runtime_support: bool, // NEW: For string conversions
        requires_validation: bool,      // NEW: For u32→char, String→primitive
    },
    /// Promotion through intermediate type
    Indirect {
        intermediate_type: IrType,
        first_cast: CastKind,
        second_cast: CastKind,
        requires_runtime_support: bool, // NEW
    },
    /// Promotion not allowed
    Forbidden { reason: String },
}

/// Groups types by mathematical properties for promotion ordering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeGroup {
    SignedIntegers(Vec<IrType>),   // I8, I16, I32, I64
    UnsignedIntegers(Vec<IrType>), // U8, U16, U32, U64
    FloatingPoint(Vec<IrType>),    // F32, F64
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
    /// Float special values in type conversions (updated for type conversion context)
    FloatSpecialValues {
        value_type: FloatSpecialValueType,  // NaN | PosInf | NegInf
        source_type: IrType,                // F32 or F64
        target_type: IrType,                // I8-I64, U8-U64
        applied_behavior: OverflowBehavior, // Wrap | Saturate | Trap | CompileError
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverflowBehavior {
    /// Wrap around using modulo arithmetic
    Wrap,
    /// Saturate to maximum/minimum values
    Saturate,
    /// Generate runtime trap/panic
    Trap,
    /// Compiler error for statically detectable overflow
    CompileError,
}

/// Quantifies potential precision loss in type conversions
#[derive(Debug, Clone, PartialEq)]
pub enum PrecisionLossEstimate {
    /// No precision loss expected
    None,
    /// Fractional part may be lost (float to int)
    FractionalPart,
    /// Significant digits may be lost (f64 to f32)
    SignificantDigits { lost_bits: u32 },
    /// Complete value range change (large int to small int)
    ValueRange { from_bits: u32, to_bits: u32 },
}

impl PromotionMatrix {
    pub fn new() -> Self {
        Self::with_overflow_behavior(OverflowBehavior::Saturate)
    }

    pub fn with_overflow_behavior(overflow_behavior: OverflowBehavior) -> Self {
        let mut matrix = PromotionMatrix {
            promotion_rules: HashMap::new(),
            type_precedence: vec![
                TypeGroup::FloatingPoint(vec![IrType::F32, IrType::F64]),
                TypeGroup::SignedIntegers(vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64]),
                TypeGroup::UnsignedIntegers(vec![IrType::U8, IrType::U16, IrType::U32, IrType::U64]),
                TypeGroup::Boolean,
                TypeGroup::Character,
            ],
            overflow_behavior,
        };

        // Initialize the promotion matrix with default rules
        matrix.initialize_default_promotions();
        matrix
    }

    pub fn get_overflow_behavior(&self) -> OverflowBehavior {
        self.overflow_behavior
    }

    pub fn set_overflow_behavior(&mut self, behavior: OverflowBehavior) {
        self.overflow_behavior = behavior;
    }

    fn initialize_default_promotions(&mut self) {
        // Floating point promotions
        self.add_promotion_rule(
            IrType::F64,
            IrType::F32,
            PromotionRule::Direct {
                cast_kind: CastKind::FloatTruncate,
                may_lose_precision: true,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
        self.add_promotion_rule(
            IrType::F32,
            IrType::F64,
            PromotionRule::Direct {
                cast_kind: CastKind::FloatExtend,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );

        // Add all signed integer widening promotions
        self.add_integer_widening_promotions(
            &[(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)],
            CastKind::IntSignExtend,
        );

        // Add all unsigned integer widening promotions
        self.add_integer_widening_promotions(
            &[(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)],
            CastKind::IntZeroExtend,
        );

        // Add float with integer promotions
        let signed_types = [(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)];
        let unsigned_types = [(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)];
        self.add_float_integer_promotions(&signed_types);
        self.add_float_integer_promotions(&unsigned_types);

        // Add integer narrowing promotions (T016)
        self.add_integer_narrowing_promotions();

        // Add cross-signedness promotion rules for same-width types
        self.add_cross_signedness_promotions();

        // Add identity promotions for all basic types
        self.add_identity_promotions();
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
                    },
                );
            }
        }
    }

    /// Helper function to add float to integer and integer to float promotions
    fn add_float_integer_promotions(&mut self, int_types: &[(IrType, u32)]) {
        for (int_type, _) in int_types {
            // F32 to int type
            self.add_promotion_rule(
                IrType::F32,
                int_type.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::FloatToInt,
                    may_lose_precision: true,
                    may_overflow: true,
                    requires_runtime_support: false,
                    requires_validation: false,
                },
            );
            // Int type to F32
            self.add_promotion_rule(
                int_type.clone(),
                IrType::F32,
                PromotionRule::Direct {
                    cast_kind: CastKind::IntToFloat,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                },
            );
            // F64 to int type
            self.add_promotion_rule(
                IrType::F64,
                int_type.clone(),
                PromotionRule::Direct {
                    cast_kind: CastKind::FloatToInt,
                    may_lose_precision: true,
                    may_overflow: true,
                    requires_runtime_support: false,
                    requires_validation: false,
                },
            );
            // Int type to F64
            self.add_promotion_rule(
                int_type.clone(),
                IrType::F64,
                PromotionRule::Direct {
                    cast_kind: CastKind::IntToFloat,
                    may_lose_precision: false,
                    may_overflow: false,
                    requires_runtime_support: false,
                    requires_validation: false,
                },
            );
        }
    }

    /// Add cross-signedness promotion rules for same-width types
    fn add_cross_signedness_promotions(&mut self) {
        // These should promote to a common type according to C++ promotion rules
        self.add_promotion_rule(
            IrType::I8,
            IrType::U8,
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
        self.add_promotion_rule(
            IrType::I16,
            IrType::U16,
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
        self.add_promotion_rule(
            IrType::I32,
            IrType::U32,
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
        self.add_promotion_rule(
            IrType::I64,
            IrType::U64,
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast,
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
    }

    /// Add identity promotions for all basic types
    fn add_identity_promotions(&mut self) {
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
            self.add_symmetric_promotion_rule(ty.clone(), ty);
        }
    }

    fn add_symmetric_promotion_rule(&mut self, from: IrType, to: IrType) {
        self.promotion_rules.insert(
            (from, to),
            PromotionRule::Direct {
                cast_kind: CastKind::Bitcast, // No cast needed for same type
                may_lose_precision: false,
                may_overflow: false,
                requires_runtime_support: false,
                requires_validation: false,
            },
        );
    }

    fn add_promotion_rule(&mut self, from: IrType, to: IrType, rule: PromotionRule) {
        self.promotion_rules.insert((from.clone(), to.clone()), rule.clone());
        // Add symmetric rule if needed (same types)
        if from == to {
            self.add_symmetric_promotion_rule(from, to);
        } else {
            // Also add the inverse if it's not already defined
            if !self.promotion_rules.contains_key(&(to.clone(), from.clone())) {
                // For now, just add the same rule in reverse, though in a real implementation
                // we might want to define specific reverse rules
                if let PromotionRule::Direct {
                    cast_kind,
                    may_lose_precision,
                    may_overflow,
                    requires_runtime_support,
                    requires_validation,
                } = &rule
                {
                    self.promotion_rules.insert(
                        (to, from),
                        PromotionRule::Direct {
                            cast_kind: *cast_kind,
                            may_lose_precision: *may_lose_precision,
                            may_overflow: *may_overflow,
                            requires_runtime_support: *requires_runtime_support,
                            requires_validation: *requires_validation,
                        },
                    );
                }
            }
        }
    }

    pub fn get_promotion_rule(&self, from: &IrType, to: &IrType) -> Option<&PromotionRule> {
        self.promotion_rules.get(&(from.clone(), to.clone()))
    }

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

            // Signed/unsigned of same width promote to next size up (or larger signed for I64/U64)
            (IrType::I64, IrType::U64) | (IrType::U64, IrType::I64) => Some(IrType::I64), // ADD THIS
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

            // Handle other type combinations as needed
            _ => Some(IrType::I32), // fallback
        }
    }

    fn get_higher_type(&self, left: &IrType, right: &IrType) -> IrType {
        // Use the type lattice to determine higher precedence
        // Delegating to a shared helper function to avoid duplication
        Self::determine_type_precedence(left, right)
    }

    /// Helper function to determine type precedence based on the type lattice
    fn determine_type_precedence(left: &IrType, right: &IrType) -> IrType {
        match (left, right) {
            // Float types take highest precedence
            (IrType::F64, _) | (_, IrType::F64) => IrType::F64,
            (IrType::F32, _) | (_, IrType::F32) => IrType::F32,

            // For same width signed/unsigned, promote to next size (as per spec)
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

            _ => left.clone(), // fallback to left type
        }
    }

    /// Add all integer narrowing conversion rules (24 rules)
    /// Narrowing: Larger → Smaller within same signedness
    fn add_integer_narrowing_promotions(&mut self) {
        // Signed narrowing (6 rules: I64→I32, I64→I16, I64→I8, I32→I16, I32→I8, I16→I8)
        let signed_types = [(IrType::I8, 8), (IrType::I16, 16), (IrType::I32, 32), (IrType::I64, 64)];
        for i in 0..signed_types.len() {
            for j in 0..i {
                let (from_type, _) = &signed_types[i];
                let (to_type, _) = &signed_types[j];
                self.add_promotion_rule(
                    from_type.clone(),
                    to_type.clone(),
                    PromotionRule::Direct {
                        cast_kind: CastKind::IntTruncate,
                        may_lose_precision: true,
                        may_overflow: true,
                        requires_runtime_support: false,
                        requires_validation: false,
                    },
                );
            }
        }

        // Unsigned narrowing (6 rules: U64→U32, U64→U16, U64→U8, U32→U16, U32→U8, U16→U8)
        let unsigned_types = [(IrType::U8, 8), (IrType::U16, 16), (IrType::U32, 32), (IrType::U64, 64)];
        for i in 0..unsigned_types.len() {
            for j in 0..i {
                let (from_type, _) = &unsigned_types[i];
                let (to_type, _) = &unsigned_types[j];
                self.add_promotion_rule(
                    from_type.clone(),
                    to_type.clone(),
                    PromotionRule::Direct {
                        cast_kind: CastKind::IntTruncate,
                        may_lose_precision: true,
                        may_overflow: true,
                        requires_runtime_support: false,
                        requires_validation: false,
                    },
                );
            }
        }
    }

    /// Add all boolean conversion rules (24 rules)
    fn add_boolean_promotions(&mut self) {
        // Implementation in T023
        todo!("T023: Implement boolean promotions")
    }

    /// Add all character conversion rules (14 rules)
    fn add_character_promotions(&mut self) {
        // Implementation in T031
        todo!("T027: Implement character promotions")
    }

    /// Add all string conversion rules (25 rules)
    fn add_string_promotions(&mut self) {
        // Implementation in T035
        todo!("T035: Implement string promotions")
    }
}

impl TypePromotion {
    pub fn new(from_type: IrType, to_type: IrType, cast_kind: CastKind, source_span: SourceSpan) -> Self {
        TypePromotion {
            from_type,
            to_type,
            cast_kind,
            may_lose_precision: false, // Will be set based on promotion rule
            may_overflow: false,       // Will be set based on promotion rule
            source_span,
        }
    }
}

/// Handles type promotion for binary operations specifically
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperationPromotion {
    /// Left operand value and type
    pub left_operand: Value,
    /// Right operand value and type
    pub right_operand: Value,
    /// The binary operation being performed
    pub operation: IrBinaryOp,
    /// Result of promotion analysis
    pub promotion_result: PromotionResult,
    /// Source location for error reporting
    pub source_span: SourceSpan,
}

// Define the required enums that might not exist yet
use crate::ir::instruction::CastKind;
use crate::ir::instruction::IrBinaryOp;
use crate::ir::value::Value;
