# Data Model: IR Type Promotion System

**Feature**: IR Type Promotion System Correction  
**Date**: 29 settembre 2025  
**Status**: Phase 1 Design Complete

## Core Entities

### TypePromotion

**Purpose**: Central entity managing type promotion logic and rules
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub struct TypePromotion {
    /// The source type being promoted from
    pub from_type: IrType,
    /// The target type being promoted to  
    pub to_type: IrType,
    /// The kind of cast operation required for this promotion
    pub cast_kind: CastKind,
    /// Whether this promotion may result in precision loss
    pub may_lose_precision: bool,
    /// Whether this promotion may result in value overflow/underflow
    pub may_overflow: bool,
    /// Source location for error reporting
    pub source_span: SourceSpan,
}
```

**Relationships**: 
- Associates with `IrType` (many-to-many promotion matrix)
- References `CastKind` for instruction generation
- Uses `SourceSpan` for error reporting

**Validation Rules**:
- `from_type` and `to_type` must be different for explicit promotions
- `cast_kind` must be appropriate for the type conversion
- Precision loss and overflow flags must be computed based on type properties

**State Transitions**:
```
Created → Validated → CastInserted → Applied
```

### PromotionMatrix

**Purpose**: Defines the complete type promotion lattice and rules
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub struct PromotionMatrix {
    /// Matrix of promotion rules indexed by (from_type, to_type)
    promotion_rules: HashMap<(IrType, IrType), PromotionRule>,
    /// Type precedence ordering for automatic promotion
    type_precedence: Vec<TypeGroup>,
    /// Configuration for runtime behavior on errors
    overflow_behavior: OverflowBehavior,
}
```

**Validation Rules**:
- Matrix must be complete for all numeric type combinations
- Type precedence must form a valid partial order
- No circular promotion dependencies allowed

### PromotionRule

**Purpose**: Defines specific promotion behavior between two types
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub enum PromotionRule {
    /// Direct promotion without intermediate steps
    Direct {
        cast_kind: CastKind,
        may_lose_precision: bool,
        may_overflow: bool,
    },
    /// Promotion through intermediate type
    Indirect {
        intermediate_type: IrType,
        first_cast: CastKind,
        second_cast: CastKind,
    },
    /// Promotion not allowed
    Forbidden {
        reason: String,
    },
}
```

**Validation Rules**:
- Direct promotions must specify valid cast operations
- Indirect promotions must use valid intermediate types
- Forbidden promotions must provide clear reasoning

### TypeGroup

**Purpose**: Groups types by mathematical properties for promotion ordering
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub enum TypeGroup {
    SignedIntegers(Vec<IrType>),      // I8, I16, I32, I64
    UnsignedIntegers(Vec<IrType>),    // U8, U16, U32, U64  
    FloatingPoint(Vec<IrType>),       // F32, F64
    Boolean,                          // Bool
    Character,                        // Char
}
```

**Relationships**:
- Contains `IrType` instances grouped by mathematical properties
- Used by `PromotionMatrix` for precedence calculations

**Validation Rules**:
- Each `IrType` must belong to exactly one group
- Groups must be ordered by promotion precedence
- Floating-point group has highest precedence

### BinaryOperationPromotion

**Purpose**: Handles type promotion for binary operations specifically
**Location**: `src/ir/generator.rs` (enhanced)

**Fields**:
```rust
pub struct BinaryOperationPromotion {
    /// Left operand value and type
    pub left_operand: Value,
    /// Right operand value and type
    pub right_operand: Value,
    /// The binary operation being performed
    pub operation: IrBinaryOp,
    /// Result of promotion analysis
    pub promotion_result: PromotionResult,
    /// Source location for error reporting
    pub source_span: SourceSpan,
}
```

**State Transitions**:
```
Created → Analyzed → CastsInserted → InstructionGenerated
```

### PromotionResult

