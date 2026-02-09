# Tasks: Automatic Target Configuration for Module

**Input**: Design documents from `/specs/024-auto-target-config/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Included — 14 new integration tests in `tests/auto_target_config.rs` (FR-007) and adaptation of existing tests (SC-005).

**Organization**: Tasks grouped by user story (US1–US4) to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Version bump, project-level configuration for breaking change, and dev-dependency additions

- [ ] T001 Bump crate version from 0.1.0 to 0.2.0 in Cargo.toml
- [ ] T001b [P] Add `gag` crate as dev-dependency in Cargo.toml (`gag = "1"` under `[dev-dependencies]`) for stderr capture in T028–T029 integration tests

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core platform detection module that ALL user stories depend on. Must complete before any user story work begins.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 Create `PlatformConfig` struct and `platform_config_for()` pure function in src/ir/platform.rs per contracts/platform-api.md
- [ ] T003 Implement `platform_config_with_warnings()` function with `eprintln!` warnings for unsupported OS/arch in src/ir/platform.rs per contracts/platform-api.md
- [ ] T004 Implement `detect_host_platform()` as thin wrapper calling `platform_config_with_warnings(std::env::consts::OS, std::env::consts::ARCH)` in src/ir/platform.rs per contracts/platform-api.md
- [ ] T005 Add `pub mod platform;` declaration and `pub use platform::platform_config_for;` re-export in src/ir/mod.rs
- [ ] T006 Update `Module::new()` to call `detect_host_platform()` instead of hardcoded Linux defaults in src/ir/module.rs

**Checkpoint**: `platform_config_for()` and `platform_config_with_warnings()` are public, `detect_host_platform()` works, `Module::new()` uses auto-detection. `cargo build` succeeds.

---

## Phase 3: User Story 1 — Default Module reflects host OS (Priority: P1) 🎯 MVP

**Goal**: When a developer creates a new Module without specifying any target configuration, the module automatically detects the host OS and sets DataLayout and TargetTriple to platform-correct x86_64 values.

**Independent Test**: Create a Module with default settings on each OS and verify DataLayout/TargetTriple match the expected platform values.

### Tests for User Story 1

- [ ] T007 [P] [US1] Write test `test_module_new_reflects_host_os` verifying `Module::new()` returns host-appropriate DataLayout/TargetTriple via conditional assertions on `std::env::consts::OS` in tests/auto_target_config.rs

### Existing Test Adaptation for User Story 1

- [ ] T008 [US1] Update `test_new_module_with_defaults` assertion and display string in tests/ir_module_test.rs to accept auto-detected defaults via conditional OS checks or explicit setter calls
- [ ] T009 [US1] Update `test_add_function` display string assertion in tests/ir_module_test.rs to use explicit Linux target before string comparison
- [ ] T010 [US1] Update `test_set_data_layout` initial-default assertion in tests/ir_module_test.rs to accept auto-detected defaults
- [ ] T011 [US1] Update `test_set_target_triple` initial-default assertion in tests/ir_module_test.rs to accept auto-detected defaults
- [ ] T012 [US1] Update `test_empty_module_display` display string assertion in tests/ir_module_test.rs to use explicit Linux target before string comparison
- [ ] T013 [P] [US1] Pin Module to explicit Linux target in tests/ir_dce_snapshot_tests.rs (2 call sites) after `Module::new()` calls
- [ ] T014 [P] [US1] Pin Module to explicit Linux target in tests/ir_dce_reachability_tests.rs (13 call sites) after `Module::new()` calls
- [ ] T015 [P] [US1] Pin Module to explicit Linux target in tests/ir_dce_liveness_tests.rs (4 call sites) after `Module::new()` calls
- [ ] T016 [P] [US1] Pin Module to explicit Linux target in tests/ir_dce_integration_tests.rs (9 call sites) after `Module::new()` calls
- [ ] T017 [P] [US1] Pin Module to explicit Linux target in tests/ir_dce_escape_tests.rs (5 call sites) after `Module::new()` calls
- [ ] T017b [P] [US1] Pin Module to explicit Linux target in tests/ir_generator_snapshot_tests.rs after all `generator.generate()` calls by calling `set_data_layout(DataLayout::LinuxX86_64)` + `set_target_triple(TargetTriple::X86_64UnknownLinuxGnu)` on the returned Module before snapshot assertions (59 snapshot files affected)
- [ ] T018 [US1] Regenerate snapshot files via `cargo insta test --accept` in tests/snapshots/

**Checkpoint**: `cargo test` passes on current host OS. `Module::new()` produces host-correct config. All existing tests pass with zero regressions (SC-005).

---

## Phase 4: User Story 2 — Deterministic testing across all platforms (Priority: P1)

**Goal**: Developers can test all three platform configurations (Windows, Linux, macOS) from any single host OS in a single test run by calling the pure `platform_config_for()` function.

**Independent Test**: Invoke `platform_config_for("windows"|"linux"|"macos", "x86_64")` and assert correct config returned, all within one test run on any OS.

### Tests for User Story 2

- [ ] T019 [P] [US2] Write test `test_platform_config_for_windows` calling `platform_config_for("windows", "x86_64")` and asserting `WindowsX86_64` + `X86_64PcWindowsGnu` in tests/auto_target_config.rs
- [ ] T020 [P] [US2] Write test `test_platform_config_for_linux` calling `platform_config_for("linux", "x86_64")` and asserting `LinuxX86_64` + `X86_64UnknownLinuxGnu` in tests/auto_target_config.rs
- [ ] T021 [P] [US2] Write test `test_platform_config_for_macos` calling `platform_config_for("macos", "x86_64")` and asserting `MacOSX86_64` + `X86_64AppleDarwin` in tests/auto_target_config.rs

**Checkpoint**: All 3 deterministic platform tests pass from any single host OS (SC-004).

---

## Phase 5: User Story 3 — DataLayout and TargetTriple consistency (Priority: P2)

**Goal**: For every supported platform, the returned DataLayout and TargetTriple always form a valid, documented pair. A Windows TargetTriple is never paired with a Linux DataLayout.

**Independent Test**: Assert that for each of the three platforms, the DataLayout mangling mode corresponds to the TargetTriple's platform.

### Tests for User Story 3

- [ ] T022 [P] [US3] Write test `test_windows_config_consistency` asserting `WindowsX86_64` DataLayout pairs with `X86_64PcWindowsGnu` TargetTriple and data layout string contains `m:w` in tests/auto_target_config.rs
- [ ] T023 [P] [US3] Write test `test_linux_config_consistency` asserting `LinuxX86_64` DataLayout pairs with `X86_64UnknownLinuxGnu` TargetTriple and data layout string contains `m:e` in tests/auto_target_config.rs
- [ ] T024 [P] [US3] Write test `test_macos_config_consistency` asserting `MacOSX86_64` DataLayout pairs with `X86_64AppleDarwin` TargetTriple and data layout string contains `m:o` in tests/auto_target_config.rs

**Checkpoint**: All 3 consistency tests pass, confirming FR-003.

---

## Phase 6: User Story 4 — Manual override remains available (Priority: P3)

**Goal**: After automatic detection, developers can still explicitly set DataLayout and TargetTriple via `set_data_layout()` / `set_target_triple()` to override defaults for cross-compilation.

**Independent Test**: Create a Module (auto-detected defaults), call manual setters with Linux values, verify overridden values persist regardless of host OS.

### Tests for User Story 4

- [ ] T025 [US4] Write test `test_manual_override_after_auto_detection` creating Module, calling `set_data_layout(LinuxX86_64)` + `set_target_triple(X86_64UnknownLinuxGnu)`, and asserting overrides persist in tests/auto_target_config.rs

**Checkpoint**: Manual override test passes on all platforms (SC-006).

---

## Phase 7: Edge Cases

**Goal**: Verify fallback behavior for unsupported OS and unsupported architecture per FR-005 and spec edge cases.

### Tests for Edge Cases

- [ ] T026 [P] Write test `test_unsupported_os_falls_back_to_linux` calling `platform_config_for("freebsd", "x86_64")` and asserting `LinuxX86_64` + `X86_64UnknownLinuxGnu` in tests/auto_target_config.rs
- [ ] T027 [P] Write test `test_unsupported_arch_uses_x86_64_config_linux` calling `platform_config_for("linux", "aarch64")` and asserting `LinuxX86_64` + `X86_64UnknownLinuxGnu` in tests/auto_target_config.rs
- [ ] T027b [P] Write test `test_unsupported_arch_uses_x86_64_config_macos` calling `platform_config_for("macos", "aarch64")` and asserting `MacOSX86_64` + `X86_64AppleDarwin` in tests/auto_target_config.rs (validates spec edge case: unsupported arch returns x86_64 variant for the **detected** OS, not Linux fallback)
- [ ] T027c [P] Write test `test_unsupported_arch_uses_x86_64_config_windows` calling `platform_config_for("windows", "aarch64")` and asserting `WindowsX86_64` + `X86_64PcWindowsGnu` in tests/auto_target_config.rs
- [ ] T028 [P] Write test `test_unsupported_os_emits_stderr_warning` calling `platform_config_with_warnings("freebsd", "x86_64")` and capturing stderr to assert output contains `"warning: unsupported host OS 'freebsd'"` in tests/auto_target_config.rs. **Stderr capture strategy**: use `gag` crate (`BufferRedirect::stderr()`) added as dev-dependency to capture `eprintln!` output in-process.
- [ ] T029 [P] Write test `test_unsupported_arch_emits_stderr_warning` calling `platform_config_with_warnings("linux", "aarch64")` and capturing stderr to assert output contains `"warning: unsupported host architecture 'aarch64'"` in tests/auto_target_config.rs. **Stderr capture strategy**: use `gag` crate (`BufferRedirect::stderr()`) added as dev-dependency to capture `eprintln!` output in-process.

**Checkpoint**: All 6 edge case tests pass, confirming FR-005 fallback behavior, OS-aware architecture fallback, **and** warning emission.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates, documentation, and CI validation

- [ ] T030 Run `cargo fmt --check` to verify formatting compliance across all modified files
- [ ] T031 Run `cargo clippy -- -D warnings` to verify zero clippy warnings across all modified files
- [ ] T032 Run full `cargo test` to confirm zero regressions across entire test suite
- [ ] T033 Validate quickstart.md scenarios by running `cargo test --test auto_target_config` in specs/024-auto-target-config/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1 (Setup)          → No dependencies — start immediately
Phase 2 (Foundational)   → Depends on Phase 1 — BLOCKS all user stories
Phase 3 (US1 - P1) 🎯   → Depends on Phase 2
Phase 4 (US2 - P1)       → Depends on Phase 2 — can parallel with Phase 3
Phase 5 (US3 - P2)       → Depends on Phase 2 — can parallel with Phase 3/4
Phase 6 (US4 - P3)       → Depends on Phase 2 — can parallel with Phase 3/4/5
Phase 7 (Edge Cases)     → Depends on Phase 2 — can parallel with Phase 3–6
Phase 8 (Polish)         → Depends on ALL previous phases
```

