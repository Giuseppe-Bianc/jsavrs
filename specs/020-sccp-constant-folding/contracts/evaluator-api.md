# Evaluator API Contract

**Module**: `src/ir/optimizer/constant_folding/evaluator.rs`  
**Version**: 1.0  
**Status**: Phase 1 Design

## Overview

This module provides constant expression evaluation for all IR types during SCCP analysis. It evaluates binary and unary operations on constant operands with correct type-specific semantics, overflow handling, and edge case management.

## Public Types

### `ConstantEvaluator`

```rust
pub struct ConstantEvaluator {
    diagnostics: DiagnosticEmitter,
}
```

**Purpose**: Evaluates constant expressions with proper error handling and diagnostics.

**Fields**:

- `diagnostics: DiagnosticEmitter` - For emitting warnings (division by zero, etc.)

---

## Public API

### Constructor

#### `new`

```rust
pub fn new() -> Self
```

**Description**: Create new constant evaluator with default diagnostic emitter.

**Returns**: `Self` - New evaluator instance

**Examples**:

```rust
let evaluator = ConstantEvaluator::new();
```

**Complexity**: O(1)

---

### Binary Operations

#### `evaluate_binary_op`

```rust
pub fn evaluate_binary_op(
    &mut self,
    op: BinaryOp,
    left: &ConstantValue,
    right: &ConstantValue,
) -> LatticeValue
```

**Description**: Evaluate binary operation on constant operands.

**Parameters**:

- `op: BinaryOp` - Operation to perform (Add, Sub, Mul, Div, Mod, And, Or, Xor, etc.)
- `left: &ConstantValue` - Left operand constant
- `right: &ConstantValue` - Right operand constant

**Returns**:

- `LatticeValue::Constant(result)` - If evaluation succeeds
- `LatticeValue::Top` - If overflow, type mismatch, or division by zero

**Side Effects**:

- Emits warning diagnostic on integer division by zero
- No warning on integer overflow (per spec)
- No warning on floating-point special values

**Type Requirements**:

- Left and right must have matching types (same variant)
- Returns `Top` on type mismatch

**Supported Operations by Type**:

| Type    | Add | Sub | Mul | Div | Mod | BitAnd | BitOr | BitXor | Shl | Shr | Eq | Lt | Gt |
| ------- | --- | --- | --- | --- | --- | ------ | ----- | ------ | --- | --- | -- | -- | -- |
| I8-I64  | ✓   | ✓   | ✓   | ✓   | ✓   | ✓      | ✓     | ✓      | ✓   | ✓   | ✓  | ✓  | ✓  |
| U8-U64  | ✓   | ✓   | ✓   | ✓   | ✓   | ✓      | ✓     | ✓      | ✓   | ✓   | ✓  | ✓  | ✓  |
| F32/F64 | ✓   | ✓   | ✓   | ✓   | -   | -      | -     | -      | -   | -   | ✓  | ✓  | ✓  |
| Bool    | -   | -   | -   | -   | -   | ✓      | ✓     | ✓      | -   | -   | ✓  | -  | -  |

**Examples**:

```rust
let mut evaluator = ConstantEvaluator::new();

// Integer addition
let result = evaluator.evaluate_binary_op(
    BinaryOp::Add,
    &ConstantValue::I32(10),
    &ConstantValue::I32(32),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::I32(42)));

// Integer overflow (returns Top, no warning)
let result = evaluator.evaluate_binary_op(
    BinaryOp::Add,
    &ConstantValue::I32(i32::MAX),
    &ConstantValue::I32(1),
);
assert_eq!(result, LatticeValue::Top);

// Division by zero (returns Top + warning)
let result = evaluator.evaluate_binary_op(
    BinaryOp::Div,
    &ConstantValue::I32(42),
    &ConstantValue::I32(0),
);
assert_eq!(result, LatticeValue::Top);
// Warning emitted: "Division by zero in constant expression"

// Floating-point division by zero (valid per IEEE 754, no warning)
let result = evaluator.evaluate_binary_op(
    BinaryOp::Div,
    &ConstantValue::F32(1.0),
    &ConstantValue::F32(0.0),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::F32(f32::INFINITY)));

// Type mismatch
let result = evaluator.evaluate_binary_op(
    BinaryOp::Add,
    &ConstantValue::I32(10),
    &ConstantValue::F32(3.14),
);
assert_eq!(result, LatticeValue::Top);
```

**Complexity**: O(1) for all operations

---

### Unary Operations

#### `evaluate_unary_op`

```rust
pub fn evaluate_unary_op(
    &self,
    op: UnaryOp,
    operand: &ConstantValue,
) -> LatticeValue
```

**Description**: Evaluate unary operation on constant operand.

**Parameters**:

- `op: UnaryOp` - Operation to perform (Neg, Not, BitwiseNot)
- `operand: &ConstantValue` - Operand constant

**Returns**:

- `LatticeValue::Constant(result)` - If evaluation succeeds
- `LatticeValue::Top` - If overflow or type mismatch

**Supported Operations by Type**:

| Type    | Neg | Not | BitwiseNot |
| ------- | --- | --- | ---------- |
| I8-I64  | ✓   | -   | ✓          |
| U8-U64  | -   | -   | ✓          |
| F32/F64 | ✓   | -   | -          |
| Bool    | -   | ✓   | -          |

**Examples**:

