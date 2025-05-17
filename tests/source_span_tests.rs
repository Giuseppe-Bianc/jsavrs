use jsavrs::location::source_location::SourceLocation;
use jsavrs::location::source_span::{SourceSpan, truncate_path};
use std::path::Path;
use std::sync::Arc;

#[test]
fn longer_than_depth() {
    let path = Path::new("a/b/c/d");
    #[cfg(unix)]
    assert_eq!(truncate_path(path, 2), "../c/d");
    #[cfg(windows)]
    assert_eq!(truncate_path(path, 2), "..\\c\\d");
}

#[test]
fn exact_depth() {
    let path = Path::new("a/b/c");
    #[cfg(unix)]
    assert_eq!(truncate_path(path, 3), "a/b/c");
    #[cfg(windows)]
    assert_eq!(truncate_path(path, 3), "a\\b\\c");
}

#[test]
fn shorter_than_depth() {
    let path = Path::new("a");
    #[cfg(unix)]
    assert_eq!(truncate_path(path, 2), "a");
    #[cfg(windows)]
    assert_eq!(truncate_path(path, 2), "a");
}

#[test]
fn depth_zero() {
    let path = Path::new("/usr/project/src/main.vn");
    #[cfg(unix)]
    assert_eq!(truncate_path(path, 0), "../");
    #[cfg(windows)]
    assert_eq!(truncate_path(path, 0), "..\\");
}

#[test]
fn single_component() {
    let path = Path::new("file.vn");
    #[cfg(unix)]
    assert_eq!(truncate_path(path, 2), "file.vn");
    #[cfg(windows)]
    assert_eq!(truncate_path(path, 2), "file.vn");
}

#[test]
fn absolute_path() {
    let path = Path::new("/usr/project/src/main.vn");
    #[cfg(unix)]
    assert_eq!(truncate_path(path, 2), "../src/main.vn");
    #[cfg(windows)]
    assert_eq!(truncate_path(path, 2), "..\\src\\main.vn");
}

#[test]
fn same_line() {
    let span = SourceSpan::new(
        Arc::from("project/src/main.vn"),
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
    #[cfg(unix)]
    assert_eq!(span.to_string(), "../src/main.vn:5:3-5:10");
    #[cfg(windows)]
    assert_eq!(span.to_string(), "..\\src\\main.vn:5:3-5:10");
}

#[test]
fn different_lines() {
    let span = SourceSpan::new(
        Arc::from("src/module/file.vn"),
        SourceLocation {
            line: 2,
            column: 1,
            absolute_pos: 0,
        },
        SourceLocation {
            line: 4,
            column: 5,
            absolute_pos: 5,
        },
    );
    #[cfg(unix)]
    assert_eq!(span.to_string(), "../module/file.vn:2:1-4:5");
    #[cfg(windows)]
    assert_eq!(span.to_string(), "..\\module\\file.vn:2:1-4:5");
}

#[test]
fn single_component_path() {
    let span = SourceSpan::new(
        Arc::from("file.vn"),
        SourceLocation {
            line: 1,
            column: 1,
            absolute_pos: 0,
        },
        SourceLocation {
            line: 1,
            column: 1,
            absolute_pos: 0,
        },
    );
    #[cfg(unix)]
    assert_eq!(span.to_string(), "file.vn:1:1-1:1");
    #[cfg(windows)]
    assert_eq!(span.to_string(), "file.vn:1:1-1:1");
}

#[test]
fn same_start_end() {
    let span = SourceSpan::new(
        Arc::from("a/b/c/d/file.vn"),
        SourceLocation {
            line: 3,
            column: 2,
            absolute_pos: 10,
        },
        SourceLocation {
            line: 3,
            column: 2,
            absolute_pos: 10,
        },
    );
    #[cfg(unix)]
    assert_eq!(span.to_string(), "../d/file.vn:3:2-3:2");
    #[cfg(windows)]
    assert_eq!(span.to_string(), "..\\d\\file.vn:3:2-3:2");
}

#[test]
fn minimal_coordinates() {
    let span = SourceSpan::new(
        Arc::from("f.vn"),
        SourceLocation {
            line: 0,
            column: 0,
            absolute_pos: 0,
        },
        SourceLocation {
            line: 0,
            column: 0,
            absolute_pos: 0,
        },
    );
    #[cfg(unix)]
    assert_eq!(span.to_string(), "f.vn:0:0-0:0");
    #[cfg(windows)]
    assert_eq!(span.to_string(), "f.vn:0:0-0:0");
}

#[test]
fn absolute_path_span() {
    #[cfg(unix)]
    let path: Arc<str> = Arc::from("/usr/project/src/main.vn");
    #[cfg(windows)]
    let path: Arc<str> = Arc::from("C:\\project\\src\\main.vn");
    let span = SourceSpan::new(
        path,
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
    #[cfg(unix)]
    assert_eq!(span.to_string(), "../src/main.vn:5:3-5:10");
    #[cfg(windows)]
    assert_eq!(span.to_string(), "..\\src\\main.vn:5:3-5:10");
}

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
    assert_eq!(
        span1.start,
        SourceLocation {
            line: 1,
            column: 1,
            absolute_pos: 0
        }
    );
    assert_eq!(
        span1.end,
        SourceLocation {
            line: 6,
            column: 5,
            absolute_pos: 0
        }
    );
}

#[test]
fn merge_different_files_no_change() {
    let mut span1 = create_span("file1.vn", 1, 1, 2, 2);
    let span2 = create_span("file2.vn", 3, 3, 4, 4);
    span1.merge(&span2);
    assert_eq!(
        span1.start,
        SourceLocation {
            line: 1,
            column: 1,
            absolute_pos: 0
        }
    );
    assert_eq!(
        span1.end,
        SourceLocation {
            line: 2,
            column: 2,
            absolute_pos: 0
        }
    );
}

#[test]
fn merged_same_file_returns_combined() {
    let span1 = create_span("file.vn", 2, 3, 5, 10);
    let span2 = create_span("file.vn", 1, 1, 6, 5);
    let merged = span1.merged(&span2).unwrap();
    assert_eq!(
        merged.start,
        SourceLocation {
            line: 1,
            column: 1,
            absolute_pos: 0
        }
    );
    assert_eq!(
        merged.end,
        SourceLocation {
            line: 6,
            column: 5,
            absolute_pos: 0
        }
    );
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
    assert_eq!(
        merged.start,
        SourceLocation {
            line: 3,
            column: 5,
            absolute_pos: 0
        }
    );
    assert_eq!(
        merged.end,
        SourceLocation {
            line: 10,
            column: 3,
            absolute_pos: 0
        }
    );
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
