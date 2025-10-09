mod access_control;
pub mod basic_block;
pub mod cfg;
pub mod dominance;
pub mod function;
pub mod generator;
pub mod instruction;
pub mod module;
pub mod scope;
pub mod scope_manager;
pub mod ssa;
pub mod terminator;
pub mod type_promotion;
pub mod type_promotion_engine;
pub mod types;
pub mod value;

pub use access_control::{AccessController, AccessRules, Operation};
pub use basic_block::BasicBlock;
pub use cfg::ControlFlowGraph;
pub use dominance::DominanceInfo;
pub use function::{Function, FunctionAttributes, IrParameter, ParamAttributes};
pub use instruction::{CastKind, Instruction, InstructionKind, IrBinaryOp, IrUnaryOp, VectorOp};
pub use module::{DataLayout, Module, TargetTriple};
pub use scope::Scope;
pub use scope_manager::ScopeManager;
pub use ssa::SsaTransformer;
pub use terminator::{Terminator, TerminatorKind};
pub use type_promotion::{
    FloatSpecialValueType, OverflowBehavior, PrecisionLossEstimate, PromotionMatrix, PromotionResult, PromotionRule,
    PromotionWarning, TypeGroup, TypePromotion,
};
pub use type_promotion_engine::TypePromotionEngine;
pub use types::{IrType, ResourceId, ScopeId};
pub use value::{IrConstantValue, IrLiteralValue, Value, ValueDebugInfo, ValueKind};
