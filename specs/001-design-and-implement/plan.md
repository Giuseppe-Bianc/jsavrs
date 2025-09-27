

```


# Implementation Plan: x86-64 Assembly Code Generator

**Branch**: `001-design-and-implement` | **Date**: 2025-09-27 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `C:\\dev\\vscode\\rust\\jsavrs\\specs\\001-design-and-implement\\spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR \"No feature spec at {path}\"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR \"Simplify approach first\"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR \"Resolve unknowns\"
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
Design and implement an x86-64 assembly code generator to translate intermediate representation (IR) defined in the internal modules @src/ir and @src/ir/value into NASM-compatible assembly code. The implementation must leverage the iced-x86 crate to ensure correct x86-64 instruction encoding while preserving program semantics, implementing proper calling conventions, and generating function prologues/epilogues across Windows, Linux, and macOS platforms.

## Technical Context
**Language/Version**: Rust 1.75  
**Primary Dependencies**: iced-x86, existing jsavrs IR modules (@src/ir and @src/ir/value)  
**Storage**: N/A (in-memory IR translation to assembly output)  
**Testing**: cargo test, insta snapshot testing  
**Target Platform**: x86-64 architecture (Windows, Linux, macOS) for NASM assembler
**Project Type**: single (compiler code generation component)  
**Performance Goals**: Under 5 seconds for modules up to 10,000 IR instructions, memory usage ≤ 2x input IR size
**Constraints**: Semantics preservation, calling convention compliance, register allocation with stack overflow, cross-platform compatibility
**Scale/Scope**: Support basic arithmetic (add/sub/mul), memory operations (load/store), simple control flow, floating-point operations via SSE/AVX

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Alignment with Core Principles

**Safety First**: The assembly code generator will leverage Rust's ownership model to ensure memory safety during IR-to-assembly translation. Using the iced-x86 crate provides safe x86-64 instruction encoding without requiring unsafe code blocks for basic operation.

**Performance Excellence**: Implementation will focus on efficient IR processing with linear time complexity for translation, meeting the 5-second performance requirement for modules with up to 10,000 IR instructions. Memory usage will be optimized to stay within 2x input IR size constraints.

**Cross-Platform Compatibility**: The generator will produce NASM-compatible assembly for x86-64 architecture across Windows, Linux, and macOS, with platform-specific calling convention support (Windows x64 ABI, System V ABI for Linux/macOS).

**Modular Extensibility**: The assembly generator will be designed as a separate module that integrates with existing IR modules (@src/ir and @src/ir/value), maintaining clean separation of concerns and allowing future enhancements to target architectures.

**Test-Driven Reliability**: Implementation will include comprehensive unit tests, integration tests, and snapshot tests using the insta crate to ensure consistent assembly output and catch regressions.

**Snapshot Validation**: Assembly output will be validated using snapshot testing to ensure semantic equivalence between IR and generated assembly across different test cases.

**Documentation Rigor**: The implementation will include comprehensive documentation in research.md, data_model.md, and quickstart.md files, detailing technical decisions, data structures, and usage instructions.

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
# Option 1: Single project (DEFAULT) - Assembly Generator Structure
src/
├── asm/                 # Assembly generation components
│   ├── generator.rs     # Main assembly generator
│   ├── register_allocator.rs
│   ├── target_platform.rs
│   ├── instruction_mapping.rs
│   ├── function_frame.rs
│   ├── output_formatter.rs
│   ├── error.rs
│   ├── validation.rs
│   └── calling_convention/
│       ├── mod.rs
│       ├── windows_x64.rs
│       └── system_v.rs
├── ir/                  # Existing IR modules (integration target)
├── lexer.rs
├── parser/
└── lib.rs

tests/
├── assembly_generator_tests.rs
├── register_allocator_tests.rs
├── calling_convention_tests.rs
├── ir_to_assembly_integration_tests.rs
├── arithmetic_translation_tests.rs
├── memory_translation_tests.rs
├── control_flow_translation_tests.rs
├── floating_point_translation_tests.rs
├── assembly_output_snapshot_tests.rs
├── performance_tests.rs
├── memory_usage_tests.rs
└── integration/

# Option 2: Web application (when \"frontend\" + \"backend\" detected)
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

# Option 3: Mobile + API (when \"iOS/Android\" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure]
```

**Structure Decision**: DEFAULT to Option 1 (Single project) with assembly generator-specific structure under `src/asm/` to integrate cleanly with the existing jsavrs compiler codebase while maintaining clear separation of assembly generation concerns

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: \"Research {unknown} for {feature context}\"
   For each technology choice:
     Task: \"Find best practices for {tech} in {domain}\"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

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
- Generate tasks from Phase 1 design docs (data-model.md, quickstart.md)
- Each entity in data-model.md → implementation task [P]
- Each component interface → unit test task [P]
- Each user scenario in quickstart.md → integration test task
- Implementation tasks to make tests pass
- Focus on iced-x86 integration, IR translation, register allocation, and ABI compliance

**Ordering Strategy**:
- TDD order: Unit tests before implementation
- Dependency order: IR parsing → register allocation → instruction selection → assembly emission
- Mark [P] for parallel execution (independent components like register allocator can be developed separately)
- Core translation engine first, then platform-specific ABI implementations

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

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
- [ ] Complexity deviations documented

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*



