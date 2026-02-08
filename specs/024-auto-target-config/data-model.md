# Data Model: Automatic Target Configuration for Module

**Feature Branch**: `024-auto-target-config`
**Date**: 2026-02-08

## Entities

### PlatformConfig (NEW)

**Location**: `src/ir/platform.rs`
**Purpose**: Immutable pair of `DataLayout` and `TargetTriple` representing a complete, internally-consistent platform configuration.

| Field | Type | Description | Constraints |
|-------|------|-------------|-------------|
| `data_layout` | `DataLayout` | Platform-specific data layout (sizes, alignments, mangling) | Must be from the `DataLayout` enum (Copy, Eq, Hash) |
| `target_triple` | `TargetTriple` | Platform identifier in `arch-vendor-os-env` format | Must be from the `TargetTriple` enum (Copy, Eq, Hash) |

**Derive traits**: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`

**Invariants**:
- A `PlatformConfig` MUST always contain a consistent (DataLayout, TargetTriple) pair — i.e., the DataLayout's mangling mode matches the TargetTriple's platform.
- Only `platform_config_for()` and `detect_host_platform()` construct this type, ensuring the invariant is maintained at the point of construction.

**Validation rules**:
- No runtime validation needed — the mapping is hardcoded and exhaustive.
- The struct is `Copy`, so no ownership concerns.

---

### DataLayout (EXISTING — NO CHANGES)

**Location**: `src/ir/data_layout.rs`
**Purpose**: Enum of platform-specific LLVM data layout specifications.

| Variant | LLVM String | Mangling |
|---------|-------------|----------|
| `LinuxX86_64` | `e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128` | ELF (`e`) |
| `WindowsX86_64` | `e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128` | Windows COFF (`w`) |
| `MacOSX86_64` | `e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128` | Mach-O (`o`) |
| `LinuxAArch64` | `e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128` | ELF (`e`) |
| `FreeBSDX86_64` | *(same as LinuxX86_64)* | ELF (`e`) |
| `NetBSDX86_64` | *(same as LinuxX86_64)* | ELF (`e`) |
| `OpenBSDX86_64` | *(same as LinuxX86_64)* | ELF (`e`) |
| `DragonFlyX86_64` | *(same as LinuxX86_64)* | ELF (`e`) |

**Status**: All variants already exist. No modifications needed.

---

### TargetTriple (EXISTING — NO CHANGES)

**Location**: `src/ir/module.rs`
**Purpose**: Enum of supported target platform identifiers.

| Variant | String Representation |
|---------|----------------------|
| `X86_64UnknownLinuxGnu` | `x86_64-unknown-linux-gnu` |
| `X86_64PcWindowsGnu` | `x86_64-pc-windows-gnu` |
| `X86_64AppleDarwin` | `x86_64-apple-darwin` |
| `AArch64UnknownLinuxGnu` | `aarch64-unknown-linux-gnu` |
| `AArch64AppleDarwin` | `aarch64-apple-darwin` |
| `AArch64PcWindowsGnu` | `aarch64-pc-windows-gnu` |
| `I686PcWindowsGnu` | `i686-pc-windows-gnu` |
| `I686UnknownLinuxGnu` | `i686-unknown-linux-gnu` |
| `Wasm32UnknownEmscripten` | `wasm32-unknown-emscripten` |

**Status**: All variants already exist. No modifications needed.

---

### Module (EXISTING — MODIFIED CONSTRUCTOR)

**Location**: `src/ir/module.rs`
**Purpose**: Top-level compilation unit containing functions, data layout, and target triple.

| Field | Type | Change |
|-------|------|--------|
| `name` | `Arc<str>` | Unchanged |
| `functions` | `Vec<Function>` | Unchanged |
| `root_scope` | `Option<ScopeId>` | Unchanged (private) |
| `data_layout` | `DataLayout` | **Default value changes**: from hardcoded `LinuxX86_64` to `detect_host_platform().data_layout` |
| `target_triple` | `TargetTriple` | **Default value changes**: from hardcoded `X86_64UnknownLinuxGnu` to `detect_host_platform().target_triple` |

**State transition**: `Module::new()` → auto-detected defaults → optionally overridden via `set_data_layout()` / `set_target_triple()`

---

## Relationships

```
PlatformConfig 1:1 DataLayout
PlatformConfig 1:1 TargetTriple
Module 1:1 DataLayout (via PlatformConfig or manual override)
Module 1:1 TargetTriple (via PlatformConfig or manual override)
```

## Platform Mapping (Source of Truth)

| OS String | Arch String | DataLayout | TargetTriple | Notes |
|-----------|-------------|------------|--------------|-------|
| `"windows"` | `"x86_64"` | `WindowsX86_64` | `X86_64PcWindowsGnu` | Primary |
| `"linux"` | `"x86_64"` | `LinuxX86_64` | `X86_64UnknownLinuxGnu` | Primary |
| `"macos"` | `"x86_64"` | `MacOSX86_64` | `X86_64AppleDarwin` | Primary |
| `"windows"` | *(any other)* | `WindowsX86_64` | `X86_64PcWindowsGnu` | Arch warning emitted |
| `"linux"` | *(any other)* | `LinuxX86_64` | `X86_64UnknownLinuxGnu` | Arch warning emitted |
| `"macos"` | *(any other)* | `MacOSX86_64` | `X86_64AppleDarwin` | Arch warning emitted |
| *(any other)* | *(any)* | `LinuxX86_64` | `X86_64UnknownLinuxGnu` | OS + arch warning emitted |
