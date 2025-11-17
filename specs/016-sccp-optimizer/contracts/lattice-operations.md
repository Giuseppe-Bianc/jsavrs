# Lattice Operations Contract

**Feature**: Sparse Conditional Constant Propagation Optimizer  
**Branch**: 016-sccp-optimizer  
**Date**: 2025-11-17

## Overview

This document specifies the API contract for the `LatticeValue` type and its operations, which form the mathematical foundation of the SCCP algorithm.

## LatticeValue Type Definition

```rust
/// Three-level lattice for constant propagation analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    /// Optimistically unknown (top element ⊤)
    Top,
    
    /// Proven constant value
    Constant(IrLiteralValue),
    
    /// Pessimistically varying (bottom element ⊥)
    Bottom,
}
```

## Lattice Operations

### Method: `meet`

```rust
impl LatticeValue {
    pub fn meet(&self, other: &Self) -> Self;
}
```

**Purpose**: Computes the greatest lower bound (meet) of two lattice values

**Mathematical Properties**:

| Property | Definition | Verification |
|----------|------------|--------------|
| Commutative | `meet(a, b) = meet(b, a)` | Swapping operands produces same result |
| Associative | `meet(meet(a, b), c) = meet(a, meet(b, c))` | Grouping doesn't matter |
| Idempotent | `meet(a, a) = a` | Meeting with self is identity |
| Top is identity | `meet(Top, x) = x` | Top doesn't affect result |
| Bottom is absorbing | `meet(Bottom, x) = Bottom` | Bottom propagates |

**Truth Table**:

| `self` | `other` | `Result` | Rationale |
|--------|---------|----------|-----------|
| `Top` | `Top` | `Top` | Both unknown, result unknown |
| `Top` | `Constant(c)` | `Constant(c)` | Unknown meets constant → constant |
| `Top` | `Bottom` | `Bottom` | Unknown meets varying → varying |
| `Constant(c)` | `Constant(c)` | `Constant(c)` | Same constant → same constant |
| `Constant(c₁)` | `Constant(c₂)` where c₁≠c₂ | `Bottom` | Different constants → varying |
| `Constant(c)` | `Bottom` | `Bottom` | Constant meets varying → varying |
| `Bottom` | `Bottom` | `Bottom` | Both varying → varying |

**Type Safety**:
- `IrLiteralValue` comparison uses structural equality
- Different types (e.g., `I32(5)` vs `I64(5)`) are considered different constants → `Bottom`
- Same type and value (e.g., `I32(5)` and `I32(5)`) → `Constant(I32(5))`

**Example Usage**:

```rust
use jsavrs::ir::optimizer::constant_folding::LatticeValue;
use jsavrs::ir::IrLiteralValue;

// Unknown meets constant → constant
assert_eq!(
    LatticeValue::Top.meet(&LatticeValue::Constant(IrLiteralValue::I32(42))),
    LatticeValue::Constant(IrLiteralValue::I32(42))
);

// Same constants → same constant
assert_eq!(
    LatticeValue::Constant(IrLiteralValue::Bool(true))
        .meet(&LatticeValue::Constant(IrLiteralValue::Bool(true))),
    LatticeValue::Constant(IrLiteralValue::Bool(true))
);

// Different constants → varying
assert_eq!(
    LatticeValue::Constant(IrLiteralValue::I32(5))
        .meet(&LatticeValue::Constant(IrLiteralValue::I32(10))),
    LatticeValue::Bottom
);

// Bottom absorbs
assert_eq!(
    LatticeValue::Bottom.meet(&LatticeValue::Constant(IrLiteralValue::I32(42))),
    LatticeValue::Bottom
);
```

**Performance**: O(1) - simple enum matching

**Preconditions**: None (all inputs valid)

**Postconditions**: Result is valid `LatticeValue`, result ⊑ self ∧ result ⊑ other (greater lower bound)

---

### Method: `is_more_precise_than`

```rust
impl LatticeValue {
    pub fn is_more_precise_than(&self, other: &Self) -> bool;
}
```

