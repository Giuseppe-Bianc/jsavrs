# API Contract: PromotionMatrix Enhancement

**Module**: `src/ir/type_promotion.rs`  
**Version**: 1.1.0 (Enhanced Casting Support)  
**Stability**: Internal API (not public crate API)

---

## Public API (No Breaking Changes)

### `PromotionMatrix::new() -> Self`

**Purpose**: Creates a new PromotionMatrix with default promotion rules

**Contract**:
- **Preconditions**: None
- **Postconditions**: 
  - Returns fully initialized PromotionMatrix
  - All 169 type conversion pairs are defined
  - Default overflow behavior is `OverflowBehavior::Saturate`
- **Invariants**: Matrix never returns `None` for any (IrType, IrType) pair from the 13 fundamental types

**No Changes**: Signature and behavior remain backward compatible

---

### `PromotionMatrix::with_overflow_behavior(behavior: OverflowBehavior) -> Self`

**Purpose**: Creates PromotionMatrix with specified overflow behavior

**Contract**:
- **Preconditions**: `behavior` must be valid `OverflowBehavior` variant
- **Postconditions**: Same as `new()` but with custom overflow behavior
- **Invariants**: Overflow behavior affects only integer narrowing and float-to-integer conversions

**No Changes**: Signature and behavior remain backward compatible

---

### `PromotionMatrix::get_promotion_rule(&self, from: &IrType, to: &IrType) -> Option<&PromotionRule>`

**Purpose**: Retrieves promotion rule for a type pair

**Contract**:
- **Preconditions**: `from` and `to` must be valid `IrType` variants
- **Postconditions**: 
  - Returns `Some(&PromotionRule)` for all 13 × 13 fundamental type pairs
  - Returns `None` for unsupported types (Pointer, Custom, Struct, Void)
- **Performance**: O(1) average case (HashMap lookup)
- **Invariants**: Same rule returned for same inputs throughout matrix lifetime

**No Changes**: Signature and behavior remain backward compatible

---

### `PromotionMatrix::compute_common_type(&self, left: &IrType, right: &IrType) -> Option<IrType>`

**Purpose**: Determines common type for binary operation operands

**Contract**:
- **Preconditions**: `left` and `right` must be valid `IrType` variants
- **Postconditions**: 
  - Returns `Some(IrType)` representing common type based on type lattice
  - Returns `None` if no common type exists
- **Type Lattice Rules**:
  1. Float types precedence: F64 > F32 > integers
  2. Signed/unsigned same width → promote to next larger signed
  3. Within same signedness, wider type takes precedence
- **Invariants**: Commutative (same result for (A, B) and (B, A))

**No Changes**: Signature and behavior remain backward compatible

---

## Internal API (Enhanced Behavior)

### `PromotionMatrix::initialize_default_promotions(&mut self)` (private)

**Purpose**: Initializes all 169 type conversion rules

**Contract**:
- **Preconditions**: `promotion_rules` HashMap is empty or being reinitialized
- **Postconditions**: 
  - All 169 fundamental type pairs have defined rules
  - All 24 CastKind variants are utilized at least once
  - Identity conversions (T→T) use `Bitcast` with no loss/overflow flags
- **Invariants**: 
  - No duplicate rules for same type pair
  - All integer widening rules have `may_lose_precision=false`
  - All integer narrowing rules have `may_overflow=true`
  - All string conversions have `requires_runtime_support=true`

**Enhancement**: Adds calls to new helper methods:
- `add_integer_narrowing_promotions()`
- `add_boolean_promotions()`
- `add_character_promotions()`
- `add_string_promotions()`

---

### New Internal Methods

#### `add_integer_narrowing_promotions(&mut self)` (private)

**Purpose**: Adds promotion rules for integer narrowing conversions (24 rules)

**Contract**:
- **Preconditions**: Called from `initialize_default_promotions()`
- **Postconditions**: 
  - All narrowing conversions within same signedness defined
  - Examples: i64→i32, i64→i16, i64→i8, u64→u32, etc.
