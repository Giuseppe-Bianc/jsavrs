use jsavrs::ir::generator::NIrGenerator;
use jsavrs::ir::{
    InstructionKind, IrBinaryOp, IrConstantValue, IrLiteralValue, IrType, IrUnaryOp, TerminatorKind, ValueKind,
};
use jsavrs::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type, UnaryOp};
use jsavrs::tokens::number::Number;
use jsavrs::utils::*;
use std::sync::Arc;

// Macro per verificare i return con valori letterali
#[macro_export]
macro_rules! assert_return_literal {
    ($block:expr, $expected_ty:expr, $expected_literal:pat) => {
        match &$block.terminator().kind {
            TerminatorKind::Return { value, ty } => {
                assert_eq!(ty, $expected_ty); // Remove the * dereference
                match &value.kind {
                    ValueKind::Literal($expected_literal) => (),
                    other => panic!("Expected literal value, got {:?}", other),
                }
            }
            other => panic!("Unexpected terminator: {:?}", other),
        }
    };
}

// Macro per verificare i return con stringhe costanti
#[macro_export]
macro_rules! assert_return_constant_string {
    ($block:expr, $expected_str:expr) => {
        match &$block.terminator().kind {
            TerminatorKind::Return { value, ty } => {
                assert_eq!(*ty, IrType::String);
                match &value.kind {
                    ValueKind::Constant(IrConstantValue::String { string }) => {
                        assert_eq!(&**string, $expected_str);
                    }
                    other => panic!("Expected constant string, got {:?}", other),
                }
            }
            other => panic!("Unexpected terminator: {:?}", other),
        }
    };
}

// Macro per verificare i return con nullptr
#[macro_export]
macro_rules! assert_return_nullptr {
    ($block:expr) => {
        match &$block.terminator().kind {
            TerminatorKind::Return { value, ty } => {
                assert_eq!(*ty, IrType::Pointer(Box::new(IrType::I8)));
                assert_eq!(value.kind, ValueKind::Literal(IrLiteralValue::I64(0)));
            }
            other => panic!("Unexpected terminator: {:?}", other),
        }
    };
}

// Macro per verificare le istruzioni Alloca
#[macro_export]
macro_rules! assert_alloca_instruction {
    ($instruction:expr, $expected_ty:expr) => {
        match &$instruction.kind {
            InstructionKind::Alloca { ty } => {
                assert_eq!(*ty, $expected_ty);
            }
            other => panic!("Expected Alloca instruction, got {:?}", other),
        }
    };
}

// Macro per verificare le istruzioni Store
#[macro_export]
macro_rules! assert_store_instruction {
    ($instruction:expr, $expected_value:expr, $expected_dest:expr) => {
        match &$instruction.kind {
            InstructionKind::Store { value, dest } => {
                assert_eq!(value.kind, $expected_value);
                assert_eq!(dest.kind, $expected_dest);
            }
            other => panic!("Expected Store instruction, got {:?}", other),
        }
    };
}

// Macro per verificare le istruzioni Binary
#[macro_export]
macro_rules! assert_binary_instruction {
    ($instruction:expr, $expected_op:expr, $expected_ty:expr, $expected_left:expr, $expected_right:expr) => {
        match &$instruction.kind {
            InstructionKind::Binary { op, left, right, ty } => {
                assert_eq!(*op, $expected_op);
                assert_eq!(left.kind, $expected_left);
                assert_eq!(right.kind, $expected_right);
                assert_eq!(*ty, $expected_ty);
            }
            other => panic!("Expected Binary instruction, got {:?}", other),
        }
    };
}

// Macro per verificare le istruzioni GetElementPtr
#[macro_export]
macro_rules! assert_gep_instruction {
    ($instruction:expr, $expected_base:expr, $expected_index:expr, $expected_element_ty:expr) => {
        match &$instruction.kind {
            InstructionKind::GetElementPtr { base, index, element_ty } => {
                assert_eq!(base.kind, $expected_base);
                assert_eq!(index.kind, $expected_index);
                assert_eq!(*element_ty, $expected_element_ty);
            }
            other => panic!("Expected GetElementPtr instruction, got {:?}", other),
        }
    };
}

// Macro per verificare i terminator ConditionalBranch
#[macro_export]
macro_rules! assert_conditional_branch {
    ($block:expr, $expected_condition:expr, $expected_true:expr, $expected_false:expr) => {
        match &$block.terminator().kind {
            TerminatorKind::ConditionalBranch { condition, true_label, false_label } => {
                assert_eq!(condition.kind, $expected_condition);
                assert_eq!(true_label, $expected_true);
                assert_eq!(false_label, $expected_false);
            }
            other => panic!("Expected ConditionalBranch terminator, got {:?}", other),
        }
    };
}

// Macro per verificare i terminator Branch
#[macro_export]
macro_rules! assert_branch {
    ($block:expr, $expected_label:expr) => {
        match &$block.terminator().kind {
            TerminatorKind::Branch { label } => {
                assert_eq!(label, $expected_label);
            }
            other => panic!("Expected Branch terminator, got {:?}", other),
        }
    };
}

