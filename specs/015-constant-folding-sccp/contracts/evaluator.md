# API Contract: Constant Evaluator

**Module**: `src/ir/optimizer/constant_folding/evaluator.rs`  
**Purpose**: Define the contract for evaluating constant expressions in IR instructions

---

## Interface Definition

### Primary Function

```rust
/// Attempts to fold an IR instruction into a constant value.
/// 
/// # Arguments
/// * `instruction` - The IR instruction to evaluate
/// * `constant_map` - Map of SSA values to their known constant values
/// 
/// # Returns
/// * `Some(IrLiteralValue)` - If the instruction can be folded to a constant
/// * `None` - If the instruction is not foldable (non-constant operands, side effects, etc.)
/// 
/// # Behavior
/// - Evaluates arithmetic, logical, comparison, bitwise, unary, and cast operations
/// - Respects type semantics (wrapping for integers, IEEE 754 for floats)
/// - Never introduces undefined behavior (conservative on division by zero, etc.)
/// - Deterministic output for all supported operations
pub fn fold_instruction(
    instruction: &Instruction,
    constant_map: &HashMap<ValueId, IrLiteralValue>
) -> Option<IrLiteralValue>;
```

---

## Supported Operations

### Arithmetic Operations

#### Integer Arithmetic

```rust
/// Folds integer addition with wrapping semantics.
/// 
/// # Examples
/// * `add i32 5, 3` → `8`
/// * `add i8 127, 1` → `-128` (wrapping overflow)
/// 
/// # Type Support
/// All integer types: i8, i16, i32, i64, u8, u16, u32, u64
fn fold_add(a: IrLiteralValue, b: IrLiteralValue, ty: &IrType) -> Option<IrLiteralValue>;

/// Folds integer subtraction with wrapping semantics.
fn fold_sub(a: IrLiteralValue, b: IrLiteralValue, ty: &IrType) -> Option<IrLiteralValue>;

/// Folds integer multiplication with wrapping semantics.
fn fold_mul(a: IrLiteralValue, b: IrLiteralValue, ty: &IrType) -> Option<IrLiteralValue>;

/// Folds integer division.
/// 
/// # Safety
/// Returns None if divisor is zero (avoids introducing UB).
/// 
/// # Examples
/// * `div i32 10, 2` → `5`
/// * `div i32 10, 0` → `None` (preserve original instruction)
fn fold_div(a: IrLiteralValue, b: IrLiteralValue, ty: &IrType) -> Option<IrLiteralValue>;

/// Folds integer modulo.
/// 
/// # Safety
/// Returns None if divisor is zero.
fn fold_mod(a: IrLiteralValue, b: IrLiteralValue, ty: &IrType) -> Option<IrLiteralValue>;
```

**Contract**:
- **Preconditions**: `a` and `b` are integer constants, `ty` is integer type
- **Postconditions**: Returns integer constant with wrapping overflow, or None if division by zero
- **Complexity**: O(1)

---

#### Floating-Point Arithmetic

```rust
/// Folds floating-point addition following IEEE 754.
/// 
/// # Examples
/// * `fadd f64 1.5, 2.5` → `4.0`
/// * `fadd f64 inf, -inf` → `NaN`
/// * `fadd f64 NaN, 1.0` → `NaN`
fn fold_fadd(a: f64, b: f64) -> f64;

/// Folds floating-point subtraction following IEEE 754.
fn fold_fsub(a: f64, b: f64) -> f64;

/// Folds floating-point multiplication following IEEE 754.
fn fold_fmul(a: f64, b: f64) -> f64;

/// Folds floating-point division following IEEE 754.
/// 
/// # Examples
/// * `fdiv f64 6.0, 2.0` → `3.0`
/// * `fdiv f64 1.0, 0.0` → `inf`
/// * `fdiv f64 0.0, 0.0` → `NaN` (deterministic indeterminate form)
fn fold_fdiv(a: f64, b: f64) -> f64;
```

**Contract**:
- **Preconditions**: `a` and `b` are valid f32/f64 values (including NaN, ±inf)
- **Postconditions**: Returns IEEE 754 compliant result (may be NaN or ±inf)
- **Determinism**: Always produces canonical NaN (0x7ff8000000000000 for f64)
- **Complexity**: O(1)

---

### Comparison Operations

```rust
/// Folds integer equality comparison.
/// 
/// # Examples
/// * `icmp eq 5, 5` → `true`
/// * `icmp eq 5, 6` → `false`
fn fold_icmp_eq(a: i64, b: i64) -> bool;

/// Folds integer less-than comparison.
fn fold_icmp_lt(a: i64, b: i64, signed: bool) -> bool;

/// Folds integer less-than-or-equal comparison.
fn fold_icmp_le(a: i64, b: i64, signed: bool) -> bool;

/// Folds integer greater-than comparison.
fn fold_icmp_gt(a: i64, b: i64, signed: bool) -> bool;

/// Folds integer greater-than-or-equal comparison.
fn fold_icmp_ge(a: i64, b: i64, signed: bool) -> bool;

/// Folds floating-point comparison following IEEE 754 ordering.
/// 
/// # NaN Handling
/// * `fcmp eq NaN, x` → `false` (NaN != NaN)
/// * `fcmp lt NaN, x` → `false` (NaN is unordered)
fn fold_fcmp(a: f64, b: f64, predicate: FPPredicate) -> bool;
```

