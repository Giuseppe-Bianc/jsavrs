# Research: Comprehensive Type Promotion Engine Test Suite

**Date**: October 6, 2025  
**Feature**: Comprehensive Type Promotion Engine Test Suite  
**Status**: Complete

## Executive Summary

This document presents comprehensive research findings for implementing a detailed, precise, thorough, and in-depth test suite for the TypePromotionEngine module in the jsavrs compiler. The research covers testing strategies, type promotion analysis, mocking approaches, concurrency testing patterns, snapshot testing best practices, and coverage analysis techniques required to achieve 100% line and branch coverage.

---

## 1. Testing Strategy Analysis

### Decision: Hybrid Testing Approach (Integration + Unit Tests)

**Rationale**:
- **Integration Tests with Real PromotionMatrix**: Validate end-to-end functionality and real-world behavior of type promotion rules
- **Unit Tests with Mocked Matrix**: Isolate TypePromotionEngine logic from PromotionMatrix dependencies, enabling precise testing of engine decision-making without dependency on matrix implementation details
- **Hybrid Assertion Strategy**: Combine snapshot testing (for complex PromotionResult structures) with explicit assertions (for critical properties like result types, cast presence, is_sound flag)

**Alternatives Considered**:
1. **Integration Tests Only**: Rejected because it doesn't isolate engine logic from matrix behavior, making it harder to test engine-specific decision paths
2. **Unit Tests Only**: Rejected because it doesn't validate real-world integration with actual PromotionMatrix rules
3. **Explicit Assertions Only**: Rejected because complex PromotionResult structures with warnings are verbose to assert manually; snapshots provide better regression detection
4. **Snapshot Tests Only**: Rejected because critical properties (types, cast presence) should be explicitly validated for clarity

**Implementation Approach**:
- Group 1: Integration tests using `PromotionMatrix::new()` - tests real promotion rules
- Group 2: Unit tests mocking PromotionMatrix behavior via traits or test doubles
- Use `insta` crate for snapshot assertions on complex outputs
- Use standard `assert_eq!`, `assert!` for critical scalar properties

---

## 2. Type Coverage Matrix Analysis

### Decision: Exhaustive Coverage of All 12 IrType Variants

**IrType Variants Requiring Testing**:
```rust
// Integer types (8 variants)
I8, I16, I32, I64  // Signed integers
U8, U16, U32, U64  // Unsigned integers

// Floating-point types (2 variants)
F32, F64

// Other types (2 variants)
Bool, Char
```

**Test Patterns to Implement**:

1. **Identity Promotions** (12 test cases):
   - Each type to itself (I8→I8, I16→I16, ..., Bool→Bool, Char→Char)
   - Expected: No casts required, result_type equals input types

2. **Widening Within Signedness** (12 test cases):
   - Signed: I8→I16, I8→I32, I8→I64, I16→I32, I16→I64, I32→I64
   - Unsigned: U8→U16, U8→U32, U8→U64, U16→U32, U16→U64, U32→U64
   - Expected: Cast to wider type, no precision loss, result_type is wider type

3. **Narrowing Within Signedness** (12 test cases):
   - Reverse of widening: I64→I32, I64→I16, I64→I8, etc.
   - Expected: PrecisionLoss warnings, ValueRange loss estimate

4. **Cross-Signedness Same Width** (4 test cases):
   - I8↔U8, I16↔U16, I32↔U32, I64↔U64
   - Expected: SignednessChange warnings, may_affect_comparisons=true

5. **Integer to Float** (16 test cases):
   - All 8 integer types → F32 (8 cases)
   - All 8 integer types → F64 (8 cases)
   - Expected: IntToFloat cast, potential precision loss for large integers to F32

6. **Float to Integer** (16 test cases):
   - F32 → all 8 integer types (8 cases)
   - F64 → all 8 integer types (8 cases)
   - Expected: FloatToInt cast, PrecisionLoss warnings (fractional part), PotentialOverflow warnings

7. **Float Promotions** (2 test cases):
   - F32→F64 (widening)
   - F64→F32 (narrowing with precision loss)

**Rationale**:
Complete coverage of all type pairs ensures no edge cases are missed in the type promotion system. The type promotion lattice is complex with different rules for signedness, width, and floating-point vs. integer conversions.

**Total Type Combination Tests**: ~62 core type pair tests (identity + widening + narrowing + cross-signedness + integer-float conversions)

---

## 3. Binary Operation Coverage Analysis

### Decision: Test All IrBinaryOp Variants with Representative Type Combinations

