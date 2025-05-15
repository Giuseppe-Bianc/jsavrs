use crate::location::source_span::SourceSpan;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("{message} at {span}")]
    LexerError { message: String, span: SourceSpan },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl CompileError {
    /// Get the error message if this is a LexerError
    pub fn message(&self) -> Option<&str> {
        match self {
            CompileError::LexerError { message, .. } => Some(message),
            _ => None,
        }
    }

    /// Get the source span if this is a LexerError
    pub fn span(&self) -> Option<&SourceSpan> {
        match self {
            CompileError::LexerError { span, .. } => Some(span),
            _ => None,
        }
    }

    /// Update the error message for LexerError variants
    pub fn set_message(&mut self, new_message: String) {
        if let CompileError::LexerError { message, .. } = self {
            *message = new_message;
        }
    }

    /// Update the source span for LexerError variants
    pub fn set_span(&mut self, new_span: SourceSpan) {
        if let CompileError::LexerError { span, .. } = self {
            *span = new_span;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::location::source_location::SourceLocation;
    use std::sync::Arc;

    #[test]
    fn test_io_error_display() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: CompileError = io_error.into();
        assert_eq!(format!("{}", error), "I/O error: File not found");
    }

    #[test]
    fn test_lexer_error_display() {
        let span = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(1, 1, 0),
            SourceLocation::new(1, 2, 1),
        );
        let error = CompileError::LexerError {
            message: "Unexpected token \"@\"".to_string(),
            span,
        };
        assert_eq!(
            format!("{}", error),
            "Unexpected token \"@\" at test_file:1:1-1:2"
        );
    }

    #[test]
    fn test_lexer_error_message() {
        let span = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(1, 1, 0),
            SourceLocation::new(1, 2, 1),
        );
        let error = CompileError::LexerError {
            message: "Unexpected token \"@\"".to_string(),
            span,
        };
        assert_eq!(error.message(), Some("Unexpected token \"@\""));
    }
    #[test]
    fn test_lexer_error_span() {
        let span = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(1, 1, 0),
            SourceLocation::new(1, 2, 1),
        );
        let error = CompileError::LexerError {
            message: "Unexpected token \"@\"".to_string(),
            span,
        };
        assert_eq!(error.span().unwrap().start.line, 1);
        assert_eq!(error.span().unwrap().end.line, 1);
        assert_eq!(error.span().unwrap().start.column, 1);
        assert_eq!(error.span().unwrap().end.column, 2);
        assert_eq!(error.span().unwrap().start.absolute_pos, 0);
        assert_eq!(error.span().unwrap().end.absolute_pos, 1);
    }

    #[test]
    fn test_set_message() {
        let span = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(1, 1, 0),
            SourceLocation::new(1, 2, 1),
        );
        let mut error = CompileError::LexerError {
            message: "Unexpected token \"@\"".to_string(),
            span,
        };
        error.set_message("New message".to_string());
        assert_eq!(error.message(), Some("New message"));
    }

    #[test]
    fn test_set_span() {
        let span1 = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(1, 1, 0),
            SourceLocation::new(1, 2, 1),
        );
        let span2 = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(2, 1, 2),
            SourceLocation::new(2, 2, 3),
        );
        let mut error = CompileError::LexerError {
            message: "Unexpected token \"@\"".to_string(),
            span: span1,
        };
        error.set_span(span2);
        assert_eq!(error.span().unwrap().start.line, 2);
        assert_eq!(error.span().unwrap().end.line, 2);
        assert_eq!(error.span().unwrap().start.column, 1);
        assert_eq!(error.span().unwrap().end.column, 2);
        assert_eq!(error.span().unwrap().start.absolute_pos, 2);
        assert_eq!(error.span().unwrap().end.absolute_pos, 3);
    }
    #[test]
    fn test_set_message_not_lexer_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let mut error: CompileError = io_error.into();
        error.set_message("New message".to_string());
        assert_eq!(error.message(), None);
    }
    #[test]
    fn test_set_span_not_lexer_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let mut error: CompileError = io_error.into();
        let span = SourceSpan::new(
            Arc::from("test_file"),
            SourceLocation::new(1, 1, 0),
            SourceLocation::new(1, 2, 1),
        );
        error.set_span(span);
        assert_eq!(error.span(), None);
    }
    #[test]
    fn test_get_span_non_lexer_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: CompileError = io_error.into();
        assert_eq!(error.span(), None);
    }
    #[test]
    fn test_get_message_non_lexer_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: CompileError = io_error.into();
        assert_eq!(error.message(), None);
    }
}
