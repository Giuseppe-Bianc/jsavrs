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
