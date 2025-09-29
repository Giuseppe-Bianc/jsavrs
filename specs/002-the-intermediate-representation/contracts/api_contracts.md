# Type Promotion API Contracts

**Feature**: IR Type Promotion System Correction  
**Date**: 29 settembre 2025  
**Status**: Phase 1 Design Complete

## TypePromotionEngine Contract

### analyze_binary_promotion

**Purpose**: Analyzes type promotion requirements for binary operations

**Signature**:
```rust
pub fn analyze_binary_promotion(
    left_type: &IrType,
    right_type: &IrType,
    operation: IrBinaryOp,
    source_span: SourceSpan,
) -> Result<PromotionResult, TypePromotionError>
```

**Input Validation**:
- `left_type` and `right_type` must be valid numeric types
- `operation` must be compatible with numeric operands
- `source_span` must be valid for error reporting

**Output Guarantees**:
- Returns `PromotionResult` with deterministic target type
- Includes all necessary cast operations
- Provides warnings for potential precision loss or overflow
- Always returns same result for same inputs (deterministic)

**Error Conditions**:
- `TypePromotionError::IncompatibleTypes` for non-numeric types
- `TypePromotionError::UnsupportedOperation` for invalid operations
- `TypePromotionError::AmbiguousPromotion` for unclear promotion paths

### insert_promotion_casts

**Purpose**: Generates explicit cast instructions for type promotion

**Signature**:
```rust
pub fn insert_promotion_casts(
    generator: &mut NIrGenerator,
    left_value: Value,
    right_value: Value,
    promotion_result: &PromotionResult,
) -> Result<(Value, Value), CastInsertionError>
```

**Input Validation**:
- `left_value` and `right_value` must have types matching promotion analysis
- `promotion_result` must be valid result from `analyze_binary_promotion`
- `generator` must be in valid state for instruction insertion

**Output Guarantees**:
- Returns promoted values with correct target types
- Inserts cast instructions into current basic block
- Maintains value semantics during conversion
- Updates temporary value counters appropriately

**Error Conditions**:
- `CastInsertionError::InvalidValue` for malformed input values
- `CastInsertionError::GeneratorError` for IR generation failures
- `CastInsertionError::CastFailed` for unsupported cast operations

## PromotionMatrix Contract

### get_promotion_rule

**Purpose**: Retrieves promotion rule for specific type pair

**Signature**:
```rust
pub fn get_promotion_rule(
    &self,
    from_type: &IrType,
    to_type: &IrType,
) -> Option<&PromotionRule>
```

**Input Validation**:
- `from_type` and `to_type` must be valid `IrType` variants
- Types must be numeric (integer or floating-point)

**Output Guarantees**:
- Returns `Some(rule)` for supported type combinations
- Returns `None` for unsupported or forbidden combinations
- Rule contains complete promotion information
- Lookup is O(1) performance

### compute_common_type

**Purpose**: Determines least upper bound type for two operands

**Signature**:
```rust
pub fn compute_common_type(
    &self,
    left_type: &IrType,
    right_type: &IrType,
) -> Result<IrType, PromotionError>
```

**Input Validation**:
- Both types must be numeric
- Types must be supported by promotion matrix

**Output Guarantees**:
- Returns deterministic common type based on lattice
- Floating-point types take precedence over integers
- Wider types take precedence over narrower types
- Signed/unsigned conflicts promote to next larger signed type

**Error Conditions**:
- `PromotionError::NonNumericType` for invalid input types
- `PromotionError::NoCommonType` for incompatible type combinations

## TypePromotion Contract

### new

**Purpose**: Creates new type promotion instance

**Signature**:
```rust
pub fn new(
    from_type: IrType,
    to_type: IrType,
    source_span: SourceSpan,
) -> Result<Self, ValidationError>
```

**Input Validation**:
- `from_type` and `to_type` must be different for meaningful promotion
- Both types must be numeric
- `source_span` must be valid

**Output Guarantees**:
- Creates valid `TypePromotion` instance
- Computes appropriate `CastKind` for conversion
- Calculates precision loss and overflow flags
- Maintains source location for error reporting

### generate_cast_instruction

**Purpose**: Creates IR cast instruction for this promotion

**Signature**:
```rust
pub fn generate_cast_instruction(
    &self,
    value: Value,
    temp_id: u64,
) -> Instruction
```

**Input Validation**:
- `value` type must match `from_type`
- `temp_id` must be unique temporary identifier

