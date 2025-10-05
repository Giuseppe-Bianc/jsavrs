# Data Model: Type Promotion Test Suite

**Date**: 2025-10-05  
**Feature**: Comprehensive Test Suite for Type Promotion Module  
**Scope**: Entity specifications for test infrastructure and type promotion domain model

---

## Executive Summary

This data model defines all entities involved in testing the `src/ir/type_promotion.rs` module. It includes:
1. **Domain Entities**: Type promotion system components (from production code)
2. **Test Entities**: Test suite infrastructure components
3. **Relationships**: Entity connections and dependencies
4. **Validation Rules**: Constraints ensuring test correctness

---

## 1. Domain Entities (Production Code)

### 1.1 Entity: `PromotionMatrix`

**Purpose**: Central registry managing type promotion rules for the jsavrs compiler IR.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `rules` | `HashMap<(IrType, IrType), PromotionRule>` | Bidirectional map from (source, target) type pairs to promotion rules | Non-null, initialized with default promotions |
| `overflow_behavior` | `OverflowBehavior` | Strategy for handling integer overflow during promotions | Enum: Wrap \| Saturate \| Trap \| CompileError |

**Relationships**:
- **Owns**: Multiple `PromotionRule` instances (via `rules` HashMap)
- **Uses**: `IrType` enum for type keys
- **Configured by**: `OverflowBehavior` enum

**Validation Rules**:
- Must contain identity promotions for all basic types (T → T with Bitcast)
- All rules must be bidirectional (if A → B exists, B → A may exist with different behavior)
- No circular promotion paths (verified via `get_promotion_rule` traversal)

**State Transitions**:
- **Initialization**: `new()` → Default rules loaded via `initialize_default_promotions()`
- **Configuration**: `with_overflow_behavior(behavior)` → Sets overflow strategy
- **Rule Addition**: `add_promotion_rule(from, to, rule)` → Inserts single rule
- **Symmetric Addition**: `add_symmetric_promotion_rule(ty1, ty2, rule)` → Inserts bidirectional rules

**Example**:
```rust
let matrix = PromotionMatrix::new();
// matrix.rules contains ~50 default promotion rules
// matrix.overflow_behavior == OverflowBehavior::Wrap (default)
```

---

### 1.2 Entity: `PromotionRule`

**Purpose**: Defines how one type can be promoted to another.

**Variants** (Enum):
| Variant | Fields | Description |
|---------|--------|-------------|
| `Direct` | `cast_kind: CastKind`, `may_lose_precision: bool`, `may_overflow: bool` | Single-step conversion (e.g., I32 → F32) |
| `Indirect` | `intermediate_type: IrType`, `may_lose_precision: bool` | Two-step conversion via intermediate (e.g., U32 → I64 → F64) |
| `Forbidden` | `reason: String` | Disallowed conversion with explanation |

**Relationships**:
- **Used by**: `PromotionMatrix` (stored in `rules` HashMap)
- **References**: `IrType` (source, target, intermediate types)
- **References**: `CastKind` (Direct variant only)

**Validation Rules**:
- **Direct**: `cast_kind` must be valid for source/target type pair
- **Indirect**: `intermediate_type` must not equal source or target (prevents cycles)
- **Forbidden**: `reason` must be non-empty

**Example**:
```rust
// Direct promotion: I32 → I64 (widening, no loss)
PromotionRule::Direct {
    cast_kind: CastKind::SignExtend,
    may_lose_precision: false,
    may_overflow: false,
}

// Indirect promotion: U32 → F32 (via I64 intermediate)
PromotionRule::Indirect {
    intermediate_type: IrType::I64,
    may_lose_precision: true, // F32 has 24-bit significand
}

// Forbidden: Bool → F32 (invalid semantics)
PromotionRule::Forbidden {
    reason: "Cannot convert boolean to floating-point".to_string(),
}
```

---

### 1.3 Entity: `TypePromotion`

**Purpose**: Describes a specific type conversion operation with metadata.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `from_type` | `IrType` | Source type before promotion | Non-null |
| `to_type` | `IrType` | Target type after promotion | Non-null, different from `from_type` |
| `cast_kind` | `CastKind` | Specific cast operation to perform | Must match `from_type` → `to_type` semantics |
| `span` | `SourceSpan` | Source code location triggering promotion | Non-null |