#[test]
fn test_generate_function_with_return() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::I32,
        vec![Stmt::Return { value: Some(num_lit_i32(42)), span: dummy_span() }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::I32);
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.clone().instructions.len(), 0);
    // VERIFICA TERMINATOR
    assert_return_literal!(entry_block, &IrType::I32, IrLiteralValue::I32(42));
}

#[test]
fn test_generate_void_function() {
    let ast = vec![function_declaration(
        "void_func".into(),
        vec![],
        Type::Void,
        vec![Stmt::Return { value: None, span: dummy_span() }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.name, "void_func");
    assert_eq!(func.return_type, IrType::Void);
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_void_func");
    let entry_block = func.cfg.get_block("entry_void_func").unwrap();
    assert_eq!(entry_block.clone().instructions.len(), 0);
    assert_return_literal!(entry_block, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_main_function() {
    let ast =
        vec![Stmt::MainFunction { body: vec![Stmt::Return { value: None, span: dummy_span() }], span: dummy_span() }];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.name, "main");
    assert_eq!(func.return_type, IrType::Void);
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_main");
    let entry_block = func.cfg.get_block("entry_main").unwrap();
    assert_eq!(entry_block.clone().instructions.len(), 0);
    // VERIFICA TERMINATOR
    assert_return_literal!(entry_block, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_binary_expression() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(binary_expr(num_lit_i32(10), BinaryOp::Add, num_lit_i32(20))),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");

    // Verifica assenza errori
    assert_eq!(ir_errors.len(), 0);

    // Verifica struttura funzione
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::I32);

    // Verifica struttura CFG
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");

    // Verifica contenuto blocco entry
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 1);

    // VERIFICA ISTRUZIONE BINARY
    let instruction = &entry_block.instructions[0];
    assert_binary_instruction!(
        instruction,
        IrBinaryOp::Add,
        IrType::I32,
        ValueKind::Literal(IrLiteralValue::I32(10)),
        ValueKind::Literal(IrLiteralValue::I32(20))
    );
}

#[test]
fn test_generate_variable_assignment() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![
            var_declaration(vec!["x".into()], Type::I32, true, vec![]),
            Stmt::Expression { expr: assign_expr(variable_expr("x"), num_lit_i32(10)) },
        ],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");

    // Verifica assenza errori
    assert_eq!(ir_errors.len(), 0);

    // Verifica struttura funzione
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::Void);

    // Verifica struttura CFG
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");

    // Verifica contenuto blocco entry
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 2);
    // VERIFICA ISTRUZIONE ALLOCA
    let alloca_instr = &entry_block.instructions[0];
    assert_alloca_instruction!(alloca_instr, IrType::I32);
    // VERIFICA ISTRUZIONE STORE
    let store_instr = &entry_block.instructions[1];
    assert_store_instruction!(store_instr, ValueKind::Literal(IrLiteralValue::I32(10)), ValueKind::Temporary(0));
}

#[test]
fn test_generate_if_statement() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![Stmt::If {
            condition: bool_lit(true),
            then_branch: vec![Stmt::Return { value: None, span: dummy_span() }],
            else_branch: None,
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 4);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_conditional_branch!(
        entry_block,
        ValueKind::Literal(IrLiteralValue::Bool(true)),
        &Arc::from("then_1"),
        &Arc::from("else_2")
    );
    let then_block = func.cfg.get_block("then_1").unwrap();
    assert_eq!(then_block.instructions.len(), 0);
    assert_return_literal!(then_block, &IrType::Void, IrLiteralValue::I32(0));
    let else_block = func.cfg.get_block("else_2").unwrap();
    assert_eq!(else_block.instructions.len(), 0);
    assert_branch!(else_block, &Arc::from("merge_3"));
    let merge_block = func.cfg.get_block("merge_3").unwrap();
    assert_eq!(merge_block.instructions.len(), 0);
    assert_return_literal!(merge_block, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_nested_expressions() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(binary_expr(
                unary_expr(UnaryOp::Negate, num_lit_i32(5)),
                BinaryOp::Multiply,
                binary_expr(num_lit_i32(3), BinaryOp::Add, num_lit_i32(2)),
            )),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 3);

    let first_instruction = &entry_block.instructions[0];
    match first_instruction.clone().kind {
        InstructionKind::Unary { op, operand, ty } => {
            assert_eq!(op, IrUnaryOp::Negate);
            assert_eq!(ty, IrType::I32);
            assert_eq!(operand.kind, ValueKind::Literal(IrLiteralValue::I32(5)));
        }
        other => panic!("Unexpected kind: {:?}", other),
    }
    let second_instruction = &entry_block.instructions[1];
    assert_binary_instruction!(
        second_instruction.clone(),
        IrBinaryOp::Add,
        IrType::I32,
        ValueKind::Literal(IrLiteralValue::I32(3)),
        ValueKind::Literal(IrLiteralValue::I32(2))
    );

    let third_instruction = &entry_block.instructions[2];
    assert_binary_instruction!(
        third_instruction.clone(),
        IrBinaryOp::Multiply,
        IrType::I32,
        ValueKind::Temporary(0),
        ValueKind::Temporary(1)
    );
}

