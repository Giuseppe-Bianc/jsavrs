#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_location_new_and_fields() {
        let loc = SourceLocation::new(3, 5, 42);
        assert_eq!(loc.line, 3);
        assert_eq!(loc.column, 5);
        assert_eq!(loc.absolute_pos, 42);
    }
}
