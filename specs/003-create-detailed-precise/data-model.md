# Data Model: Comprehensive Type Promotion Engine Test Suite

**Date**: October 6, 2025  
**Feature**: Comprehensive Type Promotion Engine Test Suite  
**Status**: Complete

## Executive Summary

This document defines the data model for the comprehensive test suite covering the TypePromotionEngine module. The model includes test fixture structures, type combination matrices, operation coverage matrices, test helper utilities, and expected output structures that support 100% coverage testing with detailed edge and corner case validation.

---

## 1. Core Test Entities

### 1.1 TypePromotionEngine (System Under Test)

```rust
pub struct TypePromotionEngine {
    pub promotion_matrix: PromotionMatrix,
}
```

**Description**: The primary system being tested. Analyzes type promotions for binary operations and inserts necessary cast instructions.

**Key Methods**:
- `new() -> Self`: Creates engine with default PromotionMatrix
- `analyze_binary_promotion(&self, left_type: &IrType, right_type: &IrType, operation: IrBinaryOp, span: SourceSpan) -> PromotionResult`
- `insert_promotion_casts(&self, generator: &mut NIrGenerator, func: &mut Function, left_value: Value, right_value: Value, promotion_result: &PromotionResult, span: SourceSpan) -> (Value, Value)`

---

### 1.2 PromotionResult (Output Entity)

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PromotionResult {
    pub result_type: IrType,
    pub left_cast: Option<TypePromotion>,
    pub right_cast: Option<TypePromotion>,
    pub warnings: Vec<PromotionWarning>,
    pub is_sound: bool,
}
```

**Description**: Result of type promotion analysis containing target type, required casts, and warnings.

**Test Validation Fields**:
- `result_type`: Critical property (explicit assertion)
- `left_cast` / `right_cast`: Cast presence (explicit assertion), structure (snapshot)
- `warnings`: Count (explicit assertion), content (snapshot for complex cases)
- `is_sound`: Critical property (explicit assertion)

---

### 1.3 TypePromotion (Cast Information)

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TypePromotion {
    pub from_type: IrType,
    pub to_type: IrType,
    pub cast_kind: CastKind,
    pub may_lose_precision: bool,
    pub may_overflow: bool,
    pub source_span: SourceSpan,
}
```

**Description**: Describes a single type conversion with cast metadata.

**Test Validation**:
- Verify correct `from_type` and `to_type` pairs
- Validate appropriate `cast_kind` selection (IntToFloat, FloatToInt, etc.)
- Check `may_lose_precision` and `may_overflow` flags match expected scenarios
- Ensure `source_span` preservation

---

### 1.4 PromotionWarning (Diagnostic Entity)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PromotionWarning {
    PrecisionLoss {
        from_type: IrType,
        to_type: IrType,
        estimated_loss: PrecisionLossEstimate,
    },
    PotentialOverflow {
        from_type: IrType,
        to_type: IrType,
        operation: IrBinaryOp,
    },
    SignednessChange {
        from_signed: bool,
        to_signed: bool,
        may_affect_comparisons: bool,
    },
    FloatSpecialValues {
        operation: IrBinaryOp,
        may_produce_nan: bool,
        may_produce_infinity: bool,
    },
}
```

**Description**: Warnings generated during type promotion analysis.

**Test Validation**:
- **PrecisionLoss**: Verify correct type pair, validate `estimated_loss` calculation
- **PotentialOverflow**: Check appropriate triggering conditions (overflow-prone operations)
- **SignednessChange**: Validate detection of mixed signed/unsigned scenarios
- **FloatSpecialValues**: Ensure special float handling is documented

---

### 1.5 IrType (Type Enumeration)

```rust
pub enum IrType {
    // Integer types (8 variants)
    I8, I16, I32, I64,    // Signed
    U8, U16, U32, U64,    // Unsigned
    
    // Floating-point types (2 variants)
    F32, F64,
    
    // Other types (2 variants)
    Bool,
    Char,
    
    // ... other variants not tested in this suite
}
```

**Description**: All intermediate representation types. Test suite covers all 12 numeric and basic types.

**Helper Methods (tested through engine usage)**:
- `is_signed_integer(&self) -> bool`: Matches I8, I16, I32, I64
- `is_unsigned_integer(&self) -> bool`: Matches U8, U16, U32, U64
- `get_bit_width(&self) -> u32`: Returns bit width (8, 16, 32, 64) or default 32

---

### 1.6 IrBinaryOp (Operation Enumeration)

```rust
pub enum IrBinaryOp {
    // Arithmetic (5 variants)
    Add, Subtract, Multiply, Divide, Modulo,
    