**Output Guarantees**:
- Returns valid `Instruction` with `Cast` kind
- Instruction includes correct type information
- Result value has target type
- Maintains debug information from source

## PromotionWarning Contract

### format_for_user

**Purpose**: Formats warning for user-friendly display

**Signature**:
```rust
pub fn format_for_user(&self) -> String
```

**Output Guarantees**:
- Returns human-readable warning message
- Includes specific type information
- Suggests potential fixes where applicable
- Consistent formatting across warning types

### severity_level

**Purpose**: Determines warning severity level

**Signature**:
```rust
pub fn severity_level(&self) -> WarningSeverity
```

**Output Guarantees**:
- Returns appropriate severity (Info, Warning, Error)
- Precision loss warnings are Warning level
- Potential overflow warnings are Warning level
- Forbidden operations are Error level

## Error Types Contract

### TypePromotionError

```rust
pub enum TypePromotionError {
    IncompatibleTypes {
        left_type: IrType,
        right_type: IrType,
        operation: IrBinaryOp,
        source_span: SourceSpan,
    },
    UnsupportedOperation {
        operation: IrBinaryOp,
        operand_types: (IrType, IrType),
        source_span: SourceSpan,
    },
    AmbiguousPromotion {
        possible_targets: Vec<IrType>,
        source_span: SourceSpan,
    },
}
```

**Error Information**:
- All variants include `source_span` for precise error location
- Detailed type information for debugging
- Context about failed operation
- Suggestions for resolution where possible

### CastInsertionError

```rust
pub enum CastInsertionError {
    InvalidValue {
        value: Value,
        expected_type: IrType,
        source_span: SourceSpan,
    },
    GeneratorError {
        message: String,
        source_span: SourceSpan,
    },
    CastFailed {
        from_type: IrType,
        to_type: IrType,
        reason: String,
        source_span: SourceSpan,
    },
}
```

**Recovery Strategies**:
- `InvalidValue`: Verify value type before promotion
- `GeneratorError`: Check generator state and retry
- `CastFailed`: Use alternative promotion path or report error

## Integration Contracts

### NIrGenerator Integration

**Modified generate_binary signature**:
```rust
fn generate_binary(
    &mut self,
    func: &mut Function,
    left: Expr,
    op: BinaryOp,
    right: Expr,
    span: SourceSpan,
) -> Value
```

**Behavioral Changes**:
- Must call type promotion analysis before generating instruction
- Must insert cast instructions when promotion required
- Must generate warnings for potentially unsafe operations
- Must maintain deterministic behavior for same inputs

### Module Integration

**Promotion configuration**:
```rust
pub struct ModulePromotionConfig {
    pub overflow_behavior: OverflowBehavior,
    pub warning_level: WarningLevel,
    pub strict_promotion: bool,
}
```

**Requirements**:
- Configuration must be consistent within module
- Settings affect all binary operations in module
- Default configuration must be mathematically sound

## Testing Contracts

### Unit Test Requirements

Each API function must have tests covering:
- **Happy Path**: Normal promotion scenarios
- **Edge Cases**: Boundary conditions and special values
- **Error Cases**: Invalid inputs and error recovery
- **Performance**: O(1) complexity for common operations

### Integration Test Requirements

End-to-end scenarios must verify:
- **Deterministic Output**: Same input produces same IR
- **Cast Insertion**: Explicit casts appear in generated IR
- **Warning Generation**: Appropriate warnings for unsafe operations
- **Backend Compatibility**: Generated IR works with assembly generator

### Contract Test Examples

```rust
#[test]
fn test_analyze_binary_promotion_contract() {
    let engine = TypePromotionEngine::new();
    let result = engine.analyze_binary_promotion(
        &IrType::I32,
        &IrType::F32,
        IrBinaryOp::Add,
        SourceSpan::default(),
    ).unwrap();
    
    // Contract: Float takes precedence
    assert_eq!(result.result_type, IrType::F32);
    
    // Contract: Left operand needs cast
    assert!(result.left_cast.is_some());
    assert_eq!(result.left_cast.unwrap().to_type, IrType::F32);
    
    // Contract: Right operand no cast needed
    assert!(result.right_cast.is_none());
}
```

These contracts ensure that the type promotion system provides predictable, well-defined behavior while integrating seamlessly with the existing IR infrastructure. All implementations must adhere to these contracts to maintain system integrity and user expectations.