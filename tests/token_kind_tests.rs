use std::sync::Arc;
use jsavrs::tokens::number::Number;
use jsavrs::tokens::number::Number::*;
use jsavrs::tokens::token_kind::TokenKind::*;
use jsavrs::tokens::token_kind::{handle_suffix, split_numeric_and_suffix, TokenKind};
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
    assert!(handle_suffix("123.45", Some("u".into())).is_none());
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
    let mut lex2 = TokenKind::lexer("1.2e3.4"); // Invalid exponent
    assert!(!lex2.next().unwrap().is_err());
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
    assert!(!IdentifierAscii("abc".into()).is_type());
    assert!(!Numeric(Integer(42)).is_type());
    assert!(!StringLiteral("test".into()).is_type());
    assert!(!CharLiteral("a".into()).is_type());
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

#[test]
fn test_eof_and_ignored() {
    assert_eq!(Eof.to_string(), "end of file");
    assert_eq!(Comment.to_string(), "comment");
    assert_eq!(Whitespace.to_string(), "whitespace");
}

#[test]
fn test_identifier_ascii_normal() {
    let ident : Arc<str> = "foo".into();
    assert_eq!(
        IdentifierAscii(ident.clone()).to_string(),
        format!("identifier '{ident}'")
    );
}

#[test]
fn test_identifier_ascii_empty() {
    let ident : Arc<str> = "".into();
    assert_eq!(IdentifierAscii(ident.clone()).to_string(), "identifier ''");
}

#[test]
fn test_identifier_unicode() {
    let ident : Arc<str> = "προεδομή".into(); // qualche stringa Unicode
    assert_eq!(
        IdentifierUnicode(ident.clone()).to_string(),
        format!("identifier '{ident}'")
    );
}

#[test]
fn test_numeric_integer() {
    let num = Number::Integer(123);
    assert_eq!(Numeric(num.clone()).to_string(), "number '123'");
    assert_eq!(Hexadecimal(num.clone()).to_string(), "hexadecimal '123'");
    assert_eq!(Octal(num.clone()).to_string(), "octal '123'");
    assert_eq!(Binary(num).to_string(), "binary '123'");
}

#[test]
fn test_string_literal_simple() {
    let s: Arc<str> = "hello".into();
    assert_eq!(
        StringLiteral(s.clone()).to_string(),
        format!("string literal \"{s}\"")
    );
}

#[test]
fn test_string_literal_with_quotes_inside() {
    let s: Arc<str> = "he said \"ciao\"".into();
    assert_eq!(
        StringLiteral(s.clone()).to_string(),
        format!("string literal \"{s}\"")
    );
}

#[test]
fn test_char_literal_simple() {
    let c: Arc<str>= "x".into();
    assert_eq!(
        CharLiteral(c.clone()).to_string(),
        format!("character literal '{c}'")
    );
}

#[test]
fn test_char_literal_unicode() {
    let c: Arc<str> = "ψ".into();
    assert_eq!(
        CharLiteral(c.clone()).to_string(),
        format!("character literal '{c}'")
    );
}

#[test]
fn test_keyword_bool_true_false() {
    assert_eq!(KeywordBool(true).to_string(), "boolean 'true'");
    assert_eq!(&KeywordBool(false).to_string(), "boolean 'false'");
}

#[test]
fn test_keyword_nullptr() {
    assert_eq!(&KeywordNullptr.to_string(), "'nullptr'");
}

// ——— Test per tutte le keyword principali ———
#[test]
fn test_all_keywords() {
    let mapping = vec![
        (KeywordFun, "'fun'"),
        (KeywordIf, "'if'"),
        (KeywordElse, "'else'"),
        (KeywordVar, "'var'"),
        (KeywordConst, "'const'"),
        (KeywordReturn, "'return'"),
        (KeywordWhile, "'while'"),
        (KeywordFor, "'for'"),
        (KeywordBreak, "'break'"),
        (KeywordContinue, "'continue'"),
        (KeywordMain, "'main'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}

// ——— Test per tutti i tipi primari ———
#[test]
fn test_all_primitive_types() {
    let mapping = vec![
        (TypeI8, "'i8'"),
        (TypeI16, "'i16'"),
        (TypeI32, "'i32'"),
        (TypeI64, "'i64'"),
        (TypeU8, "'u8'"),
        (TypeU16, "'u16'"),
        (TypeU32, "'u32'"),
        (TypeU64, "'u64'"),
        (TypeF32, "'f32'"),
        (TypeF64, "'f64'"),
        (TypeChar, "'char'"),
        (TypeString, "'string'"),
        (TypeBool, "'bool'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}

// ——— Test per punteggiatura e simboli singoli ———
#[test]
fn test_punctuation() {
    let mapping = vec![
        (OpenParen, "'('"),
        (CloseParen, "')'"),
        (OpenBrace, "'{'"),
        (CloseBrace, "'}'"),
        (OpenBracket, "'['"),
        (CloseBracket, "']'"),
        (Semicolon, "';'"),
        (Colon, "':'"),
        (Comma, "','"),
        (Dot, "'.'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}

// ——— Test per operatori semplici e composti ———
#[test]
fn test_operators_single_and_multi_char() {
    let mapping = vec![
        (Plus, "'+'"),
        (PlusPlus, "'++'"),
        (MinusMinus, "'--'"),
        (PlusEqual, "'+='"),
        (MinusEqual, "'-='"),
        (Minus, "'-'"),
        (Star, "'*'"),
        (Slash, "'/'"),
        (Percent, "'%'"),
        (PercentEqual, "'%='"),
        (Equal, "'='"),
        (EqualEqual, "'=='"),
        (NotEqual, "'!='"),
        (Less, "'<'"),
        (LessEqual, "'<='"),
        (Greater, "'>'"),
        (GreaterEqual, "'>='"),
        (AndAnd, "'&&'"),
        (OrOr, "'||'"),
        (Not, "'!'"),
        (And, "'&'"),
        (Or, "'|'"),
        (Xor, "'^'"),
        (XorEqual, "'^='"),
        (ShiftLeft, "'<<'"),
        (ShiftRight, "'>>'"),
    ];

    for (kind, expected) in mapping {
        assert_eq!(kind.to_string(), expected);
    }
}
