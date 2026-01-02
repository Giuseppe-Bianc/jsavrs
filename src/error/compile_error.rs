use std::sync::Arc;

// src/error/compile_error.rs
use crate::error::error_code::ErrorCode;
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
/// Optionally, variants can include an [`ErrorCode`] for standardized error identification.
#[derive(Debug, Error)]
pub enum CompileError {
    /// Lexical analysis error indicating invalid token sequences.
    ///
    /// Contains:
    /// - `code`: Optional standardized error code (E0xxx range)
    /// - `message`: Human-readable error description
    /// - `span`: Source location where the error occurred
    /// - `help`: Optional guidance for fixing the error
    #[error("{}{message} at {span}{}",
        .code.map_or(String::new(), |c| format!("[{}] ", c.code())),
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    LexerError { code: Option<ErrorCode>, message: Arc<str>, span: SourceSpan, help: Option<String> },

    /// Syntax error indicating invalid program structure.
    ///
    /// Contains:
    /// - `code`: Optional standardized error code (E1xxx range)
    /// - `message`: Description of the syntax violation
    /// - `span`: Location of the problematic syntax
    /// - `help`: Optional guidance for fixing the error
    #[error("{}Syntax error: {message} at {span}{}",
        .code.map_or(String::new(), |c| format!("[{}] ", c.code())),
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    SyntaxError { code: Option<ErrorCode>, message: Arc<str>, span: SourceSpan, help: Option<String> },

    /// Type checking error indicating type mismatches or unsupported operations.
    ///
    /// Contains:
    /// - `code`: Optional standardized error code (E2xxx range)
    /// - `message`: Description of the type error
    /// - `span`: Location where the type error occurred
    /// - `help`: Optional guidance for fixing the error
    #[error("{}Type error: {message} at {span}{}",
        .code.map_or(String::new(), |c| format!("[{}] ", c.code())),
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    TypeError { code: Option<ErrorCode>, message: Arc<str>, span: SourceSpan, help: Option<String> },