#[test]
fn test_generate_custom_type() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![Parameter { name: "param".into(), type_annotation: Type::Custom("MyType".into()), span: dummy_span() }],
        Type::Custom("MyType".into()),
        vec![Stmt::Return { value: Some(variable_expr("param")), span: dummy_span() }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.parameters[0].ty, IrType::Custom("MyType".into(), dummy_span()));
    assert_eq!(func.return_type, IrType::Custom("MyType".into(), dummy_span()));
}

#[test]
fn test_generate_array_type() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![var_declaration(
            vec!["arr".into()],
            Type::Array(Box::new(Type::I32), Box::new(num_lit_i64(10))),
            true,
            vec![],
        )],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 1);
    // VERIFICA ISTRUZIONE ALLOCA
    let instruction = &entry_block.instructions[0];
    assert_alloca_instruction!(instruction, IrType::Array(Box::new(IrType::I32), 10));
}

#[test]
fn test_generate_missing_return() {
    let ast = vec![Stmt::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: Type::I32,
        body: vec![],
        span: dummy_span(),
    }];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_return_literal!(entry_block, &IrType::I32, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_multiple_functions() {
    let ast = vec![
        function_declaration(
            "func1".into(),
            vec![],
            Type::Void,
            vec![Stmt::Return { value: None, span: dummy_span() }],
        ),
        function_declaration(
            "func2".into(),
            vec![],
            Type::Void,
            vec![Stmt::Return { value: None, span: dummy_span() }],
        ),
    ];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 2);
    assert_eq!(functions.functions[0].name, "func1");
    assert_eq!(functions.functions[1].name, "func2");
}

#[test]
fn test_generate_string_literal() {
    let ast = vec![Stmt::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: Type::String,
        body: vec![Stmt::Return { value: Some(string_lit("hello")), span: dummy_span() }],
        span: dummy_span(),
    }];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_return_constant_string!(entry_block, "hello");
}

#[test]
fn test_generate_nullptr() {
    let ast = vec![Stmt::Function {
        name: "test".into(),
        parameters: vec![],
        return_type: Type::NullPtr,
        body: vec![Stmt::Return { value: Some(nullptr_lit()), span: dummy_span() }],
        span: dummy_span(),
    }];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_return_nullptr!(entry_block);
}

#[test]
fn test_generate_simple_block() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![
            Stmt::Block {
                statements: vec![
                    var_declaration(vec!["y".into()], Type::I32, true, vec![num_lit_i32(5)]),
                    Stmt::Expression {
                        expr: Expr::Assign {
                            target: Box::new(variable_expr("y")),
                            value: Box::new(num_lit_i32(10)),
                            span: dummy_span(),
                        },
                    },
                ],
                span: dummy_span(),
            },
            Stmt::Return { value: None, span: dummy_span() },
        ],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 3);
    // Verifica istruzioni all'interno del blocco
    assert_alloca_instruction!(entry_block.instructions[0], IrType::I32);
    assert_store_instruction!(
        entry_block.instructions[1],
        ValueKind::Literal(IrLiteralValue::I32(5)),
        ValueKind::Temporary(0)
    );
    assert_store_instruction!(
        entry_block.instructions[2],
        ValueKind::Literal(IrLiteralValue::I32(10)),
        ValueKind::Temporary(0)
    );
}

