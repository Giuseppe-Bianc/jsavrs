//! IR instruction definitions and operations.
//!
//! This module defines all instruction types in the intermediate representation,
//! including arithmetic operations, memory operations, control flow, and type
//! conversions. Instructions are the fundamental building blocks of IR functions.
//!
//! # Instruction Categories
//!
//! - **Arithmetic**: Binary and unary operations on numeric types
//! - **Memory**: Load, store, and allocation operations
//! - **Control Flow**: Branches, calls, and returns
//! - **Type Conversions**: Casts between different types
//! - **Vector Operations**: SIMD operations on vector types
//!
//! # Design
//!
//! Each instruction carries optional result value, debug information for source
//! tracking, and scope information for symbol resolution.

// src/ir/instruction.rs
use super::{IrType, ScopeId, Value};
use crate::fmtlike::write_comma_separated;
use crate::{
    location::source_span::SourceSpan,
    parser::ast::{BinaryOp, UnaryOp},
};
use std::fmt;

/// Type casting operations for value conversions in IR.
///
/// Cast operations convert values between different types, handling sign extension,
/// truncation, and reinterpretation as needed. Each cast kind represents a specific
/// conversion strategy with well-defined semantics.
///
/// # Integer Conversions
///
/// - **Zero Extension**: Unsigned widening preserving value (u8 → u32)
/// - **Sign Extension**: Signed widening preserving value (i8 → i32)
/// - **Truncation**: Narrowing losing high bits (u64 → u16)
/// - **Bitcast**: Same-width reinterpretation (i32 ↔ u32)
///
/// # Float Conversions
///
/// - **Truncation**: Precision loss (f64 → f32)
/// - **Extension**: Precision gain (f32 → f64)
/// - **To/From Int**: Float ↔ integer conversion
///
/// # String Conversions
///
/// Support parsing and formatting between strings and primitive types.
///
/// # Safety
///
/// Most casts are safe, but some (like `IntToChar`) may fail at runtime if
/// the value is invalid for the target type.
///
/// # Examples
///
/// ```
/// // Sign extension: -1_i8 (0xFF) becomes -1_i32 (0xFFFFFFFF)
/// let cast = CastKind::IntSignExtend;
///
/// // Zero extension: 255_u8 (0xFF) becomes 255_u32 (0x000000FF)
/// let cast = CastKind::IntZeroExtend;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CastKind {
    /// Default (safe) integer widening cast
    #[default]
    IntZeroExtend, // Unsigned widening (u8 -> u32)
    IntSignExtend, // Signed widening (i8 -> i32)
    IntTruncate,   // Narrowing (u64 -> u16, i64 -> i32)

    /// Signed ↔ Unsigned of same width (bit reinterpret)
    IntBitcast, // i32 ↔ u32 (same bit width reinterpretation)

    /// Integer ↔ Float
    IntToFloat, // i32 -> f32, u64 -> f64
    FloatToInt, // f32 -> i32, f64 -> u64

    /// Float ↔ Float
    FloatTruncate, // f64 -> f32
    FloatExtend, // f32 -> f64

    /// Integer/Float ↔ Bool
    BoolToInt, // bool -> u8/i32
    IntToBool,   // i32 -> bool (nonzero)
    BoolToFloat, // bool -> f32/f64 (0.0 or 1.0)
    FloatToBool, // f32/f64 -> bool (nonzero)

    /// Char ↔ Integer
    CharToInt, // char -> u32 (Unicode scalar)
    IntToChar, // u32 -> char (checked, valid Unicode only)

    /// Char ↔ String
    CharToString, // char -> String
    StringToChar, // String (len == 1) -> char

    /// String ↔ Numeric/Bool
    StringToInt, // "123" -> 123_i32 (via parse)
    StringToFloat, // "3.14" -> f64 (via parse)
    StringToBool,  // "true" -> true (via parse)
    IntToString,   // 42 -> "42"
    FloatToString, // 3.14 -> "3.14"
    BoolToString,  // true -> "true"

    /*/// Pointer conversions
    IntToPtr,               // usize/u64 -> *const T
    PtrToInt,               // *const T -> usize/u64
    PtrCast,                // *const A -> *const B (bit reinterpretation)
    RefToPtr,               // &T -> *const T
    PtrToRef,               // *const T -> &T (unsafe)*/
    /// Bit reinterpretation (same size types)
    Bitcast, // f32 <-> u32, f64 <-> u64, pointer <-> pointer
}

