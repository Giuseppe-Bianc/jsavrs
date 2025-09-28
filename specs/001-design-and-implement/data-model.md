# Data Model: x86-64 Assembly Code Generator

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 2025-09-28  
**Status**: Phase 1 Design Complete  
**Dependencies**: research.md

## Overview

This document defines the comprehensive data model for the x86-64 assembly code generator, including all entities, enums, relationships, and validation rules. The design emphasizes type safety through Rust enums, clear separation of concerns, and extensibility for future enhancements.

## Core Entity Architecture

### 1. Assembly Generator Context

**Entity**: `AssemblyGenerator`  
**Purpose**: Main orchestrator for assembly code generation from IR  
**Lifecycle**: Created per compilation unit, destroyed after assembly generation  

```rust
pub struct AssemblyGenerator {
    /// Current target platform configuration
    target_platform: TargetPlatform,
    /// Active calling convention implementation
    calling_convention: Box<dyn CallingConvention>,
    /// Register allocation state
    register_allocator: RegisterAllocator,
    /// Generated assembly output buffer
    output_buffer: AssemblyBuffer,
    /// Symbol table for function and variable names
    symbol_table: SymbolTable,
    /// Current function generation context
    current_function: Option<FunctionContext>,
    /// Error accumulator for generation issues
    errors: Vec<CodeGenError>,
}
```

**Validation Rules**:
- Must have valid target platform before generation
- Calling convention must match target platform
- Symbol table must be initialized before function generation
- Output buffer must be writable and have sufficient capacity

**State Transitions**:
1. `Initialized` → `GeneratingFunction` (when function generation starts)
2. `GeneratingFunction` → `FunctionComplete` (when function generation finishes)
3. `FunctionComplete` → `GeneratingFunction` (for next function) OR `Complete` (when all functions done)

### 2. Target Platform Configuration