**IrBinaryOp Categories**:

1. **Arithmetic Operations** (5 variants):
   - `Add`, `Subtract`, `Multiply`, `Divide`, `Modulo`
   - Special consideration: `Divide` may have PotentialOverflow warnings

2. **Comparison Operations** (6 variants):
   - `Equal`, `NotEqual`, `Less`, `LessEqual`, `Greater`, `GreaterEqual`
   - Return type: Bool (regardless of operand types)

3. **Logical Operations** (2 variants):
   - `And`, `Or`
   - Typically used with Bool types

4. **Bitwise Operations** (5 variants):
   - `BitwiseAnd`, `BitwiseOr`, `BitwiseXor`, `ShiftLeft`, `ShiftRight`
   - Mixed signedness requires special handling

**Test Strategy**:
- Test each operation category with at least 3 representative type combinations:
  1. Same-type operands (no promotion)
  2. Mixed-width same-signedness (widening promotion)
  3. Mixed-signedness or integer-float (complex promotion with warnings)

**Total Operation Tests**: 18 operations × 3 scenarios = ~54 operation-specific tests

---

## 4. Edge Case Testing Strategy

### Decision: Comprehensive Boundary and Special Value Testing

**Edge Cases Identified**:

1. **Type Boundary Cases**:
   - Smallest to largest promotions (I8→I64, U8→U64)
   - Same-width different-signedness (I64↔U64, I32↔U32)
   - Boundary value conversions (i32::MAX, i32::MIN, u64::MAX)
   - Tests: 10-15 boundary scenarios

2. **Float-Integer Boundary Cases**:
   - Float special values: NaN, Infinity, -Infinity
   - Large integers that lose precision in F32 (e.g., 2^53 + 1)
   - Float to integer conversions at max/min representable values
   - Tests: 8-10 float boundary scenarios

3. **Promotion Matrix Edge Cases**:
   - Type pairs not explicitly defined in matrix (fallback behavior)
   - Identity promotions (already covered in type matrix)
   - Bidirectional promotions (A→B vs B→A symmetry)
   - Tests: 5-8 matrix edge scenarios

4. **Operation-Specific Edge Cases**:
   - Division with potential overflow (INT_MIN / -1)
   - Comparisons with mixed signedness
   - Bitwise operations with signed integers
   - Tests: 6-8 operation edge scenarios

**Total Edge Case Tests**: ~30-40 tests

---

## 5. Corner Case Testing Strategy

### Decision: Test Rare Scenarios and Helper Method Validation

**Corner Cases Identified**:

1. **Helper Method Validation** (through TypePromotionEngine usage):
   - `is_signed_integer()`: Test with all 12 IrType variants
   - `is_unsigned_integer()`: Test with all 12 IrType variants
   - `get_bit_width()`: Test with all numeric types (10 variants: I8-I64, U8-U64, F32, F64)
   - **Testing Approach**: Validate these methods indirectly by testing engine behavior that relies on them (signedness checks, width comparisons)
   - Tests: Integrated into main test scenarios (no separate tests per spec clarification)

2. **Complex Multi-Warning Scenarios**:
   - Single promotion generating multiple warnings (e.g., U64→I32: SignednessChange + PrecisionLoss + PotentialOverflow)
   - Promotion chains (A→B→C sequential casts if needed)
   - Tests: 5-8 multi-warning scenarios

3. **System Boundary Corner Cases**:
   - PromotionMatrix::compute_common_type() returning None
   - PromotionMatrix::get_promotion_rule() returning None
   - Missing function context in cast insertion (graceful degradation)
   - Tests: 4-6 boundary scenarios

4. **Concurrent Execution Tests**:
   - Multiple threads analyzing promotions simultaneously
   - Thread-safety validation for TypePromotionEngine
   - Read-only operations (no mutation, so thread-safe by design)
   - Tests: 2-3 concurrent execution scenarios using `std::thread::spawn`

**Total Corner Case Tests**: ~15-20 tests

---

## 6. Mocking Strategy for Unit Tests

### Decision: Use Trait-Based Mocking for PromotionMatrix Isolation

**Approach**:

1. **Trait Abstraction** (if modifying source is acceptable):
   ```rust
   pub trait PromotionMatrixTrait {
       fn compute_common_type(&self, left: &IrType, right: &IrType) -> Option<IrType>;
       fn get_promotion_rule(&self, from: &IrType, to: &IrType) -> Option<&PromotionRule>;
       fn get_overflow_behavior(&self) -> OverflowBehavior;
   }
   ```
   - TypePromotionEngine accepts `impl PromotionMatrixTrait` or generic parameter
   - Test implementations provide mock behavior

