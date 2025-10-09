# Research Document: Type Casting System Enhancement

**Feature**: Comprehensive Type Casting System Enhancement  
**Branch**: `004-enhance-the-casting`  
**Date**: 2025-10-08  
**Status**: Complete

## Overview

This document consolidates research findings for implementing comprehensive type casting support across all 13 fundamental data types in the jsavrs compiler. Research focused on resolving technical clarifications, identifying best practices, and documenting design decisions based on existing codebase analysis and established compiler design patterns.

---

## Research Tasks Completed

### 1. Analysis of Existing CastKind Enum Completeness

**Task**: Verify that all 24 CastKind variants exist and identify any missing conversion types.

**Findings**:
- **Source**: `src/ir/instruction.rs` lines 1-150
- **Current State**: The CastKind enum contains 24 variants covering:
  - Integer operations: `IntZeroExtend`, `IntSignExtend`, `IntTruncate`, `IntBitcast`
  - Integer-Float conversions: `IntToFloat`, `FloatToInt`
  - Float-Float conversions: `FloatTruncate`, `FloatExtend`
  - Boolean conversions: `BoolToInt`, `IntToBool`, `BoolToFloat`, `FloatToBool`
  - Character conversions: `CharToInt`, `IntToChar`
  - Character-String conversions: `CharToString`, `StringToChar`
  - String conversions: `StringToInt`, `StringToFloat`, `StringToBool`, `IntToString`, `FloatToString`, `BoolToString`
  - Bitcast: `Bitcast` (for same-size bit reinterpretation)

**Decision**: ✅ All required CastKind variants are present. No modifications to the CastKind enum are necessary.

**Rationale**: The existing CastKind enum comprehensively covers all conversion types required by the feature specification. Each variant has a clear semantic purpose aligned with the 13 fundamental types.

**Alternatives Considered**: Adding specialized variants like `IntNarrow` or `FloatRound` were considered but rejected because the existing variants (`IntTruncate`, `FloatToInt`) adequately represent these operations with appropriate flags (`may_lose_precision`, `may_overflow`).

---

### 2. Integer Narrowing Conversion Strategy

**Task**: Research best practices for implementing integer narrowing conversions with appropriate overflow detection.

**Findings**:
- **Reference**: Rust's `as` cast behavior, C++ integral promotions, LLVM IR trunc instruction
- **Current Implementation**: Existing code in `type_promotion.rs` implements widening conversions but lacks comprehensive narrowing rules
- **Overflow Scenarios**:
  - Unsigned to smaller unsigned: value > target_max → truncation warning
  - Signed to smaller signed: value < target_min || value > target_max → overflow warning
  - Unsigned to signed: value > signed_max → sign change warning + overflow
  - Signed to unsigned: value < 0 → sign change warning + wrap-around

**Decision**: Implement bidirectional narrowing rules for all integer type pairs with `IntTruncate` CastKind and set both `may_lose_precision=true` and `may_overflow=true` for narrowing conversions.

**Rationale**: This approach follows Rust's explicit casting semantics where narrowing is permitted but potentially lossy. The warnings inform developers of potential issues while maintaining flexibility. The `OverflowBehavior` enum (Wrap/Saturate/Trap/CompileError) allows configuration of runtime behavior.

**Alternatives Considered**: 
- **Forbidden narrowing**: Rejected because legitimate use cases exist (e.g., truncating hash values, extracting bytes from multi-byte integers)
- **Indirect narrowing through larger signed type**: Rejected because it adds unnecessary intermediate conversions and doesn't eliminate the fundamental narrowing operation

---

### 3. Boolean Conversion Semantics

**Task**: Define canonical mappings for boolean conversions to/from numeric types and establish validation rules.

**Findings**:
- **Reference**: C/C++ truthiness semantics, Rust's explicit bool conversions, LLVM i1 type
- **Standard Practice**:
  - Boolean → Integer: `true` → 1, `false` → 0 (for all integer types)
  - Boolean → Float: `true` → 1.0, `false` → 0.0
  - Integer → Boolean: 0 → `false`, non-zero → `true`
  - Float → Boolean: 0.0 → `false`, non-zero (including NaN) → `true`
- **Edge Cases**:
  - NaN → Boolean: In IEEE 754, NaN != 0.0, so NaN → `true`
  - Negative zero (-0.0) → Boolean: -0.0 == 0.0, so → `false`

