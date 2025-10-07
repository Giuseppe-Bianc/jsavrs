# Test Contract Summary: Remaining Test Groups

**Date**: October 6, 2025  
**Purpose**: Summarize contracts for warning generation, edge cases, corner cases, integration, unit, and concurrent test groups

---

## 3. Warning Generation Tests Contract

### Purpose
Validate accurate generation of `PromotionWarning` variants with correct metadata and message content.

### Warning Types to Test:

#### 3.1 PrecisionLoss Warnings (15-20 tests)
**Trigger Conditions**:
- Float to integer conversion (fractional part lost)
- F64 to F32 conversion (significant digits lost)
- Large integer to F32 (mantissa overflow)
- Integer narrowing (I64→I32, value range reduction)

**Contract**:
- Warning MUST contain correct `from_type` and `to_type`
- `estimated_loss` MUST accurately reflect loss category:
  - `FractionalPart` for float→int
  - `SignificantDigits { lost_bits }` for F64→F32
  - `ValueRange { from_bits, to_bits }` for narrowing
- Message content MUST be descriptive and accurate

**Example Test**:
```rust
test_warning_precision_loss_f64_to_f32_significant_digits
test_warning_precision_loss_i64_to_f32_large_value
test_warning_precision_loss_float_to_int_fractional
```

---

#### 3.2 PotentialOverflow Warnings (10-15 tests)
**Trigger Conditions**:
- Float to integer with value outside integer range
- Division operations with potential overflow (INT_MIN / -1)
- Type conversions that may exceed target type capacity

**Contract**:
- Warning MUST contain correct `from_type`, `to_type`, and `operation`
- MUST trigger for overflow-prone scenarios
- Message MUST describe overflow risk clearly

**Example Test**:
```rust
test_warning_potential_overflow_f64_to_i32_out_of_range
test_warning_potential_overflow_division_int_min_by_neg_one
test_warning_potential_overflow_u64_to_i32_value_exceeds_range
```

---

#### 3.3 SignednessChange Warnings (10 tests)
**Trigger Conditions**:
- Mixed signed/unsigned operands of same width (I32+U32, I64+U64, etc.)
- Comparison operations with mixed signedness

**Contract**:
- Warning MUST contain correct `from_signed`, `to_signed` flags
- `may_affect_comparisons` MUST be `true` when mixing signedness
- Message MUST explain potential comparison issues

**Example Test**:
```rust
test_warning_signedness_change_i32_u32_mixed_signedness
test_warning_signedness_change_comparison_signed_unsigned
test_warning_signedness_change_i64_u64_same_width
```

---

#### 3.4 FloatSpecialValues Warnings (5-8 tests)
**Trigger Conditions**:
- Operations that may produce NaN (0.0/0.0, sqrt(-1))
- Operations that may produce Infinity (division by zero, overflow)

**Contract**:
- Warning MUST contain correct `operation`
- `may_produce_nan` and `may_produce_infinity` flags MUST be accurate
- Message MUST describe special value conditions

**Example Test**:
```rust
test_warning_float_special_values_may_produce_nan
test_warning_float_special_values_may_produce_infinity
test_warning_float_special_values_division_by_zero_risk
```

---

#### 3.5 Multiple Warnings (5-10 tests)
**Scenario**: Single promotion generating multiple warnings

**Contract**:
- All applicable warnings MUST be present
- Warning order MUST be consistent
- Combined warnings MUST be coherent

**Example**:
```rust
// U64 to I32: SignednessChange + PrecisionLoss + PotentialOverflow
test_warning_multiple_u64_to_i32_all_warnings
test_warning_multiple_f64_to_i8_precision_and_overflow
```

---

## 4. Edge Case Tests Contract

### Purpose
Cover boundary conditions, special values, and promotion matrix edge cases.

### Edge Case Categories:

#### 4.1 Type Boundary Cases (15 tests)
- Smallest to largest type promotions (I8→I64, U8→U64)
- Same-width cross-signedness with boundary values (I32::MAX, U32::MAX)
- Maximum/minimum value conversions

**Contract**: Validate correct handling at type limits

---

#### 4.2 Float-Integer Boundary Cases (10 tests)
- NaN to integer conversion
- Infinity to integer conversion
- Large integer to F32 with precision loss
- Float boundary values (f32::MAX, f32::MIN)

**Contract**: Validate special float value handling

---

#### 4.3 Promotion Matrix Edge Cases (8 tests)
- `compute_common_type()` returning `None` (fallback behavior)
- `get_promotion_rule()` returning `None` (no explicit rule)
- Type pairs with no matrix entry
- Bidirectional promotions (A→B vs B→A)

**Contract**: Validate graceful degradation and fallback logic

---

#### 4.4 Operation-Specific Edge Cases (10 tests)
- Division with overflow potential
- Comparison operations with mixed signedness
- Bitwise operations with signed integers
- Modulo with negative operands

**Contract**: Validate operation-specific promotion behavior

---

## 5. Corner Case Tests Contract

### Purpose
Test rare scenarios, helper method validation (through engine), and system boundaries.

