use crate::location::{source_location::SourceLocation, source_span::SourceSpan};
use std::sync::Arc;

/// Precompute line start positions for efficient lookups
#[allow(dead_code)]
pub struct LineTracker {
    line_starts: Vec<usize>,
    file_path: Arc<str>,
    source: String,
}

impl LineTracker {
    /// Creates a new `LineTracker` for the given source code.
    pub fn new(file_path: &str, source: String) -> Self {
        let mut line_starts = vec![0];

        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push(i + c.len_utf8());
            }
        }

        Self {
            line_starts,
            file_path: Arc::from(file_path),
            source,
        }
    }

    /// Returns the line and column for a given offset in the source code.
    pub fn location_for(&self, offset: usize) -> SourceLocation {
        if offset > self.source.len() {
            panic!(
                "Offset {} out of bounds for source of length {}",
                offset,
                self.source.len()
            );
        }

        match self.line_starts.binary_search(&offset) {
            Ok(line) => {
                // Exact match means start of a line: column is 1
                SourceLocation::new(line + 1, 1, offset)
            }
            Err(line) => {
                // `line` is the first line *after* the offset
                let line_index = line.saturating_sub(1);
                let column = offset - self.line_starts[line_index] + 1;
                SourceLocation::new(line_index + 1, column, offset)
            }
        }
    }

    /// Returns a `SourceSpan` for the given range of offsets.
    pub fn span_for(&self, range: std::ops::Range<usize>) -> SourceSpan {
        SourceSpan::new(
            self.file_path.clone(),
            self.location_for(range.start),
            self.location_for(range.end),
        )
    }
}
