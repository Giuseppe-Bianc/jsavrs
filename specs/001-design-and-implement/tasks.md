# Tasks: x86-64 Assembly Code Generator

**Input**: Design documents from `/specs/001-design-and-implement/`
**Prerequisites**: plan.md (required), research.md, data-model.md, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Implementation plan loaded: x86-64 assembly generator using iced-x86
   → Extract: Rust 1.75, iced-x86, existing IR modules, NASM output
2. Load optional design documents:
   → data-model.md: Extract entities → AssemblyGenerator, RegisterAllocator, CallingConvention
   → quickstart.md: Extract usage scenarios → integration tests
   → research.md: Extract decisions → iced-x86 integration, ABI implementations
3. Generate tasks by category:
   → Setup: Rust project structure, iced-x86 dependency, linting
   → Tests: unit tests for core components, integration tests for IR translation
   → Core: assembly generator, register allocator, calling conventions
   → Integration: IR module integration, platform ABI implementations
   → Polish: snapshot tests, performance validation, documentation
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → All core components have unit tests
   → All IR constructs have translation tests
   → All platforms have ABI implementations
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths assume single project structure as per plan.md

## Phase 3.1: Setup
- [ ] T001 Create assembly generator module structure in src/asm/
- [ ] T002 Add iced-x86 dependency to Cargo.toml with x86-64 feature flags
- [ ] T003 [P] Configure clippy and rustfmt for assembly generation code quality

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] Unit test for AssemblyGenerator basic IR translation in tests/assembly_generator_tests.rs
- [ ] T005 [P] Unit test for RegisterAllocator round-robin allocation in tests/register_allocator_tests.rs
- [ ] T006 [P] Unit test for CallingConvention Windows x64 ABI in tests/calling_convention_tests.rs
- [ ] T007 [P] Unit test for CallingConvention System V ABI (Linux/macOS) in tests/calling_convention_tests.rs
- [ ] T008 [P] Integration test for simple function IR to assembly in tests/ir_to_assembly_integration_tests.rs
- [ ] T009 [P] Integration test for arithmetic operations translation in tests/arithmetic_translation_tests.rs
- [ ] T010 [P] Integration test for memory operations translation in tests/memory_translation_tests.rs
- [ ] T011 [P] Integration test for control flow translation in tests/control_flow_translation_tests.rs
- [ ] T011.5 [P] Integration test for floating-point SSE/AVX operations translation in tests/floating_point_translation_tests.rs
- [ ] T012 [P] Snapshot test for assembly output consistency in tests/assembly_output_snapshot_tests.rs
- [ ] T012.5 [P] Unit test for assembly output commenting and debugging annotations in tests/assembly_commenting_tests.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T013 [P] AssemblyGenerator struct and basic framework in src/asm/generator.rs
- [ ] T014 [P] RegisterAllocator with round-robin strategy in src/asm/register_allocator.rs
- [ ] T015 [P] Target platform enum and definitions in src/asm/target_platform.rs
- [ ] T016 [P] CallingConvention trait and base implementations in src/asm/calling_convention/mod.rs
- [ ] T017 [P] Windows x64 calling convention implementation in src/asm/calling_convention/windows_x64.rs
- [ ] T018 [P] System V ABI (Linux/macOS) calling convention implementation in src/asm/calling_convention/system_v.rs
- [ ] T019 IR instruction to x86-64 instruction mapping in src/asm/instruction_mapping.rs
- [ ] T020 Function prologue and epilogue generation in src/asm/function_frame.rs
- [ ] T021 Assembly output formatting and NASM syntax in src/asm/output_formatter.rs
- [ ] T021.5 [P] Assembly commenting and debugging annotations system in src/asm/debug_formatter.rs

## Phase 3.4: Integration
- [ ] T022 Integration with existing IR modules from src/ir/ and src/ir/value/
- [ ] T023 Error handling for unsupported IR constructs in src/asm/error.rs
- [ ] T024 Assembly buffer management and memory optimization
- [ ] T025 Platform detection and automatic ABI selection
- [ ] T026 Stack frame management and alignment compliance

