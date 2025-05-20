use jsavrs::tokens::number::Number::*;
use jsavrs::tokens::token_kind::TokenKind::*;
use jsavrs::tokens::token_kind::{
    TokenKind, handle_suffix, handle_unsigned_suffix, split_numeric_and_suffix,
};
use logos::Logos;

// Helper function to assert token matching
fn assert_token(input: &str, expected: TokenKind) {
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(expected));
}

#[test]
fn test_parse_integer() {
    assert_token("123", Numeric(Integer(123)));
}

#[test]
fn test_empty_slice_for_split_numeric_and_suffix() {
    let (num_part, suffix) = split_numeric_and_suffix("");
    assert_eq!(num_part, "");
    assert_eq!(suffix, None);
}

#[test]
fn test_unknown_suffix_for_handle_suffix() {
    // Fixed typo in test name
    assert!(handle_suffix("123", Some("x".into())).is_none());
}

#[test]
fn test_malformed_numeric_part_handle_unsigned_suffix() {
    // Fixed typo in test name
    assert!(handle_unsigned_suffix("123.45").is_none());
}

#[test]
fn test_parse_unsigned_integer() {
    assert_token("123u", Numeric(UnsignedInteger(123)));
}

#[test]
fn test_parse_float_suffix() {
    assert_token("45.67f", Numeric(Float32(45.67)));
}

#[test]
fn test_parse_double_suffix() {
    assert_token("89.01d", Numeric(Float64(89.01)));
}

#[test]
fn test_parse_scientific_float() {
    assert_token("1.2e3f", Numeric(Scientific32(1.2, 3)));
}

#[test]
fn test_parse_scientific_double() {
    assert_token("3.4e5", Numeric(Scientific64(3.4, 5)));
}

#[test]
fn test_number_with_invalid_suffix() {
    let mut lex = TokenKind::lexer("123x");
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Integer(123))));
    assert_eq!(lex.next().unwrap(), Ok(IdentifierAscii("x".into())));
}

#[test]
fn test_decimal_starting_with_dot() {
    assert_token(".456", Numeric(Float64(0.456)));
}

#[test]
fn test_invalid_scientific_notation() {
    let mut lex = TokenKind::lexer("1e2e3");
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Scientific64(1.0, 2))));
    assert_eq!(lex.next().unwrap(), Ok(IdentifierAscii("e3".into())));
}

#[test]
fn returns_true_for_all_type_tokens() {
    assert!(TypeI8.is_type());
    assert!(TypeI16.is_type());
    assert!(TypeI32.is_type());
    assert!(TypeI64.is_type());
    assert!(TypeU8.is_type());
    assert!(TypeU16.is_type());
    assert!(TypeU32.is_type());
    assert!(TypeU64.is_type());
    assert!(TypeF32.is_type());
    assert!(TypeF64.is_type());
    assert!(TypeChar.is_type());
    assert!(TypeString.is_type());
    assert!(TypeBool.is_type());
}

#[test]
fn returns_false_for_non_type_tokens() {
    assert!(!Plus.is_type());
    assert!(!Minus.is_type());
    assert!(!KeywordFun.is_type());
    assert!(!KeywordIf.is_type());
    assert!(!IdentifierAscii("abc".to_string()).is_type());
    assert!(!Numeric(Integer(42)).is_type());
    assert!(!StringLiteral("test".to_string()).is_type());
    assert!(!CharLiteral("a".to_string()).is_type());
    assert!(!Whitespace.is_type());
    assert!(!Eof.is_type());
}

#[test]
fn returns_false_for_edge_case_similar_names() {
    // Assicura che token con nomi simili ma non tipi non siano considerati tipi
    assert!(!KeywordVar.is_type());
    assert!(!KeywordConst.is_type());
    assert!(!KeywordBool(true).is_type());
    assert!(!KeywordBool(false).is_type());
}

#[test]
fn returns_false_for_structurally_similar_tokens() {
    // Token che hanno struttura simile ma non sono tipi
    assert!(!OpenParen.is_type());
    assert!(!CloseParen.is_type());
    assert!(!OpenBracket.is_type());
    assert!(!CloseBracket.is_type());
    assert!(!OpenBrace.is_type());
    assert!(!CloseBrace.is_type());
}