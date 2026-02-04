# Data Model: IR to x86-64 Assembly Translator

## Overview

This document describes the data model for the IR to x86-64 Assembly Translator module in the jsavrs compiler. The translator converts IR structures from `src/ir/` into NASM-compatible x86-64 assembly using existing `src/asm/` infrastructure.

## Core Entities

### Translator

**Description**: Main entry point for the translation process
**Fields**:

- `config: TranslationConfig` - Configuration options for the translation process
- `abi: Abi` - Selected ABI for target platform
**Relationships**:

- Uses `TranslationContext` during translation
- Consumes `ir::Module` as input
- Produces `String` (assembly code) as output
**Validation rules**:

- Must have valid ABI configuration
- Must handle all InstructionKind variants defined in this specification
- Must handle all BinaryOp variants (Add, Sub, Mul, Div, And, Or, Xor, Shl, Shr, Eq, Ne, Lt, Le, Gt, Ge)
- Must handle all Terminator variants (Return, Jump, ConditionalJump, Unreachable)
- Must return TranslationError with code E4001 for unsupported or malformed IR constructs
- Must validate that all referenced BasicBlockIds exist within the function being translated
**State transitions**: N/A (stateless function)

### TranslationConfig

**Description**: Configuration options for the translation process
**Fields**:

- `target_abi: AbiType` - Target ABI (System V AMD64 or Windows x64)
- `emit_mapping: bool` - Whether to generate source mapping
- `debug_symbols: bool` - Whether to generate debug symbols
**Relationships**: Used by `Translator` and `TranslationContext`
**Validation rules**:

- `target_abi` must be a valid ABI type
- Options must be consistent with target platform capabilities
**State transitions**: N/A (immutable configuration)

### TranslationContext

**Description**: Maintains state during the translation process
**Fields**:

- `abi: Abi` - ABI configuration for target platform
- `symbol_table: HashMap<String, SymbolInfo>` - Maps IR symbols to assembly symbols
- `register_allocator: TempRegisterAllocator` - Manages temporary register allocation
- `current_function: Option<FunctionSignature>` - Current function being translated
- `label_counter: u32` - Counter for generating unique labels
**Relationships**:
- Used by all translation sub-components
- Interacts with `Abi` for platform-specific details
**Validation rules**:
- Must maintain consistent symbol mappings
- Must ensure unique label generation
**State transitions**:
- Initialized at start of module translation
- Updated during function translation
- Between functions:
    - `current_function` is set to the new function signature
    - `label_counter` is reset to 0 to ensure unique labels per function
    - `register_allocator` is reset to clear temporary allocations
    - `symbol_table` persists and accumulates across all functions in the module
    - `abi` remains unchanged throughout module translation

### SymbolInfo

**Description**: Information about a symbol in the translation context
**Fields**:

- `name: String` - Original symbol name
- `asm_name: String` - Mapped assembly name
- `kind: SymbolKind` - Type of symbol (function, variable, constant)
- `address: Option<u64>` - Absolute memory address if known (None for unresolved/stack-relative symbols)
**Relationships**: Stored in `TranslationContext.symbol_table`
**Validation rules**:
- `asm_name` must be valid assembly identifier
- If `address` is `Some`:
    - Value must be non-zero (0x0 reserved for null)
    - For function symbols, must be aligned to instruction boundary (typically 16-byte for x86-64)
    - For data symbols, must be aligned according to type requirements
    - Must fall within valid address space for target architecture (< 2^48 for x86-64 canonical addresses)
- `address` is `None` for:
    - Symbols with stack-relative addresses
    - External/imported symbols not yet resolved
    - Symbols during initial registration before address assignment
**State transitions**: Created when symbol is first encountered

### TempRegisterAllocator

**Description**: Allocates temporary register names for the translation process
**Fields**:

- `next_temp: u32` - Next temporary register number to allocate
- `allocated_temps: Vec<String>` - Track allocated temporaries for conflict detection with reserved registers

**Notes**:

- Counter may skip values if temps are deallocated, ensuring unique naming across function lifetime
- Vec maintains active temps for validation that new allocations don't conflict with reserved registers
**Relationships**: Used by `TranslationContext`
**Validation rules**:
- Must generate unique temporary names
- Must not conflict with reserved registers
**State transitions**:
- Initialized at start of translation
- Updated as temporaries are allocated

## Translation Process Entities

### IrFunction

**Description**: IR representation of a function to be translated (from src/ir/)
**Fields**:

- `name: String` - Function name
- `parameters: Vec<IrParameter>` - Function parameters
- `return_type: IrType` - Return type
- `basic_blocks: Vec<BasicBlock>` - Function body as basic blocks
**Relationships**: Input to `FunctionTranslator`
**Validation rules**:
- Must have valid structure according to IR specification
- Basic blocks must form valid control flow graph
**State transitions**: N/A (immutable input)

### BasicBlock

**Description**: Basic block in the IR (from src/ir/)
**Fields**:

- `id: BasicBlockId` - Unique identifier
- `instructions: Vec<Instruction>` - Instructions in the block
- `terminator: Terminator` - Control flow terminator
**Relationships**: Part of `IrFunction`, processed by `BlockTranslator`
**Validation rules**:
- Instructions must be valid according to IR specification
- Terminator must be properly formed
**State transitions**: N/A (immutable input)

### Instruction

**Description**: IR instruction (from src/ir/)
**Fields**:

- `id: InstructionId` - Unique identifier
- `kind: InstructionKind` - Type of instruction
- `operands: Vec<Operand>` - Operands for the instruction
**Relationships**: Part of `BasicBlock`, processed by `InstructionTranslator`
**Validation rules**:

- Must have valid operands for the instruction kind
- Operands must refer to valid values
**State transitions**: N/A (immutable input)

### InstructionKind

**Description**: Enumeration of different types of IR instructions (from src/ir/)
**Variants**:

- `BinaryOp(BinaryOp)` - Binary operation
- `Load` - Memory load
- `Store` - Memory store
- `Call(CallInfo)` - Function call
- `Return(Option<Value>)` - Return statement
- `Allocate` - Memory allocation
- `Constant(Constant)` - Constant value
**Relationships**: Used in `Instruction`
**Validation rules**: Each variant must have appropriate associated data
**State transitions**: N/A (immutable enum)

### BinaryOp

**Description**: Binary operations in IR (from src/ir/)
**Variants**:

- `Add` - Addition
- `Sub` - Subtraction
- `Mul` - Multiplication
- `Div` - Division
- `And` - Bitwise AND
- `Or` - Bitwise OR
- `Xor` - Bitwise XOR
- `Shl` - Shift left
- `Shr` - Shift right
- `Eq` - Equality comparison
- `Ne` - Not equal comparison
- `Lt` - Less than comparison
- `Le` - Less than or equal comparison
- `Gt` - Greater than comparison
- `Ge` - Greater than or equal comparison
**Relationships**: Used in `InstructionKind::BinaryOp`
**Validation rules**: Operands must be compatible with operation
**State transitions**: N/A (immutable enum)

### Terminator

**Description**: Control flow terminators in IR (from src/ir/)
**Variants**:

- `Return(Option<Value>)` - Return from function
- `Jump(BasicBlockId)` - Unconditional jump
- `ConditionalJump { condition: Value, then_block: BasicBlockId, else_block: BasicBlockId }` - Conditional jump
- `Unreachable` - Unreachable code marker
**Relationships**: Part of `BasicBlock`, processed by `TerminatorTranslator`
**Validation rules**: Target blocks must exist in the function
**State transitions**: N/A (immutable enum)

## Assembly Generation Entities

### AssemblyInstruction

**Description**: Assembly instruction representation
**Fields**:

- `mnemonic: String` - Instruction mnemonic (e.g., "mov", "add")
- `operands: Vec<AssemblyOperand>` - Instruction operands
- `comment: Option<String>` - Optional comment for debugging
**Relationships**: Output from translation process, used by assembly emitter
**Validation rules**:
- Must be valid x86-64 instruction
- Operands must be compatible with mnemonic
**State transitions**: Created during translation, finalized during emission

### AssemblyOperand

**Description**: Operand for assembly instructions
**Variants**:

- `Register(String)` - CPU register
- `MemoryAddress(MemoryLocation)` - Memory address
- `Immediate(i64)` - Immediate value
- `Label(String)` - Code label
- `Symbol(String)` - Symbol reference
**Relationships**: Part of `AssemblyInstruction`
**Validation rules**: Must be valid operand type for the target architecture
**State transitions**: N/A (immutable enum)

### MemoryLocation

**Description**: Representation of a memory location
**Fields**:

- `base_register: Option<String>` - Base register (optional)
- `index_register: Option<String>` - Index register (optional)
- `scale: u8` - Scale factor (1, 2, 4, or 8)
- `displacement: i32` - Displacement offset
**Relationships**: Used in `AssemblyOperand::MemoryAddress`
**Validation rules**:
- Scale must be 1, 2, 4, or 8
- Register names must be valid x86-64 registers
**State transitions**: N/A (immutable struct)

