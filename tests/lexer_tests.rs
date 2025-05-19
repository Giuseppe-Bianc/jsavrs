use jsavrs::error::compile_error::CompileError;
use jsavrs::lexer::*;
use jsavrs::tokens::number::Number::*;
use jsavrs::tokens::token_kind::TokenKind;
use jsavrs::tokens::token_kind::TokenKind::*;

// Helper function to lex input and return TokenKinds
fn lex_kinds(input: &str) -> Vec<Result<TokenKind, CompileError>> {
    let mut lexer = Lexer::new("test", input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token.map(|t| t.kind));
    }
    tokens
}

#[test]
fn operators() {
    let input = "+ += ++ = - -= -- == != < <= > >= || && << >> %= ^= * / % ^ | & ! : , .";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    let expected = vec![
        Plus,
        PlusEqual,
        PlusPlus,
        Equal,
        Minus,
        MinusEqual,
        MinusMinus,
        EqualEqual,
        NotEqual,
        Less,
        LessEqual,
        Greater,
        GreaterEqual,
        OrOr,
        AndAnd,
        ShiftLeft,
        ShiftRight,
        PercentEqual,
        XorEqual,
        Star,
        Slash,
        Percent,
        Xor,
        Or,
        And,
        Not,
        Colon,
        Comma,
        Dot,
        Eof,
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn decimal_numbers() {
    let input = "123 45.67 9.01 1e5 2E-3 1.2e3 123. .456 10e5 3.4e+5 5e0 0e0";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            Numeric(Integer(123)),
            Numeric(Float64(45.67)),
            Numeric(Float64(9.01)),
            Numeric(Scientific64(1.0, 5)),
            Numeric(Scientific64(2.0, -3)),
            Numeric(Scientific64(1.2, 3)),
            Numeric(Float64(123.0)),
            Numeric(Float64(0.456)),
            Numeric(Scientific64(10.0, 5)),
            Numeric(Scientific64(3.4, 5)),
            Numeric(Scientific64(5.0, 0)),
            Numeric(Scientific64(0.0, 0)),
            Eof
        ]
    );
}

#[test]
fn base_specific_numbers() {
    let input = "#b1010 #o777 #x1f #b0 #o0 #x0 #b11111111 #o377 #xdeadBEEF";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            Binary(Integer(10)),
            Octal(Integer(511)),
            Hexadecimal(Integer(31)),
            Binary(Integer(0)),
            Octal(Integer(0)),
            Hexadecimal(Integer(0)),
            Binary(Integer(255)),
            Octal(Integer(255)),
            Hexadecimal(Integer(0xdeadbeef)),
            Eof
        ]
    );
}

#[test]
fn base_specific_numbers_unsinged() {
    use TokenKind::*;
    let input = "#b1010u #o777u #x1fu #b0u #o0u #x0u #b11111111u #o377u #xdeadBEEFu";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            Binary(UnsignedInteger(10)),
            Octal(UnsignedInteger(511)),
            Hexadecimal(UnsignedInteger(31)),
            Binary(UnsignedInteger(0)),
            Octal(UnsignedInteger(0)),
            Hexadecimal(UnsignedInteger(0)),
            Binary(UnsignedInteger(255)),
            Octal(UnsignedInteger(255)),
            Hexadecimal(UnsignedInteger(0xdeadbeef)),
            Eof
        ]
    );
}

#[test]
fn number_edge_cases() {
    // Max i64 value using binary (63 ones)
    let input = "#b111111111111111111111111111111111111111111111111111111111111111";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    let expected = i64::MAX; // Use i64::MAX constant directly
    assert_eq!(tokens, vec![Binary(Integer(expected)), Eof]);

    // Max i64 value using hex
    let input = "#x7FFFFFFFFFFFFFFF";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(tokens, vec![Hexadecimal(Integer(i64::MAX)), Eof]);

    // Test binary overflow with 64 bits
    let input = "#b1111111111111111111111111111111111111111111111111111111111111111";
    let (tokens, errors) = lexer_tokenize_with_errors(input, "test");
    assert_eq!(tokens.len(), 1);
    assert_eq!(errors.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Eof);
    assert_eq!(
        errors[0].to_string(),
        "Invalid token: \"#b1111111111111111111111111111111111111111111111111111111111111111\" at test:1:1-1:67"
    );
}

