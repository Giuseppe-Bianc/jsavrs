# Implementation Plan: IR to x86-64 Assembly Code Generator

**Branch**: `021-ir-x86-codegen` | **Date**: 2025-12-16 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/021-ir-x86-codegen/spec.md`

## Summary

Build a code generator that transforms validated IR modules into NASM-compatible x86-64 assembly code with full support for three platforms (Linux, macOS, Windows), using Linear Scan register allocation with liveness analysis, proper SSA phi node resolution, and platform-specific calling conventions.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2024)  
**Primary Dependencies**: Zero runtime dependencies; dev-deps: criterion, proptest, insta  
**Storage**: In-memory only (Vec, HashMap, BTreeSet from std)  
**Testing**: cargo test + insta snapshots + NASM assembly validation  
**Target Platforms**: x86_64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-pc-windows-msvc  
**Project Type**: Single project (existing Rust compiler toolchain)  
**Performance Goals**: 1000 functions within 30 seconds; streaming String buffers for minimal memory  
**Constraints**: Zero external runtime dependencies; single statically-linked executable  
**Scale/Scope**: Medium-size IR modules; up to 1000 functions per module

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle                      | Status  | Notes                                                      |
| ------------------------------ | ------- | ---------------------------------------------------------- |
| Safety First                   | ✅ PASS | Pure Rust with no unsafe; ownership model for IR/ASM data  |
| Performance Excellence         | ✅ PASS | Linear Scan O(n) allocation; streaming output buffers      |
| Cross-Platform Compatibility   | ✅ PASS | Three target platforms with trait-based ABI abstraction    |
| Modular Extensibility          | ✅ PASS | Pipeline architecture with clear phase separation          |
| Test-Driven Reliability        | ✅ PASS | Unit + insta snapshots + NASM validation                   |
| Snapshot Validation            | ✅ PASS | Golden file comparison via insta                           |
| Documentation Rigor            | ✅ PASS | Rustdoc for all public APIs                                |
| Collaboration First            | ✅ PASS | Public GitHub repo with CI                                 |
| Quality Through Community      | ✅ PASS | PR review process                                          |

**Gate Result**: ✅ PASSED - All principles satisfied

## Project Structure

### Documentation (this feature)

```text
specs/021-ir-x86-codegen/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (API contracts)
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── asm/                      # EXISTING - Assembly module
│   ├── mod.rs                # Re-exports
│   ├── abi.rs                # EXISTING - ABI definitions (SystemV/Windows)
│   ├── assembly_file.rs      # EXISTING - AssemblyFile container
│   ├── data_directive.rs     # EXISTING - Data directives (db, dw, dd, dq)
│   ├── instruction/          # EXISTING - x86 instruction definitions
│   ├── platform.rs           # EXISTING - Platform enum
│   ├── register/             # EXISTING - Register definitions (GP, XMM, etc.)
│   ├── section.rs            # EXISTING - Section types
│   │
│   ├── codegen/              # NEW - Code generation pipeline
│   │   ├── mod.rs            # Pipeline orchestration
│   │   ├── context.rs        # Generation context (current function, block, etc.)
│   │   ├── emitter.rs        # Instruction emission to assembly
│   │   ├── error.rs          # Code generation errors
│   │   └── stats.rs          # Generation statistics
│   │
│   ├── regalloc/             # NEW - Register allocation
│   │   ├── mod.rs            # Linear Scan allocator
│   │   ├── liveness.rs       # Liveness analysis
│   │   ├── interval.rs       # Live interval representation
│   │   ├── spill.rs          # Spill slot management
│   │   └── mapping.rs        # IR value → physical register mapping
│   │
│   ├── phi/                  # NEW - Phi node resolution
│   │   ├── mod.rs            # Phi resolution algorithm
│   │   └── parallel_copy.rs  # Parallel copy sequentialization
│   │
│   └── lowering/             # NEW - IR → x86 instruction lowering
│       ├── mod.rs            # Instruction selection
│       ├── arithmetic.rs     # Arithmetic operations
│       ├── memory.rs         # Load/store operations
│       ├── control.rs        # Control flow (jumps, calls)
│       ├── conversion.rs     # Type conversions
│       └── data.rs           # Global data emission
│
├── ir/                       # EXISTING - IR module (input to codegen)
│   ├── module.rs             # IR Module (top-level)
│   ├── function.rs           # IR Function
│   ├── basic_block.rs        # IR BasicBlock
│   ├── instruction.rs        # IR Instructions
│   ├── terminator.rs         # Block terminators
│   ├── types.rs              # IR type system
│   └── value/                # IR values
│
└── cli.rs                    # EXISTING - CLI (add codegen subcommand)

