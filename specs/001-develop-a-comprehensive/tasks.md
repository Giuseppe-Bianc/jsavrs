# Tasks: Comprehensive x86-64 ABI Trait System

**Input**: Design documents from `C:\dev\vscode\rust\jsavrs\specs\001-develop-a-comprehensive\`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/, quickstart.md

## Execution Flow (main)
```
1. Prerequisite Validation & Specification Discovery
   → Verify existence: specs/001-develop-a-comprehensive/plan.md (mandatory)
   → Verify existence: specs/001-develop-a-comprehensive/research.md (optional)
   → Verify existence: specs/001-develop-a-comprehensive/data-model.md (mandatory)
   → Verify existence: specs/001-develop-a-comprehensive/contracts/ (mandatory, must contain 4 files)
   → Verify existence: specs/001-develop-a-comprehensive/quickstart.md (mandatory)
   → Abort execution if mandatory files missing; log specific missing artifacts with relative paths
   → Parse plan.md and extract technical constraints:
     • Minimum Rust version: 1.75.0 (edition 2021 required for const generics, improved type inference)
     • Architecture mandate: trait-based polymorphism with compile-time dispatch via monomorphization
     • Performance SLA: ABI query overhead < 0.1% relative to direct function invocation baseline
     • Runtime performance target: median query latency < 10 nanoseconds (99th percentile < 50ns)
   → Extract dependency specifications with version constraints:
     • tracing = "0.1.x" (structured diagnostic logging with zero-cost when disabled)
     • criterion = "0.5.x" (statistical benchmarking with outlier detection)
     • insta = "1.x" (snapshot testing with automatic review workflow)
   → Validate plan.md completeness: must define success criteria, timeline estimates, risk mitigation
   → Exit Gate: All mandatory files present, plan.md syntactically valid → PROCEED; else ABORT with diagnostic

2. Domain Model Analysis & Semantic Validation
   → Parse data-model.md and extract entity definitions (expected: 50+ entities):
     • Platform enumeration: { Windows, Linux, macOS } with 1:N relationship to Abi instances
     • Abi enumeration: { WindowsX64, SystemV } with 1:1 bijective mapping to Platform
     • Register type hierarchy:
       - GPRegister64 (64-bit general purpose): RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8-R15
       - GPRegister32 (32-bit aliased views): EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D-R15D
       - GPRegister16 (16-bit aliased views): AX, BX, CX, DX, SI, DI, BP, SP, R8W-R15W
       - GPRegister8 (8-bit aliased views): AL, BL, CL, DL, SIL, DIL, BPL, SPL, R8B-R15B
       - XMMRegister (128-bit SIMD): XMM0-XMM15 (floating-point parameter passing)
       - Specialized registers: RSP (stack pointer, never allocatable), RBP (frame pointer, conditionally allocatable)
     • Type system enumeration:
       - FieldType: { Integer, Float, Pointer } (governs register class selection in SystemV)
       - AggregateClass: { ByValue(register), ByReference(stack_offset), Decomposed(register_list) }
   → Parse contracts/ directory (expected: 4 trait definition files):
     • calling_convention_trait.md: 5 methods
       - integer_param_register(index: usize) -> Option<GPRegister64>
       - float_param_register(index: usize) -> Option<XMMRegister>
       - max_integer_register_params() -> usize
       - max_float_register_params() -> usize
       - variadic_param_register(index: usize, is_float: bool) -> Option<Register>
     • stack_management_trait.md: 5 methods
       - has_red_zone() -> bool
       - red_zone_size_bytes() -> usize
       - requires_shadow_space() -> bool
       - shadow_space_bytes() -> usize
       - min_stack_alignment() -> usize
     • register_allocation_trait.md: 4 methods
       - volatile_gp_registers() -> &'static [GPRegister64]
       - non_volatile_gp_registers() -> &'static [GPRegister64]
       - volatile_xmm_registers() -> &'static [XMMRegister]
       - is_volatile(register: Register) -> bool
     • aggregate_classification_trait.md: 2 methods
       - classify_aggregate(size_bytes: usize, fields: &[FieldType]) -> AggregateClass
       - requires_implicit_pointer(size_bytes: usize) -> bool
   → Parse quickstart.md and extract integration scenarios (expected: 4 scenarios):
     • Scenario 1: Function prologue generation with shadow space allocation (Windows) vs red zone utilization (SystemV)
     • Scenario 2: Multi-parameter function with mixed integer/float arguments (validate register selection order)
     • Scenario 3: Temporary register allocation during expression evaluation (priority ordering verification)
     • Scenario 4: Structure return value handling (ByValue vs ByReference classification boundary testing)
   → Cross-reference validation (referential integrity checks):
     • Verify all register types in data-model.md referenced in register_allocation_trait.md methods
     • Verify all FieldType variants in data-model.md used in aggregate_classification_trait.md signatures
     • Verify all AggregateClass variants in data-model.md returned by classify_aggregate method
     • Verify all Platform/Abi combinations in data-model.md have corresponding trait implementations documented
   → Exit Gate: Entity count ≥ 50, contract count = 4, scenario count = 4, referential integrity satisfied → PROCEED; else ABORT with violation report