**Relationships**:
- **Component of**: `PromotionResult` (via `left_cast`, `right_cast` fields)
- **References**: `IrType` (source and target)
- **References**: `CastKind` (cast operation)
- **References**: `SourceSpan` (error reporting)

**Validation Rules**:
- `from_type != to_type` (identity conversions not represented by TypePromotion)
- `cast_kind` must be semantically valid for type pair (e.g., IntToFloat for I32 → F32)

**Example**:
```rust
TypePromotion {
    from_type: IrType::I32,
    to_type: IrType::F64,
    cast_kind: CastKind::IntToFloat,
    span: SourceSpan { start: 42, end: 45, file_id: 1 },
}
```

---

### 1.4 Entity: `PromotionResult`

**Purpose**: Outcome of type promotion analysis for binary operations.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `result_type` | `IrType` | Common type for binary operation | Non-null |
| `left_cast` | `Option<TypePromotion>` | Cast needed for left operand | `None` if left operand already matches `result_type` |
| `right_cast` | `Option<TypePromotion>` | Cast needed for right operand | `None` if right operand already matches `result_type` |
| `warnings` | `Vec<PromotionWarning>` | Potential issues (precision loss, overflow) | Empty if no warnings |
| `is_sound` | `bool` | Whether promotion preserves semantics | `true` for safe promotions |

**Relationships**:
- **Returned by**: `PromotionMatrix::compute_common_type()`, `analyze_promotion()`
- **Contains**: 0-2 `TypePromotion` instances (left/right casts)
- **Contains**: 0-N `PromotionWarning` instances

**Validation Rules**:
- At least one of `left_cast` or `right_cast` must be `Some` (unless both operands already match)
- If `is_sound == false`, `warnings` must contain at least one critical warning
- `result_type` must be valid for the binary operation (e.g., Add requires numeric types)

**Example**:
```rust
// Promotion for I32 + F64 → F64
PromotionResult {
    result_type: IrType::F64,
    left_cast: Some(TypePromotion {
        from_type: IrType::I32,
        to_type: IrType::F64,
        cast_kind: CastKind::IntToFloat,
        span: SourceSpan { ... },
    }),
    right_cast: None, // F64 already matches result_type
    warnings: vec![
        PromotionWarning::PrecisionLoss { 
            from_type: IrType::I32, 
            to_type: IrType::F64, 
            estimated_loss: PrecisionLossEstimate::None 
        }
    ],
    is_sound: true,
}
```

---

### 1.5 Entity: `PromotionWarning`

**Purpose**: Alerts about potential issues in type promotion (precision loss, overflow, etc.).

**Variants** (Enum):
| Variant | Fields | Description |
|---------|--------|-------------|
| `PrecisionLoss` | `from_type: IrType`, `to_type: IrType`, `estimated_loss: PrecisionLossEstimate` | Conversion may lose significant digits |
| `PotentialOverflow` | `from_type: IrType`, `to_type: IrType`, `operation: IrBinaryOp` | Result may exceed target type's range |
| `SignednessChange` | `from_type: IrType`, `to_type: IrType` | Conversion changes signedness (e.g., U32 → I32) |
| `FloatSpecialValues` | `may_produce_nan: bool`, `may_produce_inf: bool` | Operation may produce NaN or Infinity |

**Relationships**:
- **Contained by**: `PromotionResult` (via `warnings` Vec)
- **References**: `IrType`, `IrBinaryOp`, `PrecisionLossEstimate`

**Validation Rules**:
- `from_type` and `to_type` must be different
- `PrecisionLoss`: Only valid for numeric promotions (int → float, large int → small int)
- `PotentialOverflow`: Only valid for narrowing conversions or arithmetic operations

**Example**:
```rust
PromotionWarning::PrecisionLoss {
    from_type: IrType::I64,
    to_type: IrType::F32,
    estimated_loss: PrecisionLossEstimate::SignificantDigits(40), // F32 has 24-bit precision
}
```

---

### 1.6 Entity: `OverflowBehavior`

**Purpose**: Defines strategy for handling integer overflow during type promotions.