    // Comparison (6 variants)
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual,
    
    // Logical (2 variants)
    And, Or,
    
    // Bitwise (5 variants)
    BitwiseAnd, BitwiseOr, BitwiseXor, ShiftLeft, ShiftRight,
}
```

**Description**: All binary operations requiring type promotion. Total: 18 operations.

**Test Coverage**: Each operation tested with multiple type combinations (same-type, widening, mixed-signedness).

---

### 1.7 CastKind (Cast Type Enumeration)

```rust
pub enum CastKind {
    Bitcast,           // No actual conversion (same type)
    IntSignExtend,     // Signed integer widening
    IntZeroExtend,     // Unsigned integer widening
    IntTruncate,       // Integer narrowing
    IntToFloat,        // Integer to floating-point
    FloatToInt,        // Floating-point to integer
    FloatExtend,       // F32 to F64
    FloatTruncate,     // F64 to F32
    // ... other cast kinds
}
```

**Description**: Type conversion operations.

**Test Validation**: Verify correct `CastKind` selection for each type pair in promotions.

---

## 2. Test Fixture Structures

### 2.1 TypePairTestCase (Test Input Structure)

```rust
/// Test case for a specific type pair promotion scenario
struct TypePairTestCase {
    left_type: IrType,
    right_type: IrType,
    operation: IrBinaryOp,
    expected_result_type: IrType,
    expected_left_cast: Option<CastKind>,
    expected_right_cast: Option<CastKind>,
    expected_warnings: Vec<WarningType>,
    description: &'static str,
}

enum WarningType {
    PrecisionLoss,
    PotentialOverflow,
    SignednessChange,
    FloatSpecialValues,
}
```

**Usage**:
```rust
let test_case = TypePairTestCase {
    left_type: IrType::I32,
    right_type: IrType::F32,
    operation: IrBinaryOp::Add,
    expected_result_type: IrType::F32,
    expected_left_cast: Some(CastKind::IntToFloat),
    expected_right_cast: None,
    expected_warnings: vec![],
    description: "I32 + F32 should promote to F32 with IntToFloat cast on left",
};
```

**Purpose**: Provides structured input and expected output for parameterized tests.

---

### 2.2 OperationTestMatrix (Operation Coverage Structure)

```rust
/// Matrix of operation types with test scenarios
struct OperationTestMatrix {
    arithmetic_ops: Vec<IrBinaryOp>,      // Add, Subtract, Multiply, Divide, Modulo
    comparison_ops: Vec<IrBinaryOp>,      // Equal, NotEqual, Less, etc.
    logical_ops: Vec<IrBinaryOp>,         // And, Or
    bitwise_ops: Vec<IrBinaryOp>,         // BitwiseAnd, BitwiseOr, etc.
}

impl OperationTestMatrix {
    fn all_operations() -> Vec<IrBinaryOp> {
        vec![
            // Arithmetic
            IrBinaryOp::Add, IrBinaryOp::Subtract, IrBinaryOp::Multiply,
            IrBinaryOp::Divide, IrBinaryOp::Modulo,
            
            // Comparison
            IrBinaryOp::Equal, IrBinaryOp::NotEqual,
            IrBinaryOp::Less, IrBinaryOp::LessEqual,
            IrBinaryOp::Greater, IrBinaryOp::GreaterEqual,
            
            // Logical
            IrBinaryOp::And, IrBinaryOp::Or,
            
            // Bitwise
            IrBinaryOp::BitwiseAnd, IrBinaryOp::BitwiseOr,
            IrBinaryOp::BitwiseXor, IrBinaryOp::ShiftLeft,
            IrBinaryOp::ShiftRight,
        ]
    }
}
```

**Purpose**: Systematically test all operation categories with representative type combinations.

---

### 2.3 TypeCombinationMatrix (Type Coverage Structure)

```rust
/// Matrix of all type combinations requiring testing
struct TypeCombinationMatrix {
    // Identity combinations (12 cases)
    identity_pairs: Vec<(IrType, IrType)>,
    
