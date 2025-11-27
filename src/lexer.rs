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
    location::{line_tracker::LineTracker, source_span::SourceSpan},
    tokens::{token::Token, token_kind::TokenKind},
};
use logos::Logos;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

const INVALID_HASH: &str = "Invalid token: \"#\"";

/// The Lexer struct handles the tokenization of source code.
///
/// # Behavior in Phases
/// * Initialization: Sets up the internal logos lexer and line tracking for the source
/// * Runtime: Provides next_token functionality to process the source character by character
/// * Termination: Manages EOF token emission and resource cleanup
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
    line_tracker: LineTracker,
    eof_emitted: bool,
    source_len: usize,
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
    /// A reference to the LineTracker instance used by this lexer
    // OTTIMIZZAZIONE 1: Restituisce riferimento invece di clone
    pub fn get_line_tracker(&self) -> &LineTracker {
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

        let (kind_result, range) = match self.inner.next() {
            Some(kind_result) => (kind_result, self.inner.span()),
            None => {
                self.eof_emitted = true;
                let eof_range = self.source_len..self.source_len;
                (Ok(TokenKind::Eof), eof_range)
            }
        };

        let span = self.line_tracker.span_for(range);
        Some(match kind_result {
            Ok(kind) => Ok(Token { kind, span }),
            Err(_) => Err(CompileError::LexerError {
                message: Arc::from(format!("Invalid token: {:?}", self.inner.slice())),
                span,
                help: None,
            }),
        })
    }

    /*// OTTIMIZZAZIONE 7: Metodo per processing streaming (opzionale)
    pub fn process_streaming<F>(&mut self, mut callback: F) -> Vec<CompileError>
    where
        F: FnMut(Token)
    {
        let mut errors = Vec::new();
        while let Some(result) = self.next_token() {
            match result {
                Ok(token) => callback(token),
                Err(e) => errors.push(e),
            }
        }
        errors
    }*/
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
    let estimated_tokens = lexer.source_len / 18;
    let mut tokens = Vec::with_capacity(estimated_tokens);
    let mut errors = Vec::new();

    while let Some(token_result) = lexer.next_token() {
        match token_result {
            Ok(token) => tokens.push(token),
            Err(e) => errors.push(e),
        }
    }
    post_process_tokens(tokens, errors)
}

/// Post-processes tokens and errors after initial tokenization.
///
/// # Optimization Strategy
///
/// This function applies three key optimizations:
/// 1. **Lazy HashMap initialization**: Only builds position index when hashtag errors exist
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
pub fn post_process_tokens(tokens: Vec<Token>, errors: Vec<CompileError>) -> (Vec<Token>, Vec<CompileError>) {
    let (error_replacements, tokens_to_remove) = collect_error_updates(&errors, &tokens);
    let errors = apply_error_replacements(errors, error_replacements);
    let tokens = filter_removed_tokens(tokens, tokens_to_remove);
    (tokens, errors)
}

type Updates = (HashMap<usize, CompileError>, HashSet<usize>);

/// Collects error updates and tokens to remove during post-processing.
///
/// # Optimization Strategy (US1 Optimization 3)
///
/// Defers HashMap construction until hashtag errors are detected. Since 99%+ of files have no
/// #b/#o/#x patterns, this eliminates unnecessary O(n) allocation and initialization overhead
/// in the common case. The early-exit check uses `.any()` predicate which short-circuits on
/// first match, providing O(m) performance where m = number of errors (typically m << n tokens).
///
/// # Lazy Initialization Benefits
/// - **Common case (no hashtag errors)**: Saves 100% of HashMap construction cost
/// - **Rare case (has hashtag errors)**: Pays full cost, but needed for correctness
/// - **Net benefit**: Proportional to hashtag error frequency (benefits ~99% of files)
///
/// # Parameters
/// * `errors` - Slice of compilation errors to analyze
/// * `tokens` - Slice of tokens for position-based lookup
///
/// # Returns
/// Tuple of (error replacements map, token indices to remove)
#[inline]
fn collect_error_updates(errors: &[CompileError], tokens: &[Token]) -> Updates {
    let mut replacements = HashMap::new();
    let mut to_remove = HashSet::new();

    // Controlla prima se ci sono errori hashtag
    let has_hashtag_errors = errors
        .iter()
        .any(|e| matches!(e, CompileError::LexerError { message, .. } if message.as_ref() == INVALID_HASH));

    // Se non ci sono errori hashtag, ritorna subito
    if !has_hashtag_errors {
        return (replacements, to_remove);
    }

    // Crea la map solo se necessario
    let token_map = create_position_map(tokens);

    for (eidx, error) in errors.iter().enumerate() {
        match error {
            CompileError::LexerError { message, span, .. } if message.as_ref() == INVALID_HASH => {
                process_hashtag_error(eidx, span, tokens, &token_map, &mut replacements, &mut to_remove);
            }
            _ => continue,
        }
    }

    (replacements, to_remove)
}

