// src/rvir/instruction.rs
use super::{RIrType, RValue, RScopeId};
use crate::{
    location::source_span::SourceSpan,
    parser::ast::{BinaryOp, UnaryOp},
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RCastKind {
    IntToPtr,
    PtrToInt,
    FloatToInt,
    IntToFloat,
    FloatTruncate,
    FloatExtend,
    IntTruncate,
    IntSignExtend,
    IntZeroExtend,
    Bitcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RVectorOp {
    Add,
    Sub,
    Mul,
    Div,
    DotProduct,
    Shuffle,
}

impl fmt::Display for RVectorOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RVectorOp::Add => write!(f, "vadd"),
            RVectorOp::Sub => write!(f, "vsub"),
            RVectorOp::Mul => write!(f, "vmul"),
            RVectorOp::Div => write!(f, "vdiv"),
            RVectorOp::DotProduct => write!(f, "vdot"),
            RVectorOp::Shuffle => write!(f, "vshuffle"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RInstruction {
    pub kind: RInstructionKind,
    pub result: Option<RValue>,
    pub debug_info: DebugInfo,
    pub scope: Option<RScopeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RInstructionKind {
    Alloca {
        ty: RIrType,
    },
    Store {
        value: RValue,
        dest: RValue,
    },
    Load {
        src: RValue,
        ty: RIrType,
    },
    Binary {
        op: RIrBinaryOp,
        left: RValue,
        right: RValue,
        ty: RIrType,
    },
    Unary {
        op: RIrUnaryOp,
        operand: RValue,
        ty: RIrType,
    },
    Call {
        func: RValue,
        args: Vec<RValue>,
        ty: RIrType,
    },
    GetElementPtr {
        base: RValue,
        index: RValue,
        element_ty: RIrType,
    },
    Cast {
        kind: RCastKind,
        value: RValue,
        from_ty: RIrType,
        to_ty: RIrType,
    },
    Phi {
        ty: RIrType,
        incoming: Vec<(RValue, String)>,
    },
    Vector {
        op: RVectorOp,
        operands: Vec<RValue>,
        ty: RIrType,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum RIrBinaryOp {
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

impl From<BinaryOp> for RIrBinaryOp {
    fn from(op: BinaryOp) -> Self {
        match op {
            BinaryOp::Add => RIrBinaryOp::Add,
            BinaryOp::Subtract => RIrBinaryOp::Subtract,
            BinaryOp::Multiply => RIrBinaryOp::Multiply,
            BinaryOp::Divide => RIrBinaryOp::Divide,
            BinaryOp::Modulo => RIrBinaryOp::Modulo,
            BinaryOp::Equal => RIrBinaryOp::Equal,
            BinaryOp::NotEqual => RIrBinaryOp::NotEqual,
            BinaryOp::Less => RIrBinaryOp::Less,
            BinaryOp::LessEqual => RIrBinaryOp::LessEqual,
            BinaryOp::Greater => RIrBinaryOp::Greater,
            BinaryOp::GreaterEqual => RIrBinaryOp::GreaterEqual,
            BinaryOp::And => RIrBinaryOp::And,
            BinaryOp::Or => RIrBinaryOp::Or,
            BinaryOp::BitwiseAnd => RIrBinaryOp::BitwiseAnd,
            BinaryOp::BitwiseOr => RIrBinaryOp::BitwiseOr,
            BinaryOp::BitwiseXor => RIrBinaryOp::BitwiseXor,
            BinaryOp::ShiftLeft => RIrBinaryOp::ShiftLeft,
            BinaryOp::ShiftRight => RIrBinaryOp::ShiftRight,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RIrUnaryOp {
    Negate,
    Not,
}

impl From<UnaryOp> for RIrUnaryOp {
    fn from(op: UnaryOp) -> Self {
        match op {
            UnaryOp::Negate => RIrUnaryOp::Negate,
            UnaryOp::Not => RIrUnaryOp::Not,
        }
    }
}

impl RInstruction {
    pub fn new(kind: RInstructionKind, span: SourceSpan) -> Self {
        RInstruction {
            kind,
            result: None,
            debug_info: DebugInfo { source_span: span },
            scope: None,
        }
    }

    pub fn with_result(mut self, result: RValue) -> Self {
        self.result = Some(result);
        self
    }

    pub fn with_scope(mut self, scope: RScopeId) -> Self {
        self.scope = Some(scope);
        self
    }
}

impl fmt::Display for RInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result_str = if let Some(result) = &self.result {
            format!("{result} = ")
        } else {
            String::new()
        };

        match &self.kind {
            RInstructionKind::Alloca { ty } => write!(f, "{result_str}alloca {ty}"),
            RInstructionKind::Store { value, dest } => write!(f, "store {value} to {dest}"),
            RInstructionKind::Load { src, ty } => write!(f, "{result_str}load {ty} from {src}"),
            RInstructionKind::Binary { op, left, right, ty } => write!(f, "{result_str}{op} {left} {right}, {ty}"),
            RInstructionKind::Unary { op, operand, ty } => write!(f, "{result_str}{op} {operand}, {ty}"),
            RInstructionKind::Call { func, args, ty } => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{result_str} call {func}({args_str}) : {ty}")
            }
            RInstructionKind::GetElementPtr { base, index, element_ty } => write!(f, "{result_str} getelementptr {base}, {index} : {element_ty}"),
            RInstructionKind::Cast { kind: _, value, from_ty, to_ty } => write!(f, "{result_str} cast {value} from {from_ty} to {to_ty}"),

            RInstructionKind::Phi { ty, incoming } => {
                let incoming_str = incoming
                    .iter()
                    .map(|(val, block)| format!("[ {val}, {block} ]"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{result_str} phi {ty} [ {incoming_str} ]")
            }
            RInstructionKind::Vector { op, operands, ty } => {
                let operands_str = operands
                    .iter()
                    .map(|op| op.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{result_str} vector.{op} {operands_str} : {ty}")
            },
        }
    }
}

impl fmt::Display for RIrBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RIrBinaryOp::Add => write!(f, "add"),
            RIrBinaryOp::Subtract => write!(f, "sub"),
            RIrBinaryOp::Multiply => write!(f, "mul"),
            RIrBinaryOp::Divide => write!(f, "div"),
            RIrBinaryOp::Modulo => write!(f, "mod"),
            RIrBinaryOp::Equal => write!(f, "eq"),
            RIrBinaryOp::NotEqual => write!(f, "ne"),
            RIrBinaryOp::Less => write!(f, "lt"),
            RIrBinaryOp::LessEqual => write!(f, "le"),
            RIrBinaryOp::Greater => write!(f, "gt"),
            RIrBinaryOp::GreaterEqual => write!(f, "ge"),
            RIrBinaryOp::And => write!(f, "and"),
            RIrBinaryOp::Or => write!(f, "or"),
            RIrBinaryOp::BitwiseAnd => write!(f, "bitand"),
            RIrBinaryOp::BitwiseOr => write!(f, "bitor"),
            RIrBinaryOp::BitwiseXor => write!(f, "bitxor"),
            RIrBinaryOp::ShiftLeft => write!(f, "shl"),
            RIrBinaryOp::ShiftRight => write!(f, "shr"),
        }
    }
}

impl fmt::Display for RIrUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RIrUnaryOp::Negate => write!(f, "neg"),
            RIrUnaryOp::Not => write!(f, "not"),
        }
    }
}