**Entity**: `TargetPlatform`  
**Purpose**: Encapsulates platform-specific code generation requirements  
**Immutability**: Configuration set at creation time  

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetPlatform {
    /// Operating system (Windows, Linux, macOS)
    pub os: TargetOS,
    /// Processor architecture (x86_64)
    pub arch: TargetArch,
    /// ABI specification for function calls
    pub abi: ABISpec,
    /// Symbol naming convention
    pub symbol_convention: SymbolConvention,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetOS {
    Windows,
    Linux, 
    MacOS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetArch {
    X86_64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolConvention {
    /// No prefix (Unix/Linux style)
    None,
    /// Underscore prefix (Windows style)
    Underscore,
}
```

**Validation Rules**:
- OS must be one of supported values (Windows, Linux, macOS)
- Architecture must be X86_64 for this implementation
- ABI must be compatible with OS (Windows→WindowsX64ABI, Unix→SystemVABI)
- Symbol convention must match OS defaults

### 3. Register Management System

**Entity**: `RegisterAllocator`  
**Purpose**: Manages x86-64 register allocation and tracking  
**State Management**: Tracks register usage within function scope  

```rust
pub struct RegisterAllocator {
    /// Currently allocated general-purpose registers
    allocated_gp: BTreeSet<GPRegister>,
    /// Currently allocated XMM registers
    allocated_xmm: BTreeSet<XMMRegister>,
    /// Stack offset for spilled variables
    stack_offset: i32,
    /// Calling convention register constraints
    abi_constraints: ABIConstraints,
}

/// General-purpose x86-64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPRegister {
    // Primary registers
    RAX, RBX, RCX, RDX,
    // Index and pointer registers
    RSI, RDI, RSP, RBP,
    // Extended registers
    R8, R9, R10, R11, R12, R13, R14, R15,
}

/// XMM (SSE) registers for floating-point operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum XMMRegister {
    XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7,
    XMM8, XMM9, XMM10, XMM11, XMM12, XMM13, XMM14, XMM15,
}

/// Unified register representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    GeneralPurpose(GPRegister),
    XMM(XMMRegister),
}
```

**Validation Rules**:
- RSP and RBP cannot be allocated for general computation
- Register allocation must respect calling convention constraints
- Stack offset must maintain 16-byte alignment
- Spilled variables must have valid stack locations

**State Transitions**:
1. `Available` → `Allocated` (when register assigned to variable)
2. `Allocated` → `Available` (when variable goes out of scope)
3. `Allocated` → `Spilled` (when register needed for other use)
4. `Spilled` → `Allocated` (when register becomes available again)

### 4. Instruction Representation

**Entity**: `X86Instruction`  
**Purpose**: Type-safe representation of x86-64 assembly instructions  
**Design**: Enum-based with associated operand types  

```rust
/// Complete x86-64 instruction set representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X86Instruction {
    // Arithmetic operations
    Add { dest: Operand, src: Operand },
    Sub { dest: Operand, src: Operand },
    Mul { operand: Operand },
    Div { operand: Operand },
    
    // Logical operations
    And { dest: Operand, src: Operand },
    Or { dest: Operand, src: Operand },
    Xor { dest: Operand, src: Operand },
    Not { operand: Operand },
    
    // Data movement
    Mov { dest: Operand, src: Operand },
    Push { operand: Operand },
    Pop { operand: Operand },
    Lea { dest: Operand, src: MemoryOperand },
    
    // Control flow
    Jmp { target: JumpTarget },
    Je { target: JumpTarget },
    Jne { target: JumpTarget },
    Jl { target: JumpTarget },
    Jle { target: JumpTarget },
    Jg { target: JumpTarget },
    Jge { target: JumpTarget },
    
    // Function operations
    Call { target: CallTarget },
    Ret,
    
    // Comparison
    Cmp { left: Operand, right: Operand },
    Test { left: Operand, right: Operand },
    
    // Floating-point operations (SSE)
    Addss { dest: XMMRegister, src: Operand },
    Subss { dest: XMMRegister, src: Operand },
    Mulss { dest: XMMRegister, src: Operand },
    Divss { dest: XMMRegister, src: Operand },
    
    // Memory barriers and special instructions
    Nop,
}
```

**Validation Rules**:
- Operand types must be compatible with instruction requirements
- Memory operands must have valid addressing modes
- Register operands must be appropriate size for instruction
- Immediate values must be within instruction constraints

### 5. Operand System

**Entity**: `Operand`  
**Purpose**: Unified representation of instruction operands  
**Type Safety**: Prevents invalid operand combinations at compile time  

```rust
/// Unified operand representation for x86-64 instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// Register operand
    Register(Register),
    /// Immediate value operand
    Immediate(ImmediateValue),
    /// Memory location operand
    Memory(MemoryOperand),
}

/// Immediate value with size information
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImmediateValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(u32),  // IEEE 754 binary representation
    Float64(u64),  // IEEE 754 binary representation
}

/// Memory operand with flexible addressing modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    /// Base register (optional)
    pub base: Option<GPRegister>,
    /// Index register (optional)
    pub index: Option<GPRegister>,
    /// Scale factor (1, 2, 4, or 8)
    pub scale: Scale,
    /// Displacement offset
    pub displacement: i32,
    /// Memory operand size
    pub size: OperandSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandSize {
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8,
}
```

**Validation Rules**:
- Memory operands must have at least base or displacement
- Scale factor must be 1, 2, 4, or 8
- Index register cannot be RSP
- Displacement must fit in 32-bit signed integer
- Operand size must match instruction requirements

### 6. Function Generation Context

**Entity**: `FunctionContext`  
**Purpose**: Maintains state during individual function generation  
**Scope**: Created per function, contains local generation state  

```rust
pub struct FunctionContext {
    /// Function name in generated assembly
    pub name: String,
    /// Function signature from IR
    pub signature: FunctionSignature,
    /// Local variable allocation map
    pub locals: HashMap<LocalId, LocalVariable>,
    /// Current basic block being generated
    pub current_block: Option<BasicBlockId>,
    /// Label mapping for jump targets
    pub label_map: HashMap<BasicBlockId, String>,
    /// Stack frame size in bytes
    pub stack_frame_size: u32,
    /// Maximum number of function parameters
    pub max_params: usize,
}

/// Function signature derived from IR
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Parameter types and locations
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: ValueType,
    /// Calling convention
    pub calling_convention: CallingConventionType,
}

