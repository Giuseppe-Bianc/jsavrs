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
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_var_declaration_in_main_using_typecheck_default() {
    let input = "main { var x: i32 = 42i32 }";
    let errors = typecheckd(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_valid() {
    let input = "var x: i32 = 42i32";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_in_block_valid() {
    let ast = "{ var x: i32 = 42i32 }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_variable_declaration_type_mismatch() {
    let ast = "var x: i32 = \"test\"";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Cannot assign string to i32 for variable 'x'")
    );
}

#[test]
fn test_function_call_valid() {
    let ast = "fun add(num1: i32, num2: i32): i32 {
}
add(1i32, 2i32)";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_call_not_using_variable() {
    let ast = "num[0]()";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Callee must be a function name"));
}

#[test]
fn test_function_call_argument_mismatch() {
    let ast = "fun add(a: i32, b: i32): i32 {
}
add(1i32, \"two\")";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Argument 2: expected i32, found string")
    );
}

#[test]
fn test_return_type_mismatch() {
    let ast = "fun add(num1: i32, num2: i32): i32 {
    return true
}";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Return type mismatch: expected i32, found bool")
    );
}

#[test]
fn test_return_type_void() {
    let ast = "fun add(num1: i32, num2: i32): i32 {
    return
}";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Function requires return type i32, found void")
    );
}

#[test]
fn test_array_operations_valid() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr[0]";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_empty_array_literal() {
    let ast = "var arr: i32[2] = {}
    arr[0]";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array literal must have at least one element")
    );
}

#[test]
fn test_mismatched_types_in_array_literal() {
    let ast = "var arr: i32[2] = {1i32,'s'}
    arr[0]";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array elements must be of the same type, found i32 and char")
    );
}

#[test]
fn test_array_invalid_index_access() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr['a']";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array index must be integer, found char")
    );
}

#[test]
fn test_numeric_promotion() {
    let ast = "42i32 + 3.14f64";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_break_outside_loop() {
    let ast = "break";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Break/continue outside loop"));
}

#[test]
fn test_continue_outside_loop() {
    let ast = "continue";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Break/continue outside loop"));
}

#[test]
fn test_undefined_variable() {
    let ast = "undefined";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Undefined variable 'undefined'"));
}

#[test]
fn test_assign_to_undefined_variable() {
    let ast = "undefined = 43i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Undefined variable 'undefined'"));
}

#[test]
fn test_immutable_assignment() {
    let ast = "const x: i32 = 42i32
    x = 43i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Assignment to immutable variable 'x'")
    );
}

#[test]
fn test_assign_f64_to_i32() {
    let ast = "var x: i32 = 42i32
    x = 3.222";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Cannot assign f64 to i32"));
}

#[test]
fn test_indexing_a_non_array_type() {
    let ast = "var x: i32 = 42i32
    x[0]";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Indexing non-array type i32"));
}

#[test]
fn test_main_function_signature() {
    let ast = "main {}";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
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
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}*/

#[test]
fn test_binary_arithmetic_valid() {
    let ast = "10i32 + 20i32";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_arithmetic_in_grouping_valid() {
    let ast = "(10i32 + 20i32)";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_arithmetic_invalid() {
    let ast = "true + 20i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Binary operator 'Add' requires numeric operands, found bool and i32")
    );
}

#[test]
fn test_binary_comparison_valid() {
    let ast = "10i32 < 20i32";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_binary_comparison_invalid() {
    let ast = "true < \"test\"";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Comparison operator 'Less' requires compatible types, found bool and string")
    );
}

#[test]
fn test_logical_operations_valid() {
    let ast = "true && false";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_logical_operations_invalid() {
    let ast = "1i32 || false";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Logical operator 'Or' requires boolean operands, found i32 and bool")
    );
}

#[test]
fn test_bitwise_operations_valid() {
    let ast = "10i32 & 20i32";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_operations_invalid() {
    let ast = "true | 20i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Bitwise operator 'BitwiseOr' requires integer operands, found bool and i32")
    );
}

#[test]
fn test_unary_negate_valid() {
    let ast = "-10i32";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_unary_negate_invalid() {
    let ast = "-true";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Negation requires numeric operand, found bool")
    );
}

#[test]
fn test_unary_not_valid() {
    let ast = "!true";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_unary_not_invalid() {
    let ast = "!0i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Logical not requires boolean operand, found i32")
    );
}

#[test]
fn test_if() {
    let ast = "if (true) {
        42i32
    }";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_if_invalid_condition() {
    let ast = "if (32) {
        42i32
    }";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("If condition must be bool, found i64")
    );
}

#[test]
fn test_if_else() {
    let ast = "if (true) {
        42i32
    } else { }";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_return_outside_of_function() {
    let ast = "return 42i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Return statement outside function")
    );
}

