# Implementation Plan: Cross-Platform x86_64 Assembly Code Generator

**Branch**: `006-the-feature-to` | **Date**: 2025-10-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-the-feature-to/spec.md`

**Note**: This plan documents the implementation of a comprehensive IR-to-Assembly code generator with meticulous attention to ABI compliance, value mapping precision, and cross-platform compatibility.

## Summary

This feature implements a production-grade x86_64 assembly code generator that transforms the jsavrs custom Intermediate Representation (IR) into NASM-syntax assembly code. The generator supports both Windows ABI (Microsoft x64) and System V ABI (Linux/macOS) with automatic platform detection. The implementation prioritizes correctness over performance, ensuring meticulous IR node processing, precise type-safe value-to-operand conversion, and exact stack frame layout calculations to prevent address assignment errors and ABI violations.

**Core Technical Approach**: Modular pipeline architecture with specialized components for instruction selection, register allocation, stack frame management, and ABI-compliant function prologue/epilogue generation. All conversions utilize type-safe enumerations to eliminate string-based validation errors.

**Cross-References**:
- ABI compliance requirements from spec.md: See Functional Requirements FR-003 through FR-008
- Detailed ABI specifications: Refer to spec.md section "Requirements" and "Edge Cases"

## Technical Context

**Language/Version**: Rust 1.75+ with strict type safety, zero-cost abstractions, and comprehensive error handling  
**Primary Dependencies**: 
- Existing jsavrs IR infrastructure (`src/ir/*`): instruction.rs, value.rs, types.rs, basic_block.rs, cfg.rs, ssa.rs
- Type promotion engine (`src/ir/type_promotion.rs`, `src/ir/type_promotion_engine.rs`)
- Existing ABI/Register infrastructure (`src/asm/abi.rs`, `src/asm/register.rs`)
- External: NASM assembler for validation

**Storage**: N/A (processes in-memory IR, outputs text-based .asm files)  

**Testing**: 
- cargo test with comprehensive unit and integration tests
- Insta snapshot testing for assembly output validation
- External NASM validation for syntax correctness
- Cross-platform CI testing (Windows, Linux, macOS)

**Target Platform**: x86_64 baseline (no AVX/AVX2/SSE beyond baseline) on Windows, Linux, and macOS  

**Project Type**: Compiler backend component (single project structure)  

**Performance Goals**: 
- Process ≥1,000 IR instructions per second
- Complete generation in <30 seconds for programs up to 10,000 IR instructions
- Minimal memory overhead during generation

**Constraints**: 
- Baseline x86_64 instruction set only (reject operations requiring optional extensions)
- ABI-compliant calling conventions (no violations)
- Memory-safe Rust with minimal unsafe code
- Deterministic output for testing
- Precise stack offset calculations to prevent memory corruption

**Scale/Scope**: 
- Support all IR instruction types defined in `src/ir/instruction.rs`
- Handle complex control flow (loops, conditionals, nested calls)
- Support both primitive types and aggregates (structs, arrays)
- Process IR modules with 10,000+ instructions efficiently

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, and Documentation Rigor. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

### Alignment Verification

✅ **Safety First**: Implementation uses Rust's type system to prevent memory safety issues. All IR value conversions use type-safe enumerations. Stack frame calculations use checked arithmetic to prevent overflow. Minimal unsafe code, only where necessary for low-level operations.

✅ **Performance Excellence**: Modular architecture enables independent optimization of components. Target: ≥1,000 IR instructions/second. Efficient memory management with pre-allocated buffers where appropriate. Zero-cost abstractions through Rust's trait system.

✅ **Cross-Platform Compatibility**: Automatic ABI selection based on target platform. Comprehensive testing on Windows, Linux, and macOS through CI. Platform-agnostic IR processing with platform-specific code generation only in ABI layer.

✅ **Modular Extensibility**: Clear separation of concerns: instruction selection, register allocation, stack management, and code emission. Well-defined interfaces between components. Easy to add new instruction mappings or optimization passes.

✅ **Test-Driven Reliability**: Comprehensive unit tests for all components. Integration tests for end-to-end IR-to-assembly transformation. Snapshot testing with Insta for assembly output validation. External NASM validation for syntax correctness. Property-based testing for complex transformations.

✅ **Snapshot Validation**: Extensive use of Insta for capturing and comparing assembly output. Separate snapshots for different ABIs and instruction types. Regression detection for unintended output changes.

✅ **Documentation Rigor**: Detailed research.md documenting architecture decisions and trade-offs. Comprehensive data-model.md explaining IR-to-assembly mappings. Inline rustdoc comments for all public APIs. Usage examples in quickstart.md.

## Project Structure

### Documentation (this feature)

```
specs/006-the-feature-to/
├── plan.md              # This file (comprehensive implementation plan)
├── research.md          # Phase 0: Architecture decisions, IR analysis, ABI research
├── data-model.md        # Phase 1: IR-to-assembly mappings, type conversions, stack layouts
├── quickstart.md        # Phase 1: Usage guide and examples
└── contracts/           # Phase 1: Component interfaces and error types
    ├── codegen.md       # CodeGenerator API contract
    ├── instruction_selector.md  # InstructionSelector API contract
    ├── value_mapper.md  # ValueMapper API contract
    ├── register_allocator.md    # RegisterAllocator API contract
    └── errors.md        # Error types and conversions
```

### Source Code (repository root)

```
src/
├── asm/
│   ├── mod.rs                      # Module exports
│   ├── abi.rs                      # Existing ABI definitions (Windows/SystemV)
│   ├── register.rs                 # Existing register enumerations
│   ├── instruction.rs              # Existing instruction types
│   ├── data_directive.rs           # Existing data directive types
│   ├── section.rs                  # Existing section types
│   └── codegen/                    # NEW: Code generation pipeline
│       ├── mod.rs                  # Codegen module exports
│       ├── generator.rs            # Main CodeGenerator orchestrator
│       ├── context.rs              # CodegenContext: generation state tracking
│       ├── function_prologue.rs    # ABI-compliant function entry generation
│       ├── function_epilogue.rs    # ABI-compliant function exit generation
│       ├── instruction_selector.rs # IR instruction → assembly mapping
│       ├── value_mapper.rs         # IR Value → assembly operand conversion
│       ├── register_allocator.rs   # Register allocation and spilling
│       ├── stack_frame.rs          # Stack frame layout and offset calculation
│       ├── operand.rs              # Assembly operand types (register/memory/immediate)
│       ├── emitter.rs              # Assembly code text generation
│       └── error.rs                # Codegen-specific error types
├── ir/
│   ├── [existing IR modules]       # instruction.rs, value/, types.rs, etc.
│   └── [no changes required]
└── error/
    └── [existing error handling]   # compile_error.rs integration

tests/
├── codegen_tests.rs                # NEW: Unit tests for code generation
├── codegen_snapshot_tests.rs       # NEW: Snapshot tests for assembly output
├── abi_tests.rs                    # NEW: ABI compliance tests
├── register_allocation_tests.rs    # NEW: Register allocator tests
├── stack_frame_tests.rs            # NEW: Stack frame layout tests
├── value_mapping_tests.rs          # NEW: Value mapper tests
└── snapshots/
    ├── codegen/                    # Assembly output snapshots
    │   ├── windows_abi/
    │   └── systemv_abi/
    └── [existing snapshot dirs]
```

**Structure Decision**: Single project structure with new `src/asm/codegen/` submodule. This aligns with the existing architecture where `src/ir/` contains the IR infrastructure and `src/asm/` contains assembly-related components. The code generation pipeline is logically grouped under `codegen/` with clear separation of concerns between components.

## Complexity Tracking

*No constitutional violations detected. All architectural decisions align with jsavrs core principles.*

The modular code generation architecture with clear separation of concerns is justified by:
1. **Safety First**: Type-safe conversions prevent entire classes of bugs
2. **Modular Extensibility**: Components can be tested and optimized independently
3. **Test-Driven Reliability**: Each component has focused, comprehensive tests

The complexity of separate modules for instruction selection, register allocation, stack management, and code emission is **necessary** because:
- IR-to-assembly transformation involves fundamentally distinct concerns
- Combining these concerns would create untestable monolithic code
- ABI compliance requires precise separation between portable and platform-specific logic

No simpler architecture would meet the requirements while maintaining the quality standards defined in the constitution.

---

## Phase 0: Research & Architecture Design

### Objectives

1. **Analyze existing IR infrastructure** to understand all instruction types, value representations, and type system constraints
2. **Research x86_64 instruction selection patterns** for each IR instruction type, considering baseline instruction set limitations
3. **Document ABI calling convention details** for both Windows and System V ABIs, with focus on parameter passing, return values, and callee-saved registers
4. **Design value mapping strategy** for converting IR values (literals, temporaries, locals, globals) to assembly operands with type safety
5. **Design stack frame layout algorithm** ensuring precise offset calculations and ABI-compliant alignment
6. **Research register allocation strategies** suitable for baseline x86_64 with clear spilling policies
7. **Document error handling approach** for invalid IR, unsupported operations, and assembler failures

### Research Tasks

#### Task 1: IR Infrastructure Analysis
**Question**: What are all the IR instruction types, their operands, and their semantics?  
**Approach**: 
- Analyze `src/ir/instruction.rs`: `InstructionKind` variants (Alloca, Store, Load, Binary, Unary, Call, GetElementPtr, Cast, Phi, Vector)
- Analyze `src/ir/value/mod.rs`: `ValueKind` variants (Literal, Constant, Local, Global, Temporary)
- Analyze `src/ir/types.rs`: All `IrType` variants and their sizes/alignments
- Document operand types for each instruction
- Identify instructions requiring multi-instruction assembly sequences

**Deliverable**: Comprehensive IR instruction reference in `research.md` section "IR Instruction Catalog"

### Phase 0 Status

- **research.md**: Created at `specs/006-the-feature-to/research.md` (resolves all NEEDS CLARIFICATION)
- **Status**: Completed ✅


#### Task 2: x86_64 Instruction Selection Patterns
**Question**: How does each IR instruction map to x86_64 assembly, considering baseline instruction set?  
**Approach**:
- For each IR instruction type, document the corresponding x86_64 instruction(s)
- Identify IR operations requiring instruction sequences (e.g., division, modulo, large copies)
- Document baseline x86_64 constraints (no AVX, SSE4, etc.)
- Research floating-point operation handling with x87 or SSE2 only
- Identify operations that cannot be implemented with baseline instructions

**Deliverable**: Instruction selection matrix in `research.md` section "Instruction Selection Patterns"

#### Task 3: ABI Calling Convention Deep Dive
**Question**: What are the exact rules for parameter passing, return values, and register preservation for both ABIs?  
**Approach**:
- Study Windows x64 ABI documentation (Microsoft official docs)
- Study System V AMD64 ABI documentation (official specification)
- Document parameter passing rules (integer vs. float, register vs. stack)
- Document return value rules (register returns, hidden pointer for large structs)
- Document callee-saved register requirements
- Document stack alignment requirements (16-byte for both ABIs)
- Document shadow space (Windows) and red zone (System V)
- Document variadic function handling differences

**Deliverable**: ABI comparison table in `research.md` section "ABI Calling Conventions"

#### Task 4: Value Mapping Strategy Design
**Question**: How do we safely and precisely convert IR values to assembly operands?  
**Approach**:
- Design `Operand` enumeration: Register(X86Register), Memory(base, offset, size), Immediate(i64/f64)
- Design type-safe conversion from `IrType` to register class (GP vs. XMM)
- Design stack offset tracking for locals and temporaries
- Design global symbol reference handling
- Design literal value encoding (immediate vs. memory load)
- Ensure no string-based operand construction (use enums only)

**Deliverable**: Value mapping architecture in `research.md` section "Value Mapping Design"

#### Task 5: Stack Frame Layout Algorithm
**Question**: How do we calculate exact stack offsets for locals and temporaries to prevent memory corruption?  
**Approach**:
- Design `StackFrame` structure tracking current offset, alignment, and allocated regions
- Design allocation algorithm respecting ABI alignment (16-byte stack alignment)
- Design offset calculation for different type sizes (i8, i16, i32, i64, f32, f64, arrays, structs)
- Ensure stack grows downward consistently (negative offsets from RBP)
- Design handling for dynamic allocations (alloca instruction)
- Document alignment formulas with concrete examples

**Deliverable**: Stack frame algorithm in `research.md` section "Stack Frame Management"

#### Task 6: Register Allocation Strategy
**Question**: What register allocation approach balances simplicity with correctness?  
**Approach**:
- Evaluate simple linear scan allocation for first implementation
- Design register priority ordering (prefer caller-saved over callee-saved initially)
- Design spilling strategy when registers exhausted (spill to stack)
- Design handling for register constraints (division uses RDX:RAX, etc.)
- Design tracking of register liveness and allocation state
- Consider separate allocation for GP and XMM registers

**Deliverable**: Register allocation design in `research.md` section "Register Allocation"

#### Task 7: Error Handling Architecture
**Question**: How do we handle errors throughout the generation pipeline?  
**Approach**:
- Design `CodegenError` enumeration covering all error cases
- Design conversion to `CompileError` for integration with existing error system
- Document error cases: invalid IR, unsupported operations, register allocation failures
- Design assembler failure analysis and error enrichment (See spec.md:FR-022 for detailed requirements)
- Design error context tracking (source spans, IR instruction references)

**Deliverable**: Error handling design in `research.md` section "Error Handling Strategy"

### Research Deliverable Structure

The `research.md` file will contain:

1. **Executive Summary**: High-level architecture overview
2. **IR Instruction Catalog**: Complete reference of all IR instructions with operand types
3. **Instruction Selection Patterns**: IR → x86_64 mapping table with multi-instruction sequences
4. **ABI Calling Conventions**: Detailed comparison of Windows vs. System V ABIs
5. **Value Mapping Design**: Type-safe IR value to assembly operand conversion
6. **Stack Frame Management**: Precise offset calculation algorithm with examples
7. **Register Allocation**: Strategy for physical register assignment and spilling
8. **Error Handling Strategy**: Comprehensive error types and conversion mechanisms
9. **Architecture Decisions Summary**: Rationale for key design choices
10. **Alternatives Considered**: Rejected approaches with justifications

---

## Phase 1: Design & Contracts

### Prerequisites
- `research.md` completed with all NEEDS CLARIFICATION resolved
- Architecture decisions documented and validated

### Design Tasks

#### Task 1: Data Model Design (`data-model.md`)

**Objective**: Document all data structures, their relationships, and invariants.

**Entities to Model**:

1. **CodegenContext**
   - Fields: current_function, register_state, stack_frame, label_counter, abi, target_platform
   - Invariants: Stack alignment maintained, register allocation consistent, label uniqueness
   - Relationships: Contains StackFrame, references Abi, tracks RegisterAllocator state

2. **StackFrame**
   - Fields: current_offset (i32), alignment (usize), local_area_size (i32), allocations (HashMap<ValueId, i32>)
   - Invariants: Offsets always negative (stack grows down), alignment always multiple of ABI requirement, no overlapping allocations
   - Relationships: Owned by CodegenContext, references IR Values

3. **Operand**
   - Variants: Register(X86Register), Memory(base_reg, offset, size), Immediate(value, size)
   - Invariants: Memory offsets aligned appropriately, immediate values fit in specified size, register class matches value type
   - Relationships: Produced by ValueMapper, consumed by InstructionSelector

4. **RegisterAllocator**
   - Fields: gp_allocations (HashMap<ValueId, GPRegister64>), xmm_allocations (HashMap<ValueId, XMMRegister>), free_gp_registers (Vec), free_xmm_registers (Vec)
   - Invariants: No double allocation, callee-saved registers saved before use, allocation consistent with value types
   - Relationships: Managed by CodegenContext, interacts with StackFrame for spilling

5. **AssemblyInstruction**
   - Fields: mnemonic (enum), operands (Vec<Operand>), comment (Option<String>)
   - Invariants: Operand count matches instruction requirements, operand types compatible with instruction
   - Relationships: Emitted by InstructionSelector, written by Emitter

**Deliverable**: Complete data model with diagrams in `data-model.md`

#### Task 2: API Contracts (`contracts/`)

**Objective**: Define clear interfaces for all major components.

**Contract 1: CodeGenerator** (`contracts/codegen.md`)
```rust
pub trait CodeGenerator {
    /// Generate assembly code for an entire IR module
    fn generate_module(&mut self, module: &Module) -> Result<String, CodegenError>;
    
    /// Generate assembly code for a single function
    fn generate_function(&mut self, function: &Function) -> Result<Vec<AssemblyInstruction>, CodegenError>;
}
```

**Contract 2: InstructionSelector** (`contracts/instruction_selector.md`)
```rust
pub trait InstructionSelector {
    /// Select assembly instructions for an IR instruction
    fn select_instruction(&self, instruction: &Instruction, context: &mut CodegenContext) 
        -> Result<Vec<AssemblyInstruction>, CodegenError>;
    
    /// Check if an IR instruction is supported by baseline x86_64
    fn is_supported(&self, instruction: &Instruction) -> bool;
}
```

**Contract 3: ValueMapper** (`contracts/value_mapper.md`)
```rust
pub trait ValueMapper {
    /// Map an IR value to an assembly operand
    fn map_value(&self, value: &Value, context: &CodegenContext) 
        -> Result<Operand, CodegenError>;
    
    /// Determine register class for an IR type
    fn register_class_for_type(&self, ty: &IrType) -> RegisterClass;
}
```

**Contract 4: RegisterAllocator** (`contracts/register_allocator.md`)
```rust
pub trait RegisterAllocator {
    /// Allocate a physical register for a value
    fn allocate(&mut self, value_id: ValueId, ty: &IrType, context: &mut CodegenContext) 
        -> Result<X86Register, CodegenError>;
    
    /// Free a register allocation
    fn free(&mut self, value_id: ValueId);
    
    /// Spill a value to the stack
    fn spill(&mut self, value_id: ValueId, context: &mut CodegenContext) 
        -> Result<i32, CodegenError>; // Returns stack offset
}
```

**Contract 5: Error Types** (`contracts/errors.md`)
```rust
pub enum CodegenError {
    UnsupportedInstruction { instruction: String, reason: String },
    UnsupportedType { ty: IrType, reason: String },
    RegisterAllocationFailed { value_id: ValueId },
    StackOverflow { requested_size: usize },
    InvalidOperand { operand: String, reason: String },
    AbiViolation { description: String },
    AssemblerFailure { output: String, enriched_message: String },
}

impl From<CodegenError> for CompileError {
    fn from(err: CodegenError) -> Self {
        // Conversion implementation
    }
}
```

**Deliverable**: Complete API contracts in `contracts/` directory

#### Task 3: Quickstart Guide (`quickstart.md`)

**Objective**: Provide usage examples and integration guidance.

**Content**:
1. **Basic Usage Example**: Generating assembly from a simple IR function
2. **ABI Selection Example**: Targeting different platforms
3. **Error Handling Example**: Handling generation failures gracefully
4. **Integration Example**: Using the generator in the compiler pipeline
5. **Testing Example**: Validating generated assembly with NASM

**Deliverable**: Complete quickstart guide in `quickstart.md`

#### Task 4: Agent Context Update

**Objective**: Update AI agent context with new technologies and patterns.

**Execution**:
```powershell
cd C:\dev\vscode\rust\jsavrs
.\.specify\scripts\powershell\update-agent-context.ps1 -AgentType copilot
```

**Technologies to Add**:
- NASM assembly syntax patterns
- x86_64 instruction encoding principles
- ABI calling convention patterns
- Register allocation terminology
- Stack frame layout patterns

**Deliverable**: Updated `.github/.copilot-context.md` (or equivalent agent file)

### Phase 1 Validation

**Re-check Constitution Compliance**:
- ✅ Safety First: All designs use type-safe enumerations and checked arithmetic
- ✅ Performance Excellence: Architecture supports future optimizations without redesign
- ✅ Cross-Platform Compatibility: ABI abstraction layer ensures portability
- ✅ Modular Extensibility: Clear component interfaces enable independent development
- ✅ Documentation Rigor: Comprehensive documentation of all designs and contracts

---

## Phase 2: Implementation Planning

**NOTE**: This section documents the planned implementation phases. The actual task breakdown was created by the `/speckit.tasks` command based on this plan. The information here serves as a guide for task generation.

### Implementation Phases Overview

**Phase 2.1: Core Infrastructure** (Est. 2 weeks)
- Implement `CodegenContext` and state management
- Implement `StackFrame` with precise offset calculation
- Implement `Operand` types and validation
- Implement `CodegenError` and `CompileError` conversion
- Unit tests for all infrastructure components

**Phase 2.2: Value Mapping** (Est. 1 week)
- Implement `ValueMapper` with type-safe conversions
- Implement IR literal to immediate conversion
- Implement local/temporary to stack offset conversion
- Implement global to symbol reference conversion
- Unit tests for all value mapping cases

**Phase 2.3: Register Allocation** (Est. 1 week)
- Implement `RegisterAllocator` with linear scan approach
- Implement register priority ordering
- Implement spilling to stack
- Implement constraint handling (e.g., division)
- Unit tests for allocation and spilling

**Phase 2.4: Instruction Selection** (Est. 2 weeks)
- Implement `InstructionSelector` for all IR instruction types
- Implement pattern matching for multi-instruction sequences
- Implement baseline x86_64 instruction validation
- Implement ABI-specific instruction selection
- Unit tests for each instruction type

**Phase 2.5: Function Prologue/Epilogue** (Est. 1 week)
- Implement `FunctionPrologue` generator for both ABIs
- Implement `FunctionEpilogue` generator for both ABIs
- Implement callee-saved register preservation
- Implement stack frame setup/teardown
- Unit tests for both ABIs

**Phase 2.6: Code Emission** (Est. 1 week)
- Implement `Emitter` for text-based assembly output
- Implement NASM syntax formatting
- Implement comment generation for debugging
- Implement section management (.text, .data, .bss, .rodata)
- Unit tests for formatting

**Phase 2.7: Integration & End-to-End Testing** (Est. 2 weeks)
- Implement `CodeGenerator` orchestrator
- Integration tests for complete IR-to-assembly transformation
- Snapshot tests for various IR patterns
- External NASM validation
- Cross-platform CI testing

**Phase 2.8: Error Handling & Validation** (Est. 1 week)
- Implement assembler failure analysis
- Implement error enrichment with IR context
- Implement validation for unsupported operations
- Error handling tests

**Total Estimated Time**: 11 weeks

---

## Success Criteria Verification

### Functional Requirements Coverage

- **FR-001 through FR-022**: All functional requirements addressed in architecture design
- **IR Instruction Processing**: All `InstructionKind` variants handled in instruction selector
- **ABI Compliance**: Dedicated prologue/epilogue generators for both ABIs
- **Cross-Platform**: Automatic ABI selection based on target platform
- **Error Handling**: Comprehensive error types with CompileError conversion
- **Baseline x86_64**: Instruction selection validates against baseline constraints

### Performance Targets

- **1,000+ IR instructions/second**: Achievable with efficient data structures and minimal allocations
- **<30 seconds for 10,000 instructions**: Well within target with current architecture

### Quality Metrics

- **100% assembly success rate**: External NASM validation ensures syntax correctness
- **100% functional equivalence**: Integration tests verify IR semantics preserved
- **ABI compliance**: Dedicated tests for both Windows and System V ABIs
- **>90% test coverage**: Comprehensive unit, integration, and snapshot tests planned

---

## Next Steps

1. **Execute Phase 0**: Generate `research.md` with detailed architecture research
2. **Execute Phase 1**: Generate `data-model.md`, `contracts/`, and `quickstart.md`
3. **Update Agent Context**: Run agent context update script
4. **Run `/speckit.tasks`**: Generate detailed task breakdown for implementation
5. **Begin Implementation**: Follow task sequence from Phase 2.1 through Phase 2.8

---

## Appendix: Key Design Decisions

### Decision 1: Linear Scan Register Allocation

**Rationale**: For the initial implementation, a simple linear scan allocator provides sufficient correctness with lower implementation complexity than graph coloring approaches. Future optimizations can replace this component without affecting other modules.

### Decision 2: Type-Safe Operand System

**Rationale**: Using enumerations for all registers, instructions, and operands eliminates string-based validation errors and enables compile-time checking. This aligns with Rust's type safety principles and prevents entire classes of bugs.

### Decision 3: Separate Prologue/Epilogue Generators

**Rationale**: ABI-compliant function entry/exit code is complex and ABI-specific. Separating this logic into dedicated generators ensures correctness and maintainability, with clear testing boundaries.

### Decision 4: Precise Stack Frame Tracking

**Rationale**: Memory corruption from incorrect stack offsets is a critical failure mode. Explicit tracking of all allocations, checked arithmetic for offsets, and ABI-aligned allocation prevent these errors.

### Decision 5: Modular Pipeline Architecture

**Rationale**: Separating instruction selection, register allocation, and code emission enables independent testing, optimization, and future enhancements (e.g., adding optimization passes) without requiring major refactoring.

---

**Document Status**: Complete - Ready for Phase 0 Execution  
**Last Updated**: 2025-10-15  
**Next Command**: `/speckit.plan` Phase 0 execution to generate `research.md`