/// Parameter information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    /// Parameter name (for debugging)
    pub name: String,
    /// Parameter type
    pub param_type: ValueType,
    /// Parameter location (register or stack)
    pub location: ParameterLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterLocation {
    Register(Register),
    Stack { offset: i32, size: OperandSize },
}
```

**Validation Rules**:
- Function name must be valid assembly identifier
- Parameter count must not exceed calling convention limits
- Stack frame size must maintain 16-byte alignment
- All basic blocks must have unique labels

### 7. Calling Convention Interface

**Entity**: `CallingConvention` trait  
**Purpose**: Abstracts platform-specific ABI requirements  
**Extensibility**: Allows future calling convention additions  

```rust
/// Trait defining calling convention behavior
pub trait CallingConvention: Send + Sync {
    /// Registers used for integer/pointer parameters
    fn integer_param_registers(&self) -> &[GPRegister];
    
    /// Registers used for floating-point parameters
    fn float_param_registers(&self) -> &[XMMRegister];
    
    /// Register for return values
    fn return_registers(&self) -> (Option<GPRegister>, Option<XMMRegister>);
    
    /// Caller-saved (volatile) registers
    fn caller_saved_registers(&self) -> &[Register];
    
    /// Callee-saved (non-volatile) registers
    fn callee_saved_registers(&self) -> &[Register];
    
    /// Required stack alignment in bytes
    fn stack_alignment(&self) -> u32;
    
    /// Shadow space size (Windows-specific)
    fn shadow_space_size(&self) -> u32;
    
    /// Generate function prologue
    fn generate_prologue(&self, ctx: &FunctionContext) -> Vec<X86Instruction>;
    
    /// Generate function epilogue
    fn generate_epilogue(&self, ctx: &FunctionContext) -> Vec<X86Instruction>;
}

/// Windows x64 ABI implementation
#[derive(Debug, Clone)]
pub struct WindowsX64ABI;

/// System V ABI implementation (Linux/macOS)
#[derive(Debug, Clone)]
pub struct SystemVABI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConventionType {
    WindowsX64,
    SystemV,
}
```

### 8. IR Integration Entities

**Entity**: `IRTranslator`  
**Purpose**: Translates jsavrs IR to x86-64 instructions  
**Integration**: Bridges existing IR with new assembly generation  

```rust
pub struct IRTranslator {
    /// Current generation context
    generator: AssemblyGenerator,
    /// IR module being processed
    current_module: Option<IRModuleRef>,
    /// Instruction translation cache
    translation_cache: HashMap<IRInstructionId, Vec<X86Instruction>>,
}

/// Reference to jsavrs IR instruction
pub struct IRInstructionRef {
    pub id: IRInstructionId,
    pub opcode: IROpcode,
    pub operands: Vec<IROperand>,
    pub result_type: Option<ValueType>,
}

/// Value types supported by the generator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    // Integer types
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    
    // Floating-point types
    Float32, Float64,
    
    // Pointer types
    Pointer { pointee: Box<ValueType> },
    
    // Aggregate types
    Struct { fields: Vec<ValueType> },
    Array { element: Box<ValueType>, length: usize },
    
    // Special types
    Void,
    Bool,
}
```

### 9. Error Handling Model

**Entity**: `CodeGenError`  
**Purpose**: Comprehensive error reporting for code generation issues  
**Hierarchy**: Structured error types with context information  

```rust
/// Comprehensive error type for assembly generation
#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Unsupported IR instruction: {instruction} at {location}")]
    UnsupportedInstruction {
        instruction: String,
        location: SourceLocation,
    },
    
    #[error("Register allocation failed: {reason}")]
    RegisterAllocationFailure {
        reason: String,
        function: String,
        instruction_id: Option<IRInstructionId>,
    },
    
    #[error("Invalid operand combination for instruction {instruction}: {details}")]
    InvalidOperandCombination {
        instruction: String,
        details: String,
    },
    
    #[error("ABI constraint violation: {constraint}")]
    ABIViolation {
        constraint: String,
        function: String,
    },
    
    #[error("Type mismatch: expected {expected}, found {actual}")]
    TypeMismatch {
        expected: ValueType,
        actual: ValueType,
        location: SourceLocation,
    },
    
    #[error("Symbol resolution failed: {symbol}")]
    SymbolResolutionFailure {
        symbol: String,
        location: SourceLocation,
    },
    
    #[error("Stack overflow: frame size {size} exceeds limit {limit}")]
    StackOverflow {
        size: u32,
        limit: u32,
        function: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}
