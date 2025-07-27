use insta::assert_debug_snapshot;
use jsavrs::error::compile_error::CompileError;
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
use jsavrs::parser::ast::{Expr, Type};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::semantic::type_checker::TypeChecker;
use jsavrs::tokens::number::Number;
use jsavrs::utils::dummy_span;

// Test helper
fn typecheck(ast: &str) -> Vec<CompileError> {
    let mut lexer = Lexer::new("test.vn", &ast);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, _errors) = parser.parse();
    let mut checker = TypeChecker::new();
    checker.check(&*expr)
}

fn typecheckd(ast: &str) -> Vec<CompileError> {
    let mut lexer = Lexer::new("test.vn", &ast);
    let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
    let parser = JsavParser::new(tokens);
    let (expr, _errors) = parser.parse();
    let mut checker = TypeChecker::default();
    checker.check(&*expr)
}

#[test]
fn test_var_declaration_in_main() {
    let input = "main { var x: i32 = 42i32 }";
    let errors = typecheck(input);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_var_declaration_in_main_using_typecheck_default() {
    let input = "main { var x: i32 = 42i32 }";
    let errors = typecheckd(input);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_variable_declaration_valid() {
    let input = "var x: i32 = 42i32";
    let errors = typecheck(input);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_variable_declaration_in_block_valid() {
    let ast = "{ var x: i32 = 42i32 }";
    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_variable_declaration_type_mismatch() {
    let ast = "var x: i32 = \"test\"";
    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_function_call_valid() {
    let ast = "fun add(num1: i32, num2: i32): i32 {
}
add(1i32, 2i32)";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_function_call_not_using_variable() {
    let ast = "num[0]()";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_function_call_argument_mismatch() {
    let ast = "fun add(a: i32, b: i32): i32 {
}
add(1i32, \"two\")";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_return_type_mismatch() {
    let ast = "fun add(num1: i32, num2: i32): i32 {
    return true
}";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_return_type_void() {
    let ast = "fun add(num1: i32, num2: i32): i32 {
    return
}";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_array_operations_valid() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr[0]";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_empty_array_literal() {
    let ast = "var arr: i32[2] = {}
    arr[0]";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_mismatched_types_in_array_literal() {
    let ast = "var arr: i32[2] = {1i32,'s'}
    arr[0]";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_array_invalid_index_access() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr['a']";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_numeric_promotion() {
    let ast = "42i32 + 3.14f64";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_break_outside_loop() {
    let ast = "break";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_continue_outside_loop() {
    let ast = "continue";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_undefined_variable() {
    let ast = "undefined";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_assign_to_undefined_variable() {
    let ast = "undefined = 43i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_immutable_assignment() {
    let ast = "const x: i32 = 42i32
    x = 43i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_assign_f64_to_i32() {
    let ast = "var x: i32 = 42i32
    x = 3.222";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_indexing_a_non_array_type() {
    let ast = "var x: i32 = 42i32
    x[0]";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_main_function_signature() {
    let ast = "main {}";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

/*
#[test]
fn test_double_main_function_signature() {
    let ast = vec![
        Stmt::MainFunction {
            body: vec![],
            span: dummy_span(),
        },
        Stmt::MainFunction {
            body: vec![],
            span: dummy_span(),
        },
    ];

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}*/

#[test]
fn test_binary_arithmetic_valid() {
    let ast = "10i32 + 20i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_binary_arithmetic_in_grouping_valid() {
    let ast = "(10i32 + 20i32)";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_binary_arithmetic_invalid() {
    let ast = "true + 20i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_binary_comparison_valid() {
    let ast = "10i32 < 20i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_binary_comparison_invalid() {
    let ast = "true < \"test\"";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_logical_operations_valid() {
    let ast = "true && false";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_logical_operations_invalid() {
    let ast = "1i32 || false";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_bitwise_operations_valid() {
    let ast = "10i32 & 20i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_bitwise_operations_invalid() {
    let ast = "true | 20i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_unary_negate_valid() {
    let ast = "-10i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_unary_negate_invalid() {
    let ast = "-true";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_unary_not_valid() {
    let ast = "!true";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_unary_not_invalid() {
    let ast = "!0i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_if() {
    let ast = "if (true) {
        42i32
    }";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_if_invalid_condition() {
    let ast = "if (32) {
        42i32
    }";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_if_else() {
    let ast = "if (true) {
        42i32
    } else { }";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_return_outside_of_function() {
    let ast = "return 42i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_function_arguments_numbers_mismatch() {
    let ast = "fun add(a: i32, b: i32): i32 {
}
add(2i32, 3i32, 4i32)";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}
/*#[test]
fn test_invalid_assignment_target() {
    let ast = "fun add(a: i32, b: i32): i32 {
}
add(2i32, 3i32, 4i32) = 43i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Invalid assignment target"));
}*/

#[test]
fn test_assign_wrong_type_to_array_access() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr[0] = 3.12";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}
#[test]
fn test_assign_to_array_access_whit_nullptr_index() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr[nullptr] = 33i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}
#[test]
fn test_assign_to_a_non_array() {
    let ast = "var arr: i32 = 2i32
    arr[2] = 33i32";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_non_function_variable_call() {
    let ast = "var x: i32 = 42i32
    x()";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_undefined_function_call() {
    let ast = "undefined_function(1i32, 2i32)";

    let errors = typecheck(ast);
    assert_debug_snapshot!(errors);
}

#[test]
fn test_type_of_number_integer_variants() {
    let tc = TypeChecker::new();
    assert_debug_snapshot!("type_of_number_i8", tc.type_of_number(&Number::I8(0)));
    assert_debug_snapshot!("type_of_number_i16", tc.type_of_number(&Number::I16(0)));
    assert_debug_snapshot!("type_of_number_i32", tc.type_of_number(&Number::I32(0)));
    assert_debug_snapshot!(
        "type_of_number_integer",
        tc.type_of_number(&Number::Integer(42))
    );
    assert_debug_snapshot!("type_of_number_u8", tc.type_of_number(&Number::U8(0)));
    assert_debug_snapshot!("type_of_number_u16", tc.type_of_number(&Number::U16(0)));
    assert_debug_snapshot!("type_of_number_u32", tc.type_of_number(&Number::U32(0)));
    assert_debug_snapshot!(
        "type_of_number_unsigned_integer",
        tc.type_of_number(&Number::UnsignedInteger(42))
    );
}

#[test]
fn test_type_of_number_float_variants() {
    let tc = TypeChecker::new();
    assert_debug_snapshot!(
        "type_of_number_float32",
        tc.type_of_number(&Number::Float32(3.14))
    );
    assert_debug_snapshot!(
        "type_of_number_scientific32",
        tc.type_of_number(&Number::Scientific32(1.0e2, 2))
    );
    assert_debug_snapshot!(
        "type_of_number_float64",
        tc.type_of_number(&Number::Float64(2.71838))
    );
    assert_debug_snapshot!(
        "type_of_number_scientific64",
        tc.type_of_number(&Number::Scientific64(1.0e2, 2))
    );
}

#[test]
fn test_is_assignable_exact_and_promotions() {
    let tc = TypeChecker::new();

    // Exact matches
    assert_debug_snapshot!("exact_match_i32", tc.is_assignable(&Type::I32, &Type::I32));
    assert_debug_snapshot!("exact_match_f64", tc.is_assignable(&Type::F64, &Type::F64));

    // Signed promotions
    assert_debug_snapshot!(
        "signed_promotion_i8_to_i16",
        tc.is_assignable(&Type::I8, &Type::I16)
    );
    assert_debug_snapshot!(
        "signed_promotion_i8_to_f32",
        tc.is_assignable(&Type::I8, &Type::F32)
    );
    assert_debug_snapshot!(
        "signed_promotion_i16_to_f64",
        tc.is_assignable(&Type::I16, &Type::F64)
    );
    assert_debug_snapshot!(
        "signed_promotion_i32_to_i64",
        tc.is_assignable(&Type::I32, &Type::I64)
    );

    // Unsigned promotions
    assert_debug_snapshot!(
        "unsigned_promotion_u8_to_u16",
        tc.is_assignable(&Type::U8, &Type::U16)
    );
    assert_debug_snapshot!(
        "unsigned_promotion_u8_to_f64",
        tc.is_assignable(&Type::U8, &Type::F64)
    );
    assert_debug_snapshot!(
        "unsigned_promotion_u32_to_u64",
        tc.is_assignable(&Type::U32, &Type::U64)
    );
    // Additional U16 promotions
    assert_debug_snapshot!(
        "unsigned_promotion_u16_to_u32",
        tc.is_assignable(&Type::U16, &Type::U32)
    );
    assert_debug_snapshot!(
        "unsigned_promotion_u16_to_u64",
        tc.is_assignable(&Type::U16, &Type::U64)
    );
    assert_debug_snapshot!(
        "unsigned_promotion_u16_to_f32",
        tc.is_assignable(&Type::U16, &Type::F32)
    );
    assert_debug_snapshot!(
        "unsigned_promotion_u16_to_f64",
        tc.is_assignable(&Type::U16, &Type::F64)
    );

    // Float promotions
    assert_debug_snapshot!(
        "float_promotion_f32_to_f64",
        tc.is_assignable(&Type::F32, &Type::F64)
    );

    // Incompatible types
    assert_debug_snapshot!(
        "incompatible_i8_to_u8",
        !tc.is_assignable(&Type::I8, &Type::U8)
    );
    assert_debug_snapshot!(
        "incompatible_f64_to_f32",
        !tc.is_assignable(&Type::F64, &Type::F32)
    );
    assert_debug_snapshot!(
        "incompatible_u16_to_i32",
        !tc.is_assignable(&Type::U16, &Type::I32)
    );
}

#[test]
fn test_is_assignable_nullptr() {
    let tc = TypeChecker::new();
    let array_ty = Type::Array(Box::new(Type::I32), Box::new(Expr::null_expr(dummy_span())));
    let vector_ty = Type::Vector(Box::new(Type::I8));

    // NullPtr assignable to Array and Vector
    assert_debug_snapshot!(
        "nullptr_to_array",
        tc.is_assignable(&Type::NullPtr, &array_ty)
    );
    assert_debug_snapshot!(
        "nullptr_to_vector",
        tc.is_assignable(&Type::NullPtr, &vector_ty)
    );

    // NullPtr not assignable to non-pointer
    assert_debug_snapshot!(
        "nullptr_to_i32_incompatible",
        !tc.is_assignable(&Type::NullPtr, &Type::I32)
    );
}

#[test]
fn test_promote_numeric_types_behaviour() {
    let tc = TypeChecker::new();

    // Lower-rank gets promoted to higher-rank
    assert_debug_snapshot!(
        "promote_i8_to_i16",
        tc.promote_numeric_types(&Type::I8, &Type::I16)
    );
    assert_debug_snapshot!(
        "promote_u8_to_f32",
        tc.promote_numeric_types(&Type::U8, &Type::F32)
    );
    assert_debug_snapshot!(
        "promote_i32_to_f64",
        tc.promote_numeric_types(&Type::I32, &Type::F64)
    );
    assert_debug_snapshot!(
        "promote_u32_to_u64",
        tc.promote_numeric_types(&Type::U32, &Type::U64)
    );

    // Symmetric behaviour
    assert_debug_snapshot!(
        "symmetric_promotion_f32_and_u8",
        tc.promote_numeric_types(&Type::F32, &Type::U8)
    );

    // If neither type matches hierarchy, fallback to I64
    assert_debug_snapshot!(
        "fallback_promotion_bool_and_string",
        tc.promote_numeric_types(&Type::Bool, &Type::String),
    );
}
