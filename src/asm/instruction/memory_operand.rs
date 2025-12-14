use crate::asm::register::GPRegister64;
use std::fmt;

/// A memory addressing operand.
///
/// Fields:
/// - `base`: optional base register (e.g. `RAX`). If `None` the addressing
///   expression may still use index/displacement only.
/// - `index`: optional index register used with `scale` (e.g. `RCX * 4`).
/// - `scale`: scaling factor for the index register (valid values 1,2,4,8).
/// - `displacement`: signed displacement added to the address.
/// - `size`: operand size in bytes (used to print size prefixes such as
///   `BYTE PTR`, `DWORD PTR`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    /// Optional base register for the addressing expression.
    pub base: Option<GPRegister64>,
    /// Optional index register for the addressing expression.
    pub index: Option<GPRegister64>,
    /// Scaling factor applied to `index` (1, 2, 4, or 8).
    pub scale: u8,
    /// Signed displacement added to the computed address.
    pub displacement: i32,
    /// Operand size in bytes (affects textual prefix when formatting).
    pub size: usize,
}

impl MemoryOperand {
    /// Create a new `MemoryOperand` with the given optional base register.
    ///
    /// Inputs: `base: Option<GPRegister64>`
    /// Outputs: `MemoryOperand` with `index = None`, `scale = 1`,
    /// `displacement = 0`, and default `size = 8`.
    /// Side effects: none.
    #[must_use]
    pub const fn new(base: Option<GPRegister64>) -> Self {
        Self { base, index: None, scale: 1, displacement: 0, size: 8 }
    }

    /// Return a copy of this operand with `displacement` set to `disp`.
    ///
    /// Inputs: `self` (by value), `disp: i32` — new displacement.
    /// Outputs: modified `MemoryOperand` (builder-style).
    /// Side effects: none.
    #[must_use]
    pub const fn with_displacement(mut self, disp: i32) -> Self {
        self.displacement = disp;
        self
    }

    /// Return a copy with the index register and scale set.
    ///
    /// Inputs: `self` (by value), `index: GPRegister64`, `scale: u8`.
    /// Outputs: modified `MemoryOperand` with the index and scale updated.
    /// Side effects: none. Caller is expected to supply a valid scale (1,2,4,8).
    #[must_use]
    pub const fn with_index(mut self, index: GPRegister64, scale: u8) -> Self {
        self.index = Some(index);
        self.scale = scale;
        self
    }

    /// Return a copy with the operand size changed.
    ///
    /// Inputs: `self` (by value), `size: usize` — new size in bytes.
    /// Outputs: modified `MemoryOperand`.
    /// Side effects: none.
    #[must_use]
    pub const fn with_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
}

impl fmt::Display for MemoryOperand {
    /// Format the memory operand using Intel-style syntax.
    ///
    /// The formatted output contains an optional size prefix (e.g. " DWORD PTR ")
    /// followed by a bracketed addressing expression. Examples:
    /// - `DWORD PTR [rax + rcx*4 + 16]`
    /// - `BYTE PTR [rbx - 8]`
    /// - `[rcx]` (when size is unknown/unspecified)
    ///
    /// Inputs: `&self` and a `Formatter`.
    /// Outputs: formatted representation written to the formatter.
    /// Side effects: none beyond writing to the formatter.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determine size prefix for human-readable assembly output.
        let size_prefix = match self.size {
            1 => "BYTE PTR ",
            2 => "WORD PTR ",
            4 => "DWORD PTR ",
            8 => "QWORD PTR ",
            16 => "XMMWORD PTR ",
            32 => "YMMWORD PTR ",
            _ => "",
        };

        write!(f, "{size_prefix}")?;
        write!(f, "[")?;

        // Base register
        let mut has_component = if let Some(base) = &self.base {
            write!(f, "{base}")?;
            true
        } else {
            false
        };

        // Index register with scale
        if let Some(index) = &self.index {
            if has_component {
                write!(f, " + ")?;
            }
            write!(f, "{index}")?;
            if self.scale != 1 {
                write!(f, "*{}", self.scale)?;
            }
            has_component = true;
        }

        // Displacement
        if self.displacement != 0 {
            if has_component {
                if self.displacement > 0 {
                    write!(f, " + {}", self.displacement)?;
                } else {
                    write!(f, " - {}", -self.displacement)?;
                }
            } else {
                write!(f, "{}", self.displacement)?;
            }
        } else if !has_component {
            // Empty addressing expression — print `0` to make `[0]` explicit.
            write!(f, "0")?;
        }

        write!(f, "]")
    }
}