#[test]
fn test_generate_simple_while_loop() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![
            var_declaration(vec!["counter".into()], Type::I32, true, vec![num_lit_i32(0)]),
            Stmt::While {
                condition: binary_expr(variable_expr("counter"), BinaryOp::Less, num_lit_i32(5)),
                body: vec![Stmt::Expression {
                    expr: Expr::Assign {
                        target: Box::new(variable_expr("counter")),
                        value: Box::new(binary_expr(variable_expr("counter"), BinaryOp::Add, num_lit_i32(1))),
                        span: dummy_span(),
                    },
                }],
                span: dummy_span(),
            },
            Stmt::Return { value: None, span: dummy_span() },
        ],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 4);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 2);
    assert_alloca_instruction!(entry_block.instructions[0], IrType::I32);
    assert_store_instruction!(
        entry_block.instructions[1],
        ValueKind::Literal(IrLiteralValue::I32(0)),
        ValueKind::Temporary(0)
    );
    assert_branch!(entry_block, &Arc::from("loop_start_1"));
    let loop_start = func.cfg.get_block("loop_start_1").unwrap();
    assert_eq!(loop_start.instructions.len(), 1);
    assert_binary_instruction!(
        loop_start.instructions[0].clone(),
        IrBinaryOp::Less,
        IrType::Pointer(Box::new(IrType::I32)),
        ValueKind::Temporary(0),
        ValueKind::Literal(IrLiteralValue::I32(5))
    );
    assert_conditional_branch!(
        loop_start,
        ValueKind::Temporary(1),
        &Arc::from("loop_body_2"),
        &Arc::from("loop_end_3")
    );

    let loop_body = func.cfg.get_block("loop_body_2").unwrap();
    assert_eq!(loop_body.instructions.len(), 2);
    assert_binary_instruction!(
        loop_body.instructions[0].clone(),
        IrBinaryOp::Add,
        IrType::Pointer(Box::new(IrType::I32)),
        ValueKind::Temporary(0),
        ValueKind::Literal(IrLiteralValue::I32(1))
    );
    assert_store_instruction!(loop_body.instructions[1].clone(), ValueKind::Temporary(2), ValueKind::Temporary(0));
    assert_branch!(loop_body, &Arc::from("loop_start_1"));

    let loop_end = func.cfg.get_block("loop_end_3").unwrap();
    assert_eq!(loop_end.instructions.len(), 0);
    assert_return_literal!(loop_end, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_for_loop_basic() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![Stmt::For {
            initializer: Some(Box::new(Stmt::VarDeclaration {
                variables: vec!["i".into()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![num_lit_i32(0)],
                span: dummy_span(),
            })),
            condition: Some(binary_expr(variable_expr("i"), BinaryOp::Less, num_lit_i32(10))),
            increment: Some(Expr::Assign {
                target: Box::new(variable_expr("i")),
                value: Box::new(binary_expr(variable_expr("i"), BinaryOp::Add, num_lit_i32(1))),
                span: dummy_span(),
            }),
            body: vec![],
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    // Verify block structure: entry, for_start, for_body, for_inc, for_end
    assert_eq!(func.cfg.blocks().count(), 5);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_branch!(entry_block, &Arc::from("for_start_1"));
    let for_start = func.cfg.get_block("for_start_1").unwrap();
    assert_eq!(for_start.instructions.len(), 1);
    assert_conditional_branch!(for_start, ValueKind::Temporary(1), &Arc::from("for_body_2"), &Arc::from("for_end_4"));
    let for_body = func.cfg.get_block("for_body_2").unwrap();
    assert_eq!(for_body.instructions.len(), 0);
    assert_branch!(for_body, &Arc::from("for_inc_3"));
    let for_inc = func.cfg.get_block("for_inc_3").unwrap();
    assert_eq!(for_inc.instructions.len(), 2);
    assert_branch!(for_inc, &Arc::from("for_start_1"));
    let for_end = func.cfg.get_block("for_end_4").unwrap();
    assert_eq!(for_end.instructions.len(), 0);
    assert_return_literal!(for_end, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_for_loop_with_break() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![Stmt::For {
            initializer: Some(Box::new(Stmt::VarDeclaration {
                variables: vec!["i".into()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![num_lit_i32(0)],
                span: dummy_span(),
            })),
            condition: Some(binary_expr(variable_expr("i"), BinaryOp::Less, num_lit_i32(10))),
            increment: None,
            body: vec![Stmt::Break { span: dummy_span() }],
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    // Verify block structure: entry, for_start, for_body, for_inc, for_end
    assert_eq!(func.cfg.blocks().count(), 5);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_branch!(entry_block, &Arc::from("for_start_1"));
    let for_start = func.cfg.get_block("for_start_1").unwrap();
    assert_eq!(for_start.instructions.len(), 1);
    assert_conditional_branch!(for_start, ValueKind::Temporary(1), &Arc::from("for_body_2"), &Arc::from("for_end_4"));
    let for_body = func.cfg.get_block("for_body_2").unwrap();
    assert_eq!(for_body.instructions.len(), 0);
    assert_branch!(for_body, &Arc::from("for_end_4"));
    let for_inc = func.cfg.get_block("for_inc_3").unwrap();
    assert_eq!(for_inc.instructions.len(), 0);
    assert_branch!(for_inc, &Arc::from("for_start_1"));
    let for_end = func.cfg.get_block("for_end_4").unwrap();
    assert_eq!(for_end.instructions.len(), 0);
    assert_return_literal!(for_end, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_for_loop_with_continue() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![Stmt::For {
            initializer: Some(Box::new(Stmt::VarDeclaration {
                variables: vec!["i".into()],
                type_annotation: Type::I32,
                is_mutable: true,
                initializers: vec![num_lit_i32(0)],
                span: dummy_span(),
            })),
            condition: Some(binary_expr(variable_expr("i"), BinaryOp::Less, num_lit_i32(10))),
            increment: None,
            body: vec![Stmt::Continue { span: dummy_span() }],
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    // Verify block structure: entry, for_start, for_body, for_inc, for_end
    assert_eq!(func.cfg.blocks().count(), 5);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_branch!(entry_block, &Arc::from("for_start_1"));
    let for_start = func.cfg.get_block("for_start_1").unwrap();
    assert_eq!(for_start.instructions.len(), 1);
    assert_conditional_branch!(for_start, ValueKind::Temporary(1), &Arc::from("for_body_2"), &Arc::from("for_end_4"));
    let for_body = func.cfg.get_block("for_body_2").unwrap();
    assert_eq!(for_body.instructions.len(), 0);
    assert_branch!(for_body, &Arc::from("for_inc_3"));
    let for_inc = func.cfg.get_block("for_inc_3").unwrap();
    assert_eq!(for_inc.instructions.len(), 0);
    assert_branch!(for_inc, &Arc::from("for_start_1"));
    let for_end = func.cfg.get_block("for_end_4").unwrap();
    assert_eq!(for_end.instructions.len(), 0);
    assert_return_literal!(for_end, &IrType::Void, IrLiteralValue::I32(0));
}

#[test]
fn test_generate_grouping_expression() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::I32,
        vec![Stmt::Return {
            value: Some(grouping_expr(binary_expr(num_lit_i32(10), BinaryOp::Add, num_lit_i32(20)))),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 1);
    assert_binary_instruction!(
        entry_block.instructions[0].clone(),
        IrBinaryOp::Add,
        IrType::I32,
        ValueKind::Literal(IrLiteralValue::I32(10)),
        ValueKind::Literal(IrLiteralValue::I32(20))
    );
}

#[test]
fn test_generate_array_literal_with_elements() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Array(Box::new(Type::I32), Box::new(num_lit_i64(3))),
        vec![Stmt::Return {
            value: Some(Expr::ArrayLiteral {
                elements: vec![num_lit_i32(10), num_lit_i32(20), num_lit_i32(30)],
                span: dummy_span(),
            }),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_eq!(entry_block.instructions.len(), 7);

    assert_alloca_instruction!(entry_block.instructions[0].clone(), IrType::Array(Box::new(IrType::I32), 3));
    assert_gep_instruction!(
        entry_block.instructions[1].clone(),
        ValueKind::Temporary(0),
        ValueKind::Literal(IrLiteralValue::I32(0)),
        IrType::I32
    );
    assert_store_instruction!(
        entry_block.instructions[2].clone(),
        ValueKind::Literal(IrLiteralValue::I32(10)),
        ValueKind::Temporary(1)
    );
    assert_gep_instruction!(
        entry_block.instructions[3].clone(),
        ValueKind::Temporary(0),
        ValueKind::Literal(IrLiteralValue::I32(1)),
        IrType::I32
    );
    assert_store_instruction!(
        entry_block.instructions[4].clone(),
        ValueKind::Literal(IrLiteralValue::I32(20)),
        ValueKind::Temporary(2)
    );
    assert_gep_instruction!(
        entry_block.instructions[5].clone(),
        ValueKind::Temporary(0),
        ValueKind::Literal(IrLiteralValue::I32(2)),
        IrType::I32
    );
    assert_store_instruction!(
        entry_block.instructions[6].clone(),
        ValueKind::Literal(IrLiteralValue::I32(30)),
        ValueKind::Temporary(3)
    );
}

#[test]
fn test_default_implementation() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::I32,
        vec![Stmt::Return { value: Some(num_lit_i32(42)), span: dummy_span() }],
    )];

    let mut generator = NIrGenerator::default();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();
    assert_return_literal!(entry_block, &IrType::I32, IrLiteralValue::I32(42));
}

#[test]
fn test_generate_binary_all_operations() {
    let test_cases = vec![
        (BinaryOp::Add, IrBinaryOp::Add),
        (BinaryOp::Subtract, IrBinaryOp::Subtract),
        (BinaryOp::Multiply, IrBinaryOp::Multiply),
        (BinaryOp::Divide, IrBinaryOp::Divide),
        (BinaryOp::Modulo, IrBinaryOp::Modulo),
        (BinaryOp::Equal, IrBinaryOp::Equal),
        (BinaryOp::NotEqual, IrBinaryOp::NotEqual),
        (BinaryOp::Less, IrBinaryOp::Less),
        (BinaryOp::LessEqual, IrBinaryOp::LessEqual),
        (BinaryOp::Greater, IrBinaryOp::Greater),
        (BinaryOp::GreaterEqual, IrBinaryOp::GreaterEqual),
        (BinaryOp::And, IrBinaryOp::And),
        (BinaryOp::Or, IrBinaryOp::Or),
        (BinaryOp::BitwiseAnd, IrBinaryOp::BitwiseAnd),
        (BinaryOp::BitwiseOr, IrBinaryOp::BitwiseOr),
        (BinaryOp::BitwiseXor, IrBinaryOp::BitwiseXor),
        (BinaryOp::ShiftLeft, IrBinaryOp::ShiftLeft),
        (BinaryOp::ShiftRight, IrBinaryOp::ShiftRight),
    ];

    for (ast_op, expected_ir_op) in test_cases {
        let ast = vec![function_declaration(
            "test".into(),
            vec![],
            Type::I32,
            vec![Stmt::Return {
                value: Some(binary_expr(num_lit_i32(10), ast_op, num_lit_i32(20))),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::default();
        let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.functions.len(), 1);
        let func = &functions.functions[0];
        assert_eq!(func.cfg.blocks().count(), 1);
        assert_eq!(func.cfg.entry_label, "entry_test");
        let entry_block = func.cfg.get_block("entry_test").unwrap();
        assert_eq!(entry_block.instructions.len(), 1);
        assert_binary_instruction!(
            entry_block.instructions[0].clone(),
            expected_ir_op,
            IrType::I32,
            ValueKind::Literal(IrLiteralValue::I32(10)),
            ValueKind::Literal(IrLiteralValue::I32(20))
        );
    }
}

#[test]
fn test_generate_unary_expression() {
    let test_cases = vec![(UnaryOp::Negate, IrUnaryOp::Negate), (UnaryOp::Not, IrUnaryOp::Not)];

    for (ast_op, expected_ir_op) in test_cases {
        let ast = vec![function_declaration(
            "test".into(),
            vec![],
            Type::I32,
            vec![Stmt::Return { value: Some(unary_expr(ast_op, num_lit_i32(42))), span: dummy_span() }],
        )];

        let mut generator = NIrGenerator::default();
        let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.functions.len(), 1);
        let func = &functions.functions[0];
        assert_eq!(func.cfg.blocks().count(), 1);
        assert_eq!(func.cfg.entry_label, "entry_test");
        let entry_block = func.cfg.get_block("entry_test").unwrap();
        assert_eq!(entry_block.instructions.len(), 1);

        match &entry_block.instructions[0].kind {
            InstructionKind::Unary { op, operand, ty } => {
                assert_eq!(*op, expected_ir_op);
                assert_eq!(*ty, IrType::I32);
                assert_eq!(operand.kind, ValueKind::Literal(IrLiteralValue::I32(42)));
            }
            other => panic!("Unexpected kind: {:?}", other),
        }
    }
}

#[test]
fn test_generate_integer_literals() {
    let test_cases = vec![
        (Number::I8(42), IrLiteralValue::I8(42), IrType::I8),
        (Number::I16(1000), IrLiteralValue::I16(1000), IrType::I16),
        (Number::I32(32000), IrLiteralValue::I32(32000), IrType::I32),
        (Number::Integer(2_000_000_000), IrLiteralValue::I64(2_000_000_000), IrType::I64),
        (Number::U8(255), IrLiteralValue::U8(255), IrType::U8),
        (Number::U16(65535), IrLiteralValue::U16(65535), IrType::U16),
        (Number::U32(4_000_000_000), IrLiteralValue::U32(4_000_000_000), IrType::U32),
        (
            Number::UnsignedInteger(18_000_000_000_000_000_000),
            IrLiteralValue::U64(18_000_000_000_000_000_000),
            IrType::U64,
        ),
    ];

    for (num, expected_value, expected_type) in test_cases {
        let ast = vec![function_declaration(
            "test".into(),
            vec![],
            match num {
                Number::I8(_) => Type::I8,
                Number::I16(_) => Type::I16,
                Number::I32(_) => Type::I32,
                Number::Integer(_) => Type::I64,
                Number::U8(_) => Type::U8,
                Number::U16(_) => Type::U16,
                Number::U32(_) => Type::U32,
                Number::UnsignedInteger(_) => Type::U64,
                _ => Type::I32,
            },
            vec![Stmt::Return {
                value: Some(Expr::Literal { value: LiteralValue::Number(num), span: dummy_span() }),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::default();
        let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.functions.len(), 1);
        let func = &functions.functions[0];
        assert_eq!(func.cfg.blocks().count(), 1);
        assert_eq!(func.cfg.entry_label, "entry_test");
        let entry_block = func.cfg.get_block("entry_test").unwrap();

        match &entry_block.terminator().kind {
            TerminatorKind::Return { value, ty } => {
                assert_eq!(*ty, expected_type);
                match &value.kind {
                    ValueKind::Literal(imm) => assert_eq!(imm, &expected_value),
                    _ => panic!("Expected immediate value"),
                }
            }
            other => panic!("Unexpected terminator: {:?}", other),
        }
    }
}

#[allow(clippy::approx_constant)]
#[test]
fn test_generate_float_literals() {
    let test_cases = vec![
        (Number::Float32(3.14), IrLiteralValue::F32(3.14), IrType::F32),
        (Number::Float64(123.456), IrLiteralValue::F64(123.456), IrType::F64),
        (Number::Scientific32(2.0, 2), IrLiteralValue::F32(4.0), IrType::F32),
        (Number::Scientific64(10.0, 3), IrLiteralValue::F64(1000.0), IrType::F64),
    ];

    for (num, expected_value, expected_type) in test_cases {
        let ast = vec![function_declaration(
            "test".into(),
            vec![],
            match num {
                Number::Float32(_) => Type::F32,
                Number::Float64(_) => Type::F64,
                Number::Scientific32(_, _) => Type::F32,
                Number::Scientific64(_, _) => Type::F64,
                _ => Type::F32,
            },
            vec![Stmt::Return {
                value: Some(Expr::Literal { value: LiteralValue::Number(num), span: dummy_span() }),
                span: dummy_span(),
            }],
        )];

        let mut generator = NIrGenerator::default();
        let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.functions.len(), 1);
        let func = &functions.functions[0];
        assert_eq!(func.cfg.blocks().count(), 1);
        assert_eq!(func.cfg.entry_label, "entry_test");
        let entry_block = func.cfg.get_block("entry_test").unwrap();

        match &entry_block.terminator().kind {
            TerminatorKind::Return { value, ty } => {
                assert_eq!(*ty, expected_type);
                match &value.kind {
                    ValueKind::Literal(imm) => assert_eq!(imm, &expected_value),
                    _ => panic!("Expected immediate value"),
                }
            }
            other => panic!("Unexpected terminator: {:?}", other),
        }
    }
}

#[test]
fn test_generate_boolean_literals() {
    let test_cases = vec![(true, IrLiteralValue::Bool(true)), (false, IrLiteralValue::Bool(false))];

    for (b, expected_value) in test_cases {
        let ast = vec![function_declaration(
            "test".into(),
            vec![],
            Type::Bool,
            vec![Stmt::Return { value: Some(bool_lit(b)), span: dummy_span() }],
        )];

        let mut generator = NIrGenerator::default();
        let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
        assert_eq!(ir_errors.len(), 0);
        assert_eq!(functions.functions.len(), 1);
        let func = &functions.functions[0];
        assert_eq!(func.cfg.blocks().count(), 1);
        assert_eq!(func.cfg.entry_label, "entry_test");
        let entry_block = func.cfg.get_block("entry_test").unwrap();

        match &entry_block.terminator().kind {
            TerminatorKind::Return { value, ty } => {
                assert_eq!(*ty, IrType::Bool);
                match &value.kind {
                    ValueKind::Literal(imm) => assert_eq!(imm, &expected_value),
                    _ => panic!("Expected immediate value"),
                }
            }
            other => panic!("Unexpected terminator: {:?}", other),
        }
    }
}

#[test]
fn test_generate_char_literal() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Char,
        vec![Stmt::Return { value: Some(char_lit("A")), span: dummy_span() }],
    )];

    let mut generator = NIrGenerator::default();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();

    match &entry_block.terminator().kind {
        TerminatorKind::Return { value, ty } => {
            assert_eq!(*ty, IrType::Char);
            match &value.kind {
                ValueKind::Literal(IrLiteralValue::Char(c)) => assert_eq!(*c, 'A'),
                _ => panic!("Expected immediate value"),
            }
        }
        other => panic!("Unexpected terminator: {:?}", other),
    }
}

#[test]
fn test_generate_nullptr_literal() {
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::NullPtr,
        vec![Stmt::Return { value: Some(nullptr_lit()), span: dummy_span() }],
    )];

    let mut generator = NIrGenerator::default();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);
    let func = &functions.functions[0];
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");
    let entry_block = func.cfg.get_block("entry_test").unwrap();

    assert_return_nullptr!(entry_block);
}

#[test]
fn test_generate_array_access_assignment() {
    // Creiamo un AST che dichiara un array e assegna un valore a un elemento
    let ast = vec![function_declaration(
        "test".into(),
        vec![],
        Type::Void,
        vec![
            // Dichiarazione di un array di 3 interi
            var_declaration(vec!["arr".into()], Type::Array(Box::new(Type::I32), Box::new(num_lit_i64(3))), true, vec![]),
            // Assegnazione a un elemento dell'array: arr[1] = 42
            Stmt::Expression {
                expr: Expr::Assign {
                    target: Box::new(Expr::ArrayAccess {
                        array: Box::new(variable_expr("arr")),
                        index: Box::new(num_lit_i32(1)),
                        span: dummy_span(),
                    }),
                    value: Box::new(num_lit_i32(42)),
                    span: dummy_span(),
                },
            },
        ],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");

    // Verifica che non ci siano errori
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);

    let func = &functions.functions[0];
    assert_eq!(func.name, "test");
    assert_eq!(func.return_type, IrType::Void);

    // Verifica la struttura del CFG
    assert_eq!(func.cfg.blocks().count(), 1);
    assert_eq!(func.cfg.entry_label, "entry_test");

    let entry_block = func.cfg.get_block("entry_test").unwrap();

    // Dovremmo avere 3 istruzioni: alloca, gep e store
    assert_eq!(entry_block.instructions.len(), 3);

    // Verifica l'istruzione di allocazione per l'array
    let alloca_inst = &entry_block.instructions[0];
    assert_alloca_instruction!(alloca_inst, IrType::Array(Box::new(IrType::I32), 3));

    // Verifica l'istruzione GEP per l'accesso all'array
    let gep_inst = &entry_block.instructions[1];
    assert_gep_instruction!(
        gep_inst,
        ValueKind::Temporary(0),                    // base: puntatore all'array
        ValueKind::Literal(IrLiteralValue::I32(1)), // indice: 1
        IrType::I32                                 // tipo dell'elemento
    );

    // Verifica il tipo del risultato del GEP
    assert_eq!(gep_inst.result.as_ref().unwrap().ty, IrType::Pointer(Box::new(IrType::I32)));

    // Verifica l'istruzione di store
    let store_inst = &entry_block.instructions[2];
    assert_store_instruction!(
        store_inst,
        ValueKind::Literal(IrLiteralValue::I32(42)), // valore da memorizzare
        ValueKind::Temporary(1)                      // destinazione: risultato del GEP
    );
}

#[test]
fn test_generate_simple_function_call() {
    let ast = vec![
        function_declaration(
            "add".into(),
            vec![
                Parameter { name: "a".into(), type_annotation: Type::I32, span: dummy_span() },
                Parameter { name: "b".into(), type_annotation: Type::I32, span: dummy_span() },
            ],
            Type::I32,
            vec![Stmt::Return {
                value: Some(binary_expr(variable_expr("a"), BinaryOp::Add, variable_expr("b"))),
                span: dummy_span(),
            }],
        ),
        function_declaration(
            "main".into(),
            vec![],
            Type::I32,
            vec![Stmt::Return {
                value: Some(call_expr(variable_expr("add"), vec![num_lit_i32(5), num_lit_i32(3)])),
                span: dummy_span(),
            }],
        ),
    ];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 2);

    // Check the main function
    let main_func = functions.functions.iter().find(|f| f.name == "main").unwrap();
    assert_eq!(main_func.cfg.blocks().count(), 1);
    let entry_block = main_func.cfg.get_block("entry_main").unwrap();
    assert_eq!(entry_block.instructions.len(), 1);

    // Check the call instruction
    match &entry_block.instructions[0].kind {
        InstructionKind::Call { func, args, ty } => {
            // Check function reference
            match &func.kind {
                ValueKind::Global(name) => assert_eq!(name.as_ref(), "add"),
                _ => panic!("Expected global function reference"),
            }
            // Check arguments
            assert_eq!(args.len(), 2);
            assert_eq!(args[0].kind, ValueKind::Literal(IrLiteralValue::I32(5)));
            assert_eq!(args[1].kind, ValueKind::Literal(IrLiteralValue::I32(3)));
            // Check return type
            assert_eq!(*ty, IrType::I64); // Default return type in our implementation
        }
        _ => panic!("Expected Call instruction"),
    }
}

#[test]
fn test_generate_recursive_function_call() {
    let ast = vec![function_declaration(
        "factorial".into(),
        vec![Parameter { name: "n".into(), type_annotation: Type::I64, span: dummy_span() }],
        Type::I64,
        vec![Stmt::If {
            condition: binary_expr(variable_expr("n"), BinaryOp::LessEqual, num_lit_i64(1)),
            then_branch: vec![Stmt::Return { value: Some(num_lit_i64(1)), span: dummy_span() }],
            else_branch: Some(vec![Stmt::Return {
                value: Some(binary_expr(
                    variable_expr("n"),
                    BinaryOp::Multiply,
                    call_expr(variable_expr("factorial"), vec![binary_expr(variable_expr("n"), BinaryOp::Subtract, num_lit_i64(1))]),
                )),
                span: dummy_span(),
            }]),
            span: dummy_span(),
        }],
    )];

    let mut generator = NIrGenerator::new();
    let (functions, ir_errors) = generator.generate(ast, "test_file.vn");
    assert_eq!(ir_errors.len(), 0);
    assert_eq!(functions.functions.len(), 1);

    let func = &functions.functions[0];
    assert_eq!(func.name, "factorial");
    
    // Check that we have the right number of blocks (entry, then, else, merge)
    assert_eq!(func.cfg.blocks().count(), 4);
    
    // Check the else block which contains the recursive call
    let else_block = func.cfg.get_block("else_2").unwrap();
    // Should have the subtraction, the call, and the multiplication
    assert_eq!(else_block.instructions.len(), 3);
    
    // Check the call instruction
    match &else_block.instructions[1].kind {
        InstructionKind::Call { func, args, ty } => {
            // Check function reference
            match &func.kind {
                ValueKind::Global(name) => assert_eq!(name.as_ref(), "factorial"),
                _ => panic!("Expected global function reference"),
            }
            // Check argument (should be n - 1)
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].kind, ValueKind::Temporary(0)); // Result of n - 1
            // Check return type
            assert_eq!(*ty, IrType::I64);
        }
        _ => panic!("Expected Call instruction"),
    }
}
