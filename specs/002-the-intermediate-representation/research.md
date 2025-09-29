# Research Document: IR Type Promotion System Correction

**Feature**: IR Type Promotion System Correction  
**Date**: 29 settembre 2025  
**Status**: Phase 0 Research Complete

## Executive Summary

This research document provides a comprehensive, detailed, precise, and meticulous analysis of the requirements for implementing a systematic type promotion system in the jsavrs compiler's intermediate representation (IR). The current implementation exhibits a critical flaw where binary operations simply inherit the type of the left operand without any promotion logic, leading to inconsistent and potentially incorrect behavior for mixed-type operations.

## Current State Analysis

### Existing Type System Architecture

The jsavrs compiler currently implements a basic type system in `src/ir/types.rs` with the following numeric types:

```rust
pub enum IrType {
    I8, I16, I32, I64,     // Signed integers
    U8, U16, U32, U64,     // Unsigned integers  
    F32, F64,              // Floating-point
    Bool, Char, String,    // Other types
    // ... additional types
}
```

### Critical Issue Identification

**Current Binary Operation Logic** (from `src/ir/generator.rs:674-693`):
```rust
fn generate_binary(&mut self, func: &mut Function, left: Expr, op: BinaryOp, right: Expr, span: SourceSpan) -> Value {
    let ir_op: IrBinaryOp = op.into();
    let left_val = self.generate_expr(func, left);
    let right_val = self.generate_expr(func, right);
    let ty = left_val.ty.clone();  // ❌ PROBLEM: Always uses left operand type
    // ... rest of function
}
```

**Problems with Current Approach**:
1. **No Type Promotion**: Operations like `i32 + f32` result in `i32` type, losing floating-point precision
2. **Inconsistent Behavior**: `i32 + f32` vs `f32 + i32` produce different result types
3. **Potential Data Loss**: Signed/unsigned mixing can lead to unexpected behavior
4. **Non-Deterministic**: Result type depends on operand order rather than mathematical properties

### Existing Cast Infrastructure

The system already has basic casting infrastructure:
- `CastKind` enum with comprehensive conversion types
- `Cast` instruction in `InstructionKind`
- Integration with instruction generation

This provides a foundation for implementing explicit cast insertion.

## Type Promotion System Design

### Type Lattice Definition

Based on the clarification session and mathematical soundness principles, the type lattice follows this hierarchy (from least to most general):

```
                    F64 (highest precedence)
                     |
                    F32
                   /   \
                 I64   U64
                /  \   /  \
              I32  U32-I32  U32
             /  \   |    |   /  \
           I16  U16-I16  U16-I16  U16  
          /  \   |    |    |    |   /  \
        I8   U8-I8    U8-I8   U8-I8   U8
```

**Promotion Rules**:
1. **Float Precedence**: Any operation involving floating-point promotes to the widest float
2. **Integer Width**: Among integers, promote to the wider type
3. **Signedness Handling**: When mixing signed/unsigned of same width, promote to next larger signed type
4. **Special Cases**: Handle overflow scenarios and precision loss

### Promotion Algorithm Implementation

**Phase 1: Basic Promotion Matrix**
```rust
impl IrType {
    pub fn promote_with(&self, other: &IrType) -> Result<IrType, TypePromotionError> {
        match (self, other) {
            // Float takes precedence over any other type
            (IrType::F64, _) | (_, IrType::F64) => Ok(IrType::F64),
            (IrType::F32, IrType::F32) => Ok(IrType::F32),
            (IrType::F32, _) | (_, IrType::F32) => Ok(IrType::F32),
            
            // Signed/unsigned same width promotion
            (IrType::I32, IrType::U32) | (IrType::U32, IrType::I32) => Ok(IrType::I64),
            (IrType::I16, IrType::U16) | (IrType::U16, IrType::I16) => Ok(IrType::I32),
            (IrType::I8, IrType::U8) | (IrType::U8, IrType::I8) => Ok(IrType::I16),
            
            // Width-based promotion within same signedness
            (IrType::I64, _) | (_, IrType::I64) => Ok(IrType::I64),
            (IrType::U64, IrType::U64) => Ok(IrType::U64),
            // ... additional rules
        }
    }
}
```

### Cast Insertion Strategy

**Automatic Cast Generation**:
1. Identify promotion target type using lattice
2. Insert explicit cast instructions for both operands if needed
3. Generate warning for potential precision loss
4. Handle special cases (NaN, infinity, overflow)

**Cast Generation Example**:
```rust
// Original: i32 + f32
// Step 1: Determine promotion type -> f32
// Step 2: Insert cast for i32 operand
// Step 3: Generate binary operation with f32 result

let promoted_left = if left_val.ty != target_type {
    self.insert_cast(left_val, target_type, span)
} else {
    left_val
};
```

## Technical Integration Requirements

### IR Generator Modifications

