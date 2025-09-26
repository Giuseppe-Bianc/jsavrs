
# Implementation Plan: Add IEEE 754 Floating-Point Support

**Branch**: `001-1-add-floating` | **Date**: giovedì 25 settembre 2025 | **Spec**: [specs/001-1-add-floating/spec.md](specs/001-1-add-floating/spec.md)
**Input**: Feature specification from `C:\\dev\\vscode\\rust\\jsavrs\\specs\\001-1-add-floating\\spec.md`

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
Implement comprehensive IEEE 754 floating-point support for the jsavrs compiler system, adding floating-point registers (XMM0-XMM15, YMM0-YMM15, ZMM0-ZMM15), IEEE 754 compliant instructions, operand handling for floating-point values, proper exception handling and rounding control, and Display formatting updates. The implementation will follow an enum-based type system approach to minimize validation errors and ensure type safety, with full compliance to IEEE 754-2008 standards for binary32 and binary64 formats, supporting both Windows x64 ABI and System V ABI calling conventions while maintaining backward compatibility with existing integer-only functionality.

## Technical Context
**Language/Version**: Rust 1.75+ with IEEE 754 compliance libraries  
**Primary Dependencies**: IEEE 754 binary32 (single) and binary64 (double) precision formats, x86-64 SSE/AVX instruction sets, IEEE 754-2008 specification  
**Storage**: N/A (compiler intermediate representation)  
**Testing**: Custom floating-point validation harness with bit-exact IEEE 754 compliance verification, cargo test framework, Insta snapshot testing  
**Target Platform**: x86-64 (Windows, Linux, macOS) with XMM, YMM, and ZMM register support
**Project Type**: single (Rust compiler project)  
**Performance Goals**: IEEE 754-2008 specification compliance with full exception handling and rounding modes, efficient subnormal number handling  
**Constraints**: Maintain full backward compatibility with existing integer-only compiler functionality; support both Windows x64 ABI and System V ABI calling conventions; use enum-based data model throughout to minimize validation errors; ensure subnormal number handling with optional Flush-To-Zero and Denormals-Are-Zero modes  
**Scale/Scope**: Comprehensive IEEE 754 floating-point support for jsavrs compiler system with floating-point registers, instructions, and operations including exception handling and rounding control

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Phase 0 Evaluation**:
- Safety First: ✓ Implementation will leverage Rust's ownership model to ensure memory safety in floating-point operations
- Performance Excellence: ✓ IEEE 754 floating-point operations will be optimized using x86-64 SSE/AVX instruction sets for high-performance compilation
- Cross-Platform Compatibility: ✓ Implementation will support x86-64 architecture across Windows, macOS, and Linux platforms with appropriate ABI handling
- Modular Extensibility: ✓ Floating-point functionality will be implemented as extensions to existing asm module components without disrupting existing architecture
- Test-Driven Reliability: ✓ Comprehensive test suite will verify IEEE 754 compliance with bit-exact validation harness
- Snapshot Validation: ✓ Insta snapshot testing will be used to ensure consistent floating-point instruction output
- Documentation Rigor: ✓ Detailed documentation for research.md, data_model.md, and implementation will follow constitutional standards

**Community Principles**:
- Collaboration First: ✓ Open design process with community input on floating-point implementation
- Respectful Communication: ✓ All discussions will follow the Rust Code of Conduct
- Shared Learning: ✓ Implementation will serve as educational resource for IEEE 754 in Rust
- Quality Through Community: ✓ Peer review process will ensure high-quality implementation
- Transparency and Openness: ✓ All development will happen in public with detailed documentation
- Documentation Rigor: ✓ Thorough documentation of floating-point system design and implementation

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, and Documentation Rigor. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

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

**Structure Decision**: Option 1 - Single project (DEFAULT) as this is a Rust compiler project with the following structure:
```
src/
├── asm/
│   ├── generator.rs     # Enhanced with floating-point code generation
│   ├── instruction.rs   # Extended with IEEE 754 floating-point instructions  
│   ├── operand.rs       # Updated with floating-point operand support
│   └── register.rs      # Extended with XMM/YMM/ZMM register enums
└── [existing files]

tests/
├── snapshots/
└── [existing tests]
```

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

**Specific Tasks for IEEE 754 Floating-Point Implementation**:
- Register enum extension tasks for XMM/YMM/ZMM registers [P]
- Instruction enum implementation for IEEE 754 operations [P]
- Operand system enhancement for floating-point values [P]
- Display formatting updates for floating-point instructions [P]
- Exception handling system implementation [P]
- Rounding mode control implementation [P]
- ABI compliance implementation for floating-point parameters [P]
- Comprehensive test suite development for IEEE 754 compliance [P]

**Estimated Output**: 30-40 numbered, ordered tasks in tasks.md

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

