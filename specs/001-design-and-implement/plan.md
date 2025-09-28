
# Implementation Plan: x86-64 Assembly Code Generator

**Branch**: `001-design-and-implement` | **Date**: 2025-09-28 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-design-and-implement/spec.md`

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
Design and implement an x86-64 assembly code generator that translates intermediate representation (IR) from jsavrs internal modules (@src/ir and @src/ir/value) into NASM-compatible assembly code. The implementation leverages the iced-x86 library for precise instruction encoding, uses enum-based registers for type safety, supports cross-platform ABIs (Windows x64, System V), and maintains semantic equivalence between IR and generated assembly through comprehensive validation testing.

## Technical Context
**Language/Version**: Rust 1.75+  
**Primary Dependencies**: iced-x86 (instruction encoding), existing jsavrs IR modules (@src/ir, @src/ir/value)  
**Storage**: N/A (code generator processes in-memory IR)  
**Testing**: cargo test with insta for snapshot testing, custom test harness for semantic equivalence validation  
**Target Platform**: Cross-platform (Windows, Linux, macOS) x86-64 architecture
**Project Type**: single (compiler component within existing jsavrs architecture)  
**Performance Goals**: Assembly generation within 5 seconds for modules up to 10,000 IR instructions  
**Constraints**: Memory usage ≤ 2x input IR file size, semantic equivalence preservation, ABI compliance (Windows x64, System V)  
**Scale/Scope**: Support for basic arithmetic, memory operations, simple control flow; extensible architecture for future IR constructs

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Alignment with Core Principles

✅ **Safety First**: Leverages Rust's memory safety and ownership model, uses iced-x86 for safe instruction encoding, enum-based registers prevent type errors
✅ **Performance Excellence**: Targets 5-second generation for 10K IR instructions, optimized memory usage (≤2x input size), uses zero-cost abstractions
✅ **Cross-Platform Compatibility**: Supports Windows x64 ABI and System V ABI, generates NASM-compatible output for all major platforms
✅ **Modular Extensibility**: Trait-based architecture enables extensible calling conventions, separate modules for registers/operands/instructions/generator
✅ **Test-Driven Reliability**: Comprehensive test coverage including unit tests, integration tests, semantic equivalence validation, snapshot testing with insta
✅ **Snapshot Validation**: Uses insta library for consistent assembly output validation and regression detection
✅ **Documentation Rigor**: Will create detailed research.md and data-model.md with comprehensive technical analysis and design documentation

**Constitution Status**: PASS - No violations detected. All principles align with the assembly generator architecture.

## Project Structure

### Documentation (this feature)
```
specs/001-design-and-implement/
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

**Structure Decision**: Option 1 (Single project) - This is a compiler component extending the existing jsavrs architecture

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task in a detailed, precise, meticulous, and in-depth way
   - For each dependency → best practices task in a detailed, precise, meticulous, and in-depth way
   - For each integration → patterns task in a detailed, precise, meticulous, and in-depth way

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context} in a detailed, precise, meticulous, and in-depth way"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain} in a detailed, precise, meticulous, and in-depth way"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen] in a detailed, precise, meticulous, and in-depth way
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships in a detailed, precise, meticulous, and in-depth way
   - Validation rules from requirements
   - State transitions if applicable
   - use enums if possible to reduce errors from strings

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/powershell/update-agent-context.ps1 -AgentType qwen`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (data-model.md, contracts/, quickstart.md)
- Each entity in data model → struct/enum definition task [P]
- Each API contract → trait definition and implementation task [P]
- Each error type → error handling implementation task [P]
- Each register/instruction enum → type definition task [P]
- Each calling convention → ABI implementation task
- Main generator pipeline → integration task
- IR translation logic → core generation task
- Each quickstart example → integration test task
- Semantic equivalence validation → testing framework task
- Performance benchmarking → benchmark suite task

**Ordering Strategy**:
- TDD order: Tests and contracts before implementations
- Dependency order: Core types → Traits → Implementations → Integration → Tests
- Mark [P] for parallel execution (independent enum/struct definitions)
- Sequential order: Basic types → Register management → Instruction encoding → Code generation → Testing

**Task Categories**:
1. **Core Type Definitions** [P]: Register enums, instruction enums, operand types
2. **API Contract Implementation**: Traits and interfaces from contracts/
3. **Core Logic Implementation**: Assembly generator, register allocator, IR translator  
4. **ABI Implementation**: Windows x64 and System V calling conventions
5. **Integration Layer**: CLI integration, error handling, output formatting
6. **Testing Infrastructure**: Unit tests, integration tests, semantic validation, benchmarks
7. **Documentation**: Code comments, example programs, usage guides

**Estimated Output**: 35-40 numbered, ordered tasks in tasks.md covering complete implementation from core types through comprehensive testing

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
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
