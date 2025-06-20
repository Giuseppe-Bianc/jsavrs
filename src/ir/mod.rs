// src/ir/mod.rs
mod basic_block;
mod function;
mod instruction;
mod terminator;
mod types;
mod value;
pub mod generator;

pub use basic_block::BasicBlock;
pub use function::Function;
pub use instruction::{Instruction, IrBinaryOp, IrUnaryOp};
pub use terminator::Terminator;
pub use types::IrType;
pub use value::{ImmediateValue, Value, ValueKind};
