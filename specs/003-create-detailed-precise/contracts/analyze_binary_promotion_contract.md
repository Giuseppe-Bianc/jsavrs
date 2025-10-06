# Test Contract: analyze_binary_promotion Tests

**Date**: October 6, 2025  
**Module**: TypePromotionEngine::analyze_binary_promotion  
**Purpose**: Validate correct type promotion analysis for binary operations

---

## Contract Overview

The `analyze_binary_promotion` method must correctly analyze binary operations involving two operands of potentially different types, determine the appropriate result type based on the promotion matrix, identify required casts for each operand, generate appropriate warnings, and determine if the promotion is sound.

---

## Input Contract

### Required Inputs:
- `left_type: &IrType` - Type of left operand (must be one of 12 supported variants)
- `right_type: &IrType` - Type of right operand (must be one of 12 supported variants)
- `operation: IrBinaryOp` - Binary operation being performed (must be one of 18 supported operations)
- `span: SourceSpan` - Source location for error reporting (can be default for tests)

### Preconditions:
- TypePromotionEngine must be initialized with valid PromotionMatrix
- Both `left_type` and `right_type` must be valid IrType variants
- `operation` must be a valid IrBinaryOp variant

---

## Output Contract

### Return Type: `PromotionResult`

#### Fields:
1. **result_type: IrType**
   - MUST be the target type for the operation result
   - MUST follow promotion hierarchy (F64 > F32 > wider integer > narrower integer)
   - For identical input types, MUST equal input type

2. **left_cast: Option<TypePromotion>**
   - MUST be `Some(_)` if left operand requires type conversion
   - MUST be `None` if left operand type equals result_type
   - If `Some`, MUST contain correct `from_type`, `to_type`, and `cast_kind`

3. **right_cast: Option<TypePromotion>**
   - MUST be `Some(_)` if right operand requires type conversion
   - MUST be `None` if right operand type equals result_type
   - If `Some`, MUST contain correct `from_type`, `to_type`, and `cast_kind`

4. **warnings: Vec<PromotionWarning>**
   - MUST contain `PrecisionLoss` warning when conversion may lose precision
   - MUST contain `PotentialOverflow` warning when conversion may cause overflow
   - MUST contain `SignednessChange` warning when mixing signed/unsigned of same width
   - MAY be empty for safe, lossless promotions
   - Each warning MUST contain accurate type information and metadata

5. **is_sound: bool**
   - MUST be `true` when promotion is mathematically sound without warnings
   - MUST be `false` when warnings are present (simplified implementation)
   - Reflects overall safety of the promotion

---

## Test Scenarios

### 1. Identity Promotions (12 tests)
**Given**: Operands of identical types (I8, I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Char)  
**When**: `analyze_binary_promotion` is called with same type for both operands  
**Then**:
- `result_type` MUST equal input type
- `left_cast` MUST be `None`
- `right_cast` MUST be `None`
- `warnings` MUST be empty
- `is_sound` MUST be `true`

**Example**:
```rust
analyze_binary_promotion(&IrType::I32, &IrType::I32, IrBinaryOp::Add, span)
  => PromotionResult {
       result_type: IrType::I32,
       left_cast: None,
       right_cast: None,
       warnings: vec![],
       is_sound: true,
     }
```

---

### 2. Widening Promotions (24 tests)
**Given**: Operands where one is narrower than the other within same signedness  
**When**: `analyze_binary_promotion` is called (e.g., I8 + I32)  
**Then**:
- `result_type` MUST be the wider type
- Cast MUST be required for narrower operand
- `warnings` MUST be empty (widening is safe)
- `is_sound` MUST be `true`

**Example**:
```rust
analyze_binary_promotion(&IrType::I8, &IrType::I32, IrBinaryOp::Add, span)
  => PromotionResult {
       result_type: IrType::I32,
       left_cast: Some(TypePromotion { from_type: I8, to_type: I32, cast_kind: IntSignExtend, ... }),
       right_cast: None,
       warnings: vec![],
       is_sound: true,
     }
```

---

### 3. Narrowing Promotions (24 tests)
**Given**: Operands where promotion to narrower type would occur (atypical scenario)  
**When**: `analyze_binary_promotion` is called with wider type promoting to narrower  
**Then**:
- `result_type` MUST still promote to wider type (promotion matrix determines this)
- Cast MUST be required appropriately
- `warnings` MAY contain `PrecisionLoss` if narrowing occurs
- `is_sound` MUST reflect warning presence

**Example**:
```rust
// Note: Promotion matrix determines actual behavior; narrowing may not occur
// This tests that engine follows matrix rules correctly
```

---

### 4. Cross-Signedness Promotions (8 tests: 4 pairs × 2 directions)
**Given**: Operands of same width but different signedness (I8+U8, I16+U16, I32+U32, I64+U64)  
**When**: `analyze_binary_promotion` is called  
**Then**:
- `result_type` MUST be determined by promotion matrix (typically promotes to next larger signed)
- Cast MUST be required for one or both operands
- `warnings` MUST contain `SignednessChange` warning
- `is_sound` MUST be `false` (warning present)

