use jsavrs::error::compile_error::CompileError;
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
use jsavrs::location::source_location::SourceLocation;
use jsavrs::location::source_span::SourceSpan;
use jsavrs::parser::ast::*;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::parser::precedence::unary_binding_power;
use jsavrs::tokens::number::Number;
use jsavrs::tokens::token::Token;
use jsavrs::tokens::token_kind::TokenKind;
use jsavrs::utils::*;
use std::sync::Arc;

fn test_span(
    sline: usize,
    scolon: usize,
    spos: usize,
    eline: usize,
    ecolon: usize,
    epos: usize,
) -> SourceSpan {
    SourceSpan::new(
        Arc::from("test.vn"),
        SourceLocation::new(sline, scolon, spos),
        SourceLocation::new(eline, ecolon, epos),
    )
}

macro_rules! literal_test {
    ($name:ident, $token:expr, $expected_expr:expr) => {
        #[test]
        fn $name() {
            let tokens = create_tokens(vec![$token, TokenKind::Eof]);
            let parser = JsavParser::new(tokens);
            let (expr, errors) = parser.parse();
            assert!(errors.is_empty());
            assert_eq!(expr.len(), 1);
            assert_eq!(
                expr[0],
                Stmt::Expression {
                    expr: $expected_expr
                }
            );
        }
    };
}

// Macro per test operatori binari
macro_rules! binary_op_test {
    ($test_name:ident, $token_kind:expr, $op:expr) => {
        #[test]
        fn $test_name() {
            let tokens = vec![
                num_token(3.0),
                Token {
                    kind: $token_kind.clone(),
                    span: dummy_span(),
                },
                num_token(4.0),
                Token {
                    kind: TokenKind::Eof,
                    span: dummy_span(),
                },
            ];
            let parser = JsavParser::new(tokens);
            let (expr, errors) = parser.parse();
            assert!(errors.is_empty(), "Failed for {:?}", $token_kind);
            assert_eq!(expr.len(), 1);
            assert_eq!(
                expr[0],
                Stmt::Expression {
                    expr: binary_expr(float_lit(3.0), $op, float_lit(4.0))
                },
                "Failed for {:?}",
                $token_kind
            );
        }
    };
}

// Macro per test operatori unari
macro_rules! unary_op_test {
    ($name:ident, $token_kind:expr, $op:expr, $operand:expr, $expected_expr:expr) => {
        #[test]
        fn $name() {
            let tokens = create_tokens(vec![$token_kind, $operand, TokenKind::Eof]);
            let parser = JsavParser::new(tokens);
            let (expr, errors) = parser.parse();
            assert!(errors.is_empty());
            assert_eq!(expr.len(), 1);
            assert_eq!(
                expr[0],
                Stmt::Expression {
                    expr: unary_expr($op, $expected_expr)
                }
            );
        }
    };
}

// Test per letterali usando la macro
literal_test!(
    test_literal_number,
    TokenKind::Numeric(Number::Integer(42)),
    num_lit(42)
);
literal_test!(
    test_literal_bool,
    TokenKind::KeywordBool(true),
    bool_lit(true)
);
literal_test!(
    test_literal_nullptr,
    TokenKind::KeywordNullptr,
    nullptr_lit()
);
literal_test!(
    test_literal_string,
    TokenKind::StringLiteral("assssss".to_string()),
    string_lit("assssss")
);
literal_test!(
    test_literal_char,
    TokenKind::CharLiteral("a".to_string()),
    char_lit("a")
);

