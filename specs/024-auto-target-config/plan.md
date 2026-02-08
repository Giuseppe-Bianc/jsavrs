# Implementation Plan: Automatic Target Configuration for Module

**Branch**: `024-auto-target-config` | **Date**: 2026-02-08 | **Spec**: [spec.md](specs/024-auto-target-config/spec.md)
**Input**: Feature specification from `/specs/024-auto-target-config/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement automatic host-OS detection for `Module` construction so that `DataLayout` and `TargetTriple` fields are set to platform-correct values (Windows, Linux, macOS on x86_64) instead of hardcoded Linux defaults. A new pure function `platform_config_for(os, arch)` in `src/ir/platform.rs` provides deterministic, testable mapping; a wrapper `detect_host_platform()` calls it with `std::env::consts::OS` and `std::env::consts::ARCH`. The `Module::new()` constructor is updated to call `detect_host_platform()`. Unsupported OS/arch combinations fall back to Linux x86_64 with `eprintln!` warnings. This is a **semver-breaking change** (version 0.1.0 → 0.2.0). Existing tests and 63 snapshot files must be adapted.

## Technical Context

**Language/Version**: Rust 1.93.0 (edition 2024)
**Primary Dependencies**: None for new code (stdlib only: `std::env::consts::OS`, `std::env::consts::ARCH`)
**Storage**: N/A
**Testing**: `cargo test` + `insta` snapshots (dev-dependency)
**Target Platform**: Windows x86_64, Linux x86_64, macOS x86_64 (CI: ubuntu-latest, windows-latest, macos-latest)
**Project Type**: Single Rust crate (compiler)
**Performance Goals**: Zero runtime overhead — `platform_config_for` is a pure `const`-eligible function returning `Copy` enums
**Constraints**: Zero external dependencies for new module; breaking change requires semver major bump
**Scale/Scope**: 1 new file (`src/ir/platform.rs`, ~80 LOC), 1 modified file (`src/ir/module.rs`, ~5 lines changed), 1 modified file (`src/ir/mod.rs`, 2 lines added), 1 new test file (`tests/auto_target_config.rs`, ~200 LOC), 6+ existing test assertions updated in `tests/ir_module_test.rs`, 63 snapshot files regenerated, `Cargo.toml` version bump

### Existing Codebase State

| Entity | File | Status |
|--------|------|--------|
| `DataLayout::WindowsX86_64` | `src/ir/data_layout.rs:414` | Already exists with correct LLVM string `e-m:w-...` |
| `DataLayout::MacOSX86_64` | `src/ir/data_layout.rs:415` | Already exists with correct LLVM string `e-m:o-...` |
| `DataLayout::LinuxX86_64` | `src/ir/data_layout.rs:412` | Already exists with correct LLVM string `e-m:e-...` |
| `TargetTriple::X86_64PcWindowsGnu` | `src/ir/module.rs:36` | Already exists |
| `TargetTriple::X86_64AppleDarwin` | `src/ir/module.rs:37` | Already exists |
| `TargetTriple::X86_64UnknownLinuxGnu` | `src/ir/module.rs:35` | Already exists |
| `Module::new()` | `src/ir/module.rs:109-118` | Hardcoded to `LinuxX86_64` — **must change** |
| `Module::set_data_layout()` | `src/ir/module.rs:129` | Already exists — no change needed |
| `Module::set_target_triple()` | `src/ir/module.rs:132` | Already exists — no change needed |
| IR generator call site | `src/ir/generator.rs:248` | Only production call to `Module::new()` — inherits fix |

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Applicable? | Status | Notes |
|-----------|:-----------:|:------:|-------|
| **Safety First** | Yes | PASS | No `unsafe` code. Pure function with `Copy` types. No new allocations, no panics. |
| **Performance Excellence** | Yes | PASS | `platform_config_for` is `const`-eligible, returns `Copy` enums. Zero runtime cost vs. current hardcoded approach. |
| **Cross-Platform Compatibility** | Yes | PASS | This feature IS the cross-platform fix — replaces incorrect Linux-only default. CI on 3 platforms. |
| **Modular Extensibility** | Yes | PASS | New `platform.rs` module follows existing pattern. Pure function easily extendable for new platforms/architectures. |
| **Test-Driven Reliability** | Yes | PASS | 11 new integration tests + adaptation of existing tests. Deterministic testing of all 3 platforms from any host. |
| **Snapshot Validation** | Yes | PASS | 63 snapshot files will be regenerated via `cargo insta review`. Snapshot content becomes OS-dependent or tests set explicit targets. |
| **Documentation Rigor** | Yes | PASS | Full rustdoc on all public items. research.md + data-model.md generated. |
| **Code Quality Standards** | Yes | PASS | `cargo fmt` + `cargo clippy` compliance. No new warnings. |

**Gate Result**: **PASS** — No violations. Proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/024-auto-target-config/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── platform-api.md  # Public API contract for platform detection
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── ir/
│   ├── mod.rs           # MODIFIED: add `pub mod platform;` + re-export
│   ├── module.rs        # MODIFIED: Module::new() calls detect_host_platform()
│   ├── platform.rs      # NEW: PlatformConfig, platform_config_for(), detect_host_platform()
│   ├── data_layout.rs   # UNCHANGED: variants already exist
│   └── ...
tests/
├── auto_target_config.rs  # NEW: 11 integration tests
├── ir_module_test.rs      # MODIFIED: adapt 6+ assertions for auto-detected defaults
├── snapshots/             # MODIFIED: 63 snapshot files regenerated
└── ...
Cargo.toml                 # MODIFIED: version 0.1.0 → 0.2.0
```

**Structure Decision**: Single project (existing Rust crate). New `platform.rs` is added to the existing `src/ir/` module hierarchy following the established pattern of one-file-per-concern. Tests go in `tests/` per FR-007.

## Complexity Tracking

> No Constitution Check violations — this section is empty.

| Violation | Why Needed | Simpler Alternative Rejected Because |
| ----------- | ------------ | ------------------------------------- |
| *(none)* | — | — |
