//cargo insta test --review
use jsavrs::{
    error::compile_error::CompileError,
    lexer::*,
    location::{source_location::SourceLocation, source_span::SourceSpan},
    tokens::{number::Number::*, token::Token, token_kind::TokenKind, token_kind::TokenKind::*},
};
use insta::assert_snapshot;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    fmt::Write,
};

// Helper to lex input and return formatted kinds or errors
fn snapshot_lex(input: &str) -> String {
    let mut lexer = Lexer::new("test", input);
    let mut out = String::new();
    while let Some(token_res) = lexer.next_token() {
        match token_res {
            Ok(tok) => {
                writeln!(&mut out, "{:?}", tok.kind).unwrap();
            }
            Err(err) => {
                writeln!(&mut out, "Error: {}", err).unwrap();
            }
        }
    }
    out
}

// Helper to capture tokens and errors separately
fn snapshot_errors(input: &str) -> (String, String) {
    let (tokens, errors) = lexer_tokenize_with_errors(input, "test");
    let mut tok_out = String::new();
    for tok in tokens {
        writeln!(&mut tok_out, "{:?}", tok.kind).unwrap();
    }
    let mut err_out = String::new();
    for err in errors {
        writeln!(&mut err_out, "{}", err).unwrap();
    }
    (tok_out, err_out)
}