```rust
let evaluator = ConstantEvaluator::new();

// Integer negation
let result = evaluator.evaluate_unary_op(
    UnaryOp::Neg,
    &ConstantValue::I32(42),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::I32(-42)));

// Overflow on negation (i32::MIN cannot be negated)
let result = evaluator.evaluate_unary_op(
    UnaryOp::Neg,
    &ConstantValue::I32(i32::MIN),
);
assert_eq!(result, LatticeValue::Top);

// Boolean NOT
let result = evaluator.evaluate_unary_op(
    UnaryOp::Not,
    &ConstantValue::Bool(true),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)));

// Bitwise NOT
let result = evaluator.evaluate_unary_op(
    UnaryOp::BitwiseNot,
    &ConstantValue::U8(0b10101010),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::U8(0b01010101)));
```

**Complexity**: O(1) for all operations

---

## Edge Case Handling

### Integer Overflow

**Behavior**: Returns `LatticeValue::Top` without warning (per spec FR-018)

**Rationale**: Conservative and quiet for production compiler. Overflow behavior is well-defined (wrapping or trapping) but cannot be determined at compile time.

**Examples**:

```rust
// Addition overflow
Add(I8(127), I8(1)) → Top

// Subtraction overflow  
Sub(I8(-128), I8(1)) → Top

// Multiplication overflow
Mul(I32(100000), I32(100000)) → Top

// Negation overflow
Neg(I32(i32::MIN)) → Top
```

---

### Division and Modulo

**Integer Division by Zero**: Returns `Top` + emits warning

**Integer Modulo by Zero**: Returns `Top` + emits warning

**Floating-Point Division by Zero**: Returns `Infinity` or `-Infinity` (valid IEEE 754, no warning)

**Examples**:

```rust
// Integer division by zero (warning)
Div(I32(42), I32(0)) → Top + warning

// Floating-point division by zero (valid, no warning)
Div(F32(1.0), F32(0.0)) → Constant(F32(INFINITY))
Div(F32(-1.0), F32(0.0)) → Constant(F32(NEG_INFINITY))
```

---

### Floating-Point Special Values

**NaN Propagation**: NaN inputs produce NaN outputs (IEEE 754)

**Infinity Handling**: Infinity propagates through operations per IEEE 754

**Signed Zero**: Distinguishes -0.0 and +0.0

**Examples**:

```rust
// NaN propagation
Add(F32(NaN), F32(1.0)) → Constant(F32(NaN))
Mul(F64(INFINITY), F64(0.0)) → Constant(F64(NaN))

// Infinity operations
Add(F32(INFINITY), F32(1.0)) → Constant(F32(INFINITY))
Sub(F32(INFINITY), F32(INFINITY)) → Constant(F32(NaN))

// Signed zero
Mul(F32(-0.0), F32(1.0)) → Constant(F32(-0.0))
```

---

## Type-Specific Semantics

### Signed Integers (I8, I16, I32, I64)

- **Overflow**: Detected via `checked_*` methods, returns `Top`
- **Division**: Truncates toward zero, overflow on `MIN / -1`
- **Modulo**: Sign of result matches dividend
- **Shifts**: Undefined for shift amount ≥ bit width (returns `Top`)

### Unsigned Integers (U8, U16, U32, U64)

- **Overflow**: Detected via `checked_*` methods, returns `Top`
- **Division**: Truncates toward zero
- **Modulo**: Standard unsigned modulo
- **Shifts**: Undefined for shift amount ≥ bit width (returns `Top`)

### Floating-Point (F32, F64)

- **Arithmetic**: IEEE 754 semantics
- **Comparisons**: NaN always compares false (even to itself)
- **Rounding**: Default round-to-nearest-even

### Boolean

- **And/Or**: Short-circuit evaluation not applicable (both operands constant)
- **Xor**: Logical exclusive or

---

## Usage Examples

### Complete Evaluation Pipeline

```rust
use jsavrs::ir::optimizer::constant_folding::{ConstantEvaluator, ConstantValue, LatticeValue};

let mut evaluator = ConstantEvaluator::new();

// Evaluate: (10 + 32) * 2
let step1 = evaluator.evaluate_binary_op(
    BinaryOp::Add,
    &ConstantValue::I32(10),
    &ConstantValue::I32(32),
);

if let LatticeValue::Constant(result1) = step1 {
    let step2 = evaluator.evaluate_binary_op(
        BinaryOp::Mul,
        &result1,
        &ConstantValue::I32(2),
    );
    
    assert_eq!(step2, LatticeValue::Constant(ConstantValue::I32(84)));
}
```

### Comparison Operations

```rust
let mut evaluator = ConstantEvaluator::new();

// Integer comparison
let result = evaluator.evaluate_binary_op(
    BinaryOp::Lt,
    &ConstantValue::I32(10),
    &ConstantValue::I32(42),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)));

// Floating-point comparison with NaN
let result = evaluator.evaluate_binary_op(
    BinaryOp::Eq,
    &ConstantValue::F32(f32::NAN),
    &ConstantValue::F32(f32::NaN),
);
assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)));
```

---

## Testing Requirements

1. **Type Coverage**: All type/operation combinations
2. **Overflow Tests**: All overflow scenarios for each integer type
3. **Edge Cases**: Division by zero, NaN, Infinity, signed zero
4. **Diagnostic Tests**: Verify warnings emitted correctly
5. **Performance**: Benchmark evaluation speed

---

## Performance Characteristics

- **Evaluation Time**: O(1) for all operations
- **Memory**: Stack-allocated temporaries only, no heap allocations
- **Diagnostic Overhead**: Minimal (only on edge cases)

---

**API Contract Status**: ✅ Complete  
**Implementation Status**: Pending  
**Review Status**: Pending
