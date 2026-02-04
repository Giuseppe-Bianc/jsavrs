# Implementation Plan: IR to x86-64 Assembly Translator

**Branch**: `023-ir-to-asm-translator` | **Date**: 2026-02-03 | **Spec**: [Link to spec.md](../spec.md)
**Input**: Feature specification from `/specs/023-ir-to-asm-translator/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implementation of a translator module that converts IR structures from `src/ir/` into NASM-compatible x86-64 assembly using existing `src/asm/` infrastructure. The translator follows a strict pipeline: IR Function → Translation Context (with ABI selected from config flag or system default) → Basic Block traversal in reverse post-order → Instruction-by-instruction mapping to `src/asm/instruction::Instruction` enum variants → Final assembly emission via `AssemblyFile::text_sec_add_instruction()`. The module supports both System V AMD64 and Windows x64 ABIs with fail-fast error handling for unsupported IR constructs.

## Technical Context

**Language/Version**: Rust 1.93.0
**Primary Dependencies**: petgraph, thiserror, uuid (existing in codebase), insta = "1.40" (for testing), criterion = "0.5" (for benchmarks)
**Storage**: Files (assembly output, optional mapping files)
**Testing**: cargo test, insta for snapshot testing, criterion for performance benchmarks
**Target Platform**: x86-64 (Windows, Linux, macOS) with support for both System V and Windows x64 ABIs
**Project Type**: Single project library component
**Performance Goals**: <100ms per-function translation with <1GB memory usage, complete full module translation in <30 seconds
**Constraints**: Zero additional dependencies beyond existing codebase, direct integration with existing src/asm infrastructure, fail-fast on unsupported IR constructs
**Scale/Scope**: Individual functions up to complete modules, targeting typical 100-instruction functions

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, and Documentation Rigor. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

**Constitution Compliance Status**: PASS

- Safety First: Implemented through Rust's ownership model and safe code practices
- Performance Excellence: Targets <100ms per function with <1GB memory usage
- Cross-Platform Compatibility: Supports both System V and Windows x64 ABIs
- Modular Extensibility: Component-based architecture with clean interfaces
- Test-Driven Reliability: Comprehensive test suite with unit, integration, and snapshot tests
- Snapshot Validation: Uses insta for assembly output regression testing
- Documentation Rigor: Comprehensive documentation including research.md, data-model.md, and quickstart.md

## Project Structure

### Documentation (this feature)

```text
specs/023-ir-to-asm-translator/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
└── contracts/           # Phase 1 output (/speckit.plan command)
```

### Source Code (repository root)

```text
src/
├── translator/                 # New translator module
│   ├── mod.rs                  # Public API (Translator::translate_module)
│   ├── context.rs              # Translation state management
│   ├── function_translator.rs  # Function-level translation logic
│   ├── block_translator.rs     # Basic block translation logic
│   ├── instruction_translator.rs # Instruction mapping logic
│   ├── terminator_translator.rs # Control flow translation logic
│   └── codegen/
│       └── abi_adapter.rs      # ABI-specific code generation
├── ir/                       # Existing IR module (input to translator)
├── asm/                      # Existing assembly infrastructure (output via)
│   ├── instruction.rs        # Instruction enum used for mapping
│   ├── abi.rs               # ABI abstractions leveraged by translator
│   └── ...
└── lib.rs                   # Public exports including new translator module

tests/
├── translator_basic.rs      # Basic IR to assembly translation tests
├── translator_abi.rs        # ABI-specific behavior tests
├── translator_errors.rs     # Error handling tests
└── snapshots/              # Insta snapshot files for assembly output

benches/
└── jsavrs_benchmark.rs      # Performance benchmarks for translation speed
```

**Structure Decision**: Single project library component that extends the existing jsavrs compiler with a new translator module. This approach maintains consistency with the existing architecture while cleanly separating the translation concerns. The module integrates directly with existing src/ir and src/asm infrastructure without duplicating code.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
| ----------- | ------------ | ------------------------------------- |
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