/// Vector (SIMD) operations for parallel computation.
///
/// These operations work on vector types, performing the same operation on
/// multiple data elements simultaneously. Vector operations enable efficient
/// use of modern CPU SIMD instruction sets (SSE, AVX, NEON).
///
/// # Operations
///
/// * `Add` - Element-wise addition
/// * `Sub` - Element-wise subtraction
/// * `Mul` - Element-wise multiplication
/// * `Div` - Element-wise division
/// * `DotProduct` - Vector dot product (sum of element-wise products)
/// * `Shuffle` - Reorder vector elements according to a mask
///
/// # Examples
///
/// ```
/// // Vector addition: [1, 2, 3] + [4, 5, 6] = [5, 7, 9]
/// let op = VectorOp::Add;
///
/// // Dot product: [1, 2, 3] · [4, 5, 6] = 1*4 + 2*5 + 3*6 = 32
/// let op = VectorOp::DotProduct;
/// ```
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
            Self::Add => f.write_str("vadd"),
            Self::Sub => f.write_str("vsub"),
            Self::Mul => f.write_str("vmul"),
            Self::Div => f.write_str("vdiv"),
            Self::DotProduct => f.write_str("vdot"),
            Self::Shuffle => f.write_str("vshuffle"),
        }
    }
}

/// An IR instruction with metadata.
///
/// Instructions represent individual operations in the IR. Each instruction
/// has a kind (the operation to perform), an optional result value, debug
/// information for source tracking, and scope information for symbol resolution.
///
/// # Fields
///
/// * `kind` - The specific operation this instruction performs
/// * `result` - Optional value produced by this instruction
/// * `debug_info` - Source location information for debugging
/// * `scope` - Optional scope for variable/symbol resolution
///
/// # Examples
///
/// ```
/// let instruction = Instruction {
///     kind: InstructionKind::BinaryOp { op: IrBinaryOp::Add, lhs, rhs },
///     result: Some(result_value),
///     debug_info: DebugInfo { source_span },
///     scope: Some(scope_id),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub result: Option<Value>,
    pub debug_info: DebugInfo,
    pub scope: Option<ScopeId>,
}

/// Debug information attached to instructions.
///
/// Tracks the source code location that generated this instruction,
/// enabling accurate error messages and debugger integration.
///
/// # Fields
///
/// * `source_span` - The span in source code this instruction originated from
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugInfo {
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            BinaryOp::Add => Self::Add,
            BinaryOp::Subtract => Self::Subtract,
            BinaryOp::Multiply => Self::Multiply,
            BinaryOp::Divide => Self::Divide,
            BinaryOp::Modulo => Self::Modulo,
            BinaryOp::Equal => Self::Equal,
            BinaryOp::NotEqual => Self::NotEqual,
            BinaryOp::Less => Self::Less,
            BinaryOp::LessEqual => Self::LessEqual,
            BinaryOp::Greater => Self::Greater,
            BinaryOp::GreaterEqual => Self::GreaterEqual,
            BinaryOp::And => Self::And,
            BinaryOp::Or => Self::Or,
            BinaryOp::BitwiseAnd => Self::BitwiseAnd,
            BinaryOp::BitwiseOr => Self::BitwiseOr,
            BinaryOp::BitwiseXor => Self::BitwiseXor,
            BinaryOp::ShiftLeft => Self::ShiftLeft,
            BinaryOp::ShiftRight => Self::ShiftRight,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrUnaryOp {
    Negate,
    Not,
}

impl From<UnaryOp> for IrUnaryOp {
    fn from(op: UnaryOp) -> Self {
        match op {
            UnaryOp::Negate => Self::Negate,
            UnaryOp::Not => Self::Not,
        }
    }
}

impl Instruction {
    #[must_use]
    pub const fn new(kind: InstructionKind, span: SourceSpan) -> Self {
        Self { kind, result: None, debug_info: DebugInfo { source_span: span }, scope: None }
    }

    #[must_use]
    pub fn with_result(mut self, result: Value) -> Self {
        self.result = Some(result);
        self
    }

    #[must_use]
    pub const fn with_scope(mut self, scope: ScopeId) -> Self {
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
                write_comma_separated(f, args)?;
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
                write_comma_separated(f, operands)?;
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
            Self::Add => f.write_str("add"),
            Self::Subtract => f.write_str("sub"),
            Self::Multiply => f.write_str("mul"),
            Self::Divide => f.write_str("div"),
            Self::Modulo => f.write_str("mod"),
            Self::Equal => f.write_str("eq"),
            Self::NotEqual => f.write_str("ne"),
            Self::Less => f.write_str("lt"),
            Self::LessEqual => f.write_str("le"),
            Self::Greater => f.write_str("gt"),
            Self::GreaterEqual => f.write_str("ge"),
            Self::And => f.write_str("and"),
            Self::Or => f.write_str("or"),
            Self::BitwiseAnd => f.write_str("bitand"),
            Self::BitwiseOr => f.write_str("bitor"),
            Self::BitwiseXor => f.write_str("bitxor"),
            Self::ShiftLeft => f.write_str("shl"),
            Self::ShiftRight => f.write_str("shr"),
        }
    }
}

impl fmt::Display for IrUnaryOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negate => f.write_str("neg"),
            Self::Not => f.write_str("not"),
        }
    }
}