**Variants** (Enum):
| Variant | Description | Impact on Promotion |
|---------|-------------|---------------------|
| `Wrap` | Overflow wraps around (modulo arithmetic) | Default; no compile-time errors |
| `Saturate` | Clamps to MIN/MAX of target type | Generates saturation instructions |
| `Trap` | Panic at runtime on overflow | Inserts runtime checks |
| `CompileError` | Reject overflowing conversions at compile time | Returns `Forbidden` rule for risky promotions |

**Relationships**:
- **Configured in**: `PromotionMatrix` (via `overflow_behavior` field)
- **Affects**: `PromotionRule` generation for narrowing conversions

**Validation Rules**:
- Must be set for all `PromotionMatrix` instances (defaults to `Wrap`)

**Example**:
```rust
let matrix = PromotionMatrix::with_overflow_behavior(OverflowBehavior::Trap);
// Now U32 → I32 promotion will insert runtime overflow checks
```

---

### 1.7 Entity: `PrecisionLossEstimate`

**Purpose**: Quantifies magnitude of precision loss in floating-point conversions.

**Variants** (Enum):
| Variant | Description | Example |
|---------|-------------|---------|
| `None` | No precision loss | I8 → F32 (all I8 values fit exactly) |
| `FractionalPart` | Fractional portion lost (float → int) | F32 → I32 (loses decimal digits) |
| `SignificantDigits(u32)` | Loses specified number of significant bits | I64 → F32 (loses 40 bits: 64 - 24) |
| `Total` | Complete loss of value semantics | Bool → F32 (if ever allowed) |

**Relationships**:
- **Used by**: `PromotionWarning::PrecisionLoss` variant

**Validation Rules**:
- `SignificantDigits(n)` where `n > 0`

---

### 1.8 Entity: `TypeGroup`

**Purpose**: Categorizes types into related families for promotion rules.

**Variants** (Enum):
| Variant | Members | Description |
|---------|---------|-------------|
| `SignedIntegers` | I8, I16, I32, I64 | Signed integer types |
| `UnsignedIntegers` | U8, U16, U32, U64 | Unsigned integer types |
| `FloatingPoint` | F32, F64 | IEEE 754 floating-point types |
| `Boolean` | Bool | Single boolean type |
| `Character` | Char | Unicode character type |

**Relationships**:
- **Used by**: `initialize_default_promotions()` to bulk-add promotion rules
- **Derived from**: `IrType` (each type belongs to exactly one group)

**Validation Rules**:
- Groups are disjoint (no type in multiple groups)
- All basic types belong to exactly one group

**Example**:
```rust
// Usage in promotion initialization
for signed_type in TypeGroup::SignedIntegers {
    for float_type in TypeGroup::FloatingPoint {
        add_promotion_rule(signed_type, float_type, PromotionRule::Direct { ... });
    }
}
```

---

## 2. Test Entities (Test Infrastructure)

### 2.1 Entity: `TestCase`

**Purpose**: Represents a single test function validating one aspect of type promotion.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `name` | `String` | Descriptive test function name | Format: `test_<entity>_<operation>_<scenario>_<expected_outcome>` |
| `category` | `TestCategory` | Classification of test type | Enum: Normal \| EdgeCase \| CornerCase \| ErrorHandling |
| `entity_under_test` | `EntityType` | Primary entity being validated | Enum: PromotionMatrix \| PromotionRule \| TypePromotion \| PromotionResult |
| `scenario` | `String` | Description of test scenario | Non-empty, explains what is being tested |
| `expected_outcome` | `ExpectedOutcome` | Expected test result | Enum: Pass \| PanicWith(String) \| ReturnsError(String) |
| `test_body` | `String` (conceptual) | Rust code implementing test | Valid Rust syntax, uses assertions |
| `documentation` | `String` | Rustdoc comment explaining rationale | Explains why test exists, expected behavior |

**Relationships**:
- **Grouped by**: `TestSuite` (multiple TestCases form a suite)
- **Validates**: One or more domain entities (PromotionMatrix, etc.)
- **Uses**: `TestHelper` functions (if applicable)

**Validation Rules**:
- `name` must be unique within test suite
- `name` must follow naming convention (verified by regex `^test_[a-z_]+$`)
- `documentation` must be non-empty (FR-010 requirement)
- `expected_outcome` must match `category` (e.g., ErrorHandling → PanicWith or ReturnsError)