**Purpose**: Tests if this value is more precise (lower) than another in the lattice partial order

**Partial Order Definition**:

```text
         Top (least precise)
        /   |   \
Constant(c₁) Constant(c₂) ... Constant(cₙ)
        \   |   /
       Bottom (most precise)
```

**Truth Table**:

| `self` | `other` | `Result` | Rationale |
|--------|---------|----------|-----------|
| `Top` | `Top` | `true` | Equal precision (reflexive) |
| `Top` | `Constant(c)` | `true` | Top less precise than Constant |
| `Top` | `Bottom` | `true` | Top less precise than Bottom |
| `Constant(c)` | `Top` | `false` | Constant more precise than Top |
| `Constant(c)` | `Constant(c)` | `true` | Equal precision (reflexive) |
| `Constant(c₁)` | `Constant(c₂)` where c₁≠c₂ | `false` | Incomparable |
| `Constant(c)` | `Bottom` | `true` | Constant less precise than Bottom |
| `Bottom` | `Top` | `false` | Bottom more precise than Top |
| `Bottom` | `Constant(c)` | `false` | Bottom more precise than Constant |
| `Bottom` | `Bottom` | `true` | Equal precision (reflexive) |

**Example Usage**:

```rust
// Top is less precise than everything
assert!(LatticeValue::Top.is_more_precise_than(&LatticeValue::Top));
assert!(LatticeValue::Top.is_more_precise_than(&LatticeValue::Constant(IrLiteralValue::I32(5))));
assert!(LatticeValue::Top.is_more_precise_than(&LatticeValue::Bottom));

// Constant is more precise than Top, less precise than Bottom
assert!(!LatticeValue::Constant(IrLiteralValue::I32(5)).is_more_precise_than(&LatticeValue::Top));
assert!(LatticeValue::Constant(IrLiteralValue::I32(5)).is_more_precise_than(&LatticeValue::Bottom));

// Different constants are incomparable
assert!(!LatticeValue::Constant(IrLiteralValue::I32(5))
    .is_more_precise_than(&LatticeValue::Constant(IrLiteralValue::I32(10))));

// Bottom is most precise (least precise than nothing except itself)
assert!(!LatticeValue::Bottom.is_more_precise_than(&LatticeValue::Top));
assert!(!LatticeValue::Bottom.is_more_precise_than(&LatticeValue::Constant(IrLiteralValue::I32(5))));
assert!(LatticeValue::Bottom.is_more_precise_than(&LatticeValue::Bottom));
```

**Performance**: O(1) - simple enum matching

**Preconditions**: None

**Postconditions**: Returns boolean indicating partial order relation

---

### Method: `is_constant`

```rust
impl LatticeValue {
    pub fn is_constant(&self) -> bool;
}
```

**Purpose**: Checks if this lattice value represents a proven constant

**Returns**: `true` if `self` is `Constant(_)`, `false` otherwise

**Example Usage**:

```rust
assert!(!LatticeValue::Top.is_constant());
assert!(LatticeValue::Constant(IrLiteralValue::I32(42)).is_constant());
assert!(!LatticeValue::Bottom.is_constant());
```

**Performance**: O(1) - single enum variant check

---

### Method: `as_constant`

```rust
impl LatticeValue {
    pub fn as_constant(&self) -> Option<&IrLiteralValue>;
}
```

**Purpose**: Extracts the constant value if this is `Constant`, otherwise `None`

**Returns**:
- `Some(&literal)` if `self` is `Constant(literal)`
- `None` if `self` is `Top` or `Bottom`

**Example Usage**:

```rust
let constant = LatticeValue::Constant(IrLiteralValue::I32(42));
assert_eq!(constant.as_constant(), Some(&IrLiteralValue::I32(42)));

assert_eq!(LatticeValue::Top.as_constant(), None);
assert_eq!(LatticeValue::Bottom.as_constant(), None);
```

**Performance**: O(1) - single enum match

**Use Case**: Safe extraction of constant value for IR rewriting without panicking

---

## Monotonicity Invariant

**Critical Property**: Lattice values must only descend (Top → Constant → Bottom), never ascend

