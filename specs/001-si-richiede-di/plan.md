# Implementation Plan: ASM Generation Components Improvement

**Branch**: `001-si-richiede-di` | **Date**: lunedì 22 settembre 2025 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-si-richiede-di/spec.md`

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
This feature aims to improve the structure and functionality of the assembly code generation components in the jsavrs compiler. The primary goal is to enhance readability, modularity, and documentation of the ASM generation code located in the @src/asm directory to improve overall usability and maintainability.

## Technical Context
**Language/Version**: Rust 2024 edition  
**Primary Dependencies**: Standard Rust library, NASM
**Storage**: N/A  
**Testing**: cargo test with insta for snapshot tests  
**Target Platform**: Cross-platform (Windows, macOS, Linux)  
**Project Type**: Single project (compiler)  
**Performance Goals**: Maintain optimal compilation speeds  
**Constraints**: Maintain compatibility with existing NASM x86-64 assembly generation functionality  
**Scale/Scope**: Compiler component affecting assembly generation phase

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Based on the jsavrs Constitution v1.0.0:

1. **Safety First**: The project prioritizes memory safety and thread safety through Rust's ownership model. Our implementation will adhere to these principles by using safe Rust patterns and avoiding unsafe code unless absolutely necessary.

2. **Performance Excellence**: We are committed to achieving optimal compilation speeds. Our refactoring will maintain or improve performance by leveraging Rust's zero-cost abstractions.

3. **Cross-Platform Compatibility**: The compiler operates seamlessly across major operating systems. Our changes will maintain this compatibility.

4. **Modular Extensibility**: The project follows a component-based architecture. Our refactoring will enhance modularity to enable easier modifications and extensions.

5. **Test-Driven Reliability**: We maintain a comprehensive testing methodology. All changes will include appropriate unit tests and snapshot tests using insta.

6. **Code Quality Standards**: All contributions must adhere to Rust community standards, be formatted with `cargo fmt`, pass `cargo clippy` checks, and include comprehensive documentation.

7. **Governance**: All submissions undergo a thorough review process. Our implementation will follow established coding standards and architectural patterns.

## Project Structure

### Documentation (this feature)
```
specs/001-si-richiede-di/
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
└─── asm/                 # ASM generation components (target of this feature)
    ├── generator.rs
    ├── instruction.rs
    ├── mod.rs
    ├── operand.rs
    └── register.rs

tests/
├── snapshots/           # insta snapshot test files
├── asm_generator.rs     # ASM generation tests
└── [other test files]
```

**Structure Decision**: DEFAULT to Option 1 as this is a single project (compiler)

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - No NEEDS CLARIFICATION markers remain
   - Dependencies: Standard Rust library
   - Integration: Part of the jsavrs compiler pipeline

2. **Generate and dispatch research agents**:
   ```bash
   For each unknown in Technical Context:
     Task: "Research current ASM generation implementation in jsavrs"
   For each technology choice:
     Task: "Find best practices for modular Rust code organization"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: Refactor existing ASM generation components for improved modularity
   - Rationale: Current implementation lacks clear separation of concerns
   - Alternatives considered: Complete rewrite vs. refactoring

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - ASM Generator: Component responsible for generating NASM x86-64 assembly code
   - Instructions: Enum representing all supported assembly instructions
   - Registers: Enum representing all x86-64 registers
   - Operands: Enum representing different types of operands
   - Target OS: Enum representing supported operating systems

2. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps


**Output**: data-model.md, failing tests, quickstart.md

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

**Estimated Output**: 40-50 numbered, ordered tasks in tasks.md

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
| N/A | N/A | N/A |

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
*Based on Constitution v1.0.0 - See `/memory/constitution.md`*