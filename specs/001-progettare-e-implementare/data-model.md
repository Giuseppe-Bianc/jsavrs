# Assembly Generator Data Model

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 26 settembre 2025  
**Design Phase**: Phase 1 - Data Model Specification

## Entity Overview

This document defines the core data structures and relationships for the assembly code generator that translates jsavrs IR into x86-64 NASM-compatible assembly code.

## Core Entities

### AssemblyGenerator

**Purpose**: Primary orchestrator for IR-to-assembly translation process

**Fields**:
- `target_arch: TargetArchitecture` - Target architecture specification (x86-64)
- `calling_convention: Box<dyn CallingConvention>` - ABI implementation (System V or Microsoft x64)
- `debug_level: DebugLevel` - Debug information generation level (0-3)
- `optimization_level: OptimizationLevel` - Optimization configuration
- `symbol_table: SymbolTable` - Global symbol management
- `module_dependencies: DependencyGraph` - Inter-module relationship tracking
- `error_handler: ErrorHandler` - Error management and recovery system

**Relationships**:
- Owns multiple `ModuleGenerator` instances for concurrent processing
- Contains `OptimizationPipeline` for code improvement passes
- Uses `DebugInfoGenerator` for debug metadata creation

**Validation Rules**:
- Must validate IR modules before processing
- Target architecture must be supported (currently x86-64 only)
- Debug level must be 0-3 inclusive
- Calling convention must match target platform

**State Transitions**:
- `Initialized` → `Processing` → `Completed` | `Failed`
- Error states allow partial recovery and continuation

### ModuleGenerator

**Purpose**: Handles assembly generation for individual IR modules

**Fields**:
- `module_ir: IRModule` - Input intermediate representation
- `instruction_mapper: Box<dyn InstructionMapper>` - IR-to-x86-64 mapping
- `register_allocator: RegisterAllocator` - Register assignment management
- `basic_blocks: Vec<BasicBlockAssembly>` - Generated assembly blocks
- `function_table: HashMap<FunctionId, FunctionAssembly>` - Function definitions
- `data_section: DataSection` - Static data management
- `relocation_table: Vec<Relocation>` - Address fixup requirements

**Relationships**:
- Processes `IRModule` from `@src/ir/module.rs`
- Generates `AssemblyOutput` containing NASM code
- Interacts with `SymbolTable` for cross-module references

**Validation Rules**:
- IR module must be semantically valid before processing
- All function calls must be resolvable or marked as external
- Data alignment must conform to x86-64 requirements
- Register allocation must not exceed available physical registers

### InstructionMapper

**Purpose**: Trait for converting IR instructions to x86-64 assembly

**Trait Methods**:
- `map_arithmetic(ir_op: ArithmeticIR) -> Result<Vec<X86Instruction>>`
- `map_memory_access(ir_access: MemoryAccessIR) -> Result<Vec<X86Instruction>>`
- `map_control_flow(ir_control: ControlFlowIR) -> Result<Vec<X86Instruction>>`
- `map_function_call(ir_call: FunctionCallIR) -> Result<Vec<X86Instruction>>`

**Implementations**:
- `X86_64InstructionMapper` - Standard x86-64 instruction mapping
- `OptimizingInstructionMapper` - Mapping with inline optimizations
- `DebugInstructionMapper` - Enhanced mapping with debug annotations

**Validation Rules**:
- All IR operations must have corresponding x86-64 mappings
- Unsupported operations must generate stub code with TODO comments
- Type conversions must preserve semantics and safety

### RegisterAllocator

**Purpose**: Manages assignment of IR values to x86-64 physical registers

**Fields**:
- `live_ranges: HashMap<ValueId, LiveRange>` - Variable lifetime tracking
- `register_assignments: HashMap<ValueId, PhysicalRegister>` - Value-to-register mapping
- `spill_locations: HashMap<ValueId, StackLocation>` - Stack spill management
- `calling_convention: &CallingConvention` - ABI constraints for allocation
- `interference_graph: InterferenceGraph` - Register conflict analysis

**Relationships**:
- Interacts with `CallingConvention` for register usage constraints
- Coordinates with `StackFrame` for spill slot allocation
- Provides input to `InstructionMapper` for operand encoding

**Validation Rules**:
- Live ranges must not exceed register capacity without spilling
- Calling convention registers must be preserved across function calls
- Spill code generation must maintain program semantics

**State Transitions**:
- `Unallocated` → `Analyzing` → `Allocated` → `Optimized`

### SymbolTable

**Purpose**: Manages program symbols, labels, and cross-references

**Fields**:
- `global_symbols: HashMap<String, SymbolInfo>` - Global symbol definitions
- `local_symbols: HashMap<ModuleId, HashMap<String, SymbolInfo>>` - Module-local symbols
- `external_references: HashSet<ExternalSymbol>` - Unresolved external symbols
- `label_counter: AtomicU32` - Unique label generation
- `debug_symbols: Option<DebugSymbolTable>` - Debug information mapping

**Relationships**:
- Referenced by all `ModuleGenerator` instances
- Provides input to `LinkerInfo` for final assembly generation
- Integrates with `DebugInfoGenerator` for symbol debugging

**Validation Rules**:
- Global symbols must be unique across all modules
- External references must be resolved or marked for linker resolution
- Debug symbols must maintain accurate source location mapping

### AssemblyOutput

**Purpose**: Contains generated assembly code and metadata

