# Quickstart: Automatic Target Configuration for Module

**Feature Branch**: `024-auto-target-config`

## What Changed

`Module::new()` now automatically detects the host OS and sets `DataLayout` and `TargetTriple` to platform-correct values instead of always defaulting to Linux x86_64.

## For Users

### Before (0.1.0)

```rust
// On Windows, this silently produced Linux-targeted output:
let module = Module::new("my_program", None);
assert_eq!(*module.target_triple(), TargetTriple::X86_64UnknownLinuxGnu); // Always Linux!
```

### After (0.2.0)

```rust
// On Windows, this now correctly produces Windows-targeted output:
let module = Module::new("my_program", None);
// On Windows: module.target_triple() == TargetTriple::X86_64PcWindowsGnu
// On Linux:   module.target_triple() == TargetTriple::X86_64UnknownLinuxGnu
// On macOS:   module.target_triple() == TargetTriple::X86_64AppleDarwin
```

### Cross-Compilation (Manual Override)

The existing `set_data_layout()` and `set_target_triple()` methods still work:

```rust
let mut module = Module::new("cross_compile", None);
// Override to target Linux regardless of host
module.set_data_layout(DataLayout::LinuxX86_64);
module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
```

### Deterministic Platform Testing

Use `platform_config_for()` to test any platform from any host:

```rust
use jsavrs::ir::platform_config_for;
use jsavrs::ir::{DataLayout, TargetTriple};

// Test Windows config from a Linux machine:
let config = platform_config_for("windows", "x86_64");
assert_eq!(config.data_layout, DataLayout::WindowsX86_64);
assert_eq!(config.target_triple, TargetTriple::X86_64PcWindowsGnu);
```

## For Test Authors

### Existing Tests That Create Modules

If your test creates a `Module::new()` and asserts on the string output (snapshot or direct), the data_layout/target_triple fields will now vary by host OS. To keep your test deterministic:

```rust
let mut module = Module::new("test_module", Some(scope_id));
// Pin to Linux for deterministic output:
module.set_data_layout(DataLayout::LinuxX86_64);
module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
```

### New Tests for Auto-Detection

The new test file `tests/auto_target_config.rs` covers:

1. Deterministic mapping for all 3 platforms (3 tests)
2. Host-OS detection via `Module::new()` (1 test)
3. Manual override preservation (1 test)
4. DataLayout/TargetTriple consistency per platform (3 tests)
5. Edge cases: unsupported OS and unsupported arch (2 tests)

## Build & Test

```bash
# Run all tests including the new auto_target_config tests
cargo test

# Run only the new feature tests
cargo test --test auto_target_config

# CI must run on all three platforms:
# ubuntu-latest, windows-latest, macos-latest
```
