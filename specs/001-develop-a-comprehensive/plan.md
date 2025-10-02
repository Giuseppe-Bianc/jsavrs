
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

1. **ABI Specification Sources**: Analyzed System V AMD64 ABI, Microsoft x64 calling convention, Intel/AMD manuals
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

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
1. Load `.specify/templates/tasks-template.md` as base template
2. Generate tasks from Phase 1 design documents:
   - **From contracts/**: Trait implementation tasks + contract test tasks
   - **From data-model.md**: Entity implementation tasks (if needed beyond existing code)
   - **From quickstart.md**: Integration test tasks, example validation tasks

**Specific Task Categories**:

**Category 1: Trait Implementations** [Priority: High]
- Implement `CallingConvention` trait for WindowsX64 (1 task)
- Implement `CallingConvention` trait for SystemV (1 task)
- Implement `StackManagement` trait for both platforms (1 task)
- Implement `RegisterAllocation` trait for both platforms (1 task)
- Implement `AggregateClassification` trait for both platforms (1 task)

**Category 2: Contract Tests** [Priority: High, TDD]
- Write contract tests for CallingConvention (failing initially) (1 task)
- Write contract tests for StackManagement (failing initially) (1 task)
- Write contract tests for RegisterAllocation (failing initially) (1 task)
- Write contract tests for AggregateClassification (failing initially) (1 task)

**Category 3: Integration Tests** [Priority: Medium]
- Cross-compiler validation tests (GCC/Clang/MSVC comparison) (1 task)
- Snapshot tests for assembly output (1 task)
- Quickstart examples validation (1 task)

**Category 4: Performance Benchmarks** [Priority: Medium]
- Implement Criterion benchmarks for ABI queries (1 task)
- Validate < 0.1% compilation time overhead (1 task)

**Category 5: Logging/Observability** [Priority: Low]
- Integrate tracing instrumentation for ABI decisions (1 task)
- Add structured logging configuration (1 task)

**Category 6: Documentation** [Priority: Low]
- Generate rustdoc comments for all public APIs (1 task)
- Update existing register.rs with trait implementations (1 task)

**Ordering Strategy**:
1. **Phase A (TDD Setup)**: Contract tests first (tasks 6-9) - Mark [P] for parallel
2. **Phase B (Implementation)**: Trait implementations (tasks 1-5) following TDD - Mark [P] for parallel
3. **Phase C (Validation)**: Integration tests, benchmarks (tasks 10-13)
4. **Phase D (Polish)**: Logging, documentation (tasks 14-17)

**Dependency Rules**:
- Contract tests have NO dependencies (can write immediately)
- Trait implementations depend on contract tests (TDD workflow)
- Integration tests depend on trait implementations
- Benchmarks depend on trait implementations
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
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command) - READY FOR EXECUTION
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented: NONE (no violations)

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
