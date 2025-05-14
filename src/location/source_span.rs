use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::location::source_location::SourceLocation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    pub file_path: Arc<str>,
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl SourceSpan {
    pub fn new(file_path: Arc<str>, start: SourceLocation, end: SourceLocation) -> Self {
        Self {
            file_path,
            start,
            end,
        }
    }
}

impl std::fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempt to truncate the path if it's long
        let path = Path::new(&*self.file_path);
        let truncated_path = truncate_path(path, 2); // show last 2 components

        write!(
            f,
            "{}:{}:{}-{}:{}",
            truncated_path, self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

/// Returns the last `depth` components of a path, like `Vandior/input.vn`
fn truncate_path(path: &Path, depth: usize) -> String {
    let components: Vec<_> = path.components().collect();
    let len = components.len();
    if len <= depth {
        components.iter().collect::<PathBuf>().display().to_string()
    } else {
        PathBuf::from("..")
            .join(components[len - depth..].iter().collect::<PathBuf>())
            .display()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    //use crate::location::source_location::SourceLocation;

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
            SourceLocation { line: 5, column: 3, absolute_pos: 20 },
            SourceLocation { line: 5, column: 10, absolute_pos: 30 },
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
            SourceLocation { line: 2, column: 1, absolute_pos: 0 },
            SourceLocation { line: 4, column: 5, absolute_pos: 5 },
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
            SourceLocation { line: 1, column: 1, absolute_pos: 0 },
            SourceLocation { line: 1, column: 1, absolute_pos: 0 },
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
            SourceLocation { line: 3, column: 2 , absolute_pos: 10 },
            SourceLocation { line: 3, column: 2, absolute_pos: 10 },
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
            SourceLocation { line: 0, column: 0, absolute_pos: 0 },
            SourceLocation { line: 0, column: 0, absolute_pos: 0 },
        );
        #[cfg(unix)]
        assert_eq!(span.to_string(), "f.vn:0:0-0:0");
        #[cfg(windows)]
        assert_eq!(span.to_string(), "f.vn:0:0-0:0");
    }

    #[test]
    fn absolute_path_span() {

        #[cfg(unix)]
        let path :Arc<str> = Arc::from("/usr/project/src/main.vn");
        #[cfg(windows)]
        let path :Arc<str> = Arc::from("C:\\project\\src\\main.vn");
        let span = SourceSpan::new(
            path,
            SourceLocation { line: 5, column: 3, absolute_pos: 20 },
            SourceLocation { line: 5, column: 10, absolute_pos: 30 },
        );
        #[cfg(unix)]
        assert_eq!(span.to_string(), "../src/main.vn:5:3-5:10");
        #[cfg(windows)]
        assert_eq!(span.to_string(), "..\\src\\main.vn:5:3-5:10");
    }
}