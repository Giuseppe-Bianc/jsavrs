# Implementation Plan: x86-64 NASM Assembly Code Generator

**Branch**: `007-x86-64-asm-generator` | **Date**: 2025-10-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/007-x86-64-asm-generator/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

The primary requirement is to create a comprehensive Assembly code generator within the `src/asm` folder that translates the intermediate representation (IR) from `src/ir` into correct x86-64 NASM assembly code. The generator must handle all supported IR constructs (instructions, terminators, control flow graphs) and produce valid, platform-appropriate assembly for Windows (Microsoft x64 ABI), Linux, and macOS (System V ABI). The technical approach employs linear scan register allocation with register spilling, critical edge splitting for SSA phi function resolution, and structured error accumulation to enable partial code generation when encountering unsupported constructs. The generator will leverage existing infrastructure in `src/asm` (registers, instructions, ABIs, sections) and `src/ir` (modules, functions, CFGs, types, values) to implement a modular, extensible code generation pipeline.

## Technical Context

**Language/Version**: Rust 2024 Edition (Rust 1.75+)  
**Primary Dependencies**: 
- **Core**: `logos` (0.15.1 - lexer), `clap` (4.5.49 - CLI), `thiserror` (2.0.17 - errors), `uuid` (1.18.1 - IDs), `petgraph` (0.8.3 - CFG)
- **Testing**: `insta` (1.43.2 - snapshots), `criterion` (0.7.0 - benchmarks), `assert_cmd` (2.0.17), `predicates` (3.1.3)
- **Existing Infrastructure**: `src/asm` modules (register, instruction, abi, section, data_directive), `src/ir` modules (module, function, cfg, instruction, terminator, types, value, ssa)

**Storage**: Filesystem (reading IR input, writing .asm output files)  
**Testing**: `cargo test` (unit + integration tests), `insta` snapshot tests, `criterion` benchmarks  
**Target Platform**: x86-64 architecture on Windows (MSVC x64 ABI), Linux (System V AMD64 ABI), macOS (System V AMD64 ABI with platform-specific symbol mangling)  
**Project Type**: Single Rust library/binary project (compiler infrastructure)  
**Performance Goals**: 
- Best-effort target: <1 second per 1000 IR instructions on standard dev hardware
- Linear scan register allocation: O(n) complexity where n = number of IR instructions
- File I/O: Sub-millisecond for typical .asm file writes (<100KB)

**Constraints**: 
- Memory safety: Zero unsafe code in generator (unless absolutely necessary for performance with extensive justification)
- Correctness: All core-supported IR constructs must produce valid, executable assembly
- Error recovery: Must accumulate errors without crashing, enabling partial code generation
- Type support: Limited to I8-I64, U8-U64, F32, F64, Bool, Char, Pointer, Void (generate errors for I128, structs, first-class arrays)
- Register allocation: Must handle up to 20 live values without invalid assembly via spilling

**Scale/Scope**: 
- Handles IR modules with multiple functions (10-1000 functions typical)
- Functions with 10-500 basic blocks
- Basic blocks with 5-100 instructions each
- Supports 40+ IR instruction kinds + 6 terminator kinds
- 3 target platforms × 2 ABIs = comprehensive cross-platform support

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**✅ PASS - All principles satisfied**

- **Safety First**: Generator will use Rust's ownership system with zero unsafe code unless justified for performance. Error handling via `Result<T, E>` and `thiserror` ensures proper propagation without panics. All IR traversal uses safe iterators.

- **Performance Excellence**: Linear scan register allocation provides O(n) complexity. Leveraging existing `petgraph` CFG for efficient traversal. Criterion benchmarks will track performance across iterations. Code generation targets <1s per 1000 IR instructions.

- **Cross-Platform Compatibility**: Explicit ABI selection (System V vs Microsoft x64) based on IR `TargetTriple`. Platform-specific calling conventions, symbol mangling, and stack conventions handled through existing `Abi` infrastructure. CI will test on Windows, Linux, and macOS.

- **Modular Extensibility**: Generator implemented as separate module (`src/asm/generator.rs`) with clean interfaces. Register allocator, instruction selector, and ABI adapter are separable components. Future optimizations can be added without core refactoring.

- **Test-Driven Reliability**: Comprehensive test plan includes unit tests (per component), integration tests (end-to-end IR → assembly), snapshot tests (Insta for assembly output validation), and property tests (round-trip assembly validation). Each user story has explicit acceptance tests.

- **Snapshot Validation**: Insta library will validate generated assembly output for all test cases. Snapshots capture function prologues/epilogues, instruction sequences, label generation, and error messages to catch regressions.

- **Documentation Rigor**: Will create detailed `research.md` (algorithms, trade-offs, alternatives), `data-model.md` (generator state, register allocator internals, IR-to-asm mappings), and comprehensive rustdoc comments for all public APIs. AI assistance will ensure thoroughness.

## Project Structure

