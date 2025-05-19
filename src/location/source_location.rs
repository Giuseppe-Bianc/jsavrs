//src/location/source_location.rs
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub absolute_pos: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, absolute_pos: usize) -> Self {
        Self {
            line,
            column,
            absolute_pos,
        }
    }
}