tests/
├── codegen_snapshot_tests.rs      # NEW - Golden file tests
├── codegen_arithmetic_tests.rs    # NEW - Arithmetic lowering tests
├── codegen_platform_tests.rs      # NEW - Platform ABI tests
├── codegen_regalloc_tests.rs      # NEW - Register allocation tests
├── codegen_phi_tests.rs           # NEW - Phi resolution tests
└── codegen_integration_tests.rs   # NEW - End-to-end NASM validation
```

**Structure Decision**: Extend existing `src/asm/` module with new submodules for code generation pipeline. Leverages existing ABI, register, and instruction infrastructure.

## Complexity Tracking

No constitution violations requiring justification.

---

## Phase 0: Research

### Research Tasks

1. **Linear Scan Register Allocation** - Best practices for implementing Linear Scan with liveness intervals in a compiler backend
2. **SSA Phi Resolution** - Algorithms for resolving phi nodes into parallel copies and sequentializing them
3. **x86-64 Instruction Selection** - Patterns for lowering IR operations to x86-64 instructions
4. **NASM Syntax** - Intel syntax requirements and assembler directives for NASM compatibility

### Existing Infrastructure Analysis

The codebase already provides:

| Component                     | Location                    | Purpose                                                     | Reusability   |
| ----------------------------- | --------------------------- | ----------------------------------------------------------- | ------------- |
| `Abi`                         | `src/asm/abi.rs`            | ABI definitions with register lists, red zone, shadow space | ✅ Direct use |
| `Platform`                    | `src/asm/platform.rs`       | Target platform enum                                        | ✅ Direct use |
| `GPRegister64`, `XMMRegister` | `src/asm/register/`         | Physical register definitions                               | ✅ Direct use |
| `Instruction`                 | `src/asm/instruction/`      | x86 instruction representation                              | ✅ Direct use |
| `AssemblyFile`                | `src/asm/assembly_file.rs`  | Assembly output container                                   | ✅ Extend     |
| `AssemblySection`             | `src/asm/section.rs`        | Section management                                          | ✅ Direct use |
| `DataDirective`               | `src/asm/data_directive.rs` | Data directives (db, dw, etc.)                              | ✅ Direct use |
| `ir::Module`                  | `src/ir/module.rs`          | IR module with functions                                    | ✅ Input      |
| `ir::Function`                | `src/ir/function.rs`        | IR function with CFG                                        | ✅ Input      |
| `ir::BasicBlock`              | `src/ir/basic_block.rs`     | Instructions + terminator                                   | ✅ Input      |
| `ir::Instruction`             | `src/ir/instruction.rs`     | IR operations                                               | ✅ Input      |

---

## Phase 1: Design

### Data Model

See [data-model.md](data-model.md) for complete entity definitions.

Key new types:

1. **`CodeGenerator`** - Main pipeline orchestrator
2. **`GenerationContext`** - Per-function state during generation
3. **`LiveInterval`** - Represents liveness range of an IR value
4. **`RegisterMapping`** - Maps IR values to physical registers or spill slots
5. **`SpillSlot`** - Stack location for spilled values
6. **`CodeGenError`** - Error types for code generation failures
7. **`CodeGenStats`** - Statistics about generated code

### API Contracts

See [contracts/](contracts/) for detailed API definitions.

Main entry points:

```rust
/// Generate x86-64 assembly from an IR module
pub fn generate(module: &ir::Module, platform: Platform) -> Result<AssemblyFile, CodeGenError>;

