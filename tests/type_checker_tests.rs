use jsavrs::error::compile_error::CompileError;
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
use jsavrs::parser::ast::{Expr, LiteralValue, Type};
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
fn test_var_declaration_mismatched_num_of_inic() {
    let input = "var x: i32, y:f64 = 42i32";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 2);
    assert_eq!(
        errors[0].message(),
        Some("Variable declaration requires 1 initializers but 0 were provided")
    );
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
    return num1 + num2
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
fn test_function_call_not_using_variable_arguments() {
    let ast = "num[0](1i8, 2i32)";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Callee must be a function name"));
}

#[test]
fn test_function_call_argument_mismatch() {
    let ast = "fun add(a: i32, b: i32): i32 {
    return a + b
}
add(1i32, \"two\")";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Argument 2 type mismatch: expected i32, found string")
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
        Some("Return type mismatch: expected i32 found bool")
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
        Some("Return type mismatch, expected i32 found Void")
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
        Some("Array literals must have at least one element for type inference")
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
        Some("All array elements must be same type, found mixed types: i32 and char")
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
        Some("Array index must be integer type, found char")
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
    assert_eq!(errors[0].message(), Some("Break statement outside loop"));
}

#[test]
fn test_continue_outside_loop() {
    let ast = "continue";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Continue statement outside loop"));
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
        Some("Cannot assign to immutable variable 'x'")
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
    assert_eq!(errors[0].message(), Some("Cannot index into non-array type i32"));
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
        Some("Logical operator 'Or' requires boolean operands types, found i32 and bool")
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
        Some("Bitwise operator 'BitwiseOr' require integer operand types, found bool and i32")
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
        Some("Negation requires numeric type operand, found bool")
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
        Some("Logical not requires boolean type operand, found i32")
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
        Some("Condition in 'if' statement must be boolean, found i64")
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
        Some("Return statement must be inside function body")
    );
}

#[test]
fn test_function_arguments_numbers_mismatch() {
    let ast = "fun add(a: i32, b: i32): i32 {
    return a + b
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
        Some("Array index must be integer type, found nullptr")
    );
}

#[test]
fn test_assign_to_a_non_array() {
    let ast = "var arr: i32 = 2i32
    arr[2] = 33i32";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Cannot index into non-array type i32"));
}

#[test]
fn test_non_function_variable_call() {
    let ast = "var x: i32 = 42i32
    x()";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), Some("Undefined function: 'x'"));
}

#[test]
fn test_undefined_function_call() {
    let ast = "undefined_function(1i32, 2i32)";

    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Undefined function: 'undefined_function'")
    );
}

#[test]
fn test_while_loop_valid() {
    let ast = "while (true) { 42i32 }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_while_loop_invalid_condition() {
    let ast = "while (42i32) { }";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Condition in 'while' loop must be boolean, found i32")
    );
}

#[test]
fn test_break_inside_while() {
    let ast = "while (true) { break }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_continue_inside_while() {
    let ast = "while (true) { continue }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_for_loop_valid() {
    let ast = "for (var i: i32 = 0i32; i < 10i32; i = i + 1i32) { }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_for_loop_invalid_condition() {
    let ast = "for (var i: i32 = 0i32; 42i32; i = i + 1i32) { }";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("For loop condition must be bool, found i32")
    );
}

