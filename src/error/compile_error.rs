// src/error/compile_error.rs
use crate::location::source_span::SourceSpan;
use thiserror::Error;

/// Represents compilation errors that occur during different phases of compilation.
///
/// This enum categorizes errors into:
/// - Lexical analysis errors
/// - Syntax parsing errors
/// - Type checking errors
/// - Intermediate representation generation errors
/// - Assembly generation errors
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
    /// - `help`: Optional guidance for fixing the error
    #[error("{message} at {span}{}",
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    LexerError {
        message: String,
        span: SourceSpan,
        help: Option<String>,
    },

    /// Syntax error indicating invalid program structure.
    ///
    /// Contains:
    /// - `message`: Description of the syntax violation
    /// - `span`: Location of the problematic syntax
    /// - `help`: Optional guidance for fixing the error
    #[error("Syntax error: {message} at {span}{}",
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    SyntaxError {
        message: String,
        span: SourceSpan,
        help: Option<String>,
    },

    /// Type checking error indicating type mismatches or unsupported operations.
    ///
    /// Contains:
    /// - `message`: Description of the type error
    /// - `span`: Location where the type error occurred
    /// - `help`: Optional guidance for fixing the error
    #[error("Type error: {message} at {span}{}",
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    TypeError {
        message: String,
        span: SourceSpan,
        help: Option<String>,
    },

    /// Error during intermediate representation (IR) generation.
    ///
    /// Contains:
    /// - `message`: Description of the IR generation failure
    /// - `span`: Location associated with the error
    /// - `help`: Optional guidance for fixing the error
    #[error("Ir generator error: {message} at {span}{}",
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    IrGeneratorError {
        message: String,
        span: SourceSpan,
        help: Option<String>,
    },

    /// Error during assembly code generation.
    ///
    /// Contains:
    /// - `message`: Description of the assembly generation failure
    #[error("Assembly generation error: {message}")]
    AsmGeneratorError {
        message: String,
    },

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
    /// - `Some(&str)` for error variants with messages
    /// - `None` for `IoError` variant
    ///
    /// # Examples
    /// ```
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let err = CompileError::LexerError {
    ///     message: "Invalid token".to_string(),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// assert_eq!(err.message(), Some("Invalid token"));
    /// ```
    pub fn message(&self) -> Option<&str> {
        match self {
            CompileError::LexerError { message, .. } => Some(message),
            CompileError::SyntaxError { message, .. } => Some(message),
            CompileError::TypeError { message, .. } => Some(message),
            CompileError::IrGeneratorError { message, .. } => Some(message),
            CompileError::AsmGeneratorError { message } => Some(message),
            _ => None,
        }
    }

    /// Returns the source location span for relevant error variants.
    ///
    /// Returns:
    /// - `Some(&SourceSpan)` for variants with associated locations
    /// - `None` for variants without location information
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::location::source_location::SourceLocation;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let span = SourceSpan::new(Arc::from("file"), SourceLocation::new(1,1,1), SourceLocation::new(1,1,1));
    /// let err = CompileError::SyntaxError {
    ///     message: "Unexpected token".to_string(),
    ///     span: span.clone(),
    ///     help: None,
    /// };
    /// assert_eq!(err.span(), Some(&span));
    /// ```
    pub fn span(&self) -> Option<&SourceSpan> {
        match self {
            CompileError::LexerError { span, .. } => Some(span),
            CompileError::SyntaxError { span, .. } => Some(span),
            CompileError::TypeError { span, .. } => Some(span),
            CompileError::IrGeneratorError { span, .. } => Some(span),
            _ => None,
        }
    }

    /// Returns optional help guidance for fixing the error.
    ///
    /// Returns:
    /// - `Some(&str)` if help guidance exists
    /// - `None` if no help is available
    ///
    /// # Examples
    /// ```
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let err = CompileError::TypeError {
    ///     message: "Type mismatch".to_string(),
    ///     span: SourceSpan::default(),
    ///     help: Some("Try adding a type annotation".to_string()),
    /// };
    /// assert_eq!(err.help(), Some("Try adding a type annotation"));
    /// ```
    pub fn help(&self) -> Option<&str> {
        match self {
            CompileError::LexerError { help, .. } => help.as_deref(),
            CompileError::SyntaxError { help, .. } => help.as_deref(),
            CompileError::TypeError { help, .. } => help.as_deref(),
            CompileError::IrGeneratorError { help, .. } => help.as_deref(),
            _ => None,
        }
    }

    /// Updates the error message for variants that carry messages.
    ///
    /// No effect on variants without message fields.
    ///
    /// # Arguments
    /// * `new_message` - Replacement error message
    ///
    /// # Examples
    /// ```
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let mut err = CompileError::LexerError {
    ///     message: "Old message".to_string(),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// err.set_message("New message".to_string());
    /// assert_eq!(err.message(), Some("New message"));
    /// ```
    pub fn set_message(&mut self, new_message: String) {
        match self {
            CompileError::LexerError { message, .. } => *message = new_message,
            CompileError::SyntaxError { message, .. } => *message = new_message,
            CompileError::TypeError { message, .. } => *message = new_message,
            CompileError::IrGeneratorError { message, .. } => *message = new_message,
            CompileError::AsmGeneratorError { message } => *message = new_message,
            _ => {}
        }
    }

    /// Updates the source span for variants that carry location information.
    ///
    /// No effect on variants without span fields.
    ///
    /// # Arguments
    /// * `new_span` - Replacement source location
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::location::source_location::SourceLocation;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let mut err = CompileError::SyntaxError {
    ///     message: String::new(),
    ///     span: SourceSpan::new(Arc::from("file"), SourceLocation::new(1,1,1), SourceLocation::new(1,1,1)),
    ///     help: None,
    /// };
    /// let new_span = SourceSpan::new(Arc::from("file"), SourceLocation::new(1,2,1), SourceLocation::new(1,2,1));
    /// err.set_span(new_span.clone());
    /// assert_eq!(err.span(), Some(&new_span));
    /// ```
    pub fn set_span(&mut self, new_span: SourceSpan) {
        match self {
            CompileError::LexerError { span, .. } => *span = new_span,
            CompileError::SyntaxError { span, .. } => *span = new_span,
            CompileError::TypeError { span, .. } => *span = new_span,
            CompileError::IrGeneratorError { span, .. } => *span = new_span,
            _ => {}
        }
    }

    /// Updates the help guidance for relevant error variants.
    ///
    /// No effect on variants without help fields.
    ///
    /// # Arguments
    /// * `new_help` - New help message (or `None` to remove existing help)
    ///
    /// # Examples
    /// ```
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::location::source_span::SourceSpan;
    /// let mut err = CompileError::TypeError {
    ///     message: "Type mismatch".to_string(),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// err.set_help(Some("Try adding a type annotation".to_string()));
    /// assert_eq!(err.help(), Some("Try adding a type annotation"));
    /// ```
    pub fn set_help(&mut self, new_help: Option<String>) {
        match self {
            CompileError::LexerError { help, .. } => *help = new_help,
            CompileError::SyntaxError { help, .. } => *help = new_help,
            CompileError::TypeError { help, .. } => *help = new_help,
            CompileError::IrGeneratorError { help, .. } => *help = new_help,
            _ => {}
        }
    }
}