**Fields**:
- `text_section: String` - Executable assembly instructions
- `data_section: String` - Initialized data declarations  
- `bss_section: String` - Uninitialized data declarations
- `debug_sections: HashMap<String, String>` - Debug information sections
- `symbol_exports: Vec<ExportedSymbol>` - Publicly visible symbols
- `relocations: Vec<RelocationEntry>` - Address fixup requirements
- `metadata: AssemblyMetadata` - Generation statistics and options

**Relationships**:
- Generated by `ModuleGenerator` instances
- Consumed by NASM assembler for object file creation
- Integrated with build system for linking

**Validation Rules**:
- All sections must be valid NASM syntax
- Symbol exports must be properly declared with visibility
- Debug sections must conform to DWARF specification when enabled

### ErrorHandler

**Purpose**: Manages error detection, classification, and recovery

**Fields**:
- `error_log: Vec<CompilationError>` - Accumulated error messages
- `error_counts: HashMap<ErrorSeverity, u32>` - Statistics by severity level
- `recovery_strategies: HashMap<ErrorType, RecoveryStrategy>` - Error handling policies
- `max_errors: u32` - Threshold for compilation termination
- `stub_generator: StubGenerator` - Generates placeholder code for unsupported features

**Relationships**:
- Used by all generator components for error reporting
- Coordinates with `StubGenerator` for graceful degradation
- Provides input to diagnostic output systems

**Validation Rules**:
- Error severity must be properly classified (Critical, High, Medium, Low)
- Recovery strategies must maintain compilation state consistency
- Stub generation must include standardized TODO formatting

**State Transitions**:
- Errors can be: `Detected` → `Classified` → `Handled` → `Resolved` | `Escalated`

### DebugInfoGenerator

**Purpose**: Creates debug information for assembly output

**Fields**:
- `debug_level: DebugLevel` - Configuration for debug information depth
- `source_mapping: SourceLocationMapper` - IR-to-source correlation
- `dwarf_builder: Option<DwarfBuilder>` - DWARF section generation (Level 2+)
- `symbol_mapper: SymbolMapper` - Variable name preservation
- `line_table: LineNumberTable` - Source line correlation

**Relationships**:
- Integrates with `InstructionMapper` for instruction-level mapping
- Uses `SymbolTable` for symbol information
- Coordinates with `AssemblyOutput` for section generation

**Validation Rules**:
- Debug level must determine available information depth
- Source mapping must maintain ±2 line accuracy when possible
- DWARF sections must be valid for target debuggers (GDB, LLDB)

### OptimizationPipeline

**Purpose**: Manages assembly optimization passes

**Fields**:
- `passes: Vec<Box<dyn OptimizationPass>>` - Ordered optimization transformations
- `pass_statistics: HashMap<String, PassStatistics>` - Performance monitoring
- `enabled_passes: HashSet<String>` - Configuration-driven pass selection
- `iteration_limit: u32` - Maximum optimization iterations

**Relationships**:
- Operates on `AssemblyOutput` between generation and finalization
- Coordinates with `DebugInfoGenerator` to maintain debug accuracy
- Reports performance impact through statistics collection

**Validation Rules**:
- Optimizations must preserve program semantics
- Debug information must remain accurate after optimization
- Performance overhead must remain within configured limits

## Data Flow Architecture

### Processing Pipeline

1. **Input Validation**: `IRModule` → `ModuleGenerator` validation
2. **Symbol Resolution**: `SymbolTable` global symbol processing  
3. **Instruction Mapping**: `InstructionMapper` IR-to-x86-64 conversion
4. **Register Allocation**: `RegisterAllocator` variable assignment
5. **Optimization**: `OptimizationPipeline` code improvement
6. **Debug Generation**: `DebugInfoGenerator` metadata creation  
7. **Output Assembly**: `AssemblyOutput` NASM code generation

### Error Handling Flow

1. **Error Detection**: Components report errors to `ErrorHandler`
2. **Classification**: Errors categorized by severity and type
3. **Recovery**: `StubGenerator` creates placeholder code when possible
4. **Continuation**: Processing continues for recoverable errors
5. **Escalation**: Critical errors terminate compilation with diagnostics

### Concurrent Processing Model

- **Independent Modules**: Processed in parallel by separate `ModuleGenerator` instances
- **Shared Resources**: `SymbolTable` and `ErrorHandler` with appropriate synchronization
- **Dependency Resolution**: Topological ordering ensures correct processing sequence
- **Final Assembly**: Sequential merge of module outputs with conflict resolution

## Validation and Constraints

### Type Safety Requirements
- All IR value types must map to valid x86-64 data representations
- Register assignments must respect calling convention constraints
- Memory operations must maintain alignment and safety requirements

### Performance Constraints  
- Debug information generation overhead: Level 0 (0-2%), Level 1 (3-8%), Level 2 (9-15%), Level 3 (16-25%)
- Register allocation must complete in O(n log n) time for n variables
- Symbol resolution must scale sub-linearly with program size

### Cross-Platform Requirements
- Calling convention support must handle both System V and Microsoft x64 ABIs
- File path handling must work consistently across Windows, macOS, and Linux
- Assembly output must be compatible with NASM across all supported platforms

### Error Handling Requirements
- Maximum 100 warnings before suggesting compilation review
- JSON error output must be machine-readable for tooling integration
- Stub code must follow standardized format: `; TODO: Unsupported IR construct [type] - [reason]`

This data model provides the foundation for implementing a robust, efficient, and maintainable assembly code generator that meets all requirements while ensuring type safety and performance objectives.