**Purpose**: Contains the result of type promotion analysis
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub struct PromotionResult {
    /// The target type for the operation result
    pub result_type: IrType,
    /// Cast required for left operand (if any)
    pub left_cast: Option<TypePromotion>,
    /// Cast required for right operand (if any)  
    pub right_cast: Option<TypePromotion>,
    /// Warnings generated during promotion analysis
    pub warnings: Vec<PromotionWarning>,
    /// Whether the promotion is mathematically sound
    pub is_sound: bool,
}
```

**Validation Rules**:
- `result_type` must be valid for the given operand types
- Cast operations must be consistent with promotion matrix
- Warnings must be provided for potentially unsafe operations

### PromotionWarning

**Purpose**: Represents warnings generated during type promotion
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
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

**Relationships**:
- References `IrType` for type information
- References `IrBinaryOp` for operation context
- Used by `PromotionResult` for warning aggregation

### OverflowBehavior

**Purpose**: Configuration for runtime behavior on numeric overflow
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub enum OverflowBehavior {
    /// Wrap around using modulo arithmetic
    Wrap,
    /// Saturate to maximum/minimum values
    Saturate,
    /// Generate runtime trap/panic
    Trap,
    /// Compiler error for statically detectable overflow
    CompileError,
}
```

**Usage**: 
- Configured at module or function level
- Affects code generation for promotion operations
- Influences warning generation

### PrecisionLossEstimate

**Purpose**: Quantifies potential precision loss in type conversions
**Location**: `src/ir/type_promotion.rs`

**Fields**:
```rust
pub enum PrecisionLossEstimate {
    /// No precision loss expected
    None,
    /// Fractional part may be lost (float to int)
    FractionalPart,
    /// Significant digits may be lost (f64 to f32)
    SignificantDigits { lost_bits: u32 },
    /// Complete value range change (large int to small int)
    ValueRange { from_bits: u32, to_bits: u32 },
}
```

**Validation Rules**:
- Bit counts must reflect actual type sizes
- Estimates must be conservative (overestimate rather than underestimate)

## Data Relationships

### Type Promotion Lattice

```
    PromotionMatrix
         |
         | contains
         v
    PromotionRule ←--→ TypePromotion
         |                    |
         | uses               | references
         v                    v
     CastKind           IrType + SourceSpan
```

### Binary Operation Flow

```
BinaryOperationPromotion
         |
         | analyzes using
         v
    PromotionMatrix
         |
         | produces
         v
    PromotionResult
         |
         | contains
         v
TypePromotion + PromotionWarning
```

### Error and Warning Aggregation

```
    PromotionResult
         |
         | aggregates
         v
    PromotionWarning[]
         |
         | each contains
         v
    PrecisionLossEstimate | OverflowInfo
```

## Entity Lifecycle

### TypePromotion Creation
1. **Created**: When binary operation requires type promotion
2. **Validated**: Promotion rule verified against matrix
3. **CastInserted**: Corresponding IR cast instruction generated  
4. **Applied**: Included in final IR output

### PromotionMatrix Initialization
1. **Bootstrap**: Load default promotion rules
2. **Configuration**: Apply user/module-specific overrides
3. **Validation**: Verify matrix completeness and consistency
4. **Active**: Available for promotion analysis

### BinaryOperationPromotion Processing
1. **Created**: From AST binary expression
2. **Analyzed**: Types analyzed using promotion matrix
3. **CastsInserted**: Required cast instructions generated
4. **InstructionGenerated**: Final binary instruction with correct types

## Validation and Constraints

### Type Safety Constraints
- All promotions must maintain value representation when possible
- Explicit casts required for potentially unsafe conversions
- Warning generation for all precision-loss scenarios

### Performance Constraints  
- Promotion analysis must be O(1) for common cases
- Matrix lookup must be efficient for all type combinations
- Cast insertion must not significantly impact compilation time

### Consistency Constraints
- Same expression must always produce same result type
- Promotion rules must be transitive where mathematically sound
- Platform-independent behavior for portable code

## Extension Points

### Custom Type Support
- `PromotionMatrix` can be extended for user-defined numeric types
- `TypeGroup` enum can accommodate new type categories
- `CastKind` can be extended for custom conversion operations

### Configurable Behavior
- `OverflowBehavior` allows per-module runtime behavior
- Warning levels configurable through compiler flags
- Promotion strictness levels for different use cases

### Optimization Integration
- `PromotionResult` provides information for constant folding
- Cast elimination opportunities identified through analysis
- SIMD instruction selection based on promotion patterns

This data model provides a comprehensive, detailed, precise, and meticulous foundation for implementing the type promotion system while maintaining compatibility with the existing IR infrastructure and supporting future extensibility requirements.