**Contract**:
- **Preconditions**: `a` and `b` are constants of comparable type
- **Postconditions**: Returns boolean constant
- **NaN Semantics**: Follows IEEE 754 (NaN comparisons return false except != which returns true)
- **Complexity**: O(1)

---

### Logical Operations

```rust
/// Folds logical AND.
/// 
/// # Examples
/// * `and true, false` → `false`
/// * `and true, true` → `true`
fn fold_and(a: bool, b: bool) -> bool;

/// Folds logical OR.
fn fold_or(a: bool, b: bool) -> bool;

/// Folds logical XOR.
fn fold_xor(a: bool, b: bool) -> bool;

/// Folds logical NOT.
fn fold_not(a: bool) -> bool;
```

**Contract**:
- **Preconditions**: `a` and `b` are boolean constants
- **Postconditions**: Returns boolean constant
- **Complexity**: O(1)

---

### Bitwise Operations

```rust
/// Folds bitwise AND.
/// 
/// # Examples
/// * `bitand i32 0xFF, 0x0F` → `0x0F`
fn fold_bitand(a: i64, b: i64, ty: &IrType) -> i64;

/// Folds bitwise OR.
fn fold_bitor(a: i64, b: i64, ty: &IrType) -> i64;

/// Folds bitwise XOR.
fn fold_bitxor(a: i64, b: i64, ty: &IrType) -> i64;

/// Folds left shift.
/// 
/// # Safety
/// If shift amount >= bit width, returns None (avoid UB).
/// 
/// # Examples
/// * `shl i32 1, 3` → `8`
/// * `shl i32 1, 32` → `None` (shift amount >= width)
fn fold_shl(a: i64, shift: u32, ty: &IrType) -> Option<i64>;

/// Folds right shift (logical or arithmetic based on type signedness).
fn fold_shr(a: i64, shift: u32, ty: &IrType, signed: bool) -> Option<i64>;
```

**Contract**:
- **Preconditions**: `a`, `b`, `shift` are integer constants
- **Postconditions**: Returns integer constant or None if shift amount invalid
- **Overflow**: Shift amount >= bit width returns None to avoid UB
- **Complexity**: O(1)

---

### Unary Operations

```rust
/// Folds integer negation.
/// 
/// # Examples
/// * `neg i32 5` → `-5`
/// * `neg i8 -128` → `-128` (wrapping: -(-128) wraps to -128)
fn fold_neg(a: i64, ty: &IrType) -> i64;

/// Folds floating-point negation.
/// 
/// # Examples
/// * `fneg f64 1.5` → `-1.5`
/// * `fneg f64 -0.0` → `0.0` (sign bit flip)
fn fold_fneg(a: f64) -> f64;

/// Folds bitwise NOT.
fn fold_bitnot(a: i64, ty: &IrType) -> i64;
```

**Contract**:
- **Preconditions**: `a` is constant of appropriate type
- **Postconditions**: Returns constant of same type
- **Complexity**: O(1)

---

### Type Conversion (Casts)

```rust
/// Folds integer extension (zero-extend or sign-extend).
/// 
/// # Examples
/// * `sext i8 -1 to i32` → `-1` (sign extend)
/// * `zext i8 255 to i32` → `255` (zero extend)
fn fold_int_extend(value: i64, from_ty: &IrType, to_ty: &IrType, signed: bool) -> i64;

/// Folds integer truncation.
/// 
/// # Examples
/// * `trunc i32 300 to i8` → `44` (300 & 0xFF)
fn fold_int_trunc(value: i64, from_ty: &IrType, to_ty: &IrType) -> i64;

/// Folds integer to floating-point conversion.
fn fold_int_to_float(value: i64, from_ty: &IrType, signed: bool) -> f64;

/// Folds floating-point to integer conversion (truncating toward zero).
/// 
/// # Safety
/// Returns None if conversion would overflow target type.
fn fold_float_to_int(value: f64, to_ty: &IrType, signed: bool) -> Option<i64>;

/// Folds floating-point extension (f32 to f64).
fn fold_float_extend(value: f32) -> f64;

/// Folds floating-point truncation (f64 to f32).
fn fold_float_trunc(value: f64) -> f32;

/// Folds pointer to integer cast.
/// 
/// # Safety
/// Always returns None (cannot fold pointer addresses at compile time).
fn fold_ptr_to_int(ptr: PointerValue, ty: &IrType) -> Option<i64>;

/// Folds integer to pointer cast.
/// 
/// # Safety
/// Always returns None (cannot create compile-time pointer values).
fn fold_int_to_ptr(value: i64, ty: &IrType) -> Option<PointerValue>;
```

