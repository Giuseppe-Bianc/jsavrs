# Floating-Point API Contract

## Overview
This contract defines the expected interface and behavior for the IEEE 754 floating-point support in the jsavrs compiler system.

## Register Enum Contract

### `FloatingPointRegister` Enum
```
enum FloatingPointRegister {
    XMM0, XMM1, ..., XMM15,
    YMM0, YMM1, ..., YMM15,
    ZMM0, ZMM1, ..., ZMM15
}
```

**Requirements:**
- Must implement Display trait for proper formatting (e.g., "xmm0", "ymm1", etc.)
- Must implement conversion methods between register types where appropriate
- Index validation: registers must be in range 0-15 for each type

## Instruction Enum Contract

### `FloatingPointInstruction` Enum
```
enum FloatingPointInstruction {
    AddSS { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    AddSD { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    SubSS { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    SubSD { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    MulSS { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    MulSD { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    DivSS { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    DivSD { dst: FloatingPointRegister, src1: FloatingPointOperand, src2: FloatingPointOperand },
    SqrtSS { dst: FloatingPointRegister, src: FloatingPointOperand },
    SqrtSD { dst: FloatingPointRegister, src: FloatingPointOperand },
    // Additional instructions following the same pattern...
}
```

**Requirements:**
- All arithmetic operations must support proper IEEE 754 semantics
- Each instruction must validate the correct number and types of operands
- Display implementation must format instructions according to x86-64 assembly syntax

## Operand Contract

### `FloatingPointOperand` Enum
```
enum FloatingPointOperand {
    Register(FloatingPointRegister),
    Immediate(f64),
    Memory { base: Option<FloatingPointRegister>, index: Option<FloatingPointRegister>, displacement: i32, scale: u8 }
}
```

**Requirements:**
- Immediate values must support proper IEEE 754 representation
- Memory operands must enforce proper alignment for floating-point operations (typically 4-byte for f32, 8-byte for f64)
- All operands must be validated for correctness before instruction generation

## Exception Handling Contract

### `IEEE754ExceptionType` Enum
```
enum IEEE754ExceptionType {
    InvalidOperation,
    DivisionByZero,
    Overflow,
    Underflow,
    Inexact
}
```

**Requirements:**
- Implementation must provide configurable handling for each exception type
- Default behavior should follow IEEE 754 standard (generating special values rather than hardware exceptions)
- Compile-time option to switch between hardware exceptions and standard values

## Rounding Mode Contract

### `RoundingMode` Enum
```
enum RoundingMode {
    ToNearest,
    TowardPositiveInfinity,
    TowardNegativeInfinity,
    TowardZero
}
```

**Requirements:**
- All floating-point arithmetic operations must respect the current rounding mode
- MXCSR register must be properly managed to control rounding behavior
- Default rounding mode should be "ToNearest" as per IEEE 754 standard

## ABI Compliance Contract

### `ABIVariant` Enum
```
enum ABIVariant {
    WindowsX64,
    SystemV
}
```

**Requirements:**
- Floating-point parameter passing must follow the selected ABI specification
- Register preservation rules must be correctly implemented for each ABI
- Function return values must use appropriate floating-point registers according to ABI

## MXCSR Management Contract

### Interface Requirements
- Functions to read and write MXCSR register state
- Proper saving and restoring of MXCSR across function calls according to ABI
- Support for configuring exception masks and rounding modes

## Validation Requirements
- All generated floating-point assembly must be valid for the target architecture
- Generated code must pass external assembler validation
- Bit-exact compliance with IEEE 754-2008 for all arithmetic operations
- Special values (NaN, infinity, signed zero) must be handled correctly