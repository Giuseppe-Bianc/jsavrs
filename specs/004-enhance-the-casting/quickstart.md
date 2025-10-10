# Quick Start Guide: Enhanced Type Casting System

**Feature**: Comprehensive Type Casting System Enhancement  
**Branch**: `004-enhance-the-casting`  
**Target Audience**: jsavrs compiler developers and contributors

---

## Overview

This guide provides a quick introduction to the enhanced type casting system in jsavrs. The system now supports comprehensive conversions among all 13 fundamental data types with 169 defined conversion rules.

---

## 5-Minute Quick Start

### What's New?

The enhanced casting system adds support for:
- ✅ **Integer narrowing** (u64→u16, i64→i8, etc.) with overflow warnings
- ✅ **Boolean conversions** (bool↔integers, bool↔floats, bool↔String)
- ✅ **Character conversions** (char↔integers, char↔String, Unicode validation)
- ✅ **String conversions** (String↔all primitives with parsing/formatting)

### Basic Usage

```rust
use jsavrs::ir::type_promotion::{PromotionMatrix, PromotionRule};
use jsavrs::ir::types::IrType;
use jsavrs::ir::instruction::CastKind;

// Create promotion matrix
let matrix = PromotionMatrix::new();

// Get promotion rule for integer narrowing
let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U16).unwrap();
match rule {
    PromotionRule::Direct { cast_kind, may_overflow, .. } => {
        println!("Cast: {:?}, May overflow: {}", cast_kind, may_overflow);
        // Output: Cast: IntTruncate, May overflow: true
    }
    _ => {}
}
```

---

## Key Concepts

### Type Conversion Categories

| Category | Example | CastKind | Flags |
|----------|---------|----------|-------|
| **Widening** | u8 → u32 | IntZeroExtend | No loss, no overflow |
| **Narrowing** | u64 → u16 | IntTruncate | Loss + overflow |
| **Same-width** | i32 ↔ u32 | IntBitcast | Signedness change |
| **Integer↔Float** | i32 → f32 | IntToFloat | Possible precision loss |
| **Boolean** | bool → i32 | BoolToInt | Always safe (0/1) |
| **Character** | char → u32 | CharToInt | Always safe (scalar) |
| **String** | i32 → String | IntToString | Runtime support |

### Promotion Matrix Lookup

```rust
// All 169 type pairs are defined
let matrix = PromotionMatrix::new();

// Get promotion rule (O(1) lookup)
if let Some(rule) = matrix.get_promotion_rule(&from_type, &to_type) {
    match rule {
        PromotionRule::Direct { cast_kind, .. } => {
            // Direct conversion with single cast
        }
        PromotionRule::Indirect { intermediate_type, .. } => {
            // Conversion via intermediate type
        }
        PromotionRule::Forbidden { reason } => {
            // Conversion not allowed
        }
    }
}
```

---

## Common Use Cases

### Use Case 1: Integer Narrowing with Overflow Detection

```rust
use jsavrs::ir::type_promotion::{PromotionMatrix, OverflowBehavior};
use jsavrs::ir::types::IrType;

// Create matrix with saturating overflow behavior
let matrix = PromotionMatrix::with_overflow_behavior(OverflowBehavior::Saturate);

// Check if narrowing conversion may overflow
let rule = matrix.get_promotion_rule(&IrType::I64, &IrType::I8).unwrap();
if let PromotionRule::Direct { may_overflow: true, .. } = rule {
    println!("⚠️  Warning: Conversion may overflow, values will saturate");
}
```

**Overflow Behavior Options**:
- `Wrap`: Modulo arithmetic (fastest, default in release builds)
- `Saturate`: Clamp to min/max (safe, predictable)
- `Trap`: Runtime panic on overflow (debug builds)
- `CompileError`: Error if overflow detected at compile-time

---

### Use Case 2: Boolean Conversions

```rust
use jsavrs::ir::type_promotion::PromotionMatrix;
use jsavrs::ir::types::IrType;
use jsavrs::ir::instruction::CastKind;

let matrix = PromotionMatrix::new();

// Boolean to integer: true→1, false→0
let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I32).unwrap();
assert_eq!(
    rule,
    &PromotionRule::Direct {
        cast_kind: CastKind::BoolToInt,
        may_lose_precision: false,
        may_overflow: false,
        requires_runtime_support: false,
        requires_validation: false,
    }
);

// Integer to boolean: 0→false, non-zero→true
let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::Bool).unwrap();
assert_eq!(
    rule.cast_kind,
    CastKind::IntToBool
);

// Float to boolean: 0.0→false, non-zero→true (NaN→true!)
let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::Bool).unwrap();
assert_eq!(
    rule.cast_kind,
    CastKind::FloatToBool
);
```

