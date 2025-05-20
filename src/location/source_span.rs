//src/location/source_span.rs
use crate::location::source_location::SourceLocation;
use std::cmp::PartialOrd;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    // Versione esistente che modifica in-place
    pub fn merge(&mut self, other: &SourceSpan) {
        if self.file_path == other.file_path {
            self.start = self.clone().start.min(other.start.clone());
            self.end = self.clone().end.max(other.end.clone());
        }
    }

    // Nuova versione che restituisce un nuovo span
    pub fn merged(&self, other: &SourceSpan) -> Option<Self> {
        (self.file_path == other.file_path).then(|| Self {
            file_path: self.file_path.clone(),
            start: self.clone().start.min(other.start.clone()),
            end: self.clone().end.max(other.end.clone()),
        })
    }
}

impl Default for SourceSpan {
    fn default() -> Self {
        SourceSpan {
            file_path: Arc::from(""),
            start: SourceLocation::default(),
            end: SourceLocation::default(),
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
            "{}:line {}:column {} - line {}:column {}",
            truncated_path, self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

/// Returns the last `depth` components of a path, like `Vandior/input.vn`
pub fn truncate_path(path: &Path, depth: usize) -> String {
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
