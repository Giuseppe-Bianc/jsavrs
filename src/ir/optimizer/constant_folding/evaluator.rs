//! Instruction evaluation for SCCP constant propagation

#![allow(non_snake_case)]

use super::lattice::LatticeValue;
use crate::ir::{IrBinaryOp, IrLiteralValue, IrUnaryOp};

/// Evaluates IR instructions to determine their lattice values
pub struct InstructionEvaluator;

impl InstructionEvaluator {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates a binary operation instruction
    pub fn evaluate_binary_op(op: &IrBinaryOp, left: &LatticeValue, right: &LatticeValue) -> LatticeValue {
        if matches!(left, LatticeValue::Bottom) || matches!(right, LatticeValue::Bottom) {
            return LatticeValue::Bottom;
        }

        if matches!(left, LatticeValue::Top) || matches!(right, LatticeValue::Top) {
            return LatticeValue::Top;
        }

        let (LatticeValue::Constant(left_val), LatticeValue::Constant(right_val)) = (left, right) else {
            return LatticeValue::Bottom;
        };

        match (left_val, right_val) {
            (IrLiteralValue::I32(a), IrLiteralValue::I32(b)) => Self::eval_i32_binop(op, *a, *b),
            (IrLiteralValue::I64(a), IrLiteralValue::I64(b)) => Self::eval_i64_binop(op, *a, *b),
            (IrLiteralValue::U32(a), IrLiteralValue::U32(b)) => Self::eval_u32_binop(op, *a, *b),
            (IrLiteralValue::U64(a), IrLiteralValue::U64(b)) => Self::eval_u64_binop(op, *a, *b),
            (IrLiteralValue::Bool(a), IrLiteralValue::Bool(b)) => Self::eval_bool_binop(op, *a, *b),
            (IrLiteralValue::F32(a), IrLiteralValue::F32(b)) => Self::eval_f32_binop(op, *a, *b),
            (IrLiteralValue::F64(a), IrLiteralValue::F64(b)) => Self::eval_f64_binop(op, *a, *b),
            _ => LatticeValue::Bottom,
        }
    }

    /// Evaluates a unary operation instruction
    pub fn evaluate_unary_op(op: &IrUnaryOp, operand: &LatticeValue) -> LatticeValue {
        use IrUnaryOp::*;

        match operand {
            LatticeValue::Bottom => LatticeValue::Bottom,
            LatticeValue::Top => LatticeValue::Top,
            LatticeValue::Constant(val) => match (op, val) {
                (Neg, IrLiteralValue::I32(v)) => v
                    .checked_neg()
                    .map(|r| LatticeValue::Constant(IrLiteralValue::I32(r)))
                    .unwrap_or(LatticeValue::Bottom),
                (Neg, IrLiteralValue::I64(v)) => v
                    .checked_neg()
                    .map(|r| LatticeValue::Constant(IrLiteralValue::I64(r)))
                    .unwrap_or(LatticeValue::Bottom),
                (Neg, IrLiteralValue::F32(v)) => LatticeValue::Constant(IrLiteralValue::F32(-v)),
                (Neg, IrLiteralValue::F64(v)) => LatticeValue::Constant(IrLiteralValue::F64(-v)),
                (Not, IrLiteralValue::Bool(v)) => LatticeValue::Constant(IrLiteralValue::Bool(!v)),
                (Not, IrLiteralValue::I32(v)) => LatticeValue::Constant(IrLiteralValue::I32(!v)),
                (Not, IrLiteralValue::I64(v)) => LatticeValue::Constant(IrLiteralValue::I64(!v)),
                _ => LatticeValue::Bottom,
            },
        }
    }