**Example**:
```rust
TestCase {
    name: "test_promotion_matrix_i32_to_f64_direct_cast".to_string(),
    category: TestCategory::Normal,
    entity_under_test: EntityType::PromotionMatrix,
    scenario: "Promotion from I32 to F64 should be direct cast without precision loss".to_string(),
    expected_outcome: ExpectedOutcome::Pass,
    test_body: r#"
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::F64);
        assert!(matches!(rule, Some(PromotionRule::Direct { may_lose_precision: false, .. })));
    "#.to_string(),
    documentation: r#"
        /// Tests that I32 → F64 promotion is direct cast without precision loss.
        ///
        /// # Rationale
        /// F64 has 53-bit significand, which can exactly represent all I32 values.
        /// This test validates no PrecisionLoss warning is generated.
    "#.to_string(),
}
```

---

### 2.2 Entity: `TestCategory`

**Purpose**: Classifies tests by their focus area for organization and coverage tracking.

**Variants** (Enum):
| Variant | Description | Percentage Target (from Technical Context) |
|---------|-------------|-------------------------------------------|
| `Normal` | Standard operational scenarios | 40% of test suite |
| `EdgeCase` | Boundary conditions (MIN/MAX, type boundaries) | 30% of test suite |
| `CornerCase` | Rare combinations of edge conditions | 20% of test suite |
| `ErrorHandling` | Panic tests, invalid inputs, error paths | 10% of test suite |

**Relationships**:
- **Used by**: `TestCase` (via `category` field)
- **Used by**: `CoverageMetric` (for category-specific coverage tracking)

**Validation Rules**:
- Distribution must approximately match target percentages (±5%)

**Example**:
```rust
// Edge case test
TestCase {
    name: "test_i32_max_boundary_to_i64".to_string(),
    category: TestCategory::EdgeCase, // ← Boundary value test
    entity_under_test: EntityType::PromotionMatrix,
    scenario: "I32::MAX promotes to I64 without overflow".to_string(),
    // ...
}
```

---

### 2.3 Entity: `TestSuite`

**Purpose**: Collection of related test cases targeting the type promotion module.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `name` | `String` | Test suite identifier | Fixed: `"ir_type_promotion_tests"` |
| `test_cases` | `Vec<TestCase>` | All test functions in suite | Length: 40-60 tests (per Technical Context) |
| `coverage_metrics` | `CoverageMetric` | Line/branch/function coverage data | Target: 100% line coverage |
| `execution_time` | `Duration` | Total time to run all tests | Target: <5 seconds |

**Relationships**:
- **Contains**: Multiple `TestCase` instances
- **Measures**: `CoverageMetric` for validation
- **Located in**: `tests/ir_type_promotion_tests.rs` file

**Validation Rules**:
- `test_cases.len()` >= 40 and <= 60 (estimated range)
- `coverage_metrics.line_coverage` == 100.0% (strict requirement)
- `execution_time` < Duration::from_secs(5) (performance target)
- Test category distribution:
  - Normal: 40% ± 5%
  - EdgeCase: 30% ± 5%
  - CornerCase: 20% ± 5%
  - ErrorHandling: 10% ± 5%

**Example**:
```rust
TestSuite {
    name: "ir_type_promotion_tests".to_string(),
    test_cases: vec![
        TestCase { name: "test_promotion_matrix_new", ... },
        TestCase { name: "test_i32_to_f64_direct", ... },
        // ... 38-58 more tests
    ],
    coverage_metrics: CoverageMetric {
        line_coverage: 100.0,
        function_coverage: 100.0,
        branch_coverage: 96.2,
    },
    execution_time: Duration::from_millis(3200), // ✅ <5s
}
```

---

### 2.4 Entity: `CoverageMetric`

**Purpose**: Tracks code coverage achieved by test suite.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `line_coverage` | `f64` | Percentage of lines executed | Range: [0.0, 100.0], Target: 100.0 |
| `function_coverage` | `f64` | Percentage of functions called | Range: [0.0, 100.0], Target: 100.0 |
| `branch_coverage` | `f64` | Percentage of branches taken | Range: [0.0, 100.0], Target: ≥95.0 |
| `uncovered_lines` | `Vec<u32>` | Line numbers not executed | Empty if `line_coverage == 100.0` |
| `uncovered_functions` | `Vec<String>` | Function names not called | Empty if `function_coverage == 100.0` |