2. **Test Double Pattern** (if avoiding source modification):
   ```rust
   struct MockPromotionMatrix {
       rules: HashMap<(IrType, IrType), PromotionRule>,
       common_types: HashMap<(IrType, IrType), Option<IrType>>,
   }
   
   impl MockPromotionMatrix {
       fn new() -> Self { /* ... */ }
       fn set_rule(&mut self, from: IrType, to: IrType, rule: PromotionRule) { /* ... */ }
       fn set_common_type(&mut self, left: IrType, right: IrType, result: Option<IrType>) { /* ... */ }
   }
   ```
   - Mock implements same interface as real PromotionMatrix
   - Tests configure mock behavior for specific scenarios

**Rationale**:
- Mocking enables testing of TypePromotionEngine logic independently of PromotionMatrix implementation
- Allows testing error paths and edge cases that might be hard to trigger with real matrix
- Provides fast, deterministic tests without relying on complex matrix state

**Alternatives Considered**:
- `mockall` crate: Powerful but adds dependency; trait-based approach is simpler
- Manual test doubles: Chosen for simplicity and no additional dependencies

---

## 7. Snapshot Testing Best Practices

### Decision: Use `insta` with Selective Snapshotting for Complex Outputs

**Snapshot Testing Strategy**:

1. **What to Snapshot**:
   - Complete `PromotionResult` structures with all fields
   - Collections of `PromotionWarning` with multiple warnings
   - Complex type promotion chains with intermediate casts
   - **Rationale**: These structures are verbose and change frequently; snapshots catch regressions easily

2. **What NOT to Snapshot**:
   - Simple scalar values (result types, cast presence booleans)
   - Single warnings with predictable structure
   - Critical properties that should be explicitly validated
   - **Rationale**: Explicit assertions provide better clarity for critical properties

3. **`insta` Configuration**:
   ```rust
   use insta::{assert_debug_snapshot, assert_yaml_snapshot};
   
   // Use debug snapshots for Rust types
   assert_debug_snapshot!(promotion_result);
   
   // Use YAML for more readable output
   assert_yaml_snapshot!(promotion_result.warnings);
   ```

4. **Snapshot Organization**:
   - Snapshots stored in `tests/snapshots/ir_type_promotion_engine_tests/`
   - Naming convention: `test_function_name.snap`
   - Use `insta` redaction for non-deterministic fields (if any)

5. **Snapshot Review Workflow**:
   ```bash
   # Run tests to generate snapshots
   cargo test
   
   # Review new/changed snapshots
   cargo insta review
   
   # Accept all snapshots (after review)
   cargo insta accept
   ```

**Benefits**:
- Regression detection: Any change in output is immediately visible
- Documentation: Snapshots serve as examples of expected output
- Maintainability: Easier to update than manual assertions for complex structures

**Best Practices**:
- Keep snapshots small and focused (one aspect per snapshot)
- Review snapshots carefully during code review
- Use explicit assertions alongside snapshots for critical properties

---

## 8. Test Organization and Structure

### Decision: Group Tests by Function Under Test with Clear Module Comments

