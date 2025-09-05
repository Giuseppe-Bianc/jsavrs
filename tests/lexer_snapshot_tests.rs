use insta::assert_snapshot;
use jsavrs::{
    error::compile_error::CompileError,
    lexer::*,
    location::{source_location::SourceLocation, source_span::SourceSpan},
    tokens::{number::Number::*, token::Token, token_kind::TokenKind},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    sync::Arc,
};

/// Helper to run a hashtag error scenario and return formatted output
fn run_hashtag_error(
    eidx: usize, span: SourceSpan, tokens: Vec<Token>, token_map: HashMap<(usize, usize), usize>,
) -> String {
    let mut replacements: HashMap<usize, CompileError> = HashMap::new();
    let mut to_remove: HashSet<usize> = HashSet::new();
    process_hashtag_error(eidx, &span, &tokens, &token_map, &mut replacements, &mut to_remove);
    let mut out = String::new();
    writeln!(&mut out, "Replacements count: {}\nToRemove count: {}", replacements.len(), to_remove.len()).unwrap();
    out
}

/// Helper to create spans
fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> SourceSpan {
    SourceSpan::new(
        Arc::from("test_file.vn"),
        SourceLocation::new(start_line, start_col, 0),
        SourceLocation::new(end_line, end_col, 1),
    )
}

// Unified tests using the helper
#[test]
fn test_no_token_in_map_snapshot() {
    let span = make_span(10, 10, 10, 11);
    let tokens: Vec<Token> = vec![];
    let token_map: HashMap<(usize, usize), usize> = HashMap::new();
    let output = run_hashtag_error(0, span, tokens, token_map);
    assert_snapshot!(output);
}

#[test]
fn test_non_identifier_token_snapshot() {
    let span = make_span(5, 4, 5, 5);
    let token = Token { kind: TokenKind::Numeric(Integer(42)), span: make_span(5, 5, 5, 6) };
    let tokens = vec![token];
    let mut token_map = HashMap::new();
    token_map.insert((5, 5), 0);
    let output = run_hashtag_error(1, span, tokens, token_map);
    assert_snapshot!(output);
}

#[test]
fn test_identifier_length_gt_one_snapshot() {
    let span = make_span(7, 7, 7, 8);
    let token = Token { kind: TokenKind::IdentifierAscii("ab".into()), span: make_span(7, 8, 7, 10) };
    let mut token_map = HashMap::new();
    token_map.insert((7, 8), 0);
    let output = run_hashtag_error(2, span, vec![token], token_map);
    assert_snapshot!(output);
}

#[test]
fn test_get_error_message_none_snapshot() {
    let span = make_span(8, 8, 8, 9);
    let token = Token { kind: TokenKind::IdentifierAscii("z".into()), span: make_span(8, 9, 8, 10) };
    let mut token_map = HashMap::new();
    token_map.insert((8, 9), 0);
    let output = run_hashtag_error(3, span, vec![token], token_map);
    assert_snapshot!(output);
}

#[test]
fn test_adjacent_spans_merging_snapshot() {
    let error_span = make_span(4, 0, 4, 1);
    let token = Token { kind: TokenKind::IdentifierAscii("b".into()), span: make_span(4, 1, 4, 2) };
    let mut token_map = HashMap::new();
    token_map.insert((4, 1), 0);
    let mut merged = String::new();
    let can_merge = error_span.merged(&token.span).is_some();
    writeln!(&mut merged, "CanMerge: {}", can_merge).unwrap();
    let replacements = run_hashtag_error(1, error_span.clone(), vec![token.clone()], token_map.clone());
    writeln!(&mut merged, "{}", replacements).unwrap();
    assert_snapshot!(merged);
}