// Test per operatori binari usando la macro
binary_op_test!(test_add, TokenKind::Plus, BinaryOp::Add);
binary_op_test!(test_subtract, TokenKind::Minus, BinaryOp::Subtract);
binary_op_test!(test_multiply, TokenKind::Star, BinaryOp::Multiply);
binary_op_test!(test_divide, TokenKind::Slash, BinaryOp::Divide);
binary_op_test!(test_modulo, TokenKind::Percent, BinaryOp::Modulo);
binary_op_test!(test_equal, TokenKind::EqualEqual, BinaryOp::Equal);
binary_op_test!(test_not_equal, TokenKind::NotEqual, BinaryOp::NotEqual);
binary_op_test!(test_less, TokenKind::Less, BinaryOp::Less);
binary_op_test!(test_less_equal, TokenKind::LessEqual, BinaryOp::LessEqual);
binary_op_test!(test_greater, TokenKind::Greater, BinaryOp::Greater);
binary_op_test!(
    test_greater_equal,
    TokenKind::GreaterEqual,
    BinaryOp::GreaterEqual
);
binary_op_test!(test_and, TokenKind::AndAnd, BinaryOp::And);
binary_op_test!(test_or, TokenKind::OrOr, BinaryOp::Or);
binary_op_test!(test_bitwise_and, TokenKind::And, BinaryOp::BitwiseAnd);
binary_op_test!(test_bitwise_or, TokenKind::Or, BinaryOp::BitwiseOr);
binary_op_test!(test_bitwise_xor, TokenKind::Xor, BinaryOp::BitwiseXor);
binary_op_test!(test_shift_left, TokenKind::ShiftLeft, BinaryOp::ShiftLeft);
binary_op_test!(
    test_shift_right,
    TokenKind::ShiftRight,
    BinaryOp::ShiftRight
);

// Test per operatori unari usando la macro
unary_op_test!(
    test_unary_negation,
    TokenKind::Minus,
    UnaryOp::Negate,
    TokenKind::Numeric(Number::Integer(5)),
    num_lit(5)
);
unary_op_test!(
    test_unary_not,
    TokenKind::Not,
    UnaryOp::Not,
    TokenKind::KeywordBool(true),
    bool_lit(true)
);

macro_rules! test_binary_precedence {
    ($test_name:ident, $first_op:expr, $binary_op:expr) => {
        #[test]
        fn $test_name() {
            let tokens = create_tokens(vec![
                TokenKind::Numeric(Number::Integer(3)),
                $first_op,
                TokenKind::Numeric(Number::Integer(5)),
                TokenKind::Star,
                TokenKind::Numeric(Number::Integer(2)),
                TokenKind::Eof,
            ]);
            let parser = JsavParser::new(tokens);
            let (expr, errors) = parser.parse();
            assert!(errors.is_empty());
            assert_eq!(expr.len(), 1);
            assert_eq!(
                expr[0],
                Stmt::Expression {
                    expr: binary_expr(
                        num_lit(3),
                        $binary_op,
                        binary_expr(num_lit(5), BinaryOp::Multiply, num_lit(2))
                    )
                }
            );
        }
    };
}

test_binary_precedence!(test_binary_precedence_plus, TokenKind::Plus, BinaryOp::Add);
test_binary_precedence!(
    test_binary_precedence_minus,
    TokenKind::Minus,
    BinaryOp::Subtract
);

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
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: binary_expr(
                grouping_expr(binary_expr(num_lit(3), BinaryOp::Add, num_lit(5))),
                BinaryOp::Multiply,
                num_lit(2)
            )
        }
    );
}

macro_rules! assignment_test {
    (
        $name:ident,
        [$($tok:expr),* $(,)?],
        $expect_err:expr,
        [$($expected_stmt:expr),* $(,)?],
        $err_msg:expr
    ) => {
        #[test]
        fn $name() {
            let tokens = create_tokens(vec![$($tok),*]);
            let parser = JsavParser::new(tokens);
            let (stmts, errors) = parser.parse();

            if $expect_err {
                assert!(!errors.is_empty(), "expected at least one parse‐error");
                assert_eq!(
                    errors[0].message().unwrap(),
                    $err_msg,
                    "wrong error message"
                );
                // Skip statement checks in error case
            } else {
                assert!(
                    errors.is_empty(),
                    "expected no parse‐errors but found: {:?}",
                    errors
                );
                let expected_vec: Vec<Stmt> = vec![$($expected_stmt),*];
                assert_eq!(
                    stmts.len(),
                    expected_vec.len(),
                    "expected {} statements, got {}",
                    expected_vec.len(),
                    stmts.len()
                );
                for (i, exp) in expected_vec.into_iter().enumerate() {
                    assert_eq!(stmts[i], exp, "mismatch at stmt index {}", i);
                }
            }
        }
    };
}

