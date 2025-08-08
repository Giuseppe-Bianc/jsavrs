// src/nir/mod.rs
mod basic_block;
mod function;
mod instruction;
mod terminator;
mod types;
mod value;
pub mod generator;
mod scope;
mod scope_manager;
mod access_control;

pub use basic_block::BasicBlock;
pub use function::{Cfg, Function, FunctionAttributes, IrParameter, ParamAttributes};
pub use instruction::{CastKind, Instruction, InstructionKind, IrBinaryOp, IrUnaryOp, VectorOp};
pub use terminator::{Terminator, TerminatorKind};
pub use types::{IrType, ResourceId, ScopeId};
pub use value::{IrConstantValue, IrLiteralValue, Value, ValueDebugInfo, ValueKind};
pub use access_control::{AccessController, AccessRules, Operation};
pub use scope::Scope;
pub use scope_manager::ScopeManager;
