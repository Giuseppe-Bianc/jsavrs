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

pub fn lexer_tokenize_whit_errors(
    input: &str,
    file_path_str: &str,
) -> (Vec<Token>, Vec<CompileError>) {
    let mut lexer = Lexer::new(file_path_str, input);
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<CompileError> = Vec::new();

    while let Some(token_result) = lexer.next_token() {
        match token_result {
            Ok(token) => {
                /*let span = &token.span;
                println!("{:?} at {}", token.kind, span);*/
                tokens.push(token);
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }
    (tokens, errors)
}

//src/lexer/test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens;
    use crate::tokens::token_kind::TokenKind::Eof;

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
        use TokenKind::*;
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
        use crate::tokens::number::Number::*;
        let input = "123 45.67 9.01 1e5 2E-3 1.2e3 123. .456 10e5 3.4e+5 5e0 0e0";
        let tokens = lex_kinds(input);
        let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Number(Integer(123)),
                TokenKind::Number(Float64(45.67)),
                TokenKind::Number(Float64(9.01)),
                TokenKind::Number(Scientific64(1.0, 5)),
                TokenKind::Number(Scientific64(2.0, -3)),
                TokenKind::Number(Scientific64(1.2, 3)),
                TokenKind::Number(Float64(123.0)),
                TokenKind::Number(Float64(0.456)),
                TokenKind::Number(Scientific64(10.0, 5)),
                TokenKind::Number(Scientific64(3.4, 5)),
                TokenKind::Number(Scientific64(5.0, 0)),
                TokenKind::Number(Scientific64(0.0, 0)),
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn base_specific_numbers() {
        use TokenKind::*;
        let input = "#b1010 #o777 #x1f #b0 #o0 #x0 #b11111111 #o377 #xdeadBEEF";
        let tokens = lex_kinds(input);
        let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
        assert_eq!(
            tokens,
            vec![
                Binary(tokens::number::Number::Integer(10)),
                Octal(tokens::number::Number::Integer(511)),
                Hexadecimal(tokens::number::Number::Integer(31)),
                Binary(tokens::number::Number::Integer(0)),
                Octal(tokens::number::Number::Integer(0)),
                Hexadecimal(tokens::number::Number::Integer(0)),
                Binary(tokens::number::Number::Integer(255)),
                Octal(tokens::number::Number::Integer(255)),
                Hexadecimal(tokens::number::Number::Integer(0xdeadbeef)),
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
                Binary(tokens::number::Number::UnsignedInteger(10)),
                Octal(tokens::number::Number::UnsignedInteger(511)),
                Hexadecimal(tokens::number::Number::UnsignedInteger(31)),
                Binary(tokens::number::Number::UnsignedInteger(0)),
                Octal(tokens::number::Number::UnsignedInteger(0)),
                Hexadecimal(tokens::number::Number::UnsignedInteger(0)),
                Binary(tokens::number::Number::UnsignedInteger(255)),
                Octal(tokens::number::Number::UnsignedInteger(255)),
                Hexadecimal(tokens::number::Number::UnsignedInteger(0xdeadbeef)),
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
        assert_eq!(
            tokens,
            vec![
                TokenKind::Binary(tokens::number::Number::Integer(expected)),
                Eof
            ]
        );

        // Max i64 value using hex
        let input = "#x7FFFFFFFFFFFFFFF";
        let tokens = lex_kinds(input);
        let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Hexadecimal(tokens::number::Number::Integer(i64::MAX)),
                Eof
            ]
        );

        // Test binary overflow with 64 bits
        let input = "#b1111111111111111111111111111111111111111111111111111111111111111";
        let (tokens, errors) = lexer_tokenize_whit_errors(input, "test");
        assert_eq!(tokens.len(), 1);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
        assert_eq!(
            errors[0].to_string(),
            "Invalid token: \"#b1111111111111111111111111111111111111111111111111111111111111111\" at test:1:1-1:67");
    }

    #[test]
    fn empty_base_numbers() {
        let cases = vec![
            ("#b", TokenKind::IdentifierAscii("b".to_string()), "Invalid token: \"#\" at test:1:1-1:2"),
            ("#o", TokenKind::IdentifierAscii("o".to_string()), "Invalid token: \"#\" at test:1:1-1:2"),
            ("#x", TokenKind::IdentifierAscii("x".to_string()), "Invalid token: \"#\" at test:1:1-1:2"),
        ];

        for (input, token_kind, expected_msg) in cases {
            let (tokens, errors) = lexer_tokenize_whit_errors(input, "test");
            assert_eq!(tokens.len(), 2);
            assert_eq!(errors.len(), 1);
            assert_eq!(tokens[0].kind, token_kind);
            assert_eq!(tokens[1].kind, Eof);
            assert_eq!(
                errors[0].to_string(),
                expected_msg);
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
            let (tokens, errors) = lexer_tokenize_whit_errors(input, "test");
            assert_eq!(tokens.len(), 1);
            assert_eq!(errors.len(), 1);
            assert_eq!(tokens[0].kind, Eof);
            assert_eq!(
                errors[0].to_string(),
                expected);
        }
    }

    #[test]
    fn whitespace_handling() {
        let input = "  \t\n\u{00A0}x"; // Various whitespace chars
        let tokens = lex_kinds(input);
        let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
        assert_eq!(
            tokens,
            vec![TokenKind::IdentifierAscii("x".to_string()), Eof]
        );
    }

    #[test]
    fn mixed_expression() {
        use crate::tokens::number::Number;
        let input = "x = 42 + (y * 3.14)";
        let tokens = lex_kinds(input);
        let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
        assert_eq!(
            tokens,
            vec![
                TokenKind::IdentifierAscii("x".to_string()),
                TokenKind::Equal,
                TokenKind::Number(Number::Integer(42)),
                TokenKind::Plus,
                TokenKind::OpenParen,
                TokenKind::IdentifierAscii("y".to_string()),
                TokenKind::Star,
                TokenKind::Number(Number::Float64(3.14)),
                TokenKind::CloseParen,
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn iterator_collects_all_tokens() {
        use crate::tokens::number::Number;

        let input = "42 + x";
        let lexer = Lexer::new("test", input);
        let tokens: Vec<TokenKind> = lexer
            .map(|res| res.map(|t| t.kind))
            .map(|t| t.unwrap())
            .collect();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Number(Number::Integer(42)),
                TokenKind::Plus,
                TokenKind::IdentifierAscii("x".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    // Add the following tests to src/lexer/test.rs

    #[test]
    fn iterator_empty_input() {
        let tokens = lex_kinds("");
        let tokens: Vec<TokenKind> = tokens.into_iter().map(|t| t.unwrap()).collect();
        assert_eq!(tokens, vec![TokenKind::Eof]);
    }

    #[test]
    fn iterator_single_invalid_token() {
        let (tokens, errors) = lexer_tokenize_whit_errors(&"@", "test");
        assert_eq!(tokens.len(), 1);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
        assert_eq!(
            errors[0].to_string(),
            "Invalid token: \"@\" at test:1:1-1:2"
        );
    }

    #[test]
    fn iterator_multiple_invalid_tokens() {
        let (tokens, errors) = lexer_tokenize_whit_errors(&"@ $", "test");
        assert_eq!(tokens.len(), 1);
        assert_eq!(errors.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
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
        let (tokens, errors) = lexer_tokenize_whit_errors(&"a @ b", "test");
        assert_eq!(tokens.len(), 3);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::IdentifierAscii("a".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::IdentifierAscii("b".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Eof);
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
        use crate::tokens::number::Number;
        let input = "123\n@\n456";
        let (tokens, errors) = lexer_tokenize_whit_errors(input, "test");
        assert_eq!(tokens.len(), 3);
        assert_eq!(errors.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Number(Number::Integer(123)));
        assert_eq!(tokens[1].kind, TokenKind::Number(Number::Integer(456)));
        assert_eq!(tokens[2].kind, TokenKind::Eof);
        assert_eq!(
            errors[0].to_string(),
            "Invalid token: \"@\" at test:2:1-2:2"
        );
    }
}