#[test]
fn test_break_inside_for() {
    let ast = "for (;;) { break }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_continue_inside_for() {
    let ast = "for (;;) { continue }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}


#[test]
fn test_nested_loops_with_break_continue() {
    let ast = "
    while (true) {
        for (var i: i32 = 0i32; i < 10i32; i = i + 1i32) {
            if (i == 5i32) {
                break
            } else {
                continue
            }
        }
        continue
    }";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
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
    assert!(tc.is_assignable(&Type::String, &Type::String));

    // Signed promotions
    assert!(tc.is_assignable(&Type::I8, &Type::I16));
    assert!(tc.is_assignable(&Type::I8, &Type::F32));
    assert!(tc.is_assignable(&Type::I16, &Type::F64));
    assert!(tc.is_assignable(&Type::I32, &Type::I64));
    // Additional signed promotions
    assert!(tc.is_assignable(&Type::I16, &Type::I32));
    assert!(tc.is_assignable(&Type::I64, &Type::F64)); // Nuovo test

    // Unsigned promotions
    assert!(tc.is_assignable(&Type::U8, &Type::U16));
    assert!(tc.is_assignable(&Type::U8, &Type::F64));
    assert!(tc.is_assignable(&Type::U32, &Type::U64));
    // Additional U16 promotions
    assert!(tc.is_assignable(&Type::U16, &Type::U32));
    assert!(tc.is_assignable(&Type::U16, &Type::U64));
    assert!(tc.is_assignable(&Type::U16, &Type::F32));
    assert!(tc.is_assignable(&Type::U16, &Type::F64));
    // Additional unsigned promotions
    assert!(tc.is_assignable(&Type::U32, &Type::F32));
    assert!(tc.is_assignable(&Type::U64, &Type::F64)); // Nuovo test

    // Float promotions
    assert!(tc.is_assignable(&Type::F32, &Type::F64));

    // Char to String promotion
    assert!(tc.is_assignable(&Type::Char, &Type::String));

    // Incompatible types
    assert!(!tc.is_assignable(&Type::I8, &Type::U8));
    assert!(!tc.is_assignable(&Type::F64, &Type::F32));
    assert!(!tc.is_assignable(&Type::U16, &Type::I32));
    assert!(!tc.is_assignable(&Type::Char, &Type::I32));
    assert!(!tc.is_assignable(&Type::I64, &Type::F32)); // Nuovo test negativo
    assert!(!tc.is_assignable(&Type::U64, &Type::F32)); // Nuovo test negativo
}

#[test]
fn test_is_assignable_nullptr() {
    let tc = TypeChecker::new();
    let span = dummy_span();

    // NullPtr assignable to Array and Vector
    let array_ty = Type::Array(Box::new(Type::I32), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(0)),
        span: span.clone()
    }));
    let vector_ty = Type::Vector(Box::new(Type::I8));
    assert!(tc.is_assignable(&Type::NullPtr, &array_ty));
    assert!(tc.is_assignable(&Type::NullPtr, &vector_ty));

    // NullPtr not assignable to non-pointer
    assert!(!tc.is_assignable(&Type::NullPtr, &Type::I32));
    assert!(!tc.is_assignable(&Type::NullPtr, &Type::String));
}

#[test]
fn test_is_assignable_arrays_and_vectors() {
    let tc = TypeChecker::new();
    let span = dummy_span();

    // Array tests
    let array_i32_5 = Type::Array(Box::new(Type::I32), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: span.clone()
    }));
    let array_i32_5_again = Type::Array(Box::new(Type::I32), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: span.clone()
    }));
    let array_i32_10 = Type::Array(Box::new(Type::I32), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(10)),
        span: span.clone()
    }));
    let array_i8_5 = Type::Array(Box::new(Type::I8), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: span.clone()
    }));
    let array_i16_5 = Type::Array(Box::new(Type::I16), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: span.clone()
    }));
    let array_u8_5 = Type::Array(Box::new(Type::U8), Box::new(Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: span.clone()
    }));

    // Same array type and size -> allowed
    assert!(tc.is_assignable(&array_i32_5, &array_i32_5_again));

    // Same element type, different size -> disallowed
    assert!(!tc.is_assignable(&array_i32_5, &array_i32_10));

    // Assignable element types and same size -> allowed
    assert!(tc.is_assignable(&array_i8_5, &array_i16_5));

    // Non-assignable element types -> disallowed
    assert!(!tc.is_assignable(&array_i8_5, &array_u8_5));

    // Vector tests
    let vector_i8 = Type::Vector(Box::new(Type::I8));
    let vector_i16 = Type::Vector(Box::new(Type::I16));
    let vector_u8 = Type::Vector(Box::new(Type::U8));

    // Same vector type -> allowed
    assert!(tc.is_assignable(&vector_i8, &vector_i8));

    // Assignable element types -> allowed
    assert!(tc.is_assignable(&vector_i8, &vector_i16));

    // Non-assignable element types -> disallowed
    assert!(!tc.is_assignable(&vector_i8, &vector_u8));
}