### User Story Dependencies

- **US1 (P1)**: Depends on Foundational only. Includes existing test adaptation — largest phase.
- **US2 (P1)**: Depends on Foundational only. Independent of US1 (tests the pure function, not Module).
- **US3 (P2)**: Depends on Foundational only. Independent of US1/US2.
- **US4 (P3)**: Depends on Foundational only. Independent of US1/US2/US3.
- **Edge Cases**: Depend on Foundational only. Independent of all user stories.

### Within Each User Story

Tests are written alongside implementation. For this feature, the foundational module (Phase 2) IS the implementation — user story phases are primarily test-focused.

### Parallel Opportunities

**Phase 2** (sequential — each task depends on previous):

- T002 → T003 → T004 → T005 → T006

**Phase 3** (partial parallel after T008–T012 sequential group):

- T013, T014, T015, T016, T017, T017b can all run in parallel (different test files)
- T007 can run in parallel with T013–T017b
- T018 (snapshot regeneration) must run LAST in this phase

**Phase 4** (fully parallel):

- T019, T020, T021 can all run in parallel (same file, independent tests)

**Phase 5** (fully parallel):

- T022, T023, T024 can all run in parallel

**Phases 3–7** can all run in parallel with each other (after Phase 2 completes).

---

## Parallel Example: After Phase 2 Completes