    // Widening combinations (24 cases: signed + unsigned)
    widening_signed: Vec<(IrType, IrType)>,      // I8→I16, I8→I32, etc.
    widening_unsigned: Vec<(IrType, IrType)>,    // U8→U16, U8→U32, etc.
    
    // Narrowing combinations (24 cases)
    narrowing_signed: Vec<(IrType, IrType)>,     // I64→I32, I32→I16, etc.
    narrowing_unsigned: Vec<(IrType, IrType)>,   // U64→U32, U32→U16, etc.
    
    // Cross-signedness combinations (4 cases)
    cross_signedness: Vec<(IrType, IrType)>,     // I8↔U8, I16↔U16, etc.
    
    // Integer-float combinations (32 cases: 16 int→float + 16 float→int)
    integer_to_float: Vec<(IrType, IrType)>,
    float_to_integer: Vec<(IrType, IrType)>,
    
    // Float promotions (2 cases)
    float_widening: (IrType, IrType),            // F32→F64
    float_narrowing: (IrType, IrType),           // F64→F32
}

impl TypeCombinationMatrix {
    fn new() -> Self {
        let signed_integers = vec![IrType::I8, IrType::I16, IrType::I32, IrType::I64];
        let unsigned_integers = vec![IrType::U8, IrType::U16, IrType::U32, IrType::U64];
        let floats = vec![IrType::F32, IrType::F64];
        let all_integers = [signed_integers.clone(), unsigned_integers.clone()].concat();
        
        // Generate identity pairs
        let identity_pairs = vec![
            (IrType::I8, IrType::I8), (IrType::I16, IrType::I16),
            (IrType::I32, IrType::I32), (IrType::I64, IrType::I64),
            (IrType::U8, IrType::U8), (IrType::U16, IrType::U16),
            (IrType::U32, IrType::U32), (IrType::U64, IrType::U64),
            (IrType::F32, IrType::F32), (IrType::F64, IrType::F64),
            (IrType::Bool, IrType::Bool), (IrType::Char, IrType::Char),
        ];
        
        // Generate widening signed pairs
        let widening_signed = vec![
            (IrType::I8, IrType::I16), (IrType::I8, IrType::I32), (IrType::I8, IrType::I64),
            (IrType::I16, IrType::I32), (IrType::I16, IrType::I64),
            (IrType::I32, IrType::I64),
        ];
        
        // Generate widening unsigned pairs
        let widening_unsigned = vec![
            (IrType::U8, IrType::U16), (IrType::U8, IrType::U32), (IrType::U8, IrType::U64),
            (IrType::U16, IrType::U32), (IrType::U16, IrType::U64),
            (IrType::U32, IrType::U64),
        ];
        
        // ... similar for narrowing, cross-signedness, etc.
        
        Self {
            identity_pairs,
            widening_signed,
            widening_unsigned,
            narrowing_signed: widening_signed.iter().map(|(a, b)| (*b, *a)).collect(),
            narrowing_unsigned: widening_unsigned.iter().map(|(a, b)| (*b, *a)).collect(),
            cross_signedness: vec![
                (IrType::I8, IrType::U8), (IrType::I16, IrType::U16),
                (IrType::I32, IrType::U32), (IrType::I64, IrType::U64),
            ],
            integer_to_float: all_integers.iter()
                .flat_map(|&int_type| floats.iter().map(move |&float_type| (int_type, float_type)))
                .collect(),
            float_to_integer: floats.iter()
                .flat_map(|&float_type| all_integers.iter().map(move |&int_type| (float_type, int_type)))
                .collect(),
            float_widening: (IrType::F32, IrType::F64),
            float_narrowing: (IrType::F64, IrType::F32),
        }
    }
}
```

**Purpose**: Systematically generate all relevant type pair combinations for comprehensive coverage.

---

### 2.4 EdgeCaseScenario (Edge Case Test Structure)

```rust
/// Edge case test scenario with special conditions
struct EdgeCaseScenario {
    scenario_name: &'static str,
    left_type: IrType,
    right_type: IrType,
    left_value: Option<i64>,   // Specific value for boundary testing (e.g., i32::MAX)
    right_value: Option<i64>,
    operation: IrBinaryOp,
    expected_warnings: Vec<WarningType>,
    test_rationale: &'static str,
}
```

**Examples**:
```rust
// Boundary value test
EdgeCaseScenario {
    scenario_name: "i32_max_to_i64_no_overflow",
    left_type: IrType::I32,
    right_type: IrType::I64,
    left_value: Some(i32::MAX as i64),
    right_value: None,
    operation: IrBinaryOp::Add,
    expected_warnings: vec![],
    test_rationale: "i32::MAX promotes safely to i64 without overflow",
}