assignment_test!(
    test_assignment_valid,
    [
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ],
    false,
    [Stmt::Expression {
        expr: assign_expr(variable_expr("x"), num_lit(5)),
    },],
    "" // (unused because `expect_err = false`)
);
assignment_test!(
    test_assignment_array_indexing_valid,
    [
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::OpenBracket,
        TokenKind::Numeric(Number::Integer(0)),
        TokenKind::CloseBracket,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ],
    false,
    [Stmt::Expression {
        expr: assign_expr(
            array_access_expr(variable_expr("x"), num_lit(0)),
            num_lit(5)
        ),
    },],
    "" // (unused because `expect_err = false`)
);

assignment_test!(
    test_assignment_cained,
    [
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Equal,
        TokenKind::IdentifierAscii("y".into()),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ],
    false,
    [Stmt::Expression {
        expr: assign_expr(
            variable_expr("x"),
            assign_expr(variable_expr("y"), num_lit(5))
        ),
    },],
    ""
);

assignment_test!(
    test_assignment_invalid_target,
    [
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(10)),
        TokenKind::Eof,
    ],
    true,
    [
        Stmt::Expression { expr: num_lit(5) },
        Stmt::Expression { expr: num_lit(10) },
    ],
    "Invalid left-hand side in assignment"
);

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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(expr.len(), 0);
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: ']'");
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
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected ')' in end of grouping, found end of file.\nHelp: Try adding a ')'"
    );
    assert_eq!(expr.len(), 0);
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
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: '+'");
    assert_eq!(expr.len(), 0);
}

#[test]
fn test_empty_input() {
    let tokens = create_tokens(vec![TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert_eq!(errors.len(), 0);
    //assert_eq!(errors[0].message().unwrap(), "Unexpected token: Eof");
    assert_eq!(expr.len(), 0);
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
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr,
        vec![Stmt::Expression {
            expr: grouping_expr(grouping_expr(grouping_expr(num_lit(5))))
        }]
    );
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
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: variable_expr("変数")
        }
    );
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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(
        errors[0].message().unwrap(),
        "Invalid left-hand side in assignment"
    );
    assert_eq!(expr.len(), 0);
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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected ')' in end of function call arguments, found end of file.\nHelp: Try adding a ')'"
    );
    assert_eq!(expr.len(), 0);
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
    assert_eq!(
        errors[0].message().unwrap(),
        "Invalid left-hand side in assignment"
    );
    assert_eq!(expr.len(), 0);
}

#[test]
fn test_invalid_binary_operator() {
    // List of token kinds that are not valid binary operators
    let invalid_kinds = vec![
        TokenKind::Comma,
        TokenKind::Dot,
        TokenKind::KeywordIf,
        TokenKind::PlusEqual, // Compound assignment
    ];

    for kind in invalid_kinds {
        let token = Token {
            kind: kind.clone(),
            span: dummy_span(), // Dummy span
        };

        let result = BinaryOp::get_op(&token);
        assert!(result.is_err(), "Expected error for token: {:?}", kind);

        // Verify error details
        let err = result.unwrap_err();
        match err {
            CompileError::SyntaxError { message, span, help } => {
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
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr[0],
        Stmt::Expression {
            expr: assign_expr(variable_expr("変数"), num_lit(5))
        }
    );
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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: '*'");
    assert_eq!(expr.len(), 0);
}

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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: '+'");
}

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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: '['");
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
    assert_eq!(expr.len(), 1);
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
    assert_eq!(errors[0].message().unwrap(), "Unexpected token: '*'");
}

