// src/ir/module.rs
use super::{Function, ScopeId};
use std::fmt;
use std::sync::Arc;

/// Describes the data layout specification for target platforms.
///
/// The data layout defines memory alignment, endianness, pointer sizes, and
/// other architecture-specific data representation details. Each variant
/// corresponds to a specific target architecture and operating system.
///
/// # Layout String Format
///
/// The layout string follows LLVM's data layout specification format:
/// - `e` = little-endian
/// - `m:` = mangling style (e=ELF, w=Windows, o=Mach-O)
/// - `p270`, `p271`, `p272` = pointer address spaces
/// - `i64:64` = 64-bit integers are 64-bit aligned
/// - `f80:128` = 80-bit floats are 128-bit aligned
/// - `n8:16:32:64` = native integer widths
/// - `S128` = stack alignment is 128 bits
///
/// # Examples
///
/// ```ignore
/// let layout = DataLayout::LinuxX86_64;
/// assert_eq!(layout.as_str(), "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataLayout {
    LinuxX86_64,
    LinuxAArch64,
    WindowsX86_64,
    MacOSX86_64,
    FreeBSDX86_64,
    NetBSDX86_64,
    OpenBSDX86_64,
    DragonFlyX86_64,
}

impl DataLayout {
    /// Returns the data layout string without allocation.
    /// This can be used in const contexts unlike Display.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::LinuxAArch64 => "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
            Self::WindowsX86_64 => "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            Self::LinuxX86_64
            | Self::FreeBSDX86_64
            | Self::NetBSDX86_64
            | Self::OpenBSDX86_64
            | Self::DragonFlyX86_64 => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            Self::MacOSX86_64 => "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
        }
    }
}

// Then Display becomes:
impl fmt::Display for DataLayout {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
/// Identifies the target triple specifying architecture, OS, and environment.
///
/// Target triples follow the format `<arch>-<vendor>-<os>-<environment>`
/// and are used throughout the compilation pipeline to enable cross-compilation
/// and platform-specific code generation.
///
/// # Supported Targets
///
/// - **`x86_64`**: 64-bit x86 architecture (Intel/AMD)
/// - **`aarch64`**: 64-bit ARM architecture (ARM64)
/// - **`i686`**: 32-bit x86 architecture
/// - **`wasm32`**: WebAssembly 32-bit
///
/// # Operating Systems
///
/// - **Linux**: GNU/Linux systems
/// - **Windows**: Windows with GNU toolchain
/// - **Darwin**: macOS/iOS (Apple platforms)
/// - **Emscripten**: WebAssembly with Emscripten runtime
///
/// # Examples
///
/// ```ignore
/// let triple = TargetTriple::X86_64UnknownLinuxGnu;
/// assert_eq!(triple.as_str(), "x86_64-unknown-linux-gnu");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetTriple {
    X86_64UnknownLinuxGnu,
    X86_64PcWindowsGnu,
    X86_64AppleDarwin,
    AArch64UnknownLinuxGnu,
    AArch64AppleDarwin,
    AArch64PcWindowsGnu,
    I686PcWindowsGnu,
    I686UnknownLinuxGnu,
    Wasm32UnknownEmscripten,
}

impl TargetTriple {
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Self::X86_64PcWindowsGnu => "x86_64-pc-windows-gnu",
            Self::X86_64AppleDarwin => "x86_64-apple-darwin",
            Self::AArch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Self::AArch64AppleDarwin => "aarch64-apple-darwin",
            Self::AArch64PcWindowsGnu => "aarch64-pc-windows-gnu",
            Self::I686PcWindowsGnu => "i686-pc-windows-gnu",
            Self::I686UnknownLinuxGnu => "i686-unknown-linux-gnu",
            Self::Wasm32UnknownEmscripten => "wasm32-unknown-emscripten",
        }
    }
}