// Float special value test
EdgeCaseScenario {
    scenario_name: "float_nan_to_integer_special_handling",
    left_type: IrType::F32,
    right_type: IrType::I32,
    left_value: None,  // NaN represented separately
    right_value: None,
    operation: IrBinaryOp::Add,
    expected_warnings: vec![WarningType::FloatSpecialValues],
    test_rationale: "NaN in float operations requires special value warning",
}
```

**Purpose**: Document and test boundary conditions and special value scenarios.

---

### 2.5 ConcurrentTestScenario (Concurrency Test Structure)

```rust
/// Concurrent execution test scenario
struct ConcurrentTestScenario {
    thread_count: usize,
    operations_per_thread: usize,
    type_combinations: Vec<(IrType, IrType, IrBinaryOp)>,
    expected_consistent_results: bool,
}
```

**Example**:
```rust
ConcurrentTestScenario {
    thread_count: 10,
    operations_per_thread: 100,
    type_combinations: vec![
        (IrType::I32, IrType::F32, IrBinaryOp::Add),
        (IrType::U64, IrType::I64, IrBinaryOp::Multiply),
        (IrType::F32, IrType::F64, IrBinaryOp::Divide),
    ],
    expected_consistent_results: true,
}
```

**Purpose**: Validate thread-safety with concurrent reads of TypePromotionEngine.

---

## 3. Test Helper Structures

### 3.1 TestFixtureBuilder (Builder Pattern)

```rust
/// Builder for constructing test fixtures with default values
struct TestFixtureBuilder {
    engine: TypePromotionEngine,
    span: SourceSpan,
    generator: Option<NIrGenerator>,
    function: Option<Function>,
}

impl TestFixtureBuilder {
    fn new() -> Self {
        Self {
            engine: TypePromotionEngine::new(),
            span: SourceSpan::default(),
            generator: None,
            function: None,
        }
    }
    
    fn with_engine(mut self, engine: TypePromotionEngine) -> Self {
        self.engine = engine;
        self
    }
    
    fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = span;
        self
    }
    
    fn with_ir_context(mut self, generator: NIrGenerator, function: Function) -> Self {
        self.generator = Some(generator);
        self.function = Some(function);
        self
    }
    
    fn build(self) -> TestFixture {
        TestFixture {
            engine: self.engine,
            span: self.span,
            generator: self.generator,
            function: self.function,
        }
    }
}

struct TestFixture {
    engine: TypePromotionEngine,
    span: SourceSpan,
    generator: Option<NIrGenerator>,
    function: Option<Function>,
}
```

**Purpose**: Simplify test setup with consistent default values and optional customization.

---

### 3.2 MockPromotionMatrix (Mocking Structure)

```rust
/// Mock PromotionMatrix for unit testing TypePromotionEngine in isolation
struct MockPromotionMatrix {
    rules: HashMap<(IrType, IrType), PromotionRule>,
    common_types: HashMap<(IrType, IrType), Option<IrType>>,
    overflow_behavior: OverflowBehavior,
}

impl MockPromotionMatrix {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
            common_types: HashMap::new(),
            overflow_behavior: OverflowBehavior::Saturate,
        }
    }
    
    fn set_rule(&mut self, from: IrType, to: IrType, rule: PromotionRule) {
        self.rules.insert((from, to), rule);
    }
    
    fn set_common_type(&mut self, left: IrType, right: IrType, result: Option<IrType>) {
        self.common_types.insert((left, right), result);
    }
    
    fn get_promotion_rule(&self, from: &IrType, to: &IrType) -> Option<&PromotionRule> {
        self.rules.get(&(from.clone(), to.clone()))
    }
    
    fn compute_common_type(&self, left: &IrType, right: &IrType) -> Option<IrType> {
        self.common_types.get(&(left.clone(), right.clone())).cloned().flatten()
    }
}
```

**Purpose**: Enable unit testing of TypePromotionEngine without dependency on real PromotionMatrix implementation.

---

### 3.3 AssertionHelpers (Validation Utilities)

```rust
/// Helper functions for common test assertions
mod assertion_helpers {
    use super::*;
    