#[test]
fn test_void_function_return_value() {
    // Void function returning value should error
    let input = "fun log() { return 42i32 }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Cannot return a value from void function")
    );
}

#[test]
fn test_return_in_void_function_without_value() {
    let ast = "fun void_fn() {
        return
    }";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_return_in_void_function_in_inf() {
    let ast = "fun void_fn() {
        if (true) {
            return
        }
    }";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_return_in_void_function_in_else() {
    let ast = "fun void_fn() {
        if (false) {
        } else {
            return
        }
    }";

    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_simple_return() {
    let input = "fun test(): i32 { return 42i32 }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_no_return() {
    let input = "fun test(): i32 { }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Function 'test' may not return value in all code paths (expected return type: i32)")
    );
}

#[test]
fn test_function_has_return_if_true_branch() {
    let input = "
    fun test(cond: bool): i32 {
        if (cond) {
            return 42i32
        }
    }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("may not return value"));
}

#[test]
fn test_function_has_return_if_both_branches() {
    let input = "
    fun test(cond: bool): i32 {
        if (cond) {
            return 42i32
        } else {
            return 24i32
        }
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_after_if() {
    let input = "
    fun test(cond: bool): i32 {
        if (cond) {
            // no return
        }
        return 42i32
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_nested_if() {
    let input = "
    fun test(cond1: bool, cond2: bool): i32 {
        if (cond1) {
            if (cond2) {
                return 42i32
            } else {
                return 24i32
            }
        } else {
            return 0i32
        }
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_block_with_return() {
    let input = "
    fun test(): i32 {
        {
            return 42i32
        }
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_block_without_return() {
    let input = "
    fun test(): i32 {
        {
            // no return
        }
    }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("may not return value"));
}

/*#[test]
fn test_function_has_return_loop_with_return() {
    let input = "
    fun test(): i32 {
        while (true) {
            return 42i32
        }
        return 24i32
    }";
    let errors = typecheck(input);
    // Non è considerato safe perché il loop potrebbe non essere eseguito
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("may not return value"));
}*/

#[test]
fn test_function_has_return_loop_with_return_and_after() {
    let input = "
    fun test(): i32 {
        while (true) {
            if (false) {
                return 42i32
            }
        }
        return 24i32
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:#?}", errors);
}

#[test]
fn test_function_has_return_complex_nested() {
    let input = "
    fun test(a: bool, b: bool): i32 {
        if (a) {
            if (b) {
                return 1i32
            } else {
                return 2i32
            }
        } else {
            for (var i: i32 = 0i32; i < 10i32; i = i + 1i32) {
                if (i == 5i32) {
                    return 3i32
                }
            }
            return 4i32
        }
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_else_if() {
    let input = "
    fun test(cond: bool): i32 {
        if (cond) {
            return 1i32
        } else if (!cond) {
            return 2i32
        }
        // Manca return nel caso finale
    }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("may not return value"));
}

#[test]
fn test_function_has_return_else_if_complete() {
    let input = "
    fun test(cond: bool): i32 {
        if (cond) {
            return 1i32
        } else if (!cond) {
            return 2i32
        } else {
            return 3i32
        }
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_multiple_paths() {
    let input = "
    fun test(a: bool, b: bool): i32 {
        if (a) {
            return 1i32
        }
        if (b) {
            return 2i32
        }
        // Manca return se entrambe false
    }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("may not return value"));
}

#[test]
fn test_function_has_return_void_function() {
    let input = "fun test() { }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_void_with_return() {
    let input = "fun test() { return }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_function_has_return_void_with_value() {
    let input = "fun test() { return 42i32 }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Cannot return a value from void function")
    );
}

#[test]
fn test_function_has_return_deeply_nested() {
    let input = "
    fun test(): i32 {
        {
            {
                if (true) {
                    {
                        return 42i32
                    }
                }
            }
        }
        return 0i32
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:#?}", errors);
}

#[test]
fn test_function_has_return_in_infinite_loop() {
    let input = "
    fun test(): i32 {
        while (true) {
            // No return - dovrebbe comunque essere errore
        }
    }";
    let errors = typecheck(input);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message().unwrap().contains("may not return value"));
}

#[test]
fn test_function_has_return_in_infinite_loop_with_return() {
    let input = "
    fun test(): i32 {
        while (true) {
            return 42i32
        }
    }";
    let errors = typecheck(input);
    // Considerato safe perché l'loop è infinito
    assert!(errors.is_empty(), "Unexpected errors: {:#?}", errors);
}

#[test]
fn test_function_has_return_with_break() {
    let input = "
    fun test(): i32 {
        while (true) {
            break
        }
        return 42i32
    }";
    let errors = typecheck(input);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_and_valid() {
    let ast = "10i32 & 20i32";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_or_valid() {
    let ast = "10i32 | 20i32";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_xor_valid() {
    let ast = "10i32 ^ 20i32";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_shift_left_valid() {
    let ast = "10i32 << 2i32";
    let errors = typecheck(ast);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn test_bitwise_and_invalid() {
    let ast = "true & 20i32";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Bitwise operator 'BitwiseAnd' require integer operand types, found bool and i32")
    );
}

#[test]
fn test_bitwise_xor_invalid() {
    let ast = "10i32 ^ \"string\"";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Bitwise operator 'BitwiseXor' require integer operand types, found i32 and string")
    );
}

#[test]
fn test_shift_left_invalid() {
    let ast = "10i32 << true";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Bitwise operator 'ShiftLeft' require integer operand types, found i32 and bool")
    );
}

#[test]
fn test_shift_right_invalid() {
    let ast = "\"text\" >> 2i32";
    let errors = typecheck(ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message(),
        Some("Bitwise operator 'ShiftRight' require integer operand types, found string and i32")
    );
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
    assert_eq!(tc.promote_numeric_types(&Type::I16, &Type::F32), Type::F32);
    assert_eq!(tc.promote_numeric_types(&Type::F64, &Type::I32), Type::F64);
    assert_eq!(tc.promote_numeric_types(&Type::U64, &Type::U32), Type::U64);

    // Same type should remain unchanged
    assert_eq!(tc.promote_numeric_types(&Type::I32, &Type::I32), Type::I32);
    assert_eq!(tc.promote_numeric_types(&Type::F32, &Type::F32), Type::F32);
    assert_eq!(tc.promote_numeric_types(&Type::U16, &Type::U16), Type::U16);

    // Mixed integer/float promotions
    assert_eq!(tc.promote_numeric_types(&Type::I16, &Type::F64), Type::F64);
    assert_eq!(tc.promote_numeric_types(&Type::U32, &Type::F32), Type::F32);
    assert_eq!(tc.promote_numeric_types(&Type::F64, &Type::I8), Type::F64);
    assert_eq!(tc.promote_numeric_types(&Type::F32, &Type::U64), Type::F32);

    // Unsigned/signed promotions
    assert_eq!(tc.promote_numeric_types(&Type::U8, &Type::I16), Type::I16);
    assert_eq!(tc.promote_numeric_types(&Type::I16, &Type::U32), Type::U32);
    assert_eq!(tc.promote_numeric_types(&Type::U32, &Type::I64), Type::I64);
    assert_eq!(tc.promote_numeric_types(&Type::I8, &Type::U64), Type::U64);

    // Non-numeric types fallback to first type
    assert_eq!(
        tc.promote_numeric_types(&Type::Bool, &Type::String),
        Type::Bool
    );
    assert_eq!(
        tc.promote_numeric_types(&Type::String, &Type::Bool),
        Type::String
    );
    assert_eq!(
        tc.promote_numeric_types(&Type::Char, &Type::Bool),
        Type::Char
    );

    // Mixed numeric and non-numeric
    assert_eq!(
        tc.promote_numeric_types(&Type::I32, &Type::String),
        Type::I32
    );
    assert_eq!(
        tc.promote_numeric_types(&Type::Bool, &Type::F64),
        Type::F64
    );
}

#[test]
fn test_is_same_type_primitive() {
    let checker = TypeChecker::new();

    // Identical primitive types
    assert!(checker.is_same_type(&Type::I32, &Type::I32));
    assert!(checker.is_same_type(&Type::F64, &Type::F64));
    assert!(checker.is_same_type(&Type::Bool, &Type::Bool));

    // Different primitive types
    assert!(!checker.is_same_type(&Type::I32, &Type::I64));
    assert!(!checker.is_same_type(&Type::F32, &Type::F64));
    assert!(!checker.is_same_type(&Type::Bool, &Type::String));
}

#[test]
fn test_is_same_type_array() {
    let checker = TypeChecker::new();

    // Create size expressions
    let size_expr_5 = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(5)),
        span: dummy_span(),
    };
    let size_expr_10 = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(10)),
        span: dummy_span(),
    };

    // Same element type and size
    let array1 = Type::Array(Box::new(Type::I32), Box::new(size_expr_5.clone()));
    let array2 = Type::Array(Box::new(Type::I32), Box::new(size_expr_5.clone()));
    assert!(checker.is_same_type(&array1, &array2));

    // Same element type but different sizes
    let array3 = Type::Array(Box::new(Type::I32), Box::new(size_expr_10.clone()));
    assert!(!checker.is_same_type(&array1, &array3));

    // Different element types but same size
    let array4 = Type::Array(Box::new(Type::F64), Box::new(size_expr_5.clone()));
    assert!(!checker.is_same_type(&array1, &array4));

    // Nested arrays
    let nested1 = Type::Array(
        Box::new(Type::Array(Box::new(Type::I32), Box::new(size_expr_5.clone()))),
        Box::new(size_expr_5.clone()),
    );
    let nested2 = Type::Array(
        Box::new(Type::Array(Box::new(Type::I32), Box::new(size_expr_5.clone()))),
        Box::new(size_expr_5.clone()),
    );
    assert!(checker.is_same_type(&nested1, &nested2));

    // Nested arrays with different inner size
    let nested3 = Type::Array(
        Box::new(Type::Array(Box::new(Type::I32), Box::new(size_expr_10.clone()))),
        Box::new(size_expr_5.clone()),
    );
    assert!(!checker.is_same_type(&nested1, &nested3));
}

#[test]
fn test_get_size() {
    let checker = TypeChecker::new();

    // Positive integers
    let expr_i8 = Expr::Literal {
        value: LiteralValue::Number(Number::I8(42)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_i8), Some(42));

    let expr_i16 = Expr::Literal {
        value: LiteralValue::Number(Number::I16(100)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_i16), Some(100));

    let expr_i32 = Expr::Literal {
        value: LiteralValue::Number(Number::I32(42)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_i32), Some(42));

    let expr_i64 = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(100)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_i64), Some(100));

    let expr_u8 = Expr::Literal {
        value: LiteralValue::Number(Number::U8(200)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_u8), Some(200));

    let expr_u16 = Expr::Literal {
        value: LiteralValue::Number(Number::U16(300)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_u16), Some(300));

    let expr_u32 = Expr::Literal {
        value: LiteralValue::Number(Number::U32(400)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_u32), Some(400));

    let expr_u64 = Expr::Literal {
        value: LiteralValue::Number(Number::UnsignedInteger(500)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_u64), Some(500));

    // Zero
    let expr_zero = Expr::Literal {
        value: LiteralValue::Number(Number::I8(0)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_zero), Some(0));

    // Negative integer (should return None)
    let expr_negative = Expr::Literal {
        value: LiteralValue::Number(Number::Integer(-5)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_negative), None);

    // Non-integer literals (should return None)
    let expr_float = Expr::Literal {
        value: LiteralValue::Number(Number::Float64(3.14)),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_float), None);

    let expr_bool = Expr::Literal {
        value: LiteralValue::Bool(true),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_bool), None);

    // Non-literal expression (should return None)
    let expr_variable = Expr::Variable {
        name: "x".to_string(),
        span: dummy_span(),
    };
    assert_eq!(checker.get_size(&expr_variable), None);
}