**Example**:
```rust
analyze_binary_promotion(&IrType::I32, &IrType::U32, IrBinaryOp::Add, span)
  => PromotionResult {
       result_type: IrType::I64,  // Promoted to next larger signed
       left_cast: Some(TypePromotion { from_type: I32, to_type: I64, ... }),
       right_cast: Some(TypePromotion { from_type: U32, to_type: I64, ... }),
       warnings: vec![PromotionWarning::SignednessChange { ... }],
       is_sound: false,
     }
```

---

### 5. Integer-to-Float Promotions (16 tests)
**Given**: One integer operand, one float operand  
**When**: `analyze_binary_promotion` is called  
**Then**:
- `result_type` MUST be the float type (floats take precedence)
- Cast MUST be required for integer operand
- `warnings` MAY contain `PrecisionLoss` for large integers to F32
- `is_sound` MUST reflect warning presence

**Example**:
```rust
analyze_binary_promotion(&IrType::I32, &IrType::F32, IrBinaryOp::Multiply, span)
  => PromotionResult {
       result_type: IrType::F32,
       left_cast: Some(TypePromotion { from_type: I32, to_type: F32, cast_kind: IntToFloat, ... }),
       right_cast: None,
       warnings: vec![],  // i32 fits in f32 mantissa
       is_sound: true,
     }
```

---

### 6. Float-to-Integer Promotions (16 tests)
**Given**: Float operand attempting to promote with integer operand  
**When**: `analyze_binary_promotion` is called  
**Then**:
- `result_type` MUST be the float type (floats take precedence, not converted to integer)
- Cast MUST be required for integer operand
- If float→int conversion attempted, `warnings` MUST contain `PrecisionLoss` (fractional part lost) and MAY contain `PotentialOverflow`

**Example**:
```rust
analyze_binary_promotion(&IrType::F64, &IrType::I64, IrBinaryOp::Add, span)
  => PromotionResult {
       result_type: IrType::F64,  // Float takes precedence
       left_cast: None,
       right_cast: Some(TypePromotion { from_type: I64, to_type: F64, cast_kind: IntToFloat, ... }),
       warnings: vec![],  // i64 fits in f64
       is_sound: true,
     }
```

---

### 7. Float Widening/Narrowing (2 tests)
**Given**: F32 and F64 operands  
**When**: `analyze_binary_promotion` is called  
**Then**:
- For F32→F64: Widening, no precision loss
- For F64→F32: Narrowing, `PrecisionLoss` warning with `SignificantDigits` loss estimate

**Example (widening)**:
```rust
analyze_binary_promotion(&IrType::F32, &IrType::F64, IrBinaryOp::Add, span)
  => PromotionResult {
       result_type: IrType::F64,
       left_cast: Some(TypePromotion { from_type: F32, to_type: F64, cast_kind: FloatExtend, ... }),
       right_cast: None,
       warnings: vec![],
       is_sound: true,
     }
```

---

### 8. Operation-Specific Behaviors (54 tests: 18 ops × 3 type scenarios)
**Given**: Various operations (arithmetic, comparison, logical, bitwise) with different type combinations  
**When**: `analyze_binary_promotion` is called with specific operation  
**Then**:
- **Arithmetic operations**: Standard promotion rules apply
- **Comparison operations**: Return Bool result type (special case)
- **Logical operations**: Typically used with Bool types
- **Bitwise operations**: Mixed signedness may require special handling

**Example (comparison)**:
```rust
analyze_binary_promotion(&IrType::I32, &IrType::F32, IrBinaryOp::Less, span)
  => PromotionResult {
       result_type: IrType::Bool,  // Comparison returns boolean
       left_cast: Some(TypePromotion { from_type: I32, to_type: F32, ... }),
       right_cast: None,
       warnings: vec![],
       is_sound: true,
     }
```

---

## Edge Cases

### 1. Type Boundary Cases
- Smallest to largest (I8→I64, U8→U64): No precision loss, cast required
- Same-width cross-signedness with max values: `SignednessChange` warning

### 2. Float Special Values (tested at integration level)
- NaN, Infinity, -Infinity handling: May generate `FloatSpecialValues` warning

### 3. Promotion Matrix Edge Cases
- `compute_common_type()` returns `None`: Fallback to left_type
- `get_promotion_rule()` returns `None`: No cast generated

---

## Error Cases

### Invalid Inputs (not tested - assumed preconditions met):
- Invalid IrType variants
- Invalid IrBinaryOp variants
- Uninitialized engine

---

## Performance Contract

- MUST complete in <100ms for single analysis
- MUST be thread-safe (read-only operation)
- MUST NOT allocate excessive memory (O(1) promotions)

---

## Test Implementation Requirements

1. **Test Naming Convention**:
   ```
   test_analyze_binary_promotion_<type_scenario>_<operation>_<expected_result>
   ```
   Example: `test_analyze_binary_promotion_i32_f32_add_promotes_to_f32`

2. **Assertion Strategy**:
   - Explicit assertions for `result_type`, cast presence, `is_sound`
   - Snapshot assertions for full `PromotionResult` structure
   - Explicit assertions for warning types and counts

3. **Test Organization**:
   - Group by type scenario (identity, widening, narrowing, cross-signedness, etc.)
   - Subgroup by operation category where relevant

---

**Contract Status**: ✅ COMPLETE - Ready for test implementation
