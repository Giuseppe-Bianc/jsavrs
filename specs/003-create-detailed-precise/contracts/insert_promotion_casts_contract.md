# Test Contract: insert_promotion_casts Tests

**Date**: October 6, 2025  
**Module**: TypePromotionEngine::insert_promotion_casts  
**Purpose**: Validate correct insertion of cast instructions into IR

---

## Contract Overview

The `insert_promotion_casts` method must insert appropriate cast instructions into the IR based on the `PromotionResult` from `analyze_binary_promotion`, returning properly typed `Value` instances representing the promoted operands.

---

## Input Contract

### Required Inputs:
- `generator: &mut NIrGenerator` - IR generator for creating instructions and temporaries
- `func: &mut Function` - Function context for instruction insertion
- `left_value: Value` - Left operand value to potentially cast
- `right_value: Value` - Right operand value to potentially cast
- `promotion_result: &PromotionResult` - Result from analyze_binary_promotion
- `span: SourceSpan` - Source location for error reporting

### Preconditions:
- `promotion_result` must be valid result from `analyze_binary_promotion`
- `left_value.ty` must match `promotion_result.left_cast.from_type` if cast required
- `right_value.ty` must match `promotion_result.right_cast.from_type` if cast required
- `generator` and `func` must be valid, initialized IR contexts

---

## Output Contract

### Return Type: `(Value, Value)`

#### Tuple Fields:
1. **First Value (promoted left operand)**
   - MUST have type equal to `promotion_result.result_type`
   - If `promotion_result.left_cast` is `Some`, MUST be new temporary value
   - If `promotion_result.left_cast` is `None`, MUST be original `left_value`
   - MUST preserve source span information

2. **Second Value (promoted right operand)**
   - MUST have type equal to `promotion_result.result_type`
   - If `promotion_result.right_cast` is `Some`, MUST be new temporary value
   - If `promotion_result.right_cast` is `None`, MUST be original `right_value`
   - MUST preserve source span information

---

## Side Effects Contract

### IR Modifications:
1. **Cast Instruction Insertion** (when casts required):
   - MUST insert `Instruction::Cast` for left operand if `left_cast.is_some()`
   - MUST insert `Instruction::Cast` for right operand if `right_cast.is_some()`
   - Each cast instruction MUST have correct `CastKind`
   - Each cast instruction MUST have correct source and target types
   - Each cast instruction MUST have result value assigned

2. **Temporary Creation**:
   - MUST create new temporary via `generator.new_temp()` for each cast
   - Each temporary MUST have correct type matching cast target type
   - Each temporary MUST have debug info and span attached

3. **Instruction Order**:
   - Left operand cast MUST be inserted before right operand cast (if both present)
   - Casts MUST be inserted at current insertion point in generator

---

## Test Scenarios

### 1. No Casts Required (12 tests - identity promotions)
**Given**: `PromotionResult` with `left_cast = None` and `right_cast = None`  
**When**: `insert_promotion_casts` is called  
**Then**:
- Returned left value MUST equal input `left_value`
- Returned right value MUST equal input `right_value`
- NO cast instructions inserted into IR
- Generator temporary counter MUST NOT increment

**Example**:
```rust
let promotion_result = PromotionResult {
    result_type: IrType::I32,
    left_cast: None,
    right_cast: None,
    warnings: vec![],
    is_sound: true,
};
let (new_left, new_right) = engine.insert_promotion_casts(
    &mut generator, &mut func, left_value, right_value, &promotion_result, span
);
assert_eq!(new_left, left_value);  // Same value instance
assert_eq!(new_right, right_value);  // Same value instance
```

---

### 2. Left Operand Cast Only (30+ tests)
**Given**: `PromotionResult` with `left_cast = Some(_)` and `right_cast = None`  
**When**: `insert_promotion_casts` is called  
**Then**:
- Returned left value MUST be new temporary with `result_type`
- Returned right value MUST equal input `right_value`
- ONE cast instruction MUST be inserted for left operand
- Cast instruction MUST have correct `CastKind` from `left_cast.cast_kind`
- Cast instruction result MUST equal returned left value