**Relationships**:
- **Measured by**: `cargo llvm-cov` tool
- **Validated by**: CI/CD pipeline (fails if line_coverage < 100.0)
- **Component of**: `TestSuite` (via `coverage_metrics` field)

**Validation Rules**:
- `line_coverage` MUST be 100.0 (per FR-001, Clarifications Q1)
- `function_coverage` MUST be 100.0 (all public functions tested)
- If `line_coverage < 100.0`, then `uncovered_lines.len() > 0`
- Coverage percentages must be internally consistent (line ≥ function)

**Example**:
```rust
CoverageMetric {
    line_coverage: 100.0, // ✅ Target achieved
    function_coverage: 100.0,
    branch_coverage: 97.5, // ✅ Above 95% threshold
    uncovered_lines: vec![], // ✅ All lines covered
    uncovered_functions: vec![], // ✅ All functions tested
}
```

---

### 2.5 Entity: `ExpectedOutcome`

**Purpose**: Defines the expected result of running a test case.

**Variants** (Enum):
| Variant | Description | Test Attribute |
|---------|-------------|---------------|
| `Pass` | Test should complete successfully | `#[test]` |
| `PanicWith(String)` | Test should panic with specific message | `#[test]` + `#[should_panic(expected = "...")]` |
| `ReturnsError(String)` | Function returns Err with specific message | `#[test]` + assertion on Result |

**Relationships**:
- **Used by**: `TestCase` (via `expected_outcome` field)
- **Affects**: Test function attributes (e.g., `#[should_panic]`)

**Validation Rules**:
- `PanicWith(msg)` must have non-empty `msg`
- `ReturnsError(msg)` must have non-empty `msg`

**Example**:
```rust
// Normal pass test
ExpectedOutcome::Pass

// Panic test
ExpectedOutcome::PanicWith("Invalid type combination".to_string())
// Generates: #[should_panic(expected = "Invalid type combination")]

// Error result test
ExpectedOutcome::ReturnsError("Circular promotion detected".to_string())
// Generates: assert!(result.is_err_and(|e| e.contains("Circular promotion detected")))
```

---

### 2.6 Entity: `EntityType`

**Purpose**: Identifies which production code entity is under test.

**Variants** (Enum):
| Variant | Corresponding Production Entity |
|---------|--------------------------------|
| `PromotionMatrix` | `src/ir/type_promotion.rs::PromotionMatrix` |
| `PromotionRule` | `src/ir/type_promotion.rs::PromotionRule` |
| `TypePromotion` | `src/ir/type_promotion.rs::TypePromotion` |
| `PromotionResult` | `src/ir/type_promotion.rs::PromotionResult` |
| `PromotionWarning` | `src/ir/type_promotion.rs::PromotionWarning` |
| `OverflowBehavior` | `src/ir/type_promotion.rs::OverflowBehavior` |
| `PrecisionLossEstimate` | `src/ir/type_promotion.rs::PrecisionLossEstimate` |
| `TypeGroup` | `src/ir/type_promotion.rs::TypeGroup` |

**Relationships**:
- **Used by**: `TestCase` (via `entity_under_test` field)
- **Maps to**: Production code entities (1:1 correspondence)

**Validation Rules**:
- All production entities must have at least one test case targeting them

---

### 2.7 Entity: `TestHelper`

**Purpose**: Reusable test utility functions to reduce duplication.

**Fields**:
| Field Name | Type | Description | Constraints |
|------------|------|-------------|-------------|
| `name` | `String` | Helper function name | Snake_case, no `test_` prefix |
| `parameters` | `Vec<(String, String)>` | (param_name, param_type) pairs | Non-empty for parameterized helpers |
| `return_type` | `String` | Rust return type | Non-empty (e.g., `"IrType"`, `"PromotionMatrix"`) |
| `body` | `String` | Function implementation | Valid Rust code |

**Relationships**:
- **Used by**: Multiple `TestCase` instances
- **Located in**: `tests/ir_type_promotion_tests.rs` (above test functions)

**Validation Rules**:
- `name` must not start with `test_` (not a test function)
- Must be used by at least 2 test cases (otherwise inline)

