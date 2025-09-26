# Assembly Generator Interface Contracts

**Feature**: x86-64 Assembly Code Generator for jsavrs Compiler  
**Date**: 26 settembre 2025  
**Design Phase**: Phase 1 - Interface Contracts

## Contract Overview

This document defines the interface contracts for the assembly code generator components, specifying the expected behavior, inputs, outputs, and error conditions for each major interface.

## Core Interface Contracts

### AssemblyGenerator Contract

**Interface**: Primary assembly generation orchestrator

**Input Contract**:
```rust
pub struct GenerationRequest {
    pub ir_modules: Vec<IRModule>,
    pub target_config: TargetConfiguration,
    pub optimization_level: OptimizationLevel,
    pub debug_level: DebugLevel,
    pub output_path: PathBuf,
}

pub struct TargetConfiguration {
    pub architecture: TargetArchitecture, // Currently: X86_64
    pub calling_convention: CallingConventionType, // SystemV | MicrosoftX64
    pub platform: TargetPlatform, // Linux | Windows | MacOS
    pub pic_mode: bool, // Position-independent code generation
}
```

**Output Contract**:
```rust
pub struct GenerationResult {
    pub assembly_files: Vec<AssemblyFile>,
    pub symbol_table: SymbolTable,
    pub debug_info: Option<DebugInformation>,
    pub compilation_metrics: CompilationMetrics,
    pub errors: Vec<CompilationError>,
    pub warnings: Vec<CompilationWarning>,
}

pub struct AssemblyFile {
    pub file_path: PathBuf,
    pub content: String, // NASM-compatible assembly
    pub module_id: ModuleId,
    pub exported_symbols: Vec<ExportedSymbol>,
}
```

**Behavior Contract**:
- **MUST** validate all IR modules before processing
- **MUST** generate NASM-assemblable output for valid IR constructs
- **MUST** maintain semantic equivalence between IR and assembly
- **MUST** support concurrent processing of independent modules
- **MUST** handle errors gracefully with detailed diagnostics
- **MUST** respect target platform calling conventions

**Error Conditions**:
- `InvalidIRModule`: Malformed or semantically invalid IR
- `UnsupportedArchitecture`: Non-x86-64 target architecture
- `DependencyResolutionFailure`: Circular or unresolvable module dependencies
- `ResourceExhaustion`: Insufficient memory or system resources

### InstructionMapper Contract

**Interface**: IR instruction to x86-64 assembly translation

**Input Contract**:
```rust
pub trait InstructionMapper: Send + Sync {
    fn map_arithmetic(&self, op: ArithmeticOperation) -> Result<Vec<X86Instruction>, MappingError>;
    fn map_memory_access(&self, access: MemoryAccess) -> Result<Vec<X86Instruction>, MappingError>;
    fn map_control_flow(&self, control: ControlFlow) -> Result<Vec<X86Instruction>, MappingError>;
    fn map_function_call(&self, call: FunctionCall) -> Result<Vec<X86Instruction>, MappingError>;
    fn map_type_conversion(&self, conversion: TypeConversion) -> Result<Vec<X86Instruction>, MappingError>;
}

pub struct ArithmeticOperation {
    pub operation_type: ArithmeticType, // Add, Sub, Mul, Div, Mod, etc.
    pub operands: Vec<IRValue>,
    pub result_type: IRType,
    pub source_location: Option<SourceLocation>,
}
```

**Output Contract**:
```rust
pub struct X86Instruction {
    pub mnemonic: String,
    pub operands: Vec<X86Operand>,
    pub instruction_bytes: Vec<u8>, // iced-x86 encoded bytes
    pub source_mapping: Option<SourceLocation>,
}

pub enum X86Operand {
    Register(PhysicalRegister),
    Memory(MemoryAddress),
    Immediate(ImmediateValue),
    Label(LabelReference),
}
```

**Behavior Contract**:
- **MUST** preserve operation semantics from IR to x86-64
- **MUST** handle all supported IR data types (integers, floats, pointers, strings)
- **MUST** generate stub code for unsupported operations with TODO comments
- **MUST** maintain type safety and prevent invalid conversions
- **MUST** optimize instruction sequences when possible

**Error Conditions**:
- `UnsupportedOperation`: IR operation has no x86-64 equivalent
- `InvalidOperandType`: Type mismatch in operation
- `InstructionEncodingFailure`: iced-x86 encoding error

### RegisterAllocator Contract

**Interface**: Variable-to-register assignment with spilling support

**Input Contract**:
```rust
pub trait RegisterAllocator: Send + Sync {
    fn allocate_registers(
        &mut self,
        live_ranges: &LiveRanges,
        calling_convention: &dyn CallingConvention,
    ) -> Result<AllocationResult, AllocationError>;
    
    fn handle_register_pressure(
        &mut self,
        conflicts: &InterferenceGraph,
    ) -> Result<SpillDecision, AllocationError>;
}

pub struct LiveRanges {
    pub ranges: HashMap<ValueId, LiveRange>,
    pub interference_graph: InterferenceGraph,
}

pub struct LiveRange {
    pub start: InstructionIndex,
    pub end: InstructionIndex,
    pub value_type: IRType,
    pub usage_frequency: f64,
}
```

**Output Contract**:
```rust
pub struct AllocationResult {
    pub register_assignments: HashMap<ValueId, PhysicalRegister>,
    pub spill_locations: HashMap<ValueId, StackLocation>,
    pub spill_code: Vec<SpillInstruction>,
    pub allocation_statistics: AllocationStatistics,
}

pub struct SpillInstruction {
    pub instruction_type: SpillType, // Load | Store
    pub location: InstructionIndex,
    pub value: ValueId,
    pub stack_offset: i32,
}
```

