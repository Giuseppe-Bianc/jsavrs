# Data Model: Type Casting System Enhancement

**Feature**: Comprehensive Type Casting System Enhancement  
**Branch**: `004-enhance-the-casting`  
**Date**: 2025-10-08  
**Prerequisites**: research.md complete

---

## Overview

This document defines the data structures, relationships, and validation rules for the enhanced type casting system. The design extends the existing `PromotionMatrix` infrastructure to support all 169 type conversion pairs across 13 fundamental types, while maintaining backward compatibility with the existing API.

---

## Core Entities

### 1. IrType (Existing - No Changes)

**Location**: `src/ir/types.rs`  
**Purpose**: Represents all IR types in the jsavrs compiler

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum IrType {
    // Integer Types (8 variants)
    I8, I16, I32, I64,      // Signed integers
    U8, U16, U32, U64,      // Unsigned integers
    
    // Floating-Point Types (2 variants)
    F32, F64,
    
    // Other Primitive Types (3 variants)
    Bool,                   // Boolean type
    Char,                   // Unicode scalar value
    String,                 // String type
    
    // Non-Primitive Types (not in scope for this feature)
    #[default]
    Void,
    Pointer(Box<IrType>),
    Array(Box<IrType>, usize),
    Custom(Arc<str>, SourceSpan),
    Struct(Arc<str>, Vec<(String, IrType)>, SourceSpan),
}
```

**Relationships**: 
- Used by `TypePromotion` (from_type, to_type fields)
- Used as HashMap key in `PromotionMatrix`
- Derives `Hash`, `Eq` for HashMap usage

**Validation Rules**:
- No validation changes required
- Existing validation ensures type consistency

**State Transitions**: N/A (immutable enum)

---

### 2. CastKind (Existing - No Changes)

**Location**: `src/ir/instruction.rs`  
**Purpose**: Enumerates all possible type cast operations

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CastKind {
    // Integer Extension (2 variants)
    #[default]
    IntZeroExtend,          // u8 → u32 (zero-extend)
    IntSignExtend,          // i8 → i32 (sign-extend)
    IntTruncate,            // u64 → u16 (truncate high bits)
    
    // Same-Width Integer Reinterpretation (1 variant)
    IntBitcast,             // i32 ↔ u32 (bit reinterpret)
    
    // Integer ↔ Float (2 variants)
    IntToFloat,             // i32 → f32, u64 → f64
    FloatToInt,             // f32 → i32, f64 → u64
    
    // Float ↔ Float (2 variants)
    FloatTruncate,          // f64 → f32
    FloatExtend,            // f32 → f64
    
    // Boolean ↔ Numeric (4 variants)
    BoolToInt,              // bool → u8/i32 (0 or 1)
    IntToBool,              // i32 → bool (zero test)
    BoolToFloat,            // bool → f32/f64 (0.0 or 1.0)
    FloatToBool,            // f32/f64 → bool (zero test)
    
    // Character ↔ Integer (2 variants)
    CharToInt,              // char → u32 (Unicode scalar)
    IntToChar,              // u32 → char (validated)
    
    // Character ↔ String (2 variants)
    CharToString,           // char → String
    StringToChar,           // String (len==1) → char
    
    // Primitive ↔ String (6 variants)
    StringToInt,            // "123" → 123_i32
    StringToFloat,          // "3.14" → 3.14_f64
    StringToBool,           // "true" → true
    IntToString,            // 42 → "42"
    FloatToString,          // 3.14 → "3.14"
    BoolToString,           // true → "true"
    
    // Bit Reinterpretation (1 variant)
    Bitcast,                // Same-size types (f32↔u32, f64↔u64)
}
```

**Total**: 24 variants

**Usage**: Referenced by `PromotionRule::Direct` and `TypePromotion::cast_kind`

---

### 3. PromotionRule (Existing - Enhanced)

**Location**: `src/ir/type_promotion.rs`  
**Purpose**: Defines how a specific type conversion should be performed

