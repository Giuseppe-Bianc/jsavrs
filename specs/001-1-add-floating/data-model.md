# Data Model: IEEE 754 Floating-Point Support

## Entities

### FloatingPointRegister
- **Name**: FloatingPointRegister
- **Description**: Represents x86-64 SIMD registers used for floating-point operations
- **Fields**:
  - variant: XMM(0-15) | YMM(0-15) | ZMM(0-15)
  - width: 128 | 256 | 512 (bits)
  - type: SinglePrecision | DoublePrecision | Mixed
- **Relationships**: Used by FloatingPointInstruction as operands
- **Validation**: Register index must be within valid range (0-15 for XMM/YMM/ZMM)
- **State transitions**: N/A

### FloatingPointInstruction
- **Name**: FloatingPointInstruction
- **Description**: IEEE 754 compliant floating-point operations
- **Fields**:
  - opcode: AddSS | AddSD | SubSS | SubSD | MulSS | MulSD | DivSS | DivSD | SqrtSS | SqrtSD | CmpSS | CmpSD | etc.
  - operands: Vec<FloatingPointOperand>
  - precision: Single | Double | Extended
- **Relationships**: Operates on FloatingPointRegister and FloatingPointOperand
- **Validation**: Correct number and types of operands per opcode
- **State transitions**: N/A

### FloatingPointOperand
- **Name**: FloatingPointOperand
- **Description**: Operands for floating-point instructions
- **Fields**:
  - variant: Register(FloatingPointRegister) | Immediate(F64) | Memory(Address)
  - alignment: u8 (required for floating-point operations)
- **Relationships**: Used by FloatingPointInstruction
- **Validation**: Proper alignment for memory operands, valid immediate values
- **State transitions**: N/A

### IEEE754ExceptionType
- **Name**: IEEE754ExceptionType
- **Description**: The five standard IEEE 754 exception types
- **Fields**:
  - variant: InvalidOperation | DivisionByZero | Overflow | Underflow | Inexact
- **Relationships**: Used in exception handling system
- **Validation**: N/A
- **State transitions**: N/A

### RoundingMode
- **Name**: RoundingMode
- **Description**: The four standard IEEE 754 rounding modes
- **Fields**:
  - variant: ToNearest | TowardPositiveInfinity | TowardNegativeInfinity | TowardZero
- **Relationships**: Used by floating-point arithmetic operations
- **Validation**: N/A
- **State transitions**: N/A

### MXCSRRegister
- **Name**: MXCSRRegister
- **Description**: x86-64 MXCSR (MXCSR Register) for floating-point control and status
- **Fields**:
  - exception_masks: [bool; 5] (mask for each IEEE754ExceptionType)
  - exception_flags: [bool; 5] (status for each IEEE754ExceptionType)
  - rounding_mode: RoundingMode
  - ftz_daz_modes: FlushToZero | DenormalsAreZero | Standard
- **Relationships**: Controls behavior of all floating-point operations
- **Validation**: Proper bit layout matching x86-64 specification
- **State transitions**: N/A

### ABIConvention
- **Name**: ABIConvention
- **Description**: Application Binary Interface for floating-point parameters
- **Fields**:
  - variant: WindowsX64 | SystemV
  - float_register_usage: Vec<FloatingPointRegister>
  - parameter_passing_rules: HashMap<u32, FloatingPointRegister>
- **Relationships**: Used during code generation phase
- **Validation**: Follows specific ABI specification
- **State transitions**: N/A

## Relationships Summary
- FloatingPointInstruction uses FloatingPointRegister and FloatingPointOperand
- FloatingPointOperand can contain FloatingPointRegister
- FloatingPointInstruction may trigger IEEE754ExceptionType
- FloatingPointInstruction follows RoundingMode
- All floating-point operations are controlled by MXCSRRegister
- Code generation follows ABIConvention

## Validation Rules from Requirements
- FR-002: Floating-point registers (XMM0-XMM15, YMM0-YMM15, ZMM0-ZMM15) must be in Register enum
- FR-003: IEEE 754 compliant floating-point instructions must be in Instruction enum
- FR-004: Operand handling must support floating-point values
- FR-006: Support both single-precision (32-bit) and double-precision (64-bit) formats
- FR-007: Exception handling with configurable behaviors at compile-time
- FR-012: Proper calling conventions for target platform (Windows x64 ABI/Unix System V ABI)
- FR-015: Correct handling of subnormal numbers with optional FTZ/DAZ modes
- FR-016: All IEEE 754 comparison predicates with proper NaN handling
- FR-017: Access to floating-point control registers, specifically MXCSR
- FR-018: Proper MXCSR register management according to ABI (callee-saved control bits)
- FR-020: Handle signed zero (+0 and -0) correctly in operations and comparisons