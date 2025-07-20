// src/nir/mod.rs
mod basic_block;
mod function;
mod instruction;
mod terminator;
mod types;
mod value;
pub mod generator;


pub use basic_block::BasicBlock;
pub use function::{Function, FunctionAttributes, };
pub use instruction::{CastKind, InstructionKind, IrBinaryOp, IrUnaryOp, VectorOp};
pub use terminator::{Terminator, TerminatorKind};
pub use types::IrType;
pub use value::{IrConstantValue, IrLiteralValue, Value, ValueKind};
