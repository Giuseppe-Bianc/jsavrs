// src/location/source_span.rs
use crate::location::source_location::SourceLocation;
use std::cmp::PartialOrd;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents a contiguous range of source code in a specific file.
///
/// Spans track:
/// - Source file path
/// - Start position (inclusive)
/// - End position (exclusive)
///
/// Used for error reporting, source mapping, and semantic analysis.
/// Implements ordering based on start position then end position.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceSpan {
    /// Path to source file (shared reference)
    pub file_path: Arc<str>,

    /// Starting position of the span (inclusive)
    pub start: SourceLocation,

    /// Ending position of the span (exclusive)
    pub end: SourceLocation,
}

impl SourceSpan {
    /// Creates a new source span covering a specific range.
    ///
    /// # Arguments
    /// * `file_path` - Path to source file
    /// * `start` - Starting position (inclusive)
    /// * `end` - Ending position (exclusive)
    ///
    /// # Panics
    /// Should panic if:
    /// - `start` and `end` are from different files
    /// - `end` comes before `start`
    ///   (Currently not enforced in implementation)
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use jsavrs::location::source_location::SourceLocation;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let start = SourceLocation::new(1, 1, 0);
    /// let end = SourceLocation::new(1, 5, 4);
    /// let span = SourceSpan::new(Arc::from("test.lang"), start, end);
    /// ```
    pub fn new(file_path: Arc<str>, start: SourceLocation, end: SourceLocation) -> Self {
        // In production code, should validate:
        // assert!(start <= end, "Span start must come before end");
        // assert!(file_path == other.file_path, "Cannot merge spans from different files");
        Self { file_path, start, end }
    }

    /// Merges another span into this one in-place.
    ///
    /// Expands current span to cover both original and `other` span.
    /// Only merges if spans are from the same file.
    ///
    /// # Arguments
    /// * `other` - Span to merge with current span
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use jsavrs::location::source_location::SourceLocation;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let mut span1 = SourceSpan::new(Arc::from("f"), SourceLocation::new(1, 1, 0), SourceLocation::new(1, 5, 4));
    /// let span2 = SourceSpan::new(Arc::from("f"), SourceLocation::new(1,3, 2), SourceLocation::new(1,8, 7));
    /// span1.merge(&span2);
    /// assert_eq!(span1.start, SourceLocation::new(1, 1, 0));
    /// assert_eq!(span1.end, SourceLocation::new(1,8, 7));
    /// ```
    pub fn merge(&mut self, other: &SourceSpan) {
        if self.file_path == other.file_path {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
        }
    }

    /// Creates a new span that combines this span with another.
    ///
    /// Returns `Some(SourceSpan)` if spans are from the same file,
    /// `None` otherwise.
    ///
    /// # Arguments
    /// * `other` - Span to combine with
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use jsavrs::location::source_location::SourceLocation;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let span1 = SourceSpan::new(Arc::from("f"), SourceLocation::new(1, 1, 0), SourceLocation::new(1,5,4));
    /// let span2 = SourceSpan::new(Arc::from("f"), SourceLocation::new(1,3,2), SourceLocation::new(2,5,8));
    /// let merged = span1.merged(&span2).unwrap();
    /// assert_eq!(merged.start, SourceLocation::new(1, 1, 0));
    /// assert_eq!(merged.end, SourceLocation::new(2,5,8));
    /// ```
    pub fn merged(&self, other: &SourceSpan) -> Option<Self> {
        (self.file_path == other.file_path).then(|| Self {
            file_path: self.file_path.clone(),
            start: self.clone().start.min(other.start),
            end: self.clone().end.max(other.end),
        })
    }
}

impl Default for SourceSpan {
    /// Creates a default invalid span with empty path and zero positions.
    ///
    /// Primarily useful for placeholder values. Should not be used for
    /// actual source references.
    fn default() -> Self {
        SourceSpan { file_path: Arc::from(""), start: SourceLocation::default(), end: SourceLocation::default() }
    }
}

impl std::fmt::Display for SourceSpan {
    /// Formats the span for human-readable output.
    ///
    /// Format: `[truncated_path]:line [start_line]:column [start_col] - line [end_line]:column [end_col]`
    ///
    /// Paths are truncated to show only last 2 components for brevity.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let truncated_path = truncate_path(Path::new(&*self.file_path), 2);
        write!(
            f,
            "{}:line {}:column {} - line {}:column {}",
            truncated_path, self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

/// Truncates a path to show only the last `depth` components.
///
/// Useful for displaying long paths in error messages.
///
/// # Arguments
/// * `path` - Original file path
/// * `depth` - Number of trailing components to preserve
///
/// # Returns
/// String representation of truncated path:
/// - Full path if component count <= depth
/// - `..` + last `depth` components otherwise
///
/// # Examples
/// ```
/// use std::path::Path;
/// use jsavrs::location::source_span::truncate_path;
/// let path = if cfg!(unix) {
///         "/project/src/module/file.lang"
///     } else {
///         "C:\\project\\src.\\module\\file.lang"
///     };
/// let path = Path::new("/project/src/module/file.lang");
/// let expected = if cfg!(unix) { "../module/file.lang" } else { "..\\module\\file.lang" };
/// assert_eq!(truncate_path(path, 2), expected);
/// ```
pub fn truncate_path(path: &Path, depth: usize) -> String {
    let components: Vec<_> = path.components().collect();
    let len = components.len();

    let truncated = if len <= depth {
        PathBuf::from_iter(&components)
    } else {
        let tail = &components[len - depth..];
        PathBuf::from("..").join(PathBuf::from_iter(tail))
    };

    truncated.display().to_string()
}


pub trait HasSpan {
    fn span(&self) -> &SourceSpan;
}