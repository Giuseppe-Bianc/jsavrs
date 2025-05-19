// src/error/compile_error.rs
use crate::location::source_span::SourceSpan;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("{message} at {span}")]
    LexerError { message: String, span: SourceSpan },

    #[error("Syntax error: {message} at {span}")]
    SyntaxError { message: String, span: SourceSpan },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl CompileError {
    // Existing methods updated to handle SyntaxError
    pub fn message(&self) -> Option<&str> {
        match self {
            CompileError::LexerError { message, .. } => Some(message),
            CompileError::SyntaxError { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn span(&self) -> Option<&SourceSpan> {
        match self {
            CompileError::LexerError { span, .. } => Some(span),
            CompileError::SyntaxError { span, .. } => Some(span),
            _ => None,
        }
    }

    /// Aggiorna il messaggio di errore per le varianti LexerError e SyntaxError
    pub fn set_message(&mut self, new_message: String) {
        match self {
            CompileError::LexerError { message, .. } => *message = new_message,
            CompileError::SyntaxError { message, .. } => *message = new_message,
            _ => {}
        }
    }

    /// Aggiorna lo span per le varianti LexerError e SyntaxError
    pub fn set_span(&mut self, new_span: SourceSpan) {
        match self {
            CompileError::LexerError { span, .. } => *span = new_span,
            CompileError::SyntaxError { span, .. } => *span = new_span,
            _ => {}
        }
    }
}