**Example**:
```rust
let promotion_result = PromotionResult {
    result_type: IrType::I32,
    left_cast: Some(TypePromotion {
        from_type: IrType::I8,
        to_type: IrType::I32,
        cast_kind: CastKind::IntSignExtend,
        ...
    }),
    right_cast: None,
    ...
};
let (new_left, new_right) = engine.insert_promotion_casts(...);

// Assertions
assert_eq!(new_left.ty, IrType::I32);  // Promoted type
assert_ne!(new_left, left_value);  // New value created
assert_eq!(new_right, right_value);  // Original value unchanged

// IR validation
let last_inst = generator.get_last_instruction();
assert!(matches!(last_inst.kind, InstructionKind::Cast { kind: CastKind::IntSignExtend, ... }));
assert_eq!(last_inst.result.unwrap(), new_left);
```

---

### 3. Right Operand Cast Only (30+ tests)
**Given**: `PromotionResult` with `left_cast = None` and `right_cast = Some(_)`  
**When**: `insert_promotion_casts` is called  
**Then**:
- Returned left value MUST equal input `left_value`
- Returned right value MUST be new temporary with `result_type`
- ONE cast instruction MUST be inserted for right operand
- Cast instruction MUST have correct `CastKind` from `right_cast.cast_kind`
- Cast instruction result MUST equal returned right value

**Example**:
```rust
let promotion_result = PromotionResult {
    result_type: IrType::F32,
    left_cast: None,
    right_cast: Some(TypePromotion {
        from_type: IrType::I32,
        to_type: IrType::F32,
        cast_kind: CastKind::IntToFloat,
        ...
    }),
    ...
};
let (new_left, new_right) = engine.insert_promotion_casts(...);

assert_eq!(new_left, left_value);  // Original value unchanged
assert_eq!(new_right.ty, IrType::F32);  // Promoted type
assert_ne!(new_right, right_value);  // New value created
```

---

### 4. Both Operands Cast (20+ tests)
**Given**: `PromotionResult` with both `left_cast = Some(_)` and `right_cast = Some(_)`  
**When**: `insert_promotion_casts` is called  
**Then**:
- Returned left value MUST be new temporary with `result_type`
- Returned right value MUST be new temporary with `result_type`
- TWO cast instructions MUST be inserted (left first, then right)
- Each cast instruction MUST have correct `CastKind`
- Both cast instruction results MUST match returned values

**Example**:
```rust
let promotion_result = PromotionResult {
    result_type: IrType::I64,
    left_cast: Some(TypePromotion {
        from_type: IrType::I32,
        to_type: IrType::I64,
        cast_kind: CastKind::IntSignExtend,
        ...
    }),
    right_cast: Some(TypePromotion {
        from_type: IrType::U32,
        to_type: IrType::I64,
        cast_kind: CastKind::IntSignExtend,  // or appropriate cast
        ...
    }),
    ...
};
let (new_left, new_right) = engine.insert_promotion_casts(...);

// Both operands promoted
assert_eq!(new_left.ty, IrType::I64);
assert_eq!(new_right.ty, IrType::I64);
assert_ne!(new_left, left_value);
assert_ne!(new_right, right_value);

// Verify two cast instructions inserted
let instructions = generator.get_recent_instructions(2);
assert_eq!(instructions.len(), 2);
assert!(matches!(instructions[0].kind, InstructionKind::Cast { ... }));  // Left cast
assert!(matches!(instructions[1].kind, InstructionKind::Cast { ... }));  // Right cast
```

---

