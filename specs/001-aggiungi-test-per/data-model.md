# Data Model: Assembly Module Test Coverage

## Core Entities from Assembly Module

### Register
**Description**: Represents x86-64 registers of different sizes (8-bit, 16-bit, 32-bit, 64-bit), used to store operands, addresses, and intermediate results during instruction execution.

**Fields**:
- `name`: String - The register identifier (e.g., "rax", "eax", "ax", "ah")
- `size`: RegisterSize enum - The register size (8, 16, 32, or 64 bits)
- `encoding`: u8 - The register's hardware encoding

**Relationships**:
- Part of the Operand type (as Register variant)
- Used in Instruction operands

**Validation Rules**:
- Size must be one of the valid register sizes (8, 16, 32, 64 bits)
- Name must match valid x86-64 register naming conventions
- Encoding must be within valid x86-64 register encoding range

**Test Scenarios**:
- Valid register creation for each size (8, 16, 32, 64-bit)
- Register size conversion methods (to_64bit, to_32bit, etc.)
- Display formatting for each register type
- ABI-specific register classification (parameter, caller-saved, callee-saved)

### Operand
**Description**: Represents assembly operands including registers, immediates, labels, and memory references.

**Variants**:
- `Register(Register)` - A register operand
- `Immediate(i64)` - A 64-bit immediate value
- `Label(String)` - A code label
- `MemoryRef(MemoryReference)` - A memory reference operand

**Validation Rules**:
- Immediate values must be within valid range for the architecture
- Label names must follow assembly naming conventions
- Memory references must have valid addressing modes

**Test Scenarios**:
- All operand type variants (Register, Immediate, Label, MemoryRef)
- Display formatting for each operand type
- Utility methods (is_register, as_immediate, etc.)
- Boundary values for immediate operands (i64::MIN, i64::MAX, 0, ±1)
- Memory reference operand creation with various combinations

### MemoryReference
**Description**: Represents memory operands with complex addressing modes.

**Fields**:
- `base`: Option<Register> - The base register (optional)
- `index`: Option<Register> - The index register (optional)
- `scale`: u8 - The scale factor (1, 2, 4, 8)
- `displacement`: i32 - The displacement value
- `label`: Option<String> - Optional label reference

**Validation Rules**:
- Scale must be one of 1, 2, 4, or 8
- Base and index registers must be valid x86-64 registers
- Displacement within valid range for memory addressing
- Complex addressing combinations must be valid per x86-64 spec

**Test Scenarios**:
- All combinations of base, index, scale, and displacement
- Null or zero registers handling
- Maximum and minimum displacement values
- RIP-relative addressing modes
- Complex addressing with all components present

### Instruction
**Description**: Represents assembly instructions with operands.

**Fields**:
- `mnemonic`: String - The instruction mnemonic (e.g., "mov", "add", "call")
- `operands`: Vec<Operand> - The instruction operands
- `prefix`: Option<String> - Optional instruction prefix

**Validation Rules**:
- Operand count must match the instruction requirements (e.g., div/idiv require single operand)
- Operand types must be valid for the specific instruction
- Instruction must be valid x86-64 operation

**Test Scenarios**:
- All instruction types and operand combinations
- Instruction-specific operand constraints (e.g., div, idiv)
- Display formatting with complex operand combinations
- Utility methods (as_instruction, is_jump, etc.)

### AssemblyElement
**Description**: Represents components of assembly code such as sections, labels, instructions, directives, and comments.

**Variants**:
- `Section(String, Vec<AssemblyElement>)` - Assembly section with nested elements
- `Label(String)` - Assembly label
- `Instruction(Instruction)` - Assembly instruction
- `Directive(String, Option<String>)` - Assembly directive with optional parameter
- `Comment(String)` - Assembly comment

**Validation Rules**:
- Section names must be valid assembly section names
- Nested elements must be valid within the section context
- Comments must be properly formatted

**Test Scenarios**:
- All AssemblyElement type variants
- Assembly element manipulation methods (add_element, add_elements)
- Section handling to prevent duplicates
- Proper ordering of sections

### NasmGenerator
**Description**: The core assembly code generator responsible for translating intermediate representations into NASM-formatted x86-64 assembly.

**Fields**:
- `elements`: Vec<AssemblyElement> - The assembly elements to generate
- `section_map`: HashMap<String, AssemblyElement> - Tracking unique sections
- `label_counter`: u32 - Counter for generating unique labels

**Validation Rules**:
- Generated assembly must follow NASM syntax
- Sections must not be duplicated
- Generated code must be valid x86-64 assembly
- Unique labels must be generated correctly

**Test Scenarios**:
- Section handling to prevent duplicate sections
- Label generation with unique names
- Hello world program generation for all target operating systems
- Function prologue and epilogue generation
- Factorial function generation with recursive calls
- Generated assembly formatting and correctness

### TargetOS
**Description**: Represents the target operating system, influencing calling conventions, register usage, ABI compliance.

**Variants**:
- `Linux`
- `Windows`
- `MacOS`

**Validation Rules**:
- Each OS must have defined parameter registers
- Each OS must have defined callee-saved registers
- ABI-specific register classification must be correct

**Test Scenarios**:
- All TargetOS methods (param_register, callee_saved_registers)
- OS-specific parameter register retrieval
- Function prologue/epilogue generation per OS
- Cross-platform consistency validation