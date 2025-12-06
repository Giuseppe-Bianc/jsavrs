//! Lattice value representation for SCCP algorithm
//!
//! Implements the three-level lattice system (Bottom, Constant, Top) for tracking
//! compile-time value states during sparse conditional constant propagation.

use crate::ir::IrType;

/// Represents the compile-time state of an SSA value in the lattice
///
/// The lattice ordering is: Bottom < Constant < Top
/// - Bottom (⊥): Unreachable or uninitialized value
/// - Constant: Proven compile-time constant value
/// - Top (⊤): Overdefined runtime-varying value
#[derive(Debug, Clone, PartialEq)]
pub enum LatticeValue {
    /// Bottom (⊥): Unreachable or uninitialized value
    Bottom,
    /// Constant: Proven compile-time constant value
    Constant(ConstantValue),
    /// Top (⊤): Overdefined runtime-varying value
    Top,
}

/// Constant value representation for all IR types
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
}

impl LatticeValue {
    /// Computes the lattice meet operation for phi nodes
    ///
    /// Meet semantics (per FR-003):
    /// - meet(Constant(a), Constant(a)) = Constant(a)
    /// - meet(Constant(a), Constant(b)) where a ≠ b = Top
    /// - meet(_, Top) = Top
    /// - meet(Top, _) = Top
    /// - meet(Bottom, x) = x
    /// - meet(x, Bottom) = x
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // meet(Bottom, x) = x
            (LatticeValue::Bottom, x) | (x, LatticeValue::Bottom) => x.clone(),

            // meet(_, Top) = Top
            (LatticeValue::Top, _) | (_, LatticeValue::Top) => LatticeValue::Top,

            // meet(Constant(a), Constant(b))
            (LatticeValue::Constant(a), LatticeValue::Constant(b)) => {
                if a == b {
                    LatticeValue::Constant(a.clone())
                } else {
                    // Different constants → Top (overdefined)
                    LatticeValue::Top
                }
            }
        }
    }

    /// Returns true if this value is a constant
    pub fn is_constant(&self) -> bool {
        matches!(self, LatticeValue::Constant(_))
    }

    /// Returns the constant value if this is a Constant, otherwise None
    pub fn as_constant(&self) -> Option<&ConstantValue> {
        match self {
            LatticeValue::Constant(val) => Some(val),
            _ => None,
        }
    }

    /// Returns true if this value is Bottom (unreachable/uninitialized)
    pub fn is_bottom(&self) -> bool {
        matches!(self, LatticeValue::Bottom)
    }

    /// Returns true if this value is Top (overdefined/runtime-varying)
    pub fn is_top(&self) -> bool {
        matches!(self, LatticeValue::Top)
    }
}

impl ConstantValue {
    /// Gets the IR type of this constant value
    pub fn get_type(&self) -> IrType {
        match self {
            ConstantValue::I8(_) => IrType::I8,
            ConstantValue::I16(_) => IrType::I16,
            ConstantValue::I32(_) => IrType::I32,
            ConstantValue::I64(_) => IrType::I64,
            ConstantValue::U8(_) => IrType::U8,
            ConstantValue::U16(_) => IrType::U16,
            ConstantValue::U32(_) => IrType::U32,
            ConstantValue::U64(_) => IrType::U64,
            ConstantValue::F32(_) => IrType::F32,
            ConstantValue::F64(_) => IrType::F64,
            ConstantValue::Bool(_) => IrType::Bool,
            ConstantValue::Char(_) => IrType::Char,
        }
    }

    /// Checks if two constant values have matching types
    pub fn types_match(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    /// Attempts to extract a boolean value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ConstantValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to extract an i32 value
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            ConstantValue::I32(v) => Some(*v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_meet_bottom() {
        let bottom = LatticeValue::Bottom;
        let constant = LatticeValue::Constant(ConstantValue::I32(42));
        let top = LatticeValue::Top;

        assert_eq!(bottom.meet(&constant), constant);
        assert_eq!(constant.meet(&bottom), constant);
        assert_eq!(bottom.meet(&top), top);
        assert_eq!(top.meet(&bottom), top);
    }

    #[test]
    fn test_lattice_meet_top() {
        let constant = LatticeValue::Constant(ConstantValue::I32(42));
        let top = LatticeValue::Top;

        assert_eq!(top.meet(&constant), top);
        assert_eq!(constant.meet(&top), top);
        assert_eq!(top.meet(&top), top);
    }

    #[test]
    fn test_lattice_meet_constants() {
        let c1 = LatticeValue::Constant(ConstantValue::I32(42));
        let c2 = LatticeValue::Constant(ConstantValue::I32(42));
        let c3 = LatticeValue::Constant(ConstantValue::I32(99));

        // Same constants
        assert_eq!(c1.meet(&c2), c1);

        // Different constants → Top (overdefined)
        assert_eq!(c1.meet(&c3), LatticeValue::Top);
    }
}