### Documentation (this feature)

```
specs/007-x86-64-asm-generator/
├── plan.md              # This file (/speckit.plan command output)
├── spec.md              # Feature specification (already exists)
├── research.md          # Phase 0 output: algorithms, trade-offs, register allocation strategies
├── data-model.md        # Phase 1 output: generator state machines, IR→ASM mappings, register allocator internals
├── quickstart.md        # Phase 1 output: using the generator, API examples, integration guide
├── contracts/           # Phase 1 output: public API contracts (GeneratorContext, RegisterAllocator traits)
│   ├── generator_api.md       # Core generator interface contract
│   ├── register_allocator.md  # Register allocation interface contract
│   └── instruction_selector.md # Instruction selection interface contract
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```
src/
├── asm/
│   ├── mod.rs                    # Existing: exports all submodules
│   ├── abi.rs                    # Existing: AbiKind, Abi, VariadicInfo, calling conventions
│   ├── register.rs               # Existing: all register enums, X86Register, calling convention arrays
│   ├── instruction.rs            # Existing: Instruction enum, Operand, MemoryOperand, Immediate
│   ├── data_directive.rs         # Existing: DataDirective, AssemblyElement, AssemblySection
│   ├── section.rs                # Existing: Section enum
│   ├── generator.rs              # NEW: Main code generator (AsmGenerator struct)
│   ├── register_allocator.rs     # NEW: Linear scan register allocation with spilling
│   ├── instruction_selector.rs   # NEW: IR instruction → x86-64 instruction selection
│   ├── phi_resolver.rs           # NEW: Critical edge splitting, phi function resolution
│   ├── prologue_epilogue.rs      # NEW: Function prologue/epilogue generation
│   └── error.rs                  # NEW: Code generation error types
│
├── ir/
│   ├── mod.rs                    # Existing: exports IR modules
│   ├── module.rs                 # Existing: Module, DataLayout, TargetTriple
│   ├── function.rs               # Existing: Function, CFG, parameters
│   ├── basic_block.rs            # Existing: BasicBlock
│   ├── instruction.rs            # Existing: Instruction, InstructionKind
│   ├── terminator.rs             # Existing: Terminator, TerminatorKind
│   ├── types.rs                  # Existing: IrType enum
│   ├── value/                    # Existing: Value, ValueKind, literals, constants
│   ├── cfg.rs                    # Existing: ControlFlowGraph
│   ├── dominance.rs              # Existing: DominanceInfo
│   └── ssa.rs                    # Existing: SsaTransformer
│
├── error/
│   ├── mod.rs                    # Existing: error infrastructure
│   └── codegen_error.rs          # NEW: Code generation specific errors
│
└── lib.rs                        # Existing: library root, exports modules

tests/
├── asm_generator_tests.rs        # NEW: Unit tests for AsmGenerator
├── register_allocator_tests.rs   # NEW: Unit tests for register allocation
├── instruction_selector_tests.rs # NEW: Unit tests for instruction selection
├── phi_resolver_tests.rs         # NEW: Unit tests for phi resolution
├── codegen_integration_tests.rs  # NEW: End-to-end IR → assembly tests
├── codegen_snapshot_tests.rs     # NEW: Insta snapshots for assembly output
├── abi_compliance_tests.rs       # NEW: ABI conformance tests (calling conventions)
│
└── snapshots/                     # NEW: Insta snapshot storage
    └── codegen_snapshot_tests/
        ├── basic_arithmetic.snap
        ├── control_flow.snap
        ├── function_calls.snap
        └── ...

asm_output/                        # Existing: Generated .asm files for manual inspection
    ├── factorial.asm
    ├── float_test.asm
    └── ...

benches/
    └── codegen_benchmark.rs       # NEW: Criterion benchmarks for code generation performance
```

**Structure Decision**: Single Rust library project (Option 1). The generator is implemented as a new module within the existing `src/asm` directory, leveraging all existing infrastructure (`abi.rs`, `register.rs`, `instruction.rs`, `section.rs`, `data_directive.rs`) and consuming IR from `src/ir`. This modular approach ensures clean separation of concerns while maximizing code reuse. New components (`generator.rs`, `register_allocator.rs`, `instruction_selector.rs`, `phi_resolver.rs`, `prologue_epilogue.rs`) are independent modules with well-defined interfaces, supporting future extensibility (e.g., adding graph-coloring register allocation or optimization passes). Testing follows Rust conventions with unit tests in `tests/` directory and benchmarks in `benches/`.

## Complexity Tracking

*No violations detected - Constitution Check passed*

All architectural decisions align with jsavrs constitution principles. The modular design (separate modules for register allocation, instruction selection, phi resolution) supports extensibility without unnecessary complexity. Linear scan register allocation is the simplest correct approach for initial implementation, with clear paths for future optimization (graph coloring) if needed. Error accumulation strategy enables graceful degradation without introducing complex recovery logic.