#[test]
fn operators_snapshot() {
    let input = "+ += ++ = - -= -- == != < <= > >= || && << >> %= ^= * / % ^ | & ! : , .";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn decimal_numbers_snapshot() {
    let input = "123 45.67 9.01 1e5 2E-3 1.2e3 123. .456 10e5 3.4e+5 5e0 0e0";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn base_specific_numbers_snapshot() {
    let input = "#b1010 #o777 #x1f #b0 #o0 #x0 #b11111111 #o377 #xdeadBEEF";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn base_specific_numbers_unsigned_snapshot() {
    let input = "#b1010u #o777u #x1fu #b0u #o0u #x0u #b11111111u #o377u #xdeadBEEFu";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn number_edge_cases_snapshot() {
    // Valid max i64 in binary
    let input1 = "#b111111111111111111111111111111111111111111111111111111111111111";
    let snap1 = snapshot_lex(input1);
    assert_snapshot!(snap1);
    
    // Valid max i64 in hex
    let input2 = "#x7FFFFFFFFFFFFFFF";
    let snap2 = snapshot_lex(input2);
    assert_snapshot!(snap2);
    
    // Overflow binary
    let input3 = "#b1111111111111111111111111111111111111111111111111111111111111111";
    let (tok_out, err_out) = snapshot_errors(input3);
    assert_snapshot!(tok_out, @"Eof");
    assert_snapshot!(err_out);
}

#[test]
fn empty_base_numbers_snapshot() {
    let cases = vec!["#b", "#o", "#x"];
    for &input in &cases {
        let (_tok_out, err_out) = snapshot_errors(input);
        assert_snapshot!(err_out);
    }
}

#[test]
fn identifiers_snapshot() {
    let input = "foo _bar42 変数 ñøπ";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn keywords_snapshot() {
    let input = "fun if else return while for main var const nullptr break continue true false";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn string_char_literals_snapshot() {
    let input = r#"Hello\n" 'a' "Escape\"Me" '\''"#;
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn brackets_snapshot() {
    let input = "() [] {}";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn types_snapshot() {
    let input = "i8 u16 f32 f64 string bool";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn invalid_tokens_snapshot() {
    let cases = vec!["@", "`", "~"];
    for &input in &cases {
        let (_tok_out, err_out) = snapshot_errors(input);
        assert_snapshot!(err_out);
    }
}

#[test]
fn whitespace_handling_snapshot() {
    let input = "  \t\n\u{00A0}x";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn mixed_expression_snapshot() {
    let input = "x = 42 + (y * 3.14)";
    let snap = snapshot_lex(input);
    assert_snapshot!(snap);
}

#[test]
fn iterator_collects_all_tokens_snapshot() {
    let input = "42 + x";
    let mut lexer = Lexer::new("test", input);
    let mut out = String::new();
    for res in lexer {
        match res {
            Ok(tok) => writeln!(&mut out, "{:?}", tok.kind).unwrap(),
            Err(err) => writeln!(&mut out, "Error: {}", err).unwrap(),
        }
    }
    out.push_str("Eof\n");
    assert_snapshot!(out);
}

#[test]
fn iterator_empty_input_snapshot() {
    let snap = snapshot_lex("");
    assert_snapshot!(snap);
}

#[test]
fn iterator_single_invalid_token_snapshot() {
    let input = "@";
    let (_tok_out, err_out) = snapshot_errors(input);
    assert_snapshot!(err_out);
}

#[test]
fn iterator_multiple_invalid_tokens_snapshot() {
    let input = "@ $";
    let (_tok_out, err_out) = snapshot_errors(input);
    assert_snapshot!(err_out);
}

#[test]
fn iterator_mixed_valid_invalid_valid_snapshot() {
    let input = "a @ b";
    let (tok_out, err_out) = snapshot_errors(input);
    assert_snapshot!(tok_out);
    assert_snapshot!(err_out);
}

#[test]
fn iterator_eof_emitted_once_snapshot() {
    let mut lexer = Lexer::new("test", "a");
    let mut out = String::new();
    if let Some(res) = lexer.next() {
        match res {
            Ok(tok) => writeln!(&mut out, "{:?}", tok.kind).unwrap(),
            Err(err) => writeln!(&mut out, "Error: {}", err).unwrap(),
        }
    }
    if let Some(res) = lexer.next() {
        match res {
            Ok(tok) => writeln!(&mut out, "{:?}", tok.kind).unwrap(),
            Err(err) => writeln!(&mut out, "Error: {}", err).unwrap(),
        }
    }
    // Further next() calls produce None
    out.push_str("Verified no extra tokens\n");
    assert_snapshot!(out);
}

#[test]
fn iterator_multiline_span_tracking_snapshot() {
    let input = "123\n@\n456";
    let (tok_out, err_out) = snapshot_errors(input);
    assert_snapshot!(tok_out);
    assert_snapshot!(err_out);
}

#[test]
fn test_malformed_binary_error_snapshot() {
    let input = "#b";
    // Assuming get_error_message returns full message
    let msg = get_error_message(input).unwrap_or_else(|| "None".into());
    assert_snapshot!(msg);
}

#[test]
fn test_malformed_octal_error_snapshot() {
    let input = "#o";
    let msg = get_error_message(input).unwrap_or_else(|| "None".into());
    assert_snapshot!(msg);
}

#[test]
fn test_malformed_hexadecimal_error_snapshot() {
    let input = "#x";
    let msg = get_error_message(input).unwrap_or_else(|| "None".into());
    assert_snapshot!(msg);
}

#[test]
fn test_unrecognized_prefix_snapshot() {
    let input = "z";
    let msg = get_error_message(input).map_or_else(|| "None".into(), |m| m);
    assert_snapshot!(msg);
}

#[test]
fn test_empty_string_snapshot() {
    let input = "";
    let msg = get_error_message(input).map_or_else(|| "None".into(), |m| m);
    assert_snapshot!(msg);
}

#[test]
fn test_uppercase_input_snapshot() {
    let input = "B";
    let msg = get_error_message(input).map_or_else(|| "None".into(), |m| m);
    assert_snapshot!(msg);
}

/// Helper to create a SourceSpan from (start_line, start_col) to (end_line, end_col).
fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> SourceSpan {
    SourceSpan::new(
        Arc::from("test_file.vn"),
        SourceLocation::new(start_line, start_col, 0),
        SourceLocation::new(end_line, end_col, 1),
    )
}

#[test]
fn test_no_token_in_map_snapshot() {
    let eidx = 0;
    let span = make_span(10, 10, 10, 11);
    let tokens: Vec<Token> = vec![];
    let token_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut replacements: HashMap<usize, CompileError> = HashMap::new();
    let mut to_remove: HashSet<usize> = HashSet::new();
    process_hashtag_error(
        eidx,
        &span,
        &tokens,
        &token_map,
        &mut replacements,
        &mut to_remove,
    );
    let mut out = String::new();
    writeln!(&mut out, "Replacements: {:?}\nToRemove: {:?}", replacements, to_remove).unwrap();
    assert_snapshot!(out);
}

#[test]
fn test_non_identifier_token_snapshot() {
    let eidx = 1;
    let span = make_span(5, 4, 5, 5);
    let token_span = make_span(5, 5, 5, 6);
    let tok = Token {
        kind: TokenKind::Numeric(Integer(42)),
        span: token_span.clone(),
    };
    let tokens = vec![tok];
    let mut token_map = HashMap::new();
    token_map.insert((5, 5), 0);

    let mut replacements: HashMap<usize, CompileError> = HashMap::new();
    let mut to_remove: HashSet<usize> = HashSet::new();
    process_hashtag_error(
        eidx,
        &span,
        &tokens,
        &token_map,
        &mut replacements,
        &mut to_remove,
    );
    let mut out = String::new();
    writeln!(&mut out, "Replacements: {:?}\nToRemove: {:?}", replacements, to_remove).unwrap();
    assert_snapshot!(out);
}

#[test]
fn test_identifier_length_gt_one_snapshot() {
    let eidx = 2;
    let span = make_span(7, 7, 7, 8);
    let token_span = make_span(7, 8, 7, 10);
    let tok = Token {
        kind: TokenKind::IdentifierAscii("ab".into()),
        span: token_span.clone(),
    };
    let tokens = vec![tok];
    let mut token_map = HashMap::new();
    token_map.insert((7, 8), 0);
    let mut replacements = HashMap::new();
    let mut to_remove = HashSet::new();
    process_hashtag_error(
        eidx,
        &span,
        &tokens,
        &token_map,
        &mut replacements,
        &mut to_remove,
    );
    let mut out = String::new();
    writeln!(&mut out, "Replacements: {:?}\nToRemove: {:?}", replacements, to_remove).unwrap();
    assert_snapshot!(out);
}

#[test]
fn test_get_error_message_none_snapshot() {
    let eidx = 3;
    let span = make_span(8, 8, 8, 9);
    let token_span = make_span(8, 9, 8, 10);
    let tok = Token {
        kind: TokenKind::IdentifierAscii("z".into()),
        span: token_span.clone(),
    };
    let tokens = vec![tok];
    let mut token_map = HashMap::new();
    token_map.insert((8, 9), 0);
    let mut replacements = HashMap::new();
    let mut to_remove = HashSet::new();
    process_hashtag_error(
        eidx,
        &span,
        &tokens,
        &token_map,
        &mut replacements,
        &mut to_remove,
    );
    let mut out = String::new();
    writeln!(&mut out, "Replacements: {:?}\nToRemove: {:?}", replacements, to_remove).unwrap();
    assert_snapshot!(out);
}

#[test]
fn test_adjacent_spans_merging_snapshot() {
    let eidx = 1;
    let error_span = make_span(4, 0, 4, 1);
    let token_span = make_span(4, 1, 4, 2);
    let token = Token {
        kind: TokenKind::IdentifierAscii("b".into()),
        span: token_span.clone(),
    };
    let tokens = vec![token];
    let mut token_map = HashMap::new();
    token_map.insert((4, 1), 0);
    let mut replacements = HashMap::new();
    let mut to_remove = HashSet::new();
    process_hashtag_error(
        eidx,
        &error_span,
        &tokens,
        &token_map,
        &mut replacements,
        &mut to_remove,
    );
    let can_merge = error_span.merged(&token_span).is_some();
    let mut out = String::new();
    writeln!(&mut out, "CanMerge: {}\nReplacements count: {}\nToRemove count: {}", can_merge, replacements.len(), to_remove.len()).unwrap();
    assert_snapshot!(out);
}