#[test]
fn empty_base_numbers() {
    let cases = vec![
        (
            "#b",
            "Malformed binary number: \"#b\" at test:1:1-1:3",
        ),
        (
            "#o",
            "Malformed octal number: \"#o\" at test:1:1-1:3",
        ),
        (
            "#x",
            "Malformed hexadecimal number: \"#x\" at test:1:1-1:3",
        ),
    ];

    for (input, expected_msg) in cases {
        let (tokens, errors) = lexer_tokenize_with_errors(input, "test");
        assert_eq!(tokens.len(), 1);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens[0].kind, Eof);
        assert_eq!(errors[0].to_string(), expected_msg);
    }
}

#[test]
fn identifiers() {
    use TokenKind::*;
    let input = "foo _bar42 変数 ñøπ";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            IdentifierAscii("foo".to_string()),
            IdentifierAscii("_bar42".to_string()),
            IdentifierUnicode("変数".to_string()),
            IdentifierUnicode("ñøπ".to_string()),
            Eof
        ]
    );
}

#[test]
fn keywords() {
    use TokenKind::*;
    let input = "fun if else return while for main var const nullptr break continue true false";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            KeywordFun,
            KeywordIf,
            KeywordElse,
            KeywordReturn,
            KeywordWhile,
            KeywordFor,
            KeywordMain,
            KeywordVar,
            KeywordConst,
            KeywordNullptr,
            KeywordBreak,
            KeywordContinue,
            KeywordBool(true),
            KeywordBool(false),
            Eof
        ]
    );
}

#[test]
fn string_char_literals() {
    use TokenKind::*;
    let input = r#""Hello\n" 'a' "Escape\"Me" '\''"#;
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            StringLiteral("Hello\\n".to_string()),
            CharLiteral("a".to_string()),
            StringLiteral("Escape\\\"Me".to_string()),
            CharLiteral("\\'".to_string()),
            Eof
        ]
    );
}

#[test]
fn brackets() {
    use TokenKind::*;
    let input = "() [] {}";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            OpenParen,
            CloseParen,
            OpenBracket,
            CloseBracket,
            OpenBrace,
            CloseBrace,
            Eof
        ]
    );
}

#[test]
fn types() {
    use TokenKind::*;
    let input = "i8 u16 f32 f64 string bool";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![TypeI8, TypeU16, TypeF32, TypeF64, TypeString, TypeBool, Eof]
    );
}

#[test]
fn invalid_tokens() {
    let cases = vec![
        ("@", "Invalid token: \"@\" at test:1:1-1:2"),
        ("`", "Invalid token: \"`\" at test:1:1-1:2"),
        ("~", "Invalid token: \"~\" at test:1:1-1:2"),
    ];

    for (input, expected) in cases {
        let (tokens, errors) = lexer_tokenize_with_errors(input, "test");
        assert_eq!(tokens.len(), 1);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens[0].kind, Eof);
        assert_eq!(errors[0].to_string(), expected);
    }
}

#[test]
fn whitespace_handling() {
    let input = "  \t\n\u{00A0}x"; // Various whitespace chars
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(tokens, vec![IdentifierAscii("x".to_string()), Eof]);
}

#[test]
fn mixed_expression() {
    let input = "x = 42 + (y * 3.14)";
    let tokens = lex_kinds(input);
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(
        tokens,
        vec![
            IdentifierAscii("x".to_string()),
            Equal,
            Numeric(Integer(42)),
            Plus,
            OpenParen,
            IdentifierAscii("y".to_string()),
            Star,
            Numeric(Float64(3.14)),
            CloseParen,
            Eof
        ]
    );
}