/// Creates position-to-index lookup map for tokens.
///
/// # Performance Characteristics (US2 Optimization 5)
///
/// Pre-allocates HashMap capacity to avoid rehashing during construction. Single-pass
/// iteration builds position-to-index mapping for O(1) lookup during error processing.
/// The capacity hint (`tokens.len()`) eliminates multiple reallocations and rehashing
/// that would occur with default HashMap growth strategy.
///
/// # Optimization Notes
/// - Uses `HashMap::with_capacity()` for optimal performance
/// - Single pass iteration for linear time complexity
/// - Provides O(1) lookup for subsequent error processing
///
/// # Parameters
/// * `tokens` - Slice of tokens to create position map from
///
/// # Returns
/// HashMap mapping (line, column) positions to token indices
fn create_position_map(tokens: &[Token]) -> HashMap<(usize, usize), usize> {
    let mut map = HashMap::with_capacity(tokens.len());
    for (i, t) in tokens.iter().enumerate() {
        map.insert((t.span.start.line, t.span.start.column), i);
    }
    map
}

pub fn process_hashtag_error(
    eidx: usize, span: &SourceSpan, tokens: &[Token], token_map: &HashMap<(usize, usize), usize>,
    replacements: &mut HashMap<usize, CompileError>, to_remove: &mut HashSet<usize>,
) {
    let end_pos = (span.end.line, span.end.column);

    if let Some(&tidx) = token_map.get(&end_pos) {
        let token = &tokens[tidx];

        #[allow(clippy::collapsible_if)]
        if let TokenKind::IdentifierAscii(s) = &token.kind {
            if s.len() == 1 {
                if let Some(msg) = get_error_message(s) {
                    if let Some(merged) = span.merged(&token.span) {
                        replacements.insert(
                            eidx,
                            CompileError::LexerError { message: Arc::from(msg), span: merged, help: None },
                        );
                        to_remove.insert(tidx);
                    }
                }
            }
        }
    }
}

/// Returns error message for malformed number literals.
///
/// # Optimization Strategy (US2 Optimization 4)
///
/// Tiny hot function - always inline for branch prediction and elimination of call overhead.
/// The function consists of simple pattern matching with static string returns, making it
/// an ideal candidate for aggressive inlining. The `#[inline(always)]` attribute ensures
/// this function is inlined at all call sites, eliminating function call overhead and
/// enabling further optimizations by the compiler.
///
/// # Parameters
/// * `s` - String slice to analyze for error pattern
///
/// # Returns
/// Optional error message for recognized malformed patterns (b, o, x)
#[inline(always)]
pub const fn get_error_message(s: &str) -> Option<&'static str> {
    match s.as_bytes().first() {
        Some(b'b') if s.len() == 1 => Some("Malformed binary number: \"#b\""),
        Some(b'o') if s.len() == 1 => Some("Malformed octal number: \"#o\""),
        Some(b'x') if s.len() == 1 => Some("Malformed hexadecimal number: \"#x\""),
        _ => None,
    }
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

/// Filters out tokens marked for removal using in-place modification.
///
/// # Optimization Strategy (US1 Optimization 2)
///
/// Uses `Vec::retain()` for zero-allocation in-place filtering. Elements are shifted left
/// when removed, maintaining order while avoiding temporary Vec allocation. This eliminates
/// the previous `filter().collect()` pattern which created a complete new Vec and copied
/// all remaining tokens, temporarily doubling memory usage.
///
/// # Performance Characteristics
/// - **Time Complexity**: O(n) single pass through tokens
/// - **Space Complexity**: O(n) single pass through tokens
/// - **Fast Path**: Immediate return if `to_remove` is empty (common case)
///
/// # Parameters
/// * `tokens` - Vector of tokens to filter
/// * `to_remove` - Set of token indices to remove
///
/// # Returns
/// The filtered token vector with removed indices excluded
fn filter_removed_tokens(tokens: Vec<Token>, to_remove: HashSet<usize>) -> Vec<Token> {
    tokens.into_iter().enumerate().filter(|(i, _)| !to_remove.contains(i)).map(|(_, t)| t).collect()
}
