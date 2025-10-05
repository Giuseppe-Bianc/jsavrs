# Research: Testing Strategies for Type Promotion Module

**Date**: 2025-10-05  
**Feature**: Comprehensive Test Suite for Type Promotion Module  
**Scope**: Research testing methodologies, coverage techniques, and edge case identification for Rust compiler type promotion logic

---

## Executive Summary

This research document analyzes testing strategies for achieving 100% line coverage of the `src/ir/type_promotion.rs` module while ensuring comprehensive validation of normal operations, boundary conditions, edge cases, and corner cases. The research identifies optimal testing patterns for Rust compiler infrastructure, coverage measurement tools, and systematic approaches to eliminate ambiguity and maximize defect detection.

---

## 1. Testing Framework Selection

### Decision: Rust Built-in Testing + Insta Snapshot Testing

**Rationale**:
- **Rust's `cargo test`**: Native support, zero additional dependencies, excellent IDE integration
- **Insta crate**: Already in project dependencies, ideal for complex struct validation (PromotionResult, PromotionWarning)
- **cargo llvm-cov**: LLVM-based coverage instrumentation provides accurate line/branch coverage with HTML reports

**Alternatives Considered**:
1. **Proptest (Property-Based Testing)**: 
   - **Pros**: Automatic generation of test cases, discovers edge cases through randomization
   - **Cons**: Non-deterministic failures complicate debugging; overkill for deterministic type promotion logic
   - **Rejected**: Type promotion has well-defined rules; exhaustive deterministic testing is preferable

2. **Criterion.rs (Benchmarking)**:
   - **Pros**: Precise performance measurement with statistical analysis
   - **Cons**: Violates FR-008 (no performance benchmarks); introduces timing non-determinism
   - **Rejected**: Explicitly excluded per Clarifications Q3

3. **Mockall/Mockito (Mocking)**:
   - **Pros**: Isolate units from dependencies
   - **Cons**: Type promotion module has minimal external dependencies; pure logic testing preferred
   - **Rejected**: Not needed for stateless type promotion functions

**Implementation Details**:
```rust
// Example: Using Insta for complex result validation
#[test]
fn test_f32_to_i64_promotion_with_warnings() {
    let matrix = PromotionMatrix::new();
    let result = matrix.analyze_promotion(...);
    
    // Snapshot test captures entire PromotionResult structure
    insta::assert_debug_snapshot!(result);
    
    // Specific assertions for critical properties
    assert_eq!(result.result_type, IrType::I64);
    assert!(result.warnings.contains(&PromotionWarning::PrecisionLoss { ... }));
}
```

---

## 2. Coverage Measurement Strategy

### Decision: cargo llvm-cov with 100% Line Coverage Target

**Rationale**:
- **Line Coverage**: Measures every executed line; achievable and verifiable (FR-001, FR-006)
- **Branch Coverage**: Implicit in Rust match statements; line coverage captures most branches
- **LLVM-based**: Accurate instrumentation at LLVM IR level, consistent across platforms

**Coverage Targets**:
| Metric | Target | Rationale |
|--------|--------|-----------|
| **Line Coverage** | 100% | Mandatory per Clarifications Q1 |
| **Function Coverage** | 100% | All public functions + complex helpers must be tested |
| **Branch Coverage** | ≥95% | Implicit via comprehensive match arm testing |
| **Path Coverage** | Not measured | Exponential complexity; line coverage sufficient for type promotion |

**Tooling**:
```bash
# Generate coverage report
cargo llvm-cov --html --open

# Verify 100% coverage for target module
cargo llvm-cov --package jsavrs --lib --text | grep "src/ir/type_promotion.rs"
# Expected output: src/ir/type_promotion.rs: 100.00%
```

**Exclusion Strategy**:
- **Unreachable Code**: `unreachable!()` macros legitimately excluded if proven unreachable
- **Debug Assertions**: `debug_assert!` excluded in release builds; validated in debug tests
- **Default Trait Impls**: If mechanically derived, may skip if equivalent to `new()` (e.g., `Default::default()`)

---

## 3. Edge Case Identification Methodology

### Decision: Systematic Boundary Value Analysis + Error Injection

