
# Implementation Plan: Assembly Code Generator x86-64

**Branch**: `001-progettare-e-implementare` | **Date**: 26 settembre 2025 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-progettare-e-implementare/spec.md`

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
Develop an assembly code generator targeting the x86-64 (64 bits) architecture for the jsavrs compiler framework. This generator will translate intermediate representations (IR) from `@src/ir` and `@src/ir/value` modules into NASM-compatible assembly code using the iced-x86 library. The implementation must support both System V ABI and Microsoft x64 calling conventions, provide configurable debug information levels, handle error conditions gracefully, and apply basic optimizations while maintaining semantic preservation and cross-platform compatibility.

## Technical Context
**Language/Version**: Rust 1.75+ leveraging safety, performance, and modern tooling for system-level development  
**Primary Dependencies**: iced-x86 (x86 instruction encoding/emission), thiserror (error handling), insta (snapshot testing)  
**Storage**: Generated assembly files (.asm), intermediate files, debug information sections  
**Testing**: cargo test (unit/integration tests), insta (snapshot testing), criterion (benchmarking)  
**Target Platform**: Cross-platform (Windows, macOS, Linux) generating x86-64 assembly for NASM assembler
**Project Type**: single - compiler infrastructure component  
**Performance Goals**: Balance compilation speed with code quality, <15% overhead for debug information generation  
**Constraints**: NASM syntax compatibility, ABI compliance (System V + Microsoft x64), semantic preservation  
**Scale/Scope**: Handle complex IR modules with 1M+ nodes, concurrent processing for independent modules

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All implementations must follow the Core Principles of Safety First, Performance Excellence, Cross-Platform Compatibility, Modular Extensibility, Test-Driven Reliability, Snapshot Validation, and Documentation Rigor. Additionally, all community interactions must follow the Core Principles of Collaboration First, Respectful Communication, Shared Learning, Quality Through Community, and Transparency and Openness.

**Constitution Compliance Assessment:**

✅ **Safety First**: Implementation leverages Rust's ownership model and memory safety guarantees. The iced-x86 library provides safe instruction encoding without manual memory management risks.

✅ **Performance Excellence**: Design emphasizes efficient compilation with balanced optimization approach, concurrent module processing, and minimal overhead for debug information generation.

✅ **Cross-Platform Compatibility**: Explicit support for Windows, macOS, and Linux with dual ABI support (System V + Microsoft x64) ensures seamless cross-platform operation.

✅ **Modular Extensibility**: Component-based architecture with trait-driven instruction mapping enables extension for new IR constructs and optimization passes without core system modifications.

✅ **Test-Driven Reliability**: Comprehensive testing strategy with unit tests, integration tests, snapshot testing via insta, and benchmarking via criterion ensures correctness across diverse scenarios.

✅ **Snapshot Validation**: Extensive use of insta library for validating assembly output, error messages, and debug information generation across all compiler phases.

✅ **Documentation Rigor**: Commitment to creating detailed research.md and data-model.md files with comprehensive documentation of all architectural decisions and implementation approaches.

**Initial Assessment: PASS** - No constitutional violations identified. All core principles are properly addressed in the feature design.

## Project Structure

### Documentation (this feature)
```
specs/001-progettare-e-implementare/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# jsavrs compiler project structure (existing)
src/
├── cli.rs               # Command-line interface
├── lexer.rs             # Lexical analyzer
├── lib.rs               # Library root module
├── main.rs              # Application entry point
├── error/               # Error handling modules
├── ir/                  # Intermediate representation modules
│   ├── access_control.rs
│   ├── basic_block.rs
│   ├── cfg.rs
│   ├── dominance.rs
│   ├── function.rs
│   ├── generator.rs
│   ├── instruction.rs
│   ├── mod.rs
│   ├── module.rs
│   ├── scope.rs
│   ├── scope_manager.rs
│   ├── ssa.rs
│   ├── terminator.rs
│   ├── types.rs
│   └── value/                # IR value representations
│       ├── constant.rs
│       ├── debug_info.rs
│       ├── kind.rs
│       ├── literal.rs
│       └── mod.rs
├── location/            # Source location tracking
├── parser/              # Parser modules
├── semantic/            # Semantic analysis
├── time/                # Timing modules
├── tokens/              # Token definitions
└── utils/               # Utility functions

