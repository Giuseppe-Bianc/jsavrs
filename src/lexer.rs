use std::collections::{HashMap, HashSet};
//src/lexer.rs
use crate::{
    error::compile_error::CompileError,
    location:: {
        source_span::SourceSpan,
        line_tracker::LineTracker
    },
    tokens::{token::Token, token_kind::TokenKind}
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
    post_process_tokens(tokens, errors)
}


pub fn post_process_tokens(
    tokens: Vec<Token>,
    errors: Vec<CompileError>,
) -> (Vec<Token>, Vec<CompileError>) {
    let (error_replacements, tokens_to_remove) = collect_error_updates(&errors, &tokens);
    let errors = apply_error_replacements(errors, error_replacements);
    let tokens = filter_removed_tokens(tokens, tokens_to_remove);
    (tokens, errors)
}

type Updates = (HashMap<usize, CompileError>, HashSet<usize>);

fn collect_error_updates(errors: &[CompileError], tokens: &[Token]) -> Updates {
    let mut replacements = HashMap::new();
    let mut to_remove = HashSet::new();
    let token_map = create_position_map(tokens);

    for (eidx, error) in errors.iter().enumerate() {
        match error {
            CompileError::LexerError { message, span } if message == "Invalid token: \"#\"" => {
                process_hashtag_error(eidx, span, tokens, &token_map, &mut replacements, &mut to_remove);
            }
            _ => {continue}
        }

    }

    (replacements, to_remove)
}

fn create_position_map(tokens: &[Token]) -> HashMap<(usize, usize), usize> {
    tokens
        .iter()
        .enumerate()
        .map(|(i, t)| ((t.span.start.line, t.span.start.column), i))
        .collect()
}

fn process_hashtag_error(
    eidx: usize,
    span: &SourceSpan,
    tokens: &[Token],
    token_map: &HashMap<(usize, usize), usize>,
    replacements: &mut HashMap<usize, CompileError>,
    to_remove: &mut HashSet<usize>,
) {
    let end_pos = (span.end.line, span.end.column);

    if let Some(&tidx) = token_map.get(&end_pos) {
        let token = &tokens[tidx];

        if let TokenKind::IdentifierAscii(s) = &token.kind {
            if s.len() == 1 {
                if let Some(msg) = get_error_message(s) {
                    if let Some(merged) = span.merged(&token.span) {
                        replacements.insert(eidx, CompileError::LexerError {
                            message: msg.to_string(),
                            span: merged,
                        });
                        to_remove.insert(tidx);
                    }
                }
            }
        }
    }
}

fn get_error_message(s: &str) -> Option<&'static str> {
    match s {
        "b" => Some("Malformed binary number: \"#b\""),
        "o" => Some("Malformed octal number: \"#o\""),
        "x" => Some("Malformed hexadecimal number: \"#x\""),
        _ => None,
    }
}

fn apply_error_replacements(
    mut errors: Vec<CompileError>,
    mut replacements: HashMap<usize, CompileError>,
) -> Vec<CompileError> {
    for (i, error) in errors.iter_mut().enumerate() {
        if let Some(new_err) = replacements.remove(&i) {
            *error = new_err;
        }
    }
    errors
}

fn filter_removed_tokens(tokens: Vec<Token>, to_remove: HashSet<usize>) -> Vec<Token> {
    tokens
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !to_remove.contains(i))
        .map(|(_, t)| t)
        .collect()
}