3. Task Generation Strategy (Test-Driven Development Phasing)
   → Phase 3.1 (Infrastructure Setup): Generate foundation tasks
     • T001: Augment Cargo.toml [dependencies] section at ./Cargo.toml
       - Add tracing = "0.1" with features = ["attributes", "std"]
       - Add criterion = "0.5" with features = ["html_reports"]
       - Add insta = "1" with features = ["yaml"]
     • T002: Create Criterion benchmark harness at benches/abi_benchmarks.rs
       - Configure warm-up time = 3s, measurement time = 5s, sample size = 100
       - Enable Criterion HTML report generation in target/criterion/
     • T003: Validate linting configuration at rustfmt.toml
       - Verify max_width = 120, edition = "2021", use_field_init_shorthand = true
       - Ensure clippy.toml exists with deny = ["warnings", "clippy::all", "clippy::pedantic"]
     • Parallelization: T003 independent of T001-T002 (modifies different files)
     • Success Criteria: cargo check succeeds, rustfmt --check passes, clippy returns zero diagnostics
   
   → Phase 3.2 (Test-First Implementation - TDD Red Phase): Generate failing test tasks
     • T004: Write CallingConvention contract tests at tests/abi_calling_convention_tests.rs
       - Test parameter register allocation: integer_param_register(0..=10) expected sequences
       - Test parameter count limits: max_integer_register_params() = {4 Windows, 6 SystemV}
       - Test variadic conventions: variadic_param_register() stack transition behavior
       - Test index space semantics: Windows overlapping (RCX = integer[0] = float[0]), SystemV independent (RDI ≠ XMM0)
       - Performance assertion: Verify O(1) lookup via const array indexing (no branching in disassembly)
       - Expected Result: cargo test abi_calling_convention → ALL TESTS FAIL (trait not implemented)
     
     • T005: Write StackManagement contract tests at tests/abi_stack_management_tests.rs
       - Test red zone semantics: has_red_zone() = {false Windows, true SystemV}, red_zone_size_bytes() = {0, 128}
       - Test shadow space requirements: requires_shadow_space() = {true Windows, false SystemV}, shadow_space_bytes() = {32, 0}
       - Test alignment constraints: min_stack_alignment() = 16 for both ABIs (SSE requirement)
       - Test frame pointer policy: requires_frame_pointer() = false (optimization-dependent, not ABI-mandated)
       - Verification: Confirm all methods use const evaluation (check via cargo asm or const context usage)
       - Expected Result: cargo test abi_stack_management → ALL TESTS FAIL (trait not implemented)
     
     • T006: Write RegisterAllocation contract tests at tests/abi_register_allocation_tests.rs
       - Test volatile register priority: volatile_gp_registers() order = [RAX, R10, R11, ...] (allocation preference)
       - Test non-volatile register preservation: non_volatile_gp_registers() Windows = [..., RDI, RSI] (callee-saved), SystemV excludes RDI/RSI
       - Test volatility classification: is_volatile(RCX) = {true Windows, true SystemV}, is_volatile(RDI) = {false Windows, true SystemV}
       - Test edge cases: RSP always excluded, RBP conditionally excluded (frame pointer dependency)
       - Expected Result: cargo test abi_register_allocation → ALL TESTS FAIL (trait not implemented)
     
     • T007: Write AggregateClassification contract tests at tests/abi_aggregate_classification_tests.rs
       - Test small aggregate classification: size ≤ 8 bytes → ByValue (Windows), size ≤ 16 bytes → ByValue or Decomposed (SystemV)
       - Test large aggregate passing: size > 8 bytes → ByReference (Windows), size > 16 bytes → ByReference (SystemV)
       - Test SystemV decomposition: struct { int32, float32 } → Decomposed([RDI, XMM0])
       - Test field type influence: FieldType::Float in first eightbyte → XMM register class selection
       - Reference compiler validation: Compare with GCC/Clang/MSVC assembly for struct { int64, int32 }, struct { double, int32 }, etc.
       - Expected Result: cargo test abi_aggregate_classification → ALL TESTS FAIL (trait not implemented)
     
     • T008: Write integration tests at tests/abi_integration_tests.rs
       - Scenario 1: Generate function prologue for `fn(i32, i32, i32, i32, i32)` → verify shadow space (Windows) vs red zone (SystemV)
       - Scenario 2: Allocate registers for `fn(i32, f64, i32, f64)` → verify Windows [RCX, XMM1, R8, XMM3] vs SystemV [RDI, XMM0, RSI, XMM1]
       - Scenario 3: Select temporary register for intermediate computation → verify priority ordering (RAX before R10 before R11)
       - Scenario 4: Handle structure return `fn() -> Struct128` → verify Windows implicit pointer in RCX vs SystemV decomposition
       - Expected Result: cargo test abi_integration → ALL TESTS FAIL (traits not implemented)
     
     • Parallelization: T004-T008 fully independent (5 separate test files, no shared state)
     • TDD Verification Gate: Execute cargo test --lib --tests → Expected failure rate = 100% (all tests must fail before proceeding)
   
   → Phase 3.3 (Implementation - TDD Green Phase): Generate trait implementation tasks
     • T009: Implement CallingConvention at src/asm/calling_convention.rs
       - Define WindowsX64 struct with static const arrays: INTEGER_PARAMS = [RCX, RDX, R8, R9], FLOAT_PARAMS = [XMM0, XMM1, XMM2, XMM3]
       - Define SystemV struct with static const arrays: INTEGER_PARAMS = [RDI, RSI, RDX, RCX, R8, R9], FLOAT_PARAMS = [XMM0..=XMM7]
       - Implement integer_param_register: index < MAX ? Some(ARRAY[index]) : None (guaranteed O(1) with bounds check elimination)
       - Implement variadic_param_register: Windows → stack after index 4, SystemV → AL register holds XMM count for varargs
       - Add comprehensive rustdoc: /// # Examples, /// # Platform Differences, /// # Performance (O(1) const array access)
       - Verification: cargo test abi_calling_convention → Expected success rate = 100%
     
     • T010: Implement StackManagement at src/asm/stack_management.rs
       - Implement WindowsX64: const HAS_RED_ZONE = false, SHADOW_SPACE = 32, ALIGNMENT = 16
       - Implement SystemV: const HAS_RED_ZONE = true, RED_ZONE_SIZE = 128, ALIGNMENT = 16
       - Use const fn for all methods → zero runtime cost, compile-time evaluation
       - Add rustdoc examples: Function prologue code generation with shadow space allocation
       - Verification: cargo test abi_stack_management → Expected success rate = 100%
     
     • T011: Implement RegisterAllocation at src/asm/register_allocation.rs
       - Define static slices: VOLATILE_GP_WIN: &[GPRegister64] = &[RAX, R10, R11, RCX, RDX, R8, R9]
       - Define static slices: NON_VOLATILE_GP_WIN: &[GPRegister64] = &[RBX, RDI, RSI, R12, R13, R14, R15]
       - Implement is_volatile: self.volatile_gp_registers().contains(&reg) (O(N) but N ≤ 16, cacheable)
       - Add rustdoc: Document Windows vs SystemV differences (RDI/RSI volatility inversion)
       - Verification: cargo test abi_register_allocation → Expected success rate = 100%
     
     • T012: Implement AggregateClassification at src/asm/aggregate_classification.rs
       - Define enums: AggregateClass { ByValue(GPRegister64), ByReference, Decomposed(Vec<Register>) }
       - Define enums: FieldType { Integer, Float, Pointer }
       - Implement WindowsX64::classify_aggregate: size ≤ 8 ? ByValue(RCX) : ByReference
       - Implement SystemV::classify_aggregate: size > 16 ? ByReference : complex eightbyte classification (defer to reference compiler for edge cases)
       - Add rustdoc: Reference GCC ABI documentation (https://github.com/hjl-tools/x86-psABI/wiki/x86-64-psABI-1.0.pdf)
       - Verification: cargo test abi_aggregate_classification → Expected success rate = 100%
     
     • T013: Refactor existing code at src/asm/register.rs
       - Update GPRegister64::is_volatile(abi: &dyn CallingConvention) → delegate to abi.volatile_gp_registers().contains(self)
       - Update GPRegister64::is_callee_saved(abi: &dyn CallingConvention) → delegate to abi.non_volatile_gp_registers().contains(self)
       - Add #[deprecated] annotations to old direct methods: is_volatile_windows(), is_volatile_systemv() (migration path)
       - Maintain backward compatibility: Provide default implementations calling deprecated methods (one release cycle)
       - Verification: cargo test (all existing tests) → Expected success rate = 100% (no regressions)
     
     • Parallelization: T009-T012 fully independent (4 separate source files), T013 sequential after T009-T012 complete (modifies shared register.rs)
     • TDD Verification Gate: Execute cargo test --lib --tests → Expected failure rate = 0% (all tests must pass)
   
   → Phase 3.4 (Integration Validation): Generate cross-cutting validation tasks
     • T014: Cross-compiler validation at tests/abi_cross_compiler_validation.rs
       - Generate C test files: int add5(int a, int b, int c, int d, int e) { return a+b+c+d+e; }
       - Compile with MSVC (cl.exe /c /FaWindows_add5.asm), GCC (gcc -S -masm=intel -o Linux_add5.s), Clang (clang -S -o Darwin_add5.s)
       - Parse assembly: Extract mov instructions for parameter registers using regex /mov\s+(\w+),\s+(\w+)/
       - Compare with jsavrs: WindowsX64::integer_param_register(0..5) vs parsed [RCX, RDX, R8, R9, stack]
       - Assert: Match rate ≥ 95% (tolerate minor instruction reordering or optimization differences)
       - Document discrepancies: Known cases like tail-call optimization altering register usage
       - Success Criteria: 95%+ concordance with reference compilers across 20+ test functions
     
     • T015: Snapshot testing at tests/abi_snapshot_tests.rs
       - Use insta::assert_yaml_snapshot! for deterministic output verification
       - Snapshot parameter sequences: serialize WindowsX64::integer_param_register(0..10) → ["RCX", "RDX", "R8", "R9", null, ...]
       - Snapshot volatility classifications: serialize all GPRegister64 variants with is_volatile() results
       - Snapshot aggregate classifications: struct sizes [1, 2, 4, 8, 12, 16, 20, 24] with field types [Integer], [Float], [Integer, Float]
       - Generate baselines: cargo insta test --review --accept (manual inspection required)
       - Success Criteria: cargo insta test → zero snapshot differences (idempotent output)
     
     • T016: Quickstart validation at tests/abi_quickstart_validation.rs
       - Execute Scenario 1 example code: let prologue = generate_prologue(WindowsX64); assert_eq!(prologue, "sub rsp, 32 ; shadow space");
       - Execute Scenario 2 example code: let regs = allocate_params(&[Int32, Float64, Int32, Float64]); assert_eq!(regs, [RCX, XMM1, R8, XMM3]);
       - Execute Scenario 3 example code: let temp = select_temporary(WindowsX64); assert_eq!(temp, RAX); // highest priority volatile
       - Execute Scenario 4 example code: let ret = classify_return(Struct128); assert_eq!(ret, ByReference); // Windows, size > 8
       - Success Criteria: All 4 scenarios execute without panics, produce documented outputs
     
     • Parallelization: T015-T016 parallel after T014 completes (T014 establishes correctness baseline)
     • Validation Gate: All integration tests pass → PROCEED to Phase 3.5
   
   → Phase 3.5 (Performance & Observability): Generate quality assurance tasks
     • T017: Performance benchmarking at benches/abi_benchmarks.rs
       - Benchmark WindowsX64::integer_param_register(3): black_box to prevent constant folding, 1000 iterations
       - Benchmark SystemV::float_param_register(5): measure median, mean, std deviation, identify outliers
       - Benchmark GPRegister64::is_volatile(RAX, WindowsX64): compare with baseline direct array contains
       - Benchmark classify_aggregate(12, &[Integer, Float, Integer]): measure allocation overhead for Decomposed variant
       - Use Criterion statistical analysis: Detect performance regressions > 5% with Mann-Whitney U test
       - Performance Target: Median < 10ns (verify via criterion HTML reports at target/criterion/report/index.html)
       - Success Criteria: All benchmarks meet latency targets, zero regressions detected
     
     • T018: Tracing instrumentation at src/asm/calling_convention.rs (+ other trait impl files)
       - Add #[tracing::instrument(level = "debug")] to CallingConvention::integer_param_register
       - Log ABI decisions: tracing::debug!(index, register = ?result, "Selected integer parameter register");
       - Add #[tracing::instrument(level = "trace")] to performance-critical paths (< 10ns budget)
       - Add span context: let _span = tracing::span!(tracing::Level::INFO, "abi_query", platform = "Windows").entered();
       - Verification: RUST_LOG=jsavrs=trace cargo test 2>&1 | grep "Selected integer parameter register" → expect non-zero matches
     
     • T019: Documentation generation at src/asm/ (all public trait modules)
       - Add detailed trait docs: /// The `CallingConvention` trait defines the interface for querying platform-specific parameter passing conventions.
       - Add method examples: /// # Examples\n/// ```\n/// let win = WindowsX64;\n/// assert_eq!(win.integer_param_register(0), Some(GPRegister64::RCX));\n/// ```
       - Document platform differences: /// # Platform Differences\n/// - **Windows**: First 4 integer parameters in RCX, RDX, R8, R9\n/// - **SystemV**: First 6 integer parameters in RDI, RSI, RDX, RCX, R8, R9
       - Document performance: /// # Performance\n/// This method uses constant-time array indexing with no branching. Median latency < 5ns.
       - Run cargo doc --no-deps --open → manually verify rendering, check for broken links
       - Success Criteria: 100% public API coverage (verify with cargo deadlinks), zero rustdoc warnings
     
     • T020: Code duplication analysis at ./ (project root)
       - Execute: similarity-rs --skip-test --threshold 0.8 --output duplication_report.txt
       - Parse report: Identify duplicated blocks in src/asm/calling_convention.rs (WindowsX64 vs SystemV implementations)
       - Extract common patterns: If duplication > 10 lines, extract to shared helper in src/asm/abi_common.rs
       - Document justified duplication: Platform-specific constant arrays (INTEGER_PARAMS) cannot be abstracted without runtime cost
       - Success Criteria: < 5% duplicated code in src/asm/ (excluding justified platform-specific constants)
     
     • Parallelization: T017-T018 sequential (benchmarks before instrumentation to avoid measurement skew), T019-T020 parallel (independent concerns)
     • Quality Gate: All performance targets met, documentation complete, duplication minimized → RELEASE CANDIDATE READY

4. Task Dependency Resolution & Scheduling
   → Construct directed acyclic graph (DAG) of task dependencies:
     • Phase 3.1 → Phase 3.2 (setup before tests)
     • Phase 3.2 → Phase 3.3 (tests must exist and fail before implementation)
     • T009-T012 → T013 (trait implementations before existing code refactor)
     • Phase 3.3 → Phase 3.4 (implementation before validation)
     • T014 → T015-T016 (cross-compiler baseline before snapshots)
     • Phase 3.4 → Phase 3.5 (validation before performance/observability)
     • T017 → T018 (benchmarks before tracing to avoid measurement pollution)
   → Identify parallelization opportunities (critical path optimization):
     • T004-T008: 5-way parallelism (test file generation)
     • T009-T012: 4-way parallelism (trait implementation)
     • T015-T016: 2-way parallelism (validation tests)
     • T019-T020: 2-way parallelism (documentation vs analysis)
   → Calculate critical path length: 8 sequential phases (Setup → Tests → Impl → Refactor → CrossCompile → Validate → Bench → Trace → Doc/Dup)
   → Estimate wall-clock time with parallelization: ~60% reduction vs sequential execution (Amdahl's law with 40% inherently sequential work)

5. Task Enumeration & Metadata Assignment
   → Assign unique identifiers: T001 through T020 (zero-padded for sortability)
   → Assign phase labels: 3.1 (Setup), 3.2 (Tests), 3.3 (Implementation), 3.4 (Integration), 3.5 (Polish)
   → Assign file paths: Relative paths from project root
     • Source files: src/asm/{calling_convention, stack_management, register_allocation, aggregate_classification, abi_common, register}.rs
     • Test files: tests/abi_{calling_convention, stack_management, register_allocation, aggregate_classification, integration, cross_compiler_validation, snapshot, quickstart_validation}_tests.rs
     • Benchmark files: benches/abi_benchmarks.rs
     • Configuration files: {Cargo.toml, rustfmt.toml, clippy.toml}
   → Assign success criteria (quantitative, measurable, automated):
     • Test coverage: ≥ 90% line coverage, ≥ 85% branch coverage (verify with cargo llvm-cov)
     • Performance: Median latency < 10ns, 99th percentile < 50ns (verify with Criterion reports)
     • Cross-compiler concordance: ≥ 95% match rate with GCC/Clang/MSVC (verify with assembly parsing)
     • Snapshot stability: Zero differences on repeated runs (verify with cargo insta test)
     • Documentation coverage: 100% public API (verify with cargo deadlinks)
     • Code duplication: < 5% in src/asm/ (verify with similarity-rs)
   → Assign explicit dependencies: Task IDs with prerequisite relationships (e.g., T013 depends_on [T009, T010, T011, T012])

6. Pre-Execution Validation (Quality Gate)
   → Contract coverage verification:
     • For each of 4 contracts in contracts/, verify existence of test task (T004-T007) and implementation task (T009-T012)
     • Assert: |contract test tasks| = 4 AND |contract impl tasks| = 4
   → TDD enforcement verification:
     • Verify all test tasks (T004-T008) have lower task IDs than implementation tasks (T009-T013)
     • Assert: max(test_task_ids) < min(impl_task_ids)
   → File conflict detection:
     • For all tasks marked [P], verify no two tasks modify the same file path
     • Build adjacency matrix: conflicts[i][j] = (tasks[i].file_path == tasks[j].file_path)
     • Assert: ∀ i,j where [P] marked, conflicts[i][j] = false
   → Dependency cycle detection:
     • Perform topological sort on task dependency DAG
     • Assert: Topological sort completes successfully (no back edges detected)
   → Integration coverage verification:
     • Verify T008 (integration tests) references all 4 scenarios from quickstart.md
     • Assert: |scenarios in T008| = 4
   → Performance coverage verification:
     • Verify T017 (benchmarks) covers all trait methods from contracts/
     • Assert: |benchmarked methods| ≥ |trait methods| (minimum full coverage)
   → Documentation coverage verification:
     • Verify T019 (rustdoc) lists all public APIs from T009-T012
     • Assert: |documented APIs in T019| = |public APIs in src/asm/|
   → Exit criteria evaluation:
     • If all 7 validation checks pass → Log "Pre-execution validation PASSED" → PROCEED to task execution
     • If any validation check fails → Log specific failure with diagnostic details → ABORT with error code
     • Generate validation report: validation_report.json with pass/fail status for each check
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Source**: `c:\dev\vscode\rust\jsavrs\src\asm\` (existing ABI infrastructure)
- **Tests**: `c:\dev\vscode\rust\jsavrs\tests\` (contract and integration tests)
- **Benchmarks**: `c:\dev\vscode\rust\jsavrs\benches\` (performance validation)

## Phase 3.1: Setup & Configuration
- [ ] **T001** Add dependencies to `Cargo.toml`: `tracing = "0.1"`, `criterion = "0.5"`, `insta = "1.x"` at repository root
- [ ] **T002** Configure Criterion benchmarking infrastructure in `benches/abi_benchmarks.rs`
- [ ] **T003** [P] Verify rustfmt and clippy configuration in `rustfmt.toml` at repository root

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests [All Parallel - Different Files]
- [ ] **T004** [P] Write CallingConvention contract tests in `tests/abi_calling_convention_tests.rs`
  - Test parameter register allocation (integer_param_register, float_param_register)
  - Test max parameter counts (max_integer_register_params, max_float_register_params)
  - Test variadic function conventions (variadic_param_register)
  - Test index space behavior (Windows overlapping vs SystemV independent)
  - Verify constant-time lookups via assertions
  - **Expected Result**: All tests FAIL (traits not implemented yet)

- [ ] **T005** [P] Write StackManagement contract tests in `tests/abi_stack_management_tests.rs`
  - Test red zone queries (has_red_zone, red_zone_size_bytes)
  - Test shadow space requirements (requires_shadow_space, shadow_space_bytes)
  - Test stack alignment (min_stack_alignment)
  - Test frame pointer requirements (requires_frame_pointer)
  - Verify Windows vs SystemV differences
  - **Expected Result**: All tests FAIL (traits not implemented yet)

- [ ] **T006** [P] Write RegisterAllocation contract tests in `tests/abi_register_allocation_tests.rs`
  - Test volatile register priority ordering (volatile_gp_registers, volatile_xmm_registers)
  - Test non-volatile register priority ordering (non_volatile_gp_registers)
  - Test volatility checks (is_volatile, is_callee_saved)
  - Verify platform-specific lists (Windows RDI/RSI callee-saved, SystemV volatile)
  - Test edge cases (RSP special handling, RBP optional)
  - **Expected Result**: All tests FAIL (traits not implemented yet)

- [ ] **T007** [P] Write AggregateClassification contract tests in `tests/abi_aggregate_classification_tests.rs`
  - Test small aggregate classification (size ≤ 8 bytes Windows, ≤ 16 bytes SystemV)
  - Test large aggregate classification (by-reference passing)
  - Test structure decomposition (SystemV register splitting)
  - Test field type influence (Integer, Float, Pointer)
  - Verify reference compiler behavior (GCC/Clang/MSVC alignment)
  - **Expected Result**: All tests FAIL (traits not implemented yet)

### Integration Tests [Parallel - Different Files]
- [ ] **T008** [P] Write quickstart scenario integration tests in `tests/abi_integration_tests.rs`
  - Test function prologue generation (Scenario 1 from quickstart.md)
  - Test parameter register allocation (Scenario 2)
  - Test temporary register selection (Scenario 3)
  - Test structure return handling (Scenario 4)
  - Verify Windows x64 add5 function example
  - Verify SystemV compute function example
  - **Expected Result**: All tests FAIL (traits not implemented yet)

## Phase 3.3: Core Implementation (ONLY after tests are failing)
**Dependencies**: T004-T008 must be complete and failing

### Trait Implementations [All Parallel - Different Files]
- [ ] **T009** [P] Implement CallingConvention trait in `src/asm/calling_convention.rs`
  - Create `WindowsX64` struct implementing `CallingConvention`
    - `integer_param_register`: RCX, RDX, R8, R9 (indices 0-3, None for ≥4)
    - `float_param_register`: XMM0-XMM3 (indices 0-3, overlaps with integer indices)
    - `max_integer_register_params`: 4
    - `max_float_register_params`: 4
    - `variadic_param_register`: All parameters on stack after first 4
  - Create `SystemV` struct implementing `CallingConvention`
    - `integer_param_register`: RDI, RSI, RDX, RCX, R8, R9 (indices 0-5)
    - `float_param_register`: XMM0-XMM7 (indices 0-7, independent index space)
    - `max_integer_register_params`: 6
    - `max_float_register_params`: 8
    - `variadic_param_register`: AL register holds XMM count
  - Use static constant arrays for O(1) lookups
  - Add comprehensive rustdoc comments with examples
  - **Verify**: T004 tests now PASS

- [ ] **T010** [P] Implement StackManagement trait in `src/asm/stack_management.rs`
  - Implement for `WindowsX64`:
    - `has_red_zone`: false
    - `red_zone_size_bytes`: 0
    - `requires_shadow_space`: true
    - `shadow_space_bytes`: 32
    - `min_stack_alignment`: 16
    - `requires_frame_pointer`: false
  - Implement for `SystemV`:
    - `has_red_zone`: true
    - `red_zone_size_bytes`: 128
    - `requires_shadow_space`: false
    - `shadow_space_bytes`: 0
    - `min_stack_alignment`: 16
    - `requires_frame_pointer`: false
  - All methods use const evaluation (zero runtime cost)
  - **Verify**: T005 tests now PASS

- [ ] **T011** [P] Implement RegisterAllocation trait in `src/asm/register_allocation.rs`
  - Implement for `WindowsX64`:
    - `volatile_gp_registers`: [RAX, R10, R11, RCX, RDX, R8, R9]
    - `non_volatile_gp_registers`: [RBX, RDI, RSI, R12, R13, R14, R15]
    - `volatile_xmm_registers`: [XMM0-XMM5]
    - `is_volatile`: Check against volatile lists
    - `is_callee_saved`: Check against non-volatile lists
  - Implement for `SystemV`:
    - `volatile_gp_registers`: [RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11]
    - `non_volatile_gp_registers`: [RBX, R12, R13, R14, R15]
    - `volatile_xmm_registers`: [XMM0-XMM15]
  - Use static slices for zero-allocation queries
  - **Verify**: T006 tests now PASS

- [ ] **T012** [P] Implement AggregateClassification trait in `src/asm/aggregate_classification.rs`
  - Define `AggregateClass` enum (ByValue, ByReference, Decomposed)
  - Define `FieldType` enum (Integer, Float, Pointer)
  - Implement for `WindowsX64`:
    - size ≤ 8 → `ByValue(RCX)`
    - size > 8 → `ByReference`
  - Implement for `SystemV`:
    - size > 16 → `ByReference`
    - size ≤ 8, single field → `ByValue` or `Decomposed` based on type
    - size ≤ 16, multiple fields → `Decomposed` with register list
    - Defer complex cases to reference compiler behavior (document with comments)
  - **Verify**: T007 tests now PASS

- [ ] **T013** Update existing `src/asm/register.rs` to use new traits
  - Refactor `is_volatile`, `is_callee_saved`, `is_parameter_register` methods
  - Delegate to appropriate trait implementations
  - Maintain backward compatibility with existing code
  - Add deprecation warnings for old direct method calls (if applicable)
  - **Verify**: Existing tests in `tests/` still pass

## Phase 3.4: Integration & Validation
**Dependencies**: T009-T013 must be complete

- [ ] **T014** Cross-compiler validation tests in `tests/abi_cross_compiler_validation.rs`
  - Generate C test files for function signatures (int add5(int, int, int, int, int))
  - Compile with MSVC (Windows), GCC (Linux), Clang (macOS)
  - Parse generated assembly with regex/asm parser
  - Compare register usage with jsavrs ABI trait queries
  - Assert matching parameter registers, stack offsets, calling conventions
  - Document any discrepancies with reference compiler behavior
  - **Success Criteria**: 95%+ match rate with reference compilers

- [ ] **T015** Snapshot tests for ABI query outputs in `tests/abi_snapshot_tests.rs`
  - Use insta crate for snapshot assertions
  - Snapshot parameter register sequences (0-10 parameters, mixed types)
  - Snapshot volatility classifications for all register types
  - Snapshot structure classification results (various sizes/field types)
  - Generate baseline snapshots with `cargo insta test --review`
  - **Success Criteria**: All snapshots consistent across test runs

- [ ] **T016** Validate quickstart.md examples in `tests/abi_quickstart_validation.rs`
  - Execute all code examples from quickstart.md as integration tests
  - Verify function prologue generation produces expected instructions
  - Verify parameter allocation matches documented behavior
  - Verify temporary register selection follows priority ordering
  - **Success Criteria**: All examples execute without errors, produce expected output

## Phase 3.5: Performance & Observability
**Dependencies**: T014-T016 must be complete

- [ ] **T017** Implement performance benchmarks in `benches/abi_benchmarks.rs`
  - Benchmark `integer_param_register(index)` for 1000 iterations
  - Benchmark `float_param_register(index)` for 1000 iterations
  - Benchmark `is_volatile(register)` for all register types
  - Benchmark `classify_aggregate(size, fields)` for various structures
  - Use Criterion library for statistical analysis
  - **Performance Target**: Median < 10 nanoseconds per query
  - **Success Criteria**: All benchmarks meet performance targets

- [ ] **T018** Integrate tracing instrumentation in `src/asm/calling_convention.rs`, `src/asm/stack_management.rs`, `src/asm/register_allocation.rs`, `src/asm/aggregate_classification.rs`
  - Add `#[tracing::instrument]` to all public trait methods
  - Log ABI decisions at DEBUG level (parameter register choices, structure classifications)
  - Log performance-critical paths at TRACE level
  - Add span context for compiler phases
  - **Verify**: `RUST_LOG=trace cargo test` produces comprehensive logs

- [ ] **T019** [P] Generate rustdoc documentation for all public APIs
  - Add detailed documentation comments to all trait definitions
  - Add examples to all trait methods (rustdoc `/// # Examples` sections)
  - Document platform-specific behavior in method docs
  - Document performance contracts (`/// # Performance`)
  - Run `cargo doc --open` to verify rendering
  - **Success Criteria**: 100% public API coverage, no rustdoc warnings

- [ ] **T020** [P] Run duplication analysis with `similarity-rs --skip-test`
  - Execute: `similarity-rs --skip-test --threshold 0.8`
  - Identify duplicated ABI query logic across WindowsX64/SystemV implementations
  - Extract common patterns into shared helper functions in `src/asm/abi_common.rs`
  - Document justified duplication (platform-specific behavior that can't be abstracted)
  - **Success Criteria**: < 5% duplicated code in ABI implementation modules

## Dependencies Graph
```
Setup (T001-T003)
   ↓
Contract Tests (T004-T007) [PARALLEL]
   ↓
Integration Tests (T008) [PARALLEL with T004-T007]
   ↓
Trait Implementations (T009-T013) [PARALLEL - after tests fail]
   ↓
Existing Code Update (T013) [SEQUENTIAL - modifies shared file]
   ↓
Validation (T014-T016) [SEQUENTIAL - T014 first, then T015-T016 parallel]
   ↓
Performance & Observability (T017-T020) [T017-T018 sequential, T019-T020 parallel]
```

## Parallel Execution Examples

### Phase 1: Write All Contract Tests Simultaneously
```bash
# Execute T004-T008 in parallel (5 separate test files)
Task: "Write CallingConvention contract tests in tests/abi_calling_convention_tests.rs"
Task: "Write StackManagement contract tests in tests/abi_stack_management_tests.rs"
Task: "Write RegisterAllocation contract tests in tests/abi_register_allocation_tests.rs"
Task: "Write AggregateClassification contract tests in tests/abi_aggregate_classification_tests.rs"
Task: "Write quickstart integration tests in tests/abi_integration_tests.rs"
```

### Phase 2: Implement All Traits Simultaneously (After Tests Fail)
```bash
# Execute T009-T012 in parallel (4 separate source files)
Task: "Implement CallingConvention trait in src/asm/calling_convention.rs"
Task: "Implement StackManagement trait in src/asm/stack_management.rs"
Task: "Implement RegisterAllocation trait in src/asm/register_allocation.rs"
Task: "Implement AggregateClassification trait in src/asm/aggregate_classification.rs"
```

### Phase 3: Parallel Documentation and Analysis
```bash
# Execute T019-T020 in parallel (different concerns)
Task: "Generate rustdoc documentation for all public APIs"
Task: "Run duplication analysis with similarity-rs --skip-test"
```

## Notes
- **[P] Marking**: Tasks marked [P] modify different files and have no dependencies
- **TDD Enforcement**: T004-T008 must be written first and must fail before T009-T013
- **Commit Strategy**: Commit after each task completion (atomic commits)
- **Performance Validation**: T017 benchmarks must run on every commit to detect regressions
- **Cross-Compiler Testing**: T014 requires GCC, Clang, MSVC installations (document in CI/CD setup)

## Task Generation Rules Applied

1. **From Contracts** (contracts/*.md):
   - calling_convention_trait.md → T004 (contract test), T009 (implementation)
   - stack_management_trait.md → T005 (contract test), T010 (implementation)
   - register_allocation_trait.md → T006 (contract test), T011 (implementation)
   - aggregate_classification_trait.md → T007 (contract test), T012 (implementation)

2. **From Data Model** (data-model.md):
   - Entities already exist in src/asm/register.rs (GPRegister64, XMMRegister, etc.)
   - T013 updates existing code to integrate with new traits

3. **From Quickstart** (quickstart.md):
   - Scenario 1-4 → T008 (integration test)
   - Verification workflow → T014 (cross-compiler validation)
   - Example code → T016 (quickstart validation)

4. **Ordering Applied**:
   - Setup (T001-T003) before everything
   - Tests (T004-T008) before implementation (TDD principle)
   - Implementation (T009-T013) after tests fail
   - Validation (T014-T016) after implementation complete
   - Polish (T017-T020) after validation passes

## Validation Checklist
*GATE: Verified before task execution*

- [x] All 4 contracts have corresponding test tasks (T004-T007)
- [x] All 4 contracts have implementation tasks (T009-T012)
- [x] All tests come before implementation (T004-T008 before T009-T013)
- [x] Parallel tasks truly independent (different files)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task (except T013 sequential)
- [x] Integration tests validate quickstart examples (T008, T016)
- [x] Performance benchmarks validate < 0.1% overhead (T017)
- [x] Cross-compiler validation ensures correctness (T014)
- [x] Documentation coverage meets rigor standards (T019)