    /// Error during intermediate representation (IR) generation.
    ///
    /// Contains:
    /// - `code`: Optional standardized error code (E3xxx range)
    /// - `message`: Description of the IR generation failure
    /// - `span`: Location associated with the error
    /// - `help`: Optional guidance for fixing the error
    #[error("{}IR generator error: {message} at {span}{}",
        .code.map_or(String::new(), |c| format!("[{}] ", c.code())),
        .help.as_ref().map_or(String::new(), |h| format!("\nhelp: {h}"))
    )]
    IrGeneratorError { code: Option<ErrorCode>, message: Arc<str>, span: SourceSpan, help: Option<String> },

    /// Error during assembly code generation.
    ///
    /// Contains:
    /// - `code`: Optional standardized error code (E4xxx range)
    /// - `message`: Description of the assembly generation failure
    #[error("{}Assembly generation error: {message}",
        .code.map_or(String::new(), |c| format!("[{}] ", c.code()))
    )]
    AsmGeneratorError { code: Option<ErrorCode>, message: Arc<str> },

    /// I/O operation failure during compilation (e.g., file access issues).
    ///
    /// Wraps the standard [`std::io::Error`] for seamless error propagation.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl CompileError {
    /// Returns the error code if one is associated with this error.
    ///
    /// # Returns
    /// - `Some(&ErrorCode)` for errors with associated codes
    /// - `None` for errors without codes or I/O errors
    ///
    /// # Examples
    /// ```
    /// use jsavrs::error::compile_error::CompileError;
    /// use jsavrs::error::error_code::ErrorCode;
    /// use jsavrs::location::source_span::SourceSpan;
    /// use std::sync::Arc;
    /// let err = CompileError::TypeError {
    ///     code: Some(ErrorCode::E2023),
    ///     message: Arc::from("Undefined variable 'x'"),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// assert_eq!(err.error_code(), Some(&ErrorCode::E2023));
    /// ```
    #[must_use]
    pub const fn error_code(&self) -> Option<&ErrorCode> {
        match self {
            Self::LexerError { code, .. }
            | Self::SyntaxError { code, .. }
            | Self::TypeError { code, .. }
            | Self::IrGeneratorError { code, .. }
            | Self::AsmGeneratorError { code, .. } => code.as_ref(),
            Self::IoError(_) => None,
        }
    }

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
    /// use std::sync::Arc;
    /// let err = CompileError::LexerError {
    ///     code: None,
    ///     message: Arc::from("Invalid token"),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// assert_eq!(err.message(), Some("Invalid token"));
    /// ```
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::LexerError { message, .. }
            | Self::SyntaxError { message, .. }
            | Self::TypeError { message, .. }
            | Self::IrGeneratorError { message, .. }
            | Self::AsmGeneratorError { message, .. } => Some(message),
            Self::IoError(_) => None,
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
    ///     code: None,
    ///     message: Arc::from("Unexpected token"),
    ///     span: span.clone(),
    ///     help: None,
    /// };
    /// assert_eq!(err.span(), Some(&span));
    /// ```
    #[must_use]
    pub const fn span(&self) -> Option<&SourceSpan> {
        match self {
            Self::LexerError { span, .. }
            | Self::SyntaxError { span, .. }
            | Self::TypeError { span, .. }
            | Self::IrGeneratorError { span, .. } => Some(span),
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
    /// use std::sync::Arc;
    /// let err = CompileError::TypeError {
    ///     code: None,
    ///     message: Arc::from("Type mismatch"),
    ///     span: SourceSpan::default(),
    ///     help: Some("Try adding a type annotation".to_string()),
    /// };
    /// assert_eq!(err.help(), Some("Try adding a type annotation"));
    /// ```
    #[must_use]
    pub fn help(&self) -> Option<&str> {
        match self {
            Self::LexerError { help, .. }
            | Self::SyntaxError { help, .. }
            | Self::TypeError { help, .. }
            | Self::IrGeneratorError { help, .. } => help.as_deref(),
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
    /// use std::sync::Arc;
    /// let mut err = CompileError::LexerError {
    ///     code: None,
    ///     message: Arc::from("Old message"),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// err.set_message(Arc::from("New message"));
    /// assert_eq!(err.message(), Some("New message"));
    /// ```
    pub fn set_message(&mut self, new_message: Arc<str>) {
        match self {
            Self::LexerError { message, .. }
            | Self::SyntaxError { message, .. }
            | Self::TypeError { message, .. }
            | Self::IrGeneratorError { message, .. }
            | Self::AsmGeneratorError { message, .. } => *message = new_message,
            Self::IoError(_) => {}
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
    ///     code: None,
    ///     message: Arc::from(""),
    ///     span: SourceSpan::new(Arc::from("file"), SourceLocation::new(1,1,1), SourceLocation::new(1,1,1)),
    ///     help: None,
    /// };
    /// let new_span = SourceSpan::new(Arc::from("file"), SourceLocation::new(1,2,1), SourceLocation::new(1,2,1));
    /// err.set_span(new_span.clone());
    /// assert_eq!(err.span(), Some(&new_span));
    /// ```
    pub fn set_span(&mut self, new_span: SourceSpan) {
        match self {
            Self::LexerError { span, .. }
            | Self::SyntaxError { span, .. }
            | Self::TypeError { span, .. }
            | Self::IrGeneratorError { span, .. } => *span = new_span,
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
    /// use std::sync::Arc;
    /// let mut err = CompileError::TypeError {
    ///     code: None,
    ///     message: Arc::from("Type mismatch"),
    ///     span: SourceSpan::default(),
    ///     help: None,
    /// };
    /// err.set_help(Some("Try adding a type annotation".to_string()));
    /// assert_eq!(err.help(), Some("Try adding a type annotation"));
    /// ```
    pub fn set_help(&mut self, new_help: Option<String>) {
        match self {
            Self::LexerError { help, .. }
            | Self::SyntaxError { help, .. }
            | Self::TypeError { help, .. }
            | Self::IrGeneratorError { help, .. } => *help = new_help,
            _ => {}
        }
    }
}