**Rationale**:
Edge cases in type promotion arise from:
1. **Type System Boundaries**: Invalid combinations, unsupported conversions
2. **Numeric Boundaries**: MIN/MAX values, overflow/underflow conditions
3. **Algorithmic Boundaries**: Circular dependencies, resource exhaustion
4. **Semantic Boundaries**: Signedness changes, precision loss thresholds

**Edge Case Taxonomy** (from Feature Spec):

#### Category 1: Type System Boundary Violations
**Scenarios**:
- Invalid type combinations (e.g., `Bool` → `F32`)
- Unsupported conversions (e.g., `Char` → `I32` without explicit rules)
- Type lattice inconsistencies (circular promotion paths)

**Testing Strategy**:
```rust
#[test]
#[should_panic(expected = "Invalid type combination")]
fn test_bool_to_float_promotion_panics() {
    let matrix = PromotionMatrix::new();
    // Attempt forbidden promotion
    let _ = matrix.get_promotion_rule(&IrType::Bool, &IrType::F32)
        .expect("Should panic on invalid promotion");
}

#[test]
fn test_char_to_int_returns_error() {
    let matrix = PromotionMatrix::new();
    let result = matrix.compute_common_type(&IrType::Char, &IrType::I32);
    // Expect graceful error handling
    assert!(result.is_none() || result == Some(IrType::I32)); // Fallback behavior
}
```

#### Category 2: Numeric Boundary Conditions
**Scenarios**:
- **Integer MAX values**: `i8::MAX` → `i16`, `i32::MAX` → `i64`, `u64::MAX` → `i64`
- **Integer MIN values**: `i8::MIN` → `i16`, `i16::MIN` → `i32`
- **Float special values**: `f32::INFINITY`, `f32::NEG_INFINITY`, `f32::NAN`
- **Denormalized floats**: Subnormal numbers near zero
- **Overflow scenarios**: `u32::MAX` → `i32` (overflow detection)

**Testing Strategy**:
```rust
#[test]
fn test_i32_max_to_i64_promotion() {
    let matrix = PromotionMatrix::new();
    let common_type = matrix.compute_common_type(&IrType::I32, &IrType::I64);
    
    assert_eq!(common_type, Some(IrType::I64));
    
    // Verify no overflow warning for widening conversion
    let result = matrix.analyze_promotion(/* i32::MAX value */);
    assert!(!result.warnings.iter().any(|w| matches!(w, PromotionWarning::PotentialOverflow { .. })));
}

#[test]
fn test_u64_max_to_i64_overflow_warning() {
    let matrix = PromotionMatrix::new();
    let result = matrix.analyze_promotion(/* u64::MAX value to i64 */);
    
    // Expect overflow warning for narrowing conversion
    assert!(result.warnings.iter().any(|w| matches!(w, PromotionWarning::PotentialOverflow { .. })));
}

#[test]
fn test_float_nan_propagation() {
    let matrix = PromotionMatrix::new();
    // Test that NaN handling is documented in warnings
    let result = matrix.analyze_binary_op(/* NaN + 1.0 */);
    
    assert!(result.warnings.iter().any(|w| matches!(
        w, 
        PromotionWarning::FloatSpecialValues { may_produce_nan: true, .. }
    )));
}
```

#### Category 3: Circular Type Dependency Detection
**Scenarios**:
- Type A requires promotion to Type B, which requires promotion to Type A
- Indirect cycles through intermediate types
- Termination guarantees for promotion graph traversal

**Testing Strategy**:
```rust
#[test]
#[should_panic(expected = "Circular dependency detected")]
fn test_circular_promotion_panics() {
    let mut matrix = PromotionMatrix::new();
    
    // Manually construct circular dependency (if possible via API)
    // This tests defensive programming against invalid matrix configurations
    
    let _ = matrix.compute_common_type(&IrType::I32, &IrType::F32);
    // Expect panic if circular path exists
}

#[test]
fn test_promotion_graph_acyclic_property() {
    let matrix = PromotionMatrix::new();
    
    // Verify all promotion paths terminate
    for from_type in all_types() {
        for to_type in all_types() {
            if let Some(rule) = matrix.get_promotion_rule(&from_type, &to_type) {
                match rule {
                    PromotionRule::Indirect { intermediate_type, .. } => {
                        // Ensure intermediate doesn't create cycle
                        assert_ne!(intermediate_type, &from_type);
                        assert_ne!(intermediate_type, &to_type);
                    }
                    _ => {}
                }
            }
        }
    }
}
```

