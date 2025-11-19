//! Lattice value representation for SCCP analysis
//!
//! This module implements the three-level lattice used by the Sparse Conditional
//! Constant Propagation algorithm:
//! - Top: Optimistically unknown (not yet determined)
//! - Constant(value): Proven compile-time constant
//! - Bottom: Pessimistically varying (runtime-dependent or multiple values)

use crate::ir::IrLiteralValue;

/// Represents the constant state of an SSA value in the three-level lattice
/// used by the SCCP algorithm.
///
/// The lattice forms a partial order: Top ⊑ Constant ⊑ Bottom
/// Values can only descend (Top → Constant → Bottom), ensuring termination.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    /// Optimistically unknown - value has not yet been determined
    /// Indicates the analysis has not yet reached this definition
    /// or the value depends on unanalyzed control flow paths
    Top,

    /// Proven constant value - the value is guaranteed to equal this literal
    /// on all executable control-flow paths reaching this point
    Constant(IrLiteralValue),

    /// Pessimistically varying - the value may be different on different paths
    /// or is unknown at compile time (e.g., function parameters, memory loads)
    Bottom,
}

impl LatticeValue {
    /// Computes the meet (greatest lower bound) of two lattice values.
    ///
    /// Meet operation properties:
    /// - Commutative: meet(a, b) = meet(b, a)
    /// - Associative: meet(meet(a, b), c) = meet(a, meet(b, c))
    /// - Idempotent: meet(a, a) = a
    /// - Top is identity: meet(Top, x) = x
    /// - Bottom is absorbing: meet(Bottom, x) = Bottom
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::ir::optimizer::constant_folding::LatticeValue;
    /// use jsavrs::ir::IrLiteralValue;
    ///
    /// let top = LatticeValue::Top;
    /// let const5 = LatticeValue::Constant(IrLiteralValue::I32(5));
    /// let const6 = LatticeValue::Constant(IrLiteralValue::I32(6));
    /// let bottom = LatticeValue::Bottom;
    ///
    /// assert_eq!(top.meet(&const5), const5);
    /// assert_eq!(const5.meet(&const5), const5);
    /// assert_eq!(const5.meet(&const6), bottom);
    /// assert_eq!(bottom.meet(&const5), bottom);
    /// ```
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // Top is top element: meet with anything yields that thing
            (LatticeValue::Top, x) | (x, LatticeValue::Top) => x.clone(),

            // Bottom is bottom element: meet with anything yields Bottom
            (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,

            // Same constants: meet yields that constant
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) if c1 == c2 => LatticeValue::Constant(c1.clone()),

            // Different constants: meet yields Bottom (varying)
            (LatticeValue::Constant(_), LatticeValue::Constant(_)) => LatticeValue::Bottom,
        }
    }

    /// Returns true if this lattice value is more precise than the other
    /// (i.e., this ⊑ other in the lattice partial order).
    ///
    /// Partial order:
    /// - Top ⊑ anything
    /// - Constant(c) ⊑ Bottom
    /// - Constant(c1) and Constant(c2) are incomparable if c1 ≠ c2
    pub fn is_more_precise_than(&self, other: &Self) -> bool {
        match (self, other) {
            (LatticeValue::Top, _) => true,
            (_, LatticeValue::Bottom) => true,
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) => c1 == c2,
            _ => false,
        }
    }

    /// Returns true if this lattice value represents a proven constant
    pub fn is_constant(&self) -> bool {
        matches!(self, LatticeValue::Constant(_))
    }

    /// Extracts the constant value if this is Constant, otherwise None
    pub fn as_constant(&self) -> Option<&IrLiteralValue> {
        match self {
            LatticeValue::Constant(lit) => Some(lit),
            _ => None,
        }
    }
}

impl PartialOrd for LatticeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            // Reflexive: x ⊑ x
            (a, b) if a == b => Some(Ordering::Equal),

            // Top ⊑ anything
            (LatticeValue::Top, _) => Some(Ordering::Less),
            (_, LatticeValue::Top) => Some(Ordering::Greater),

            // anything ⊑ Bottom
            (_, LatticeValue::Bottom) => Some(Ordering::Less),
            (LatticeValue::Bottom, _) => Some(Ordering::Greater),

            // Different constants are incomparable
            (LatticeValue::Constant(_), LatticeValue::Constant(_)) => None,
        }
    }
}
