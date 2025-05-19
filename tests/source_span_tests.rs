use jsavrs::location::source_location::SourceLocation;
use jsavrs::location::source_span::{SourceSpan, truncate_path};
use std::path::Path;
use std::sync::Arc;

macro_rules! truncate_test {
    ($name:ident, $path:expr, $depth:expr, $unix:expr, $windows:expr) => {
        #[test]
        fn $name() {
            let path = Path::new($path);
            let truncated = truncate_path(path, $depth);
            let expected = if cfg!(unix) { $unix } else { $windows };
            assert_eq!(truncated, expected);
        }
    };
}

macro_rules! span_str_test {
    ($name:ident, $file:expr, $sl:expr, $sc:expr, $el:expr, $ec:expr, $unix:expr, $windows:expr) => {
        #[test]
        fn $name() {
            let span = SourceSpan::new(
                Arc::from($file),
                SourceLocation {
                    line: $sl,
                    column: $sc,
                    absolute_pos: 0,
                },
                SourceLocation {
                    line: $el,
                    column: $ec,
                    absolute_pos: 0,
                },
            );
            let expected = if cfg!(unix) { $unix } else { $windows };
            assert_eq!(span.to_string(), expected);
        }
    };
}

// Test di troncamento percorso
truncate_test!(longer_than_depth, "a/b/c/d", 2, "../c/d", "..\\c\\d");
truncate_test!(exact_depth, "a/b/c", 3, "a/b/c", "a\\b\\c");
truncate_test!(shorter_than_depth, "a", 2, "a", "a");
truncate_test!(depth_zero, "/usr/project/src/main.vn", 0, "../", "..\\");
truncate_test!(single_component, "file.vn", 2, "file.vn", "file.vn");
truncate_test!(
    absolute_path,
    "/usr/project/src/main.vn",
    2,
    "../src/main.vn",
    "..\\src\\main.vn"
);

// Test formattazione stringa span
span_str_test!(
    same_line,
    "project/src/main.vn",
    5,
    3,
    5,
    10,
    "../src/main.vn:line 5:column 3 - line 5:column 10",
    "..\\src\\main.vn:line 5:column 3 - line 5:column 10"
);
span_str_test!(
    different_lines,
    "src/module/file.vn",
    2,
    1,
    4,
    5,
    "../module/file.vn:line 2:column 1 - line 4:column 5",
    "..\\module\\file.vn:line 2:column 1 - line 4:column 5"
);
span_str_test!(
    single_component_path,
    "file.vn",
    1,
    1,
    1,
    1,
    "file.vn:line 1:column 1 - line 1:column 1",
    "file.vn:line 1:column 1 - line 1:column 1"
);
span_str_test!(
    same_start_end,
    "a/b/c/d/file.vn",
    3,
    2,
    3,
    2,
    "../d/file.vn:line 3:column 2 - line 3:column 2",
    "..\\d\\file.vn:line 3:column 2 - line 3:column 2"
);
span_str_test!(
    minimal_coordinates,
    "f.vn",
    0,
    0,
    0,
    0,
    "f.vn:line 0:column 0 - line 0:column 0",
    "f.vn:line 0:column 0 - line 0:column 0"
);

#[test]
fn absolute_path_span() {
    let path = if cfg!(unix) {
        "/usr/project/src/main.vn"
    } else {
        "C:\\project\\src\\main.vn"
    };
    let span = SourceSpan::new(
        Arc::from(path),
        SourceLocation {
            line: 5,
            column: 3,
            absolute_pos: 20,
        },
        SourceLocation {
            line: 5,
            column: 10,
            absolute_pos: 30,
        },
    );
    let expected = if cfg!(unix) {
        "../src/main.vn:line 5:column 3 - line 5:column 10"
    } else {
        "..\\src\\main.vn:line 5:column 3 - line 5:column 10"
    };
    assert_eq!(span.to_string(), expected);
}

// Test di merging
fn create_span(
    file_path: &str,
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
) -> SourceSpan {
    SourceSpan {
        file_path: Arc::from(file_path),
        start: SourceLocation {
            line: start_line,
            column: start_col,
            absolute_pos: 0,
        },
        end: SourceLocation {
            line: end_line,
            column: end_col,
            absolute_pos: 0,
        },
    }
}

#[test]
fn merge_same_file_expands_span() {
    let mut span1 = create_span("file.vn", 2, 3, 5, 10);
    let span2 = create_span("file.vn", 1, 1, 6, 5);
    span1.merge(&span2);
    assert_eq!(span1.start.line, 1);
    assert_eq!(span1.start.column, 1);
    assert_eq!(span1.end.line, 6);
    assert_eq!(span1.end.column, 5);
}

#[test]
fn merge_different_files_no_change() {
    let mut span1 = create_span("file1.vn", 1, 1, 2, 2);
    let span2 = create_span("file2.vn", 3, 3, 4, 4);
    span1.merge(&span2);
    assert_eq!(span1.start.line, 1);
    assert_eq!(span1.end.line, 2);
}

#[test]
fn merged_same_file_returns_combined() {
    let span1 = create_span("file.vn", 2, 3, 5, 10);
    let span2 = create_span("file.vn", 1, 1, 6, 5);
    let merged = span1.merged(&span2).unwrap();
    assert_eq!(merged.start.line, 1);
    assert_eq!(merged.end.line, 6);
}

#[test]
fn merged_different_files_returns_none() {
    let span1 = create_span("file1.vn", 1, 1, 2, 2);
    let span2 = create_span("file2.vn", 3, 3, 4, 4);
    assert!(span1.merged(&span2).is_none());
}

#[test]
fn merged_other_within_span() {
    let span1 = create_span("file.vn", 1, 1, 5, 10);
    let span2 = create_span("file.vn", 2, 2, 4, 8);
    let merged = span1.merged(&span2).unwrap();
    assert_eq!(merged.start, span1.start);
    assert_eq!(merged.end, span1.end);
}

#[test]
fn merged_overlapping_spans() {
    let span1 = create_span("file.vn", 3, 5, 8, 9);
    let span2 = create_span("file.vn", 5, 2, 10, 3);
    let merged = span1.merged(&span2).unwrap();
    assert_eq!(merged.start.line, 3);
    assert_eq!(merged.end.line, 10);
}

#[test]
fn merged_other_before() {
    let span1 = create_span("file.vn", 5, 5, 6, 6);
    let span2 = create_span("file.vn", 3, 3, 4, 4);
    let merged = span1.merged(&span2).unwrap();
    assert_eq!(merged.start, span2.start);
    assert_eq!(merged.end, span1.end);
}

#[test]
fn merged_other_after() {
    let span1 = create_span("file.vn", 3, 3, 4, 4);
    let span2 = create_span("file.vn", 5, 5, 6, 6);
    let merged = span1.merged(&span2).unwrap();
    assert_eq!(merged.start, span1.start);
    assert_eq!(merged.end, span2.end);
}

#[test]
fn merged_with_self() {
    let span = create_span("file.vn", 1, 1, 2, 2);
    let merged = span.merged(&span).unwrap();
    assert_eq!(merged.start, span.start);
    assert_eq!(merged.end, span.end);
}