### AssemblyFile

**Description**: Complete assembly file structure
**Fields**:

- `header: Vec<String>` - File header directives
- `text_section: Vec<AssemblyInstruction>` - Code section
- `data_section: Vec<DataDirective>` - Data section (if needed)
- `bss_section: Vec<BssDirective>` - Uninitialized data section (if needed)
**Relationships**: Final output of translation process
**Validation rules**:
- Must follow NASM syntax requirements
- Sections must be properly ordered
**State transitions**: Built incrementally during translation

### DataDirective

**Description**: Directive for data section
**Fields**:

- `label: String` - Data label
- `directive: String` - Data directive (e.g., "dd", "dq")
- `value: DataValue` - Data value (typed)
**Relationships**: Part of `AssemblyFile.data_section`
**Validation rules**:
- Must be valid NASM data directive (dd, dq, dw, db, etc.)
- value type must be compatible with directive size
**State transitions**: N/A (immutable struct)

### BssDirective

**Description**: Directive for bss section
**Fields**:

- `label: String` - Data label
- `directive: String` - Space allocation directive (e.g., "resd", "resq")
- `size: u32` - Size to reserve
**Relationships**: Part of `AssemblyFile.bss_section`
**Validation rules**: Must be valid NASM bss directive
**State transitions**:
- Initialized with empty sections at start of module translation
- Text section populated as functions are translated (instructions added sequentially)
- Data section populated when global constants/strings are encountered
- BSS section populated for uninitialized global variables
- Once translation completes, AssemblyFile is finalized (immutable)
- Finalized AssemblyFile is rendered to NASM syntax string for output

## ABI-Specific Entities

### AbiType

**Description**: Enumeration of supported ABIs
**Variants**:

- `SystemV` - System V AMD64 ABI (Linux/macOS)
- `Windows64` - Windows x64 ABI
**Relationships**: Used in `TranslationConfig.target_abi`
**Validation rules**: Must be a supported ABI type
**State transitions**: N/A (immutable enum)

### Abi

**Description**: ABI implementation with platform-specific details (from src/asm/abi.rs)
**Methods**:

- `int_param_registers() -> Vec<String>` - Integer parameter registers
- `float_param_registers() -> Vec<String>` - Float parameter registers
- `callee_saved_gp_registers() -> Vec<String>` - Callee-saved general purpose registers
- `shadow_space() -> u32` - Windows shadow space size
- `red_zone() -> u32` - System V red zone size
**Relationships**: Used by `TranslationContext` and ABI adapter
**Validation rules**: Methods must return valid register names for the target ABI
**State transitions**: N/A (immutable struct with methods)

## Error Handling Entities

### TranslationError

**Description**: Error type for translation failures
**Fields**:

- `code: ErrorCode` - Error code (e.g., E4001)
- `message: String` - Human-readable error message
- `ir_location: Option<IrLocation>` - Location in IR where error occurred
**Relationships**: Returned by translation functions
**Validation rules**:
- Must have a valid error code
- Message must be informative for debugging
**State transitions**: Created when translation error occurs

### ErrorCode

**Description**: Error codes for translation (from existing error infrastructure)
**Variants**:

- `E4001` - Translation-specific error code
- Other existing codes as appropriate
**Relationships**: Used in `TranslationError`
**Validation rules**: Must be a valid error code
**State transitions**: N/A (immutable enum)

### IrLocation

**Description**: Location information in the IR
**Fields**:

- `file: String` - Source file name
- `line: u32` - Line number
- `column: u32` - Column number
- `function: String` - Function name
**Relationships**: Used in `TranslationError.ir_location`
**Validation rules**: Must represent a valid location in the IR
**State transitions**: N/A (immutable struct)

## Source Mapping Entities

### SourceMapping

**Description**: Mapping between IR locations and assembly output
**Fields**:

- `ir_location: IrLocation` - Location in IR
- `asm_line: u32` - Line number in assembly output
- `asm_label: Option<String>` - Associated assembly label
**Relationships**: Generated when `emit_mapping` is enabled
**Validation rules**:
- Must have valid line numbers
- Label must be valid if present
**State transitions**: Created during translation when mapping is enabled

### MappingFile

**Description**: File containing source mappings
**Fields**:

- `mappings: Vec<SourceMapping>` - Collection of mappings
- `format_version: u32` - Version of mapping format
**Relationships**: Output when `emit_mapping` is enabled
**Validation rules**: Must follow specified format "IR_LINE:COL → ASM_LINE:LABEL"
**State transitions**: Built incrementally during translation when enable