**Key Changes Required**:
1. **Replace Binary Generation Logic**: Implement promotion algorithm in `generate_binary`
2. **Add Cast Insertion Helper**: Create `insert_promotion_cast` method
3. **Type Compatibility Checking**: Validate promotion is mathematically sound
4. **Error Handling**: Comprehensive error messages for invalid promotions

### Error Handling and Diagnostics

**Compiler Warnings**:
- Precision loss warnings for float→int conversions
- Overflow warnings for integer promotions
- Compatibility warnings during transition period

**Runtime Behavior Configuration**:
- Saturate: Clamp values to target type range
- Wrap: Use modulo arithmetic for overflow
- Trap: Runtime error on overflow/underflow

### Performance Considerations

**Initial Implementation**:
- Focus on correctness over performance (constitutional compliance)
- Generate explicit cast instructions for all promotions
- Maintain deterministic behavior

**Future Optimization Opportunities**:
- Constant folding with promotion
- Dead cast elimination
- SIMD instruction utilization for batch promotions

## Testing Strategy

### Unit Test Coverage

**Core Promotion Logic Tests**:
```rust
#[test]
fn test_int_float_promotion() {
    assert_eq!(IrType::I32.promote_with(&IrType::F32), Ok(IrType::F32));
    assert_eq!(IrType::F32.promote_with(&IrType::I32), Ok(IrType::F32));
}

#[test]
fn test_signed_unsigned_promotion() {
    assert_eq!(IrType::I32.promote_with(&IrType::U32), Ok(IrType::I64));
}
```

**Integration Test Scenarios**:
1. Complex expressions: `(i32 + f32) * u64`
2. Special float values: `NaN + 1i32`
3. Overflow conditions: `i32::MAX + 1u32`
4. Precision loss: `f64 → i32` conversions

### Snapshot Testing with Insta

**IR Output Validation**:
- Generate IR for representative mixed-type expressions
- Validate cast insertion correctness
- Ensure consistent output across compiler runs

### Regression Test Suite

**Historical Bug Prevention**:
- Test cases for each promotion rule
- Edge cases for all type combinations
- Platform-specific behavior validation

## Implementation Dependencies

### Required Crate Additions

No new external dependencies required:
- Rust 1.90.0 provides sufficient type system features
- Existing `uuid`, `petgraph`, and testing frameworks are adequate
- `insta` framework already available for snapshot testing

### Code Generation Backend Integration

**Assembly Generation Impact**:
- Cast instructions must be translated to appropriate machine code
- Platform-specific floating-point behavior handling
- Optimization passes must understand promotion semantics

### Documentation Updates

**Required Documentation**:
1. **User Documentation**: Type promotion rules and behavior
2. **Developer Documentation**: Implementation details and extension points  
3. **Migration Guide**: Transition from old to new promotion behavior
4. **Architecture Documentation**: Integration with existing IR modules

## Risk Analysis and Mitigation

### Breaking Changes

**Compatibility Risk**: Existing code may rely on current (incorrect) promotion behavior
**Mitigation**: Gradual transition with compiler warnings and feature flags

### Performance Impact

**Risk**: Additional cast instructions may impact compilation speed
**Mitigation**: Focus on correctness first, optimize in subsequent phases per constitutional principles

### Complexity Management

**Risk**: Type promotion system adds significant complexity
**Mitigation**: Modular design allows isolated testing and incremental deployment

## Research Conclusions

### Technology Decisions

**Decision**: Implement promotion system as extension to existing IR instruction system
**Rationale**: Leverages existing cast infrastructure while maintaining backward compatibility
**Alternatives Considered**: Complete IR redesign rejected due to scope and risk

**Decision**: Use deterministic promotion matrix rather than configurable policies
**Rationale**: Ensures consistent behavior across all compilation contexts
**Alternatives Considered**: User-configurable promotion rejected due to complexity

**Decision**: Generate explicit cast instructions rather than implicit type coercion
**Rationale**: Maintains IR transparency and enables better optimization
**Alternatives Considered**: Implicit coercion rejected due to debugging difficulty

### Implementation Approach

**Phase 1**: Core promotion algorithm and cast insertion
**Phase 2**: Comprehensive test suite and regression prevention
**Phase 3**: Documentation and migration tooling
**Phase 4**: Performance optimization and advanced features

### Success Criteria

1. ✅ **Deterministic Behavior**: Same expression always produces same result type
2. ✅ **Precision Preservation**: Mathematical operations maintain maximum precision
3. ✅ **Standard Compliance**: IEEE floating-point and integer overflow behavior
4. ✅ **Backward Compatibility**: Gradual transition with clear migration path
5. ✅ **Performance Acceptable**: No more than 10% compilation time increase initially

## Next Steps

This research phase has resolved all technical unknowns and provides a clear foundation for the design phase. The promotion system can be implemented using existing infrastructure with well-defined algorithms and comprehensive testing strategies.

**Ready for Phase 1**: Design and contract generation based on research findings.