impl fmt::Display for TargetTriple {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Represents an IR (Intermediate Representation) module.
///
/// A module is the top-level container for all IR constructs including functions,
/// global variables, and metadata. It corresponds to a single compilation unit
/// and carries target-specific information for code generation.
///
/// # Fields
///
/// * `name` - Module identifier, typically the source file name
/// * `functions` - All function definitions in this module
/// * `root_scope` - Optional root scope ID for symbol resolution
/// * `data_layout` - Target-specific data layout specification
/// * `target_triple` - Target platform triple for code generation
///
/// # Design
///
/// Modules are immutable once the root scope is set during construction.
/// This ensures consistent scoping throughout the compilation pipeline.
///
/// # Examples
///
/// ```ignore
/// use jsavrs::ir::Module;
///
/// let module = Module::new("my_program", None);
/// assert_eq!(module.name.as_ref(), "my_program");
/// ```
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Module {
    pub name: Arc<str>,
    pub functions: Vec<Function>,
    root_scope: Option<ScopeId>, // Root scope ID for the module settable only at creation
    pub data_layout: DataLayout,
    pub target_triple: TargetTriple,
}

impl Module {
    /// Creates a new module with the specified name and default settings.
    #[inline]
    pub fn new(name: impl Into<Arc<str>>, root_scope: Option<ScopeId>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            data_layout: DataLayout::LinuxX86_64,
            target_triple: TargetTriple::X86_64UnknownLinuxGnu,
            root_scope,
        }
    }

    /// Adds a function to the module.
    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    /// Sets the data layout.
    pub const fn set_data_layout(&mut self, layout: DataLayout) {
        self.data_layout = layout;
    }

    /// Sets the target triple.
    pub const fn set_target_triple(&mut self, triple: TargetTriple) {
        self.target_triple = triple;
    }

    /// Finds a function by name (immutable reference).
    #[must_use]
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name.as_ref() == name)
    }
    /// Finds a function by name (mutable reference).
    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.iter_mut().find(|f| f.name.as_ref() == name)
    }
    /// Returns all functions in the module.
    #[must_use]
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Returns the data layout.
    #[must_use]
    pub const fn data_layout(&self) -> &DataLayout {
        &self.data_layout
    }

    /// Returns the target triple.
    #[must_use]
    pub const fn target_triple(&self) -> &TargetTriple {
        &self.target_triple
    }

    /// Returns the module name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Counts the total number of instructions across all functions in the module.
    ///
    /// This method iterates through all functions in the module, then through all basic blocks
    /// in each function's control flow graph, and finally sums up the instruction counts from
    /// each basic block.
    ///
    /// # Returns
    ///
    /// The total number of instructions in the module as a `usize`. Returns `0` for empty
    /// modules or modules with no instructions.
    ///
    /// # Performance
    ///
    /// This operation has O(n) time complexity where n is the total number of instructions
    /// in the module. It performs a single linear traversal with no allocations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jsavrs::ir::Module;
    /// let module = Module::new("test_module", None);
    /// assert_eq!(module.count_instructions(), 0);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// - Empty module (no functions): returns `0`
    /// - Functions with no basic blocks: contributes `0` to the count
    /// - Basic blocks with no instructions: contributes `0` to the count
    #[inline]
    #[must_use]
    pub fn count_instructions(&self) -> usize {
        self.functions.iter().flat_map(|function| function.cfg.blocks()).map(|block| block.instructions.len()).sum()
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module {} {{", self.name)?;
        writeln!(f, "  data_layout = \"{}\";", self.data_layout)?;
        writeln!(f, "  target_triple = \"{}\";", self.target_triple)?;
        /*if let Some(rs) = self.root_scope {
            writeln!(f, "  root_scope = \"{}\";", rs)?;
        } else {
            writeln!(f, "  // root_scope: none")?;
        }*/

        if self.functions.is_empty() {
            writeln!(f, "  // No functions")?;
        } else {
            for function in &self.functions {
                let s = function.to_string();
                for line in s.trim_end_matches('\n').lines() {
                    writeln!(f, "  {line}")?;
                }
            }
        }

        write!(f, "}}")
    }
}
