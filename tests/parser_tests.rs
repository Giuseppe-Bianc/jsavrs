use jsavrs::error::compile_error::CompileError;
use jsavrs::location::source_span::SourceSpan;
use jsavrs::parser::ast::*;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::parser::precedence::unary_binding_power;
use jsavrs::tokens::number::Number;
use jsavrs::tokens::token::Token;
use jsavrs::tokens::token_kind::TokenKind;

// Helper to create tokens with dummy spans
fn dummy_span() -> SourceSpan {
    SourceSpan::default()
}

fn create_tokens(kinds: Vec<TokenKind>) -> Vec<Token> {
    kinds
        .into_iter()
        .map(|k| Token {
            kind: k,
            span: dummy_span(),
        })
        .collect()
}

fn num_token(n: f64) -> Token {
    Token {
        kind: TokenKind::Numeric(Number::Float64(n)),
        span: dummy_span(),
    }
}

// Helper functions per costruire AST
fn num_lit(n: i64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Integer(n)),
        span: dummy_span(),
    }
}

fn float_lit(n: f64) -> Expr {
    Expr::Literal {
        value: LiteralValue::Number(Number::Float64(n)),
        span: dummy_span(),
    }
}

fn bool_lit(b: bool) -> Expr {
    Expr::Literal {
        value: LiteralValue::Bool(b),
        span: dummy_span(),
    }
}

fn nullptr_lit() -> Expr {
    Expr::Literal {
        value: LiteralValue::Nullptr,
        span: dummy_span(),
    }
}

fn string_lit(s: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::StringLit(s.to_string()),
        span: dummy_span(),
    }
}

fn char_lit(c: &str) -> Expr {
    Expr::Literal {
        value: LiteralValue::CharLit(c.to_string()),
        span: dummy_span(),
    }
}

fn binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
    Expr::Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
        span: dummy_span(),
    }
}

fn unary_expr(op: UnaryOp, expr: Expr) -> Expr {
    Expr::Unary {
        op,
        expr: Box::new(expr),
        span: dummy_span(),
    }
}

fn grouping_expr(expr: Expr) -> Expr {
    Expr::Grouping {
        expr: Box::new(expr),
        span: dummy_span(),
    }
}

fn assign_expr(name: &str, value: Expr) -> Expr {
    Expr::Assign {
        name: name.into(),
        value: Box::new(value),
        span: dummy_span(),
    }
}

fn variable_expr(name: &str) -> Expr {
    Expr::Variable {
        name: name.into(),
        span: dummy_span(),
    }
}

fn call_expr(callee: Expr, arguments: Vec<Expr>) -> Expr {
    Expr::Call {
        callee: Box::new(callee),
        arguments,
        span: dummy_span(),
    }
}

fn array_access_expr(array: Expr, index: Expr) -> Expr {
    Expr::ArrayAccess {
        array: Box::new(array),
        index: Box::new(index),
        span: dummy_span(),
    }
}

#[test]
fn test_literal_number() {
    let tokens = create_tokens(vec![
        TokenKind::Numeric(Number::Integer(42)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0],  Stmt::Expression { expr: num_lit(42) });
}

#[test]
fn test_literal_bool() {
    let tokens = create_tokens(vec![TokenKind::KeywordBool(true), TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: bool_lit(true)});
}

#[test]
fn test_literal_nullptr() {
    let tokens = create_tokens(vec![TokenKind::KeywordNullptr, TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: nullptr_lit()});
}

#[test]
fn test_literal_string() {
    let tokens = create_tokens(vec![
        TokenKind::StringLiteral("assssss".to_string()),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: string_lit("assssss")});
}

#[test]
fn test_literal_char() {
    let tokens = create_tokens(vec![
        TokenKind::CharLiteral("a".to_string()),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: char_lit("a")});
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
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: unary_expr(UnaryOp::Negate, num_lit(5))});
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
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: unary_expr(UnaryOp::Not, bool_lit(true))});
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
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {expr: binary_expr(
            num_lit(3),
            BinaryOp::Add,
            binary_expr(num_lit(5), BinaryOp::Multiply, num_lit(2))
        )}
    );
}