**File Structure**:
```rust
// tests/ir_type_promotion_engine_tests.rs

// ============================================================================
// Module: Tests for TypePromotionEngine::analyze_binary_promotion
// ============================================================================
// Tests validate that analyze_binary_promotion correctly determines result
// types and generates appropriate TypePromotion structures for all type
// combinations and binary operations.

#[test]
fn test_analyze_binary_promotion_identity_i32_add() { /* ... */ }

#[test]
fn test_analyze_binary_promotion_widening_i8_to_i32_add() { /* ... */ }

// ... more analyze_binary_promotion tests ...

// ============================================================================
// Module: Tests for TypePromotionEngine::insert_promotion_casts
// ============================================================================
// Tests validate that insert_promotion_casts correctly inserts cast
// instructions into the IR and returns properly typed Value instances.

#[test]
fn test_insert_promotion_casts_left_operand_i8_to_i32() { /* ... */ }

// ... more insert_promotion_casts tests ...

// ============================================================================
// Module: Warning Generation Tests
// ============================================================================
// Tests validate accurate generation of PrecisionLoss, PotentialOverflow,
// and SignednessChange warnings with correct metadata and message content.

#[test]
fn test_warning_precision_loss_f64_to_f32() { /* ... */ }

// ... more warning tests ...

// ============================================================================
// Module: Edge Case Tests
// ============================================================================
// Tests cover boundary conditions, special float values, promotion matrix
// edge cases, and operation-specific edge scenarios.

#[test]
fn test_edge_case_i8_to_i64_max_value() { /* ... */ }

// ... more edge case tests ...

// ============================================================================
// Module: Corner Case Tests
// ============================================================================
// Tests cover rare scenarios including helper method validation (through
// engine usage), multi-warning scenarios, and system boundaries.

#[test]
fn test_corner_case_multiple_warnings_u64_to_i32() { /* ... */ }

// ... more corner case tests ...

// ============================================================================
// Module: Integration Tests (Real PromotionMatrix)
// ============================================================================
// Tests validate end-to-end functionality with actual PromotionMatrix rules.

#[test]
fn test_integration_real_matrix_f32_i32_add() { /* ... */ }

// ... more integration tests ...

// ============================================================================
// Module: Unit Tests (Mocked PromotionMatrix)
// ============================================================================
// Tests isolate TypePromotionEngine logic by mocking matrix behavior.

#[test]
fn test_unit_mocked_matrix_compute_common_type_none() { /* ... */ }

// ... more unit tests ...

// ============================================================================
// Module: Concurrent Execution Tests
// ============================================================================
// Tests validate thread-safety of TypePromotionEngine with multiple threads.

#[test]
fn test_concurrent_execution_multiple_threads_analyze_promotion() { /* ... */ }

// ... more concurrent tests ...
```

**Test Naming Convention**:
```
test_<function_or_group>_<scenario>_<expected_outcome>
```

Examples:
- `test_analyze_binary_promotion_i32_f32_promotes_to_f32`
- `test_insert_promotion_casts_both_operands_different_types`
- `test_warning_precision_loss_i64_to_f32_large_value`
- `test_edge_case_float_nan_to_integer_special_handling`
- `test_corner_case_promotion_chain_i8_to_i32_to_f64`

**Rationale**:
- Clear separation by functionality aids navigation
- Module comments provide context without reading individual tests
- Consistent naming enables quick search and understanding
- Group organization enables parallel test execution

---

## 9. Code Coverage Strategy

### Decision: Achieve 100% Line and Branch Coverage Using `cargo-llvm-cov`

**Coverage Measurement Approach**:

1. **Tool Selection**: `cargo-llvm-cov`
   ```bash
   # Install
   cargo install cargo-llvm-cov
   
   # Generate coverage report
   cargo llvm-cov --html
   
   # View report
   # Open target/llvm-cov/html/index.html
   ```

2. **Coverage Targets**:
   - **Line Coverage**: 100% for `src/ir/type_promotion_engine.rs`
   - **Branch Coverage**: 100% for all `if`, `match`, and conditional expressions
   - **Function Coverage**: 100% for all public methods

3. **Uncovered Code Identification**:
   - Red-highlighted lines in HTML report indicate uncovered code
   - Use `--show-missing` flag for line-by-line coverage in terminal
   - Focus on conditional branches (`if`, `match` arms) often missed

4. **Iterative Coverage Improvement**:
   ```bash
   # Run tests with coverage
   cargo llvm-cov --html
   
   # Identify uncovered lines
   # Add tests for uncovered scenarios
   
   # Verify improvement
   cargo llvm-cov --html
   ```

5. **Coverage Exclusions** (if needed):
   - Use `#[cfg(not(tarpaulin_include))]` for unreachable code (e.g., panic branches)
   - Document why code is excluded

**Rationale**:
- `cargo-llvm-cov` is the standard Rust coverage tool with accurate branch coverage
- 100% coverage ensures all code paths are tested, critical for type safety
- HTML report provides visual feedback for identifying gaps

**Alternatives Considered**:
- `tarpaulin`: Less accurate branch coverage than `cargo-llvm-cov`
- Manual coverage tracking: Too error-prone; automated tools are essential

---

## 10. Concurrency Testing Approach

### Decision: Multi-Threaded Read Tests Using `std::thread`

**Concurrency Testing Strategy**:

1. **Thread-Safety Analysis**:
   - `TypePromotionEngine` contains `PromotionMatrix` which is read-only after initialization
   - All methods are `&self` (immutable borrows)
   - **Conclusion**: Thread-safe by design (no mutation, no synchronization needed)