**Example**:
```rust
TestHelper {
    name: "create_matrix_with_custom_overflow".to_string(),
    parameters: vec![
        ("behavior".to_string(), "OverflowBehavior".to_string()),
    ],
    return_type: "PromotionMatrix".to_string(),
    body: r#"
        PromotionMatrix::with_overflow_behavior(behavior)
    "#.to_string(),
}

// Usage in tests:
#[test]
fn test_overflow_behavior_wrap() {
    let matrix = create_matrix_with_custom_overflow(OverflowBehavior::Wrap);
    // ...
}
```

---

## 3. Relationships Diagram

```
┌─────────────────────────┐
│   PromotionMatrix       │◄────┐
│  (rules, overflow_beh.) │     │
└────────┬────────────────┘     │
         │ 1                     │
         │ owns                  │
         │ N                     │
         ▼                       │
┌─────────────────────────┐     │
│   PromotionRule         │     │ uses
│  (Direct/Indirect/      │     │
│   Forbidden)            │     │
└─────────────────────────┘     │
         │                       │
         │ used in               │
         ▼                       │
┌─────────────────────────┐     │
│   PromotionResult       │     │
│  (result_type, casts,   │     │
│   warnings)             │     │
└────────┬────────────────┘     │
         │ 0..2                  │
         │ contains              │
         │                       │
         ▼                       │
┌─────────────────────────┐     │
│   TypePromotion         │     │
│  (from, to, cast_kind)  │     │
└─────────────────────────┘     │
                                │
┌─────────────────────────┐     │
│   TestSuite             │     │
│  (test_cases,           │     │
│   coverage_metrics)     │     │
└────────┬────────────────┘     │
         │ N                     │
         │ contains              │
         │                       │
         ▼                       │
┌─────────────────────────┐     │
│   TestCase              │     │
│  (name, category,       │     │
│   entity_under_test)    │─────┘
└─────────────────────────┘   validates
```

---

## 4. Validation Rules Summary

### 4.1 Domain Entity Constraints

| Entity | Constraint | Enforcement |
|--------|-----------|-------------|
| `PromotionMatrix` | Must have identity rules for all types | Runtime assertion in `new()` |
| `PromotionRule::Indirect` | `intermediate_type` ≠ source ≠ target | Compile-time type check |
| `PromotionResult` | At least one cast or both operands match | Logic assertion in `compute_common_type()` |
| `OverflowBehavior` | Must be set for all matrices | Default value in `new()` |

### 4.2 Test Entity Constraints

| Entity | Constraint | Enforcement |
|--------|-----------|-------------|
| `TestSuite` | 100% line coverage | CI/CD gate with `cargo llvm-cov` |
| `TestCase.name` | Unique + follows naming convention | Code review + regex validation |
| `TestCase.documentation` | Non-empty rustdoc comment | FR-010 requirement, code review |
| `TestCategory` distribution | 40% Normal, 30% Edge, 20% Corner, 10% Error (±5%) | Manual review during planning |

---

## 5. State Transition Diagrams

### 5.1 PromotionMatrix Lifecycle

```
[Uninitialized]
      │
      │ PromotionMatrix::new()
      ▼
[Default Rules Loaded]
      │
      │ add_promotion_rule() / add_symmetric_promotion_rule()
      ▼
[Custom Rules Added]
      │
      │ get_promotion_rule() / compute_common_type()
      ▼
[Active Use]
```

### 5.2 TestCase Execution

```
[Not Started]
      │
      │ cargo test <test_name>
      ▼
[Running]
      │
      ├─ Expected: Pass ──────────►[Passed ✓]
      ├─ Expected: PanicWith ─────►[Panicked with message ✓]
      ├─ Expected: ReturnsError ──►[Error validated ✓]
      │
      ├─ Unexpected panic ────────►[Failed ✗]
      ├─ Assertion failed ────────►[Failed ✗]
      └─ Timeout (>100ms) ────────►[Failed ✗ (Performance)]
```

---

## 6. Enums Reference

### 6.1 Domain Enums

**`IrType`** (from `src/ir/types.rs`):
```rust
pub enum IrType {
    I8, I16, I32, I64,      // Signed integers
    U8, U16, U32, U64,      // Unsigned integers
    F32, F64,               // IEEE 754 floats
    Bool,                   // Boolean
    Char,                   // Unicode character
    // ... (other types omitted for brevity)
}
```

