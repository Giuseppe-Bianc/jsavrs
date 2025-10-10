# Feature Specification: Comprehensive Type Casting System Enhancement

**Feature Branch**: `004-enhance-the-casting`  
**Created**: 2025-10-08  
**Status**: Draft  
**Input**: User description: "Enhance the casting functionality in the file located at src/ir/type_promotion.rs to support conversions among all fundamental data types: u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, String, bool. Define all possible conversions between these types, ensure compliance with CastKind enum, consider edge cases like overflow and precision loss, and include tests."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Numeric Type Conversions (Priority: P1)

Compiler users need to perform standard numeric conversions between integer types of different widths and signedness, and between integer and floating-point types. These conversions should handle widening operations safely and narrowing operations with appropriate warnings about potential data loss.

**Why this priority**: Numeric conversions form the foundation of any type system and are the most frequently used type casts in typical programs. Without these, the compiler cannot support basic arithmetic operations involving mixed types.

**Independent Test**: Can be fully tested by compiling programs that perform integer-to-integer widening (u8→u32, i8→i64), narrowing (u64→u16, i64→i8), and integer-float conversions (i32→f32, f64→u64), verifying correct value transformations and appropriate warnings for lossy conversions.

**Acceptance Scenarios**:

1. **Given** source code with unsigned integer widening (u8 to u32), **When** the compiler performs type promotion, **Then** the value is zero-extended without data loss and no warnings are generated
2. **Given** source code with signed integer widening (i8 to i64), **When** the compiler performs type promotion, **Then** the value is sign-extended preserving negative values and no warnings are generated
3. **Given** source code with integer narrowing (u64 to u16), **When** the compiler performs type promotion, **Then** the value is truncated and a warning is generated about potential overflow
4. **Given** source code with integer to float conversion (i32 to f32), **When** the compiler performs type promotion, **Then** the integer value is converted to the nearest representable float with warnings for precision loss if value exceeds 24-bit mantissa precision
5. **Given** source code with float to integer conversion (f64 to i32), **When** the compiler performs type promotion, **Then** fractional parts are truncated and warnings are generated about precision loss and potential overflow
6. **Given** source code converting floating-point special values (NaN, positive infinity, negative infinity) to integer types, **When** the compiler performs type promotion, **Then** a `FloatSpecialValues` warning is generated and the behavior follows the configured OverflowBehavior with deterministic semantics: (a) **Wrap** produces NaN→0, +∞→INT_MAX (UINT_MAX for unsigned types), -∞→INT_MIN (0 for unsigned types), (b) **Saturate** produces NaN→0, +∞→INT_MAX (UINT_MAX for unsigned), -∞→INT_MIN (0 for unsigned), (c) **Trap** causes runtime panic with descriptive error message, (d) **CompileError** fails compilation if the value is a compile-time constant; negative zero (-0.0) converts to integer 0 without warnings

---

### User Story 2 - Boolean and Character Conversions (Priority: P2)

Compiler users need to convert boolean values to numeric types for conditional logic and bitwise operations, and convert between character types and their numeric Unicode representations.

**Why this priority**: Boolean and character conversions enable common programming patterns like using boolean flags in arithmetic expressions and working with Unicode code points. These are essential for practical programming but less frequently used than basic numeric conversions.

**Independent Test**: Can be fully tested by compiling programs that convert bool to integers (true→1, false→0), integers to bool (nonzero→true, zero→false), char to u32 Unicode values, and u32 to char with validation for valid Unicode ranges.

**Acceptance Scenarios**:

1. **Given** source code converting boolean true to integer, **When** the compiler performs type promotion, **Then** the result is numeric value 1 with appropriate type (u8, i32, f32, etc.)
2. **Given** source code converting boolean false to integer, **When** the compiler performs type promotion, **Then** the result is numeric value 0
3. **Given** source code converting non-zero integer to boolean, **When** the compiler performs type promotion, **Then** the result is boolean true
4. **Given** source code converting zero integer to boolean, **When** the compiler performs type promotion, **Then** the result is boolean false
5. **Given** source code converting char to u32, **When** the compiler performs type promotion, **Then** the result is the Unicode scalar value of the character
6. **Given** source code converting valid u32 Unicode value to char, **When** the compiler performs type promotion, **Then** a valid character is produced
7. **Given** source code converting invalid u32 value (e.g., 0xD800 surrogate or > 0x10FFFF) to char, **When** the compiler performs type promotion, **Then** a compilation error or runtime check prevents invalid character creation

