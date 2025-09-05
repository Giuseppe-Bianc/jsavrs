// src/nir/instruction.rs
use super::{IrType, ScopeId, Value};
use crate::{
    location::source_span::SourceSpan,
    parser::ast::{BinaryOp, UnaryOp},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CastKind {
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

#[derive(Debug, Clone, PartialEq)]
pub enum VectorOp {
    Add,
    Sub,
    Mul,
    Div,
    DotProduct,
    Shuffle,
}

impl fmt::Display for VectorOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorOp::Add => write!(f, "vadd"),
            VectorOp::Sub => write!(f, "vsub"),
            VectorOp::Mul => write!(f, "vmul"),
            VectorOp::Div => write!(f, "vdiv"),
            VectorOp::DotProduct => write!(f, "vdot"),
            VectorOp::Shuffle => write!(f, "vshuffle"),
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
        let result_str = if let Some(result) = &self.result { format!("{result} = ") } else { String::new() };

        match &self.kind {
            InstructionKind::Alloca { ty } => write!(f, "{result_str}alloca {ty}"),
            InstructionKind::Store { value, dest } => write!(f, "store {value} to {dest}"),
            InstructionKind::Load { src, ty } => write!(f, "{result_str}load {ty} from {src}"),
            InstructionKind::Binary { op, left, right, ty } => write!(f, "{result_str}{op} {left} {right}, {ty}"),
            InstructionKind::Unary { op, operand, ty } => write!(f, "{result_str}{op} {operand} {ty}"),
            InstructionKind::Call { func, args, ty } => {
                let args_str = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "{result_str}{func}({args_str}) : {ty}")
            }
            InstructionKind::GetElementPtr { base, index, element_ty } => {
                write!(f, "{result_str} getelementptr {base}, {index} : {element_ty}")
            }
            InstructionKind::Cast { kind: _, value, from_ty, to_ty } => {
                write!(f, "{result_str} cast {value} from {from_ty} to {to_ty}")
            }

            InstructionKind::Phi { ty, incoming } => {
                let incoming_str =
                    incoming.iter().map(|(val, block)| format!("[ {val}, {block} ]")).collect::<Vec<_>>().join(", ");
                write!(f, "{result_str} phi {ty} [ {incoming_str} ]")
            }
            InstructionKind::Vector { op, operands, ty } => {
                let operands_str = operands.iter().map(|op| op.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "{result_str} vector.{op} {operands_str} : {ty}")
            }
        }
    }
}

impl fmt::Display for IrBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrBinaryOp::Add => write!(f, "add"),
            IrBinaryOp::Subtract => write!(f, "sub"),
            IrBinaryOp::Multiply => write!(f, "mul"),
            IrBinaryOp::Divide => write!(f, "div"),
            IrBinaryOp::Modulo => write!(f, "mod"),
            IrBinaryOp::Equal => write!(f, "eq"),
            IrBinaryOp::NotEqual => write!(f, "ne"),
            IrBinaryOp::Less => write!(f, "lt"),
            IrBinaryOp::LessEqual => write!(f, "le"),
            IrBinaryOp::Greater => write!(f, "gt"),
            IrBinaryOp::GreaterEqual => write!(f, "ge"),
            IrBinaryOp::And => write!(f, "and"),
            IrBinaryOp::Or => write!(f, "or"),
            IrBinaryOp::BitwiseAnd => write!(f, "bitand"),
            IrBinaryOp::BitwiseOr => write!(f, "bitor"),
            IrBinaryOp::BitwiseXor => write!(f, "bitxor"),
            IrBinaryOp::ShiftLeft => write!(f, "shl"),
            IrBinaryOp::ShiftRight => write!(f, "shr"),
        }
    }
}

impl fmt::Display for IrUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrUnaryOp::Negate => write!(f, "neg"),
            IrUnaryOp::Not => write!(f, "not"),
        }
    }
}
