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
mod module;
//mod validator;

pub use access_control::{AccessController, AccessRules, Operation};
pub use basic_block::BasicBlock;
pub use function::{Cfg, Function, FunctionAttributes, IrParameter, ParamAttributes};
pub use instruction::{CastKind, Instruction, InstructionKind, IrBinaryOp, IrUnaryOp, VectorOp};
pub use module::{DataLayout, Module, TargetTriple};
pub use scope::Scope;
pub use scope_manager::ScopeManager;
pub use terminator::{Terminator, TerminatorKind};
pub use types::{IrType, ResourceId, ScopeId};
pub use value::{IrConstantValue, IrLiteralValue, Value, ValueDebugInfo, ValueKind};