    /// Assert that promotion result has expected type
    fn assert_result_type(result: &PromotionResult, expected: IrType, context: &str) {
        assert_eq!(
            result.result_type, expected,
            "{}: Expected result type {:?}, got {:?}",
            context, expected, result.result_type
        );
    }
    
    /// Assert that left cast is present with specific cast kind
    fn assert_left_cast(result: &PromotionResult, expected_kind: CastKind, context: &str) {
        assert!(
            result.left_cast.is_some(),
            "{}: Expected left cast, got None",
            context
        );
        let cast = result.left_cast.as_ref().unwrap();
        assert_eq!(
            cast.cast_kind, expected_kind,
            "{}: Expected left cast kind {:?}, got {:?}",
            context, expected_kind, cast.cast_kind
        );
    }
    
    /// Assert that specific warning type is present
    fn assert_has_warning(result: &PromotionResult, warning_type: WarningType, context: &str) {
        let has_warning = match warning_type {
            WarningType::PrecisionLoss => result.warnings.iter().any(|w| matches!(w, PromotionWarning::PrecisionLoss { .. })),
            WarningType::PotentialOverflow => result.warnings.iter().any(|w| matches!(w, PromotionWarning::PotentialOverflow { .. })),
            WarningType::SignednessChange => result.warnings.iter().any(|w| matches!(w, PromotionWarning::SignednessChange { .. })),
            WarningType::FloatSpecialValues => result.warnings.iter().any(|w| matches!(w, PromotionWarning::FloatSpecialValues { .. })),
        };
        assert!(
            has_warning,
            "{}: Expected warning {:?}, but not found in warnings: {:?}",
            context, warning_type, result.warnings
        );
    }
    
    /// Assert that result is sound (no warnings or explicitly sound despite warnings)
    fn assert_is_sound(result: &PromotionResult, expected: bool, context: &str) {
        assert_eq!(
            result.is_sound, expected,
            "{}: Expected is_sound={}, got {}",
            context, expected, result.is_sound
        );
    }
}
```

**Purpose**: Provide reusable, expressive assertion functions for common validation patterns.

---

## 4. Expected Output Structures

### 4.1 ExpectedPromotionResult (Test Oracle)

```rust
/// Expected output for a promotion analysis test
struct ExpectedPromotionResult {
    result_type: IrType,
    left_cast_present: bool,
    left_cast_kind: Option<CastKind>,
    right_cast_present: bool,
    right_cast_kind: Option<CastKind>,
    warning_count: usize,
    warning_types: Vec<WarningType>,
    is_sound: bool,
}

impl ExpectedPromotionResult {
    /// Validate actual PromotionResult against expected values
    fn validate(&self, actual: &PromotionResult, test_name: &str) {
        assertion_helpers::assert_result_type(actual, self.result_type, test_name);
        
        if self.left_cast_present {
            assert!(actual.left_cast.is_some(), "{}: Expected left cast", test_name);
            if let Some(kind) = self.left_cast_kind {
                assertion_helpers::assert_left_cast(actual, kind, test_name);
            }
        } else {
            assert!(actual.left_cast.is_none(), "{}: Expected no left cast", test_name);
        }
        
        assert_eq!(
            actual.warnings.len(), self.warning_count,
            "{}: Expected {} warnings, got {}",
            test_name, self.warning_count, actual.warnings.len()
        );
        
        for warning_type in &self.warning_types {
            assertion_helpers::assert_has_warning(actual, *warning_type, test_name);
        }
        
        assertion_helpers::assert_is_sound(actual, self.is_sound, test_name);
    }
}
```

**Purpose**: Encapsulate expected test outcomes for systematic validation.

---

## 5. Test Data Constants

### 5.1 Type Constants

```rust
/// Commonly used type constants for tests
mod test_types {
    use super::*;
    