#[test]
fn test_binary_precedence_minus() {
    let tokens = create_tokens(vec![
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::Minus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(2)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {expr: binary_expr(
            num_lit(3),
            BinaryOp::Subtract,
            binary_expr(num_lit(5), BinaryOp::Multiply, num_lit(2))
        )}
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
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {expr: binary_expr(
            grouping_expr(binary_expr(num_lit(3), BinaryOp::Add, num_lit(5))),
            BinaryOp::Multiply,
            num_lit(2)
        )}
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
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {expr: assign_expr("x", num_lit(5))});
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
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {
        expr: assign_expr("x", assign_expr("y", num_lit(5))),
    })
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
    assert_eq!(errors[0].message().unwrap(), "Invalid assignment target: Equal");
    assert_eq!(expr.len(),2);
    assert_eq!(expr[0], Stmt::Expression {
        expr: num_lit(5)
    });
    assert_eq!(expr[1], Stmt::Expression {
        expr: num_lit(10)
    });
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
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: call_expr(
                variable_expr("foo"),
                vec![
                    num_lit(1),
                    binary_expr(num_lit(2), BinaryOp::Add, num_lit(3)),
                ]
            )
        }
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
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: array_access_expr(
                variable_expr("arr"),
                binary_expr(num_lit(0), BinaryOp::Add, num_lit(1))
            )
        }
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
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: array_access_expr(variable_expr("arr"), nullptr_lit())
        }
    );
    assert_eq!(
        errors[0].message().unwrap(),
        "Unexpected token: CloseBracket"
    );
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
    assert_eq!(errors[0].message().unwrap(), "Unclosed parenthesis: Expected 'CloseParen' but found Eof");
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression { expr: grouping_expr(num_lit(5))});
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
    assert_eq!(expr.len(),0);
}

#[test]
fn test_empty_input() {
    let tokens = create_tokens(vec![TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert_eq!(errors.len(), 0);
    //assert_eq!(errors[0].message().unwrap(), "Unexpected token: Eof");
    assert_eq!(expr.len(),0);
    //assert_eq!(expr, None);
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
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr,
        vec![Stmt::Expression {
            expr: grouping_expr(grouping_expr(grouping_expr(num_lit(5))))
        }]
    );
}

// Test all binary operators
#[test]
fn test_all_binary_operators() {
    let test_cases = vec![
        (TokenKind::Plus, BinaryOp::Add),
        (TokenKind::Minus, BinaryOp::Subtract),
        (TokenKind::Star, BinaryOp::Multiply),
        (TokenKind::Slash, BinaryOp::Divide),
        (TokenKind::Percent, BinaryOp::Modulo),
        (TokenKind::EqualEqual, BinaryOp::Equal),
        (TokenKind::NotEqual, BinaryOp::NotEqual),
        (TokenKind::Less, BinaryOp::Less),
        (TokenKind::LessEqual, BinaryOp::LessEqual),
        (TokenKind::Greater, BinaryOp::Greater),
        (TokenKind::GreaterEqual, BinaryOp::GreaterEqual),
        (TokenKind::AndAnd, BinaryOp::And),
        (TokenKind::OrOr, BinaryOp::Or),
        (TokenKind::And, BinaryOp::BitwiseAnd),
        (TokenKind::Or, BinaryOp::BitwiseOr),
        (TokenKind::Xor, BinaryOp::BitwiseXor),
        (TokenKind::ShiftLeft, BinaryOp::ShiftLeft),
        (TokenKind::ShiftRight, BinaryOp::ShiftRight),
    ];

    for (token_kind, op) in test_cases {
        let tokens: Vec<Token> = vec![
            crate::num_token(3.0),
            Token {
                kind: token_kind.clone(),
                span: dummy_span(),
            },
            crate::num_token(4.0),
            Token {
                kind: TokenKind::Eof,
                span: dummy_span(),
            },
        ];
        let parser = JsavParser::new(tokens);
        let (expr, errors) = parser.parse();

        assert!(errors.is_empty(), "Failed for {:?}", token_kind);
        assert_eq!(expr.len(),1);
        assert_eq!(
            expr[0],
            Stmt::Expression {
                expr: binary_expr(float_lit(3.0), op, float_lit(4.0))
            },
            "Failed for {:?}",
            token_kind
        );
    }
}

#[test]
fn test_unary_operators_bp() {
    let token = Token {
        kind: TokenKind::Dot,
        span: dummy_span(),
    };
    assert_eq!(unary_binding_power(&token), (0, 0))
}

// Test for line 128-132: Variable with Unicode identifier
#[test]
fn test_variable_unicode_identifier() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierUnicode("変数".into()),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0], Stmt::Expression {
               expr: variable_expr("変数")});
}

