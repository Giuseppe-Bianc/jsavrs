
# Implementation Plan: IR Type Promotion System Correction

**Branch**: `002-the-intermediate-representation` | **Date**: 29 settembre 2025 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-the-intermediate-representation/spec.md`

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
The intermediate representation (IR) currently exhibits incorrect or inconsistent type-promotion behavior when evaluating operations that involve operands of different types. This implementation plan addresses systematic correction of the IR's type-promotion rules through: 1) defining a comprehensive type lattice with clear promotion precedence, 2) implementing deterministic promotion algorithms, 3) inserting explicit cast nodes where needed, 4) ensuring precision preservation and standard compliance, and 5) comprehensive testing and documentation updates. The approach prioritizes correctness over performance initially, with floating-point types taking precedence in mixed operations and signed/unsigned interactions promoting to larger signed types.

## Technical Context
**Language/Version**: Rust 1.90.0 (cargo 1.90.0)  
**Primary Dependencies**: logos 0.15.1 (lexing), uuid 1.18.1 (unique IDs), petgraph 0.8.2 (CFG), iced-x86 1.21.0 (assembly generation)  
**Storage**: File-based IR serialization (no database)  
**Testing**: cargo test (unit/integration), insta 1.43.2 (snapshot testing), criterion 0.7.0 (benchmarking)  
**Target Platform**: Cross-platform (Windows, macOS, Linux) compiler system
**Project Type**: single - compiler/transpiler project with modular architecture  
**Performance Goals**: Focus on correctness over performance initially, with future optimization phases  
**Constraints**: Must preserve numeric precision, handle IEEE floating-point standards, maintain deterministic behavior  
**Scale/Scope**: Compiler IR system supporting mixed-type operations across all numeric types (i8-i64, u8-u64, f32-f64)

**IR Implementation Location**: User specified the IR implementation is in the `src/ir` folder, which contains modules for types, instructions, basic blocks, dominance analysis, SSA form, and code generation integration.

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Compliance Analysis**:
- ✅ **Safety First**: Implementation leverages Rust's ownership model and type system for memory safety. Type promotion system will use strong typing to prevent type confusion vulnerabilities.
- ✅ **Performance Excellence**: Initial focus on correctness aligns with constitutional principle allowing performance optimization in subsequent phases. Deterministic algorithms enable future performance tuning.
- ✅ **Cross-Platform Compatibility**: Type promotion rules will be platform-independent, ensuring consistent behavior across Windows, macOS, and Linux.
- ✅ **Modular Extensibility**: Implementation will integrate with existing IR module structure without requiring core system rewrites. New promotion system will be pluggable and extensible.
- ✅ **Test-Driven Reliability**: Comprehensive unit tests, integration tests, and snapshot testing with insta framework. Regression test coverage for all mixed-type operation cases.
- ✅ **Snapshot Validation**: Using insta library for validating IR output consistency and catching regressions in type promotion behavior.
- ✅ **Documentation Rigor**: Will produce detailed research.md and data-model.md files with comprehensive analysis of type lattice design and promotion algorithms using AI assistance for thoroughness.

**Result**: PASS - All constitutional principles are satisfied by the proposed approach.

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

**Structure Decision**: [DEFAULT to Option 1 unless Technical Context indicates web/mobile app]

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
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

**Detailed Task Generation Strategy for IR Type Promotion System**:

Based on the comprehensive design artifacts generated in Phase 1, the /tasks command will create the following task categories:

**Core Infrastructure Tasks** (Priority: High, Parallel: Yes):
1. Create `src/ir/type_promotion.rs` with core promotion engine [P]
2. Implement `PromotionMatrix` with complete type lattice [P]
3. Add `TypePromotion` and `PromotionResult` data structures [P]
4. Extend `CastKind` enum for new promotion scenarios [P]

**Integration Tasks** (Priority: High, Sequential):
5. Modify `generate_binary` in `src/ir/generator.rs` to use promotion analysis
6. Implement cast insertion logic in IR generation
7. Add promotion warning system integration
8. Update error handling for type promotion failures

**Testing Infrastructure** (Priority: High, Parallel: Yes):
9. Unit tests for promotion matrix lookups [P]
10. Unit tests for type lattice algorithms [P]
11. Integration tests for binary operation promotion [P]
12. Snapshot tests for IR output validation [P]

**Edge Case Handling** (Priority: Medium, Sequential):
13. Implement special float value handling (NaN, infinity)
14. Add overflow detection and configurable behavior
15. Handle precision loss warnings and user feedback
16. Implement signed/unsigned interaction edge cases

**Documentation and Validation** (Priority: Medium, Parallel: Yes):
17. Update module documentation for new promotion system [P]
18. Create migration guide for breaking changes [P]
19. Add user-facing documentation for type promotion rules [P]
20. Performance benchmark suite for promotion overhead [P]

**Backend Integration** (Priority: Low, Sequential):
21. Verify assembly generation compatibility with promoted IR
22. Update code generation for new cast instruction patterns
23. Cross-platform validation for promotion behavior
24. Memory layout consistency checks

**Task Dependencies**:
- Tasks 1-4 must complete before task 5
- Task 5 must complete before tasks 6-8
- Tasks 9-12 can run in parallel with infrastructure development
- Tasks 13-16 depend on tasks 5-8 completion
- Documentation tasks (17-20) can start after task 8
- Backend integration tasks (21-24) require all core tasks complete

**Ordering Strategy**: TDD approach with parallel execution where possible
- Contract tests first (based on API contracts from Phase 1)
- Core data structures and algorithms
- Integration with existing IR generator
- Comprehensive testing and validation
- Documentation and migration support

**Estimated Output**: 24 numbered, ordered tasks in tasks.md with clear dependencies and parallelization markers

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
