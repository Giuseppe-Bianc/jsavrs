
# Implementation Plan: Comprehensive Test Suite for Type Promotion Module

**Branch**: `002-creating-detailed-precise` | **Date**: 2025-10-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-creating-detailed-precise/spec.md`

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
Develop a comprehensive, detailed, precise, thorough, and in-depth test suite for the `src/ir/type_promotion.rs` module that achieves 100% line coverage. Tests must validate all execution pathways including normal operations, edge cases (boundary conditions, extreme values), and corner cases (rare combinations, failure scenarios). The test suite must verify type promotion correctness, handle both panic and graceful error scenarios, and maintain compatibility with existing test infrastructure without performance benchmarks.

## Technical Context
**Language/Version**: Rust (edition 2024, current stable per project Cargo.toml)  
**Primary Dependencies**: 
- **Testing Framework**: Rust built-in `cargo test` + `insta` for snapshot testing (existing in project)
- **Coverage Tool**: `cargo llvm-cov` for 100% line coverage measurement and HTML report generation
- **Code Under Test**: `src/ir/type_promotion.rs` module with PromotionMatrix, TypePromotion, PromotionResult entities
- **Related Modules**: `src/ir/types.rs` (IrType enum), `src/ir/instruction.rs` (CastKind, IrBinaryOp), `src/ir/value.rs` (Value)
**Storage**: N/A (test suite for stateless type promotion logic)  
**Testing Strategy**: 
- **Unit Tests**: Direct testing of PromotionMatrix methods, TypePromotion construction, rule application
- **Integration Tests**: Binary operation type promotion scenarios combining multiple components
- **Panic Tests**: `#[should_panic(expected = "...")]` for defensive programming validations
- **Error Tests**: `Result<T, E>` assertion for graceful error handling pathways
- **Snapshot Tests**: `insta` for complex promotion result validation where applicable
**Target Platform**: Cross-platform (Windows, macOS, Linux) per project constitution  
**Project Type**: Single project (Rust compiler infrastructure)  
**Performance Goals**: 
- Test execution time: <5 seconds for entire test suite (fast feedback loop)
- Individual test execution: <100ms (deterministic, no I/O)
- Coverage computation: <10 seconds with `cargo llvm-cov --html`
**Constraints**: 
- **100% Line Coverage Mandate**: Every executable line in `src/ir/type_promotion.rs` must be tested
- **No Performance Benchmarks**: Functional correctness only (per Clarifications Q3)
- **Non-Disruptive Integration**: Existing tests in `tests/ir_type_promotion_tests.rs` must continue passing
- **Append-Only Strategy**: New tests added to end of file to minimize merge conflicts
- **Documentation Standard**: Each test with descriptive name + doc comment (per Clarifications Q5)
**Scale/Scope**: 
- **Module Size**: ~428 lines of Rust code in `src/ir/type_promotion.rs`
- **Existing Tests**: ~966 lines in `tests/ir_type_promotion_tests.rs` (baseline to extend)
- **Estimated New Tests**: 40-60 test functions to achieve 100% coverage + edge/corner cases
- **Test Categories**: Normal ops (40%), Edge cases (30%), Corner cases (20%), Error handling (10%)
**User-Provided Requirements**:
Create tests that are detailed, precise, thorough, and in-depth, ensuring they comprehensively validate functionality across all dimensions. The tests must not only confirm expected behavior under normal conditions but must also rigorously cover boundary conditions, corner cases, and edge cases, including invalid inputs, extreme values, concurrency issues (if applicable), and failure scenarios. Each test should be designed to eliminate ambiguity, maximize coverage, and expose hidden defects, with clear criteria for inputs, execution steps, and expected outcomes. The goal is to achieve reliability, robustness, and confidence in the system by verifying correctness, stability, scalability, and resilience under diverse and challenging circumstances.

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Alignment with Core Principles

#### ✅ Safety First
- **Compliance**: Test suite validates defensive programming (panic tests) and safe error handling (Result types)
- **Application**: Tests verify that type promotions preserve memory safety invariants and catch invalid type combinations
- **Evidence**: FR-007 mandates testing both panic conditions and graceful error propagation per Rust safety model