**Decision**: 
- Implement all boolean conversions using existing CastKind variants:
  - Bool → Integer: `BoolToInt` (produce 0 or 1 in target integer type)
  - Bool → Float: `BoolToFloat` (produce 0.0 or 1.0)
  - Integer → Bool: `IntToBool` (zero test)
  - Float → Bool: `FloatToBool` (zero test, NaN→true)
- Set `may_lose_precision=false` and `may_overflow=false` for bool conversions (no precision loss in 0/1 mapping)

**Rationale**: These semantics align with established programming language conventions and LLVM IR behavior, ensuring predictable and intuitive behavior for developers.

**Alternatives Considered**:
- **Ternary boolean** (true/false/error): Rejected as overly complex for a systems language
- **NaN → false**: Rejected because IEEE 754 defines NaN != 0.0, making it truthy

---

### 4. Character and Unicode Handling

**Task**: Research Unicode scalar value validation and establish rules for char ↔ u32 conversions.

**Findings**:
- **Reference**: Rust `char` type definition (RFC 2005), Unicode Standard Chapter 3
- **Unicode Scalar Value**: Valid range is U+0000 to U+D7FF and U+E000 to U+10FFFF
  - **Excluded**: Surrogate code points U+D800 to U+DFFF (reserved for UTF-16 encoding)
  - **Maximum**: U+10FFFF (1,114,111 in decimal)
- **Rust Implementation**: `char::from_u32()` returns `Option<char>`, validating the range

**Decision**: 
- `CharToInt` (char → u32): Always valid, direct mapping to Unicode scalar value, `may_lose_precision=false`, `may_overflow=false`
- `IntToChar` (u32 → char): Potentially invalid, requires validation
  - Valid range: 0..=0xD7FF and 0xE000..=0x10FFFF
  - Invalid conversions generate compile-time error or runtime check based on context
  - `may_overflow=false` (validation handles this), but requires runtime check flag

**Rationale**: This follows Rust's safety guarantees for char validity. Preventing invalid char creation at compile time (when statically known) or runtime (for dynamic values) maintains type system integrity.

**Alternatives Considered**:
- **Unchecked conversion**: Rejected due to safety concerns (invalid char could cause UB in string operations)
- **Surrogate remapping**: Rejected because surrogates have no valid Unicode meaning outside UTF-16 encoding

---

### 5. String Conversion Runtime Requirements

**Task**: Identify which string conversions require runtime support versus compile-time evaluation.

**Findings**:
- **Compile-Time**: String literals → primitive conversions can be evaluated at compile time if the string is known
  - Example: `"42"` → `42_i32` can be const-evaluated
- **Runtime**: Dynamic string parsing always requires runtime support
  - Parsing functions: `str::parse::<T>()` in Rust standard library
  - Error handling: `Result<T, ParseError>` for fallible conversions

**Decision**:
- **Primitive → String**: Mark as runtime operation (requires heap allocation)
  - CastKind: `IntToString`, `FloatToString`, `BoolToString`, `CharToString`
  - Implementation note: Add `requires_runtime_support` flag to PromotionRule
- **String → Primitive**: Mark as fallible runtime operation
  - CastKind: `StringToInt`, `StringToFloat`, `StringToBool`, `StringToChar`
  - Error handling: Invalid string should generate `PromotionWarning::InvalidStringConversion`
  - For compile-time evaluation: Use const evaluator when string literal is statically known

**Rationale**: This separation allows the compiler to optimize const-evaluable conversions while preserving correct semantics for runtime conversions. The `requires_runtime_support` flag enables code generation phase to emit appropriate runtime calls.

**Alternatives Considered**:
- **Forbid string conversions**: Rejected because string I/O is essential for practical programs
- **Compile-time-only string conversions**: Rejected because dynamic input (user input, file parsing) requires runtime conversion

---

### 6. Floating-Point Special Value Handling

**Task**: Define behavior for NaN, infinity, and subnormal numbers in type conversions.

**Findings**:
- **IEEE 754 Special Values**:
  - **NaN (Not a Number)**: Result of invalid operations (0/0, sqrt(-1))
    - NaN != NaN (unique property)
    - NaN in boolean context → `true` (non-zero)
    - NaN → integer → behavior controlled by OverflowBehavior configuration (see Decision section for deterministic semantics)
  - **Infinity (±∞)**: Result of overflow (1.0/0.0)
    - +∞ > all finite numbers
    - +∞ → integer → behavior controlled by OverflowBehavior configuration (see Decision section for deterministic semantics)
  - **Subnormal Numbers**: Very small numbers near zero (gradual underflow)
    - f64 subnormals → f32: May become zero or remain subnormal
    - f32 subnormals → f64: Exact conversion (f64 has more precision)
  - **Negative Zero (-0.0)**: Distinct from +0.0 in IEEE 754
    - -0.0 == +0.0 in comparisons
    - -0.0 → integer → 0

