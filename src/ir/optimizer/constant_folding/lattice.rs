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
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            // meet(Bottom, x) = x
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),

            // meet(_, Top) = Top
            (Self::Top, _) | (_, Self::Top) => Self::Top,

            // meet(Constant(a), Constant(b))
            (Self::Constant(a), Self::Constant(b)) => {
                if a == b {
                    Self::Constant(a.clone())
                } else {
                    // Different constants → Top (overdefined)
                    Self::Top
                }
            }
        }
    }

    /// Returns true if this value is a constant
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Returns the constant value if this is a Constant, otherwise None
    #[must_use]
    pub const fn as_constant(&self) -> Option<&ConstantValue> {
        match self {
            Self::Constant(val) => Some(val),
            _ => None,
        }
    }

    /// Returns true if this value is Bottom (unreachable/uninitialized)
    #[must_use]
    pub const fn is_bottom(&self) -> bool {
        matches!(self, Self::Bottom)
    }

    /// Returns true if this value is Top (overdefined/runtime-varying)
    #[must_use]
    pub const fn is_top(&self) -> bool {
        matches!(self, Self::Top)
    }
}

impl ConstantValue {
    /// Gets the IR type of this constant value
    #[must_use]
    pub const fn get_type(&self) -> IrType {
        match self {
            Self::I8(_) => IrType::I8,
            Self::I16(_) => IrType::I16,
            Self::I32(_) => IrType::I32,
            Self::I64(_) => IrType::I64,
            Self::U8(_) => IrType::U8,
            Self::U16(_) => IrType::U16,
            Self::U32(_) => IrType::U32,
            Self::U64(_) => IrType::U64,
            Self::F32(_) => IrType::F32,
            Self::F64(_) => IrType::F64,
            Self::Bool(_) => IrType::Bool,
            Self::Char(_) => IrType::Char,
        }
    }

    /// Checks if two constant values have matching types
    #[must_use]
    pub fn types_match(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    /// Attempts to extract a boolean value
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to extract an i32 value
    #[must_use]
    pub const fn as_i32(&self) -> Option<i32> {
        match self {
            Self::I32(v) => Some(*v),
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
