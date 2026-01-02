// src/lexer.rs
/// # Lexer Module
///
/// The lexer module handles the transformation of source text into tokens.
/// This is the first phase of the compilation process, responsible for
/// recognizing language keywords, identifiers, literals, and operators.
///
/// ## Phase-specific responsibilities:
/// * Initialization: Sets up character scanning and token recognition patterns using logos
/// * Runtime: Processes input character stream, identifying and categorizing tokens
/// * Termination: Finalizes token output, ensuring proper stream termination
use crate::{
    error::compile_error::CompileError,
    error::error_code::ErrorCode,
    location::line_tracker::LineTracker,
    tokens::{token::Token, token_kind::TokenKind},
};
use logos::Logos;
use std::{collections::HashMap, sync::Arc};

/// The Lexer struct handles the tokenization of source code.
///
/// # Behavior in Phases
/// * Initialization: Sets up the internal logos lexer and line tracking for the source
/// * Runtime: Provides `next_token` functionality to process the source character by character
/// * Termination: Manages EOF token emission and resource cleanup
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
    line_tracker: LineTracker,
    source_len: usize, // Move before bool
    eof_emitted: bool,
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer instance for the given file path and source code.
    ///
    /// # Behavior in Phases
    /// * Initialization: Sets up the logos lexer with the provided source and creates a line tracker
    /// * Runtime: Not applicable - this is a setup method
    /// * Termination: Not applicable - this is a setup method
    ///
    /// # Parameters
    /// * `file_path` - Path to the source file being lexed
    /// * `source` - Reference to the source code string to be tokenized
    ///
    /// # Returns
    /// A new Lexer instance ready to tokenize the source code
    ///
    /// # Examples
    /// ```
    /// # use jsavrs::lexer::Lexer;
    /// let source = "let x = 42;";
    /// let mut lexer = Lexer::new("test.vn", source);
    /// ```
    #[must_use]
    pub fn new(file_path: &str, source: &'a str) -> Self {
        let line_tracker = LineTracker::new(file_path, source.to_owned());
        let inner = TokenKind::lexer(source);
        let source_len = source.len();
        Lexer { inner, line_tracker, eof_emitted: false, source_len }
    }

    /// Returns a reference to the line tracker containing position information.
    ///
    /// # Behavior in Phases
    /// * Initialization: Provides access to position tracking set up during lexer creation
    /// * Runtime: Used to retrieve position information for error reporting
    /// * Termination: Provides final position information when processing completes
    ///
    /// # Returns
    /// A reference to the `LineTracker` instance used by this lexer
    // OTTIMIZZAZIONE 1: Restituisce riferimento invece di clone
    #[must_use]
    pub const fn get_line_tracker(&self) -> &LineTracker {
        &self.line_tracker
    }

    /// Retrieves the next token from the source code.
    ///
    /// # Behavior in Phases
    /// * Initialization: Not applicable - this processes existing setup
    /// * Runtime: Processes the next sequence of characters to identify a token
    /// * Termination: Eventually emits an EOF token to signal end of input
    ///
    /// # Returns
    /// * `Some(Ok(Token))` - A successfully identified token
    /// * `Some(Err(CompileError))` - An error occurred during tokenization
    /// * `None` - End of file has been reached
    ///
    /// # Examples
    /// ```
    /// # use jsavrs::lexer::Lexer;
    /// let mut lexer = Lexer::new("test.vn", "let x = 42;");
    /// if let Some(Ok(token)) = lexer.next_token() {
    ///     // Process the token
    /// }
    /// ```
    #[inline]
    pub fn next_token(&mut self) -> Option<Result<Token, CompileError>> {
        if self.eof_emitted {
            return None;
        }

        let (kind_result, range) = if let Some(kind_result) = self.inner.next() {
            (kind_result, self.inner.span())
        } else {
            self.eof_emitted = true;
            let eof_range = self.source_len..self.source_len;
            (Ok(TokenKind::Eof), eof_range)
        };

        let span = self.line_tracker.span_for(range);
        Some(match kind_result {
            Ok(kind) => Ok(Token { kind, span }),
            Err(()) => Err(CompileError::LexerError {
                code: Some(ErrorCode::E0001),
                message: Arc::from(format!("Invalid token: {:?}", self.inner.slice())),
                span,
                help: None,
            }),
        })
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, CompileError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

/// Tokenizes the source code using the provided lexer, collecting both tokens and errors.
///
/// # Behavior in Phases
/// * Initialization: Sets up vectors with estimated capacity for tokens and errors
/// * Runtime: Processes all tokens from the lexer, separating valid tokens from errors
/// * Termination: Performs post-processing to handle special error cases like hashtag tokens
///
/// # Parameters
/// * `lexer` - A mutable reference to the Lexer instance to use for tokenization
///
/// # Returns
/// A tuple containing (valid tokens, compilation errors) found during tokenization
///
/// # Examples
/// ```
/// # use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
/// let mut lexer = Lexer::new("test.vn", "let x = 42;");
/// let (tokens, errors) = lexer_tokenize_with_errors(&mut lexer);
/// assert!(errors.is_empty());
/// ```
pub fn lexer_tokenize_with_errors(lexer: &mut Lexer) -> (Vec<Token>, Vec<CompileError>) {
    let estimated_tokens = lexer.source_len / 8;
    let mut tokens = Vec::with_capacity(estimated_tokens);
    let mut errors = Vec::with_capacity(4);

    while let Some(token_result) = lexer.next_token() {
        match token_result {
            Ok(token) => tokens.push(token),
            Err(e) => errors.push(e),
        }
    }
    post_process_tokens(tokens, errors)
}

#[inline]
fn has_malformed_errors(errors: &[CompileError]) -> bool {
    errors.iter().any(
        |e| matches!(e, CompileError::LexerError { message, .. } if matches!(message.as_ref(), "Invalid token: \"#b\"" | "Invalid token: \"#o\"" | "Invalid token: \"#x\"")),
    )
}

/// Post-processes tokens and errors after initial tokenization.
///
/// # Optimization Strategy
///
/// This function applies three key optimizations:
/// 1. **Lazy `HashMap` initialization**: Only builds position index when hashtag errors exist
/// 2. **In-place token filtering**: Uses `retain()` instead of `filter().collect()`
/// 3. **Early-exit pattern**: Skips expensive operations when not needed
///
/// # Performance Characteristics
/// - **Best case (no hashtag errors)**: O(m) where m = error count
/// - **Worst case (has hashtag errors)**: O(n + m) where n = token count
///
/// # Parameters
/// * `tokens` - Vector of tokens to post-process
/// * `errors` - Vector of errors to enhance
///
/// # Returns
/// Tuple of (filtered tokens, enhanced errors)
#[inline]
#[must_use]
pub fn post_process_tokens(tokens: Vec<Token>, errors: Vec<CompileError>) -> (Vec<Token>, Vec<CompileError>) {
    // Se non ci sono errori hashtag, ritorna subito
    if !has_malformed_errors(&errors) {
        return (tokens, errors);
    }

    let mut replacements = HashMap::new();

    for (eidx, error) in errors.iter().enumerate() {
        if let CompileError::LexerError { message, span, help, .. } = error
            && let Some((msg, error_code)) = extract_malformed_base_number_message(message.as_ref())
        {
            replacements.insert(
                eidx,
                CompileError::LexerError {
                    code: Some(error_code),
                    message: Arc::from(msg),
                    span: span.clone(),
                    help: help.clone(),
                },
            );
        }
    }
    let errors = apply_error_replacements(errors, replacements);
    //let tokens = filter_removed_tokens(tokens, tokens_to_remove);
    (tokens, errors)
}

fn apply_error_replacements(
    mut errors: Vec<CompileError>, mut replacements: HashMap<usize, CompileError>,
) -> Vec<CompileError> {
    for (i, error) in errors.iter_mut().enumerate() {
        if let Some(new_err) = replacements.remove(&i) {
            *error = new_err;
        }
    }
    errors
}

/// Extracts the appropriate error message for malformed base numbers.
///
/// Converts generic error messages into specific, user-friendly messages
/// and returns the corresponding error code.
///
/// # Parameters
/// * `msg` - The error message from logos
///
/// # Returns
/// Optional tuple of (specific error message, error code) if the input matches a malformed base number pattern
#[inline]
const fn extract_malformed_base_number_message(msg: &str) -> Option<(&'static str, ErrorCode)> {
    match msg.as_bytes() {
        b"Invalid token: \"#b\"" => Some(("Malformed binary number: \"#b\"", ErrorCode::E0002)),
        b"Invalid token: \"#o\"" => Some(("Malformed octal number: \"#o\"", ErrorCode::E0003)),
        b"Invalid token: \"#x\"" => Some(("Malformed hexadecimal number: \"#x\"", ErrorCode::E0004)),
        _ => None,
    }
}
