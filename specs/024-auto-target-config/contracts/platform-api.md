# API Contract: Platform Detection

**Module**: `jsavrs::ir::platform`
**Exported from**: `jsavrs::ir` (via `pub use platform::platform_config_for;`)
**Feature Branch**: `024-auto-target-config`

## Types

### `PlatformConfig`

```rust
/// A consistent pair of DataLayout and TargetTriple for a single platform.
///
/// This struct is always constructed via `platform_config_for()` or
/// `detect_host_platform()`, which guarantee internal consistency
/// (i.e., the DataLayout's mangling mode always matches the TargetTriple's platform).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlatformConfig {
    /// Platform-specific data layout specification.
    pub data_layout: DataLayout,
    /// Platform identifier triple.
    pub target_triple: TargetTriple,
}
```

**Visibility**: `pub` (exported from `jsavrs::ir`)
**Derive traits**: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`

---

## Functions

### `platform_config_for`

```rust
/// Returns the platform configuration for the given OS and architecture strings.
///
/// This is a pure, deterministic function with no side effects. It maps
/// OS names (as returned by `std::env::consts::OS`) and architecture names
/// (as returned by `std::env::consts::ARCH`) to consistent
/// `(DataLayout, TargetTriple)` pairs.
///
/// # Supported Platforms
///
/// | OS | Arch | DataLayout | TargetTriple |
/// |---|---|---|---|
/// | `"windows"` | `"x86_64"` | `WindowsX86_64` | `X86_64PcWindowsGnu` |
/// | `"linux"` | `"x86_64"` | `LinuxX86_64` | `X86_64UnknownLinuxGnu` |
/// | `"macos"` | `"x86_64"` | `MacOSX86_64` | `X86_64AppleDarwin` |
///
/// # Fallback Behavior
///
/// - Recognized OS with non-`"x86_64"` arch: Returns x86_64 config for that OS.
/// - Unrecognized OS: Returns Linux x86_64 config.
///
/// Note: This function does NOT emit warnings. Warnings are emitted by
/// `detect_host_platform()` which wraps this function.
///
/// # Examples
///
/// ```
/// use jsavrs::ir::{platform_config_for, DataLayout, TargetTriple};
///
/// let config = platform_config_for("windows", "x86_64");
/// assert_eq!(config.data_layout, DataLayout::WindowsX86_64);
/// assert_eq!(config.target_triple, TargetTriple::X86_64PcWindowsGnu);
/// ```
#[must_use]
pub fn platform_config_for(os: &str, arch: &str) -> PlatformConfig;
```

**Properties**:
- **Pure**: No side effects, no I/O, no allocations
- **Deterministic**: Same inputs always produce same outputs
- **Total**: Always returns a valid `PlatformConfig` (never panics, never returns `Result`)
- **Visibility**: `pub` (exported from `jsavrs::ir`)

---

### `detect_host_platform`

```rust
/// Detects the host platform and returns the corresponding configuration.
///
/// Calls `platform_config_for()` with `std::env::consts::OS` and
/// `std::env::consts::ARCH`. Emits `eprintln!` warnings when:
/// - The host OS is not one of `"windows"`, `"linux"`, `"macos"`
/// - The host architecture is not `"x86_64"`
///
/// # Examples
///
/// ```
/// use jsavrs::ir::platform::detect_host_platform;
///
/// let config = detect_host_platform();
/// // Returns platform-appropriate config for the current host
/// ```
#[must_use]
pub fn detect_host_platform() -> PlatformConfig;
```

**Properties**:
- **Impure**: May emit `eprintln!` warnings (side effect on stderr)
- **Deterministic on a given host**: Always returns the same config for a given binary
- **Visibility**: `pub` (used internally by `Module::new()`, available for direct use)

---

## Integration Point: `Module::new()`

**Before** (0.1.0):
```rust
pub fn new(name: impl Into<Arc<str>>, root_scope: Option<ScopeId>) -> Self {
    Self {
        name: name.into(),
        functions: Vec::new(),
        data_layout: DataLayout::LinuxX86_64,              // hardcoded
        target_triple: TargetTriple::X86_64UnknownLinuxGnu, // hardcoded
        root_scope,
    }
}
```

**After** (0.2.0):
```rust
pub fn new(name: impl Into<Arc<str>>, root_scope: Option<ScopeId>) -> Self {
    let platform = crate::ir::platform::detect_host_platform();
    Self {
        name: name.into(),
        functions: Vec::new(),
        data_layout: platform.data_layout,
        target_triple: platform.target_triple,
        root_scope,
    }
}
```

**Breaking change**: `Module::new()` on non-Linux hosts will return different `data_layout` and `target_triple` values.

---

## Export Path

```rust
// src/ir/mod.rs additions:
pub mod platform;
pub use platform::platform_config_for;
```

**Public API surface**:
- `jsavrs::ir::platform::PlatformConfig`
- `jsavrs::ir::platform::platform_config_for`
- `jsavrs::ir::platform::detect_host_platform`
- `jsavrs::ir::platform_config_for` (re-exported)

---

## Warning Messages (Exact Format)

```text
warning: unsupported host OS '{os}', falling back to Linux x86_64 configuration
warning: unsupported host architecture '{arch}', using x86_64 configuration
```

Where `{os}` and `{arch}` are substituted with the actual values from `std::env::consts`.
