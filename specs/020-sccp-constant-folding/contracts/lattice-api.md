# Lattice API Contract

**Module**: `src/ir/optimizer/constant_folding/lattice.rs`  
**Version**: 1.0  
**Status**: Phase 1 Design

## Overview

This module provides lattice-based abstract interpretation types for SCCP analysis. It defines the three-level lattice (Bottom, Constant, Top) and type-safe constant value representations with correct meet/join operations.

## Public Types

### `LatticeValue`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LatticeValue {
    Bottom,
    Constant(ConstantValue),
    Top,
}
```

**Purpose**: Represents the abstract interpretation state of an SSA value.

**Variants**:
- `Bottom`: Unreachable or uninitialized value (⊥)
- `Constant(ConstantValue)`: Proven compile-time constant
- `Top`: Overdefined runtime-varying value (⊤)

**Lattice Ordering**: `Bottom ≤ Constant ≤ Top`

---

### `ConstantValue`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    I8(i8),   I16(i16),   I32(i32),   I64(i64),
    U8(u8),   U16(u16),   U32(u32),   U64(u64),
    F32(f32), F64(f64),
    Bool(bool),
    Char(char),
}
```

**Purpose**: Type-safe representation of compile-time constant values.

**Variants**: One for each IR primitive type.

**Equality**: Structural equality except floating-point uses bit-level comparison.

## Public API

### LatticeValue Methods

#### `meet`
```rust
pub fn meet(&self, other: &Self) -> Self
```

**Description**: Compute lattice meet (greatest lower bound).

**Parameters**:
- `other: &Self` - Other lattice value to meet with

**Returns**: `Self` - Result of meet operation

**Semantics**:
- `Bottom ⊓ x = x` (Bottom is identity)
- `Top ⊓ x = Top` (Top absorbs)
- `Constant(v1) ⊓ Constant(v2) = Constant(v1)` if `v1 == v2`, else `Top`

**Examples**:
```rust
let bottom = LatticeValue::Bottom;
let const_42 = LatticeValue::Constant(ConstantValue::I32(42));
let const_10 = LatticeValue::Constant(ConstantValue::I32(10));
let top = LatticeValue::Top;

assert_eq!(bottom.meet(&const_42), const_42);
assert_eq!(const_42.meet(&const_42), const_42);
assert_eq!(const_42.meet(&const_10), top);
assert_eq!(top.meet(&const_42), top);
```

**Complexity**: O(1)

---

#### `is_constant`
```rust
pub fn is_constant(&self) -> bool
```

**Description**: Check if value is a constant.

**Returns**: `true` if `Constant` variant, `false` otherwise

**Examples**:
```rust
let const_val = LatticeValue::Constant(ConstantValue::Bool(true));
assert!(const_val.is_constant());
assert!(!LatticeValue::Bottom.is_constant());
```

**Complexity**: O(1)

---

#### `as_constant`
```rust
pub fn as_constant(&self) -> Option<&ConstantValue>
```

**Description**: Extract constant value if present.

**Returns**: `Some(&ConstantValue)` if Constant, `None` otherwise

**Examples**:
```rust
let const_val = LatticeValue::Constant(ConstantValue::I32(42));
if let Some(ConstantValue::I32(v)) = const_val.as_constant() {
    assert_eq!(*v, 42);
}
```

**Complexity**: O(1)

---

#### `is_bottom`
```rust
pub fn is_bottom(&self) -> bool
```

**Description**: Check if value is Bottom (unreachable).

**Returns**: `true` if `Bottom`, `false` otherwise

**Complexity**: O(1)

---

#### `is_top`
```rust
pub fn is_top(&self) -> bool
```

**Description**: Check if value is Top (overdefined).

**Returns**: `true` if `Top`, `false` otherwise

**Complexity**: O(1)

---

### ConstantValue Methods

#### `get_type`
```rust
pub fn get_type(&self) -> IRType
```

**Description**: Get the IR type of this constant value.

**Returns**: `IRType` - Type enum matching the constant variant

**Examples**:
```rust
let const_val = ConstantValue::I32(42);
assert_eq!(const_val.get_type(), IRType::I32);
```

**Complexity**: O(1)

---

#### `types_match`
```rust
pub fn types_match(&self, other: &Self) -> bool
```

**Description**: Check if two constant values have the same type.