#[test]
fn test_nested_unknown_binding_power() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierAscii("assssss".to_string()),
        TokenKind::PlusEqual,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(errors[0].message().unwrap(), "Unexpected operator: '+='");
}

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
    assert_eq!(
        statements[0],
        Stmt::If {
            condition: bool_lit(true),
            then_branch: vec![Stmt::Block {
                statements: vec![Stmt::If {
                    condition: bool_lit(false),
                    then_branch: vec![Stmt::Block {
                        statements: vec![Stmt::Return {
                            value: None,
                            span: dummy_span()
                        }],
                        span: dummy_span(),
                    }],
                    else_branch: None,
                    span: dummy_span(),
                }],
                span: dummy_span(),
            }],
            else_branch: None,
            span: dummy_span(),
        }
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
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected ':' in after parameter name, found ','.\nHelp: Try adding a ':'"
    );
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
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);
    assert_eq!(
        statements[0],
        Stmt::Function {
            name: "こんにちは".into(),
            parameters: vec![],
            return_type: Type::Void,
            body: vec![Stmt::Block {
                statements: vec![],
                span: dummy_span(),
            }],
            span: dummy_span(),
        }
    );
}

#[test]
fn test_block_stmt() {
    let tokens = create_tokens(vec![
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);
    assert_eq!(
        statements[0],
        Stmt::Block {
            statements: vec![],
            span: dummy_span()
        }
    );
}

#[test]
fn test_break_statement_in_if() {
    // TokenKind::KeywordIf
    let tokens = create_tokens(vec![
        TokenKind::KeywordIf,
        TokenKind::OpenParen,
        TokenKind::KeywordBool(true),
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::KeywordBreak,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);
    assert_eq!(
        statements[0],
        Stmt::If {
            condition: bool_lit(true),
            then_branch: vec![Stmt::Block {
                statements: vec![Stmt::Break { span: dummy_span() }],
                span: dummy_span(),
            }],
            else_branch: None,
            span: dummy_span(),
        }
    );
}

#[test]
fn test_continue_statement_in_if() {
    // TokenKind::KeywordIf
    let tokens = create_tokens(vec![
        TokenKind::KeywordIf,
        TokenKind::OpenParen,
        TokenKind::KeywordBool(true),
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::KeywordContinue,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);
    assert_eq!(
        statements[0],
        Stmt::If {
            condition: bool_lit(true),
            then_branch: vec![Stmt::Block {
                statements: vec![Stmt::Continue { span: dummy_span() }],
                span: dummy_span(),
            }],
            else_branch: None,
            span: dummy_span(),
        }
    );
}

#[test]
fn test_empty_for_loop() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordFor,
        TokenKind::OpenParen,
        TokenKind::Semicolon,
        TokenKind::Semicolon,
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);
    assert_eq!(
        statements[0],
        Stmt::For {
            initializer: None,
            condition: None,
            increment: None,
            body: vec![],
            span: dummy_span(),
        }
    );
}

#[test]
fn test_full_for_loop() {
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
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);
    assert_eq!(
        statements[0],
        Stmt::For {
            initializer: Some(Box::from(var_declaration(
                vec!["i".to_string()],
                Type::I32,
                true,
                vec![num_lit(0)]
            ))),
            condition: Some(binary_expr(variable_expr("i"), BinaryOp::Less, num_lit(10))),
            increment: None,
            body: vec![],
            span: dummy_span(),
        }
    );
}