#### ✅ Performance Excellence  
- **Compliance**: Fast test execution (<5s suite, <100ms per test) enables rapid development cycles
- **Application**: No performance benchmarks in test suite (per Clarifications Q3), maintaining test determinism
- **Evidence**: Tests focus on correctness validation without timing assertions, supporting fast CI/CD feedback

#### ✅ Cross-Platform Compatibility
- **Compliance**: Tests use pure Rust without platform-specific dependencies
- **Application**: Type promotion logic is platform-agnostic; tests validate consistent behavior across Windows/macOS/Linux
- **Evidence**: Existing project CI tests on multiple platforms; new tests follow same patterns

#### ✅ Modular Extensibility
- **Compliance**: Tests organized by functionality (PromotionMatrix, TypePromotion, edge cases)
- **Application**: Test suite structure mirrors module architecture, enabling independent testing of components
- **Evidence**: Tests target public APIs and complex helper functions (per FR-009), supporting refactoring

#### ✅ Test-Driven Reliability
- **Compliance**: 100% line coverage mandate (FR-001, FR-006) with comprehensive scenario coverage
- **Application**: Tests validate normal ops, edge cases, corner cases, and failure modes systematically
- **Evidence**: FR-004 requires both normal operation and edge case scenarios; Clarifications define coverage targets

#### ✅ Snapshot Validation
- **Compliance**: Insta crate available for complex promotion result validation
- **Application**: Snapshot tests can capture PromotionResult structures with warnings, cast instructions
- **Evidence**: Project already uses Insta; tests can leverage for regression detection on complex outputs

#### ✅ Documentation Rigor
- **Compliance**: Every test documented with descriptive name + rustdoc comment (FR-010, Clarifications Q5)
- **Application**: Tests serve as executable documentation explaining type promotion behavior
- **Evidence**: "Detailed, precise, meticulous, in-depth" testing mandate aligns with documentation rigor principle

#### ✅ Community Principles (Collaboration, Respectful Communication, Shared Learning, Quality Through Community, Transparency)
- **Compliance**: Test suite supports collaborative development through clear, maintainable test code
- **Application**: Well-documented tests enable newcomers to understand type promotion system
- **Evidence**: Tests as learning resources (Shared Learning), peer-reviewable quality (Quality Through Community)

### Constitutional Violations
**None Identified** - All requirements align with project principles. No complexity tracking needed.

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

**Status**: ✅ COMPLETED (2025-10-05)

**Research Activities Completed**:
1. ✅ **Extracted research requirements from Technical Context**:
   - Testing framework selection for Rust compiler infrastructure
   - Coverage measurement strategies (100% line coverage mandate)
   - Edge case identification methodologies (type boundaries, numeric boundaries, circular dependencies)
   - Test organization and naming conventions
   - Error testing strategy (panic vs Result)
   - Documentation standards compliance

2. ✅ **Generated and executed comprehensive research**:
   - Task: "Research testing strategies for type promotion module in a detailed, precise, meticulous, and in-depth way"
   - Task: "Find best practices for cargo test in compiler infrastructure in a detailed, precise, meticulous, and in-depth way"
   - Task: "Find patterns for comprehensive type promotion testing in Rust in a detailed, precise, meticulous, and in-depth way"

3. ✅ **Consolidated findings** in `research.md` with 14 detailed sections:
   - Decision: Rust Built-in Testing + Insta Snapshot Testing + cargo llvm-cov
   - Rationale: Native support, minimal dependencies, accurate coverage measurement
   - Alternatives considered: Proptest (property-based), Criterion (benchmarking), Mockall (mocking)

**Research Outputs**:
- ✅ `research.md` created (67KB, 600+ lines) with all NEEDS CLARIFICATION resolved
- ✅ Testing framework selection documented with decision rationale
- ✅ Coverage strategy defined (100% line coverage target)
- ✅ Edge case taxonomy established (4 categories with test examples)
- ✅ Test organization patterns and naming conventions specified
- ✅ Dual-mode error testing strategy (panic + Result tests)
- ✅ Helper function testing granularity criteria defined
- ✅ Documentation standards template provided
- ✅ CI/CD integration strategy outlined
- ✅ Snapshot testing patterns documented
- ✅ Test matrix design approach (equivalence partitioning + boundary value analysis)
- ✅ Rust-specific best practices identified
- ✅ Test data management patterns established
- ✅ Risk analysis with mitigation strategies