**Allowed Transitions**:
```text
Top → Top (no change, allowed)
Top → Constant(c) (refinement, allowed)
Top → Bottom (pessimistic refinement, allowed)
Constant(c) → Constant(c) (no change, allowed)
Constant(c) → Bottom (pessimistic refinement, allowed)
Bottom → Bottom (no change, allowed)
```

**Forbidden Transitions** (violate monotonicity):
```text
Constant(c) → Top (upward movement, INVALID)
Bottom → Top (upward movement, INVALID)
Bottom → Constant(c) (upward movement, INVALID)
Constant(c₁) → Constant(c₂) where c₁ ≠ c₂ (lateral movement, INVALID)
```

**Enforcement**:
- Before updating lattice: `new_value.is_more_precise_than(&old_value)` must be true
- If invariant violated, return `SCCPError::LatticeInvariantViolation`
- Post-analysis validation checks no upward movements occurred

**Example Validation**:

```rust
fn update_lattice_value(
    lattice: &mut HashMap<Value, LatticeValue>,
    value: Value,
    new_lattice_value: LatticeValue
) -> Result<bool, SCCPError> {
    let old_value = lattice.get(&value).unwrap_or(&LatticeValue::Top);
    
    // Check monotonicity
    if !new_lattice_value.is_more_precise_than(old_value) {
        return Err(SCCPError::LatticeInvariantViolation(
            format!("{:?}", value),
            old_value.clone(),
            new_lattice_value
        ));
    }
    
    // Check if value actually changed
    let changed = old_value != &new_lattice_value;
    if changed {
        lattice.insert(value, new_lattice_value);
    }
    
    Ok(changed)
}
```

## Type-Specific Constant Evaluation

### Integer Constants

**Supported Operations**:
- Arithmetic: `Add`, `Sub`, `Mul`, `Div`, `Mod`
- Bitwise: `And`, `Or`, `Xor`, `Shl`, `Shr`
- Comparison: `Eq`, `Ne`, `Lt`, `Le`, `Gt`, `Ge`

**Overflow Handling**:
```rust
match left.checked_add(right) {
    Some(result) => Constant(IrLiteralValue::I32(result)),
    None => Bottom  // Overflow → varying
}
```

**Division by Zero**:
```rust
if right == 0 {
    Bottom  // Division by zero → varying
} else {
    // Proceed with division
}
```

### Floating-Point Constants

**Special Value Handling**:
```rust
let result = left + right;
if result.is_nan() || result.is_infinite() {
    Bottom  // Conservative: NaN/Infinity → varying
} else {
    Constant(IrLiteralValue::F64(result))
}
```

**Rationale**: IEEE 754 semantics are complex (NaN ≠ NaN, Infinity arithmetic), conservative approach avoids incorrect optimizations

### Boolean Constants

**Logical Operations**:
```rust
match (left, right) {
    (true, true) => Constant(IrLiteralValue::Bool(true)),
    (true, false) | (false, true) => Constant(IrLiteralValue::Bool(false)),
    (false, false) => Constant(IrLiteralValue::Bool(false)),
}
```

**Short-Circuit Evaluation**: Not applicable in SSA form (all operands already evaluated)

### Character Constants

**Unicode Validation**:
```rust
if !value.is_valid_unicode_scalar() {
    Bottom  // Invalid character → varying
} else {
    Constant(IrLiteralValue::Char(value))
}
```

### String Constants

**Policy**: Always `Bottom` in initial implementation

**Rationale**: String operations (concatenation, slicing) are complex and low-priority for constant propagation

## Summary Table

| Method | Purpose | Complexity | Returns |
|--------|---------|------------|---------|
| `meet(other)` | Greatest lower bound | O(1) | `LatticeValue` |
| `is_more_precise_than(other)` | Partial order check | O(1) | `bool` |
| `is_constant()` | Check if proven constant | O(1) | `bool` |
| `as_constant()` | Extract constant value | O(1) | `Option<&IrLiteralValue>` |

---

**End of Lattice Operations Contract**