    pub const ALL_SIGNED_INTEGERS: [IrType; 4] = [IrType::I8, IrType::I16, IrType::I32, IrType::I64];
    pub const ALL_UNSIGNED_INTEGERS: [IrType; 4] = [IrType::U8, IrType::U16, IrType::U32, IrType::U64];
    pub const ALL_FLOATS: [IrType; 2] = [IrType::F32, IrType::F64];
    pub const ALL_INTEGERS: [IrType; 8] = [
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
    ];
    pub const ALL_NUMERIC: [IrType; 10] = [
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
        IrType::F32, IrType::F64,
    ];
    pub const ALL_TYPES: [IrType; 12] = [
        IrType::I8, IrType::I16, IrType::I32, IrType::I64,
        IrType::U8, IrType::U16, IrType::U32, IrType::U64,
        IrType::F32, IrType::F64,
        IrType::Bool, IrType::Char,
    ];
}
```

**Purpose**: Provide constants for iterating over type categories in parameterized tests.

---

### 5.2 Operation Constants

```rust
/// Commonly used operation constants for tests
mod test_operations {
    use super::*;
    
    pub const ARITHMETIC_OPS: [IrBinaryOp; 5] = [
        IrBinaryOp::Add, IrBinaryOp::Subtract, IrBinaryOp::Multiply,
        IrBinaryOp::Divide, IrBinaryOp::Modulo,
    ];
    
    pub const COMPARISON_OPS: [IrBinaryOp; 6] = [
        IrBinaryOp::Equal, IrBinaryOp::NotEqual,
        IrBinaryOp::Less, IrBinaryOp::LessEqual,
        IrBinaryOp::Greater, IrBinaryOp::GreaterEqual,
    ];
    
    pub const LOGICAL_OPS: [IrBinaryOp; 2] = [IrBinaryOp::And, IrBinaryOp::Or];
    
    pub const BITWISE_OPS: [IrBinaryOp; 5] = [
        IrBinaryOp::BitwiseAnd, IrBinaryOp::BitwiseOr,
        IrBinaryOp::BitwiseXor, IrBinaryOp::ShiftLeft,
        IrBinaryOp::ShiftRight,
    ];
    
    pub const ALL_OPS: [IrBinaryOp; 18] = [
        // Arithmetic
        IrBinaryOp::Add, IrBinaryOp::Subtract, IrBinaryOp::Multiply,
        IrBinaryOp::Divide, IrBinaryOp::Modulo,
        // Comparison
        IrBinaryOp::Equal, IrBinaryOp::NotEqual,
        IrBinaryOp::Less, IrBinaryOp::LessEqual,
        IrBinaryOp::Greater, IrBinaryOp::GreaterEqual,
        // Logical
        IrBinaryOp::And, IrBinaryOp::Or,
        // Bitwise
        IrBinaryOp::BitwiseAnd, IrBinaryOp::BitwiseOr,
        IrBinaryOp::BitwiseXor, IrBinaryOp::ShiftLeft,
        IrBinaryOp::ShiftRight,
    ];
}
```

**Purpose**: Provide constants for iterating over operation categories in parameterized tests.

---

## 6. Snapshot Test Structures

### 6.1 SnapshotablePromotionResult (Snapshot Format)

```rust
/// Serializable version of PromotionResult for snapshot testing
#[derive(Debug, Serialize)]
struct SnapshotablePromotionResult {
    result_type: String,  // Debug format of IrType
    left_cast: Option<SnapshotableTypePromotion>,
    right_cast: Option<SnapshotableTypePromotion>,
    warnings: Vec<SnapshotableWarning>,
    is_sound: bool,
}

#[derive(Debug, Serialize)]
struct SnapshotableTypePromotion {
    from_type: String,
    to_type: String,
    cast_kind: String,
    may_lose_precision: bool,
    may_overflow: bool,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum SnapshotableWarning {
    PrecisionLoss {
        from_type: String,
        to_type: String,
        estimated_loss: String,
    },
    PotentialOverflow {
        from_type: String,
        to_type: String,
        operation: String,
    },
    SignednessChange {
        from_signed: bool,
        to_signed: bool,
        may_affect_comparisons: bool,
    },
    FloatSpecialValues {
        operation: String,
        may_produce_nan: bool,
        may_produce_infinity: bool,
    },
}

impl From<&PromotionResult> for SnapshotablePromotionResult {
    fn from(result: &PromotionResult) -> Self {
        // Convert to snapshotable format
        // ...
    }
}
```

**Purpose**: Provide consistent, readable snapshot format for `insta` snapshot testing.

---

## 7. Test Organization Model

### 7.1 Test Group Structure

```rust
/// Logical grouping of tests within the test file
mod test_groups {
    // Group 1: analyze_binary_promotion tests
    // - Basic functionality (identity, widening, narrowing)
    // - Type coverage (all 12 IrType variants)
    // - Operation coverage (all 18 IrBinaryOp variants)
    // - Edge cases specific to analyze_binary_promotion
    