```rust
#[derive(Debug, Clone, PartialEq)]
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

**Enhancement Required**: Add runtime support flags

```rust
/// ENHANCED VERSION (to be implemented)
#[derive(Debug, Clone, PartialEq)]
pub enum PromotionRule {
    Direct {
        cast_kind: CastKind,
        may_lose_precision: bool,
        may_overflow: bool,
        requires_runtime_support: bool,    // NEW: For string conversions
        requires_validation: bool,         // NEW: For u32→char, String→primitive
    },
    Indirect {
        intermediate_type: IrType,
        first_cast: CastKind,
        second_cast: CastKind,
        requires_runtime_support: bool,    // NEW
    },
    Forbidden {
        reason: String,
    },
}
```

**Validation Rules**:
- `requires_validation=true` implies runtime check generation
- `requires_runtime_support=true` implies runtime function call generation
- String conversions MUST set `requires_runtime_support=true`
- u32→char and String→primitive MUST set `requires_validation=true`

**Relationships**:
- Stored in `PromotionMatrix::promotion_rules` HashMap
- Referenced during type promotion analysis

---

### 4. TypePromotion (Existing - No Changes)

**Location**: `src/ir/type_promotion.rs`  
**Purpose**: Represents a single type casting operation

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TypePromotion {
    pub from_type: IrType,              // Source type
    pub to_type: IrType,                // Target type
    pub cast_kind: CastKind,            // Cast operation
    pub may_lose_precision: bool,       // Precision loss flag
    pub may_overflow: bool,             // Overflow flag
    pub source_span: SourceSpan,        // For error reporting
}
```

**Validation Rules**:
- `from_type` and `to_type` must be valid `IrType` variants
- `cast_kind` must be appropriate for the type pair
- `source_span` must be valid for error reporting

**State Transitions**: N/A (immutable after construction)

---

### 5. PromotionMatrix (Existing - Enhanced)

**Location**: `src/ir/type_promotion.rs`  
**Purpose**: Central registry of all type promotion rules

```rust
#[derive(Debug, Clone)]
pub struct PromotionMatrix {
    /// Matrix of promotion rules indexed by (from_type, to_type)
    promotion_rules: HashMap<(IrType, IrType), PromotionRule>,
    
    /// Type precedence ordering for automatic promotion
    type_precedence: Vec<TypeGroup>,
    
    /// Configuration for runtime behavior on errors
    overflow_behavior: OverflowBehavior,
}
```

**Current State**: ~50 promotion rules defined  
**Target State**: 169 promotion rules (13 types × 13 types)

**Initialization Enhancement Required**:
```rust
impl PromotionMatrix {
    fn initialize_default_promotions(&mut self) {
        // EXISTING: Float promotions (2 rules)
        self.add_float_promotions();
        
        // EXISTING: Integer widening (12 + 12 = 24 rules)
        self.add_integer_widening_promotions();
        
        // EXISTING: Float-Integer conversions (16 rules)
        self.add_float_integer_promotions();
        
        // EXISTING: Cross-signedness same-width (8 rules)
        self.add_cross_signedness_promotions();
        
        // EXISTING: Identity promotions (12 rules)
        self.add_identity_promotions();
        
        // NEW: Integer narrowing (24 rules)
        self.add_integer_narrowing_promotions();
        
        // NEW: Boolean conversions (24 rules)
        self.add_boolean_promotions();
        
        // NEW: Character conversions (6 rules)
        self.add_character_promotions();
        
        // NEW: String conversions (28 rules)
        self.add_string_promotions();
        
        // Total: ~156 rules + identity = 169 rules
    }
}
```

**Validation Rules**:
- All 169 type pairs must have a defined rule
- No duplicate rules for the same type pair
- Identity rules (T→T) must use `Bitcast` with no loss/overflow flags

---

### 6. PromotionWarning (Existing - Enhanced)