    fn eval_i32_binop(op: &IrBinaryOp, a: i32, b: i32) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            Add => a.checked_add(b),
            Subtract => a.checked_sub(b),
            Multiply => a.checked_mul(b),
            Divide => {
                if b == 0 {
                    None
                } else {
                    a.checked_div(b)
                }
            }
            Modulo => {
                if b == 0 {
                    None
                } else {
                    a.checked_rem(b)
                }
            }
            Equal => return LatticeValue::Constant(IrLiteralValue::Bool(a == b)),
            NotEqual => return LatticeValue::Constant(IrLiteralValue::Bool(a != b)),
            Less => return LatticeValue::Constant(IrLiteralValue::Bool(a < b)),
            LessEqual => return LatticeValue::Constant(IrLiteralValue::Bool(a <= b)),
            Greater => return LatticeValue::Constant(IrLiteralValue::Bool(a > b)),
            GreaterEqual => return LatticeValue::Constant(IrLiteralValue::Bool(a >= b)),
            BitwiseAnd => return LatticeValue::Constant(IrLiteralValue::I32(a & b)),
            BitwiseOr => return LatticeValue::Constant(IrLiteralValue::I32(a | b)),
            BitwiseXor => return LatticeValue::Constant(IrLiteralValue::I32(a ^ b)),
            ShiftLeft => b.try_into().ok().and_then(|shift| a.checked_shl(shift)),
            ShiftRight => b.try_into().ok().and_then(|shift| a.checked_shr(shift)),
            And | Or => return LatticeValue::Bottom, // Logical ops not valid for i32
        };
        result.map(|r| LatticeValue::Constant(IrLiteralValue::I32(r))).unwrap_or(LatticeValue::Bottom)
    }

    fn eval_i64_binop(op: &IrBinaryOp, a: i64, b: i64) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            Add => a.checked_add(b),
            Sub => a.checked_sub(b),
            Mul => a.checked_mul(b),
            Div => {
                if b == 0 {
                    None
                } else {
                    a.checked_div(b)
                }
            }
            Mod => {
                if b == 0 {
                    None
                } else {
                    a.checked_rem(b)
                }
            }
            Eq => return LatticeValue::Constant(IrLiteralValue::Bool(a == b)),
            Ne => return LatticeValue::Constant(IrLiteralValue::Bool(a != b)),
            Lt => return LatticeValue::Constant(IrLiteralValue::Bool(a < b)),
            Le => return LatticeValue::Constant(IrLiteralValue::Bool(a <= b)),
            Gt => return LatticeValue::Constant(IrLiteralValue::Bool(a > b)),
            Ge => return LatticeValue::Constant(IrLiteralValue::Bool(a >= b)),
            And => return LatticeValue::Constant(IrLiteralValue::I64(a & b)),
            Or => return LatticeValue::Constant(IrLiteralValue::I64(a | b)),
            Xor => return LatticeValue::Constant(IrLiteralValue::I64(a ^ b)),
            Shl => b.try_into().ok().and_then(|shift| a.checked_shl(shift)),
            Shr => b.try_into().ok().and_then(|shift| a.checked_shr(shift)),
        };
        result.map(|r| LatticeValue::Constant(IrLiteralValue::I64(r))).unwrap_or(LatticeValue::Bottom)
    }

    fn eval_u32_binop(op: &IrBinaryOp, a: u32, b: u32) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            Add => a.checked_add(b),
            Sub => a.checked_sub(b),
            Mul => a.checked_mul(b),
            Div => {
                if b == 0 {
                    None
                } else {
                    a.checked_div(b)
                }
            }
            Mod => {
                if b == 0 {
                    None
                } else {
                    a.checked_rem(b)
                }
            }
            Eq => return LatticeValue::Constant(IrLiteralValue::Bool(a == b)),
            Ne => return LatticeValue::Constant(IrLiteralValue::Bool(a != b)),
            Lt => return LatticeValue::Constant(IrLiteralValue::Bool(a < b)),
            Le => return LatticeValue::Constant(IrLiteralValue::Bool(a <= b)),
            Gt => return LatticeValue::Constant(IrLiteralValue::Bool(a > b)),
            Ge => return LatticeValue::Constant(IrLiteralValue::Bool(a >= b)),
            And => return LatticeValue::Constant(IrLiteralValue::U32(a & b)),
            Or => return LatticeValue::Constant(IrLiteralValue::U32(a | b)),
            Xor => return LatticeValue::Constant(IrLiteralValue::U32(a ^ b)),
            Shl => b.try_into().ok().and_then(|shift| a.checked_shl(shift)),
            Shr => b.try_into().ok().and_then(|shift| a.checked_shr(shift)),
        };
        result.map(|r| LatticeValue::Constant(IrLiteralValue::U32(r))).unwrap_or(LatticeValue::Bottom)
    }

    fn eval_u64_binop(op: &IrBinaryOp, a: u64, b: u64) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            Add => a.checked_add(b),
            Sub => a.checked_sub(b),
            Mul => a.checked_mul(b),
            Div => {
                if b == 0 {
                    None
                } else {
                    a.checked_div(b)
                }
            }
            Mod => {
                if b == 0 {
                    None
                } else {
                    a.checked_rem(b)
                }
            }
            Eq => return LatticeValue::Constant(IrLiteralValue::Bool(a == b)),
            Ne => return LatticeValue::Constant(IrLiteralValue::Bool(a != b)),
            Lt => return LatticeValue::Constant(IrLiteralValue::Bool(a < b)),
            Le => return LatticeValue::Constant(IrLiteralValue::Bool(a <= b)),
            Gt => return LatticeValue::Constant(IrLiteralValue::Bool(a > b)),
            Ge => return LatticeValue::Constant(IrLiteralValue::Bool(a >= b)),
            And => return LatticeValue::Constant(IrLiteralValue::U64(a & b)),
            Or => return LatticeValue::Constant(IrLiteralValue::U64(a | b)),
            Xor => return LatticeValue::Constant(IrLiteralValue::U64(a ^ b)),
            Shl => b.try_into().ok().and_then(|shift: u32| a.checked_shl(shift)),
            Shr => b.try_into().ok().and_then(|shift: u32| a.checked_shr(shift)),
        };
        result.map(|r| LatticeValue::Constant(IrLiteralValue::U64(r))).unwrap_or(LatticeValue::Bottom)
    }

    fn eval_bool_binop(op: &IrBinaryOp, a: bool, b: bool) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            And => a && b,
            Or => a || b,
            Xor => a ^ b,
            Eq => a == b,
            Ne => a != b,
            _ => return LatticeValue::Bottom,
        };
        LatticeValue::Constant(IrLiteralValue::Bool(result))
    }

    fn eval_f32_binop(op: &IrBinaryOp, a: f32, b: f32) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
            Mod => a % b,
            Eq => return LatticeValue::Constant(IrLiteralValue::Bool(a == b)),
            Ne => return LatticeValue::Constant(IrLiteralValue::Bool(a != b)),
            Lt => return LatticeValue::Constant(IrLiteralValue::Bool(a < b)),
            Le => return LatticeValue::Constant(IrLiteralValue::Bool(a <= b)),
            Gt => return LatticeValue::Constant(IrLiteralValue::Bool(a > b)),
            Ge => return LatticeValue::Constant(IrLiteralValue::Bool(a >= b)),
            _ => return LatticeValue::Bottom,
        };
        if result.is_nan() || result.is_infinite() {
            LatticeValue::Bottom
        } else {
            LatticeValue::Constant(IrLiteralValue::F32(result))
        }
    }

    fn eval_f64_binop(op: &IrBinaryOp, a: f64, b: f64) -> LatticeValue {
        use IrBinaryOp::*;
        let result = match op {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
            Mod => a % b,
            Eq => return LatticeValue::Constant(IrLiteralValue::Bool(a == b)),
            Ne => return LatticeValue::Constant(IrLiteralValue::Bool(a != b)),
            Lt => return LatticeValue::Constant(IrLiteralValue::Bool(a < b)),
            Le => return LatticeValue::Constant(IrLiteralValue::Bool(a <= b)),
            Gt => return LatticeValue::Constant(IrLiteralValue::Bool(a > b)),
            Ge => return LatticeValue::Constant(IrLiteralValue::Bool(a >= b)),
            _ => return LatticeValue::Bottom,
        };
        if result.is_nan() || result.is_infinite() {
            LatticeValue::Bottom
        } else {
            LatticeValue::Constant(IrLiteralValue::F64(result))
        }
    }
}

impl Default for InstructionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

// Module-level convenience functions for external use
pub fn evaluate_binary_op(op: IrBinaryOp, left: &LatticeValue, right: &LatticeValue) -> LatticeValue {
    InstructionEvaluator::evaluate_binary_op(&op, left, right)
}

pub fn evaluate_unary_op(op: IrUnaryOp, operand: &LatticeValue) -> LatticeValue {
    InstructionEvaluator::evaluate_unary_op(&op, operand)
}
