// src/tokens/token.rs
use crate::location::source_span::SourceSpan;
use crate::tokens::token_kind::TokenKind;

/// Represents a lexical token with its type and location in source code.
///
/// Tokens are the fundamental building blocks produced by the lexer during
/// lexical analysis. Each token captures:
/// - Its semantic type (identifier, keyword, literal, operator, etc.)
/// - Its exact location in the source file (file path, start/end positions)
///
/// This combination allows for precise error reporting and enables the parser
/// to maintain source location information throughout the compilation pipeline.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// The semantic type of the token.
    ///
    /// Determines how the token will be interpreted by the parser.
    /// See [`TokenKind`] for available token types.
    pub kind: TokenKind,

    /// The source location span of the token.
    ///
    /// Captures:
    /// - Source file path
    /// - Starting position (inclusive)
    /// - Ending position (exclusive)
    ///
    /// Used for error reporting, debugging, and source mapping.
    pub span: SourceSpan,
}