```

## Entity Relationships

### Primary Relationships

1. **AssemblyGenerator** `1:1` **TargetPlatform**  
   - Generator configured for specific platform
   - Platform determines ABI and symbol conventions

2. **AssemblyGenerator** `1:1` **CallingConvention**  
   - Generator uses convention for function generation
   - Convention implementation varies by platform

3. **AssemblyGenerator** `1:1` **RegisterAllocator**  
   - Generator manages registers through allocator
   - Allocator maintains state across function generation

4. **FunctionContext** `1:N` **X86Instruction**  
   - Each function generates multiple instructions
   - Instructions reference function context for symbols

5. **X86Instruction** `1:N` **Operand**  
   - Most instructions have multiple operands
   - Operands can be shared between instructions

6. **Operand** `0:1` **Register**  
   - Register operands reference register entities
   - Registers can be used in multiple operands

### Secondary Relationships

7. **IRTranslator** `N:M` **X86Instruction**  
   - One IR instruction may produce multiple x86 instructions
   - Complex optimizations may merge IR instructions

8. **SymbolTable** `1:N` **FunctionContext**  
   - Symbol table contains all function symbols
   - Functions add symbols during generation

9. **RegisterAllocator** `N:M` **LocalVariable**  
   - Variables may be allocated to registers or stack
   - Register allocation changes during function generation

## Data Flow and State Management

### Code Generation Pipeline

```
IR Module → IRTranslator → AssemblyGenerator → NASM Output
    ↓           ↓              ↓                    ↓
  Functions  Instructions   x86 Assembly     Text File
```

### State Transitions During Generation

1. **Initialization Phase**:
   - Create AssemblyGenerator with target platform
   - Initialize register allocator and symbol table
   - Set up calling convention

2. **Function Processing Phase** (per function):
   - Create FunctionContext
   - Generate function prologue
   - Translate IR instructions to x86 instructions
   - Manage register allocation
   - Generate function epilogue

3. **Finalization Phase**:
   - Resolve symbol references
   - Generate section headers
   - Output final assembly text

### Memory Management Strategy

- **Stack Allocation**: Function-local contexts
- **Heap Allocation**: Symbol tables and instruction buffers
- **Reference Counting**: Shared IR references
- **RAII Pattern**: Automatic cleanup of generation context

## Validation and Constraints

### Cross-Entity Validation Rules

1. **Register Consistency**:
   - Allocated registers must match instruction requirements
   - Calling convention constraints must be respected
   - Register spilling must maintain data integrity

2. **Type System Coherence**:
   - IR value types must map to valid x86 operand sizes
   - Floating-point types require XMM registers
   - Pointer types must use 64-bit general-purpose registers

3. **ABI Compliance**:
   - Function signatures must conform to calling convention
   - Stack alignment must be maintained
   - Parameter passing must follow ABI rules

4. **Symbol Resolution**:
   - All function calls must have valid targets
   - Jump targets must reference valid labels
   - External symbols must be properly declared

### Performance Constraints

1. **Memory Usage**: Generator state ≤ 2x IR size
2. **Generation Time**: ≤ 5 seconds for 10,000 IR instructions
3. **Register Pressure**: Minimize stack spills through efficient allocation
4. **Code Size**: Generate compact assembly without sacrificing correctness

## Extension Points

### Future Enhancements

1. **Advanced Register Classes**:
   - YMM/ZMM registers for AVX operations
   - Mask registers for AVX-512
   - Special-purpose registers (segment, control)

2. **Optimization Framework**:
   - Peephole optimization passes
   - Register coalescing
   - Dead code elimination

3. **Debug Information**:
   - DWARF debug info generation
   - Source line mapping
   - Variable location tracking

4. **Additional Calling Conventions**:
   - Vectorcall convention
   - Custom embedded ABIs
   - Language-specific conventions

This data model provides a comprehensive foundation for type-safe, extensible x86-64 assembly generation while maintaining clear separation of concerns and supporting the full range of requirements specified in the feature specification.