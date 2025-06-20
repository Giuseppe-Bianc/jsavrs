//src/ir/instruction.rs
use super::{types::IrType, value::Value};
use std::fmt;

/// Binary operations in IR
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

/// Unary operations in IR
#[derive(Debug, Clone, PartialEq)]
pub enum IrUnaryOp {
    Negate,
    Not,
}

/// IR instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Alloca {
        dest: String,
        ty: IrType,
    },
    Store {
        value: Value,
        dest: Value,
    },
    Load {
        dest: String,
        src: Value,
        ty: IrType,
    },
    Binary {
        op: IrBinaryOp,
        dest: String,
        left: Value,
        right: Value,
        ty: IrType,
    },
    Unary {
        op: IrUnaryOp,
        dest: String,
        operand: Value,
        ty: IrType,
    },
    Call {
        dest: Option<String>,
        func: String,
        args: Vec<Value>,
        ty: IrType,
    },
    GetElementPtr {
        dest: String,
        base: Value,
        index: Value,
        element_ty: IrType,
    },
    Cast {
        dest: String,
        value: Value,
        from_ty: IrType,
        to_ty: IrType,
    },
    Phi {
        dest: String,
        ty: IrType,
        incoming: Vec<(Value, String)>,
    },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Alloca { dest, ty } => write!(f, "{dest} = alloca {ty}"),
            Instruction::Store { value, dest } => write!(f, "store {value} to {dest}"),
            Instruction::Load { dest, src, ty } => write!(f, "{dest} = load {ty} from {src}"),
            Instruction::Binary {
                op,
                dest,
                left,
                right,
                ty,
            } => write!(f, "{dest} = {op} {left} {right}, {ty}"),
            Instruction::Unary {
                op,
                dest,
                operand,
                ty,
            } => write!(f, "{dest} = {op} {operand} {ty}"),
            Instruction::Call {
                dest,
                func,
                args,
                ty,
            } => {
                let dest_str = dest
                    .as_ref()
                    .map_or_else(|| "".to_string(), |d| format!("{d} = "));
                let args_str = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{dest_str}{func}({args_str}) : {ty}")
            }
            Instruction::GetElementPtr {
                dest,
                base,
                index,
                element_ty,
            } => write!(f, "{dest} = getelementptr {base}, {index} : {element_ty}",),
            Instruction::Cast {
                dest,
                value,
                from_ty,
                to_ty,
            } => write!(f, "{dest} = cast {value} from {from_ty} to {to_ty}"),
            Instruction::Phi { dest, ty, incoming } => {
                let incoming_str = incoming
                    .iter()
                    .map(|(val, block)| format!("[ {val}, {block} ]"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{dest} = phi {ty} [ {incoming_str} ]")
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
