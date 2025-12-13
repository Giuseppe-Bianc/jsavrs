// src/location/source_location.rs

/// Represents a specific position in source code with line/column information.
///
/// Stores both human-readable (line/column) and machine-oriented (byte offset)
/// positioning data. Useful for error reporting, debugging information, and
/// source mapping.
///
/// # Indexing Conventions
/// - `line`: 1-indexed line number (first line is line 1)
/// - `column`: 1-indexed column number (first character in line is column 1)
/// - `absolute_pos`: 0-indexed byte offset from start of source
///
/// # Ordering
/// Implements lexicographic ordering based on:
/// 1. Line number
/// 2. Column number
/// 3. Byte offset
///
/// This ordering matches how humans read source code (top-to-bottom, left-to-right)
/// while maintaining consistency with the derived implementations of `PartialOrd`/`Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default, Hash)]
pub struct SourceLocation {
    /// Line number in source file (1-indexed).
    pub line: usize,

    /// Column position in line (1-indexed, byte-based).
    pub column: usize,

    /// Absolute byte offset from start of source (0-indexed).
    pub absolute_pos: usize,
}

impl SourceLocation {
    /// Creates a new source location with specified position data.
    ///
    /// # Arguments
    /// * `line` - 1-indexed line number
    /// * `column` - 1-indexed column number
    /// * `absolute_pos` - 0-indexed byte offset
    ///
    /// # Examples
    /// ```
    /// use jsavrs::location::source_location::SourceLocation;
    /// let loc = SourceLocation::new(3, 5, 20);
    /// assert_eq!(loc.line, 3);
    /// assert_eq!(loc.column, 5);
    /// assert_eq!(loc.absolute_pos, 20);
    /// ```
    #[must_use]
    pub const fn new(line: usize, column: usize, absolute_pos: usize) -> Self {
        Self { line, column, absolute_pos }
    }
}