**Contract**:
- **Preconditions**: `value` is constant, `from_ty` and `to_ty` are compatible types
- **Postconditions**: Returns converted constant or None if conversion invalid/unsafe
- **Safety**: Pointer casts always return None (no compile-time pointer manipulation)
- **Overflow**: Float-to-int returns None on overflow rather than wrapping
- **Complexity**: O(1)

---

## Error Handling

### Conservative Folding Rules

The evaluator follows conservative rules to avoid introducing undefined behavior:

1. **Division by Zero**: Returns `None` for `div` and `mod` with zero divisor
2. **Shift Overflow**: Returns `None` if shift amount >= bit width
3. **Float-to-Int Overflow**: Returns `None` if float value exceeds target integer range
4. **Pointer Casts**: Always returns `None` (cannot compute compile-time addresses)
5. **Unknown Operations**: Returns `None` for unsupported instructions

### Validation

```rust
/// Validates that operands are constants before folding.
fn get_constant_operands(
    instruction: &Instruction,
    constant_map: &HashMap<ValueId, IrLiteralValue>
) -> Option<Vec<IrLiteralValue>>;
```

**Behavior**:
- Looks up all operands in `constant_map`
- Returns `None` if any operand is not constant
- Returns `Some(vec![...])` with operand values if all are constant

---

## Usage Examples

### Example 1: Basic Arithmetic Folding

```rust
use std::collections::HashMap;

let mut constant_map = HashMap::new();
constant_map.insert(value_1, IrLiteralValue::Int(5));
constant_map.insert(value_2, IrLiteralValue::Int(3));

let instruction = Instruction::Add {
    result: value_3,
    lhs: value_1,
    rhs: value_2,
    ty: IrType::I32,
};

let result = fold_instruction(&instruction, &constant_map);
assert_eq!(result, Some(IrLiteralValue::Int(8)));
```

### Example 2: Division by Zero (Conservative)

```rust
constant_map.insert(value_1, IrLiteralValue::Int(10));
constant_map.insert(value_2, IrLiteralValue::Int(0));

let instruction = Instruction::Div {
    result: value_3,
    lhs: value_1,
    rhs: value_2,
    ty: IrType::I32,
};

let result = fold_instruction(&instruction, &constant_map);
assert_eq!(result, None); // Preserve original instruction
```

### Example 3: Floating-Point NaN Propagation

```rust
constant_map.insert(value_1, IrLiteralValue::Float(0.0));
constant_map.insert(value_2, IrLiteralValue::Float(0.0));

let instruction = Instruction::FDiv {
    result: value_3,
    lhs: value_1,
    rhs: value_2,
    ty: IrType::F64,
};

let result = fold_instruction(&instruction, &constant_map);
assert!(matches!(result, Some(IrLiteralValue::Float(f)) if f.is_nan()));
```

### Example 4: Type Conversion

```rust
constant_map.insert(value_1, IrLiteralValue::Int(-1));

let instruction = Instruction::Sext {
    result: value_2,
    value: value_1,
    from_ty: IrType::I8,
    to_ty: IrType::I32,
};

let result = fold_instruction(&instruction, &constant_map);
assert_eq!(result, Some(IrLiteralValue::Int(-1))); // Sign extended
```

---

## Performance Characteristics

| Operation Category | Complexity | Notes |
|--------------------|-----------|-------|
| Constant lookup | O(1) avg | HashMap access |
| Arithmetic operation | O(1) | Direct computation |
| Comparison operation | O(1) | Direct computation |
| Bitwise operation | O(1) | Direct computation |
| Cast operation | O(1) | Type conversion |
| Overall `fold_instruction()` | O(1) | Single instruction evaluation |

**Memory**: No allocations except for return value (constant literal).

---

## Testing Contract

### Required Unit Tests

1. **Arithmetic Tests**: All operations with edge cases (overflow, underflow)
2. **Comparison Tests**: All predicates including NaN handling
3. **Logical Tests**: Boolean operations with all input combinations
4. **Bitwise Tests**: All operations including shifts with boundary values
5. **Cast Tests**: All conversion types with overflow cases
6. **Conservative Tests**: Division by zero, shift overflow, pointer casts

### Property-Based Tests

- **Commutativity**: `fold_add(a, b) == fold_add(b, a)`
- **Associativity**: `fold_add(fold_add(a, b), c) == fold_add(a, fold_add(b, c))`
- **Identity**: `fold_add(a, 0) == a`
- **Determinism**: Repeated calls with same inputs produce identical results

---

## Summary

| Aspect | Specification |
|--------|--------------|
| Primary Function | `fold_instruction()` |
| Supported Operations | Arithmetic, Comparison, Logical, Bitwise, Unary, Casts |
| Integer Overflow | Wrapping (two's complement) |
| Float Semantics | IEEE 754 with deterministic NaN |
| Error Strategy | Conservative: return None rather than introduce UB |
| Performance | O(1) per instruction |
| Thread Safety | Pure function (no shared mutable state) |