**Key Research Findings**:
- **Testing Stack**: cargo test (native), Insta (snapshots), cargo llvm-cov (coverage)
- **Coverage Target**: 100% line coverage with LLVM-based instrumentation
- **Test Count Estimate**: 40-60 test functions (40% normal, 30% edge, 20% corner, 10% error)
- **Performance Targets**: <5s full suite, <100ms per test
- **Edge Case Categories**: Type system boundaries, numeric boundaries, circular dependencies, resource exhaustion
- **Test Naming**: `test_<entity>_<operation>_<scenario>_<expected_outcome>`
- **Documentation**: Descriptive name + rustdoc comment per FR-010

**Output**: research.md with all NEEDS CLARIFICATION resolved ✅

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

**Status**: ✅ COMPLETED (2025-10-05)

### Task Generation Strategy for Test Suite Implementation

The `/tasks` command will generate a comprehensive `tasks.md` file with ordered, actionable tasks derived from the design artifacts created in Phase 1. This section documents the task generation approach for implementing the comprehensive test suite for `src/ir/type_promotion.rs`.

---

### 1. Input Artifacts Analysis

**Task Generation Inputs**:
- `spec.md`: 10 functional requirements (FR-001 to FR-010), 4 edge case categories
- `data-model.md`: 15 entities (8 domain, 7 test), 50+ fields, 20+ validation rules
- `contracts/test-contracts.md`: 6 contract patterns (C-001 to C-006) with signature templates
- `research.md`: 14 sections on testing strategies, edge case taxonomy, best practices
- `quickstart.md`: Validation workflow and success criteria
- `src/ir/type_promotion.rs`: 428 lines of production code to test

**Key Extraction Points**:
- **From spec.md**: Test requirements (100% coverage, documentation standards, edge cases)
- **From data-model.md**: Entities to validate (PromotionMatrix, PromotionRule, etc.)
- **From contracts**: Test function patterns and naming conventions
- **From research.md**: Edge case taxonomy (4 categories with specific scenarios)

---

### 2. Task Categorization and Ordering

#### 2.1 Task Categories

Tasks will be organized into logical categories following TDD principles:

**Category A: Test Infrastructure Setup** (1-2 tasks)
- Task 1: Set up test file structure and helper functions
- Task 2: Create test data constants (ALL_INTEGER_TYPES, ALL_FLOAT_TYPES, etc.)

**Category B: Normal Operation Tests** (15-20 tasks, 40% of test suite)
- Tasks for PromotionMatrix methods (new, get_promotion_rule, compute_common_type)
- Tasks for PromotionRule variants (Direct, Indirect, Forbidden)
- Tasks for TypePromotion operations
- Tasks for identity promotions (T → T for all types)

**Category C: Edge Case Tests** (12-15 tasks, 30% of test suite)
- Tasks for type system boundary violations (Category 1 from research.md)
- Tasks for numeric boundary conditions (Category 2: MIN, MAX, NAN, INFINITY)
- Tasks for circular dependency detection (Category 3)
- Tasks for resource exhaustion scenarios (Category 4)

**Category D: Corner Case Tests** (8-10 tasks, 20% of test suite)
- Tasks for rare combinations of edge conditions
- Tasks for deeply nested promotion sequences
- Tasks for pathological type combinations