### 5. CastKind Validation (50+ tests covering all cast types)
**Given**: Various type pair promotions requiring different `CastKind` values  
**When**: `insert_promotion_casts` is called  
**Then**:
- Cast instruction MUST use correct `CastKind`:
  - `IntSignExtend`: Signed integer widening (I8→I16, I8→I32, etc.)
  - `IntZeroExtend`: Unsigned integer widening (U8→U16, U8→U32, etc.)
  - `IntTruncate`: Integer narrowing (I32→I16, U64→U32, etc.)
  - `IntToFloat`: Integer to float conversion (I32→F32, I64→F64, etc.)
  - `FloatToInt`: Float to integer conversion (F32→I32, F64→I64, etc.)
  - `FloatExtend`: F32→F64
  - `FloatTruncate`: F64→F32
  - `Bitcast`: Same type (should not insert cast, but if present must be bitcast)

**Example**:
```rust
// I8 to I32 (signed widening)
let promotion_result = PromotionResult {
    result_type: IrType::I32,
    left_cast: Some(TypePromotion {
        from_type: IrType::I8,
        to_type: IrType::I32,
        cast_kind: CastKind::IntSignExtend,
        ...
    }),
    ...
};
let (new_left, _) = engine.insert_promotion_casts(...);
let cast_inst = generator.get_last_instruction();
assert!(matches!(cast_inst.kind, InstructionKind::Cast {
    kind: CastKind::IntSignExtend,
    from_ty: IrType::I8,
    to_ty: IrType::I32,
    ...
}));
```

---

### 6. SourceSpan Preservation (10 tests)
**Given**: Input values and promotion_result with specific `SourceSpan`  
**When**: `insert_promotion_casts` is called  
**Then**:
- Cast instructions MUST preserve original `span` parameter
- Returned value debug info MUST include `span`
- All IR elements MUST be traceable to original source location

**Example**:
```rust
let span = SourceSpan::new(10, 20, "test.vn");
let (new_left, _) = engine.insert_promotion_casts(..., span.clone());
let cast_inst = generator.get_last_instruction();
assert_eq!(cast_inst.span, span);
assert_eq!(new_left.debug_info.as_ref().unwrap().span, span);
```

---

## IR Verification Contract

### Generated Cast Instruction Structure:
```rust
Instruction {
    kind: InstructionKind::Cast {
        kind: <appropriate_cast_kind>,
        value: <original_value>,
        from_ty: <original_type>,
        to_ty: <target_type>,
    },
    result: Some(<new_temporary_value>),
    span: <preserved_source_span>,
    ...
}
```

### Verification Steps:
1. Extract cast instructions from generator
2. Verify `CastKind` matches `TypePromotion.cast_kind`
3. Verify `from_ty` matches original value type
4. Verify `to_ty` matches promotion result type
5. Verify result value has correct type and is new temporary

---

## Edge Cases

### 1. Invalid PromotionResult (not tested - assumed valid input)
- Mismatched types between promotion_result and input values
- Invalid cast kinds

### 2. Generator State Edge Cases (integration tests)
- Generator with existing instructions
- Function with multiple basic blocks (casts inserted in current block)

---

## Performance Contract

- MUST complete in <50ms for single cast insertion
- MUST complete in <100ms for bilateral cast insertion
- MUST NOT allocate excessive memory beyond necessary temporaries

---

## Test Implementation Requirements

1. **Test Naming Convention**:
   ```
   test_insert_promotion_casts_<scenario>_<cast_types>_<verification_aspect>
   ```
   Examples:
   - `test_insert_promotion_casts_no_casts_identity_i32`
   - `test_insert_promotion_casts_left_only_i8_to_i32_sign_extend`
   - `test_insert_promotion_casts_both_i32_u32_to_i64_bilateral`

2. **Assertion Strategy**:
   - Explicit assertions for returned value types and equality
   - IR verification for instruction presence and structure
   - Snapshot assertions for complex IR state (optional)

3. **Test Organization**:
   - Group by cast scenario (no casts, left only, right only, both)
   - Subgroup by CastKind validation

4. **Test Fixtures**:
   - Helper function to create test NIrGenerator and Function
   - Helper function to create test Value instances
   - Helper function to verify cast instruction structure

---

**Contract Status**: ✅ COMPLETE - Ready for test implementation
