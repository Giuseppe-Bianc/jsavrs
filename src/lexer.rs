//src/lexer.rs
use crate::{
    error::compile_error::CompileError,
    location::line_tracker::LineTracker,
    tokens::{token::Token, token_kind::TokenKind},
};
use logos::Logos;

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
    line_tracker: LineTracker,
    eof_emitted: bool,
    source_len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file_path: &str, source: &'a str) -> Self {
        let line_tracker = LineTracker::new(file_path, source.to_owned());
        let inner = TokenKind::lexer(source);
        let source_len = source.len();
        Lexer {
            inner,
            line_tracker,
            eof_emitted: false,
            source_len,
        }
    }

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
                message: format!("Invalid token: {:?}", self.inner.slice()),
                span,
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

pub fn lexer_tokenize_with_errors(
    input: &str,
    file_path_str: &str,
) -> (Vec<Token>, Vec<CompileError>) {
    let mut lexer = Lexer::new(file_path_str, input);
    let mut tokens = Vec::with_capacity(input.len() / 4);
    let mut errors = Vec::new();

    while let Some(token_result) = lexer.next_token() {
        match token_result {
            Ok(token) => tokens.push(token),
            Err(e) => errors.push(e),
        }
    }
    (tokens, errors)
}
