use insta::assert_debug_snapshot;
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
use jsavrs::parser::ast::*;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::parser::precedence::unary_binding_power;
use jsavrs::tokens::number::Number;
use jsavrs::tokens::token::Token;
use jsavrs::tokens::token_kind::TokenKind;
use jsavrs::utils::*;

macro_rules! literal_test {
    ($test_name:ident, $token:expr) => {
        #[test]
        fn $test_name() {
            let tokens = create_tokens(vec![$token, TokenKind::Eof]);
            let parser = JsavParser::new(tokens);
            assert_debug_snapshot!(stringify!($test_name), parser.parse());
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
            assert_debug_snapshot!(stringify!($test_name), parser.parse());
        }
    };
}

// Macro per test operatori unari
macro_rules! unary_op_test {
    ($test_name:ident, $token_kind:expr, $op:expr, $operand:expr) => {
        #[test]
        fn $test_name() {
            let tokens = create_tokens(vec![$token_kind, $operand, TokenKind::Eof]);
            let parser = JsavParser::new(tokens);
            assert_debug_snapshot!(stringify!($test_name), parser.parse());
        }
    };
}

// Test per letterali usando la macro
literal_test!(test_literal_number, TokenKind::Numeric(Number::Integer(42)));
literal_test!(test_literal_bool, TokenKind::KeywordBool(true));
literal_test!(test_literal_nullptr, TokenKind::KeywordNullptr);
literal_test!(
    test_literal_string,
    TokenKind::StringLiteral("assssss".to_string())
);
literal_test!(test_literal_char, TokenKind::CharLiteral("a".to_string()));

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
    TokenKind::Numeric(Number::Integer(5))
);
unary_op_test!(
    test_unary_not,
    TokenKind::Not,
    UnaryOp::Not,
    TokenKind::KeywordBool(true)
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
            assert_debug_snapshot!(stringify!($test_name), parser.parse());
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
        $test_name:ident,
        // the list of TokenKind-expressions to feed into `create_tokens(...)`
        [$($tok:expr),* $(,)?],
    ) => {
        #[test]
        fn $test_name() {
            // build the token vector
            let tokens = create_tokens(vec![$($tok),*]);
            let parser = JsavParser::new(tokens);
            assert_debug_snapshot!(stringify!($test_name),parser.parse());
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
);

assignment_test!(
    test_assignment_invalid_target,
    [
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Equal,
        TokenKind::Numeric(Number::Integer(10)),
        TokenKind::Eof,
    ],
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
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_unclosed_parenthesis() {
    let tokens = create_tokens(vec![
        TokenKind::OpenParen,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_unexpected_token() {
    let tokens = create_tokens(vec![
        TokenKind::Plus,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_empty_input() {
    let tokens = create_tokens(vec![TokenKind::Eof]);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_unary_operators_bp() {
    let token = Token {
        kind: TokenKind::Dot,
        span: dummy_span(),
    };
    assert_debug_snapshot!(unary_binding_power(&token));
}

// Test for line 128-132: Variable with Unicode identifier
#[test]
fn test_variable_unicode_identifier() {
    let tokens = create_tokens(vec![
        TokenKind::IdentifierUnicode("変数".into()),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
        assert_debug_snapshot!(result);
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_invalid_unary_operator() {
    let tokens = create_tokens(vec![
        TokenKind::Star,
        TokenKind::Numeric(Number::Integer(5)),
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_block_stmt() {
    let tokens = create_tokens(vec![
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::Eof,
    ]);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
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
    assert_debug_snapshot!(parser.parse());
}

/// Macro per generare automaticamente un test di dichiarazione di variabile.
///
/// Parametri:
/// - `$test_name`: identificatore del test (es. `test_b_i8_5`)
/// - `$input`: stringa di sorgente da passare al lexer (es. `"var b: i8 = 5"`)
/// - `$var_name`: nome della variabile come stringa (es. `"b"`)
/// - `$type`: tipo atteso, come variante di `Type` (es. `Type::I8`)
/// - `$lit_val`: valore letterale intero atteso (es. `5`)
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
    ) => {
        #[allow(clippy::approx_constant)]
        #[test]
        fn $test_name() {
            // 1) Tokenizzazione
            let input = $input;
            let mut lexer = Lexer::new("test.vn", &input);
            let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
            // 2) Parsing
            let parser = JsavParser::new(tokens);
            assert_debug_snapshot!(stringify!($test_name), parser.parse());
        }
    };
}

test_var_decl!(
    test_b_u8_5,
    "var b: u8 = 5u",                                 // input
    "b",                                              // var_name
    Type::U8,                                         // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
);

test_var_decl!(
    test_b_u16_5,
    "var b: u16 = 5u",                                // input
    "b",                                              // var_name
    Type::U16,                                        // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
);

test_var_decl!(
    test_b_u32_5,
    "var b: u32 = 5u",                                // input
    "b",                                              // var_name
    Type::U32,                                        // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
);

test_var_decl!(
    test_b_u64_5,
    "var b: u64 = 5u",                                // input
    "b",                                              // var_name
    Type::U64,                                        // tipo atteso
    LiteralValue::Number(Number::UnsignedInteger(5)), // valore letterale
);

test_var_decl!(
    test_b_i8_5,
    "var b: i8 = 5",                          // input
    "b",                                      // var_name
    Type::I8,                                 // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
);

test_var_decl!(
    test_b_i16_5,
    "var b: i16 = 5",                         // input
    "b",                                      // var_name
    Type::I16,                                // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
);

test_var_decl!(
    test_b_i32_5,
    "var b: i32 = 5",                         // input
    "b",                                      // var_name
    Type::I32,                                // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
);

test_var_decl!(
    test_b_i64_5,
    "var b: i64 = 5",                         // input
    "b",                                      // var_name
    Type::I64,                                // tipo atteso
    LiteralValue::Number(Number::Integer(5)), // valore letterale
);

test_var_decl!(
    test_char_decl,
    "var b: char = 'a'",                    // input
    "b",                                    // var_name
    Type::Char,                             // tipo atteso,
    LiteralValue::CharLit("a".to_string()), // valore letterale
);

test_var_decl!(
    test_custom_decl,
    "var b: string = \"a\"",                  // input
    "b",                                      // var_name
    Type::String,                             // tipo atteso,
    LiteralValue::StringLit("a".to_string()), // valore letterale
);

test_var_decl!(
    test_string_decl,
    "var b: custom = \"a\"",                  // input
    "b",                                      // var_name
    Type::Custom("custom".to_string()),       // tipo atteso,
    LiteralValue::StringLit("a".to_string()), // valore letterale
);

test_var_decl!(
    test_bool_decl,
    "var b: bool = true",     // input
    "b",                      // var_name
    Type::Bool,               // tipo atteso,
    LiteralValue::Bool(true), // valore letterale
);

test_var_decl!(
    test_b_f32_3_14,                             // nome del test
    "var b: f32 = 3.14f",                        // input
    "b",                                         // var_name
    Type::F32,                                   // tipo atteso
    LiteralValue::Number(Number::Float32(3.14)), // valore letterale (3.14 in f32)
);

test_var_decl!(
    test_b_f64_3_14,                             // nome del test
    "var b: f64 = 3.14",                         // input
    "b",                                         // var_name
    Type::F64,                                   // tipo atteso
    LiteralValue::Number(Number::Float64(3.14)), // valore letterale (3.14 in f32)
);

#[test]
fn array_declaration() {
    let input = "var arr: i8[5] = {1, 2, 3, 4, 5}";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn vector_declaration() {
    let input = "var arr: vector<i8> = {1, 2, 3, 4, 5}";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_function_inputs() {
    let input = "fun a(num1: i8, num2: i8): i8 { }";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
}

#[test]
fn test_main() {
    let input = "main { }";
    let mut lexer = Lexer::new("test.vn", &input);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    assert_debug_snapshot!(parser.parse());
}