/// Generate with options
pub fn generate_with_options(
    module: &ir::Module,
    options: CodeGenOptions,
) -> Result<AssemblyFile, CodeGenError>;
```

### Pipeline Architecture

```text
┌─────────────┐    ┌──────────────┐    ┌─────────────┐    ┌─────────────┐
│  IR Module  │───▶│   Platform   │───▶│  Liveness   │───▶│  Linear     │
│  (input)    │    │  Detection   │    │  Analysis   │    │  Scan       │
└─────────────┘    └──────────────┘    └─────────────┘    └─────────────┘
                                                                 │
                   ┌──────────────┐    ┌─────────────┐    ┌──────▼──────┐
                   │   Assembly   │◀───│ Instruction │◀───│    Phi      │
                   │   Output     │    │   Emitter   │    │ Resolution  │
                   └──────────────┘    └─────────────┘    └─────────────┘
```

### Quickstart

See [quickstart.md](quickstart.md) for getting started guide.

---

## Phase 2: Implementation Tasks

*To be generated by `/speckit.tasks` command*

### High-Level Task Breakdown

| Priority | Component                  | Description                      | Estimated Effort |
|----------|----------------------------|----------------------------------|------------------|
| P1       | `codegen/context.rs`       | Generation context and state     | Small            |
| P1       | `codegen/error.rs`         | Error types                      | Small            |
| P1       | `lowering/mod.rs`          | Basic instruction selection      | Medium           |
| P1       | `lowering/arithmetic.rs`   | Arithmetic operations            | Medium           |
| P1       | `codegen/emitter.rs`       | Instruction emission             | Medium           |
| P1       | `codegen/mod.rs`           | Pipeline orchestration           | Medium           |
| P2       | `regalloc/liveness.rs`     | Liveness analysis                | Medium           |
| P2       | `regalloc/interval.rs`     | Live interval representation     | Small            |
| P2       | `regalloc/mod.rs`          | Linear Scan allocator            | Large            |
| P2       | `regalloc/spill.rs`        | Spill slot management            | Medium           |
| P2       | `phi/mod.rs`               | Phi resolution                   | Medium           |
| P2       | `phi/parallel_copy.rs`     | Copy sequentialization           | Medium           |
| P2       | `lowering/memory.rs`       | Load/store operations            | Medium           |
| P2       | `lowering/control.rs`      | Control flow                     | Medium           |
| P2       | `lowering/data.rs`         | Global data emission             | Small            |
| P2       | `lowering/conversion.rs`   | Type conversions                 | Medium           |
| P3       | `codegen/stats.rs`         | Generation statistics            | Small            |
| P3       | Debug comments             | Source location preservation     | Small            |
| P3       | Fall-through optimization  | Eliminate unnecessary jumps      | Small            |

### Test Strategy

1. **Unit Tests**: Each module with targeted tests
2. **Snapshot Tests**: Golden file comparison with insta for generated assembly
3. **Integration Tests**: End-to-end NASM assembly validation
4. **Platform Tests**: ABI compliance verification per platform

---

## Dependencies

### Build-time Dependencies (existing)

- `clap` - CLI argument parsing
- `thiserror` - Error derive macros
- `logos` - Lexer (not used by codegen)

### Dev Dependencies (existing)

- `insta` - Snapshot testing
- `criterion` - Benchmarking
- `assert_cmd` - CLI testing

### New Dependencies

None required - implementation uses only Rust standard library.

---

## Risks and Mitigations

|                    Risk                     |  Likelihood  | Impact   |                  Mitigation                    |
| --------------------------------------------| ------------ | -------- | ---------------------------------------------- |
| Complex phi resolution edge cases           | Medium       | Medium   | Comprehensive test suite with cycle detection  |
| Register pressure causing excessive spills  | Low          | Medium   | Implement spill weight heuristics              |
| Platform ABI compliance issues              | Medium       | High     | Golden file tests against known-good output    |
| NASM syntax incompatibilities               | Low          | Medium   | CI validation with actual NASM assembly        |

---

## Success Metrics

- [ ] All 56 functional requirements implemented
- [ ] SC-001: NASM assembles all outputs without errors
- [ ] SC-002: Correct output matching interpreter results
- [ ] SC-003: 1000 functions in <30 seconds
- [ ] SC-004: Platform ABI compliance tests pass
- [ ] >90% code coverage on new modules
- [ ] All insta snapshots approved and stable