**Category E: Error Handling Tests** (4-5 tasks, 10% of test suite)
- Tasks for panic tests (#[should_panic])
- Tasks for Result/Option error paths
- Tasks for invalid input validation

**Category F: Coverage and Validation** (2-3 tasks)
- Task: Run cargo llvm-cov and verify 100% line coverage
- Task: Review and accept Insta snapshots (if applicable)
- Task: Validate documentation completeness (all tests have rustdoc comments)

---

#### 2.2 Ordering Strategy

**TDD Order: Tests Before Implementation Verification**
1. Write failing tests first (Category A → B → C → D → E)
2. Verify tests fail appropriately (no false positives)
3. Run coverage analysis (Category F)
4. Iterate until 100% coverage achieved

**Dependency Order: Infrastructure → Tests → Validation**
```
Test Infrastructure (Category A)
      ↓
Normal Operation Tests (Category B)
      ↓
Edge Case Tests (Category C)
      ↓
Corner Case Tests (Category D)
      ↓
Error Handling Tests (Category E)
      ↓
Coverage Validation (Category F)
```

**Rationale**:
- **Infrastructure first**: Helper functions enable efficient test implementation
- **Normal operations before edge cases**: Establish baseline behavior before testing boundaries
- **Edge cases before corner cases**: Common boundaries before rare combinations
- **Error handling last**: Validate defensive programming after happy paths
- **Coverage validation final**: Ensure 100% target met

---

#### 2.3 Parallel Execution Marking

Tasks that can be executed independently will be marked with **[P]** for parallel execution:

**Parallelizable Tasks**:
- All tests within the same category (e.g., Category B tasks are independent)
- Helper function creation tasks
- Documentation tasks
- Snapshot review tasks (if no dependencies)

**Non-Parallelizable Tasks**:
- Test infrastructure setup (Task 1) → Must complete before other tests
- Coverage validation (final task) → Depends on all tests being implemented

**Example Task Ordering with Parallel Marks**:
```
1. [REQUIRED] Set up test infrastructure (helper functions, constants)
2. [P] Implement PromotionMatrix::new() tests (Category B)
3. [P] Implement PromotionMatrix::get_promotion_rule() tests (Category B)
4. [P] Implement identity promotion tests (Category B)
5. [P] Implement I32 → F64 direct cast tests (Category B)
... (15-20 Category B tasks, all [P])
21. [P] Implement i32::MAX boundary tests (Category C)
22. [P] Implement f32::NAN handling tests (Category C)
... (12-15 Category C tasks, all [P])
N. [REQUIRED] Verify 100% coverage with cargo llvm-cov (Category F)
```

---

### 3. Task Generation Rules

#### 3.1 Task Granularity

**Guideline**: Each task represents **1-3 test functions** (not individual assertions).

**Reasoning**:
- **Too coarse** (10+ tests per task): Loss of progress tracking granularity
- **Too fine** (1 assertion per task): Overhead of task management exceeds value
- **Optimal** (1-3 tests): Balance between progress visibility and task count

**Example Granularity**:
```
✅ Good Task:
Task 12: Implement signed integer widening tests (I8 → I16, I16 → I32, I32 → I64)
  - test_i8_to_i16_widening_no_loss()
  - test_i16_to_i32_widening_no_loss()
  - test_i32_to_i64_widening_no_loss()

❌ Too Coarse:
Task 12: Implement all numeric type promotion tests
  (Would include 20+ tests across multiple categories)

❌ Too Fine:
Task 12a: Assert i8_to_i16 returns Direct cast
Task 12b: Assert i8_to_i16 has no precision loss
Task 12c: Assert i8_to_i16 has no overflow warning
```

---

#### 3.2 Task Template

Each task will follow this structure:

```markdown
## Task N: [Category] [Action] [Scope]

**Status**: Not Started / In Progress / Completed
**Priority**: High / Medium / Low
**Estimated Effort**: 15-30 minutes (per task)
**Parallelizable**: [P] Yes / No
**Dependencies**: Task M (if applicable)

### Description
[1-2 sentences explaining what this task accomplishes]

### Acceptance Criteria
- [ ] Test functions implemented with descriptive names following `test_<entity>_<operation>_<scenario>` convention
- [ ] All tests have rustdoc comments explaining rationale (FR-010)
- [ ] Tests pass when executed with `cargo test --test ir_type_promotion_tests`
- [ ] [Specific criteria for this task category]

### Implementation Guidance
[2-3 bullet points with specific guidance from research.md or contracts]

### Examples
[Code snippet showing expected test structure from contracts/test-contracts.md]

### Validation
```bash
# Command to verify this task is complete
cargo test --test ir_type_promotion_tests [test_pattern]
```
```

---

#### 3.3 Task Generation Algorithm (Pseudocode)

```
FOR each entity in data-model.md:
    FOR each method in entity:
        IF method is public:
            CREATE Task(category=Normal, entity=entity, method=method)
        ELSE IF method is complex helper (cyclomatic complexity >3):
            CREATE Task(category=Normal, entity=entity, method=method, note="direct test required")

FOR each edge_case in research.md edge_case_taxonomy:
    IF edge_case in category_1 (type boundaries):
        CREATE Task(category=EdgeCase, scenario=edge_case.scenario)
    ELSE IF edge_case in category_2 (numeric boundaries):
        CREATE Task(category=EdgeCase, scenario=edge_case.scenario)
    ELSE IF edge_case in category_3 (circular deps):
        CREATE Task(category=EdgeCase, scenario=edge_case.scenario)
    ELSE IF edge_case in category_4 (resource exhaustion):
        CREATE Task(category=CornerCase, scenario=edge_case.scenario)

FOR each contract in contracts/test-contracts.md:
    IF contract.type == "panic_test":
        CREATE Task(category=ErrorHandling, contract=contract)

CREATE Task(category=Validation, action="verify_coverage", target="100%")
CREATE Task(category=Validation, action="review_snapshots")
CREATE Task(category=Validation, action="validate_documentation")
```

---

### 4. Estimated Task Count and Distribution

**Total Estimated Tasks**: 25-30 tasks

**Distribution**:
| Category | Task Count | Percentage | Examples |
|----------|-----------|------------|----------|
| A: Infrastructure | 1-2 | 5% | Helper functions, test data constants |
| B: Normal Operations | 15-20 | 50% | PromotionMatrix methods, identity promotions |
| C: Edge Cases | 8-10 | 30% | Boundary values, type system violations |
| D: Corner Cases | 4-5 | 15% | Deeply nested sequences, pathological combinations |
| E: Error Handling | 2-3 | 10% | Panic tests, Result validation |
| F: Validation | 2-3 | 5% | Coverage verification, snapshot review |

**Rationale for Distribution**:
- **Normal operations (50%)**: Most tasks because production code has many public methods
- **Edge cases (30%)**: Systematic boundary testing requires dedicated tasks
- **Corner + Error (25%)**: Fewer tasks but still comprehensive coverage
- **Validation (5%)**: Final verification tasks

---

### 5. Task Execution Workflow

**Step 1: Read tasks.md** (generated by /tasks command)
```bash
cat specs/002-creating-detailed-precise/tasks.md
```

**Step 2: Execute tasks in order** (respecting dependencies)
```bash
# Example: Execute Task 1 (Infrastructure Setup)
# Manually implement helper functions in tests/ir_type_promotion_tests.rs

# Example: Execute Task 2 (PromotionMatrix::new tests) [P]
# Implement test_promotion_matrix_new()
# Run: cargo test --test ir_type_promotion_tests test_promotion_matrix_new
```

**Step 3: Mark tasks as completed** (update tasks.md Status field)
```markdown
**Status**: ~~Not Started~~ → **Completed** ✅
```

**Step 4: Iterate until all tasks completed**
```bash
# After each task, verify tests pass
cargo test --test ir_type_promotion_tests

# Periodically check coverage
cargo llvm-cov --package jsavrs --lib --text | grep "type_promotion"
```

**Step 5: Final validation** (Category F tasks)
```bash
# Verify 100% coverage
cargo llvm-cov --package jsavrs --lib --html --open
# Expect: src/ir/type_promotion.rs: 100.00%

# Review snapshots (if any)
cargo insta review

# Validate documentation completeness
cargo doc --package jsavrs --lib
# Manually verify all test functions have rustdoc comments
```

---

### 6. Task Metadata Fields

Each task will include these metadata fields:

| Field | Purpose | Values |
|-------|---------|--------|
| **Status** | Tracks completion | Not Started \| In Progress \| Completed |
| **Priority** | Guides execution order | High (dependencies) \| Medium \| Low |
| **Estimated Effort** | Time planning | 15-30 minutes per task (typical) |
| **Parallelizable** | Enables concurrent work | [P] Yes \| No |
| **Dependencies** | Enforces task order | Task N (if dependent on prior task) |
| **Category** | Organizes tasks | A (Infra) \| B (Normal) \| C (Edge) \| D (Corner) \| E (Error) \| F (Validation) |

---

### 7. Success Criteria for tasks.md

The generated `tasks.md` file will be considered complete when:

**Completeness Criteria**:
- ✅ All 15 entities from data-model.md have associated test tasks
- ✅ All 4 edge case categories from research.md have test tasks
- ✅ All 6 contract patterns from contracts/test-contracts.md are represented
- ✅ Task count: 25-30 tasks (matches estimated test function count of 40-60 with 1-3 tests per task)
- ✅ Distribution matches target percentages (±5%): 50% Normal, 30% Edge, 15% Corner, 10% Error

**Quality Criteria**:
- ✅ Every task has clear Acceptance Criteria
- ✅ Every task has Implementation Guidance from design docs
- ✅ Every task has validation command (e.g., `cargo test <pattern>`)
- ✅ Dependencies are correctly identified (no circular dependencies)
- ✅ Parallelizable tasks marked with [P]

**Traceability Criteria**:
- ✅ Each task references source design document (spec.md, data-model.md, research.md, contracts)
- ✅ Each task maps to at least one functional requirement (FR-001 to FR-010)
- ✅ Each task aligns with constitutional principles (Safety, Testing, Documentation)

---

### 8. Example Task Generation Output (Sample)

**Task 1: Infrastructure Setup**
```markdown
## Task 1: [Infrastructure] Set Up Test Helper Functions and Constants

**Status**: Not Started
**Priority**: High
**Estimated Effort**: 20 minutes
**Parallelizable**: No (dependency for all other tasks)
**Dependencies**: None
**Category**: A (Infrastructure)

### Description
Create reusable helper functions and test data constants to enable efficient test implementation. This includes functions to create PromotionMatrix instances with custom configurations and constants for common type lists.

### Acceptance Criteria
- [ ] Helper function `create_matrix_with_overflow(behavior: OverflowBehavior) -> PromotionMatrix` implemented
- [ ] Helper function `all_numeric_types() -> Vec<IrType>` implemented
- [ ] Constants `ALL_INTEGER_TYPES`, `ALL_FLOAT_TYPES`, `ALL_BASIC_TYPES` defined
- [ ] All helpers documented with rustdoc comments
- [ ] Helpers pass `cargo fmt` and `cargo clippy` checks

### Implementation Guidance
- Follow contract C-005 (Helper Functions) from contracts/test-contracts.md
- Reference research.md Section 12 (Test Data Management) for constant patterns
- Ensure helpers are deterministic (same inputs → same outputs)

### Examples
```rust
/// Creates a PromotionMatrix with custom overflow behavior.
fn create_matrix_with_overflow(behavior: OverflowBehavior) -> PromotionMatrix {
    PromotionMatrix::with_overflow_behavior(behavior)
}

const ALL_INTEGER_TYPES: &[IrType] = &[
    IrType::I8, IrType::I16, IrType::I32, IrType::I64,
    IrType::U8, IrType::U16, IrType::U32, IrType::U64,
];
```

### Validation
```bash
# Verify helpers compile
cargo test --test ir_type_promotion_tests --no-run

# Verify no clippy warnings
cargo clippy --test ir_type_promotion_tests
```
```

**Task 12: Edge Case - I32 MAX Boundary**
```markdown
## Task 12: [EdgeCase] Implement I32 MAX Boundary Value Tests

**Status**: Not Started
**Priority**: Medium
**Estimated Effort**: 25 minutes
**Parallelizable**: [P] Yes
**Dependencies**: Task 1 (Infrastructure)
**Category**: C (Edge Cases)

### Description
Implement tests validating type promotion behavior for I32::MAX boundary value. Tests should cover widening conversions (I32 → I64, I32 → F32, I32 → F64) and verify no overflow/precision loss warnings for safe conversions.

### Acceptance Criteria
- [ ] Test `test_i32_max_to_i64_widening_no_overflow()` implemented
- [ ] Test `test_i32_max_to_f32_potential_precision_loss()` implemented (F32 has 24-bit significand)
- [ ] Test `test_i32_max_to_f64_exact_representation()` implemented (F64 has 53-bit significand)
- [ ] All tests have rustdoc comments explaining boundary condition rationale (FR-010)
- [ ] Tests pass with `cargo test --test ir_type_promotion_tests i32_max`

### Implementation Guidance
- Reference research.md Section 3 (Edge Case Identification) → Category 2: Numeric Boundary Conditions
- Follow contract C-003 (Edge Case Tests) from contracts/test-contracts.md
- Use boundary value `i32::MAX` explicitly in test setup

### Examples
```rust
/// Tests that I32::MAX promotes to I64 without overflow warning.
///
/// # Rationale
/// I64 can represent all I32 values including MAX. This validates FR-003
/// (widening conversions should not generate overflow warnings).
#[test]
fn test_i32_max_to_i64_widening_no_overflow() {
    let matrix = PromotionMatrix::new();
    let result = matrix.compute_common_type(&IrType::I32, &IrType::I64);
    
    assert_eq!(result, Some(IrType::I64));
    // Verify no overflow warning for widening conversion
    let analysis = matrix.analyze_promotion(/* i32::MAX value */);
    assert!(!analysis.warnings.iter().any(|w| matches!(w, PromotionWarning::PotentialOverflow { .. })));
}
```

### Validation
```bash
cargo test --test ir_type_promotion_tests i32_max
# Expected: 3 tests pass
```
```

---

### 9. Task Generation Tool Invocation

**Command**:
```bash
# From repository root
/tasks
```

**Expected Output**:
```
✅ Loaded design artifacts:
   - spec.md (10 functional requirements)
   - data-model.md (15 entities)
   - contracts/test-contracts.md (6 contract patterns)
   - research.md (14 sections, 4 edge case categories)
   - quickstart.md (validation workflow)

✅ Generated 28 tasks:
   - Category A (Infrastructure): 2 tasks
   - Category B (Normal Operations): 16 tasks [P]
   - Category C (Edge Cases): 8 tasks [P]
   - Category D (Corner Cases): 4 tasks [P]
   - Category E (Error Handling): 3 tasks [P]
   - Category F (Validation): 2 tasks

✅ Wrote tasks.md to specs/002-creating-detailed-precise/tasks.md

Next steps:
1. Review tasks.md for completeness
2. Execute tasks in order (respecting dependencies)
3. Run /tasks validate after each task to track progress
```

---

### 10. Summary

This Phase 2 task planning approach establishes:
- **6 task categories** (A-F) with clear responsibilities
- **Ordering strategy** (TDD order, dependency order, parallel execution marking)
- **Task granularity** (1-3 test functions per task, 25-30 total tasks)
- **Task template** (Status, Priority, Effort, Parallelizable, Dependencies, Acceptance Criteria, Implementation Guidance, Examples, Validation)
- **Task generation algorithm** (pseudocode for extracting tasks from design artifacts)
- **Success criteria** (completeness, quality, traceability)
- **Example tasks** (Infrastructure Setup, Edge Case Boundary Tests)
- **Execution workflow** (Read → Execute → Mark Complete → Validate → Iterate)

The `/tasks` command will use this strategy to generate a detailed, actionable `tasks.md` file for implementing the comprehensive test suite.
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
- [x] Phase 0: Research complete (/plan command) ✅ 2025-10-05
- [x] Phase 1: Design complete (/plan command) ✅ 2025-10-05
- [x] Phase 2: Task planning complete (/plan command - describe approach only) ✅ 2025-10-05
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS ✅ 2025-10-05
- [x] Post-Design Constitution Check: PASS ✅ 2025-10-05 (no violations identified)
- [x] All NEEDS CLARIFICATION resolved ✅ 2025-10-05 (research.md created)
- [x] Complexity deviations documented ✅ 2025-10-05 (none - straightforward test suite)

**Deliverables Summary**:
- ✅ research.md (67KB, 600+ lines, 14 sections)
- ✅ data-model.md (90KB, 800+ lines, 15 entities, 50+ fields)
- ✅ contracts/test-contracts.md (6 contract patterns with templates)
- ✅ quickstart.md (validation workflow, CI/CD integration)
- ✅ QWEN.md updated (current development focus section added)
- ✅ plan.md (complete implementation plan with Phase 0-2 details)

**Next Action**: Execute `/tasks` command to generate tasks.md with 25-30 ordered, actionable tasks

---
*Based on Constitution v1.4.1 - See `/memory/constitution.md`*