**`CastKind`** (from `src/ir/instruction.rs`):
```rust
pub enum CastKind {
    Bitcast,        // Reinterpret bits (no conversion)
    ZeroExtend,     // Unsigned widening
    SignExtend,     // Signed widening
    Truncate,       // Narrowing (loses high bits)
    IntToFloat,     // Integer to floating-point
    FloatToInt,     // Floating-point to integer (loses fractional part)
    FloatTruncate,  // F64 → F32 (loses precision)
    FloatExtend,    // F32 → F64 (exact)
}
```

### 6.2 Test Enums

**`TestCategory`**:
```rust
pub enum TestCategory {
    Normal,         // 40% of tests
    EdgeCase,       // 30% of tests
    CornerCase,     // 20% of tests
    ErrorHandling,  // 10% of tests
}
```

**`ExpectedOutcome`**:
```rust
pub enum ExpectedOutcome {
    Pass,                     // #[test]
    PanicWith(String),        // #[should_panic(expected = "...")]
    ReturnsError(String),     // assert!(result.is_err())
}
```

---

## 7. Example Data Flow

### Scenario: Testing I32 + F64 Promotion

**1. Test Setup**:
```rust
#[test]
fn test_i32_f64_addition_promotes_to_f64() {
    let matrix = PromotionMatrix::new(); // ← PromotionMatrix entity
    
    // Inputs
    let left_type = IrType::I32;
    let right_type = IrType::F64;
    let operation = IrBinaryOp::Add;
```

**2. Promotion Analysis**:
```rust
    let result = matrix.compute_common_type(&left_type, &right_type);
    // ↑ Returns PromotionResult entity
```

**3. Result Validation**:
```rust
    // Validate PromotionResult fields
    assert_eq!(result.result_type, IrType::F64);
    
    // Validate TypePromotion for left operand
    assert!(result.left_cast.is_some());
    let left_cast = result.left_cast.unwrap();
    assert_eq!(left_cast.from_type, IrType::I32);
    assert_eq!(left_cast.to_type, IrType::F64);
    assert_eq!(left_cast.cast_kind, CastKind::IntToFloat);
    
    // Validate no cast needed for right operand
    assert!(result.right_cast.is_none());
    
    // Validate warnings
    assert!(result.warnings.is_empty()); // I32 fits exactly in F64
}
```

**4. Coverage Tracking**:
```bash
# After test execution
cargo llvm-cov --package jsavrs --lib --text
# Reports CoverageMetric:
# src/ir/type_promotion.rs:42 (compute_common_type): COVERED ✓
```

---

## 8. Design Principles Applied

### 8.1 Single Responsibility
- **PromotionMatrix**: Manages rules only
- **PromotionResult**: Represents outcome only
- **TestCase**: Validates one aspect only

### 8.2 Immutability
- **PromotionRule**: Enum variants are immutable
- **TypePromotion**: Value object (no mutation after creation)
- **TestCase**: Conceptually immutable (defined once, executed many times)

### 8.3 Type Safety
- **Enums over strings**: `TestCategory` enum instead of `"edge_case"` strings
- **Constrained types**: `CoverageMetric.line_coverage` is `f64` (not unbounded integer)
- **Validation rules**: Enforced at compile time (enum exhaustiveness) or runtime (assertions)

---

## Conclusion

This data model provides a complete, detailed, precise, and meticulous specification of all entities involved in the type promotion test suite. It serves as:

1. **Blueprint for Implementation**: Clear entity definitions with fields, constraints, and relationships
2. **Validation Checklist**: Explicit validation rules for test correctness
3. **Documentation**: Enums, state transitions, and example data flows for maintainability
4. **Coverage Reference**: Links entities to coverage targets and test categories

**Next Steps** (Phase 1 continuation):
- Create `contracts/` directory with test function signatures (if applicable)
- Generate `quickstart.md` with example test execution workflow
- Update `QWEN.md` agent context with entity knowledge via `update-agent-context.ps1`

**Estimated Entity Count**:
- **Domain Entities**: 8 (PromotionMatrix, PromotionRule, TypePromotion, PromotionResult, PromotionWarning, OverflowBehavior, PrecisionLossEstimate, TypeGroup)
- **Test Entities**: 7 (TestCase, TestCategory, TestSuite, CoverageMetric, ExpectedOutcome, EntityType, TestHelper)
- **Total**: 15 entities with 50+ fields and 20+ validation rules
