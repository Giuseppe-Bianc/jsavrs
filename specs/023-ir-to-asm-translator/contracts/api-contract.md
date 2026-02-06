# API Contract: IR to x86-64 Assembly Translator

## Overview
This document defines the API contract for the IR to x86-64 Assembly Translator module. The translator converts IR structures from `src/ir/` into NASM-compatible x86-64 assembly using existing `src/asm/` infrastructure.

## Public API

### Module: `jsavrs::translator`

#### Struct: `Translator`
Main entry point for the translation process.

##### Method: `translate_module(module: &Module) -> Result<String, TranslationError>`
Converts an IR module to NASM-compatible x86-64 assembly text.

**Parameters:**
- `module: &Module` - Reference to the IR module to translate

**Returns:**
- `Ok(String)` - NASM-compatible assembly code as a string
- `Err(TranslationError)` - Error during translation

**Errors:**
- `TranslationError` - When the IR contains constructs that cannot be translated to assembly

**Example:**
```rust
use jsavrs::translator::Translator;
use jsavrs::ir::Module;

let ir_module = /* your IR module */;
let asm_code = Translator::translate_module(&ir_module)?;
println!("{}", asm_code);
```

#### Struct: `TranslationConfig`
Configuration options for the translation process.

**Fields:**
- `target_abi: AbiType` - Target ABI (System V AMD64 or Windows x64)
- `emit_mapping: bool` - Whether to generate source mapping
- `debug_symbols: bool` - Whether to generate debug symbols

#### Enum: `AbiType`
Supported ABI types for target platforms.

**Variants:**
- `SystemV` - System V AMD64 ABI (Linux/macOS)
- `Windows64` - Windows x64 ABI

#### Struct: `TranslationError`
Error type for translation failures.

**Fields:**
- `code: ErrorCode` - Error code (e.g., E4001)
- `message: String` - Human-readable error message
- `ir_location: Option<IrLocation>` - Location in IR where error occurred

## Internal API

### Module: `jsavrs::translator::context`
Manages state during the translation process.

#### Struct: `TranslationContext`
Maintains state during translation.

**Fields:**
- `abi: Abi` - ABI configuration for target platform
- `symbol_table: HashMap<String, SymbolInfo>` - Maps IR symbols to assembly symbols
- `register_allocator: TempRegisterAllocator` - Manages temporary register allocation
- `current_function: Option<FunctionSignature>` - Current function being translated
- `label_counter: u32` - Counter for generating unique labels

### Module: `jsavrs::translator::function_translator`
Handles function-level translation logic.

#### Struct: `FunctionTranslator`
Translates IR functions to assembly.

**Methods:**
- `translate_function(func: &IrFunction, context: &mut TranslationContext) -> Result<Vec<AssemblyInstruction>, TranslationError>`

### Module: `jsavrs::translator::block_translator`
Handles basic block translation logic.

#### Struct: `BlockTranslator`
Translates IR basic blocks to assembly.

**Methods:**
- `translate_block(block: &BasicBlock, context: &mut TranslationContext) -> Result<Vec<AssemblyInstruction>, TranslationError>`

### Module: `jsavrs::translator::instruction_translator`
Handles instruction mapping logic.

#### Struct: `InstructionTranslator`
Maps IR instructions to assembly instructions.

**Methods:**
- `translate_instruction(instr: &Instruction, context: &mut TranslationContext) -> Result<Vec<AssemblyInstruction>, TranslationError>`

### Module: `jsavrs::translator::terminator_translator`
Handles control flow translation logic.

#### Struct: `TerminatorTranslator`
Translates IR terminators to assembly control flow.

**Methods:**
- `translate_terminator(terminator: &Terminator, context: &mut TranslationContext) -> Result<Vec<AssemblyInstruction>, TranslationError>`

### Module: `jsavrs::translator::codegen::abi_adapter`
Handles ABI-specific code generation.

#### Struct: `AbiAdapter`
Generates ABI-specific assembly code.

**Methods:**
- `generate_prologue(context: &TranslationContext) -> Vec<AssemblyInstruction>`
- `generate_epilogue(context: &TranslationContext) -> Vec<AssemblyInstruction>`
- `map_parameters(params: &[IrParameter], context: &TranslationContext) -> Vec<AssemblyInstruction>`

## Data Types

### AssemblyInstruction
Represents a single assembly instruction.

**Fields:**
- `mnemonic: String` - Instruction mnemonic (e.g., "mov", "add")
- `operands: Vec<AssemblyOperand>` - Instruction operands
- `comment: Option<String>` - Optional comment for debugging

### AssemblyOperand
Represents an operand for assembly instructions.

**Variants:**
- `Register(String)` - CPU register
- `MemoryAddress(MemoryLocation)` - Memory address
- `Immediate(i64)` - Immediate value
- `Label(String)` - Code label
- `Symbol(String)` - Symbol reference

### MemoryLocation
Represents a memory location.

**Fields:**
- `base_register: Option<String>` - Base register (optional)
- `index_register: Option<String>` - Index register (optional)
- `scale: u8` - Scale factor (1, 2, 4, or 8)
- `displacement: i32` - Displacement offset

## Error Codes

### ErrorCode::E4001
Specific error code for translation failures when IR constructs have no direct assembly equivalent.

## ABI Integration

The translator leverages existing `src/asm/abi.rs` for:
- Parameter passing via `Abi::int_param_registers()`
- Prologue/epilogue generation via `Abi::callee_saved_gp_registers()`
- Platform-specific stack allocation via `Abi::shadow_space()` and `Abi::red_zone()`

## Configuration Flags

The translator supports the following configuration flags:
- `--target-abi`: Selects target ABI (system-v or windows-x64), defaults to platform default
- `--emit-mapping`: Enables source mapping generation (IR_LINE:COL → ASM_LINE:LABEL)

## Testing

Tests for the translator module are located directly in the tests folder:
- `tests/translator_basic.rs` - Basic IR to assembly translation tests
- `tests/translator_abi.rs` - ABI-specific behavior tests
- `tests/translator_errors.rs` - Error handling tests
- `tests/snapshots/` - Insta snapshot files for assembly output validation

Benchmarks for the translator module are located in:
- `benches/jsavrs_benchmark.rs` - Performance benchmarks for translation speed