---

### Use Case 3: Character and Unicode Validation

```rust
use jsavrs::ir::type_promotion::PromotionMatrix;
use jsavrs::ir::types::IrType;
use jsavrs::ir::instruction::CastKind;

let matrix = PromotionMatrix::new();

// char → u32: Always safe (Unicode scalar value)
let rule = matrix.get_promotion_rule(&IrType::Char, &IrType::U32).unwrap();
assert_eq!(rule.cast_kind, CastKind::CharToInt);
assert_eq!(rule.may_overflow, false);

// u32 → char: Requires validation (exclude surrogates, >0x10FFFF)
let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();
assert_eq!(rule.cast_kind, CastKind::IntToChar);
assert_eq!(rule.requires_validation, true);  // Runtime check needed!

// Invalid Unicode code points:
// - U+D800 to U+DFFF (surrogates, reserved for UTF-16)
// - > U+10FFFF (beyond Unicode range)
```

**Unicode Validation Logic**:
```rust
fn is_valid_unicode_scalar(value: u32) -> bool {
    value <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&value)
}
```

---

### Use Case 4: String Conversions (Runtime Support)

```rust
use jsavrs::ir::type_promotion::PromotionMatrix;
use jsavrs::ir::types::IrType;
use jsavrs::ir::instruction::CastKind;

let matrix = PromotionMatrix::new();

// Integer to String: Formatting (always succeeds)
let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::String).unwrap();
assert_eq!(rule.cast_kind, CastKind::IntToString);
assert_eq!(rule.requires_runtime_support, true);  // Heap allocation

// String to Integer: Parsing (may fail at runtime)
let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I32).unwrap();
assert_eq!(rule.cast_kind, CastKind::StringToInt);
assert_eq!(rule.requires_runtime_support, true);  // Runtime function
assert_eq!(rule.requires_validation, true);       // Parse can fail!

// String to char: Length check required
let rule = matrix.get_promotion_rule(&IrType::String, &IrType::Char).unwrap();
assert_eq!(rule.requires_validation, true);  // Must be exactly 1 char
```

---

## Warning System

### Precision Loss Warnings

```rust
use jsavrs::ir::type_promotion::{PromotionMatrix, PrecisionLossEstimate, PromotionWarning};
use jsavrs::ir::types::IrType;

let matrix = PromotionMatrix::new();

// Float to integer: Fractional part lost
let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::I32).unwrap();
if rule.may_lose_precision {
    // Generate warning: PrecisionLoss { estimated_loss: FractionalPart }
}

// f64 to f32: Mantissa precision lost
let rule = matrix.get_promotion_rule(&IrType::F64, &IrType::F32).unwrap();
if rule.may_lose_precision {
    // Generate warning: PrecisionLoss { estimated_loss: SignificantDigits { lost_bits: 29 } }
}

// Integer narrowing: High bits lost
let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U16).unwrap();
if rule.may_lose_precision {
    // Generate warning: PrecisionLoss { estimated_loss: ValueRange { from_bits: 64, to_bits: 16 } }
}
```

### Signedness Change Warnings

```rust
use jsavrs::ir::type_promotion::{PromotionMatrix, PromotionWarning};
use jsavrs::ir::types::IrType;

let matrix = PromotionMatrix::new();

// Same-width signed↔unsigned: Bit reinterpretation
let rule = matrix.get_promotion_rule(&IrType::I32, &IrType::U32).unwrap();
// Generates: SignednessChange { from_signed: true, to_signed: false, may_affect_comparisons: true }

// Examples of potential issues:
// -1_i32 as u32 → 4294967295_u32 (two's complement)
// -1_i32 < 0 → true, but 4294967295_u32 < 0 → false
```

---

## Type Lattice and Common Types

### Type Precedence Hierarchy

```
F64 (highest precedence)
 ↑
F32
 ↑
I64 / U64
 ↑
I32 / U32
 ↑
I16 / U16
 ↑
I8 / U8
 ↑
Bool / Char (lowest precedence)
```

### Computing Common Type

```rust
use jsavrs::ir::type_promotion::PromotionMatrix;
use jsavrs::ir::types::IrType;

let matrix = PromotionMatrix::new();

// Float takes precedence over integer
let common = matrix.compute_common_type(&IrType::I32, &IrType::F32).unwrap();
assert_eq!(common, IrType::F32);

// Wider integer takes precedence
let common = matrix.compute_common_type(&IrType::U16, &IrType::I64).unwrap();
assert_eq!(common, IrType::I64);

// Same-width signed/unsigned → promote to next larger signed
let common = matrix.compute_common_type(&IrType::I32, &IrType::U32).unwrap();
assert_eq!(common, IrType::I64);
```