**Parameters**:
- `other: &Self` - Other constant to compare types with

**Returns**: `true` if same variant (type), `false` otherwise

**Examples**:
```rust
let a = ConstantValue::I32(42);
let b = ConstantValue::I32(10);
let c = ConstantValue::F32(3.14);

assert!(a.types_match(&b));
assert!(!a.types_match(&c));
```

**Complexity**: O(1)

---

#### `as_bool`
```rust
pub fn as_bool(&self) -> Option<bool>
```

**Description**: Convert to boolean if possible (for branch conditions).

**Returns**: `Some(bool)` if `Bool` variant, `None` otherwise

**Examples**:
```rust
let const_true = ConstantValue::Bool(true);
assert_eq!(const_true.as_bool(), Some(true));

let const_int = ConstantValue::I32(42);
assert_eq!(const_int.as_bool(), None);
```

**Complexity**: O(1)

---

## Trait Implementations

### LatticeValue Traits

- `Debug`: Debug formatting
- `Clone`: Deep clone
- `PartialEq, Eq`: Structural equality
- `Hash`: Hash for use in HashMaps/HashSets

### ConstantValue Traits

- `Debug`: Debug formatting
- `Clone`: Deep clone
- `PartialEq`: Structural equality (bit-level for floats)

**Note**: `ConstantValue` does NOT implement `Eq` or `Hash` due to floating-point NaN semantics.

---

## Usage Examples

### Basic Lattice Operations
```rust
use jsavrs::ir::optimizer::constant_folding::{LatticeValue, ConstantValue};

// Create lattice values
let bottom = LatticeValue::Bottom;
let const_42 = LatticeValue::Constant(ConstantValue::I32(42));
let top = LatticeValue::Top;

// Meet operations
let result1 = bottom.meet(&const_42); // = Constant(I32(42))
let result2 = const_42.meet(&const_42); // = Constant(I32(42))
let result3 = const_42.meet(&top); // = Top

// Check types
assert!(result1.is_constant());
assert!(result2.is_constant());
assert!(result3.is_top());
```

### Phi Node Evaluation (Conceptual)
```rust
fn evaluate_phi(incoming_values: &[(BlockId, LatticeValue)]) -> LatticeValue {
    let mut result = LatticeValue::Bottom;
    
    for (_, value) in incoming_values {
        result = result.meet(value);
    }
    
    result
}

// Example: phi with two incoming constants
let phi_result = evaluate_phi(&[
    (block1, LatticeValue::Constant(ConstantValue::I32(42))),
    (block2, LatticeValue::Constant(ConstantValue::I32(42))),
]);
assert_eq!(phi_result, LatticeValue::Constant(ConstantValue::I32(42)));

// Example: phi with conflicting constants
let phi_result = evaluate_phi(&[
    (block1, LatticeValue::Constant(ConstantValue::I32(42))),
    (block2, LatticeValue::Constant(ConstantValue::I32(10))),
]);
assert_eq!(phi_result, LatticeValue::Top);
```

---

## Invariants

1. **Monotonicity**: Lattice values never decrease in ordering
2. **Type Safety**: Constant values maintain correct Rust type invariants
3. **Meet Idempotency**: `x.meet(&x) == x`
4. **Meet Commutativity**: `x.meet(&y) == y.meet(&x)`
5. **Meet Associativity**: `(x.meet(&y)).meet(&z) == x.meet(&(y.meet(&z)))`

---

## Error Handling

This module does not produce errors - all operations are infallible. Type mismatches in constant values should be detected by callers before invoking operations.

---

## Performance Characteristics

- **Meet Operation**: O(1) time, O(1) space
- **Type Queries**: O(1) time, O(1) space
- **Memory Overhead**: 
  - `LatticeValue`: 16 bytes (enum discriminant + largest variant)
  - `ConstantValue`: 8 bytes (enum discriminant + largest primitive)

---

## Testing Requirements

1. **Unit Tests**: All meet operation combinations
2. **Property Tests**: Meet operation properties (commutativity, associativity, idempotency)
3. **Type Safety Tests**: Ensure Rust type invariants maintained
4. **Floating-Point Tests**: NaN handling, -0.0 vs +0.0, Infinity

---

**API Contract Status**: ✅ Complete  
**Implementation Status**: Pending  
**Review Status**: Pending