# NEW: Assembly generator integration
src/
├── asm/                 # NEW: Assembly generation module
│   ├── mod.rs           # Module root
│   ├── generator.rs     # Assembly generator core
│   ├── x86_64/          # x86-64 specific code
│   │   ├── mod.rs
│   │   ├── instruction_mapper.rs  # IR to x86-64 mapping
│   │   ├── register_alloc.rs      # Register allocation
│   │   └── abi/         # Calling convention support
│   ├── optimization/    # Assembly optimizations
│   └── debug/           # Debug information generation

tests/
├── asm_tests.rs         # NEW: Assembly generator tests
├── asm_snapshot_tests.rs # NEW: Snapshot tests for assembly output
└── [existing test files]
```

**Structure Decision**: Option 1 (Single project) - Extending existing jsavrs compiler infrastructure

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
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

**Assembly Generator Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base template
- Generate tasks from Phase 1 artifacts (data-model.md, contracts/interfaces.md, quickstart.md)
- Create implementation tasks following TDD methodology with assembly-specific validation

**Core Entity Implementation Tasks**:
- `AssemblyGenerator` → Core orchestrator implementation task [P]
- `InstructionMapper` trait → IR-to-x86-64 mapping interface task [P]
- `RegisterAllocator` → Linear scan allocation algorithm task [P]
- `DebugInfoGenerator` → Multi-level debug information creation task [P]
- `ErrorHandler` → Structured error management and stub generation task [P]

**Interface Contract Validation Tasks**:
- Each contract interface → contract test task validating input/output behavior [P]
- NASM compatibility validation → integration test ensuring assembly syntax correctness
- Cross-platform ABI compliance → test task for System V and Microsoft x64 conventions
- Performance benchmark → test task measuring compilation speed and debug overhead

**Assembly-Specific Integration Tasks**:
- IR module integration → test task validating connection to existing `@src/ir` components
- iced-x86 integration → test task ensuring proper instruction encoding
- NASM validation pipeline → test task running generated assembly through NASM assembler
- Debug information accuracy → test task validating DWARF section generation and debugger compatibility

**Ordering Strategy**:
- **Foundation First**: Core traits and interfaces before implementations
- **TDD Workflow**: Contract tests before implementation tasks
- **Dependency Respect**: IR integration before assembly generation, basic mapping before optimization
- **Platform Independence**: Core logic before platform-specific ABI implementations
- **Quality Gates**: Assembly validation and performance tests after core functionality
- **Parallelization**: Mark [P] for tasks targeting independent modules/components

**Sequential Dependencies**:
1. Core trait definitions and interface contracts
2. Basic IR parsing and module loading capability  
3. Simple instruction mapping (arithmetic operations)
4. Register allocation with stack fallback
5. NASM output generation and validation
6. Error handling and stub generation
7. Debug information generation (configurable levels)
8. Optimization passes and performance validation

**Estimated Output**: 35-40 numbered, ordered tasks covering complete assembly generator implementation

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
- [x] Phase 0: Research complete (/plan command) - ✅ research.md generated with comprehensive x86-64 assembly generation analysis
- [x] Phase 1: Design complete (/plan command) - ✅ data-model.md, contracts/interfaces.md, quickstart.md created; AGENTS.md updated
- [x] Phase 2: Task planning complete (/plan command - describe approach only) - ✅ Assembly-specific task generation strategy documented
- [x] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete  
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS - ✅ No violations identified, all principles addressed
- [x] Post-Design Constitution Check: PASS - ✅ Design maintains constitutional compliance
- [x] All NEEDS CLARIFICATION resolved - ✅ Technical context fully specified with concrete technology choices
- [x] Complexity deviations documented - ✅ No complexity violations requiring justification

**Execution Summary**:
- **Feature**: x86-64 Assembly Code Generator for jsavrs Compiler
- **Artifacts Generated**: plan.md, research.md, data-model.md, contracts/interfaces.md, quickstart.md
- **Agent Context Updated**: AGENTS.md enhanced with assembly generator development context
- **Ready for**: /tasks command execution to generate detailed implementation tasks

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