#[test]
fn test_function_arguments_numbers_mismatch() {
    let ast = "fun add(a: i32, b: i32): i32 {
}
add(2i32, 3i32, 4i32)";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Function 'add' expects 2 arguments, found 3")
    );
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
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Cannot assign f64 to array element of type i32")
    );
}
#[test]
fn test_assign_to_array_access_whit_nullptr_index() {
    let ast = "var arr: i32[2] = {1i32,2i32}
    arr[nullptr] = 33i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Array index must be integer, found nullptr")
    );
}
#[test]
fn test_assign_to_a_non_array() {
    let ast = "var arr: i32 = 2i32
    arr[2] = 33i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Indexing non-array type i32"));
}

#[test]
fn test_non_function_variable_call() {
    let ast = "var x: i32 = 42i32
    x()";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("'x' is not a function"));
}

#[test]
fn test_undefined_function_call() {
    let ast = "undefined_function(1i32, 2i32)";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Undefined function 'undefined_function'")
    );
}

#[test]
fn test_type_of_number_integer_variants() {
    let tc = TypeChecker::new();
    // Signed ints
    assert_eq!(tc.type_of_number(&Number::I8(0)), Type::I8);
    assert_eq!(tc.type_of_number(&Number::I16(0)), Type::I16);
    assert_eq!(tc.type_of_number(&Number::I32(0)), Type::I32);
    assert_eq!(tc.type_of_number(&Number::Integer(42)), Type::I64);

    // Unsigned ints
    assert_eq!(tc.type_of_number(&Number::U8(0)), Type::U8);
    assert_eq!(tc.type_of_number(&Number::U16(0)), Type::U16);
    assert_eq!(tc.type_of_number(&Number::U32(0)), Type::U32);
    assert_eq!(tc.type_of_number(&Number::UnsignedInteger(42)), Type::U64);
}

#[test]
fn test_type_of_number_float_variants() {
    let tc = TypeChecker::new();
    // 32-bit float
    assert_eq!(tc.type_of_number(&Number::Float32(3.14)), Type::F32);
    assert_eq!(
        tc.type_of_number(&Number::Scientific32(1.0e2, 2)),
        Type::F32
    );

    // 64-bit float
    assert_eq!(tc.type_of_number(&Number::Float64(2.71838)), Type::F64);
    assert_eq!(
        tc.type_of_number(&Number::Scientific64(1.0e2, 2)),
        Type::F64
    );
}

#[test]
fn test_is_assignable_exact_and_promotions() {
    let tc = TypeChecker::new();

    // Exact matches
    assert!(tc.is_assignable(&Type::I32, &Type::I32));
    assert!(tc.is_assignable(&Type::F64, &Type::F64));

    // Signed promotions
    assert!(tc.is_assignable(&Type::I8, &Type::I16));
    assert!(tc.is_assignable(&Type::I8, &Type::F32));
    assert!(tc.is_assignable(&Type::I16, &Type::F64));
    assert!(tc.is_assignable(&Type::I32, &Type::I64));

    // Unsigned promotions
    assert!(tc.is_assignable(&Type::U8, &Type::U16));
    assert!(tc.is_assignable(&Type::U8, &Type::F64));
    assert!(tc.is_assignable(&Type::U32, &Type::U64));
    // Additional U16 promotions
    assert!(tc.is_assignable(&Type::U16, &Type::U32));
    assert!(tc.is_assignable(&Type::U16, &Type::U64));
    assert!(tc.is_assignable(&Type::U16, &Type::F32));
    assert!(tc.is_assignable(&Type::U16, &Type::F64));

    // Float promotions
    assert!(tc.is_assignable(&Type::F32, &Type::F64));

    // Incompatible types
    assert!(!tc.is_assignable(&Type::I8, &Type::U8));
    assert!(!tc.is_assignable(&Type::F64, &Type::F32));
    assert!(!tc.is_assignable(&Type::U16, &Type::I32));
}

#[test]
fn test_is_assignable_nullptr() {
    let tc = TypeChecker::new();

    // NullPtr assignable to Array and Vector
    let array_ty = Type::Array(Box::new(Type::I32), Box::new(Expr::null_expr(dummy_span())));
    let vector_ty = Type::Vector(Box::new(Type::I8));
    assert!(tc.is_assignable(&Type::NullPtr, &array_ty));
    assert!(tc.is_assignable(&Type::NullPtr, &vector_ty));

    // NullPtr not assignable to non-pointer
    assert!(!tc.is_assignable(&Type::NullPtr, &Type::I32));
}

#[test]
fn test_promote_numeric_types_behaviour() {
    let tc = TypeChecker::new();

    // Lower-rank gets promoted to higher-rank
    assert_eq!(tc.promote_numeric_types(&Type::I8, &Type::I16), Type::I16);
    assert_eq!(tc.promote_numeric_types(&Type::U8, &Type::F32), Type::F32);
    assert_eq!(tc.promote_numeric_types(&Type::I32, &Type::F64), Type::F64);
    assert_eq!(tc.promote_numeric_types(&Type::U32, &Type::U64), Type::U64);

    // Symmetric behaviour
    assert_eq!(tc.promote_numeric_types(&Type::F32, &Type::U8), Type::F32);

    // If neither type matches hierarchy, fallback to I64
    assert_eq!(
        tc.promote_numeric_types(&Type::Bool, &Type::String),
        Type::I64
    );
}