    // Group 2: insert_promotion_casts tests
    // - Left operand casting
    // - Right operand casting
    // - Bilateral casting (both operands)
    // - IR verification (cast instruction generation)
    
    // Group 3: Warning generation tests
    // - PrecisionLoss scenarios with message validation
    // - PotentialOverflow scenarios with message validation
    // - SignednessChange scenarios with message validation
    // - FloatSpecialValues scenarios with message validation
    
    // Group 4: Edge case tests
    // - Type boundaries (I8→I64, same-width cross-signedness)
    // - Float-integer boundaries (NaN, Infinity, precision loss)
    // - Promotion matrix edge cases (None returns, bidirectionality)
    // - Operation-specific edge cases (division overflow, bitwise signed)
    
    // Group 5: Corner case tests
    // - Helper method validation (through engine usage)
    // - Multi-warning scenarios
    // - System boundaries (missing rules, invalid contexts)
    
    // Group 6: Integration tests (real matrix)
    // - End-to-end promotion with real PromotionMatrix
    // - Complex multi-step promotions
    // - Real-world type combination scenarios
    
    // Group 7: Unit tests (mocked matrix)
    // - Engine logic isolation
    // - Error path testing (None returns from matrix)
    // - Fallback behavior validation
    
    // Group 8: Concurrent execution tests
    // - Multi-threaded read tests
    // - Consistent result validation
    // - Thread-safety demonstration
}
```

**Total Estimated Tests**: 100-120 test functions organized into 8 groups.

---

## 8. Coverage Tracking Model

### 8.1 Coverage Metrics

```rust
/// Coverage tracking for TypePromotionEngine module
struct CoverageMetrics {
    total_lines: usize,
    covered_lines: usize,
    total_branches: usize,
    covered_branches: usize,
    uncovered_lines: Vec<usize>,
    uncovered_branches: Vec<String>,
}

impl CoverageMetrics {
    fn line_coverage_percentage(&self) -> f64 {
        (self.covered_lines as f64 / self.total_lines as f64) * 100.0
    }
    
    fn branch_coverage_percentage(&self) -> f64 {
        (self.covered_branches as f64 / self.total_branches as f64) * 100.0
    }
    
    fn is_complete(&self) -> bool {
        self.line_coverage_percentage() >= 100.0 && self.branch_coverage_percentage() >= 100.0
    }
}
```

**Target**:
- Line coverage: 100%
- Branch coverage: 100%
- Function coverage: 100% (all public methods tested)

---

## 9. Summary

### Key Data Entities:
1. **Core Entities**: TypePromotionEngine, PromotionResult, TypePromotion, PromotionWarning
2. **Type System**: IrType (12 variants), IrBinaryOp (18 variants), CastKind
3. **Test Fixtures**: TypePairTestCase, OperationTestMatrix, TypeCombinationMatrix, EdgeCaseScenario
4. **Test Helpers**: TestFixtureBuilder, MockPromotionMatrix, AssertionHelpers
5. **Expected Outputs**: ExpectedPromotionResult, SnapshotablePromotionResult
6. **Constants**: test_types (type categories), test_operations (operation categories)

### Coverage Model:
- **Test Count**: 100-120 test functions
- **Test Groups**: 8 logical groups (analyze, insert_casts, warnings, edges, corners, integration, unit, concurrent)
- **Type Pairs**: ~60 type combination scenarios
- **Operations**: 18 binary operations with multiple type combinations each
- **Edge Cases**: 30-40 boundary and special value tests
- **Corner Cases**: 15-20 rare scenario tests

### Quality Targets:
- 100% line coverage for TypePromotionEngine
- 100% branch coverage for all conditionals
- Hybrid assertions (snapshots + explicit)
- All 12 IrType variants tested
- All 18 IrBinaryOp variants tested

---

**Data Model Status**: ✅ COMPLETE - Ready for contract generation and quickstart guide.