#[test]
fn if_whit_else_brach() {
    let input = "if (true) { continue } else { break }";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lerrors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (statements, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(statements.len(), 1);

    // Calculate expected spans based on input string
    assert_eq!(
        statements[0],
        Stmt::If {
            condition: Expr::Literal {
                value: LiteralValue::Bool(true),
                span: test_span(1, 5, 4, 1, 9, 8)
            },
            then_branch: vec![Stmt::Block {
                statements: vec![Stmt::Continue {
                    span: test_span(1, 13, 12, 1, 21, 20)
                }],
                span: test_span(1, 11, 10, 1, 23, 22)
            }],
            else_branch: Some(vec![Stmt::Block {
                statements: vec![Stmt::Break {
                    span: test_span(1, 31, 30, 1, 36, 35)
                }],
                span: test_span(1, 29, 28, 1, 38, 37)
            }]),
            span: test_span(1, 1, 0, 1, 38, 37)
        }
    );
}

/// Macro per generare automaticamente un test di dichiarazione di variabile.
///
/// Parametri:
/// - `$test_name`: identificatore del test (es. `test_b_i8_5`)
/// - `$input`: stringa di sorgente da passare al lexer (es. `"var b: i8 = 5"`)
/// - `$var_name`: nome della variabile come stringa (es. `"b"`)
/// - `$type`: tipo atteso, come variante di `Type` (es. `Type::I8`)
/// - `$lit_val`: valore letterale intero atteso (es. `5`)
/// - `$lit_start_line`, `$lit_start_col`, `$lit_start_off`: posizione di inizio del literal (linea, colonna, offset)
/// - `$lit_end_line`,   `$lit_end_col`,   `$lit_end_off`: posizione di fine del literal (linea, colonna, offset)
/// - `$full_start_line`, `$full_start_col`, `$full_start_off`: posizione di inizio dell’intera dichiarazione
/// - `$full_end_line`,   `$full_end_col`,   `$full_end_off`: posizione di fine dell’intera dichiarazione
///
/// Nota: se non vuoi passare esplicitamente tutti gli span, puoi farne un overload o fissarli a `0`, ma in questo esempio
/// vengono esplicitati per riflettere esattamente ciò che hai nel test originale.
macro_rules! test_var_decl {
    (
        $test_name:ident,
        $input:expr,
        $var_name:expr,
        $type:expr,
        $lit:expr,
        // span del literal
        $lit_start_line:expr, $lit_start_col:expr, $lit_start_off:expr,
        $lit_end_line:expr,   $lit_end_col:expr,   $lit_end_off:expr,
        // span dell’intera dichiarazione
        $full_start_line:expr, $full_start_col:expr, $full_start_off:expr,
        $full_end_line:expr,   $full_end_col:expr,   $full_end_off:expr
    ) => {
        #[allow(clippy::approx_constant)]
        #[test]
        fn $test_name() {
            // 1) Tokenizzazione
            let input = $input;
            let mut lexer = Lexer::new("test.vn", &input);
            let (tokens, lex_errors) = lexer_tokenize_with_errors(&mut lexer);
            assert!(
                lex_errors.is_empty(),
                "Errori di lexing in test `{}`: {:?}",
                stringify!($test_name),
                lex_errors
            );

            // 2) Parsing
            let parser = JsavParser::new(tokens);
            let (statements, parse_errors) = parser.parse();
            assert!(
                parse_errors.is_empty(),
                "Errori di parsing in test `{}`: {:?}",
                stringify!($test_name),
                parse_errors
            );
            assert_eq!(
                statements.len(),
                1,
                "Si attende esattamente 1 statement, ma ne sono stati prodotti {}",
                statements.len()
            );

            // 3) Costruzione dello span per il literal
            let lit_span = SourceSpan {
                file_path: Arc::from("test.vn"),
                start: SourceLocation::new($lit_start_line, $lit_start_col, $lit_start_off),
                end: SourceLocation::new($lit_end_line, $lit_end_col, $lit_end_off),
            };

            // 4) Costruzione dello span per l’intera dichiarazione
            let full_span = SourceSpan {
                file_path: Arc::from("test.vn"),
                start: SourceLocation::new($full_start_line, $full_start_col, $full_start_off),
                end: SourceLocation::new($full_end_line, $full_end_col, $full_end_off),
            };

            // 5) Asserzione finale: confronto con il `Stmt::VarDeclaration`
            assert_eq!(
                statements[0],
                Stmt::VarDeclaration {
                    variables: vec![$var_name.to_string()],
                    type_annotation: $type,
                    is_mutable: true,
                    initializers: vec![Expr::Literal {
                        value: $lit,
                        span: lit_span.clone(),
                    }],
                    span: full_span.clone(),
                }
            );
        }
    };
}

test_var_decl!(
    test_b_u8_5,
    "var b: u8 = 5u",                                 // input
    "b",                                              // var_name
    Type::U8,                                         // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
    // span
    1,
    13,
    12, // start: riga 1, col 13, offset 12
    1,
    15,
    14, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    15,
    14 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_u16_5,
    "var b: u16 = 5u",                                // input
    "b",                                              // var_name
    Type::U16,                                        // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
    // span
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    16,
    15, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    16,
    15 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_u32_5,
    "var b: u32 = 5u",                                // input
    "b",                                              // var_name
    Type::U32,                                        // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
    // span
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    16,
    15, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    16,
    15 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_u64_5,
    "var b: u64 = 5u",                                // input
    "b",                                              // var_name
    Type::U64,                                        // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
    // span
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    16,
    15, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    16,
    15 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_i8_5,
    "var b: i8 = 5",                          // input
    "b",                                      // var_name
    Type::I8,                                 // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
    // span
    1,
    13,
    12, // start: riga 1, col 13, offset 12
    1,
    14,
    13, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    14,
    13 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_i16_5,
    "var b: i16 = 5",                         // input
    "b",                                      // var_name
    Type::I16,                                // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
    // span
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    15,
    14, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    15,
    14 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_i32_5,
    "var b: i32 = 5",                         // input
    "b",                                      // var_name
    Type::I32,                                // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
    // span
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    15,
    14, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    15,
    14 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_i64_5,
    "var b: i64 = 5",                         // input
    "b",                                      // var_name
    Type::I64,                                // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
    // span
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    15,
    14, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    15,
    14 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_char_decl,
    "var b: char = 'a'",                    // input
    "b",                                    // var_name
    Type::Char,                             // tipo atteso,
    LiteralValue::CharLit("a".to_string()), // valore letterale
    // span
    1,
    15,
    14, // start: riga 1, col 13, offset 12
    1,
    18,
    17, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    18,
    17 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_custom_decl,
    "var b: string = \"a\"",                  // input
    "b",                                      // var_name
    Type::String,                             // tipo atteso,
    LiteralValue::StringLit("a".to_string()), // valore letterale
    // span
    1,
    17,
    16, // start: riga 1, col 13, offset 12
    1,
    20,
    19, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    20,
    19 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_string_decl,
    "var b: custom = \"a\"",                  // input
    "b",                                      // var_name
    Type::Custom("custom".to_string()),       // tipo atteso,
    LiteralValue::StringLit("a".to_string()), // valore letterale
    // span
    1,
    17,
    16, // start: riga 1, col 13, offset 12
    1,
    20,
    19, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    20,
    19 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_bool_decl,
    "var b: bool = true",     // input
    "b",                      // var_name
    Type::Bool,               // tipo atteso,
    LiteralValue::Bool(true), // valore letterale
    // span
    1,
    15,
    14, // start: riga 1, col 13, offset 12
    1,
    19,
    18, // end:   riga 1, col 14, offset 13
    // span of the entire declaration
    1,
    1,
    0, // start: riga 1, col 1,  offset 0
    1,
    19,
    18 // end:   riga 1, col 14, offset 13
);

test_var_decl!(
    test_b_f32_3_14,                             // nome del test
    "var b: f32 = 3.14f",                        // input
    "b",                                         // var_name
    Type::F32,                                   // tipo atteso
    LiteralValue::Number(Number::Float32(3.14)), // valore letterale (3.14 in f32)
    // span del literal "3.14" in `"var b: f32 = 3.14"`
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    19,
    18, // end:   riga 1, col 18, offset 17
    // span dell’intera dichiarazione "var b: f32 = 3.14"
    1,
    1,
    0, // start: riga 1, col 1, offset 0
    1,
    19,
    18 // end:   riga 1, col 18, offset 17
);

test_var_decl!(
    test_b_f64_3_14,                             // nome del test
    "var b: f64 = 3.14",                         // input
    "b",                                         // var_name
    Type::F64,                                   // tipo atteso
    LiteralValue::Number(Number::Float64(3.14)), // valore letterale (3.14 in f32)
    // span del literal "3.14" in `"var b: f32 = 3.14"`
    1,
    14,
    13, // start: riga 1, col 13, offset 12
    1,
    18,
    17, // end:   riga 1, col 18, offset 17
    // span dell’intera dichiarazione "var b: f32 = 3.14"
    1,
    1,
    0, // start: riga 1, col 1, offset 0
    1,
    18,
    17 // end:   riga 1, col 18, offset 17
);

#[test]
fn array_declaration() {
    let input = "var arr: i8[5] = {1, 2, 3, 4, 5}";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr[0],
        Stmt::VarDeclaration {
            variables: vec!["arr".into()],
            type_annotation: Type::Array(
                Box::new(Type::I8),
                Box::from(Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(5)),
                    span: test_span(1, 13, 12, 1, 14, 13)
                })
            ),
            is_mutable: true,
            initializers: vec![Expr::ArrayLiteral {
                elements: vec![
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(1)),
                        span: test_span(1, 19, 18, 1, 20, 19)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(2)),
                        span: test_span(1, 22, 21, 1, 23, 22)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(3)),
                        span: test_span(1, 25, 24, 1, 26, 25)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(4)),
                        span: test_span(1, 28, 27, 1, 29, 28)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(5)),
                        span: test_span(1, 31, 30, 1, 32, 31)
                    }
                ],
                span: test_span(1, 18, 17, 1, 33, 32)
            }],
            span: test_span(1, 1, 0, 1, 33, 32)
        }
    );
}