#### Category 4: Resource Exhaustion Scenarios
**Scenarios**:
- Memory allocation failures for intermediate types (platform-dependent)
- Stack overflow from deep recursion (if promotion uses recursion)
- Pathological cases with extreme type nesting

**Testing Strategy** (Limited due to Rust's allocation guarantees):
```rust
#[test]
fn test_deeply_nested_promotion_sequence() {
    let matrix = PromotionMatrix::new();
    
    // Simulate worst-case promotion chain: I8 → I16 → I32 → I64 → F64
    let start = IrType::I8;
    let end = IrType::F64;
    
    let result = matrix.compute_common_type(&start, &end);
    assert_eq!(result, Some(IrType::F64));
    
    // Verify no stack overflow or allocation issues
    // (Rust's compile-time checks prevent most pathological cases)
}

// Note: True memory exhaustion testing requires OS-level resource limits,
// which are platform-specific and non-deterministic. Focus on algorithmic bounds.
```

---

## 4. Test Organization and Naming Conventions

### Decision: Function-Based Grouping with Descriptive Names

**Rationale**:
- **Discoverability**: Test names reveal what functionality is validated
- **Maintainability**: Clear naming enables easy test updates when requirements change
- **Documentation**: Test names serve as executable specifications (per FR-010)

**Naming Convention**:
```
test_<entity>_<operation>_<scenario>_<expected_outcome>
```

**Examples**:
```rust
// ✅ Good: Self-documenting
#[test]
fn test_promotion_matrix_f32_to_f64_returns_direct_cast() { ... }

#[test]
fn test_promotion_matrix_u32_to_i32_warns_overflow() { ... }

#[test]
#[should_panic(expected = "Invalid type")]
fn test_type_promotion_bool_to_float_panics() { ... }

// ❌ Bad: Vague names
#[test]
fn test_promotion() { ... } // What aspect? What scenario?

#[test]
fn test_f32() { ... } // Which operation? Which expected outcome?
```

**Organization Strategy**:
```rust
// Group by entity/functionality
mod promotion_matrix_tests {
    // Tests for PromotionMatrix methods
    #[test] fn test_promotion_matrix_new() { ... }
    #[test] fn test_promotion_matrix_get_promotion_rule() { ... }
    // ...
}

mod type_promotion_tests {
    // Tests for TypePromotion struct
    #[test] fn test_type_promotion_new() { ... }
    // ...
}

mod edge_case_tests {
    // Tests for boundary conditions
    #[test] fn test_i32_max_to_i64() { ... }
    #[test] fn test_float_nan_handling() { ... }
    // ...
}

mod error_handling_tests {
    // Tests for panic and Result error paths
    #[test] #[should_panic] fn test_invalid_promotion_panics() { ... }
    // ...
}
```

---

## 5. Panic vs. Result Error Testing Strategy

### Decision: Dual-Mode Testing per Clarifications Q2

**Rationale**:
- **Panic Tests**: Validate defensive programming for unrecoverable errors (e.g., internal invariants violated)
- **Result Tests**: Validate graceful error propagation for recoverable errors (e.g., user input validation)

**When to Use Panic Tests** (`#[should_panic]`):
1. **Internal Consistency Violations**: Code paths that should never execute if module state is valid
2. **Defensive Programming**: `assert!`, `unreachable!`, `panic!` in debug builds
3. **Contract Violations**: Public API misuse that violates documented preconditions

**Example**:
```rust
#[test]
#[should_panic(expected = "Promotion matrix not initialized")]
fn test_uninitialized_matrix_panics() {
    let matrix = PromotionMatrix::default(); // Hypothetically uninitialized
    let _ = matrix.get_promotion_rule(&IrType::I32, &IrType::F32);
    // Expect panic due to internal state violation
}
```

**When to Use Result Tests**:
1. **User Input Validation**: Invalid type combinations from compiler frontend
2. **External Resource Failures**: (Not applicable for stateless type promotion)
3. **Configurable Behavior**: When API returns `Option` or `Result` for expected failure modes

**Example**:
```rust
#[test]
fn test_invalid_promotion_returns_none() {
    let matrix = PromotionMatrix::new();
    let result = matrix.compute_common_type(&IrType::Bool, &IrType::F32);
    
    // Expect graceful handling (None or fallback type)
    assert!(result.is_none() || result == Some(IrType::I32)); // Fallback
}
```

**Best Practices**:
- **Specify expected panic message**: `#[should_panic(expected = "specific substring")]` for precision
- **Test both positive and negative paths**: Ensure error handling doesn't mask valid cases
- **Document panic conditions**: Rustdoc comments explain why panic is appropriate

---

## 6. Granularity of Helper Function Testing

### Decision: Mixed Strategy per Clarifications Q4

**Rationale**:
- **Direct Testing**: Complex helpers with non-trivial logic, branching, or state management
- **Indirect Testing**: Simple one-line helpers or trivial wrappers validated through public API

**Complexity Criteria for Direct Testing**:
1. **Cyclomatic Complexity > 3**: Multiple branches, nested conditionals
2. **Internal State Mutation**: Modifies struct fields or global state
3. **Error Handling**: Returns `Result` or may panic
4. **Performance-Critical**: O(n²) or worse algorithmic complexity

**Example Analysis** (from `src/ir/type_promotion.rs`):

| Helper Function | Complexity | Testing Strategy | Rationale |
|-----------------|------------|------------------|-----------|
| `add_promotion_rule()` | Low | Indirect | Simple HashMap insertion, validated via `get_promotion_rule()` |
| `add_symmetric_promotion_rule()` | Low | Indirect | Wrapper around `add_promotion_rule()` |
| `initialize_default_promotions()` | High | Direct | Complex logic with 40+ rule insertions, critical for correctness |
| `get_higher_type()` | Medium | Direct | Multiple match arms, core promotion logic |
| `compute_common_type()` | High | Direct | Primary public API, complex fallback logic |

**Direct Testing Example**:
```rust
#[test]
fn test_get_higher_type_float_precedence() {
    let matrix = PromotionMatrix::new();
    
    // Test internal helper directly
    let result = matrix.get_higher_type(&IrType::I32, &IrType::F32);
    assert_eq!(result, IrType::F32); // Float takes precedence
    
    let result = matrix.get_higher_type(&IrType::F64, &IrType::I64);
    assert_eq!(result, IrType::F64); // F64 > I64
}
```

**Indirect Testing Example**:
```rust
#[test]
fn test_symmetric_promotion_through_public_api() {
    let matrix = PromotionMatrix::new();
    
    // Indirectly validate add_symmetric_promotion_rule() via get_promotion_rule()
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::I32);
    assert!(matches!(rule, Some(PromotionRule::Direct { cast_kind: CastKind::Bitcast, .. })));
}
```

---

## 7. Documentation Standards for Test Cases

### Decision: Descriptive Name + Rustdoc Comment per FR-010

**Rationale**:
- **Self-Documenting Names**: Test intent clear from function name alone
- **Rustdoc Comments**: Explain *why* test exists, expected behavior, and edge case rationale
- **No Inline Comments**: Keep test bodies clean; use doc comments and assertions

**Template**:
```rust
/// Tests that promotion from i32 to f32 preserves value exactly without precision loss.
///
/// # Rationale
/// IEEE 754 float32 has 24-bit significand, which can exactly represent all i32 values
/// within the range [-2^24, 2^24]. This test validates that no PrecisionLoss warning
/// is generated for this conversion.
///
/// # Expected Behavior
/// - Promotion rule: Direct cast (IntToFloat)
/// - Warnings: None (no precision loss)
/// - Result type: F32
#[test]
fn test_i32_to_f32_promotion_preserves_value() {
    let matrix = PromotionMatrix::new();
    let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::F32);
    
    assert!(matches!(rule, Some(PromotionRule::Direct { 
        cast_kind: CastKind::IntToFloat,
        may_lose_precision: false, // Key assertion
        ..
    })));
}
```

**Bad Example** (violates FR-010):
```rust
#[test]
fn test_promotion() { // ❌ Vague name
    let m = PromotionMatrix::new(); // ❌ Cryptic variable names
    let r = m.get_promotion_rule(&IrType::I32, &IrType::F32);
    assert!(r.is_some()); // ❌ What property is being tested?
    // Lots of inline comments explaining logic ❌ Should be in doc comment
}
```

---

## 8. Test Execution and CI/CD Integration

### Decision: Fast, Deterministic Tests with Automated Coverage Reporting

**Performance Targets** (from Technical Context):
- **Per-Test Execution**: <100ms (no I/O, pure computation)
- **Full Suite Execution**: <5 seconds (fast feedback loop)
- **Coverage Computation**: <10 seconds with `cargo llvm-cov --html`

**CI/CD Integration Strategy**:
```yaml
# Example GitHub Actions workflow (conceptual)
name: Test & Coverage
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov
      - name: Run tests
        run: cargo test --package jsavrs --test ir_type_promotion_tests
      - name: Generate coverage
        run: cargo llvm-cov --package jsavrs --lib --html
      - name: Verify 100% coverage
        run: |
          COVERAGE=$(cargo llvm-cov --package jsavrs --lib --text | grep "src/ir/type_promotion.rs" | awk '{print $2}')
          if [ "$COVERAGE" != "100.00%" ]; then
            echo "Coverage is $COVERAGE, expected 100.00%"
            exit 1
          fi
      - name: Upload coverage report
        uses: actions/upload-artifact@v3
        with:
          name: coverage-report
          path: target/llvm-cov/html
```

**Local Development Workflow**:
```bash
# Run tests during development
cargo test ir_type_promotion --lib

# Check coverage before committing
cargo llvm-cov --package jsavrs --lib --open

# Validate specific test file
cargo test --test ir_type_promotion_tests -- --nocapture
```

---

## 9. Snapshot Testing with Insta

### Decision: Use Insta for Complex Struct Validation

**When to Use Snapshot Tests**:
1. **Complex Output Structures**: `PromotionResult` with multiple fields, nested warnings
2. **Regression Detection**: Catch unintended changes in promotion logic
3. **Documentation**: Snapshots serve as human-readable expected outputs

**Example**:
```rust
#[test]
fn test_f64_to_i32_promotion_generates_warnings() {
    let matrix = PromotionMatrix::new();
    let result = matrix.analyze_promotion(
        &IrType::F64,
        &IrType::I32,
        IrBinaryOp::Add,
        SourceSpan::default()
    );
    
    // Snapshot captures entire PromotionResult for regression detection
    insta::assert_debug_snapshot!(result);
    
    // Critical assertions remain explicit
    assert_eq!(result.result_type, IrType::I32);
    assert!(result.warnings.len() > 0);
}
```

**Snapshot File** (`.snapshots/ir_type_promotion_tests__f64_to_i32_promotion_generates_warnings.snap`):
```
---
source: tests/ir_type_promotion_tests.rs
expression: result
---
PromotionResult {
    result_type: I32,
    left_cast: Some(TypePromotion { from_type: F64, to_type: I32, cast_kind: FloatToInt, ... }),
    right_cast: None,
    warnings: [
        PrecisionLoss { from_type: F64, to_type: I32, estimated_loss: FractionalPart },
        PotentialOverflow { from_type: F64, to_type: I32, operation: Add }
    ],
    is_sound: true
}
```

**Best Practices**:
- **Review snapshots on first run**: `cargo insta review`
- **Update snapshots intentionally**: `cargo insta accept` after validating changes
- **Combine with assertions**: Snapshot for regression, explicit asserts for critical properties

---

## 10. Addressing Ambiguity and Maximizing Defect Detection

### Decision: Systematic Test Matrix + Equivalence Partitioning

**Eliminating Ambiguity**:
1. **Explicit Assertions**: Every test has clear pass/fail criteria
2. **Single Responsibility**: One test validates one property
3. **Descriptive Names**: Test intent unambiguous from name alone

**Maximizing Defect Detection**:
1. **Equivalence Partitioning**: Group inputs by expected behavior, test one from each partition
2. **Boundary Value Analysis**: Test MIN/MAX/zero/one-above/one-below for numeric types
3. **Negative Testing**: Explicitly test invalid inputs and error paths
4. **Mutation Testing** (Future): Verify tests catch intentional bugs (e.g., `cargo mutants`)

**Test Matrix Example** (Equivalence Partitioning for Integer Promotions):

| Source Type | Target Type | Partition | Expected Behavior | Test Case |
|-------------|-------------|-----------|-------------------|-----------|
| I8 | I16 | Widening signed | Direct cast, no loss | `test_i8_to_i16_widening` |
| I32 | I64 | Widening signed | Direct cast, no loss | `test_i32_to_i64_widening` |
| U8 | U16 | Widening unsigned | Direct cast, no loss | `test_u8_to_u16_widening` |
| I32 | U32 | Sign change | Promote to I64, signedness warning | `test_i32_to_u32_sign_change` |
| U64 | I64 | Narrowing (potential overflow) | Direct cast, overflow warning | `test_u64_to_i64_overflow` |
| I8 | F32 | Int to float | Direct cast, no loss | `test_i8_to_f32_exact` |
| I64 | F32 | Int to float (large) | Direct cast, precision loss warning | `test_i64_to_f32_precision_loss` |

**Boundary Value Test Cases**:
```rust
#[test]
fn test_i32_max_boundary() {
    // Test upper boundary of i32
    let value = i32::MAX;
    let result = promote(IrType::I32, IrType::I64, value);
    assert_eq!(result.value, i64::from(i32::MAX));
}

#[test]
fn test_i32_min_boundary() {
    // Test lower boundary of i32
    let value = i32::MIN;
    let result = promote(IrType::I32, IrType::I64, value);
    assert_eq!(result.value, i64::from(i32::MIN));
}

#[test]
fn test_i32_zero_boundary() {
    // Test zero (common edge case)
    let value = 0_i32;
    let result = promote(IrType::I32, IrType::F32, value);
    assert_eq!(result.value, 0.0_f32);
}

#[test]
fn test_i32_one_above_min_boundary() {
    // Test just above minimum (off-by-one errors)
    let value = i32::MIN + 1;
    let result = promote(IrType::I32, IrType::I64, value);
    assert_eq!(result.value, i64::from(i32::MIN + 1));
}
```

---

## 11. Rust-Specific Testing Best Practices

### Decision: Leverage Rust's Type System and Compiler Checks

**Compile-Time Safety**:
- **Ownership Rules**: Prevent most resource leaks; no need for explicit cleanup tests
- **Type Safety**: Invalid type combinations caught at compile time; focus tests on runtime logic
- **Borrow Checker**: Prevents data races; no need for concurrency tests in stateless module

**Rust Testing Idioms**:
```rust
// ✅ Use type system to enforce invariants
#[test]
fn test_promotion_result_is_valid() {
    let result = compute_promotion(...);
    
    // Type system ensures result_type is valid IrType (no need to test)
    // Focus on logical correctness
    assert_eq!(result.result_type, expected_type);
}

// ✅ Leverage match exhaustiveness checking
#[test]
fn test_all_overflow_behaviors_handled() {
    let behaviors = [
        OverflowBehavior::Wrap,
        OverflowBehavior::Saturate,
        OverflowBehavior::Trap,
        OverflowBehavior::CompileError,
    ];
    
    for behavior in &behaviors {
        let matrix = PromotionMatrix::with_overflow_behavior(*behavior);
        assert_eq!(matrix.get_overflow_behavior(), *behavior);
    }
    
    // Compiler ensures match is exhaustive; no need to test "default" case
}

// ✅ Test Debug/Display impls for diagnostics
#[test]
fn test_promotion_warning_display() {
    let warning = PromotionWarning::PrecisionLoss { ... };
    let display = format!("{:?}", warning);
    assert!(display.contains("PrecisionLoss"));
}
```

---

## 12. Test Data Management

### Decision: Inline Test Data with Constants

**Rationale**:
- **Type Promotion is Stateless**: No external data sources or fixtures needed
- **Deterministic Logic**: Fixed input/output mappings defined by type lattice
- **Self-Contained Tests**: Each test has all data inline for clarity

**Test Data Patterns**:
```rust
// ✅ Use constants for reusable test data
const ALL_INTEGER_TYPES: &[IrType] = &[
    IrType::I8, IrType::I16, IrType::I32, IrType::I64,
    IrType::U8, IrType::U16, IrType::U32, IrType::U64,
];

const ALL_FLOAT_TYPES: &[IrType] = &[IrType::F32, IrType::F64];

const ALL_BASIC_TYPES: &[IrType] = &[
    IrType::I8, IrType::I16, IrType::I32, IrType::I64,
    IrType::U8, IrType::U16, IrType::U32, IrType::U64,
    IrType::F32, IrType::F64, IrType::Bool, IrType::Char,
];

#[test]
fn test_identity_promotions_for_all_types() {
    let matrix = PromotionMatrix::new();
    
    for ty in ALL_BASIC_TYPES {
        let rule = matrix.get_promotion_rule(ty, ty);
        assert!(matches!(rule, Some(PromotionRule::Direct { 
            cast_kind: CastKind::Bitcast, 
            may_lose_precision: false,
            may_overflow: false
        })));
    }
}
```

**Avoid External Test Data Files** (unless necessary for integration tests):
- **Inline Data**: Keep test data in test code for maintainability
- **Constants**: Reuse common test values across multiple tests
- **Generators**: Use helper functions to generate test cases programmatically

---

## 13. Continuous Improvement and Maintenance

### Decision: Test Suite as Living Documentation

**Maintenance Strategies**:
1. **Refactor Tests with Code**: When module changes, update tests simultaneously
2. **Monitor Coverage Regressions**: CI fails if coverage drops below 100%
3. **Snapshot Review**: Regularly review Insta snapshots for unintended changes
4. **Test Naming Convention Enforcement**: Code review ensures adherence to naming standards

**Future Enhancements** (Out of Scope for Initial Implementation):
- **Mutation Testing**: Use `cargo-mutants` to verify tests catch intentional bugs
- **Fuzz Testing**: Explore `cargo-fuzz` for automatic edge case discovery (low priority due to deterministic logic)
- **Property-Based Testing**: If promotion rules become more complex, consider Proptest

---

## 14. Risk Analysis and Mitigation

### Identified Risks:

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Test Suite Execution Time Exceeds 5s** | Medium | Medium | Profile slow tests; optimize or mark as `#[ignore]` for local dev |
| **Coverage Tool Inaccuracy** | Low | High | Cross-verify with manual code inspection; use `cargo tarpaulin` as fallback |
| **Snapshot Test Churn** | Medium | Low | Review snapshots carefully; use `.snap` diffs in PRs; document snapshot update process |
| **Flaky Tests (Non-Determinism)** | Low | High | Enforce no I/O, no timing, no randomness in tests; CI reruns on failure |
| **Test Maintenance Burden** | Medium | Medium | Enforce documentation standards; refactor tests with code changes |

---

## Conclusion

This research establishes a comprehensive, detailed, precise, and in-depth testing strategy for the `src/ir/type_promotion.rs` module. The approach combines:

- **Rust Built-in Testing + Insta**: Native frameworks with minimal dependencies
- **cargo llvm-cov**: Accurate 100% line coverage measurement
- **Systematic Edge Case Analysis**: Boundary value analysis, error injection, equivalence partitioning
- **Dual-Mode Error Testing**: Panic tests for internal invariants, Result tests for recoverable errors
- **Documentation Rigor**: Every test has descriptive name + rustdoc comment
- **Fast, Deterministic Execution**: <5s suite, <100ms per test, no non-determinism

**Next Steps** (Phase 1 - Design):
1. Generate `data-model.md` documenting test entities (TestCase, TestSuite, CoverageMetric)
2. Create `contracts/test-interface.yaml` defining test function signatures
3. Produce `quickstart.md` with example test execution workflow
4. Update `QWEN.md` with testing context for AI-assisted development

**Estimated Test Count**: 40-60 test functions to achieve 100% coverage + comprehensive edge case validation.
