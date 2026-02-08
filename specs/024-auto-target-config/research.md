# Research: Automatic Target Configuration for Module

**Feature Branch**: `024-auto-target-config`
**Date**: 2026-02-08
**Status**: Complete — All items resolved

## Research Task 1: LLVM Data Layout String Differences Across Platforms

**Context**: Verify that the existing `DataLayout` variants in `src/ir/data_layout.rs` use LLVM-standard data layout strings with the correct mangling mode per platform.

**Decision**: The existing variants are correct and require no modification.

**Rationale**: The three x86_64 platforms differ only in the mangling prefix within the data layout string:
- **Linux x86_64**: `e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128` — ELF mangling (`m:e`)
- **Windows x86_64**: `e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128` — Windows COFF mangling (`m:w`)
- **macOS x86_64**: `e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128` — Mach-O mangling (`m:o`)

These match LLVM 18 defaults for `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-gnu`, and `x86_64-apple-darwin` respectively. All three are already present in the `DataLayout` enum as `LinuxX86_64`, `WindowsX86_64`, and `MacOSX86_64`.

**Alternatives considered**:
- Storing data layout as a raw `String` — rejected because enum variants are `Copy`, zero-allocation, and exhaustively matchable.
- Adding new variants — unnecessary, all three already exist.

---

## Research Task 2: `std::env::consts` Values Per Platform

**Context**: Determine the exact string values returned by `std::env::consts::OS` and `std::env::consts::ARCH` on each target platform.

**Decision**: Use direct string matching on `std::env::consts::OS` and `std::env::consts::ARCH`.

**Rationale**: Rust's `std::env::consts` provides compile-time constants (not runtime detection), which are stable across Rust editions:

| Platform | `std::env::consts::OS` | `std::env::consts::ARCH` |
|----------|----------------------|------------------------|
| Windows | `"windows"` | `"x86_64"` |
| Linux | `"linux"` | `"x86_64"` |
| macOS | `"macos"` | `"x86_64"` |
| FreeBSD | `"freebsd"` | `"x86_64"` |
| Apple Silicon macOS | `"macos"` | `"aarch64"` |

These are `&'static str` values, making them ideal for a `match` expression. The lowercase format is guaranteed by the Rust standard library documentation.

**Alternatives considered**:
- `cfg!(target_os = ...)` — rejected for the core logic because it cannot be tested deterministically across platforms from a single host. The `cfg!` approach would be used implicitly via `std::env::consts::OS` but the pure function takes `&str` parameters for testability.
- `#[cfg(target_os)]` conditional compilation — rejected because it prevents testing all platform paths from a single machine.

---

## Research Task 3: Function Purity and Testability Design

**Context**: Design a function that enables deterministic testing of all three platform configurations from any single host OS (FR-004, US-2).

**Decision**: Implement a two-layer design:
1. **Pure function**: `platform_config_for(os: &str, arch: &str) -> PlatformConfig` — deterministic, testable, no side effects.
2. **Wrapper**: `detect_host_platform() -> PlatformConfig` — calls `platform_config_for` with `std::env::consts::OS` and `std::env::consts::ARCH`.

**Rationale**: The pure function accepts string parameters and returns a value without performing I/O, enabling tests to call it with any OS/arch combination. The `eprintln!` warnings for unsupported platforms are emitted inside `detect_host_platform()` (the wrapper), NOT inside the pure function, so that the pure function remains side-effect-free. This is essential for FR-004 and US-2: tests can verify all three platforms without triggering `eprintln!` output.

**Alternatives considered**:
- Single function with `cfg!(target_os)` branches — rejected because it's untestable for non-host platforms.
- Trait-based abstraction with mock — rejected as over-engineered for a simple mapping function. The two-layer approach achieves testability with zero complexity.
- Returning `Option<PlatformConfig>` for unsupported platforms — rejected because FR-005 mandates fallback to Linux, not an error. The function always returns a valid config.

---

## Research Task 4: Snapshot Test Adaptation Strategy

**Context**: 63 snapshot files in `tests/snapshots/` contain hardcoded Linux `data_layout` and `target_triple` strings. After `Module::new()` uses auto-detection, snapshots will break on non-Linux hosts.