#[test]
fn iterator_collects_all_tokens() {
    let input = "42 + x";
    let lexer = Lexer::new("test", input);
    let tokens: Vec<TokenKind> = lexer
        .map(|res| res.map(|t| t.kind))
        .map(|t| t.unwrap())
        .collect();
    assert_eq!(
        tokens,
        vec![
            Numeric(Integer(42)),
            Plus,
            IdentifierAscii("x".to_string()),
            Eof,
        ]
    );
}

// Add the following tests to src/lexer/test.rs

#[test]
fn iterator_empty_input() {
    let tokens = lex_kinds("");
    let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
    assert_eq!(tokens, vec![Eof]);
}

#[test]
fn iterator_single_invalid_token() {
    let (tokens, errors) = lexer_tokenize_with_errors(&"@", "test");
    assert_eq!(tokens.len(), 1);
    assert_eq!(errors.len(), 1);
    assert_eq!(tokens[0].kind, Eof);
    assert_eq!(
        errors[0].to_string(),
        "Invalid token: \"@\" at test:1:1-1:2"
    );
}

#[test]
fn iterator_multiple_invalid_tokens() {
    let (tokens, errors) = lexer_tokenize_with_errors(&"@ $", "test");
    assert_eq!(tokens.len(), 1);
    assert_eq!(errors.len(), 2);
    assert_eq!(tokens[0].kind, Eof);
    assert_eq!(
        errors[0].to_string(),
        "Invalid token: \"@\" at test:1:1-1:2"
    );
    assert_eq!(
        errors[1].to_string(),
        "Invalid token: \"$\" at test:1:3-1:4"
    );
}

#[test]
fn iterator_mixed_valid_invalid_valid() {
    let (tokens, errors) = lexer_tokenize_with_errors(&"a @ b", "test");
    assert_eq!(tokens.len(), 3);
    assert_eq!(errors.len(), 1);
    assert_eq!(tokens[0].kind, IdentifierAscii("a".to_string()));
    assert_eq!(tokens[1].kind, IdentifierAscii("b".to_string()));
    assert_eq!(tokens[2].kind, Eof);
    assert_eq!(
        errors[0].to_string(),
        "Invalid token: \"@\" at test:1:3-1:4"
    );
}

#[test]
fn iterator_eof_emitted_once() {
    let mut lexer = Lexer::new("test", "a");
    assert!(lexer.next().is_some()); // Identifier
    assert!(lexer.next().is_some()); // Eof
    assert!(lexer.next().is_none());
    assert!(lexer.next().is_none());
}

#[test]
fn iterator_multiline_span_tracking() {
    let input = "123\n@\n456";
    let (tokens, errors) = lexer_tokenize_with_errors(input, "test");
    assert_eq!(tokens.len(), 3);
    assert_eq!(errors.len(), 1);
    assert_eq!(tokens[0].kind, Numeric(Integer(123)));
    assert_eq!(tokens[1].kind, Numeric(Integer(456)));
    assert_eq!(tokens[2].kind, Eof);
    assert_eq!(
        errors[0].to_string(),
        "Invalid token: \"@\" at test:2:1-2:2"
    );
}


#[test]
fn test_malformed_binary_error() {
    let input = "b";
    let expected = Some("Malformed binary number: \"#b\"");
    let result = get_error_message(input);
    assert_eq!(result, expected);
}

#[test]
fn test_malformed_octal_error() {
    let input = "o";
    let expected = Some("Malformed octal number: \"#o\"");
    let result = get_error_message(input);
    assert_eq!(result, expected);
}

#[test]
fn test_malformed_hexadecimal_error() {
    let input = "x";
    let expected = Some("Malformed hexadecimal number: \"#x\"");
    let result = get_error_message(input);
    assert_eq!(result, expected);
}

#[test]
fn test_unrecognized_prefix() {
    let input = "z";
    let result = get_error_message(input);
    assert!(result.is_none());
}

#[test]
fn test_empty_string() {
    let input = "";
    let result = get_error_message(input);
    assert!(result.is_none());
}

#[test]
fn test_uppercase_input() {
    let input = "B";
    let result = get_error_message(input);
    assert!(result.is_none());
}
