use jsavrs::location::source_span::SourceSpan;
use jsavrs::parser::ast::{BinaryOp, Expr, LiteralValue, UnaryOp};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::tokens::number::Number;
use jsavrs::tokens::token::Token;
use jsavrs::tokens::token_kind::TokenKind;

// Helper to create tokens with dummy spans
fn dummy_span() -> SourceSpan {
    SourceSpan::default()
}

fn create_tokens(kinds: Vec<TokenKind>) -> Vec<Token> {
    kinds.into_iter().map(|k| Token { kind: k, span: dummy_span() }).collect()
}


#[test]
fn test_literal_number() {
    let tokens = create_tokens(vec![TokenKind::Numeric(Number::Integer(42)), TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(42)),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_literal_bool() {
    let tokens = create_tokens(vec![TokenKind::KeywordBool(true), TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_literal_nullptr() {
    let tokens = create_tokens(vec![TokenKind::KeywordNullptr, TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Literal {
            value: LiteralValue::Nullptr,
            span: dummy_span(),
        })
    );
}

#[test]
fn test_literal_string() {
    let tokens = create_tokens(vec![TokenKind::StringLiteral("assssss".to_string()), TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Literal {
            value: LiteralValue::StringLit("assssss".to_string()),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_literal_char() {
    let tokens = create_tokens(vec![TokenKind::CharLiteral("a".to_string()), TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Literal {
            value: LiteralValue::CharLit("a".to_string()),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_unary_negation() {
    let tokens = create_tokens(vec![
        TokenKind::Minus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Unary {
            op: UnaryOp::Negate,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(5)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_unary_not() {
    let tokens = create_tokens(vec![
        TokenKind::Not,
        TokenKind::KeywordBool(true),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Unary {
            op: UnaryOp::Not,
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Bool(true),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_binary_precedence() {
    let tokens = create_tokens(vec![
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(2)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(3)),
                span: dummy_span(),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(5)),
                    span: dummy_span(),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(2)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_grouping() {
    let tokens = create_tokens(vec![
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::CloseParen,
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(2)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Binary {
            left: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(3)),
                        span: dummy_span(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(5)),
                        span: dummy_span(),
                    }),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(2)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_assignment_valid() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Assign {
            name: "x".into(),
            value: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(5)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_assignment_chained() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Equal,
        TokenKind::IdentifierAscii("y".into()),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Assign {
            name: "x".into(),
            value: Box::new(Expr::Assign {
                name: "y".into(),
                value: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(5)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_assignment_invalid_target() {
    let tokens = create_tokens(vec![
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(10)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Invalid assignment target");
    assert_eq!(
        expr,
        Some(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(5)),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_function_call() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("foo".into()),
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::Comma,
        TokenKind::Numeric(Number::Integer(2)),
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Call {
            callee: Box::new(Expr::Variable {
                name: "foo".into(),
                span: dummy_span(),
            }),
            arguments: vec![
                Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(1)),
                    span: dummy_span(),
                },
                Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(2)),
                        span: dummy_span(),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(3)),
                        span: dummy_span(),
                    }),
                    span: dummy_span(),
                },
            ],
            span: dummy_span(),
        })
    );
}

#[test]
fn test_array_access() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("arr".into()),
        TokenKind::OpenBracket,
        TokenKind::Numeric(Number::Integer(0)),
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::CloseBracket,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::ArrayAccess {
            array: Box::new(Expr::Variable {
                name: "arr".into(),
                span: dummy_span(),
            }),
            index: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(0)),
                    span: dummy_span(),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(1)),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_array_access_empty_index() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("arr".into()),
        TokenKind::OpenBracket,
        TokenKind::CloseBracket,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::ArrayAccess {
            array: Box::new(Expr::Variable {
                name: "arr".into(),
                span: dummy_span(),
            }),
            index: Box::new(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: CloseBracket");
}


#[test]
fn test_unclosed_parenthesis() {
    let tokens = create_tokens(vec![
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unclosed parenthesis");
    assert_eq!(
        expr,
        Some(Expr::Grouping {
            expr: Box::new(Expr::Literal {
                value: LiteralValue::Number(Number::Integer(5)),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}

#[test]
fn test_unexpected_token() {
    let tokens = create_tokens(vec![
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: Plus");
    assert_eq!(expr, None);
}

#[test]
fn test_empty_input() {
    let tokens = create_tokens(vec![TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr, None);
}

#[test]
fn test_deep_nesting() {
    let tokens = create_tokens(vec![
        TokenKind::OpenParen,
        TokenKind::OpenParen,
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::CloseParen,
        TokenKind::CloseParen,
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(
        expr,
        Some(Expr::Grouping {
            expr: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Grouping {
                    expr: Box::new(Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(5)),
                        span: dummy_span(),
                    }),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            span: dummy_span(),
        })
    );
}