// Test for lines 143-144: Unary operator precedence
#[test]
fn test_unary_precedence() {
    let tokens = create_tokens(vec![
        TokenKind::Minus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: binary_expr(
                unary_expr(UnaryOp::Negate, num_lit(5)),
                BinaryOp::Multiply,
                num_lit(3)
            )
        }
    );
}

// Test for line 170: Assignment with invalid target (function call)
#[test]
fn test_assignment_invalid_target_function_call() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("foo".into()),
        TokenKind::OpenParen,
        TokenKind::CloseParen,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Invalid assignment target: Equal");
    assert_eq!(expr.len(),2);
    /*if let Some(Expr::Call { .. }) = expr {
    } else {
        panic!("Expected call expression");
    }*/
    assert_eq!(expr[0], Stmt::Expression {
        expr: call_expr(variable_expr("foo"), vec![])
    });
    assert_eq!(expr[1], Stmt::Expression {
        expr: num_lit(5)
    });
}

// Test for lines 178-179: Function call with zero arguments
#[test]
fn test_function_call_zero_arguments() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("foo".into()),
        TokenKind::OpenParen,
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: call_expr(variable_expr("foo"), vec![])
        }
    );
}

// Test for lines 178-179: Unclosed function call
#[test]
fn test_function_call_unclosed_paren() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("foo".into()),
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unclosed function call: Expected 'CloseParen' but found Eof");
    assert_eq!(expr.len(),1);
    assert!(matches!(expr[0], Stmt::Expression {
        expr: Expr::Call { .. }
    }));
}

// Test for line 170: Assignment with binary expr target
#[test]
fn test_assignment_invalid_target_binary() {
    let tokens = create_tokens(vec![
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(10)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Invalid assignment target: Equal");
    assert_eq!(expr.len(),2);
    assert!(matches!(expr[0], Stmt::Expression {
        expr: Expr::Binary { .. }
    }));
}


#[test]
fn test_invalid_binary_operator() {
    // List of token kinds that are not valid binary operators
    let invalid_kinds = vec![
        TokenKind::Comma,
        TokenKind::Dot,
        TokenKind::KeywordIf,
        TokenKind::PlusEqual,    // Compound assignment
    ];

    for kind in invalid_kinds {
        let token = Token {
            kind: kind.clone(),
            span: dummy_span(), // Dummy span
            // Include other required fields for Token if any
        };

        let result = BinaryOp::get_op(&token);
        assert!(result.is_err(), "Expected error for token: {:?}", kind);

        // Verify error details
        let err = result.unwrap_err();
        match err {
            CompileError::SyntaxError { message, span } => {
                assert_eq!(
                    message,
                    format!("Invalid binary operator: {:?}", kind),
                    "Incorrect message for token: {:?}",
                    kind
                );
                assert_eq!(span, token.span, "Incorrect span for token: {:?}", kind);
            }
            _ => panic!("Unexpected error type for token: {:?}", kind),
        }
    }
}

// Test per identificatori Unicode in assegnazioni
#[test]
fn test_assignment_unicode_variable() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierUnicode("変数".into()),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(expr[0],  Stmt::Expression { expr: assign_expr("変数", num_lit(5))});
}

// Test per accessi ad array concatenati
#[test]
fn test_chained_array_access() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("arr".into()),
        TokenKind::OpenBracket,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::CloseBracket,
        TokenKind::OpenBracket,
        TokenKind::Numeric(Number::Integer(2)),
        TokenKind::CloseBracket,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: array_access_expr(
                array_access_expr(variable_expr("arr"), num_lit(1)),
                num_lit(2)
            )
        }
    );
}

// Test per operatore unario non valido
#[test]
fn test_invalid_unary_operator() {
    let tokens = create_tokens(vec![
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: Star");
    assert_eq!(expr.len(),0);
    //assert_eq!(expr, None);
}

// Test per chiamate a funzione annidate
#[test]
fn test_nested_function_calls() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("foo".into()),
        TokenKind::OpenParen,
        TokenKind::IdentifierAscii("bar".into()),
        TokenKind::OpenParen,
        TokenKind::CloseParen,
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: call_expr(
                variable_expr("foo"),
                vec![call_expr(variable_expr("bar"), vec![])]
            )
        }
    );
}

