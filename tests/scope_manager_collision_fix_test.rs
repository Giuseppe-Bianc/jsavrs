use std::sync::Arc;

use jsavrs::ir::generator::IrGenerator;
use jsavrs::parser::ast::{Stmt, Type};
use jsavrs::utils::*;

// Test that simulates the actual issue from the IR generator
#[test]
fn test_ir_generator_multiple_functions_no_collision() {
    // Create a simple program with two functions
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

    // This should not panic anymore
    let mut generator = IrGenerator::new();
    let (module, ir_errors) = generator.generate(ast, "test_file.vn");

    // Verify no errors
    assert_eq!(ir_errors.len(), 0);

    // Verify both functions were generated
    assert_eq!(module.functions.len(), 2);
    assert_eq!(module.functions[0].name, Arc::from("func1"));
    assert_eq!(module.functions[1].name, Arc::from("func2"));
}