---

### User Story 3 - String Conversions (Priority: P3)

Compiler users need to convert between string representations and primitive types for parsing user input and formatting output, including conversions from characters to strings and vice versa.

**Why this priority**: String conversions support I/O operations and user interaction scenarios. While important for complete functionality, they are typically handled at runtime through library functions rather than being critical to the core type system.

**Independent Test**: Can be fully tested by compiling programs that convert primitives to strings (42→"42", 3.14→"3.14", true→"true"), parse strings to primitives ("123"→123, "3.14"→3.14, "true"→true), and convert between char and single-character strings.

**Acceptance Scenarios**:

1. **Given** source code converting integer to string, **When** the compiler performs type promotion, **Then** the numeric value is formatted as a decimal string representation
2. **Given** source code converting float to string, **When** the compiler performs type promotion, **Then** the float value is formatted with appropriate precision (e.g., "3.14")
3. **Given** source code converting boolean to string, **When** the compiler performs type promotion, **Then** the result is either "true" or "false"
4. **Given** source code parsing valid numeric string to integer, **When** the compiler performs type promotion, **Then** the string is parsed to the correct numeric value
5. **Given** source code parsing invalid string to numeric type, **When** the compiler performs type promotion, **Then** an error is generated (compile-time or runtime)
6. **Given** source code converting single character to string, **When** the compiler performs type promotion, **Then** a single-character string is produced
7. **Given** source code converting single-character string to char, **When** the compiler performs type promotion, **Then** the character is extracted successfully
8. **Given** source code converting multi-character or empty string to char, **When** the compiler performs type promotion, **Then** an error is generated

---

### Edge Cases