**Location**: `src/ir/type_promotion.rs`  
**Purpose**: Represents warnings generated during type promotion

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PromotionWarning {
    /// Precision loss during conversion
    PrecisionLoss {
        from_type: IrType,
        to_type: IrType,
        estimated_loss: PrecisionLossEstimate,
    },
    
    /// Potential overflow during conversion
    PotentialOverflow {
        from_type: IrType,
        to_type: IrType,
        operation: IrBinaryOp,
    },
    
    /// Signedness change may affect semantics
    SignednessChange {
        from_signed: bool,
        to_signed: bool,
        may_affect_comparisons: bool,
    },
    
    /// Warning generated when converting floating-point special values
    /// (NaN, positive infinity, negative infinity) to integer types.
    /// The actual conversion behavior depends on OverflowBehavior configuration.
    FloatSpecialValues {
        /// The specific special value being converted
        value_type: FloatSpecialValueType,
        
        /// Source floating-point type (F32 or F64)
        source_type: IrType,
        
        /// Target integer type (I8-I64, U8-U64)
        target_type: IrType,
        
        /// The overflow behavior that will be applied
        applied_behavior: OverflowBehavior,
        
        /// Source location for error reporting
        source_span: SourceSpan,
    },
}

/// Types of floating-point special values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSpecialValueType {
    NaN,
    PositiveInfinity,
    NegativeInfinity,
}
```

**Enhancement Required**: Add new warning variants

```rust
/// NEW VARIANTS (to be added)
pub enum PromotionWarning {
    // ... existing variants ...
    
    /// Invalid string conversion (unparseable)
    InvalidStringConversion {
        string_value: Option<String>,  // If statically known
        target_type: IrType,
        reason: String,
    },
    
    /// Invalid Unicode code point for char
    InvalidUnicodeCodePoint {
        value: u32,
        reason: String,  // "surrogate", "out of range", etc.
    },
}
```

---

### 7. PrecisionLossEstimate (Existing - No Changes)

**Location**: `src/ir/type_promotion.rs`  
**Purpose**: Quantifies potential precision loss

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PrecisionLossEstimate {
    None,                                   // No precision loss
    FractionalPart,                         // Float → Int
    SignificantDigits { lost_bits: u32 },   // f64 → f32
    ValueRange { from_bits: u32, to_bits: u32 }, // u64 → u16
}
```

**Usage Examples**:
- Integer narrowing: `ValueRange { from_bits: 64, to_bits: 16 }`
- f64 → f32: `SignificantDigits { lost_bits: 29 }`
- Float → Int: `FractionalPart`

---

### 8. OverflowBehavior (Existing - No Changes)

