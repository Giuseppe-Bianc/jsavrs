use crate::location::source_span::SourceSpan;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("{message} at {span}")]
    LexerError { message: String, span: SourceSpan },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_display() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: CompileError = io_error.into();
        assert_eq!(format!("{}", error), "I/O error: File not found");
    }
}