// Test per errori multipli in un'espressione
#[test]
fn test_multiple_errors_in_expression() {
    let tokens = create_tokens(vec![
        TokenKind::Plus,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: Plus");
}

// Test per tipi misti di letterali in chiamate a funzione
#[test]
fn test_mixed_literals_function_call() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("test".into()),
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::Comma,
        TokenKind::StringLiteral("due".into()),
        TokenKind::Comma,
        TokenKind::KeywordBool(true),
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: call_expr(
                variable_expr("test"),
                vec![num_lit(1), string_lit("due"), bool_lit(true)]
            )
        }
    );
}

// Test per errori di annidamento complessi
#[test]
fn test_complex_nesting_errors() {
    let tokens = create_tokens(vec![
        TokenKind::OpenParen,
        TokenKind::OpenBracket,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: OpenBracket");
}

#[test]
fn test_bitwise_operator_precedence() {
    let tokens = create_tokens(vec![
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::Or,
        TokenKind::Numeric(Number::Integer(2)),
        TokenKind::And,
        TokenKind::Numeric(Number::Integer(3)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(),1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: binary_expr(
                num_lit(1),
                BinaryOp::BitwiseOr,
                binary_expr(num_lit(2), BinaryOp::BitwiseAnd, num_lit(3))
            )
        }
    );
}

// Test per errori di parsing in contesti annidati
#[test]
fn test_nested_parsing_errors() {
    let tokens = create_tokens(vec![
        TokenKind::OpenParen,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::CloseParen,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: Star");
}

// Test per errori di parsing in contesti annidati
#[test]
fn test_nested_unkown_binding_power() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("assssss".to_string()),
        TokenKind::PlusEqual,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unexpected operator: PlusEqual");
}

// Existing imports and helper functions remain unchanged

/*#[test]
fn test_function_declaration() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordFun,
        TokenKind::IdentifierAscii("add".into()),
        TokenKind::OpenParen,
        TokenKind::IdentifierAscii("a".into()),
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::Comma,
        TokenKind::IdentifierAscii("b".into()),
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::CloseParen,
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::OpenBrace,
        TokenKind::KeywordReturn,
        TokenKind::IdentifierAscii("a".into()),
        TokenKind::Plus,
        TokenKind::IdentifierAscii("b".into()),
        TokenKind::Semicolon,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();

    assert!(!errors.is_empty());
    assert_eq!(statements.len(), 0);

    if let Stmt::Function {
        name,
        parameters,
        return_type,
        body,
        ..
    } = &statements[0] {
        assert_eq!(name, "add");
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].name, "a");
        assert_eq!(parameters[0].type_annotation, Type::I32);
        assert_eq!(parameters[1].name, "b");
        assert_eq!(parameters[1].type_annotation, Type::I32);
        assert_eq!(*return_type, Type::I32);
        assert_eq!(body.len(), 1);

        if let Stmt::Return { value, .. } = &body[0] {
            assert!(value.is_some());
            if let Expr::Binary { left, op, right, .. } = value.as_ref().unwrap() {
                assert_eq!(**left, variable_expr("a"));
                assert_eq!(*op, BinaryOp::Add);
                assert_eq!(**right, variable_expr("b"));
            } else {
                panic!("Expected binary expression in return");
            }
        } else {
            panic!("Expected return statement");
        }
    } else {
        panic!("Expected function declaration");
    }
}*/

/*#[test]
fn test_variable_declaration_with_type() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordVar,
        TokenKind::IdentifierAscii("count".into()),
        TokenKind::Colon,
        TokenKind::TypeU64,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(100)),
        TokenKind::Semicolon,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();

    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);

    if let Stmt::VarDeclaration {
        variables,
        type_annotation,
        initializers,
        ..
    } = &statements[0] {
        assert_eq!(variables, &vec!["count"]);
        assert_eq!(*type_annotation, Type::U64);
        assert_eq!(initializers.len(), 1);
        assert_eq!(initializers[0], num_lit(100));
    } else {
        panic!("Expected variable declaration");
    }
}*/

#[test]
fn test_nested_if_statements() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordIf,
        TokenKind::OpenParen,
        TokenKind::KeywordBool(true),
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::KeywordIf,
        TokenKind::OpenParen,
        TokenKind::KeywordBool(false),
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::KeywordReturn,
        TokenKind::Semicolon,
        TokenKind::CloseBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();

    assert!(!errors.is_empty());
    assert_eq!(statements.len(), 1);

    if let Stmt::If {
        condition,
        then_branch,
        else_branch: _else_branch,
        ..
    } = &statements[0] {
        assert_eq!(*condition, bool_lit(true));
        assert_eq!(then_branch.len(), 1);

        if let Stmt::If {
            condition: nested_cond,
            then_branch: nested_then,
            else_branch: nested_else,
            ..
        } = &then_branch[0] {
            assert_eq!(*nested_cond, bool_lit(false));
            assert_eq!(nested_then.len(), 1);
            assert!(nested_else.is_none());
        } else {
            panic!("Expected nested if statement");
        }
    } else {
        panic!("Expected if statement");
    }
}

/*#[test]
fn test_for_loop_desugaring() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordFor,
        TokenKind::OpenParen,
        TokenKind::KeywordVar,
        TokenKind::IdentifierAscii("i".into()),
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(0)),
        TokenKind::Semicolon,
        TokenKind::IdentifierAscii("i".into()),
        TokenKind::Less,
        TokenKind::Numeric(Number::Integer(10)),
        TokenKind::Semicolon,
        TokenKind::IdentifierAscii("i".into()),
        TokenKind::Equal,
        TokenKind::IdentifierAscii("i".into()),
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(1)),
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();

    assert!(!errors.is_empty());
    assert_eq!(statements.len(), 0);

    if let Stmt::Block { statements: block_stmts, .. } = &statements[0] {
        assert_eq!(block_stmts.len(), 2);

        // Check init statement
        if let Stmt::VarDeclaration { variables, .. } = &block_stmts[0] {
            assert_eq!(variables, &vec!["i"]);
        } else {
            panic!("Expected var declaration in init");
        }

        // Check while loop
        if let Stmt::While { condition, body, .. } = &block_stmts[1] {
            assert_eq!(*condition, binary_expr(
                variable_expr("i"),
                BinaryOp::Less,
                num_lit(10)
            ));

            // Body should include both the empty block and increment
            assert_eq!(body.len(), 1);
            if let Stmt::Expression { expr } = &body[0] {
                assert_eq!(*expr, assign_expr(
                    "i",
                    binary_expr(
                        variable_expr("i"),
                        BinaryOp::Add,
                        num_lit(1)
                    )
                ));
            }
        } else {
            panic!("Expected while loop");
        }
    } else {
        panic!("Expected block statement");
    }
}*/

/*#[test]
fn test_type_parsing() {
    let test_cases = vec![
        (TokenKind::TypeI8, Type::I8),
        (TokenKind::TypeU16, Type::U16),
        (TokenKind::TypeF32, Type::F32),
        (TokenKind::TypeString, Type::String),
        (TokenKind::TypeBool, Type::Bool),
    ];

    for (token_kind, expected_type) in test_cases {
        let tokens = create_tokens(vec![token_kind, TokenKind::Eof]);
        let mut parser = JsavParser::new(tokens);
        let parsed_type = parser.parse_type().unwrap();
        assert_eq!(parsed_type, expected_type);
    }
}*/

#[test]
fn test_invalid_type_error() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordVar,
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Colon,
        TokenKind::IdentifierAscii("invalid".into()), // Not a valid type
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(42)),
        TokenKind::Semicolon,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_, errors) = parser.parse();

    assert!(!errors.is_empty());
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected type annotation but found: IdentifierAscii(\"invalid\")"
    );
}

#[test]
fn test_error_recovery_after_invalid_statement() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordReturn, // Invalid without function
        TokenKind::Numeric(Number::Integer(42)),
        TokenKind::Semicolon,
        TokenKind::KeywordVar,
        TokenKind::IdentifierAscii("valid".into()),
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::Semicolon,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();

    assert!(!errors.is_empty());
    assert_eq!(statements.len(), 2); // Should parse the valid var declaration
}

#[test]
fn test_function_parameter_errors() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordFun,
        TokenKind::IdentifierAscii("foo".into()),
        TokenKind::OpenParen,
        TokenKind::IdentifierAscii("a".into()),
        TokenKind::Comma, // Missing colon and type
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_, errors) = parser.parse();

    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.message().unwrap().contains("Expected ':' after parameter name")));
}

#[test]
fn test_unicode_function_name() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordFun,
        TokenKind::IdentifierUnicode("こんにちは".into()),
        TokenKind::OpenParen,
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_statements, errors) = parser.parse();

    assert!(!errors.is_empty());
    /*if let Stmt::Function { name, .. } = &statements[0] {
        assert_eq!(name, "こんにちは");
    } else {
        panic!("Expected function declaration");
    }*/
}