#[test]
fn vector_declaration() {
    let input = "var arr: vector<i8> = {1, 2, 3, 4, 5}";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr[0],
        Stmt::VarDeclaration {
            variables: vec!["arr".into()],
            type_annotation: Type::Vector(Box::new(Type::I8)),
            is_mutable: true,
            initializers: vec![Expr::ArrayLiteral {
                elements: vec![
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(1)),
                        span: test_span(1, 24, 23, 1, 25, 24)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(2)),
                        span: test_span(1, 27, 26, 1, 28, 27)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(3)),
                        span: test_span(1, 30, 29, 1, 31, 30)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(4)),
                        span: test_span(1, 33, 32, 1, 34, 33)
                    },
                    Expr::Literal {
                        value: LiteralValue::Number(Number::Integer(5)),
                        span: test_span(1, 36, 35, 1, 37, 36)
                    }
                ],
                span: test_span(1, 23, 22, 1, 38, 37)
            }],
            span: test_span(1, 1, 0, 1, 38, 37)
        }
    );
}

#[test]
fn test_function_inputs() {
    let input = "fun a(num1: i8, num2: i8): i8 { }";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);
    assert_eq!(
        expr[0],
        Stmt::Function {
            name: "a".into(),
            parameters: vec![
                Parameter {
                    name: "num1".into(),
                    type_annotation: Type::I8,
                    span: test_span(1, 7, 6, 1, 15, 14)
                },
                Parameter {
                    name: "num2".into(),
                    type_annotation: Type::I8,
                    span: test_span(1, 17, 16, 1, 25, 24)
                }
            ],
            return_type: Type::I8,
            body: vec![Stmt::Block {
                statements: vec![],
                span: test_span(1, 31, 30, 1, 34, 33)
            }],
            span: test_span(1, 1, 0, 1, 34, 33)
        }
    );
}

