// src/location/line_tracker.rs
use crate::location::{source_location::SourceLocation, source_span::SourceSpan};
use std::sync::Arc;

/// Tracks line/column positions in source code through efficient offset-to-location conversion.
///
/// Precomputes line start positions during initialization to enable O(log n) lookups. Stores
/// the entire source text and file path for location resolution and diagnostic purposes.
///
/// # Implementation Notes
/// - Line numbers are 1-indexed (first line is line 1)
/// - Column numbers are 1-indexed (first column in a line is column 1)
/// - Handles multibyte UTF-8 characters correctly through `char_indices()`
/// - Uses binary search for efficient offset lookups
#[repr(C)]  // For predictable layout
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineTracker {
    /// Original source code content, shared via Arc.
    source: Arc<str>,
    /// Precomputed starting byte offsets for each line.
    /// First element is always 0, subsequent elements mark positions after newlines.
    line_starts: Vec<usize>,
    /// Path to source file, shared via Arc for efficient cloning.
    file_path: Arc<str>,
}

impl LineTracker {
    /// Creates a new line tracker for the given source code.
    ///
    /// Precomputes all line start positions in the source text. This operation has O(n) time complexity
    /// where n is the length of the source string.
    ///
    /// # Arguments
    /// * `file_path` - Path to source file for diagnostic purposes
    /// * `source` - Complete source code content
    ///
    /// # Examples
    /// ```
    /// use jsavrs::location::line_tracker::LineTracker;
    /// let tracker = LineTracker::new("example.lang", "print(1);\nprint(2);".to_string());
    /// ```
    pub fn new(file_path: &str, source: String) -> Self {
        let line_starts = std::iter::once(0)
            .chain(
                source.match_indices('\n').map(|(pos, _)| pos + 1), // +1 to start after newline
            )
            .collect();

        Self { line_starts, file_path: Arc::from(file_path), source: source.into() }
    }
    /// Converts a byte offset to its corresponding line/column position.
    ///
    /// # Arguments
    /// * `offset` - Byte offset in source text
    ///
    /// # Returns
    /// [`SourceLocation`] containing:
    /// - Line number (1-indexed)
    /// - Column number (1-indexed)
    /// - Original byte offset
    ///
    /// # Panics
    /// Panics if `offset` exceeds source length
    ///
    /// # Algorithm
    /// 1. Uses binary search on precomputed line starts
    /// 2. For exact match: returns start of line (column 1)
    /// 3. For non-match: calculates column from nearest line start
    ///
    /// # Examples
    /// ```
    /// use jsavrs::location::line_tracker::LineTracker;
    /// let src = "a\nbc";
    /// let tracker = LineTracker::new("test.lang", src.to_string());
    /// let loc = tracker.location_for(3);
    /// assert_eq!(loc.line, 2);
    /// assert_eq!(loc.column, 2);
    /// ```
    pub fn location_for(&self, offset: usize) -> SourceLocation {
        // Validate offset is within source bounds
        if offset > self.source.len() {
            panic!("Offset {} out of bounds for source of length {}", offset, self.source.len());
        }

        match self.line_starts.binary_search(&offset) {
            // Exact match: offset is at line start
            Ok(line) => SourceLocation::new(line + 1, 1, offset),

            // Between lines: calculate column from preceding line start
            Err(line) => {
                let line_index = line.saturating_sub(1);
                let column = offset - self.line_starts[line_index] + 1;
                SourceLocation::new(line_index + 1, column, offset)
            }
        }
    }

    /// Creates a source span from a byte offset range.
    ///
    /// # Arguments
    /// * `range` - Byte offset range (start-inclusive, end-exclusive)
    ///
    /// # Returns
    /// [`SourceSpan`] containing:
    /// - File path
    /// - Start location (converted via `location_for`)
    /// - End location (converted via `location_for`)
    ///
    /// # Panics
    /// Panics if either offset exceeds source length
    ///
    /// # Examples
    /// ```
    /// use jsavrs::location::line_tracker::LineTracker;
    /// let src = "fn main() {}";
    /// let tracker = LineTracker::new("test.lang", src.to_string());
    /// let span = tracker.span_for(3..8);
    /// ```
    #[inline]
    pub fn span_for(&self, range: std::ops::Range<usize>) -> SourceSpan {
        SourceSpan::new(self.file_path.clone(), self.location_for(range.start), self.location_for(range.end))
    }

    /// Gets a specific line from the source (1-indexed)
    pub fn get_line(&self, line_number: usize) -> Option<&str> {
        // Convert to 0-indexed and get line start offset
        let start_index = *self.line_starts.get(line_number.checked_sub(1)?)?;

        // Find line end (next newline or EOF)
        let end_index = self.source[start_index..].find('\n').map(|rel| start_index + rel).unwrap_or(self.source.len());

        Some(&self.source[start_index..end_index])
    }
}
