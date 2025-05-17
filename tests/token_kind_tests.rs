use jsavrs::tokens::number::Number::*;
use jsavrs::tokens::token_kind::TokenKind::*;
use jsavrs::tokens::token_kind::{
    TokenKind, handle_suffix, handle_unsigned_suffix, split_numeric_and_suffix,
};
use logos::Logos;

#[test]
fn test_parse_integer() {
    let input = "123";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Integer(123))));
}

#[test]
fn test_empty_slice_for_split_numeric_and_suffix() {
    let input = "";
    let (num_part, suffix) = split_numeric_and_suffix(input);
    assert_eq!(num_part, "");
    assert_eq!(suffix, None);
}

#[test]
fn tets_unknown_suffix_for_handle_suffix() {
    let numeric_part = "123";
    let suffix = Some("x".to_string());
    let result = handle_suffix(numeric_part, suffix);
    assert_eq!(result, None);
}

#[test]
fn tets_malformed_numeri_part_handle_unsigned_suffix() {
    let numeric_part = "123.45";
    let result = handle_unsigned_suffix(numeric_part);
    assert_eq!(result, None);
}

#[test]
fn test_parse_unsigned_integer() {
    let input = "123u";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(UnsignedInteger(123))));
}

#[test]
fn test_parse_float_suffix() {
    let input = "45.67f";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Float32(45.67))));
}

#[test]
fn test_parse_double_suffix() {
    let input = "89.01d";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Float64(89.01))));
}

#[test]
fn test_parse_scientific_float() {
    let input = "1.2e3f";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Scientific32(1.2, 3))));
}

#[test]
fn test_parse_scientific_double() {
    let input = "3.4e5";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Scientific64(3.4, 5))));
}

#[test]
fn test_number_with_invalid_suffix() {
    let input = "123x";
    let mut lex = TokenKind::lexer(input);
    // 'x' is not a valid suffix, should parse as number then identifier
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Integer(123))));
    assert_eq!(lex.next().unwrap(), Ok(IdentifierAscii("x".to_string())));
}

#[test]
fn test_decimal_starting_with_dot() {
    let input = ".456";
    let mut lex = TokenKind::lexer(input);
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Float64(0.456))));
}

#[test]
fn test_invalid_scientific_notation() {
    let input = "1e2e3"; // Invalid multiple exponents
    let mut lex = TokenKind::lexer(input);
    // Should parse first '1e2' then 'e3' as separate tokens
    assert_eq!(lex.next().unwrap(), Ok(Numeric(Scientific64(1.0, 2))));
    assert_eq!(lex.next().unwrap(), Ok(IdentifierAscii("e3".to_string())));
}