### Corner Case Categories:

#### 5.1 Helper Method Validation (Integrated) (No separate tests)
**Approach**: Validate `is_signed_integer()`, `is_unsigned_integer()`, `get_bit_width()` through engine behavior

**Contract**: Engine behavior MUST correctly reflect helper method logic for all 12 IrType variants

---

#### 5.2 Multi-Warning Scenarios (8 tests)
- Promotions generating 3+ warnings
- Cascading promotions with accumulated warnings

**Contract**: All warnings MUST be accurately generated and preserved

---

#### 5.3 System Boundary Cases (6 tests)
- Missing promotion rules with fallback
- Invalid type combinations (if possible)
- Edge cases in cast insertion with minimal IR context

**Contract**: Graceful degradation without panics

---

#### 5.4 Promotion Chains (5 tests)
- Multi-step promotions (A→B→C scenarios if applicable)
- Consistent promotion results across chains

**Contract**: Promotion transitivity and consistency

---

## 6. Integration Tests Contract (Real PromotionMatrix)

### Purpose
Validate end-to-end functionality with actual PromotionMatrix rules.

### Test Scenarios (30-40 tests):

1. **Real-World Type Combinations**: Test common programming scenarios
2. **Complex Multi-Step Promotions**: Operations involving multiple casts
3. **Warning Integration**: Validate warning generation with real matrix
4. **Performance Validation**: Ensure acceptable performance with real matrix

**Contract**:
- Tests MUST use `PromotionMatrix::new()` (no mocking)
- Tests MUST validate complete workflow (analyze + insert_casts)
- Tests MUST verify real promotion rule application

---

## 7. Unit Tests Contract (Mocked PromotionMatrix)

### Purpose
Isolate TypePromotionEngine logic from PromotionMatrix dependencies.

### Test Scenarios (20-30 tests):

1. **Matrix Behavior Isolation**: Mock `compute_common_type()` to return specific values
2. **Error Path Testing**: Mock matrix to return `None` for edge cases
3. **Engine Logic Validation**: Test engine decision-making independently

**Contract**:
- Tests MUST use `MockPromotionMatrix` or trait-based mocking
- Tests MUST isolate engine logic from matrix implementation
- Tests MUST verify correct matrix API usage

**Mock Setup Example**:
```rust
let mut mock_matrix = MockPromotionMatrix::new();
mock_matrix.set_common_type(IrType::I32, IrType::F32, Some(IrType::F32));
mock_matrix.set_rule(IrType::I32, IrType::F32, PromotionRule::Direct { ... });
// Use mock_matrix in engine tests
```

---

## 8. Concurrent Execution Tests Contract

### Purpose
Validate thread-safety of TypePromotionEngine with concurrent reads.

### Test Scenarios (2-3 tests):

#### 8.1 Multi-Threaded Read Test
**Given**: Single `Arc<TypePromotionEngine>` instance  
**When**: 10+ threads perform 100+ promotions each simultaneously  
**Then**:
- All threads MUST complete without panics
- All threads MUST produce consistent results
- No data races MUST occur (validated by Rust's type system)

**Contract**:
- Use `Arc::new(TypePromotionEngine::new())` for sharing
- Use `std::thread::spawn` for thread creation
- Validate result consistency across all threads
- No `unsafe` code or mutable state

**Example**:
```rust
#[test]
fn test_concurrent_analyze_promotion_10_threads_100_ops() {
    let engine = Arc::new(TypePromotionEngine::new());
    let mut handles = vec![];
    
    for thread_id in 0..10 {
        let engine_clone = Arc::clone(&engine);
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let result = engine_clone.analyze_binary_promotion(
                    &IrType::I32, &IrType::F32, IrBinaryOp::Add, SourceSpan::default()
                );
                assert_eq!(result.result_type, IrType::F32);
                // More assertions...
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---

#### 8.2 Varied Type Combinations Concurrent Test
**Given**: Multiple type combinations tested concurrently  
**When**: Threads test different type pairs simultaneously  
**Then**: Results MUST be independent and correct for each type pair

---

#### 8.3 High Thread Count Stress Test
**Given**: 50-100 threads  
**When**: Each performs promotions  
**Then**: System MUST remain stable and produce consistent results

---

## Test Quality Metrics

### Coverage Targets:
- **Line Coverage**: 100% for TypePromotionEngine
- **Branch Coverage**: 100% for all conditionals
- **Test Count**: 100-120 total tests across all groups

### Assertion Standards:
- Hybrid approach: Snapshots for complex outputs, explicit for critical properties
- All warnings validated for type accuracy and message content
- All cast instructions validated for correctness
- All edge and corner cases documented and tested

---

## Test Execution Requirements

### Performance:
- Full suite: <10 seconds
- Individual test: <100ms (except concurrent tests)
- Concurrent tests: <2 seconds each

### Organization:
- Clear module-level comments separating test groups
- Consistent naming conventions
- No test dependencies or order requirements

---

**Contract Summary Status**: ✅ COMPLETE - All test contracts defined, ready for implementation