---

## Performance Characteristics

### Lookup Performance

- **Average Case**: O(1) via HashMap lookup
- **Worst Case**: O(1) (hash collision rare for enum keys)
- **Identity Conversions**: Fast path (equality check before HashMap lookup)
- **Memory Overhead**: ~10.8 KB for 169-entry matrix (negligible)

### Benchmark Example

```rust
use jsavrs::ir::type_promotion::PromotionMatrix;
use jsavrs::ir::types::IrType;
use std::time::Instant;

let matrix = PromotionMatrix::new();
let start = Instant::now();

// Lookup 10,000 promotion rules
for _ in 0..10_000 {
    let _ = matrix.get_promotion_rule(&IrType::I32, &IrType::F64);
}

let elapsed = start.elapsed();
println!("10,000 lookups: {:?}", elapsed);
// Expected: < 1ms
```

---

## Testing

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use jsavrs::ir::type_promotion::PromotionMatrix;
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::instruction::CastKind;

    #[test]
    fn test_integer_narrowing_u64_to_u16() {
        let matrix = PromotionMatrix::new();
        let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U16).unwrap();

        assert_eq!(
            rule,
            &PromotionRule::Direct {
                cast_kind: CastKind::IntTruncate,
                may_lose_precision: true,
                may_overflow: true,
                requires_runtime_support: false,
                requires_validation: false,
            }
        );
    }

    #[test]
    fn test_all_169_pairs_defined() {
        let matrix = PromotionMatrix::new();
        let types = vec![
            IrType::I8, IrType::I16, IrType::I32, IrType::I64,
            IrType::U8, IrType::U16, IrType::U32, IrType::U64,
            IrType::F32, IrType::F64,
            IrType::Bool, IrType::Char, IrType::String,
        ];

        for from in &types {
            for to in &types {
                assert!(
                    matrix.get_promotion_rule(from, to).is_some(),
                    "Missing rule for {:?} → {:?}",
                    from,
                    to
                );
            }
        }
    }
}
```

---

## FAQ

### Q: What happens if I try to convert a string "abc" to an integer?

**A**: The `StringToInt` cast kind has `requires_validation=true`, which means the code generation phase will emit a runtime check. At runtime, the parsing will fail and generate an `InvalidStringConversion` warning or error (depending on error handling configuration).

### Q: Are char values guaranteed to be valid Unicode?

**A**: Yes. The `IntToChar` cast kind has `requires_validation=true`, which ensures that invalid Unicode scalar values (surrogates U+D800–U+DFFF and values >U+10FFFF) are rejected at compile-time (if statically known) or runtime (if dynamic).

### Q: How do I handle NaN in float-to-integer conversions?

**A**: The `FloatToInt` cast generates a `FloatSpecialValues` warning when NaN or infinity may be involved. The runtime behavior depends on `OverflowBehavior`:
- `Wrap`: Platform-dependent (typically undefined behavior)
- `Saturate`: Clamp to integer min/max (NaN→0 by convention)
- `Trap`: Runtime panic on NaN/infinity
- `CompileError`: Error if statically detectable

### Q: What's the difference between `IntBitcast` and `IntTruncate`?

**A**: 
- **IntBitcast**: Same-width reinterpretation (i32↔u32), preserves bit pattern but changes signedness
- **IntTruncate**: Narrowing conversion (u64→u16), discards high-order bits

### Q: Can I customize overflow behavior per conversion?

**A**: Currently, `OverflowBehavior` is set globally for the entire `PromotionMatrix`. Future enhancements may support per-conversion override.

---

## Next Steps

1. **Read the full specification**: See `spec.md` for comprehensive requirements
2. **Review the data model**: See `data-model.md` for detailed entity definitions
3. **Check the API contract**: See `contracts/promotion_matrix_api.md` for API guarantees
4. **Run the test suite**: `cargo test ir_type_promotion` to verify implementation
5. **Benchmark performance**: `cargo bench jsavrs_benchmark` to measure optimization impact

---

## Resources

- **Implementation**: `src/ir/type_promotion.rs`
- **Tests**: `tests/ir_type_promotion_tests.rs`
- **CastKind Enum**: `src/ir/instruction.rs`
- **IrType Enum**: `src/ir/types.rs`
- **Benchmarks**: `benches/jsavrs_benchmark.rs`

---

**Quick Start Status**: ✅ COMPLETE - Ready for development