## Phase 3.5: Polish
- [ ] T027 [P] Performance tests for 10,000 IR instruction benchmark in tests/performance_tests.rs
- [ ] T028 [P] Memory usage validation tests (≤2x IR size) in tests/memory_usage_tests.rs
- [ ] T029 [P] Create comprehensive documentation following Documentation Rigor principle with detailed, precise, and meticulous explanations covering architecture decisions, implementation details, usage patterns, performance characteristics, and troubleshooting guides in docs/assembly_generation.md
- [ ] T030 [P] Add semantic equivalence validation utilities in src/asm/validation.rs
- [ ] T031 Code cleanup and duplicate removal across assembly generation modules
- [ ] T032 Execute quickstart.md validation scenarios and verify assembly output

## Dependencies
- Setup (T001-T003) before all other phases
- Tests (T004-T012.5) before implementation (T013-T021.5)
- T013 (AssemblyGenerator) blocks T019, T020, T021, T021.5 (depends on generator framework)
- T014 (RegisterAllocator) blocks T024 (stack frame management needs allocator)
- T015 (TargetPlatform) blocks T017, T018 (platform-specific implementations)
- T016 (CallingConvention trait) blocks T017, T018 (concrete implementations)
- Implementation (T013-T021) before integration (T022-T026)
- Integration before polish (T027-T032)

## Parallel Example
```
# Launch T004-T012 together (all test files):
Task: "Unit test for AssemblyGenerator basic IR translation in tests/assembly_generator_tests.rs"
Task: "Unit test for RegisterAllocator round-robin allocation in tests/register_allocator_tests.rs"  
Task: "Unit test for CallingConvention Windows x64 ABI in tests/calling_convention_tests.rs"
Task: "Unit test for CallingConvention System V ABI (Linux/macOS) in tests/calling_convention_tests.rs"
Task: "Integration test for simple function IR to assembly in tests/ir_to_assembly_integration_tests.rs"
Task: "Integration test for arithmetic operations translation in tests/arithmetic_translation_tests.rs"
Task: "Integration test for memory operations translation in tests/memory_translation_tests.rs"
Task: "Integration test for control flow translation in tests/control_flow_translation_tests.rs"
Task: "Integration test for floating-point SSE/AVX operations translation in tests/floating_point_translation_tests.rs"
Task: "Snapshot test for assembly output consistency in tests/assembly_output_snapshot_tests.rs"

# Launch T013-T018 together (independent module files):
Task: "AssemblyGenerator struct and basic framework in src/asm/generator.rs"
Task: "RegisterAllocator with round-robin strategy in src/asm/register_allocator.rs"
Task: "Target platform enum and definitions in src/asm/target_platform.rs"
Task: "CallingConvention trait and base implementations in src/asm/calling_convention/mod.rs"
Task: "Windows x64 calling convention implementation in src/asm/calling_convention/windows_x64.rs"
Task: "System V ABI (Linux/macOS) calling convention implementation in src/asm/calling_convention/system_v.rs"
```

## Notes
- [P] tasks = different files, no dependencies
- Verify tests fail before implementing
- Follow TDD: write failing tests first, then make them pass
- Use insta crate for snapshot testing of assembly output
- Ensure semantic equivalence between IR and generated assembly
- Commit after each task completion
- Focus on correctness over optimization initially

## Task Generation Rules
*Applied during main() execution*

1. **From Data Model**:
   - AssemblyGenerator entity → generator implementation task
   - RegisterAllocator entity → allocator implementation task [P]
   - CallingConvention entity → ABI implementation tasks [P]
   - Error handling entities → error management task

2. **From Quickstart Scenarios**:
   - Basic usage example → integration test [P]
   - Advanced usage examples → integration tests [P]  
   - Error handling examples → error handling tests [P]
   - Floating-point operations → SSE/AVX translation test [P]

3. **From Research Decisions**:
   - iced-x86 integration → dependency setup task
   - NASM output format → output formatter task
   - Platform ABI handling → calling convention tasks

4. **Ordering**:
   - Setup → Tests → Core modules → Integration → Polish
   - Dependencies block parallel execution
   - TDD approach: tests before implementation

5. **Community Integration**:
   - Comprehensive documentation following Documentation Rigor principle
   - Code reviews following respectful communication principles  
   - Tests exemplify shared learning opportunities for x86-64 assembly concepts

## Validation Checklist
*GATE: Checked by main() before returning*

- [x] All core components have corresponding unit tests
- [x] All IR translation scenarios have integration tests
- [x] All tests come before implementation (T004-T012 before T013-T021)
- [x] Parallel tasks are truly independent (different files, no shared state)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Performance and memory requirements addressed (T027, T028)
- [x] Cross-platform compatibility covered (T017, T018, T025)
- [x] Error handling and validation included (T023, T030)