- **Rule Properties**:
  - CastKind: `IntTruncate`
  - Flags: `may_lose_precision=true`, `may_overflow=true`
  - Warning: `PrecisionLoss` with `ValueRange` estimate

**Performance**: O(n²) initialization, but called once at startup

---

#### `add_boolean_promotions(&mut self)` (private)

**Purpose**: Adds promotion rules for boolean conversions (24 rules)

**Contract**:
- **Preconditions**: Called from `initialize_default_promotions()`
- **Postconditions**: Boolean conversions to/from all numeric types and string defined
- **Rule Categories**:
  1. bool → integers (8 rules): `BoolToInt`, 0/1 mapping
  2. bool → floats (2 rules): `BoolToFloat`, 0.0/1.0 mapping
  3. integers → bool (8 rules): `IntToBool`, zero test
  4. floats → bool (2 rules): `FloatToBool`, zero test (NaN→true)
  5. bool ↔ String (2 rules): Runtime support required
  6. bool ↔ char (2 rules): Indirect via u32

**Invariants**: 
- All boolean conversions have `may_lose_precision=false`
- All boolean conversions have `may_overflow=false` (0/1 always fits)

---

#### `add_character_promotions(&mut self)` (private)

**Purpose**: Adds promotion rules for character conversions (14 rules)

**Contract**:
- **Preconditions**: Called from `initialize_default_promotions()`
- **Postconditions**: Character conversions to/from integers and string defined
- **Rule Categories**:
  1. char → u32 (1 rule): `CharToInt`, Unicode scalar extraction
  2. u32 → char (1 rule): `IntToChar`, validated conversion
  3. char → other integers (6 rules): Indirect via u32
  4. other integers → char (6 rules): Indirect via u32
  5. char ↔ String (2 rules): Runtime support required

**Validation**:
- u32 → char: Rejects surrogates (U+D800–U+DFFF) and out-of-range (>U+10FFFF)
- String → char: Requires length==1 validation

---

#### `add_string_promotions(&mut self)` (private)

**Purpose**: Adds promotion rules for string conversions (25 rules)

**Contract**:
- **Preconditions**: Called from `initialize_default_promotions()`
- **Postconditions**: String conversions to/from all primitives defined
- **Rule Categories**:
  1. integers → String (8 rules): `IntToString`
  2. floats → String (2 rules): `FloatToString`
  3. bool → String (1 rule): `BoolToString`
  4. char → String (1 rule): `CharToString`
  5. String → integers (8 rules): `StringToInt`, parsing
  6. String → floats (2 rules): `StringToFloat`, parsing
  7. String → bool (1 rule): `StringToBool`, parsing
  8. String → char (1 rule): `StringToChar`, length check
  9. String → String (1 rule): Identity via `Bitcast`

**Invariants**:
- All string conversions have `requires_runtime_support=true`
- All String→primitive conversions have `requires_validation=true`
- Formatting conversions always succeed (primitive→String)
- Parsing conversions are fallible (String→primitive)

---

## Data Structure Enhancements

### `PromotionRule::Direct` Enhancements

**Current**:
```rust
Direct {
    cast_kind: CastKind,
    may_lose_precision: bool,
    may_overflow: bool,
}
```

**Enhanced**:
```rust
Direct {
    cast_kind: CastKind,
    may_lose_precision: bool,
    may_overflow: bool,
    requires_runtime_support: bool,    // NEW
    requires_validation: bool,         // NEW
}
```

**Migration Strategy**: 
- Add fields with default values to maintain backward compatibility
- Existing rules get `requires_runtime_support=false`, `requires_validation=false`
- Only string and validated conversions use these flags

---

### `PromotionWarning` Enhancements

**New Variants**:
```rust
pub enum PromotionWarning {
    // ... existing variants ...
    
    /// Invalid string conversion (unparseable)
    InvalidStringConversion {
        string_value: Option<String>,
        target_type: IrType,
        reason: String,
    },
    
    /// Invalid Unicode code point for char
    InvalidUnicodeCodePoint {
        value: u32,
        reason: String,
    },
}
```

