use std::fmt;

/// Endianness (E / e)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

/// Symbol mangling style `(m:<style>)`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mangling {
    Elf,
    MachO,
    Coff,
    Unknown(String),
}

/// Pointer layout for a specific address space
#[derive(Debug, Clone)]
pub struct PointerLayout {
    /// Pointer size in bits
    pub size_bits: u32,

    /// ABI alignment in bits
    pub abi_align_bits: u32,

    /// Address space number (0 = default)
    pub address_space: u32,

    /// Preferred alignment in bits (None => same as ABI)
    pub pref_align_bits: Option<u32>,
}

/// Integer type layout (i8, i16, i32, i64, ...)
#[derive(Debug, Clone)]
pub struct IntegerLayout {
    /// Integer width in bits (e.g. 8, 16, 32, 64)
    pub size_bits: u32,

    /// ABI alignment in bits
    pub abi_align_bits: u32,

    /// Preferred alignment in bits (None => same as ABI)
    pub pref_align_bits: Option<u32>,
}

/// Floating-point type layout (f32, f64, f80, f128)
#[derive(Debug, Clone)]
pub struct FloatLayout {
    /// Float size in bits
    pub size_bits: u32,

    /// ABI alignment in bits
    pub abi_align_bits: u32,

    /// Preferred alignment in bits
    pub pref_align_bits: Option<u32>,
}

/// Vector type layout (e.g. v128)
#[derive(Debug, Clone)]
pub struct VectorLayout {
    /// Vector size in bits
    pub size_bits: u32,

    /// ABI alignment in bits
    pub abi_align_bits: u32,

    /// Preferred alignment in bits
    pub pref_align_bits: Option<u32>,
}

/// Aggregate (struct/array) alignment rules
#[derive(Debug, Clone)]
pub struct AggregateLayout {
    /// ABI alignment in bits (0 => use max field alignment)
    pub abi_align_bits: u32,

    /// Preferred alignment in bits
    pub pref_align_bits: Option<u32>,
}

/// Function pointer alignment (e.g. Fi8)
#[derive(Debug, Clone)]
pub struct FunctionPointerLayout {
    /// ABI alignment in bits
    pub abi_align_bits: u32,
}

/// Native integer register widths
#[derive(Debug, Clone)]
pub struct NativeIntWidths {
    /// Supported native integer sizes in bits
    pub widths_bits: Vec<u32>,
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedDataLayout {
    pointer_layouts: HashMap<u32, PointerLayout>,
    integer_layouts: HashMap<u32, IntegerLayout>,
    float_layouts: HashMap<u32, FloatLayout>,
    vector_layouts: HashMap<u32, VectorLayout>,
    mangling: Option<Mangling>,
    native_int_widths: Option<NativeIntWidths>,
    aggregate_layout: Option<AggregateLayout>,
    function_pointer_layout: Option<FunctionPointerLayout>,
    stack_align_bits: Option<u32>,
    endianness: Endianness,
}

impl ParsedDataLayout {
    // Public immutable accessors
    #[must_use]
    pub const fn endianness(&self) -> Endianness {
        self.endianness
    }
    #[must_use]
    pub const fn mangling(&self) -> Option<&Mangling> {
        self.mangling.as_ref()
    }
    #[must_use]
    pub const fn pointer_layouts(&self) -> &HashMap<u32, PointerLayout> {
        &self.pointer_layouts
    }
    #[must_use]
    pub const fn integer_layouts(&self) -> &HashMap<u32, IntegerLayout> {
        &self.integer_layouts
    }
    #[must_use]
    pub const fn float_layouts(&self) -> &HashMap<u32, FloatLayout> {
        &self.float_layouts
    }
    #[must_use]
    pub const fn vector_layouts(&self) -> &HashMap<u32, VectorLayout> {
        &self.vector_layouts
    }
    #[must_use]
    pub const fn aggregate_layout(&self) -> Option<&AggregateLayout> {
        self.aggregate_layout.as_ref()
    }
    #[must_use]
    pub const fn function_pointer_layout(&self) -> Option<&FunctionPointerLayout> {
        self.function_pointer_layout.as_ref()
    }
    #[must_use]
    pub const fn native_int_widths(&self) -> Option<&NativeIntWidths> {
        self.native_int_widths.as_ref()
    }
    #[must_use]
    pub const fn stack_align_bits(&self) -> Option<u32> {
        self.stack_align_bits
    }

