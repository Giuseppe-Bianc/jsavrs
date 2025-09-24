# Implementation Plan: Add Comprehensive Tests for Assembly Module

**Branch**: `001-aggiungi-test-per` | **Date**: martedì 23 settembre 2025 | **Spec**: [specs/001-aggiungi-test-per/spec.md](specs/001-aggiungi-test-per/spec.md)
**Input**: Feature specification from `C:\\dev\\vscode\\rust\\jsavrs\\specs\\001-aggiungi-test-per\\spec.md`

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
This feature implements comprehensive tests for the assembly module in the jsavrs compiler, specifically targeting components in the `src/asm/` directory. The implementation will achieve 100% statement coverage across all assembly module components including registers, operands, instructions, and code generators for multiple target operating systems. The tests will include detailed edge cases, corner cases, and cross-platform consistency validation using Rust's testing framework with additional tools like insta for snapshot testing.

## Technical Context
**Language/Version**: Rust 1.75+  
**Primary Dependencies**: cargo test (Rust's built-in testing framework), insta (snapshot testing), tarpaulin (code coverage)  
**Storage**: N/A (testing framework, no persistent storage)  
**Testing**: cargo test with 100% statement coverage requirement, snapshot testing using insta, cross-platform mocking for OS-specific functionality  
**Target Platform**: x86-64 architecture supporting multiple operating systems (Linux, Windows, macOS)
**Project Type**: Single project (compiler)  
**Performance Goals**: Test suite execution under 1 minute on standard development hardware  
**Constraints**: 100% statement coverage for all assembly module components, cross-platform compatibility, modular test organization  
**Scale/Scope**: Comprehensive test coverage for all assembly module components (registers, operands, instructions, generators)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, and Snapshot Validation. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

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

**Structure Decision**: DEFAULT Option 1 (Single project) - The jsavrs compiler is a single Rust project with a testing focus for the assembly module components.

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
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

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
- [x] Phase 0: Research complete (/plan command) - research.md created with technology decisions, alternatives, and test strategies
- [x] Phase 1: Design complete (/plan command) - data-model.md created with entity definitions and test scenarios
- [x] Phase 2: Task planning complete (/plan command - describe approach only) - Task generation strategy defined in plan template
- [x] Phase 3: Tasks generated (/tasks command) - tasks.md created with 26 implementation tasks
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS (All principles satisfied: Safety First through Rust's guarantees, Performance Excellence with 1-minute execution target, Cross-Platform Compatibility across OSes, Modular Extensibility with modular tests, Test-Driven Reliability with 100% coverage requirement, Snapshot Validation with insta)
- [x] Post-Design Constitution Check: PASS (Design aligns with all constitutional principles; data model and test strategies support safety, performance, compatibility, extensibility, and reliability goals)
- [x] All NEEDS CLARIFICATION resolved (Spec has comprehensive clarifications section from Session 2025-09-23)
- [ ] Complexity deviations documented

---
*Based on Constitution v1.3.0 - See `/memory/constitution.md`*