**Behavior Contract**:
- **MUST** respect calling convention register usage constraints
- **MUST** minimize register spilling through efficient allocation
- **MUST** generate correct spill/reload code for stack operations
- **MUST** maintain program correctness across register pressure scenarios
- **SHOULD** optimize allocation for frequently used variables

**Error Conditions**:
- `AllocationFailure`: Unable to allocate registers even with spilling
- `CallingConventionViolation`: Register usage conflicts with ABI requirements
- `StackOverflow`: Spill requirements exceed stack space

### DebugInfoGenerator Contract

**Interface**: Debug information generation with configurable levels

**Input Contract**:
```rust
pub trait DebugInfoGenerator: Send + Sync {
    fn generate_debug_info(
        &self,
        assembly_output: &AssemblyOutput,
        debug_level: DebugLevel,
    ) -> Result<DebugInformation, DebugGenerationError>;
    
    fn map_source_locations(
        &self,
        ir_mapping: &IRSourceMapping,
    ) -> Result<SourceLocationTable, MappingError>;
}

pub enum DebugLevel {
    Minimal,     // Level 0: Basic symbols
    Standard,    // Level 1: Variables + IR mapping  
    Enhanced,    // Level 2: Types + DWARF sections
    Full,        // Level 3: Complete debugging support
}
```

**Output Contract**:
```rust
pub struct DebugInformation {
    pub symbol_table: DebugSymbolTable,
    pub line_number_table: LineNumberTable,
    pub dwarf_sections: Option<DwarfSections>, // Level 2+
    pub type_information: Option<TypeInfoTable>, // Level 2+
    pub variable_locations: VariableLocationTable,
}

pub struct DwarfSections {
    pub debug_info: Vec<u8>,
    pub debug_line: Vec<u8>,
    pub debug_str: Vec<u8>,
    pub debug_abbrev: Vec<u8>,
}
```

**Behavior Contract**:
- **MUST** generate debug information according to specified level
- **MUST** maintain ±2 line accuracy for source location mapping
- **MUST** produce DWARF-compatible sections for Level 2+
- **MUST** ensure debugger compatibility (GDB, LLDB) for enhanced levels
- **MUST** keep performance overhead within specified limits per level

**Error Conditions**:
- `InvalidDebugLevel`: Unsupported debug level specification
- `SourceMappingFailure`: Unable to correlate IR with source locations
- `DwarfGenerationError`: DWARF section creation failure

### ErrorHandler Contract

**Interface**: Structured error handling and recovery

**Input Contract**:
```rust
pub trait ErrorHandler: Send + Sync {
    fn report_error(&mut self, error: CompilationError) -> HandleResult;
    fn report_warning(&mut self, warning: CompilationWarning) -> HandleResult;
    fn generate_stub_code(&self, unsupported: UnsupportedConstruct) -> StubCode;
    fn should_continue_compilation(&self) -> bool;
}

pub struct CompilationError {
    pub severity: ErrorSeverity, // Critical | High | Medium | Low
    pub error_type: ErrorType,
    pub location: Option<SourceLocation>,
    pub message: String,
    pub suggestions: Vec<String>,
}
```

**Output Contract**:
```rust
pub struct HandleResult {
    pub action: ErrorAction, // Continue | GenerateStub | Abort
    pub recovery_code: Option<StubCode>,
    pub diagnostic_info: DiagnosticInfo,
}

pub struct StubCode {
    pub assembly_instructions: Vec<String>,
    pub todo_comment: String, // Format: "; TODO: Unsupported IR construct [type] - [reason]"
    pub placeholder_behavior: PlaceholderBehavior,
}

pub struct DiagnosticInfo {
    pub json_output: String, // Machine-readable error information
    pub human_readable: String,
    pub error_count: u32,
    pub warning_count: u32,
}
```

**Behavior Contract**:
- **MUST** classify errors by severity and type accurately
- **MUST** generate standardized stub code for recoverable errors
- **MUST** provide JSON output for tooling integration
- **MUST** continue compilation when possible (95% of recoverable errors)
- **MUST** maintain error statistics and thresholds

**Error Conditions**:
- `ErrorThresholdExceeded`: Too many errors to continue safely
- `CriticalSystemFailure`: Unrecoverable system or resource error

## Contract Validation Requirements

### Input Validation
- All IR modules must pass semantic validation before processing
- Configuration parameters must be within valid ranges
- File paths must be accessible and writable

### Output Validation
- Generated assembly must be syntactically correct NASM
- All exported symbols must be properly declared
- Debug information must be valid for target debuggers

### Performance Contracts
- Assembly generation must complete within 2x baseline IR processing time
- Memory usage must not exceed 4x input IR size
- Debug information overhead must remain within specified level limits

### Error Handling Contracts
- All error conditions must be properly classified and handled
- Recovery mechanisms must preserve compilation state consistency
- Diagnostic output must be complete and actionable

## Testing Contracts

Each interface contract will be validated through:
- **Unit Tests**: Individual contract compliance verification
- **Integration Tests**: End-to-end contract behavior validation
- **Property Tests**: Contract invariant verification across input ranges
- **Performance Tests**: Contract performance requirement validation

These contracts provide the specification for implementing and testing the assembly generator components while ensuring consistent behavior and error handling across the entire system.