**Location**: `src/ir/type_promotion.rs`  
**Purpose**: Configures runtime behavior on numeric overflow

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverflowBehavior {
    Wrap,           // Modulo arithmetic (fastest)
    Saturate,       // Clamp to min/max (safe)
    Trap,           // Runtime panic (debug)
    CompileError,   // Compile-time error when detectable
}
```

**Default**: `Saturate` (per existing implementation)

**Applicable Conversions**:
- Integer narrowing (u64 → u16, i64 → i8)
- Float → Integer (f64 → i32)
- Not applicable: Widening, boolean, char, string conversions

---

## Promotion Rule Matrix (169 Rules)

### Matrix Dimensions
- **13 types**: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, String
- **169 total pairs**: 13 × 13 (including identity conversions)

### Rule Categories

#### Category 1: Identity Conversions (13 rules)
**Pattern**: T → T for all 13 types

| From | To | CastKind | Flags |
|------|-----|----------|-------|
| i8 | i8 | Bitcast | precision=false, overflow=false |
| ... | ... | ... | ... (same for all 13 types) |

#### Category 2: Integer Widening (24 rules)
**Pattern**: Smaller → Larger within same signedness

| From | To | CastKind | Flags |
|------|-----|----------|-------|
| i8 | i16 | IntSignExtend | precision=false, overflow=false |
| i8 | i32 | IntSignExtend | precision=false, overflow=false |
| i8 | i64 | IntSignExtend | precision=false, overflow=false |
| i16 | i32 | IntSignExtend | precision=false, overflow=false |
| i16 | i64 | IntSignExtend | precision=false, overflow=false |
| i32 | i64 | IntSignExtend | precision=false, overflow=false |
| u8 | u16 | IntZeroExtend | precision=false, overflow=false |
| u8 | u32 | IntZeroExtend | precision=false, overflow=false |
| u8 | u64 | IntZeroExtend | precision=false, overflow=false |
| u16 | u32 | IntZeroExtend | precision=false, overflow=false |
| u16 | u64 | IntZeroExtend | precision=false, overflow=false |
| u32 | u64 | IntZeroExtend | precision=false, overflow=false |

**Total**: 6 + 6 = 12 signed + 12 unsigned = 24 rules

#### Category 3: Integer Narrowing (24 rules)
**Pattern**: Larger → Smaller within same signedness

| From | To | CastKind | Flags |
|------|-----|----------|-------|
| i16 | i8 | IntTruncate | precision=true, overflow=true |
| i32 | i8 | IntTruncate | precision=true, overflow=true |
| i32 | i16 | IntTruncate | precision=true, overflow=true |
| i64 | i8 | IntTruncate | precision=true, overflow=true |
| i64 | i16 | IntTruncate | precision=true, overflow=true |
| i64 | i32 | IntTruncate | precision=true, overflow=true |
| u16 | u8 | IntTruncate | precision=true, overflow=true |
| u32 | u8 | IntTruncate | precision=true, overflow=true |
| u32 | u16 | IntTruncate | precision=true, overflow=true |
| u64 | u8 | IntTruncate | precision=true, overflow=true |
| u64 | u16 | IntTruncate | precision=true, overflow=true |
| u64 | u32 | IntTruncate | precision=true, overflow=true |

**Total**: 6 + 6 = 12 signed + 12 unsigned = 24 rules

#### Category 4: Cross-Signedness Same-Width (8 rules)
**Pattern**: Signed ↔ Unsigned of same bit width

| From | To | CastKind | Flags | Warning |
|------|-----|----------|-------|---------|
| i8 | u8 | IntBitcast | precision=false, overflow=false | SignednessChange |
| u8 | i8 | IntBitcast | precision=false, overflow=false | SignednessChange |
| i16 | u16 | IntBitcast | precision=false, overflow=false | SignednessChange |
| u16 | i16 | IntBitcast | precision=false, overflow=false | SignednessChange |
| i32 | u32 | IntBitcast | precision=false, overflow=false | SignednessChange |
| u32 | i32 | IntBitcast | precision=false, overflow=false | SignednessChange |
| i64 | u64 | IntBitcast | precision=false, overflow=false | SignednessChange |
| u64 | i64 | IntBitcast | precision=false, overflow=false | SignednessChange |

**Total**: 8 rules

#### Category 5: Integer ↔ Float (32 rules)
**Pattern**: All integer types ↔ f32/f64

**Integer → Float** (16 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| i8/i16/i32/i64 | f32 | IntToFloat | precision=false (unless i64), overflow=false |
| i8/i16/i32/i64 | f64 | IntToFloat | precision=false, overflow=false |
| u8/u16/u32/u64 | f32 | IntToFloat | precision=false (unless u64), overflow=false |
| u8/u16/u32/u64 | f64 | IntToFloat | precision=false, overflow=false |

**Float → Integer** (16 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| f32 | i8/i16/i32/i64 | FloatToInt | precision=true, overflow=true |
| f64 | i8/i16/i32/i64 | FloatToInt | precision=true, overflow=true |
| f32 | u8/u16/u32/u64 | FloatToInt | precision=true, overflow=true |
| f64 | u8/u16/u32/u64 | FloatToInt | precision=true, overflow=true |

**Total**: 16 + 16 = 32 rules

#### Category 6: Float ↔ Float (2 rules)
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| f32 | f64 | FloatExtend | precision=false, overflow=false |
| f64 | f32 | FloatTruncate | precision=true, overflow=false |

**Total**: 2 rules

#### Category 7: Boolean Conversions (24 rules)

**Boolean → Numeric** (10 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| bool | i8/i16/i32/i64 | BoolToInt | precision=false, overflow=false |
| bool | u8/u16/u32/u64 | BoolToInt | precision=false, overflow=false |
| bool | f32 | BoolToFloat | precision=false, overflow=false |
| bool | f64 | BoolToFloat | precision=false, overflow=false |

**Numeric → Boolean** (10 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| i8/i16/i32/i64 | bool | IntToBool | precision=false, overflow=false |
| u8/u16/u32/u64 | bool | IntToBool | precision=false, overflow=false |
| f32 | bool | FloatToBool | precision=false, overflow=false |
| f64 | bool | FloatToBool | precision=false, overflow=false |

**Boolean ↔ String** (2 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| bool | String | BoolToString | runtime_support=true |
| String | bool | StringToBool | runtime_support=true, validation=true |

**Boolean ↔ char** (2 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| bool | char | Indirect (bool→u32→char) | - |
| char | bool | Indirect (char→u32→bool) | - |

**Total**: 10 + 10 + 2 + 2 = 24 rules

#### Category 8: Character Conversions (14 rules)

**char ↔ u32** (2 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| char | u32 | CharToInt | precision=false, overflow=false |
| u32 | char | IntToChar | validation=true |

**char ↔ Other Integers** (12 rules via Indirect):
| From | To | Route | CastKind Chain |
|------|-----|-------|----------------|
| char | i8/i16/i32/i64/u8/u16/u64 | via u32 | CharToInt → IntTruncate/IntZeroExtend/IntSignExtend |
| i8/i16/i32/i64/u8/u16/u64 | char | via u32 | IntTruncate/IntZeroExtend → IntToChar |

**char ↔ String** (2 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| char | String | CharToString | runtime_support=true |
| String | char | StringToChar | runtime_support=true, validation=true |

**Total**: 2 + 12 + 2 = 16 rules (14 new, 2 already counted in identity)

#### Category 9: String Conversions (28 rules)

**Primitive → String** (12 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| i8/i16/i32/i64 | String | IntToString | runtime_support=true |
| u8/u16/u32/u64 | String | IntToString | runtime_support=true |
| f32 | String | FloatToString | runtime_support=true |
| f64 | String | FloatToString | runtime_support=true |

**String → Primitive** (12 rules):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| String | i8/i16/i32/i64 | StringToInt | runtime_support=true, validation=true |
| String | u8/u16/u32/u64 | StringToInt | runtime_support=true, validation=true |
| String | f32 | StringToFloat | runtime_support=true, validation=true |
| String | f64 | StringToFloat | runtime_support=true, validation=true |

**String → String** (1 rule):
| From | To | CastKind | Flags |
|------|-----|----------|-------|
| String | String | Bitcast | precision=false, overflow=false |

**Total**: 12 + 12 + 1 = 25 rules (3 already counted: bool↔String, char↔String)

---

## Validation Summary

### Compile-Time Validation
- Type pair existence in promotion matrix (all 169 pairs)
- CastKind appropriateness for type pair
- Unicode scalar value range for u32→char (0..=0xD7FF, 0xE000..=0x10FFFF)
- Const-evaluable string conversions (e.g., "42" → 42)

### Runtime Validation Required
- String → Primitive parsing (unparseable strings)
- String → char length check (must be 1 character)
- u32 → char range check (non-const values)
- Float → Integer overflow checks (based on OverflowBehavior)
- Integer narrowing overflow checks (based on OverflowBehavior)

---

## Implementation Checklist

- [ ] Add `requires_runtime_support` field to `PromotionRule::Direct`
- [ ] Add `requires_validation` field to `PromotionRule::Direct`
- [ ] Implement `add_integer_narrowing_promotions()` method
- [ ] Implement `add_boolean_promotions()` method
- [ ] Implement `add_character_promotions()` method
- [ ] Implement `add_string_promotions()` method
- [ ] Add `InvalidStringConversion` and `InvalidUnicodeCodePoint` warning variants
- [ ] Update `PromotionMatrix::initialize_default_promotions()` to call new methods
- [ ] Verify all 169 type pairs are defined (write validation test)
- [ ] Verify all 24 CastKind variants are used at least once (write verification test)

---

**Data Model Status**: ✅ COMPLETE - Ready for contract generation (Phase 1)
