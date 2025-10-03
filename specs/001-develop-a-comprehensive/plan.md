
# Implementation Plan: Comprehensive x86-64 ABI Trait System

**Branch**: `001-develop-a-comprehensive` | **Date**: October 2, 2025 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `C:\dev\vscode\rust\jsavrs\specs\001-develop-a-comprehensive\spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary

Develop a comprehensive trait-based system that encapsulates all Application Binary Interface (ABI) specifications for x86-64 assembly generation across Windows, Linux, and macOS platforms. The system provides authoritative calling conventions, register allocation rules, stack management requirements, and parameter passing mechanisms through a type-safe, high-performance interface with negligible overhead (< 0.1% compilation time). All implementations leverage constant-time lookups via static tables and defer to reference compiler behavior (GCC/Clang/MSVC) for complex edge cases.

## Technical Context
**Language/Version**: Rust 1.75+ (stable toolchain)  
**Primary Dependencies**: None (stdlib only for core ABI implementation), tracing 0.1 (logging), criterion 0.5 (benchmarking), insta 1.x (snapshot testing)  
**Storage**: N/A (pure computation, no persistent storage)  
**Testing**: cargo test (unit/integration), cargo bench (performance), insta (snapshot validation)  
**Target Platform**: Windows x64, Linux x86-64, macOS x86-64 (pure 64-bit, no mixed-mode)
**Project Type**: Single project (compiler internal library - Option 1 structure)  
**Performance Goals**: ABI queries < 0.1% of total compilation time, < 10 nanoseconds per query (median)  
**Constraints**: Zero-cost abstraction required, no heap allocations for core queries, compile-time type safety enforced  
**Scale/Scope**: 3 platforms (Windows/Linux/macOS), 2 ABI variants (SystemV/Windows), 16 GP registers + 16 XMM registers + extended ISA support

**User-Provided Context**: "All required software components are located in the 'src/asm' directory."

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, and Documentation Rigor. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

### Initial Check (Pre-Research): ✅ PASS

- ✅ **Safety First**: Type system prevents invalid ABI queries at compile time via traits and phantom types
- ✅ **Performance Excellence**: < 0.1% compilation overhead via constant-time lookups, zero-cost abstractions
- ✅ **Cross-Platform Compatibility**: Explicit support for Windows, Linux, macOS with consistent interface
- ✅ **Modular Extensibility**: Trait-based architecture enables new platforms without modifying existing code
- ✅ **Test-Driven Reliability**: Comprehensive unit tests, integration tests with reference compilers, snapshot testing
- ✅ **Snapshot Validation**: Insta library integration for output consistency and regression detection
- ✅ **Documentation Rigor**: Detailed research.md, data-model.md, contracts, quickstart.md with AI-enhanced completeness

### Post-Design Check: ✅ PASS

No new violations introduced during design phase. All architectural decisions align with constitutional principles:
- Trait-based design maintains modular extensibility
- Static dispatch ensures zero-cost abstraction
- Comprehensive contract tests ensure test-driven reliability
- Platform-specific implementations maintain cross-platform compatibility

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure]
```

**Structure Decision**: Option 1 (Single project) - Compiler internal library, no web/mobile components

## Phase 0: Outline & Research ✅ COMPLETE

### Research Tasks Completed

1. **ABI Specification Sources**: Analyzed SystemV ABI, Microsoft x64 calling convention, Intel/AMD manuals
2. **Reference Compiler Behavior**: Documented GCC, Clang, MSVC parameter passing, structure classification, red zone usage
3. **Architectural Decisions**: Selected trait-based design over enum dispatch for type safety and performance
4. **Performance Optimization**: Designed constant-time lookup tables, inlining strategies, cache-friendly layouts
5. **Testing Strategies**: Defined unit testing, snapshot testing, cross-compiler validation, performance benchmarking
6. **Logging/Observability**: Specified tracing-based comprehensive logging for ABI decision debugging
7. **Risk Analysis**: Identified and mitigated risks (ABI ambiguities, performance targets, type system complexity)

