# Tasks: x86-64 Assembly Code Generator

**Input**: Design documents from `/specs/001-design-and-implement/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory ✅
   → Tech stack: Rust 1.75+, iced-x86, existing jsavrs IR modules
   → Structure: Single project (compiler component)
2. Load design documents: ✅
   → data-model.md: 9 core entities extracted
   → contracts/: API contracts for assembly generator
   → research.md: Technology decisions and patterns
   → quickstart.md: Test scenarios and validation
3. Generate tasks by category: ✅
   → Setup: Project dependencies, module structure
   → Tests: Contract tests, integration tests, semantic validation
   → Core: Enums, structs, traits, implementations
   → Integration: CLI integration, error handling
   → Polish: Unit tests, performance, memory validation, documentation
4. Task rules applied: ✅
   → Different files marked [P] for parallel execution
   → Same file sequential (no [P])
   → Tests before implementation (TDD)
5. Tasks numbered sequentially T001-T041
6. Dependencies validated ✅
7. Parallel execution examples included ✅
8. Task completeness verified ✅
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Assembly generator in: `src/asm/` module
- Integration with existing: `src/ir/` modules

## Phase 3.1: Setup
- [] T001 Add iced-x86 dependency to Cargo.toml and create src/asm module structure
- [] T002 Create assembly generator module hierarchy in src/asm/{mod.rs, generator.rs, register.rs, instruction.rs, operand.rs}
- [] T003 [P] Configure additional linting rules for assembly generation code quality

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests [P] - Different test files, independent
- [] T004 [P] Contract test AssemblyCodeGenerator trait in tests/test_assembly_generator_trait.rs
- [] T005 [P] Contract test TargetPlatform configuration in tests/test_target_platform.rs
- [] T006 [P] Contract test CodeGenOptions validation in tests/test_codegen_options.rs
- [] T007 [P] Contract test CallingConvention interface in tests/test_calling_convention.rs
- [] T008 [P] Contract test RegisterInfo interface in tests/test_register_info.rs

### Integration Tests [P] - Based on quickstart scenarios
- [] T009 [P] Integration test simple function translation (add_numbers example) in tests/test_simple_function.rs
- [] T010 [P] Integration test memory operations and local variables in tests/test_memory_operations.rs
- [] T011 [P] Integration test cross-platform ABI differences in tests/test_cross_platform_abi.rs
- [] T012 [P] Integration test semantic equivalence validation in tests/test_semantic_equivalence.rs
- [] T013 [P] Integration test performance benchmarking in tests/test_performance_benchmark.rs

### Snapshot Tests [P] - For regression detection
- [] T014 [P] Snapshot test Linux assembly output in tests/test_linux_assembly_snapshots.rs
- [] T015 [P] Snapshot test Windows assembly output in tests/test_windows_assembly_snapshots.rs
- [] T016 [P] Snapshot test macOS assembly output in tests/test_macos_assembly_snapshots.rs

## Phase 3.3: Core Type Definitions (ONLY after tests are failing) [P]
**All enum and struct definitions can run in parallel - different files**

### Register and Instruction Types [P]
- [] T017 [P] GPRegister enum with Display trait in src/asm/register.rs
- [] T018 [P] MMRegister enum with conversion methods in src/asm/register.rs
- [] T019 [P] Register unified enum in src/asm/register.rs
- [] T020 [P] 86Instruction enum with all instruction variants in src/asm/instruction.rs
- [] T021 [P] Operand enum and MemoryOperand struct in src/asm/operand.rs
- [] T022 [P] ImmediateValue enum with type safety in src/asm/operand.rs

### Platform and Configuration Types [P]
- [] T023 [P] TargetPlatform struct and related enums in src/asm/platform.rs
- [] T024 [P] CodeGenOptions struct and InstructionSetFlags in src/asm/options.rs
- [] T025 [P] CodeGenError comprehensive error enum in src/asm/error.rs

## Phase 3.4: Core Trait Implementations (Sequential - same files)
**These modify same files as previous tasks, so must be sequential**

### Calling Convention System
- [] T026 CallingConvention trait definition in src/asm/abi/mod.rs
- [] T027 Windows64ABI implementation in src/asm/abi/windows_x64.rs
- [] T028 SystemVABI implementation in src/asm/abi/system_v.rs

### Register Management
- [] T029 RegisterAllocator struct and allocation logic in src/asm/register.rs
- [] T030 RegisterInfo trait implementation in src/asm/register.rs

## Phase 3.5: Core Generator Implementation (Sequential - interdependent)
**Main assembly generation pipeline - dependencies between components**

### Assembly Generator Core
- [] T031 AssemblyGenerator struct and basic methods in src/asm/generator.rs
- [] T032 IRTranslator struct and IR processing logic in src/asm/generator.rs
- [] T033 Instruction encoding integration with iced-x86 in src/asm/instruction.rs
- [] T034 Function generation (prologue/epilogue/body) in src/asm/generator.rs

## Phase 3.6: Integration Layer
**Connect with existing jsavrs infrastructure**

- [] T035 CLI integration for --emit-asm flag in src/cli.rs
- [] T036 Error handling integration with existing error system in src/error/mod.rs
- [] T037 IR module integration with @src/ir modules in src/asm/generator.rs

## Phase 3.7: Polish and Validation
- [] T038 [P] Unit tests for register allocation algorithms in tests/test_register_allocation.rs
- [] T039 Performance benchmarking with criterion.rs and memory usage validation (≤2x IR size constraint) in benches/assembly_generation_bench.rs
- [] T040 [P] Memory profiling integration and constraint validation in tests/test_memory_constraints.rs
- [] T041 [P] Documentation updates and code examples in src/asm/mod.rs

## Dependencies
**Critical ordering constraints:**

### Phase Dependencies
- Setup (T001-T003) before everything
- All tests (T004-T016) before any implementation (T017+)
- Core types (T017-T025) before trait implementations (T026-T030)
- Traits (T026-T030) before generator implementation (T031-T034)
- Core implementation (T031-T034) before integration (T035-T037)
- Everything before polish (T038-T041)

### Specific Dependencies
- T017-T019 (register types) before T029-T030 (register allocation)
- T020-T022 (instruction/operand types) before T033 (instruction encoding)
- T023-T025 (platform/error types) before T031 (assembly generator)
- T026-T028 (calling conventions) before T034 (function generation)
- T031-T034 (generator core) before T035-T037 (integration)

## Parallel Execution Examples

### Contract Tests (Can run simultaneously)
```bash
# Launch T004-T008 together:
Task: "Contract test AssemblyCodeGenerator trait in tests/test_assembly_generator_trait.rs"
Task: "Contract test TargetPlatform configuration in tests/test_target_platform.rs"
Task: "Contract test CodeGenOptions validation in tests/test_codegen_options.rs"
Task: "Contract test CallingConvention interface in tests/test_calling_convention.rs"
Task: "Contract test RegisterInfo interface in tests/test_register_info.rs"
```

### Integration Tests (Can run simultaneously)
```bash
# Launch T009-T013 together:
Task: "Integration test simple function translation in tests/test_simple_function.rs"
Task: "Integration test memory operations and local variables in tests/test_memory_operations.rs"
Task: "Integration test cross-platform ABI differences in tests/test_cross_platform_abi.rs"
Task: "Integration test semantic equivalence validation in tests/test_semantic_equivalence.rs"
Task: "Integration test performance benchmarking in tests/test_performance_benchmark.rs"
```

### Core Type Definitions (Can run simultaneously)
```bash
# Launch T017-T025 together (different files):
Task: "GPRegister enum with Display trait in src/asm/register.rs"
Task: "X86Instruction enum with all instruction variants in src/asm/instruction.rs"
Task: "Operand enum and MemoryOperand struct in src/asm/operand.rs"
Task: "TargetPlatform struct and related enums in src/asm/platform.rs"
Task: "CodeGenOptions struct and InstructionSetFlags in src/asm/options.rs"
Task: "CodeGenError comprehensive error enum in src/asm/error.rs"
```

## Task Validation Rules Applied

### From Contracts (5 contract interfaces)
✅ T004-T008: Each contract interface → contract test [P]
✅ T026-T030: Each interface → implementation task

### From Data Model (9 core entities)
✅ T017-T025: Each entity → type definition [P]
✅ T029-T034: Relationships → service layer tasks

### From User Stories (Quickstart scenarios)
✅ T009-T013: Each scenario → integration test [P]
✅ T038-T041: Validation tasks → polish phase

### TDD Ordering
✅ All tests (T004-T016) before implementation (T017+)
✅ Dependencies properly sequenced
✅ Parallel tasks use different files

## Notes
- [P] tasks = different files, no dependencies, can run in parallel
- Verify all tests fail before implementing (TDD requirement)
- Commit after each task for incremental progress
- Use insta for snapshot testing as specified in constitution
- Follow Documentation Rigor principle with comprehensive comments
- Maintain Safety First through Rust type system and iced-x86 integration

## Constitutional Compliance Checklist
✅ **Safety First**: Type-safe enums, iced-x86 integration, Rust ownership model
✅ **Performance Excellence**: Benchmarking tasks, performance and memory constraints validation (T039, T040)
✅ **Cross-Platform Compatibility**: Multi-platform ABI testing and implementation
✅ **Modular Extensibility**: Trait-based architecture, separate modules
✅ **Test-Driven Reliability**: Comprehensive test-first approach
✅ **Snapshot Validation**: insta integration for regression testing
✅ **Documentation Rigor**: Code documentation and usage examples included