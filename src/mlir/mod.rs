//! MLIR module: organizes compiler IR layers (HIR -> MIR -> LIR).
//! Invariant: dependencies flow downward only (HIR -> MIR -> LIR).
pub mod hir;
pub mod lir;
pub mod mir;