- What happens when converting positive integers exceeding target type's maximum representable value to a smaller signed integer type (e.g., u64::MAX (18,446,744,073,709,551,615) to i32 with max 2,147,483,647, or any value > i32::MAX to i32)?
- What happens when converting negative signed integers to unsigned types (e.g., -1_i32 to u32)?
- **What happens when converting floating-point infinity or NaN to integer types?** → Generates `FloatSpecialValues` warning; behavior is deterministic based on OverflowBehavior configuration: **Wrap** mode produces NaN→0, +∞→INT_MAX (or UINT_MAX for unsigned), -∞→INT_MIN (or 0 for unsigned); **Saturate** mode produces same values (NaN→0, clamped to type bounds); **Trap** mode causes runtime panic; **CompileError** mode fails compilation for compile-time constants (see User Story 1, Acceptance Scenario #6 for complete specification)
- What happens when converting floating-point values outside the target integer range (e.g., 1e20_f64 to i32)?
- What happens when converting subnormal floating-point numbers between f32 and f64?
- What happens when converting integers larger than 24 bits (f32) or 53 bits (f64) to float types?
- What happens when converting between signed and unsigned types of the same width (e.g., i32 ↔ u32)?
- What happens when parsing strings with leading/trailing whitespace or special formatting?
- What happens when converting Unicode characters outside the Basic Multilingual Plane (BMP)?
- What happens when the target type cannot represent special values (e.g., bool cannot represent multiple non-zero values)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support all integer widening conversions (u8→u16→u32→u64 and i8→i16→i32→i64) without data loss using appropriate zero-extension or sign-extension
- **FR-002**: System MUST support all integer narrowing conversions with appropriate truncation and generate warnings about potential overflow
- **FR-003**: System MUST support conversions between signed and unsigned integers of the same width using bit reinterpretation
- **FR-004**: System MUST support bidirectional conversions between all integer types and both floating-point types (f32 and f64)
- **FR-005**: System MUST support bidirectional conversion between f32 and f64 floating-point types
- **FR-006**: System MUST support conversions from boolean to all numeric types (integers and floats) where true maps to 1 and false maps to 0
- **FR-007**: System MUST support conversions from all numeric types to boolean where zero maps to false and non-zero maps to true
- **FR-008**: System MUST support bidirectional conversions between char and u32 using Unicode scalar values
- **FR-009**: System MUST support conversions from char to String producing single-character strings
- **FR-010**: System MUST support conversions from String to char for single-character strings only
- **FR-011**: System MUST support conversions from all primitive types (integers, floats, bool) to String with appropriate formatting
- **FR-012**: System MUST support parsing conversions from String to numeric types (integers, floats) and boolean
- **FR-013**: System MUST generate warnings for conversions that may result in precision loss with quantitative criteria: (a) **Float→Integer**: ALWAYS warn (fractional part loss), (b) **f64→f32**: warn if value magnitude > f32::MAX (3.4028235e38) or subnormal range (< f32::MIN_POSITIVE = 1.1754944e-38), (c) **Integer→Float**: warn if integer value exceeds mantissa precision (f32: 24 bits = values > 16,777,216; f64: 53 bits = values > 9,007,199,254,740,992), (d) **Integer Narrowing**: warn if source value outside target range (e.g., u32 value > 65,535 to u16)
- **FR-014**: System MUST generate warnings for conversions that may result in value overflow or underflow with quantitative criteria: (a) **Overflow**: source value > target::MAX (e.g., 128 > i8::MAX=127, or 256 > u8::MAX=255), (b) **Underflow**: source value < target::MIN (e.g., -129 < i8::MIN=-128), (c) **Signed→Unsigned**: warn for negative values (e.g., -1_i32 to u32), (d) **Cross-signedness**: warn when converting between signed/unsigned of same width if value interpretation changes (e.g., i32::MIN=-2,147,483,648 to u32=2,147,483,648)
- **FR-015**: System MUST prevent creation of invalid char values from out-of-range Unicode code points
- **FR-016**: System MUST use the appropriate CastKind variant for each type of conversion as defined in the CastKind enum
- **FR-017**: System MUST handle floating-point special values during conversions as follows: (a) generate `FloatSpecialValues` warning when converting NaN, +∞, or -∞ to integer types, (b) apply OverflowBehavior configuration with deterministic semantics: **Wrap** produces NaN→0, +∞→INT_MAX (or UINT_MAX for unsigned), -∞→INT_MIN (or 0 for unsigned); **Saturate** clamps to INT_MIN/INT_MAX/0; **Trap** causes runtime panic; **CompileError** fails compilation for compile-time constants, (c) convert negative zero (-0.0) to integer 0 without warnings, (d) preserve NaN and infinity when converting between f32 and f64
- **FR-018**: System MUST provide clear error messages for invalid conversion attempts (e.g., multi-character string to char, unparseable string to numeric)
- **FR-019**: System MUST maintain consistency with the existing PromotionMatrix and TypePromotion infrastructure
- **FR-020**: System MUST support identity conversions (same type to same type) as no-op operations (note: the 13 identity conversions are included in the 169 total type conversion pairs: 13 types × 13 types = 169, where the diagonal represents identity conversions)

### Key Entities

- **TypePromotion**: Represents a single type casting operation, containing source type, destination type, CastKind variant, and flags indicating potential precision loss or overflow. Source location is included for error reporting.

- **PromotionMatrix**: A comprehensive lookup table defining all valid type conversion rules. Maps (from_type, to_type) pairs to PromotionRule instances. Includes configuration for overflow behavior (wrap, saturate, trap, compile error).

- **PromotionRule**: Defines how a specific type conversion should be performed. Can be Direct (single cast operation), Indirect (requires intermediate type), or Forbidden (conversion not allowed). Direct rules include CastKind, precision loss flag, and overflow flag.

- **CastKind**: Enumeration of all possible casting operations supported by the compiler, including integer extensions (zero/sign), integer truncation, integer-float conversions, float-float conversions, boolean conversions, character conversions, string conversions, and bitcasts.

- **PromotionWarning**: Represents warnings generated during type conversion analysis, including precision loss estimates, potential overflow notifications, signedness change alerts, and floating-point special value warnings.

- **PrecisionLossEstimate**: Quantifies the type of precision loss that may occur with specific thresholds: (a) **FractionalPartLoss**: float→integer conversions (ALWAYS triggers warning), (b) **MantissaOverflow**: integer exceeds float mantissa precision (f32: values > 2^24 = 16,777,216; f64: values > 2^53 = 9,007,199,254,740,992), (c) **FloatRangeReduction**: f64→f32 when |value| > f32::MAX (3.4028235e38) or subnormal (< f32::MIN_POSITIVE = 1.1754944e-38), (d) **ValueRangeNarrowing**: integer narrowing when source value outside target bounds (e.g., source > target::MAX or source < target::MIN).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 169 type conversion pairs (13 types × 13 types, including 13 identity conversions for same-type to same-type) are defined in the promotion matrix with appropriate rules (direct, indirect, or forbidden)
- **SC-002**: All 24 CastKind variants are utilized at least once in the promotion matrix for their intended conversions
- **SC-003**: Compiler correctly handles test programs performing at least 50 different type conversion scenarios without crashes or undefined behavior
- **SC-004**: Precision loss warnings are generated for 100% of conversions that may lose precision (float→int, f64→f32, large int→float, int narrowing)
- **SC-005**: Overflow warnings are generated for 100% of potentially unsafe narrowing conversions
- **SC-006**: Invalid conversions (e.g., invalid Unicode to char, unparseable string to number) produce clear error messages within 1 compiler pass
- **SC-007**: Identity conversions (same type to same type) complete in O(1) time without unnecessary processing
- **SC-008**: Test suite coverage for type promotion module reaches at least 95% of code paths
- **SC-009**: All edge cases enumerated in the specification have corresponding test cases with documented expected behavior
- **SC-010**: Compilation time for programs with extensive type conversions increases by no more than 5% compared to baseline

## Assumptions

- The existing CastKind enum in `src/ir/instruction.rs` contains all necessary cast operation variants for the required conversions
- The existing IrType enum supports all fundamental types mentioned (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, String, bool)
- String conversion operations will be marked as requiring runtime support, as string parsing cannot always be performed at compile time
- The overflow behavior configuration (Wrap, Saturate, Trap, CompileError) should apply consistently across all numeric conversions
- IEEE 754 floating-point representation is assumed for f32 and f64 types
- Unicode scalar values for char type follow Rust's definition (valid Unicode code points excluding surrogates)
- The existing PromotionMatrix infrastructure can be extended without breaking existing functionality

## Dependencies

- Existing `src/ir/types.rs` module defining IrType enum
- Existing `src/ir/instruction.rs` module defining CastKind enum
- Existing `src/ir/type_promotion.rs` infrastructure including PromotionMatrix, TypePromotion, and PromotionRule
- Test infrastructure in `tests/ir_type_promotion_tests.rs` for validation

## Constraints

- Must maintain backward compatibility with existing type promotion code
- Cannot change the public API of TypePromotion or PromotionMatrix structures
- Must follow existing code style and documentation conventions in the codebase
- Warning generation must not cause compilation failures unless overflow behavior is set to CompileError
- Performance overhead for promotion rule lookup must remain O(1) for common cases using the HashMap-based matrix

## Out of Scope

- Conversions involving pointer types (Pointer, Array types in IrType) are not included in this enhancement
- Conversions involving custom types (Custom, Struct in IrType) are not included in this enhancement
- Void type conversions are not applicable and excluded
- Optimization of generated IR code for type conversions (this focuses on correctness, not optimization)
- Runtime implementation of string parsing functions (specification covers compiler support for marking these conversions)
- Implicit type coercion rules (this focuses on explicit casting only)

- User-defined type conversion traits or operator overloading