    /// Parse a data layout string into a `ParsedDataLayout`
    ///
    /// # Errors
    ///
    /// Returns an error if the data layout string contains invalid specifications
    /// or has an invalid format
    #[allow(clippy::too_many_lines)]
    pub fn parse(layout_str: &str) -> Result<Self, String> {
        let mut endianness = Endianness::Little;
        let mut mangling = None;
        let mut pointer_layouts = HashMap::new();
        let mut integer_layouts = HashMap::new();
        let mut float_layouts = HashMap::new();
        let mut vector_layouts = HashMap::new();
        let mut aggregate_layout = None;
        let mut function_pointer_layout = None;
        let mut native_int_widths = None;
        let mut stack_align_bits = None;

        for spec in layout_str.split('-') {
            let spec = spec.trim();
            if spec.is_empty() {
                continue;
            }

            match spec.chars().next() {
                Some('e') if spec.len() == 1 => {
                    endianness = Endianness::Little;
                }
                Some('E') if spec.len() == 1 => {
                    endianness = Endianness::Big;
                }
                Some('m') if spec.starts_with("m:") => {
                    let style = &spec[2..];
                    mangling = Some(match style {
                        "e" => Mangling::Elf,
                        "o" => Mangling::MachO,
                        "w" => Mangling::Coff,
                        _ => Mangling::Unknown(style.to_string()),
                    });
                }
                Some('p') => {
                    // Pointer layout: p[addr_space]:<size>:<abi>[:<pref>]
                    let parts: Vec<&str> = spec.split(':').collect();
                    if parts.len() < 3 {
                        return Err(format!("Invalid pointer layout: {spec}"));
                    }

                    let addr_space = parts[0][1..].parse().unwrap_or(0);
                    let size = parts[1].parse().map_err(|_| format!("Invalid size in {spec}"))?;
                    let abi = parts[2].parse().map_err(|_| format!("Invalid ABI in {spec}"))?;
                    let pref = parts.get(3).and_then(|s| s.parse().ok());

                    pointer_layouts.insert(
                        addr_space,
                        PointerLayout {
                            address_space: addr_space,
                            size_bits: size,
                            abi_align_bits: abi,
                            pref_align_bits: pref,
                        },
                    );
                }
                Some('i') => {
                    // Integer layout: i<size>:<abi>[:<pref>]
                    let parts: Vec<&str> = spec.split(':').collect();
                    if parts.len() < 2 {
                        return Err(format!("Invalid integer layout: {spec}"));
                    }

                    let size = parts[0][1..].parse().map_err(|_| format!("Invalid size in {spec}"))?;
                    let abi = parts[1].parse().map_err(|_| format!("Invalid ABI in {spec}"))?;
                    let pref = parts.get(2).and_then(|s| s.parse().ok());

                    integer_layouts
                        .insert(size, IntegerLayout { size_bits: size, abi_align_bits: abi, pref_align_bits: pref });
                }
                Some('f') => {
                    // Float layout: f<size>:<abi>[:<pref>]
                    let parts: Vec<&str> = spec.split(':').collect();
                    if parts.len() < 2 {
                        return Err(format!("Invalid float layout: {spec}"));
                    }

                    let size = parts[0][1..].parse().map_err(|_| format!("Invalid size in {spec}"))?;
                    let abi = parts[1].parse().map_err(|_| format!("Invalid ABI in {spec}"))?;
                    let pref = parts.get(2).and_then(|s| s.parse().ok());

                    float_layouts
                        .insert(size, FloatLayout { size_bits: size, abi_align_bits: abi, pref_align_bits: pref });
                }
                Some('v') => {
                    // Vector layout: v<size>:<abi>[:<pref>]
                    let parts: Vec<&str> = spec.split(':').collect();
                    if parts.len() < 2 {
                        return Err(format!("Invalid vector layout: {spec}"));
                    }

                    let size = parts[0][1..].parse().map_err(|_| format!("Invalid size in {spec}"))?;
                    let abi = parts[1].parse().map_err(|_| format!("Invalid ABI in {spec}"))?;
                    let pref = parts.get(2).and_then(|s| s.parse().ok());

                    vector_layouts
                        .insert(size, VectorLayout { size_bits: size, abi_align_bits: abi, pref_align_bits: pref });
                }
                Some('a') if spec.starts_with("a:") => {
                    // Aggregate layout: a:<abi>[:<pref>]
                    let parts: Vec<&str> = spec.split(':').collect();
                    if parts.len() < 2 {
                        return Err(format!("Invalid aggregate layout: {spec}"));
                    }

                    let abi = parts[1].parse().map_err(|_| format!("Invalid ABI in {spec}"))?;
                    let pref = parts.get(2).and_then(|s| s.parse().ok());

                    aggregate_layout = Some(AggregateLayout { abi_align_bits: abi, pref_align_bits: pref });
                }
                Some('F') if spec.starts_with("Fi") => {
                    // Function pointer: Fi<abi>
                    let abi = spec[2..].parse().map_err(|_| format!("Invalid function pointer: {spec}"))?;
                    function_pointer_layout = Some(FunctionPointerLayout { abi_align_bits: abi });
                }
                Some('n') => {
                    // Native int widths: n<w1>:<w2>:...
                    let widths: Result<Vec<u32>, _> = spec[1..].split(':').map(str::parse).collect();
                    native_int_widths = Some(NativeIntWidths {
                        widths_bits: widths.map_err(|_| format!("Invalid native widths: {spec}"))?,
                    });
                }
                Some('S') => {
                    // Stack alignment: S<align>
                    stack_align_bits = Some(spec[1..].parse().map_err(|_| format!("Invalid stack alignment: {spec}"))?);
                }
                _ => {
                    // Unknown specification - can be silently ignored or warned
                }
            }
        }

        Ok(Self {
            pointer_layouts,
            integer_layouts,
            float_layouts,
            vector_layouts,
            mangling,
            native_int_widths,
            aggregate_layout,
            function_pointer_layout,
            stack_align_bits,
            endianness,
        })
    }
}

/// Describes the LLVM data layout specification for a target platform.
///
/// The data layout defines how LLVM IR types are represented in memory for a
/// specific target ABI. This includes endianness, pointer sizes and alignments,
/// integer and floating-point alignment rules, native register widths, and
/// stack alignment constraints.
///
/// In LLVM, the data layout is an explicit, target-specific component that
/// complements the target triple. It is required for correct code generation
/// and is relied upon by both optimization passes and backend lowering.
///
/// This structure models the information encoded in LLVM's `target datalayout`
/// string as defined in the LLVM Language Reference (LLVM 17+).
///
/// # LLVM Data Layout String Format
///
/// The layout string is a sequence of dash-separated (`-`) specifiers, each
/// defining one aspect of the target ABI. Common components include:
///
/// - `e` / `E`
///   Endianness:
///   - `e` = little-endian
///   - `E` = big-endian
///
/// - `m:<style>`
///   Symbol mangling convention:
///   - `m:e` = ELF (System V, Linux, BSD)
///   - `m:o` = Mach-O (macOS, iOS)
///   - `m:w` = COFF / Windows
///
/// - `p[:<addrspace>]:<size>:<abi>[:<pref>]`
///   Pointer layout specification:
///   - `<addrspace>` = address space number (default is 0)
///   - `<size>` = pointer size in bits
///   - `<abi>` = ABI-required alignment in bits
///   - `<pref>` = preferred alignment in bits (optional)
///
///   Examples:
///   - `p:64:64` → 64-bit pointers in address space 0
///   - `p270:32:32` → 32-bit pointers in address space 270
///
/// - `i<size>:<abi>[:<pref>]`
///   Integer type alignment rules:
///   - `i64:64` → 64-bit integers with 64-bit ABI alignment
///
/// - `f<size>:<abi>[:<pref>]`
///   Floating-point type alignment rules:
///   - `f80:128` → x87 80-bit floats stored with 128-bit alignment
///
/// - `v<size>:<abi>[:<pref>]`
///   Vector type alignment rules:
///   - `v128:128` → 128-bit vectors with 128-bit alignment
///
/// - `a:<abi>[:<pref>]`
///   Aggregate (struct/array) alignment:
///   - `a:0:64` → ABI alignment determined by fields, preferred alignment 64 bits
///
/// - `n<size1>:<size2>:...`
///   Native integer register widths supported by the target:
///   - `n8:16:32:64` → target natively supports 8-, 16-, 32-, and 64-bit integers
///
/// - `S<size>`
///   Stack alignment in bits:
///   - `S128` → stack is aligned to 128 bits at function entry
///
/// # Semantics
///
/// - ABI alignment values represent the *minimum* alignment guaranteed by the ABI
/// - Preferred alignment values may be used by optimizations but are not required
///   for correctness
/// - The data layout fully determines struct layout, padding, and GEP offsets
/// - IR from different data layouts must not be mixed
///
/// # Example
///
/// ```ignore
/// let layout = DataLayout::LinuxX86_64;
/// assert_eq!(
///     layout.as_str(),
///     "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
/// );
/// ```
///
/// # References (Online Documentation)
///
/// - LLVM Language Reference Manual — *Data Layout*
///   <https://llvm.org/docs/LangRef.html#data-layout>
///
/// - LLVM Doxygen — `llvm::DataLayout` class
///   <https://llvm.org/doxygen/classllvm_1_1DataLayout.html>
///
/// - LLVM Doxygen — `llvm::StructLayout`
///   <https://llvm.org/doxygen/classllvm_1_1StructLayout.html>
///
/// - LLVM Target Triple Reference
///   <https://llvm.org/docs/LangRef.html#target-triple>
///
/// - LLVM Backend Documentation (X86, ARM, RISC-V)
///   <https://llvm.org/docs/CodeGenerator.html>

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

    /// Parse this platform's data layout into a detailed representation.
    ///
    /// This is lazily computed and allocates memory for the parsed structure.
    /// Use `as_str()` if you only need the string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the data layout string contains invalid specifications
    /// or has an invalid format.
    #[inline]
    pub fn parse(&self) -> Result<ParsedDataLayout, String> {
        ParsedDataLayout::parse(self.as_str())
    }

    /// Get a parsed data layout, panicking on parse errors.
    ///
    /// This should only be used when you're confident the layout string is valid
    /// (which should be the case for all built-in platform variants).
    ///
    /// # Panics
    ///
    /// Panics if the data layout string cannot be parsed. This should never happen
    /// for built-in platform variants as they are guaranteed to have valid layout strings.
    #[inline]
    #[must_use]
    pub fn parsed(&self) -> ParsedDataLayout {
        self.parse().expect("built-in data layout should always parse")
    }
}

// Then Display becomes:
impl fmt::Display for DataLayout {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