2. **Test Implementation**:
   ```rust
   use std::thread;
   use std::sync::Arc;
   
   #[test]
   fn test_concurrent_analyze_promotion_multiple_threads() {
       let engine = Arc::new(TypePromotionEngine::new());
       let mut handles = vec![];
       
       // Spawn 10 threads, each performing 100 promotions
       for _ in 0..10 {
           let engine_clone = Arc::clone(&engine);
           let handle = thread::spawn(move || {
               for _ in 0..100 {
                   let result = engine_clone.analyze_binary_promotion(
                       &IrType::I32,
                       &IrType::F32,
                       IrBinaryOp::Add,
                       SourceSpan::default()
                   );
                   assert_eq!(result.result_type, IrType::F32);
               }
           });
           handles.push(handle);
       }
       
       // Wait for all threads
       for handle in handles {
           handle.join().unwrap();
       }
   }
   ```

3. **Test Scenarios**:
   - Multiple threads reading same engine instance (Arc-wrapped)
   - Different type combinations per thread
   - Validate consistent results across threads

4. **Expected Behavior**:
   - No data races (verified by absence of `unsafe` and use of `Arc`)
   - Consistent results regardless of thread interleaving
   - No panics or deadlocks

**Rationale**:
- Demonstrates thread-safety property explicitly
- Validates that concurrent usage produces consistent results
- Simple implementation using standard library (no external dependencies)

**Test Count**: 2-3 concurrent execution tests with varying thread counts and operations

---

## 11. Assertion Strategy Summary

### Hybrid Approach: Snapshots + Explicit Assertions

**When to Use Snapshots** (`insta`):
- ✅ Complete `PromotionResult` structures
- ✅ Collections of warnings (multiple warnings)
- ✅ Complex nested data with multiple fields
- ✅ Regression detection for output format changes

**When to Use Explicit Assertions** (`assert_eq!`, `assert!`):
- ✅ Result types (critical property)
- ✅ Cast presence/absence (`left_cast.is_some()`)
- ✅ `is_sound` flag (critical property)
- ✅ Warning counts (`warnings.len()`)
- ✅ Specific cast kinds (`CastKind::IntToFloat`)
- ✅ Critical warning metadata (types, operation)

**Hybrid Example**:
```rust
#[test]
fn test_analyze_binary_promotion_i32_f32_hybrid_assertions() {
    let engine = TypePromotionEngine::new();
    let result = engine.analyze_binary_promotion(
        &IrType::I32,
        &IrType::F32,
        IrBinaryOp::Add,
        SourceSpan::default()
    );
    
    // Explicit assertions for critical properties
    assert_eq!(result.result_type, IrType::F32, "Result type must be F32");
    assert!(result.left_cast.is_some(), "Left cast required for I32→F32");
    assert!(result.right_cast.is_none(), "Right operand already F32");
    assert!(result.is_sound, "Promotion is mathematically sound");
    
    // Snapshot for complete structure
    assert_debug_snapshot!(result);
}
```

---

## 12. Test Quality Metrics

### Targets for Comprehensive Testing:

1. **Coverage Metrics**:
   - Line coverage: 100% for TypePromotionEngine
   - Branch coverage: 100% for all conditionals
   - Function coverage: 100% for all public methods

2. **Test Count Targets**:
   - Total tests: 100-120 individual test functions
   - Integration tests: 30-40 tests
   - Unit tests (mocked): 20-30 tests
   - Edge case tests: 30-40 tests
   - Corner case tests: 15-20 tests
   - Concurrent tests: 2-3 tests

3. **Quality Indicators**:
   - Each test validates single, well-defined scenario
   - Test names clearly describe scenario and expected outcome
   - No test dependencies or execution order requirements
   - All tests pass independently and in parallel
   - Snapshot tests have corresponding explicit assertions for critical properties

4. **Execution Performance**:
   - Full test suite: <10 seconds
   - Individual test: <100ms (except concurrent tests)
   - Snapshot comparison: <50ms per snapshot

---

## 13. Dependencies and Tools

### Required Crates and Tools:

1. **Testing Framework**: `cargo test` (built-in)
2. **Snapshot Testing**: `insta` crate (add to `Cargo.toml` dev-dependencies)
   ```toml
   [dev-dependencies]
   insta = "1.34"
   ```
3. **Coverage Analysis**: `cargo-llvm-cov` (installed globally)
   ```bash
   cargo install cargo-llvm-cov
   ```
4. **Mocking** (if using `mockall`): `mockall` crate (optional)
   ```toml
   [dev-dependencies]
   mockall = "0.12"  # Optional, only if trait mocking needed
   ```