// Add these tests to the existing test module

// Test for line 259: peek() in parse_stmt
#[test]
fn test_peek_in_parse_stmt() {
    let tokens = create_tokens(vec![TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert!(stmts.is_empty());
}

// Tests for lines 264-265: var/const declarations
#[test]
fn test_var_declaration() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordVar,
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(stmts.len(), 1);
    assert!(matches!(stmts[0], Stmt::VarDeclaration { .. }));
}

#[test]
fn test_const_declaration() {
    let tokens = create_tokens(vec![
        TokenKind::KeywordConst,
        TokenKind::IdentifierAscii("x".into()),
        TokenKind::Colon,
        TokenKind::TypeI32,
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(stmts.len(), 1);
    assert!(matches!(stmts[0], Stmt::VarDeclaration { .. }));
}

// Tests for lines 273-277: break statement
#[test]
fn test_break_statement() {
    let tokens = create_tokens(vec![TokenKind::KeywordBreak, TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(stmts.len(), 1);
    assert!(matches!(stmts[0], Stmt::Break { .. }));
}

// Tests for lines 288-292: continue statement
#[test]
fn test_continue_statement() {
    let tokens = create_tokens(vec![TokenKind::KeywordContinue, TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(stmts.len(), 1);
    assert!(matches!(stmts[0], Stmt::Continue { .. }));
}

// Tests for line 298: block statement
#[test]
fn test_block_statement() {
    let tokens = create_tokens(vec![
        TokenKind::OpenBrace,
        TokenKind::KeywordBreak,
        TokenKind::KeywordContinue,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(stmts.len(), 1);

    if let Stmt::Block { statements, .. } = &stmts[0] {
        assert_eq!(statements.len(), 2);
        assert!(matches!(statements[0], Stmt::Break { .. }));
        assert!(matches!(statements[1], Stmt::Continue { .. }));
    } else {
        panic!("Expected Block statement");
    }
}

// Test for line 298 with empty block
#[test]
fn test_empty_block_statement() {
    let tokens = create_tokens(vec![
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    let (stmts, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(stmts.len(), 1);

    if let Stmt::Block { statements, .. } = &stmts[0] {
        assert!(statements.is_empty());
    } else {
        panic!("Expected Block statement");
    }
}

#[test]
fn test_var_no_name() {
    let input = "var ";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected identifier after the 'var' or 'const': end of file"
    );
}

#[test]
fn test_var_no_initializer() {
    let input = "var eee: i32";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected '=' in after type annotation, found end of file.\nHelp: Try adding a '='"
    );
}

#[test]
fn test_unclose_array_literal() {
    let input = "var eee: i32[2] = {1, 2, 3";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(
        errors[0].message().unwrap(),
        "Expected '}' in end of array literal, found end of file.\nHelp: Try adding a '}'"
    );
    assert!(!expr.is_empty());
}

#[test]
fn test_var_invaild_type() {
    let input = "var eee: 5";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (_expr, errors) = parser.parse();
    assert!(!errors.is_empty());
    assert_eq!(
        errors[0].message().unwrap(),
        "Invalid type specification, expected primitive type or custom identifier: number '5'"
    );
}

#[test]
fn test_main() {
    let input = "main { }";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, errors) = parser.parse();
    assert!(errors.is_empty());
    assert_eq!(expr.len(), 1);

    // Calculate expected spans based on input string
    assert_eq!(
        expr[0],
        Stmt::MainFunction {
            body: vec![Stmt::Block {
                statements: vec![],
                span: test_span(1, 6, 5, 1, 9, 8)
            }],
            span: test_span(1, 1, 0, 1, 9, 8)
        }
    );
}
