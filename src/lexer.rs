// src/lexer.rs
use crate::{
    error::compile_error::CompileError,
    location::{line_tracker::LineTracker, source_span::SourceSpan},
    tokens::{token::Token, token_kind::TokenKind},
};
use logos::Logos;
use std::collections::{HashMap, HashSet};

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
        Lexer { inner, line_tracker, eof_emitted: false, source_len }
    }

    // OTTIMIZZAZIONE 1: Restituisce riferimento invece di clone
    pub fn get_line_tracker(&self) -> LineTracker {
        self.line_tracker.clone()
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

pub fn lexer_tokenize_with_errors(lexer: &mut Lexer) -> (Vec<Token>, Vec<CompileError>) {
    let estimated_tokens = lexer.source_len / 5;
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

pub fn post_process_tokens(tokens: Vec<Token>, errors: Vec<CompileError>) -> (Vec<Token>, Vec<CompileError>) {
    let (error_replacements, tokens_to_remove) = collect_error_updates(&errors, &tokens);
    let errors = apply_error_replacements(errors, error_replacements);
    let tokens = filter_removed_tokens(tokens, tokens_to_remove);
    (tokens, errors)
}

type Updates = (HashMap<usize, CompileError>, HashSet<usize>);

fn collect_error_updates(errors: &[CompileError], tokens: &[Token]) -> Updates {
    let mut replacements = HashMap::new();
    let mut to_remove = HashSet::new();

    // Controlla prima se ci sono errori hashtag
    let has_hashtag_errors = errors
        .iter()
        .any(|e| matches!(e, CompileError::LexerError { message, .. } if message == "Invalid token: \"#\""));

    // Se non ci sono errori hashtag, ritorna subito
    if !has_hashtag_errors {
        return (replacements, to_remove);
    }

    // Crea la map solo se necessario
    let token_map = create_position_map(tokens);

    for (eidx, error) in errors.iter().enumerate() {
        match error {
            CompileError::LexerError { message, span, .. } if message == "Invalid token: \"#\"" => {
                process_hashtag_error(eidx, span, tokens, &token_map, &mut replacements, &mut to_remove);
            }
            _ => continue,
        }
    }

    (replacements, to_remove)
}

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
                            CompileError::LexerError { message: msg.to_string(), span: merged, help: None },
                        );
                        to_remove.insert(tidx);
                    }
                }
            }
        }
    }
}

pub fn get_error_message(s: &str) -> Option<&'static str> {
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

fn filter_removed_tokens(tokens: Vec<Token>, to_remove: HashSet<usize>) -> Vec<Token> {
    tokens.into_iter().enumerate().filter(|(i, _)| !to_remove.contains(i)).map(|(_, t)| t).collect()
}