### Clarifications Resolved

All NEEDS CLARIFICATION items from Technical Context resolved via clarifications session:
- ✅ Performance target: < 0.1% compilation time
- ✅ Vector type handling: Match reference compiler behavior
- ✅ Nested structure alignment: Defer to GCC/Clang/MSVC layout
- ✅ Red zone specification: Query interface for availability and size
- ✅ Observability: Comprehensive logging for compiler debugging

**Output**: `research.md` (13 sections, 14,000+ words) - Complete architectural foundation

## Phase 1: Design & Contracts ✅ COMPLETE

### Entities Extracted → data-model.md

Documented 9 core entity categories with 50+ specific types:
1. **Platform Entities**: Platform enum, Abi enum with mapping functions
2. **Register Entities**: GPRegister64/32/16/8, XMM/YMM/ZMM, FPU, MMX, Mask, Segment, Control, Debug, Flags, InstructionPointer
3. **Register Classifications**: Volatility rules (Windows vs SystemV), Parameter mappings, Return value designations
4. **Immediate Values**: Imm8/16/32/64 (signed/unsigned) with conversion methods
5. **Memory Operands**: Base + Index + Scale + Displacement addressing
6. **Instruction Operands**: Unified Register/Immediate/Memory/Label taxonomy
7. **Instructions**: MOV, arithmetic, logical, control flow, SSE/AVX operations
8. **Assembly Sections**: Text/Data/BSS/Rodata with DataDirective and AssemblyElement
9. **ABI Trait Contracts**: CallingConvention, StackManagement, RegisterAllocation, AggregateClassification

**Output**: `data-model.md` (13 sections, 30+ pages) - Complete entity specifications

### API Contracts Generated → contracts/

Created 4 trait definition contracts:
1. **calling_convention_trait.md**: Parameter register allocation, platform mapping, index space handling
2. **stack_management_trait.md**: Red zone queries, shadow space requirements, alignment rules
3. **register_allocation_trait.md**: Volatility checks, priority orderings, callee-saved rules
4. **aggregate_classification_trait.md**: Structure/union parameter passing classification

Each contract includes:
- Complete trait definition with method signatures and documentation
- Reference implementations for WindowsX64 and SystemV
- Comprehensive test suites (15-20 tests per contract)
- Performance contracts and benchmarking requirements
- Error handling specifications

**Output**: 4 contract files in `contracts/` directory

### Test Scenarios Extracted → quickstart.md

Created comprehensive usage guide with:
- 3 basic usage patterns (parameter queries, volatility checks, stack management)
- 4 common scenarios (function prologue generation, register allocation, temp selection, structure returns)
- 2 platform-specific examples (Windows add5, System V compute)
- Verification workflow against reference compilers (GCC/Clang/MSVC)
- Performance validation benchmarks
- Common pitfalls and solutions

**Output**: `quickstart.md` (250+ lines) - Practical implementation guide

### Agent Context Update

QWEN.md updated with new ABI trait system context (note: update-agent-context.ps1 completed)

## Phase 2: Task Generation ✅ COMPLETE

