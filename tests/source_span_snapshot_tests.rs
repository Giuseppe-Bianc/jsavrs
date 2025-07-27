use jsavrs::location::source_location::SourceLocation;
use jsavrs::location::source_span::{truncate_path, SourceSpan};
use jsavrs::utils::create_span;
use std::path::Path;
use std::sync::Arc;

macro_rules! truncate_test {
    ($name:ident, $path:expr, $depth:expr) => {
        #[test]
        fn $name() {
            let path = Path::new($path);
            let truncated = truncate_path(path, $depth);
            let snapshot_name = if cfg!(unix) {
                concat!(stringify!($name), "_unix")
            } else {
                concat!(stringify!($name), "_windows")
            };
            insta::assert_snapshot!(snapshot_name, truncated);
        }
    };
}

macro_rules! span_str_test {
    ($name:ident, $file:expr, $sl:expr, $sc:expr, $el:expr, $ec:expr) => {
        #[test]
        fn $name() {
            let span = create_span($file, $sl, $sc, $el, $ec);

            let snapshot_name = if cfg!(unix) {
                concat!(stringify!($name), "_unix")
            } else {
                concat!(stringify!($name), "_windows")
            };
            insta::assert_snapshot!(snapshot_name, span.to_string());
        }
    };
}

// Test di troncamento percorso
truncate_test!(longer_than_depth, "a/b/c/d", 2);
truncate_test!(exact_depth, "a/b/c", 3);
truncate_test!(shorter_than_depth, "a", 2);
truncate_test!(depth_zero, "/usr/project/src/main.vn", 0);
truncate_test!(single_component, "file.vn", 2);
truncate_test!(absolute_path, "/usr/project/src/main.vn", 2);

// Test formattazione stringa span
span_str_test!(same_line, "project/src/main.vn", 5, 3, 5, 10);
span_str_test!(different_lines, "src/module/file.vn", 2, 1, 4, 5);
span_str_test!(single_component_path, "file.vn", 1, 1, 1, 1);
span_str_test!(same_start_end, "a/b/c/d/file.vn", 3, 2, 3, 2);
span_str_test!(minimal_coordinates, "f.vn", 0, 0, 0, 0);

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
    let snapshot_name = if cfg!(unix) {
        "absolute_path_span_unix"
    } else {
        "absolute_path_span_windows"
    };
    insta::assert_snapshot!(snapshot_name, span.to_string());
}

#[test]
fn merge_same_file_expands_span() {
    let mut span1 = create_span("file.vn", 2, 3, 5, 10);
    let span2 = create_span("file.vn", 1, 1, 6, 5);
    span1.merge(&span2);
    insta::assert_debug_snapshot!(span1);
}

#[test]
fn merge_different_files_no_change() {
    let mut span1 = create_span("file1.vn", 1, 1, 2, 2);
    let span2 = create_span("file2.vn", 3, 3, 4, 4);
    span1.merge(&span2);
    insta::assert_debug_snapshot!(span1);
}

#[test]
fn merged_same_file_returns_combined() {
    let span1 = create_span("file.vn", 2, 3, 5, 10);
    let span2 = create_span("file.vn", 1, 1, 6, 5);
    let merged = span1.merged(&span2).unwrap();
    insta::assert_debug_snapshot!(merged);
}

#[test]
fn merged_different_files_returns_none() {
    let span1 = create_span("file1.vn", 1, 1, 2, 2);
    let span2 = create_span("file2.vn", 3, 3, 4, 4);
    insta::assert_debug_snapshot!(span1.merged(&span2));
}

#[test]
fn merged_other_within_span() {
    let span1 = create_span("file.vn", 1, 1, 5, 10);
    let span2 = create_span("file.vn", 2, 2, 4, 8);
    let merged = span1.merged(&span2).unwrap();
    insta::assert_debug_snapshot!(merged);
}

#[test]
fn merged_overlapping_spans() {
    let span1 = create_span("file.vn", 3, 5, 8, 9);
    let span2 = create_span("file.vn", 5, 2, 10, 3);
    let merged = span1.merged(&span2).unwrap();
    insta::assert_debug_snapshot!(merged);
}

#[test]
fn merged_other_before() {
    let span1 = create_span("file.vn", 5, 5, 6, 6);
    let span2 = create_span("file.vn", 3, 3, 4, 4);
    let merged = span1.merged(&span2).unwrap();
    insta::assert_debug_snapshot!(merged);
}

#[test]
fn merged_other_after() {
    let span1 = create_span("file.vn", 3, 3, 4, 4);
    let span2 = create_span("file.vn", 5, 5, 6, 6);
    let merged = span1.merged(&span2).unwrap();
    insta::assert_debug_snapshot!(merged);
}

#[test]
fn merged_with_self() {
    let span = create_span("file.vn", 1, 1, 2, 2);
    let merged = span.merged(&span).unwrap();
    insta::assert_debug_snapshot!(merged);
}

#[test]
fn source_span_default() {
    let span = SourceSpan::default();
    insta::assert_debug_snapshot!(span);
}
