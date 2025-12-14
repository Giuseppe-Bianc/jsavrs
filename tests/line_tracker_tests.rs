use jsavrs::location::line_tracker::LineTracker;
use std::sync::Arc;

// Helper function for location assertions
fn assert_location(tracker: &LineTracker, offset: usize, line: usize, column: usize) {
    let loc = tracker.location_for(offset);
    assert_eq!(loc.line, line, "Failed at line {line}");
    assert_eq!(loc.column, column, "Failed at column {column}");
    assert_eq!(loc.absolute_pos, offset, "Failed at offset {offset}");
}

// Macro for multiple location tests
macro_rules! test_locations {
    ($name:ident, $source:expr, $($offset:expr => ($line:expr, $column:expr)),+ $(,)?) => {
        #[test]
        fn $name() {
            let tracker = LineTracker::new("test.txt", $source.to_string());
            $( assert_location(&tracker, $offset, $line, $column); )+
        }
    };
}

// Helper for span assertions
fn assert_span(source: &str, range: std::ops::Range<usize>, start: (usize, usize), end: (usize, usize)) {
    let tracker = LineTracker::new("test.txt", source.to_string());
    let span = tracker.span_for(range);
    assert_eq!((span.start.line, span.start.column), start);
    assert_eq!((span.end.line, span.end.column), end);
}

// Tests using the new helpers
test_locations!(
    empty_source,
    "",
    0 => (1, 1)
);

test_locations!(
    single_line_no_newline,
    "abcdef",
    0 => (1, 1),
    3 => (1, 4),
    6 => (1, 7)
);

test_locations!(
    single_line_with_newline,
    "a\n",
    0 => (1, 1),
    1 => (1, 2),
    2 => (2, 1)
);

test_locations!(
    multiple_lines,
    "a\nbc\ndef\n",
    0 => (1, 1),
    1 => (1, 2),
    2 => (2, 1),
    4 => (2, 3),
    5 => (3, 1),
    8 => (3, 4),
    9 => (4, 1)
);

test_locations!(
    consecutive_newlines,
    "\n\n\n",
    0 => (1, 1),
    1 => (2, 1),
    2 => (3, 1),
    3 => (4, 1)
);

test_locations!(
    offset_at_line_starts,
    "a\nb\nc",
    0 => (1, 1),
    2 => (2, 1),
    4 => (3, 1)
);

test_locations!(
    offset_beyond_line_starts,
    "a\nbcd",
    3 => (2, 2),
    4 => (2, 3),
    5 => (2, 4)
);

test_locations!(
    multi_byte_characters,
    "αβ\nγ",
    4 => (1, 5),
    5 => (2, 1),
    6 => (2, 2)
);

test_locations!(
    only_newlines,
    "\n\n\n",
    0 => (1, 1),
    1 => (2, 1),
    2 => (3, 1),
    3 => (4, 1)
);

test_locations!(
    single_character,
    "a",
    0 => (1, 1),
    1 => (1, 2)
);

#[test]
fn span_across_lines() {
    assert_span("a\nb\nc", 1..4, (1, 2), (3, 1));
}

#[test]
fn empty_span() {
    assert_span("abc", 0..0, (1, 1), (1, 1));
}

#[test]
fn span_at_end() {
    assert_span("abc", 3..3, (1, 4), (1, 4));
}

#[test]
fn span_same_line() {
    assert_span("abcdef", 2..5, (1, 3), (1, 6));
}

#[test]
fn multi_line_span_with_multi_byte() {
    assert_span("α\nβ\nγ", 2..5, (1, 3), (2, 3));
}

#[test]
#[should_panic(expected = "Offset 4 out of bounds for source of length 3")]
#[allow(unused_must_use)]
fn offset_out_of_bounds() {
    let tracker = LineTracker::new("test.txt", "abc".to_string());
    tracker.location_for(4);
}

#[test]
fn file_path_correct() {
    let file = "special.txt";
    let tracker = LineTracker::new(file, "abc".to_string());
    let span = tracker.span_for(0..1);
    assert_eq!(span.file_path, Arc::from(file));
}
