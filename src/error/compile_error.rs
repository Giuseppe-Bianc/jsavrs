// src/error/compile_error.rs
use crate::location::source_span::SourceSpan;
use thiserror::Error;

/// Represents compilation errors that occur during different phases of compilation.
///
/// This enum categorizes errors into:
/// - Lexical analysis errors
/// - Syntax parsing errors
/// - General I/O errors
///
/// Each variant carries context-specific information about the error's nature and location.
#[derive(Debug, Error)]
pub enum CompileError {
    /// Lexical analysis error indicating invalid token sequences.
    ///
    /// Contains:
    /// - `message`: Human-readable error description
    /// - `span`: Source location where the error occurred
    #[error("{message} at {span}")]
    LexerError { message: String, span: SourceSpan },

    /// Syntax error indicating invalid program structure.
    ///
    /// Contains:
    /// - `message`: Description of the syntax violation
    /// - `span`: Location of the problematic syntax
    #[error("Syntax error: {message} at {span}")]
    SyntaxError { message: String, span: SourceSpan },

    /// I/O operation failure during compilation (e.g., file access issues).
    ///
    /// Wraps the standard [`std::io::Error`] for seamless error propagation.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl CompileError {
    /// Returns the error message for variants that carry messages.
    ///
    /// Returns:
    /// - `Some(&str)` for `LexerError`/`SyntaxError` variants
    /// - `None` for `IoError` variant
    ///
    /// # Examples
    /// ```
    /// let err = CompileError::LexerError {
    ///     message: "Invalid token".to_string(),
    ///     span: SourceSpan::default(),
    /// };
    /// assert_eq!(err.message(), Some("Invalid token"));
    /// ```
    pub fn message(&self) -> Option<&str> {
        match self {
            CompileError::LexerError { message, .. } => Some(message),
            CompileError::SyntaxError { message, .. } => Some(message),
            _ => None,
        }
    }

    /// Returns the source location span for relevant error variants.
    ///
    /// Returns:
    /// - `Some(&SourceSpan)` for `LexerError`/`SyntaxError`
    /// - `None` for `IoError`
    ///
    /// # Examples
    /// ```
    /// let span = SourceSpan::new(0, 5);
    /// let err = CompileError::SyntaxError {
    ///     message: "Unexpected token".to_string(),
    ///     span: span.clone(),
    /// };
    /// assert_eq!(err.span(), Some(&span));
    /// ```
    pub fn span(&self) -> Option<&SourceSpan> {
        match self {
            CompileError::LexerError { span, .. } => Some(span),
            CompileError::SyntaxError { span, .. } => Some(span),
            _ => None,
        }
    }

    /// Updates the error message for `LexerError` and `SyntaxError` variants.
    ///
    /// No effect on `IoError` variant.
    ///
    /// # Arguments
    /// * `new_message` - Replacement error message
    ///
    /// # Examples
    /// ```
    /// let mut err = CompileError::LexerError {
    ///     message: "Old message".to_string(),
    ///     span: SourceSpan::default(),
    /// };
    /// err.set_message("New message".to_string());
    /// assert_eq!(err.message(), Some("New message"));
    /// ```
    pub fn set_message(&mut self, new_message: String) {
        match self {
            CompileError::LexerError { message, .. } => *message = new_message,
            CompileError::SyntaxError { message, .. } => *message = new_message,
            _ => {}
        }
    }

    /// Updates the source span for `LexerError` and `SyntaxError` variants.
    ///
    /// No effect on `IoError` variant.
    ///
    /// # Arguments
    /// * `new_span` - Replacement source location
    ///
    /// # Examples
    /// ```
    /// let mut err = CompileError::SyntaxError {
    ///     message: String::new(),
    ///     span: SourceSpan::new(0, 5),
    /// };
    /// let new_span = SourceSpan::new(10, 15);
    /// err.set_span(new_span.clone());
    /// assert_eq!(err.span(), Some(&new_span));
    /// ```
    pub fn set_span(&mut self, new_span: SourceSpan) {
        match self {
            CompileError::LexerError { span, .. } => *span = new_span,
            CompileError::SyntaxError { span, .. } => *span = new_span,
            _ => {}
        }
    }
}