### Tool Installation Verification:
```bash
# Verify cargo test
cargo test --version

# Install and verify insta
cargo install cargo-insta
cargo insta --version

# Install and verify coverage tool
cargo install cargo-llvm-cov
cargo llvm-cov --version
```

---

## 14. Implementation Risks and Mitigations

### Identified Risks:

1. **Risk**: Test suite size may become unwieldy (2000-3000 lines)
   - **Mitigation**: Modular organization with clear comments; consider splitting into multiple files if >3000 lines

2. **Risk**: Snapshot tests may become brittle with frequent changes
   - **Mitigation**: Use explicit assertions for stable properties; snapshots only for complex outputs

3. **Risk**: Achieving 100% branch coverage may be difficult for error paths
   - **Mitigation**: Mock PromotionMatrix to trigger error conditions (None returns)

4. **Risk**: Concurrent tests may be flaky due to timing issues
   - **Mitigation**: Use deterministic test scenarios with fixed type combinations; avoid time-dependent behavior

5. **Risk**: Test execution time may exceed 10 seconds with 100+ tests
   - **Mitigation**: Use `cargo test --release` for faster execution; parallelize test execution (default cargo behavior)

---

## 15. Success Criteria

### Criteria for Phase 0 Completion:

- ✅ All unknowns from Technical Context resolved
- ✅ Testing strategy documented (hybrid integration + unit tests)
- ✅ Type coverage matrix defined (12 IrType variants, exhaustive patterns)
- ✅ Operation coverage defined (all IrBinaryOp variants)
- ✅ Edge case testing strategy defined (boundary values, special floats, matrix edges)
- ✅ Corner case testing strategy defined (helper validation, multi-warnings, concurrent)
- ✅ Mocking strategy defined (trait-based or test doubles)
- ✅ Snapshot testing best practices defined (`insta` usage patterns)
- ✅ Test organization structure defined (8 test groups with clear modules)
- ✅ Coverage measurement approach defined (`cargo-llvm-cov`, 100% targets)
- ✅ Concurrency testing approach defined (`std::thread`, Arc-wrapped engine)
- ✅ Assertion strategy defined (hybrid snapshots + explicit)
- ✅ Dependencies identified (`insta`, `cargo-llvm-cov`)
- ✅ Risk mitigation strategies documented

**Status**: ✅ COMPLETE - All research objectives met. Ready for Phase 1 (Design & Contracts).

---

## Appendix A: Type Promotion Rules Summary

Reference from `src/ir/type_promotion.rs`:

1. **Float Precedence**: F64 > F32 > All integer types
2. **Widening**: Smaller type → Larger type within signedness
3. **Cross-Signedness**: Same-width signed/unsigned → Promote to next larger signed type
4. **Identity**: Type → Same type (no cast)
5. **Integer to Float**: May lose precision for large integers to F32
6. **Float to Integer**: Always loses fractional part, may overflow

---

## Appendix B: IrBinaryOp Reference

Complete list of binary operations requiring type promotion testing:

- **Arithmetic**: Add, Subtract, Multiply, Divide, Modulo
- **Comparison**: Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual
- **Logical**: And, Or
- **Bitwise**: BitwiseAnd, BitwiseOr, BitwiseXor, ShiftLeft, ShiftRight

---

## Appendix C: Helper Method Validation Strategy

Per spec clarification, helper methods (`is_signed_integer`, `is_unsigned_integer`, `get_bit_width`) should be tested **through their usage in TypePromotionEngine**, not as separate isolated tests.

**Validation Approach**:
- Tests that rely on signedness checks (e.g., cross-signedness promotions) implicitly validate `is_signed_integer()` and `is_unsigned_integer()`
- Tests that handle different type widths (e.g., I8 vs I64) implicitly validate `get_bit_width()`
- No separate test functions for these helpers required

---

## References

1. jsavrs Type Promotion System: `src/ir/type_promotion.rs`
2. jsavrs Type Promotion Engine: `src/ir/type_promotion_engine.rs`
3. Existing Type Promotion Tests: `tests/ir_type_promotion_tests.rs`
4. Feature Specification: `specs/003-create-detailed-precise/spec.md`
5. Rust Testing Documentation: https://doc.rust-lang.org/book/ch11-00-testing.html
6. Insta Snapshot Testing: https://insta.rs/
7. Cargo LLVM Coverage: https://github.com/taiki-e/cargo-llvm-cov

---

**Research Complete**: October 6, 2025
