//! Constant Folding Optimizer with Sparse Conditional Constant Propagation (SCCP)
//!
//! This module implements the Wegman-Zadeck SCCP algorithm for constant propagation
//! and dead code identification in SSA-form intermediate representation.
//!
//! # Algorithm Overview
//!
//! The SCCP algorithm operates on a three-level lattice system to track compile-time value states:
//!
//! ```text
//!           Top (⊤)
//!          /  |  \
//!    Const  Const  ...
//!          \  |  /
//!        Bottom (⊥)
//! ```
//!
//! - **Bottom (⊥)**: Value is unreachable or uninitialized
//! - **Constant**: Value is proven to be a compile-time constant
//! - **Top (⊤)**: Value is overdefined (runtime-varying)
//!
//! # Invariants
//!
//! 1. **Monotonicity**: Lattice values can only move upward (Bottom → Constant → Top), never downward
//! 2. **SSA Preservation**: All transformations maintain SSA form - never modify LHS of assignments
//! 3. **Dominance**: Definitions dominate uses before and after transformation
//! 4. **Convergence**: Fixed-point iteration must terminate within `max_iterations`
//!
//! # Phases
//!
//! 1. **Initialization**: Set parameters to Top, locals to Bottom, mark entry edge executable
//! 2. **Propagation**: Process CFG and SSA worklists until fixed-point
//! 3. **Rewriting**: Transform IR based on discovered constants and unreachable code
//!
//! # Example
//!
//! ```rust,ignore
//! use jsavrs::ir::optimizer::constant_folding::*;
//!
//! let mut optimizer = optimizer::ConstantFoldingOptimizer::default();
//! optimizer.optimize_function(&mut function)?;
//! println!("Optimized {} constants", optimizer.stats().constants_propagated);
//! ```

pub mod evaluator;
pub mod lattice;
pub mod optimizer;
pub mod propagator;
pub mod rewriter;

pub use lattice::{ConstantValue, LatticeValue};
pub use optimizer::{ConstantFoldingOptimizer, OptimizationStats, SCCPConfig};
pub use propagator::SCCPropagator;
pub use rewriter::IRRewriter;
