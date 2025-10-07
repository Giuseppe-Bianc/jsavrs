
# Implementation Plan: Comprehensive Type Promotion Engine Test Suite

**Branch**: `003-create-detailed-precise` | **Date**: October 6, 2025 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-create-detailed-precise/spec.md`

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
This plan implements a comprehensive, detailed, precise, thorough, and in-depth test suite for the TypePromotionEngine module in the jsavrs compiler. The test suite validates all type promotion scenarios across 12 IrType variants (I8, I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Char) with all binary operations (arithmetic, comparison, logical, bitwise). Testing includes edge cases (boundary values, special float values, matrix edge cases), corner cases (helper method validation through engine usage, promotion chains, concurrent execution), and employs a hybrid testing approach combining snapshot testing for complex outputs with explicit assertions for critical properties. The suite targets 100% line and branch coverage with both integration tests using real PromotionMatrix instances and unit tests with mocked matrix behavior.

## Technical Context
**Language/Version**: Rust 1.75+ (stable)  
**Primary Dependencies**: 
- `cargo test` - Standard Rust testing framework
- `insta` - Snapshot testing library for complex output validation
- `mockall` or manual mocking - For isolating TypePromotionEngine from PromotionMatrix dependencies
- `criterion` (if performance benchmarks needed)

**Storage**: N/A (test suite only)  
**Testing**: 
- Unit tests with `cargo test`
- Snapshot tests with `insta` crate
- Integration tests with real PromotionMatrix
- Unit tests with mocked PromotionMatrix
- Concurrent execution tests using `std::thread`

**Target Platform**: Cross-platform (Windows, macOS, Linux) via Rust's standard library  
**Project Type**: single (Rust compiler project with standard source structure)  
**Performance Goals**: 
- Fast test execution (<10 seconds for full suite)
- Individual test performance <100ms except concurrent tests
- Snapshot comparison performance <50ms per snapshot

**Constraints**: 
- Tests must be added to NEW file `tests/ir_type_promotion_engine_tests.rs`
- 100% line coverage required for TypePromotionEngine module
- 100% branch coverage for all conditional logic
- All 12 IrType variants must be tested
- All IrBinaryOp variants must be tested
- Hybrid assertion strategy (snapshots + explicit assertions)

**Scale/Scope**: 
- Target: 100+ individual test functions
- 12 IrType variants × 12 IrType variants = 144 type pair combinations (subset tested based on relevance)
- All IrBinaryOp variants (15+ operations)
- 8 test groups organized by functionality
- Expected test file size: 2000-3000 lines

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Safety First**: ✅ PASS - Test suite promotes safety through comprehensive validation of type promotion logic, ensuring that the TypePromotionEngine prevents type-related runtime errors. Tests validate proper cast generation, warning generation for unsafe conversions, and correct handling of edge cases that could lead to undefined behavior.

**Performance Excellence**: ✅ PASS - Tests are designed for fast execution using Rust's efficient testing framework. Snapshot testing with `insta` provides performance benefits over manual assertion generation. Test organization allows parallel execution of independent test groups.

**Cross-Platform Compatibility**: ✅ PASS - Test suite uses only cross-platform Rust standard library features and platform-independent crates (insta, cargo test). No platform-specific code or assumptions.

**Modular Extensibility**: ✅ PASS - Test suite is organized into clear functional groups (analyze_binary_promotion, insert_promotion_casts, warnings, edge cases, etc.), enabling easy addition of new test cases as the TypePromotionEngine evolves. Hybrid testing strategy (integration + unit tests) supports testing at multiple abstraction levels.

**Test-Driven Reliability**: ✅ PASS - This feature IS the implementation of comprehensive testing methodology. The test suite provides 100% coverage targets, includes regression testing via snapshots, covers unit and integration scenarios, and validates correctness across diverse type combinations and operations.

**Snapshot Validation**: ✅ PASS - Hybrid approach uses `insta` snapshot testing for complex PromotionResult structures and warning collections, enabling easy detection of unintended changes in type promotion behavior while maintaining explicit assertions for critical properties (result types, cast presence, is_sound flag).

**Collaboration First**: ✅ PASS - Test suite documentation (through clear test names, module comments, and organization) facilitates collaborative development by making test intent immediately clear. Comprehensive coverage enables confident refactoring by team members.

**Respectful Communication**: ✅ PASS - Test code and documentation maintain professional, clear communication style. Test names follow consistent patterns making intent obvious without requiring extensive explanation.

**Shared Learning**: ✅ PASS - Test suite serves as living documentation of TypePromotionEngine behavior. Comprehensive edge and corner case coverage demonstrates correct usage patterns and expected behaviors, providing learning resource for contributors.

**Quality Through Community**: ✅ PASS - Comprehensive test suite enables effective code review by providing clear acceptance criteria. Tests document expected behavior unambiguously, facilitating community validation of implementation correctness.

**No Constitution Violations Detected**

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

The `/tasks` command will generate implementation tasks from the design artifacts (contracts, data-model, research) created in Phase 1. Each task will be discrete, testable, and ordered according to TDD principles.

### Task Categories:

1. **Test Infrastructure Setup Tasks** (3-5 tasks, [P] parallel):
   - T001 [P]: Create test file `tests/ir_type_promotion_engine_tests.rs` with module structure and imports
   - T002 [P]: Add `insta = "1.34"` to `Cargo.toml` dev-dependencies for snapshot testing
   - T003 [P]: Create test fixture builder helpers (TestFixtureBuilder, assertion_helpers module)
   - T004 [P]: Create mock PromotionMatrix structure for unit tests
   - T005 [P]: Set up test constants (test_types, test_operations modules)

2. **Core Function Tests: analyze_binary_promotion** (40-50 tasks, grouped by scenario):
   - T006-T017 [P]: Identity promotion tests (12 tests, one per IrType variant)
   - T018-T041: Widening promotion tests (24 tests: 6 signed + 6 unsigned, each with multiple targets)
   - T042-T065: Narrowing promotion tests (24 tests: reverse of widening)
   - T066-T073: Cross-signedness promotion tests (8 tests: 4 pairs × 2 directions)
   - T074-T089: Integer-to-float promotion tests (16 tests: 8 int types × 2 float types)
   - T090-T105: Float-to-integer promotion tests (16 tests: 2 float types × 8 int types)
   - T106-T107: Float widening/narrowing tests (2 tests: F32↔F64)
   - T108-T161: Operation-specific tests (54 tests: 18 operations × 3 type scenarios)

3. **Core Function Tests: insert_promotion_casts** (30-40 tasks):
   - T162-T173 [P]: No casts required tests (12 tests, identity promotions)
   - T174-T203: Left operand cast only tests (30 tests, various type pairs and cast kinds)
   - T204-T233: Right operand cast only tests (30 tests, mirror of left tests)
   - T234-T253: Both operands cast tests (20 tests, bilateral casting scenarios)
   - T254-T303: CastKind validation tests (50 tests, all cast type combinations)
   - T304-T313: SourceSpan preservation tests (10 tests, verify span propagation)

4. **Warning Generation Tests** (40-50 tasks):
   - T314-T333: PrecisionLoss warning tests (20 tests: float→int, F64→F32, large int→F32, narrowing)
   - T334-T348: PotentialOverflow warning tests (15 tests: float→int overflow, division, value exceeds range)
   - T349-T358: SignednessChange warning tests (10 tests: mixed signed/unsigned, comparisons)
   - T359-T366: FloatSpecialValues warning tests (8 tests: NaN, Infinity scenarios)
   - T367-T376: Multiple warnings tests (10 tests: combined warnings in single promotion)

5. **Edge Case Tests** (40-50 tasks):
   - T377-T391: Type boundary tests (15 tests: I8→I64, same-width cross-sign, boundary values)
   - T392-T401: Float-integer boundary tests (10 tests: NaN, Infinity, large int→F32)
   - T402-T409: Promotion matrix edge tests (8 tests: None returns, missing rules, bidirectional)
   - T410-T419: Operation-specific edge tests (10 tests: division overflow, bitwise signed, modulo)

6. **Corner Case Tests** (15-20 tasks):
   - T420-T427: Multi-warning scenarios (8 tests: 3+ warnings, cascading)
   - T428-T433: System boundary tests (6 tests: missing rules, fallbacks)
   - T434-T438: Promotion chain tests (5 tests: multi-step promotions)

7. **Integration Tests with Real Matrix** (30-40 tasks):
   - T439-T458: Real-world type combination tests (20 tests: common programming scenarios)
   - T459-T473: Complex multi-step promotion tests (15 tests: operations with multiple casts)
   - T474-T483: Warning integration tests (10 tests: validate warning generation with real matrix)
   - T484-T488: Performance validation tests (5 tests: ensure acceptable performance)

8. **Unit Tests with Mocked Matrix** (20-30 tasks):
   - T489-T503: Matrix behavior isolation tests (15 tests: mock specific matrix behaviors)
   - T504-T513: Error path tests (10 tests: mock matrix returning None for edge cases)
   - T514-T523: Engine logic validation tests (10 tests: verify engine decision-making independently)

9. **Concurrent Execution Tests** (3 tasks):
   - T524: Multi-threaded read test (10 threads, 100 operations each)
   - T525: Varied type combinations concurrent test (different type pairs per thread)
   - T526: High thread count stress test (50-100 threads)

10. **Coverage Verification & Documentation** (5 tasks):
    - T527: Generate coverage report and verify 100% line coverage
    - T528: Verify 100% branch coverage for all conditionals
    - T529: Add module-level documentation to test file
    - T530: Create snapshot test baseline (run tests, accept all snapshots)
    - T531: Validate test execution performance (<10 seconds full suite)

### Task Ordering Strategy:

**Phase-Based Ordering** (TDD order: Tests before implementation):

1. **Phase A - Infrastructure** (T001-T005): Set up test infrastructure first [P]
2. **Phase B - Core Tests** (T006-T313): Implement all core function tests
   - Subphase B1: analyze_binary_promotion tests (T006-T161)
   - Subphase B2: insert_promotion_casts tests (T162-T313)
3. **Phase C - Warning Tests** (T314-T376): Implement warning generation tests
4. **Phase D - Edge/Corner Tests** (T377-T438): Implement edge and corner case tests
5. **Phase E - Integration Tests** (T439-T488): Implement integration tests with real matrix
6. **Phase F - Unit Tests** (T489-T523): Implement unit tests with mocked matrix
7. **Phase G - Concurrent Tests** (T524-T526): Implement concurrent execution tests
8. **Phase H - Verification** (T527-T531): Coverage verification and documentation

**Parallel Execution Markers**:
- Tasks marked [P] are independent and can be executed in parallel
- Most test implementation tasks are independent (separate test functions)
- Sequential dependencies only exist for infrastructure → tests ordering

### Estimated Task Breakdown:
- **Infrastructure**: 5 tasks
- **Core Tests**: 230 tasks (analyze: 156, insert_casts: 142, adjusted for overlap: ~152)
- **Warning Tests**: 63 tasks
- **Edge/Corner Tests**: 62 tasks
- **Integration/Unit Tests**: 60 tasks
- **Concurrent Tests**: 3 tasks
- **Verification**: 5 tasks
- **Total**: ~**388 tasks** (will be consolidated to ~120-150 discrete tasks in tasks.md)

**Note**: The `/tasks` command will consolidate related test scenarios into logical task groups to keep tasks.md manageable (e.g., "Implement all identity promotion tests" as one task covering T006-T017).

### Expected tasks.md Output:
- **Section 1**: Infrastructure setup (5 tasks)
- **Section 2**: Core function tests - grouped by scenario type (20-30 tasks)
- **Section 3**: Warning, edge, corner tests - grouped logically (15-20 tasks)
- **Section 4**: Integration, unit, concurrent tests (10-15 tasks)
- **Section 5**: Verification and documentation (5 tasks)
- **Total tasks in tasks.md**: ~**55-75 consolidated tasks**

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
- [x] Phase 0: Research complete (/plan command) - ✅ research.md created
- [x] Phase 1: Design complete (/plan command) - ✅ data-model.md, contracts/, quickstart.md created
- [x] Phase 2: Task planning complete (/plan command - describe approach only) - ✅ Strategy documented
- [ ] Phase 3: Tasks generated (/tasks command) - Ready for execution
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS - All principles satisfied
- [x] Post-Design Constitution Check: PASS - No violations introduced
- [x] All NEEDS CLARIFICATION resolved - Clarifications section complete in spec.md
- [x] Complexity deviations documented - None required

**Artifact Status**:
- [x] `specs/003-create-detailed-precise/spec.md` - Feature specification complete
- [x] `specs/003-create-detailed-precise/plan.md` - This file, implementation plan complete
- [x] `specs/003-create-detailed-precise/research.md` - Research complete (15 sections, all decisions documented)
- [x] `specs/003-create-detailed-precise/data-model.md` - Data model complete (9 sections, all entities defined)
- [x] `specs/003-create-detailed-precise/contracts/analyze_binary_promotion_contract.md` - Contract complete
- [x] `specs/003-create-detailed-precise/contracts/insert_promotion_casts_contract.md` - Contract complete
- [x] `specs/003-create-detailed-precise/contracts/remaining_contracts_summary.md` - All contracts documented
- [x] `specs/003-create-detailed-precise/quickstart.md` - Quickstart guide complete
- [ ] `specs/003-create-detailed-precise/tasks.md` - To be generated by /tasks command

**Coverage Goals**:
- Target: 100% line coverage for `src/ir/type_promotion_engine.rs`
- Target: 100% branch coverage for all conditional logic
- Target: All 12 IrType variants tested
- Target: All 18 IrBinaryOp variants tested
- Target: 100-120 test functions implemented
- Target: Test execution time <10 seconds

**Ready for Next Command**: ✅ `/tasks` - Generate tasks.md from design artifacts

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
