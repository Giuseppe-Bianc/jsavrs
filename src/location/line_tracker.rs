use std::sync::Arc;
use crate::location::{
    source_location::SourceLocation,
    source_span::SourceSpan
};

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


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_location(tracker: &LineTracker, offset: usize, line: usize, column: usize) {
        let loc = tracker.location_for(offset);
        assert_eq!(loc.line, line, "Failed at line {}", line);
        assert_eq!(loc.column, column, "Failed at column {}", column);
        assert_eq!(loc.absolute_pos, offset, "Failed at offset {}", offset);
    }

    #[test]
    fn empty_source() {
        let tracker = LineTracker::new("test.txt", String::new());
        assert_location(&tracker, 0, 1, 1);
    }

    #[test]
    fn single_line_no_newline() {
        let source = "abcdef".to_string();
        let tracker = LineTracker::new("test.txt", source.clone());

        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 3, 1, 4);
        assert_location(&tracker, source.len(), 1, source.len() + 1);
    }

    #[test]
    fn single_line_with_newline() {
        let source = "a\n".to_string();
        let tracker = LineTracker::new("test.txt", source);

        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 1, 1, 2);
        assert_location(&tracker, 2, 2, 1);
    }

    #[test]
    fn multiple_lines() {
        let source = "a\nbc\ndef\n".to_string();
        let tracker = LineTracker::new("test.txt", source);

        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 1, 1, 2);
        assert_location(&tracker, 2, 2, 1);
        assert_location(&tracker, 4, 2, 3);
        assert_location(&tracker, 5, 3, 1);
        assert_location(&tracker, 8, 3, 4);
        assert_location(&tracker, 9, 4, 1);
    }

    #[test]
    fn consecutive_newlines() {
        let source = "\n\n\n".to_string();
        let tracker = LineTracker::new("test.txt", source);

        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 1, 2, 1);
        assert_location(&tracker, 2, 3, 1);
        assert_location(&tracker, 3, 4, 1);
    }

    #[test]
    fn offset_at_line_starts() {
        let source = "a\nb\nc".to_string();
        let tracker = LineTracker::new("test.txt", source);

        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 2, 2, 1);
        assert_location(&tracker, 4, 3, 1);
    }

    #[test]
    fn offset_beyond_line_starts() {
        let source = "a\nbcd".to_string();
        let tracker = LineTracker::new("test.txt", source);

        assert_location(&tracker, 3, 2, 2);
        assert_location(&tracker, 4, 2, 3);
        assert_location(&tracker, 5, 2, 4);
    }

    #[test]
    fn multi_byte_characters() {
        let source = "αβ\nγ".to_string();
        let tracker = LineTracker::new("test.txt", source);

        assert_location(&tracker, 4, 1, 5);
        assert_location(&tracker, 5, 2, 1);
        assert_location(&tracker, 6, 2, 2);
    }

    #[test]
    fn span_across_lines() {
        let source = "a\nb\nc".to_string();
        let tracker = LineTracker::new("test.txt", source);

        let span = tracker.span_for(1..4);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 2);
        assert_eq!(span.end.line, 3);
        assert_eq!(span.end.column, 1);
    }

    #[test]
    fn empty_span() {
        let tracker = LineTracker::new("test.txt", "abc".to_string());
        let span = tracker.span_for(0..0);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 1);
        assert_eq!(span.end.line, 1);
        assert_eq!(span.end.column, 1);
    }

    #[test]
    fn span_at_end() {
        let source = "abc".to_string();
        let tracker = LineTracker::new("test.txt", source.clone());
        let span = tracker.span_for(3..3);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 4);
        assert_eq!(span.end.line, 1);
        assert_eq!(span.end.column, 4);
    }

    #[test]
    #[should_panic(expected = "Offset 4 out of bounds for source of length 3")]
    fn offset_out_of_bounds() {
        let tracker = LineTracker::new("test.txt", "abc".to_string());
        tracker.location_for(4);
    }

    #[test]
    fn only_newlines() {
        let source = "\n\n\n".to_string();
        let tracker = LineTracker::new("test.txt", source);
        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 1, 2, 1);
        assert_location(&tracker, 2, 3, 1);
        assert_location(&tracker, 3, 4, 1);
    }

    #[test]
    fn single_character() {
        let source = "a".to_string();
        let tracker = LineTracker::new("test.txt", source);
        assert_location(&tracker, 0, 1, 1);
        assert_location(&tracker, 1, 1, 2);
    }

    #[test]
    fn span_same_line() {
        let source = "abcdef".to_string();
        let tracker = LineTracker::new("test.txt", source);
        let span = tracker.span_for(2..5);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 3);
        assert_eq!(span.end.line, 1);
        assert_eq!(span.end.column, 6);
    }

    #[test]
    fn multi_line_span_with_multi_byte() {
        let source = "α\nβ\nγ".to_string();
        let tracker = LineTracker::new("test.txt", source);
        let span = tracker.span_for(2..5);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 3);
        assert_eq!(span.end.line, 2);
        assert_eq!(span.end.column, 3);
    }

    #[test]
    fn file_path_correct() {
        let source = "abc".to_string();
        let file = "test.txt";
        let file_arc:Arc<str> = Arc::from(file);
        let tracker = LineTracker::new(file, source);
        let span = tracker.span_for(0..1);
        assert_eq!(span.file_path, file_arc);
    }
}