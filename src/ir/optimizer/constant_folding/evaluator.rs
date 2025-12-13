//! Constant expression evaluation engine
//!
//! Evaluates binary and unary operations on constant operands at compile time,
//! handling type-specific semantics and edge cases.
//!
//! # Type Support
//!
//! - Signed integers: I8, I16, I32, I64
//! - Unsigned integers: U8, U16, U32, U64
//! - Floating-point: F32, F64 (IEEE 754 semantics)
//! - Characters: Char (Unicode support)
//! - Booleans: Bool
//!
//! # Overflow Handling
//!
//! Integer operations use `checked_*` methods. Overflow → `LatticeValue::Top` (per FR-004).
//!
//! # IEEE 754 Compliance
//!
//! Floating-point operations preserve NaN propagation, signed zero, and infinity semantics.

use super::lattice::{ConstantValue, LatticeValue};
use BinaryOp::{And, Eq, Ge, Gt, Le, Lt, Ne, Or};
use UnaryOp::{Neg, Not};

/// Evaluates constant expressions during SCCP analysis
///
/// Provides type-safe constant folding with proper edge case handling.
/// All evaluation methods are pure functions with no side effects.
pub struct ConstantEvaluator;

impl ConstantEvaluator {
    /// Evaluates a binary operation on two constant values
    /// Returns Top if overflow occurs or operands are incompatible
    #[must_use]
    pub fn eval_binary_i32(op: BinaryOp, lhs: i32, rhs: i32) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            // Arithmetic operations
            Add => lhs
                .checked_add(rhs)
                .map_or(LatticeValue::Top, |result| LatticeValue::Constant(ConstantValue::I32(result))),
            Sub => lhs
                .checked_sub(rhs)
                .map_or(LatticeValue::Top, |result| LatticeValue::Constant(ConstantValue::I32(result))),
            Mul => lhs
                .checked_mul(rhs)
                .map_or(LatticeValue::Top, |result| LatticeValue::Constant(ConstantValue::I32(result))),
            Div => {
                if rhs == 0 {
                    // Division by zero → Top + warning (handled by caller)
                    LatticeValue::Top
                } else if let Some(result) = lhs.checked_div(rhs) {
                    LatticeValue::Constant(ConstantValue::I32(result))
                } else {
                    LatticeValue::Top
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else if let Some(result) = lhs.checked_rem(rhs) {
                    LatticeValue::Constant(ConstantValue::I32(result))
                } else {
                    LatticeValue::Top
                }
            }
            // Comparison and boolean ops not handled here
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    /// Evaluates a unary operation on a constant value
    #[must_use]
    pub fn eval_unary_i32(op: UnaryOp, operand: i32) -> LatticeValue {
        use UnaryOp::{Neg, Not};

        match op {
            Neg => operand
                .checked_neg()
                .map_or(LatticeValue::Top, |result| LatticeValue::Constant(ConstantValue::I32(result))),
            Not => LatticeValue::Top, // Not is for booleans, not i32
        }
    }

    // ========================================================================
    // I8 Arithmetic Operations (T079)
    // ========================================================================

    /// Evaluates binary operations on I8 values
    #[must_use]
    pub fn eval_binary_i8(op: BinaryOp, lhs: i8, rhs: i8) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I8(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I8(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I8(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I8(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I8(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    /// Evaluates unary operations on I8 values
    #[must_use]
    pub fn eval_unary_i8(op: UnaryOp, operand: i8) -> LatticeValue {
        use UnaryOp::{Neg, Not};
        match op {
            Neg => operand.checked_neg().map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I8(r))),
            Not => LatticeValue::Top,
        }
    }

    // ========================================================================
    // I16 Arithmetic Operations (T080)
    // ========================================================================

    /// Evaluates binary operations on I16 values
    #[must_use]
    pub fn eval_binary_i16(op: BinaryOp, lhs: i16, rhs: i16) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I16(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I16(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I16(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I16(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I16(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    /// Evaluates unary operations on I16 values
    #[must_use]
    pub fn eval_unary_i16(op: UnaryOp, operand: i16) -> LatticeValue {
        use UnaryOp::{Neg, Not};
        match op {
            Neg => operand.checked_neg().map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I16(r))),
            Not => LatticeValue::Top,
        }
    }

    // ========================================================================
    // I64 Arithmetic Operations (T081)
    // ========================================================================

    /// Evaluates binary operations on I64 values
    #[must_use]
    pub fn eval_binary_i64(op: BinaryOp, lhs: i64, rhs: i64) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I64(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I64(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I64(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I64(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I64(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    /// Evaluates unary operations on I64 values
    #[must_use]
    pub fn eval_unary_i64(op: UnaryOp, operand: i64) -> LatticeValue {
        use UnaryOp::{Neg, Not};
        match op {
            Neg => operand.checked_neg().map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::I64(r))),
            Not => LatticeValue::Top,
        }
    }

    // ========================================================================
    // U8 Arithmetic Operations (T082)
    // ========================================================================

    /// Evaluates binary operations on U8 values
    #[must_use]
    pub fn eval_binary_u8(op: BinaryOp, lhs: u8, rhs: u8) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U8(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U8(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U8(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U8(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U8(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    // ========================================================================
    // U16 Arithmetic Operations (T083)
    // ========================================================================

    /// Evaluates binary operations on U16 values
    #[must_use]
    pub fn eval_binary_u16(op: BinaryOp, lhs: u16, rhs: u16) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U16(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U16(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U16(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U16(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U16(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    // ========================================================================
    // U32 Arithmetic Operations (T084)
    // ========================================================================

    /// Evaluates binary operations on U32 values
    #[must_use]
    pub fn eval_binary_u32(op: BinaryOp, lhs: u32, rhs: u32) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U32(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U32(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U32(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U32(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U32(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    // ========================================================================
    // U64 Arithmetic Operations (T085)
    // ========================================================================

    /// Evaluates binary operations on U64 values
    #[must_use]
    pub fn eval_binary_u64(op: BinaryOp, lhs: u64, rhs: u64) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => lhs.checked_add(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U64(r))),
            Sub => lhs.checked_sub(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U64(r))),
            Mul => lhs.checked_mul(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U64(r))),
            Div => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_div(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U64(r)))
                }
            }
            Mod => {
                if rhs == 0 {
                    LatticeValue::Top
                } else {
                    lhs.checked_rem(rhs).map_or(LatticeValue::Top, |r| LatticeValue::Constant(ConstantValue::U64(r)))
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    // ========================================================================
    // F32 Arithmetic Operations (T086) - IEEE 754 semantics
    // ========================================================================

    /// Evaluates binary operations on F32 values with IEEE 754 semantics
    #[must_use]
    pub fn eval_binary_f32(op: BinaryOp, lhs: f32, rhs: f32) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => {
                let result = lhs + rhs;
                LatticeValue::Constant(ConstantValue::F32(result))
            }
            Sub => {
                let result = lhs - rhs;
                LatticeValue::Constant(ConstantValue::F32(result))
            }
            Mul => {
                let result = lhs * rhs;
                LatticeValue::Constant(ConstantValue::F32(result))
            }
            Div => {
                let result = lhs / rhs;
                LatticeValue::Constant(ConstantValue::F32(result))
            }
            Mod => {
                let result = lhs % rhs;
                LatticeValue::Constant(ConstantValue::F32(result))
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    /// Evaluates unary operations on F32 values
    #[must_use]
    pub fn eval_unary_f32(op: UnaryOp, operand: f32) -> LatticeValue {
        use UnaryOp::{Neg, Not};
        match op {
            Neg => LatticeValue::Constant(ConstantValue::F32(-operand)),
            Not => LatticeValue::Top,
        }
    }

    // ========================================================================
    // F64 Arithmetic Operations (T087) - IEEE 754 semantics
    // ========================================================================

    /// Evaluates binary operations on F64 values with IEEE 754 semantics
    #[must_use]
    pub fn eval_binary_f64(op: BinaryOp, lhs: f64, rhs: f64) -> LatticeValue {
        use BinaryOp::{Add, And, Div, Eq, Ge, Gt, Le, Lt, Mod, Mul, Ne, Or, Sub};

        match op {
            Add => {
                let result = lhs + rhs;
                LatticeValue::Constant(ConstantValue::F64(result))
            }
            Sub => {
                let result = lhs - rhs;
                LatticeValue::Constant(ConstantValue::F64(result))
            }
            Mul => {
                let result = lhs * rhs;
                LatticeValue::Constant(ConstantValue::F64(result))
            }
            Div => {
                let result = lhs / rhs;
                LatticeValue::Constant(ConstantValue::F64(result))
            }
            Mod => {
                let result = lhs % rhs;
                LatticeValue::Constant(ConstantValue::F64(result))
            }
            Eq | Ne | Lt | Le | Gt | Ge | And | Or => LatticeValue::Top,
        }
    }

    /// Evaluates unary operations on F64 values
    #[must_use]
    pub fn eval_unary_f64(op: UnaryOp, operand: f64) -> LatticeValue {
        use UnaryOp::{Neg, Not};
        match op {
            Neg => LatticeValue::Constant(ConstantValue::F64(-operand)),
            Not => LatticeValue::Top,
        }
    }

    // ========================================================================
    // Special Float Value Checks (T088, T089, T090)
    // ========================================================================

    /// Checks if a floating-point value is NaN (T088)
    #[must_use]
    pub const fn is_nan_f32(value: f32) -> bool {
        value.is_nan()
    }

    /// Checks if a floating-point value is NaN (T088)
    #[must_use]
    pub const fn is_nan_f64(value: f64) -> bool {
        value.is_nan()
    }

    /// Checks if a floating-point value is infinite (T089)
    #[must_use]
    pub const fn is_infinite_f32(value: f32) -> bool {
        value.is_infinite()
    }

    /// Checks if a floating-point value is infinite (T089)
    #[must_use]
    pub const fn is_infinite_f64(value: f64) -> bool {
        value.is_infinite()
    }

    /// Checks if a floating-point value is negative zero (T090)
    #[must_use]
    pub fn is_neg_zero_f32(value: f32) -> bool {
        value == 0.0 && value.is_sign_negative()
    }

    /// Checks if a floating-point value is negative zero (T090)
    #[must_use]
    pub fn is_neg_zero_f64(value: f64) -> bool {
        value == 0.0 && value.is_sign_negative()
    }

    // ========================================================================
    // Char Operations (T091)
    // ========================================================================

    /// Evaluates operations on Char values
    /// Most operations on chars are not meaningful, return Top
    #[must_use]
    pub const fn eval_char_eq(lhs: char, rhs: char) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::Bool(lhs == rhs))
    }

    #[must_use]
    pub const fn eval_char_ne(lhs: char, rhs: char) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::Bool(lhs != rhs))
    }

    // ========================================================================
    // Bitwise Operations for Integer Types (T092)
    // ========================================================================

    /// Evaluates bitwise operations on I8 values
    /// Evaluates bitwise operations on I8 values
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub const fn eval_bitwise_i8(op: BitwiseOp, lhs: i8, rhs: i8) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::I8(result))
    }

    /// Evaluates bitwise operations on I16 values
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub const fn eval_bitwise_i16(op: BitwiseOp, lhs: i16, rhs: i16) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::I16(result))
    }

    /// Evaluates bitwise operations on I32 values
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub const fn eval_bitwise_i32(op: BitwiseOp, lhs: i32, rhs: i32) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::I32(result))
    }

    /// Evaluates bitwise operations on I64 values
    #[must_use]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub const fn eval_bitwise_i64(op: BitwiseOp, lhs: i64, rhs: i64) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::I64(result))
    }

    /// Evaluates bitwise operations on U8 values
    #[must_use]
    pub const fn eval_bitwise_u8(op: BitwiseOp, lhs: u8, rhs: u8) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::U8(result))
    }

    /// Evaluates bitwise operations on U16 values
    #[must_use]
    pub const fn eval_bitwise_u16(op: BitwiseOp, lhs: u16, rhs: u16) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::U16(result))
    }

    /// Evaluates bitwise operations on U32 values
    #[must_use]
    pub const fn eval_bitwise_u32(op: BitwiseOp, lhs: u32, rhs: u32) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs),
            Shr => lhs.wrapping_shr(rhs),
        };
        LatticeValue::Constant(ConstantValue::U32(result))
    }

    /// Evaluates bitwise operations on U64 values
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn eval_bitwise_u64(op: BitwiseOp, lhs: u64, rhs: u64) -> LatticeValue {
        use BitwiseOp::{And, Or, Shl, Shr, Xor};
        let result = match op {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),
        };
        LatticeValue::Constant(ConstantValue::U64(result))
    }

    /// Evaluates bitwise NOT on integer values
    #[must_use]
    pub const fn eval_bitwise_not_i8(operand: i8) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::I8(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_i16(operand: i16) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::I16(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_i32(operand: i32) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::I32(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_i64(operand: i64) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::I64(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_u8(operand: u8) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::U8(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_u16(operand: u16) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::U16(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_u32(operand: u32) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::U32(!operand))
    }

    #[must_use]
    pub const fn eval_bitwise_not_u64(operand: u64) -> LatticeValue {
        LatticeValue::Constant(ConstantValue::U64(!operand))
    }

    /// Checks if a division by zero occurred
    #[must_use]
    pub const fn is_division_by_zero(op: BinaryOp, rhs: &ConstantValue) -> bool {
        matches!(op, BinaryOp::Div | BinaryOp::Mod)
            && matches!(
                rhs,
                ConstantValue::I8(0)
                    | ConstantValue::I16(0)
                    | ConstantValue::I32(0)
                    | ConstantValue::I64(0)
                    | ConstantValue::U8(0)
                    | ConstantValue::U16(0)
                    | ConstantValue::U32(0)
                    | ConstantValue::U64(0)
            )
    }

    /// Evaluates comparison operations on I32 values
    /// Returns Bool constant for valid comparisons
    #[must_use]
    pub const fn eval_compare_i32(op: BinaryOp, lhs: i32, rhs: i32) -> LatticeValue {
        let result = match op {
            Eq => lhs == rhs,
            Ne => lhs != rhs,
            Lt => lhs < rhs,
            Le => lhs <= rhs,
            Gt => lhs > rhs,
            Ge => lhs >= rhs,
            _ => return LatticeValue::Top, // Not a comparison op
        };

        LatticeValue::Constant(ConstantValue::Bool(result))
    }

    /// Evaluates boolean operations (And, Or, Not)
    #[must_use]
    pub const fn eval_binary_bool(op: BinaryOp, lhs: bool, rhs: bool) -> LatticeValue {
        let result = match op {
            And => lhs && rhs,
            Or => lhs || rhs,
            _ => return LatticeValue::Top, // Not a boolean op
        };

        LatticeValue::Constant(ConstantValue::Bool(result))
    }

    /// Evaluates unary Not operation on boolean
    #[must_use]
    pub const fn eval_unary_bool(op: UnaryOp, operand: bool) -> LatticeValue {
        match op {
            Not => LatticeValue::Constant(ConstantValue::Bool(!operand)),
            Neg => LatticeValue::Top, // Not a boolean unary op
        }
    }
}

/// Binary operations
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Logical
    And,
    Or,
}

/// Unary operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// Bitwise operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitwiseOp {
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_arithmetic() {
        assert_eq!(
            ConstantEvaluator::eval_binary_i32(BinaryOp::Add, 5, 10),
            LatticeValue::Constant(ConstantValue::I32(15))
        );

        assert_eq!(
            ConstantEvaluator::eval_binary_i32(BinaryOp::Sub, 10, 5),
            LatticeValue::Constant(ConstantValue::I32(5))
        );

        assert_eq!(
            ConstantEvaluator::eval_binary_i32(BinaryOp::Mul, 3, 7),
            LatticeValue::Constant(ConstantValue::I32(21))
        );

        assert_eq!(
            ConstantEvaluator::eval_binary_i32(BinaryOp::Div, 20, 4),
            LatticeValue::Constant(ConstantValue::I32(5))
        );

        assert_eq!(
            ConstantEvaluator::eval_binary_i32(BinaryOp::Mod, 17, 5),
            LatticeValue::Constant(ConstantValue::I32(2))
        );
    }

    #[test]
    fn test_i32_overflow() {
        // Overflow → Top
        assert_eq!(ConstantEvaluator::eval_binary_i32(BinaryOp::Add, i32::MAX, 1), LatticeValue::Top);

        assert_eq!(ConstantEvaluator::eval_binary_i32(BinaryOp::Sub, i32::MIN, 1), LatticeValue::Top);

        assert_eq!(ConstantEvaluator::eval_binary_i32(BinaryOp::Mul, i32::MAX, 2), LatticeValue::Top);
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(ConstantEvaluator::eval_binary_i32(BinaryOp::Div, 10, 0), LatticeValue::Top);

        assert_eq!(ConstantEvaluator::eval_binary_i32(BinaryOp::Mod, 10, 0), LatticeValue::Top);

        assert!(ConstantEvaluator::is_division_by_zero(BinaryOp::Div, &ConstantValue::I32(0)));
    }

    #[test]
    fn test_unary_neg() {
        assert_eq!(
            ConstantEvaluator::eval_unary_i32(UnaryOp::Neg, 42),
            LatticeValue::Constant(ConstantValue::I32(-42))
        );

        // -i32::MIN overflows → Top
        assert_eq!(ConstantEvaluator::eval_unary_i32(UnaryOp::Neg, i32::MIN), LatticeValue::Top);
    }

    #[test]
    fn test_i32_comparisons() {
        // Equality
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 5, 5),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 5, 10),
            LatticeValue::Constant(ConstantValue::Bool(false))
        );

        // Not equal
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Ne, 5, 10),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );

        // Less than
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 5, 10),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 10, 5),
            LatticeValue::Constant(ConstantValue::Bool(false))
        );

        // Less than or equal
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Le, 5, 5),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );

        // Greater than
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 10, 5),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );

        // Greater than or equal
        assert_eq!(
            ConstantEvaluator::eval_compare_i32(BinaryOp::Ge, 10, 10),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );
    }

    #[test]
    fn test_boolean_operations() {
        // And
        assert_eq!(
            ConstantEvaluator::eval_binary_bool(BinaryOp::And, true, true),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );
        assert_eq!(
            ConstantEvaluator::eval_binary_bool(BinaryOp::And, true, false),
            LatticeValue::Constant(ConstantValue::Bool(false))
        );
        assert_eq!(
            ConstantEvaluator::eval_binary_bool(BinaryOp::And, false, false),
            LatticeValue::Constant(ConstantValue::Bool(false))
        );

        // Or
        assert_eq!(
            ConstantEvaluator::eval_binary_bool(BinaryOp::Or, true, false),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );
        assert_eq!(
            ConstantEvaluator::eval_binary_bool(BinaryOp::Or, false, false),
            LatticeValue::Constant(ConstantValue::Bool(false))
        );
    }

    #[test]
    fn test_unary_not() {
        assert_eq!(
            ConstantEvaluator::eval_unary_bool(UnaryOp::Not, true),
            LatticeValue::Constant(ConstantValue::Bool(false))
        );
        assert_eq!(
            ConstantEvaluator::eval_unary_bool(UnaryOp::Not, false),
            LatticeValue::Constant(ConstantValue::Bool(true))
        );
    }
}