**Usage**:
- `InvalidStringConversion`: Generated when string parsing fails (e.g., "abc" → i32)
- `InvalidUnicodeCodePoint`: Generated when u32 value is invalid Unicode scalar (surrogates, >0x10FFFF)

---

## Invariants and Guarantees

### Completeness Guarantee
**Property**: All 169 fundamental type pairs have defined promotion rules  
**Verification**: Test case validates HashMap contains all expected keys

### Consistency Guarantee
**Property**: Same type pair always returns same promotion rule  
**Verification**: HashMap ensures deterministic lookup

### Safety Guarantee
**Property**: Invalid conversions (e.g., invalid Unicode) are caught at compile-time or runtime  
**Verification**: Validation flags trigger appropriate checks in code generation phase

### Performance Guarantee
**Property**: Promotion rule lookup is O(1) average case  
**Verification**: Benchmark test verifies <1ms for promotion analysis

---

## Usage Examples

### Example 1: Integer Narrowing

```rust
let matrix = PromotionMatrix::new();
let rule = matrix.get_promotion_rule(&IrType::U64, &IrType::U16).unwrap();

assert_eq!(rule, &PromotionRule::Direct {
    cast_kind: CastKind::IntTruncate,
    may_lose_precision: true,
    may_overflow: true,
    requires_runtime_support: false,
    requires_validation: false,
});
```

### Example 2: Boolean to Integer

```rust
let matrix = PromotionMatrix::new();
let rule = matrix.get_promotion_rule(&IrType::Bool, &IrType::I32).unwrap();

assert_eq!(rule, &PromotionRule::Direct {
    cast_kind: CastKind::BoolToInt,
    may_lose_precision: false,
    may_overflow: false,
    requires_runtime_support: false,
    requires_validation: false,
});
```

### Example 3: String to Integer (Runtime Support)

```rust
let matrix = PromotionMatrix::new();
let rule = matrix.get_promotion_rule(&IrType::String, &IrType::I32).unwrap();

assert_eq!(rule, &PromotionRule::Direct {
    cast_kind: CastKind::StringToInt,
    may_lose_precision: false,
    may_overflow: false,
    requires_runtime_support: true,
    requires_validation: true,
});
```

### Example 4: u32 to char (Validation Required)

```rust
let matrix = PromotionMatrix::new();
let rule = matrix.get_promotion_rule(&IrType::U32, &IrType::Char).unwrap();

assert_eq!(rule, &PromotionRule::Direct {
    cast_kind: CastKind::IntToChar,
    may_lose_precision: false,
    may_overflow: false,
    requires_runtime_support: false,
    requires_validation: true,  // Unicode range check
});
```

---

## Error Handling

### Compile-Time Errors
- Invalid type pair (unsupported types like Void, Pointer)
- Invalid Unicode scalar value (statically detectable)
- Invalid string literal parsing (const evaluation)

### Runtime Errors
- String parsing failure → `InvalidStringConversion` warning
- Invalid Unicode value → `InvalidUnicodeCodePoint` warning
- Overflow based on `OverflowBehavior` configuration

### Warning Generation
- Precision loss warnings for lossy conversions
- Overflow warnings for potentially unsafe narrowing
- Signedness change warnings for same-width signed↔unsigned

---

## Testing Contract

### Unit Tests Required
- ✅ All 169 type pairs have defined rules
- ✅ All 24 CastKind variants are used
- ✅ Identity conversions use Bitcast with no flags
- ✅ Integer widening has no precision loss
- ✅ Integer narrowing has overflow flag
- ✅ Boolean conversions have no precision loss
- ✅ String conversions have runtime support flag
- ✅ Validated conversions have validation flag

### Integration Tests Required
- ✅ Type promotion analysis produces correct warnings
- ✅ Overflow behavior configuration affects narrowing
- ✅ Common type computation follows type lattice
- ✅ HashMap lookup performance is O(1)

### Snapshot Tests Required
- ✅ Warning message format consistency
- ✅ Error message clarity for invalid conversions

---

**Contract Status**: ✅ COMPLETE - Ready for implementation
