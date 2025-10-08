// src/ir/instruction.rs
use super::{IrType, ScopeId, Value};
use crate::{
    location::source_span::SourceSpan,
    parser::ast::{BinaryOp, UnaryOp},
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CastKind {
    /// Default (safe) integer widening cast
    #[default]
    IntZeroExtend,          // Unsigned widening (u8 -> u32)
    IntSignExtend,          // Signed widening (i8 -> i32)
    IntTruncate,            // Narrowing (u64 -> u16, i64 -> i32)

    /// Signed ↔ Unsigned of same width (bit reinterpret)
    IntBitcast,             // i32 ↔ u32 (same bit width reinterpretation)

    /// Integer ↔ Float
    IntToFloat,             // i32 -> f32, u64 -> f64
    FloatToInt,             // f32 -> i32, f64 -> u64

    /// Float ↔ Float
    FloatTruncate,          // f64 -> f32
    FloatExtend,            // f32 -> f64

    /// Integer/Float ↔ Bool
    BoolToInt,              // bool -> u8/i32
    IntToBool,              // i32 -> bool (nonzero)
    BoolToFloat,            // bool -> f32/f64 (0.0 or 1.0)
    FloatToBool,            // f32/f64 -> bool (nonzero)

    /// Char ↔ Integer
    CharToInt,              // char -> u32 (Unicode scalar)
    IntToChar,              // u32 -> char (checked, valid Unicode only)

    /// Char ↔ String
    CharToString,           // char -> String
    StringToChar,           // String (len == 1) -> char

    /// String ↔ Numeric/Bool
    StringToInt,            // "123" -> 123_i32 (via parse)
    StringToFloat,          // "3.14" -> f64 (via parse)
    StringToBool,           // "true" -> true (via parse)
    IntToString,            // 42 -> "42"
    FloatToString,          // 3.14 -> "3.14"
    BoolToString,           // true -> "true"

    /*/// Pointer conversions
    IntToPtr,               // usize/u64 -> *const T
    PtrToInt,               // *const T -> usize/u64
    PtrCast,                // *const A -> *const B (bit reinterpretation)
    RefToPtr,               // &T -> *const T
    PtrToRef,               // *const T -> &T (unsafe)*/

    /// Bit reinterpretation (same size types)
    Bitcast,                // f32 <-> u32, f64 <-> u64, pointer <-> pointer
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorOp {
    Add,
    Sub,
    Mul,
    Div,
    DotProduct,
    Shuffle,
}

impl fmt::Display for VectorOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorOp::Add => f.write_str("vadd"),
            VectorOp::Sub => f.write_str("vsub"),
            VectorOp::Mul => f.write_str("vmul"),
            VectorOp::Div => f.write_str("vdiv"),
            VectorOp::DotProduct => f.write_str("vdot"),
            VectorOp::Shuffle => f.write_str("vshuffle"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub result: Option<Value>,
    pub debug_info: DebugInfo,
    pub scope: Option<ScopeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionKind {
    Alloca { ty: IrType },
    Store { value: Value, dest: Value },
    Load { src: Value, ty: IrType },
    Binary { op: IrBinaryOp, left: Value, right: Value, ty: IrType },
    Unary { op: IrUnaryOp, operand: Value, ty: IrType },
    Call { func: Value, args: Vec<Value>, ty: IrType },
    GetElementPtr { base: Value, index: Value, element_ty: IrType },
    Cast { kind: CastKind, value: Value, from_ty: IrType, to_ty: IrType },
    Phi { ty: IrType, incoming: Vec<(Value, String)> },
    Vector { op: VectorOp, operands: Vec<Value>, ty: IrType },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

impl From<BinaryOp> for IrBinaryOp {
    fn from(op: BinaryOp) -> Self {
        match op {
            BinaryOp::Add => IrBinaryOp::Add,
            BinaryOp::Subtract => IrBinaryOp::Subtract,
            BinaryOp::Multiply => IrBinaryOp::Multiply,
            BinaryOp::Divide => IrBinaryOp::Divide,
            BinaryOp::Modulo => IrBinaryOp::Modulo,
            BinaryOp::Equal => IrBinaryOp::Equal,
            BinaryOp::NotEqual => IrBinaryOp::NotEqual,
            BinaryOp::Less => IrBinaryOp::Less,
            BinaryOp::LessEqual => IrBinaryOp::LessEqual,
            BinaryOp::Greater => IrBinaryOp::Greater,
            BinaryOp::GreaterEqual => IrBinaryOp::GreaterEqual,
            BinaryOp::And => IrBinaryOp::And,
            BinaryOp::Or => IrBinaryOp::Or,
            BinaryOp::BitwiseAnd => IrBinaryOp::BitwiseAnd,
            BinaryOp::BitwiseOr => IrBinaryOp::BitwiseOr,
            BinaryOp::BitwiseXor => IrBinaryOp::BitwiseXor,
            BinaryOp::ShiftLeft => IrBinaryOp::ShiftLeft,
            BinaryOp::ShiftRight => IrBinaryOp::ShiftRight,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrUnaryOp {
    Negate,
    Not,
}

impl From<UnaryOp> for IrUnaryOp {
    fn from(op: UnaryOp) -> Self {
        match op {
            UnaryOp::Negate => IrUnaryOp::Negate,
            UnaryOp::Not => IrUnaryOp::Not,
        }
    }
}

impl Instruction {
    pub fn new(kind: InstructionKind, span: SourceSpan) -> Self {
        Instruction { kind, result: None, debug_info: DebugInfo { source_span: span }, scope: None }
    }

    pub fn with_result(mut self, result: Value) -> Self {
        self.result = Some(result);
        self
    }

    pub fn with_scope(mut self, scope: ScopeId) -> Self {
        self.scope = Some(scope);
        self
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write result prefix if present
        if let Some(result) = &self.result {
            result.fmt(f)?;
            f.write_str(" = ")?;
        }

        match &self.kind {
            InstructionKind::Alloca { ty } => {
                f.write_str("alloca ")?;
                ty.fmt(f)
            }
            InstructionKind::Store { value, dest } => {
                f.write_str("store ")?;
                value.fmt(f)?;
                f.write_str(" to ")?;
                dest.fmt(f)
            }
            InstructionKind::Load { src, ty } => {
                f.write_str("load ")?;
                ty.fmt(f)?;
                f.write_str(" from ")?;
                src.fmt(f)
            }
            InstructionKind::Binary { op, left, right, ty } => {
                op.fmt(f)?;
                f.write_str(" ")?;
                left.fmt(f)?;
                f.write_str(" ")?;
                right.fmt(f)?;
                f.write_str(", ")?;
                ty.fmt(f)
            }
            InstructionKind::Unary { op, operand, ty } => {
                op.fmt(f)?;
                f.write_str(" ")?;
                operand.fmt(f)?;
                f.write_str(" ")?;
                ty.fmt(f)
            }
            InstructionKind::Call { func, args, ty } => {
                f.write_str(" call ")?;
                func.fmt(f)?;
                f.write_str("(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    arg.fmt(f)?;
                }
                f.write_str(") : ")?;
                ty.fmt(f)
            }
            InstructionKind::GetElementPtr { base, index, element_ty } => {
                f.write_str(" getelementptr ")?;
                base.fmt(f)?;
                f.write_str(", ")?;
                index.fmt(f)?;
                f.write_str(" : ")?;
                element_ty.fmt(f)
            }
            InstructionKind::Cast { kind: _, value, from_ty, to_ty } => {
                f.write_str(" cast ")?;
                value.fmt(f)?;
                f.write_str(" from ")?;
                from_ty.fmt(f)?;
                f.write_str(" to ")?;
                to_ty.fmt(f)
            }
            InstructionKind::Phi { ty, incoming } => {
                f.write_str(" phi ")?;
                ty.fmt(f)?;
                f.write_str(" [ ")?;
                for (i, (val, block)) in incoming.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    f.write_str("[ ")?;
                    val.fmt(f)?;
                    f.write_str(", ")?;
                    block.fmt(f)?;
                    f.write_str(" ]")?;
                }
                f.write_str(" ]")
            }
            InstructionKind::Vector { op, operands, ty } => {
                f.write_str(" vector.")?;
                op.fmt(f)?;
                f.write_str(" ")?;
                for (i, operand) in operands.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    operand.fmt(f)?;
                }
                f.write_str(" : ")?;
                ty.fmt(f)
            }
        }
    }
}

impl fmt::Display for IrBinaryOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrBinaryOp::Add => f.write_str("add"),
            IrBinaryOp::Subtract => f.write_str("sub"),
            IrBinaryOp::Multiply => f.write_str("mul"),
            IrBinaryOp::Divide => f.write_str("div"),
            IrBinaryOp::Modulo => f.write_str("mod"),
            IrBinaryOp::Equal => f.write_str("eq"),
            IrBinaryOp::NotEqual => f.write_str("ne"),
            IrBinaryOp::Less => f.write_str("lt"),
            IrBinaryOp::LessEqual => f.write_str("le"),
            IrBinaryOp::Greater => f.write_str("gt"),
            IrBinaryOp::GreaterEqual => f.write_str("ge"),
            IrBinaryOp::And => f.write_str("and"),
            IrBinaryOp::Or => f.write_str("or"),
            IrBinaryOp::BitwiseAnd => f.write_str("bitand"),
            IrBinaryOp::BitwiseOr => f.write_str("bitor"),
            IrBinaryOp::BitwiseXor => f.write_str("bitxor"),
            IrBinaryOp::ShiftLeft => f.write_str("shl"),
            IrBinaryOp::ShiftRight => f.write_str("shr"),
        }
    }
}

impl fmt::Display for IrUnaryOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrUnaryOp::Negate => f.write_str("neg"),
            IrUnaryOp::Not => f.write_str("not"),
        }
    }
}