```text
Thread A (US1 existing tests):     T008 → T009 → T010 → T011 → T012
Thread B (US1 test pin — batch 1): T013, T014, T015 (parallel, diff files)
Thread C (US1 test pin — batch 2): T016, T017, T017b (parallel, diff files)
Thread D (US2 + US3 + US4 + Edge): T019–T021, T022–T024, T025, T026–T029 (all parallel, same new file)

After all above complete:
Thread A: T018 (snapshot regeneration)
Thread A: T030 → T031 → T032 → T033 (polish, sequential)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001 — version bump)
2. Complete Phase 2: Foundational (T002–T006 — platform module + Module::new() change)
3. Complete Phase 3: US1 (T007–T018 incl. T017b — host-OS detection tests + existing test adaptation)
4. **STOP and VALIDATE**: `cargo test` passes on current host with zero regressions
5. This is a shippable increment — Module::new() works correctly on host OS

### Incremental Delivery

1. Setup + Foundational → Core platform detection works (T001–T006)
2. US1 → Host-OS auto-detection tested + existing tests adapted (T007–T018) **MVP!**
3. US2 → Deterministic cross-platform testing validated (T019–T021)
4. US3 → Consistency invariant validated (T022–T024)
5. US4 → Manual override confirmed (T025)
6. Edge Cases → Fallback behavior + warning emission confirmed (T026–T029 incl. T027b–T027c)
7. Polish → Full quality gate passed (T030–T033)

### Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable after Phase 2
- Commit after each phase or logical group
- Total: 37 tasks (2 setup + 5 foundational + 13 US1 + 3 US2 + 3 US3 + 1 US4 + 6 edge + 4 polish)
- Total new test count: 14 (1 US1 + 3 US2 + 3 US3 + 1 US4 + 6 edge)