**Task Generation Execution**:
1. ✅ Loaded `.specify/templates/tasks-template.md` as base template
2. ✅ Generated tasks from Phase 1 design documents:
   - **From contracts/**: 4 contract test tasks (T004-T007) + 4 trait implementation tasks (T009-T012)
   - **From data-model.md**: Existing entity integration task (T013 for src/asm/register.rs)
   - **From quickstart.md**: Integration test tasks (T008, T016), cross-compiler validation (T014)

**Generated Task Categories**:

**Category 1: Setup & Configuration** [T001-T003]
- Add dependencies to Cargo.toml (tracing, criterion, insta)
- Configure Criterion benchmarking infrastructure
- Verify rustfmt and clippy configuration

**Category 2: Contract Tests (TDD)** [T004-T007] [Priority: High, All Parallel]
- T004: CallingConvention contract tests (tests/abi_calling_convention_tests.rs)
- T005: StackManagement contract tests (tests/abi_stack_management_tests.rs)
- T006: RegisterAllocation contract tests (tests/abi_register_allocation_tests.rs)
- T007: AggregateClassification contract tests (tests/abi_aggregate_classification_tests.rs)

**Category 3: Integration Tests (TDD)** [T008] [Priority: High, Parallel with Contract Tests]
- T008: Quickstart scenario integration tests (tests/abi_integration_tests.rs)

**Category 4: Trait Implementations** [T009-T013] [Priority: High, After Tests Fail]
- T009: CallingConvention trait (src/asm/calling_convention.rs)
- T010: StackManagement trait (src/asm/stack_management.rs)
- T011: RegisterAllocation trait (src/asm/register_allocation.rs)
- T012: AggregateClassification trait (src/asm/aggregate_classification.rs)
- T013: Update existing register.rs (src/asm/register.rs)

**Category 5: Validation** [T014-T016] [Priority: Medium]
- T014: Cross-compiler validation (tests/abi_cross_compiler_validation.rs)
- T015: Snapshot tests with insta (tests/abi_snapshot_tests.rs)
- T016: Quickstart examples validation (tests/abi_quickstart_validation.rs)

**Category 6: Performance & Observability** [T017-T018] [Priority: Medium]
- T017: Criterion benchmarks (benches/abi_benchmarks.rs)
- T018: Tracing instrumentation (src/asm/*.rs)

**Category 7: Documentation & Polish** [T019-T020] [Priority: Low, Parallel]
- T019: Rustdoc documentation (all public APIs)
- T020: Duplication analysis with similarity-rs

**Task Ordering Applied**:
1. **Phase 3.1 (Setup)**: T001-T003 (sequential setup tasks)
2. **Phase 3.2 (TDD Tests)**: T004-T008 (5 parallel test tasks - MUST FAIL initially)
3. **Phase 3.3 (Implementation)**: T009-T013 (4 parallel + 1 sequential - AFTER tests fail)
4. **Phase 3.4 (Validation)**: T014-T016 (T014 first, then T015-T016 parallel)
5. **Phase 3.5 (Polish)**: T017-T020 (T017-T018 sequential, T019-T020 parallel)

**Dependency Rules Applied**:
- Contract tests (T004-T008) have NO dependencies → Can execute immediately in parallel
- Trait implementations (T009-T012) require tests to fail first → Parallel after T004-T008
- T013 modifies shared file → Sequential after T009-T012
- Validation tests (T014-T016) require implementations → After T009-T013
- Benchmarks/observability (T017-T018) require implementations → After T009-T013
- Documentation/analysis (T019-T020) can run anytime → Parallel at end

**Output**: `tasks.md` (20 tasks, 5 phases, clear dependencies, parallel execution guidance)
- All tasks in same phase can run in parallel [P]

**Estimated Output**: 17 numbered tasks in tasks.md with clear dependencies and parallel execution markers

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
|  |  |  |
|  |  |  |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task generation complete (/tasks command) - tasks.md created with 20 tasks
- [ ] Phase 3: Implementation pending - Execute tasks T001-T020
- [ ] Phase 4: Testing pending - Validate all test suites pass
- [ ] Phase 5: Documentation pending - Generate final documentation

**Gate Status**:
- [x] Initial Constitution Check: PASS (7/7 principles verified)
- [x] Post-Design Constitution Check: PASS (no new violations)
- [x] Clarification Gate: PASS (all NEEDS CLARIFICATION resolved)
- [x] Task Generation Gate: PASS (tasks.md created, 20 tasks, all validation criteria met)
- [ ] Implementation Gate: Pending (all tests pass + benchmarks meet < 0.1% target)
- [ ] Final Review Gate: Pending (documentation complete + cross-compiler validation)
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented: NONE (no violations)

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