**Decision**: Adopt a hybrid strategy:
1. **IR generator snapshot tests** (`ir_generator_snapshot_tests`): These use the `IrGenerator` which calls `Module::new()` internally. Since these tests run through the full compilation pipeline, they must accept host-dependent output. The snapshots will be regenerated per-platform in CI. Use `insta`'s `#[cfg]`-based snapshot suffixes or redact the data_layout/target_triple lines.
2. **DCE snapshot tests** (`ir_dce_snapshot_tests`): These create `Module::new()` directly. Modify these tests to explicitly set Linux target via `set_data_layout()` / `set_target_triple()` after creation, making them OS-independent.
3. **Module unit tests** (`ir_module_test.rs`): Update assertions to use conditional checks based on `std::env::consts::OS` or explicitly set the platform before assertions.

**Rationale**: Strategy 2 (explicit setter in DCE tests) is preferred for most snapshot tests because it makes tests deterministic regardless of host. Strategy 1 (per-platform regeneration) is needed only for tests that must validate the full auto-detection pipeline. The recommended approach is to **explicitly set Linux targets in existing snapshot tests** so they remain deterministic, and let the new `auto_target_config.rs` tests cover the auto-detection behavior.

**Alternatives considered**:
- `insta` snapshot suffixes per OS — rejected as too complex, produces 3× snapshot files, hard to review.
- Redacting data_layout/target_triple from all snapshots — rejected because these are important to validate.
- Reverting to hardcoded Linux in `Module::new()` and requiring callers to detect — rejected because it defeats the purpose of the feature.

---

## Research Task 5: Semver Impact and Version Bump

**Context**: Changing `Module::new()` default behavior is a breaking change for callers who relied on Linux defaults.

**Decision**: Bump version from `0.1.0` to `0.2.0`.

**Rationale**: Per semver, for versions `0.x.y`, the minor version (`x`) serves as the breaking change indicator. Since the project is at `0.1.0`, bumping to `0.2.0` correctly signals a breaking API change. The change is breaking because:
- `Module::new()` on Windows/macOS no longer returns Linux-specific DataLayout/TargetTriple.
- Existing code that assumed `module.data_layout() == DataLayout::LinuxX86_64` after `Module::new()` will fail on non-Linux hosts.

**Alternatives considered**:
- `1.0.0` — rejected because the project is pre-1.0 and not ready for stability guarantees.
- `0.1.1` (patch) — rejected because this is a behavioral breaking change, not a bugfix.
- Keeping `0.1.0` — rejected because semver requires signaling breaking changes.

---

## Research Task 6: Warning Emission Strategy for Unsupported Platforms

**Context**: FR-005 requires warning on unsupported OS; the spec also requires warning on non-x86_64 architecture.

**Decision**: Emit warnings via `eprintln!` in `detect_host_platform()` only (not in the pure function).

**Rationale**: The warning must be emitted when actual host detection occurs, not during deterministic testing. By placing `eprintln!` in the wrapper function:
- The pure function remains side-effect-free and testable.
- Warnings are emitted exactly once per program run (at Module creation time).
- Warning format: `"warning: unsupported host OS '{os}', falling back to Linux x86_64 configuration"` and `"warning: unsupported host architecture '{arch}', using x86_64 configuration"`.

**Alternatives considered**:
- `log` crate — rejected per zero-external-dependency constraint for new code.
- Returning `Result` with warnings — over-engineered; `eprintln!` matches the spec requirement.
- Warning inside `platform_config_for()` — rejected because it would fire during deterministic tests.

---

## Research Task 7: Existing Test File Impact Assessment

**Context**: Quantify which test files and assertions need modification.

**Decision**: The following files require changes:

### Files with direct Linux-default assertions (MUST modify)

| File | Lines | Change Type |
|------|-------|-------------|
| `tests/ir_module_test.rs` | ~22-39, ~96, ~121, ~194, ~44-57, ~108-115 | Change assertions to use conditional OS checks or explicit setter calls |

### Files with Module::new() but no default assertions (SET explicit target)

| File | Calls | Change Type |
|------|-------|-------------|
| `tests/ir_dce_snapshot_tests.rs` | 2 | Add `set_data_layout(LinuxX86_64)` + `set_target_triple(X86_64UnknownLinuxGnu)` after `Module::new()` |
| `tests/ir_dce_reachability_tests.rs` | 12 | Same pattern |
| `tests/ir_dce_liveness_tests.rs` | 4 | Same pattern |
| `tests/ir_dce_integration_tests.rs` | 9 | Same pattern |
| `tests/ir_dce_escape_tests.rs` | 5 | Same pattern |

### Snapshot files (REGENERATE after test fixes)

63 snapshot files in `tests/snapshots/` containing `x86_64-unknown-linux-gnu` and the Linux data layout string. These will be automatically updated by `cargo insta review` after the DCE/module tests are fixed to use explicit Linux targets.

**Alternatives considered**: N/A — this is an inventory, not a design decision.