**Decision**:
- **Float → Integer Conversions**:
  - NaN → integer: Generate `PromotionWarning::FloatSpecialValues` with `value_type: FloatSpecialValueType::NaN`
  - ±∞ → integer: Generate `PromotionWarning::FloatSpecialValues` with `value_type: FloatSpecialValueType::PositiveInfinity` or `NegativeInfinity`
  - Behavior controlled by `OverflowBehavior` enum (stored in warning's `applied_behavior` field):
    - `Wrap`: Deterministic - NaN→0, +∞→INT_MAX/UINT_MAX, -∞→INT_MIN/0
    - `Saturate`: Clamp to integer min/max (same as Wrap for special values)
    - `Trap`: Runtime panic/trap
    - `CompileError`: Compile-time error if statically detectable
- **Float → Float Conversions**:
  - f64 → f32: NaN remains NaN, ±∞ remains ±∞, subnormals may flush to zero (warn)
  - f32 → f64: Exact conversion, no special handling needed
- **Float → Bool**: NaN → `true`, -0.0 → `false`

**Rationale**: This approach follows IEEE 754 semantics while providing developer control through `OverflowBehavior`. Warnings ensure developers are aware of potential special value scenarios.

**Alternatives Considered**:
- **NaN → panic always**: Rejected because valid use cases exist (checked arithmetic with NaN propagation)
- **Forbid float → integer for NaN/∞**: Rejected because runtime values can't always be statically validated

---

### 7. Precision Loss Estimation Strategy

**Task**: Design a system to quantify and report precision loss in type conversions.

**Findings**:
- **Types of Precision Loss**:
  1. **Fractional part loss**: Float → Integer (3.14 → 3)
  2. **Mantissa precision loss**: f64 (53-bit mantissa) → f32 (24-bit mantissa)
  3. **Integer range reduction**: u64 → u16 (64 bits → 16 bits)
  4. **Integer → Float precision**: 64-bit integer → f32 (24-bit mantissa) loses low-order bits
- **Existing Implementation**: `PrecisionLossEstimate` enum in `type_promotion.rs` lines 150-160

**Decision**: Utilize existing `PrecisionLossEstimate` enum with three categories:
1. **None**: No precision loss (widening conversions, exact conversions)
2. **FractionalPart**: Float → Integer truncates fractional digits
3. **SignificantDigits**: Mantissa reduction (e.g., f64 → f32 loses 29 bits)
4. **ValueRange**: Narrowing conversions (e.g., u32 → u16 loses high 16 bits)

Apply precision loss warnings based on conversion type:
- **Integer widening**: `None`
- **Integer narrowing**: `ValueRange { from_bits: source_width, to_bits: target_width }`
- **Float → Integer**: `FractionalPart`
- **f64 → f32**: `SignificantDigits { lost_bits: 29 }` (53 - 24 = 29)
- **Integer → Float** (large integers): 
  - i64/u64 → f32: `SignificantDigits { lost_bits: 40 }` (64 - 24 = 40)
  - i64/u64 → f64: `None` (64 bits <= 53-bit mantissa for most values, warn for >53 bits)

**Rationale**: Quantifying precision loss helps developers make informed decisions about type conversions. The enum provides a structured way to communicate the nature and severity of precision loss.

**Alternatives Considered**:
- **Numeric precision loss amount**: Rejected because exact loss depends on runtime values, not just types
- **Binary warning flag**: Rejected because it doesn't communicate the nature of the precision loss

---

### 8. Cross-Signedness Conversion Rules

**Task**: Establish rules for conversions between signed and unsigned types of the same width.

**Findings**:
- **Current Implementation**: `add_cross_signedness_promotions()` in lines 290-325 uses `Bitcast` for i8↔u8, i16↔u16, i32↔u32, i64↔u64
- **Semantics**: Bit reinterpretation without value transformation
  - Positive signed → unsigned: Direct mapping (123_i32 → 123_u32)
  - Negative signed → unsigned: Two's complement reinterpretation (-1_i32 → 4294967295_u32)
  - Large unsigned → signed: Wrap to negative (-128_i8 from 128_u8)

**Decision**: Use `IntBitcast` for same-width signed↔unsigned conversions with `may_lose_precision=false` and `may_overflow=false`, but add `PromotionWarning::SignednessChange` with `may_affect_comparisons=true`.

**Rationale**: While bit patterns are preserved (no overflow), the semantic meaning of values changes dramatically for negative/large numbers. The warning alerts developers to potential logical errors in comparisons or arithmetic after the conversion.

**Alternatives Considered**:
- **Indirect conversion through larger signed type**: Rejected because it adds unnecessary overhead and doesn't preserve bit patterns
- **Forbid signed↔unsigned**: Rejected because legitimate use cases exist (bit manipulation, FFI, cryptographic operations)

---

### 9. Performance Optimization Strategy

**Task**: Verify that O(1) performance can be achieved for promotion rule lookups and identify optimization opportunities.

**Findings**:
- **Current Implementation**: `PromotionMatrix` uses `HashMap<(IrType, IrType), PromotionRule>` (line 70)
- **Lookup Complexity**: HashMap provides O(1) average-case lookup
- **Hash Performance**: IrType derives `Hash` (line 7 of types.rs), enabling efficient hashing
- **Memory Overhead**: 169 type pairs × ~64 bytes per HashMap entry ≈ 10.8 KB (negligible)

**Decision**: Continue using HashMap for promotion rule storage. Add the following optimizations:
1. **Pre-initialize matrix in lazy_static**: Compute all 169 rules once at program start
2. **Identity conversion fast path**: Check `from == to` before HashMap lookup (O(1) comparison)
3. **Cache common conversions**: Keep frequently used rules in a small array (i32↔i64, i32↔f32, etc.)

**Rationale**: HashMap provides excellent performance for the 169-entry matrix. The optimizations target hot paths (identity conversions, common numeric operations) without complicating the implementation.

**Alternatives Considered**:
- **2D array with enum indices**: Rejected because IrType enum includes non-indexable variants (Pointer, Custom, Struct)
- **Perfect hash function**: Rejected due to implementation complexity without significant performance gain

---

### 10. Overflow Behavior Configuration

**Task**: Document how the four overflow behaviors (Wrap, Saturate, Trap, CompileError) should be implemented for each conversion type.

**Findings**:
- **Current Implementation**: `OverflowBehavior` enum exists (lines 140-150), stored in `PromotionMatrix`
- **Applicable Conversions**: Narrowing integer conversions, float→integer conversions
- **Not Applicable**: Widening conversions (no overflow possible), boolean conversions (0/1 always fits)

**Decision**: Apply overflow behavior configuration as follows:

1. **Wrap** (default): Two's complement wrap-around for normal values; deterministic behavior for float special values
   - Integer overflow example: 256_u16 → 0_u8
   - Float special values behavior (DETERMINISTIC):
     * NaN → 0 (zero, for both signed and unsigned targets)
     * +∞ → INT_MAX (for signed types) or UINT_MAX (for unsigned types)
     * -∞ → INT_MIN (for signed types) or 0 (for unsigned types, clamped)
   - Implementation: Use modulo arithmetic for normal values; explicit checks for float special values

2. **Saturate**: Clamp to target type's min/max (same for normal and special float values)
   - Integer example: 256_u16 → 255_u8, -1_i16 → 0_u8
   - Float special values: NaN→0, +∞→INT_MAX/UINT_MAX, -∞→INT_MIN/0
   - Implementation: `value.clamp(min, max)` with special value preprocessing

3. **Trap**: Runtime panic/exception
   - Example: 256_u16 → u8 → panic!("overflow converting u16 to u8")
   - Float special values: panic!("cannot convert NaN/infinity to integer")
   - Implementation: Insert runtime check + trap instruction in IR

4. **CompileError**: Compile-time error for statically detectable overflows
   - Example: `const X: u8 = 256_u16;` → compile error
   - Float special values: `const Y: i32 = f64::NAN;` → compile error
   - Implementation: Const evaluator checks during compile-time evaluation

**Rationale**: This four-tier approach balances safety, performance, and flexibility. `Wrap` is fastest (no checks), `Saturate` prevents corruption, `Trap` catches errors at runtime, `CompileError` catches errors at compile time when possible.

**Float Special Values Rationale**: The deterministic behavior for Wrap mode (NaN→0, ±∞→INT_MIN/MAX) ensures cross-platform compatibility and predictable program behavior, aligning with the project's **Cross-Platform Compatibility** and **Safety First** constitution principles. This matches LLVM's documented behavior for `fptosi`/`fptoui` with undefined behavior mitigation and Rust's saturating cast semantics. The deterministic approach prevents subtle platform-dependent bugs where different architectures might produce different results for NaN/infinity conversions.

**Alternatives Considered**:
- **Panic always**: Rejected because performance-critical code needs unchecked conversions
- **Two behaviors instead of four**: Rejected because different domains need different safety guarantees (embedded: Saturate, debug: Trap, release: Wrap)
- **Platform-dependent Wrap behavior**: Rejected because it violates Cross-Platform Compatibility principle and makes testing impossible
- **Match IEEE 754 undefined behavior**: Rejected because undefined behavior contradicts Safety First principle

---

## Summary of Key Decisions

| Decision Area | Choice | CastKind Variant | Flags |
|--------------|--------|------------------|-------|
| Integer Widening | Direct promotion | IntZeroExtend/IntSignExtend | may_lose_precision=false, may_overflow=false |
| Integer Narrowing | Direct promotion with warnings | IntTruncate | may_lose_precision=true, may_overflow=true |
| Same-width Signed↔Unsigned | Bitcast with signedness warning | IntBitcast | may_lose_precision=false, may_overflow=false |
| Integer → Float | Direct promotion | IntToFloat | may_lose_precision=(if >53 bits for f64, >24 bits for f32) |
| Float → Integer | Direct with fractional loss warning | FloatToInt | may_lose_precision=true, may_overflow=true |
| f32 → f64 | Exact promotion | FloatExtend | may_lose_precision=false, may_overflow=false |
| f64 → f32 | Truncation with precision loss | FloatTruncate | may_lose_precision=true, may_overflow=false |
| Bool → Integer | Direct (0/1) | BoolToInt | may_lose_precision=false, may_overflow=false |
| Integer → Bool | Zero test | IntToBool | may_lose_precision=false, may_overflow=false |
| Bool → Float | Direct (0.0/1.0) | BoolToFloat | may_lose_precision=false, may_overflow=false |
| Float → Bool | Zero test (NaN→true) | FloatToBool | may_lose_precision=false, may_overflow=false |
| char → u32 | Unicode scalar value extraction | CharToInt | may_lose_precision=false, may_overflow=false |
| u32 → char | Validated Unicode scalar | IntToChar | requires_validation=true |
| char → String | Single-char string | CharToString | requires_runtime_support=true |
| String → char | Length check (must be 1) | StringToChar | requires_validation=true, requires_runtime_support=true |
| Primitive → String | Formatting | IntToString/FloatToString/BoolToString | requires_runtime_support=true |
| String → Primitive | Parsing | StringToInt/StringToFloat/StringToBool | requires_validation=true, requires_runtime_support=true |

---

## Implementation Recommendations

Based on this research, the following implementation approach is recommended:

### Phase 1: Extend Promotion Matrix Initialization
1. Add all integer narrowing rules (64 pairs)
2. Add boolean conversion rules (24 pairs)
3. Add character conversion rules (6 pairs)
4. Add string conversion rules (28 pairs)
5. Verify all 169 pairs are defined

### Phase 2: Enhance Warning Generation
1. Implement `PromotionWarning::SignednessChange`
2. Update `PromotionWarning::FloatSpecialValues` to use type conversion context fields (value_type, source_type, target_type, applied_behavior, source_span)
3. Implement `PromotionWarning::InvalidStringConversion`
4. Implement `PromotionWarning::InvalidUnicodeCodePoint`
5. Update `PrecisionLossEstimate` application logic
6. Add `requires_runtime_support` flag to `PromotionRule`
7. Add `requires_validation` flag for checked conversions

### Phase 3: Comprehensive Testing
1. Write tests for all 169 conversion pairs
2. Write edge case tests (NaN, infinity, overflow, invalid Unicode)
3. Write performance benchmarks (verify O(1) lookup)
4. Write snapshot tests for warning messages

### Phase 4: Documentation
1. Update module-level documentation in `type_promotion.rs`
2. Add rustdoc examples for each conversion category
3. Document overflow behavior configuration
4. Update QWEN.md with type casting design

---

## References

1. **Rust Language Reference**: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
2. **IEEE 754-2008 Standard**: Floating-point arithmetic specification
3. **Unicode Standard 15.0**: Chapter 3 (Conformance), Chapter 2 (General Structure)
4. **LLVM Language Reference**: Cast instructions (trunc, zext, sext, fptrunc, fpext, fptoui, fptosi, uitofp, sitofp)
5. **Rust RFC 2005**: Match ergonomics and char validation
6. **jsavrs QWEN.md**: Existing type system documentation
7. **jsavrs AGENTS.md**: Agent-based development framework

---

**Research Status**: ✅ COMPLETE - All clarifications resolved, ready for Phase 1 (Design & Contracts)
