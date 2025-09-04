use jsavrs::error::compile_error::CompileError;
use jsavrs::location::source_span::SourceSpan;
use jsavrs::parser::ast::{Parameter, Type};
use jsavrs::semantic::symbol_table::{FunctionSymbol, ScopeKind, Symbol, SymbolTable, VariableSymbol};
use jsavrs::utils::{
    create_func_symbol, create_span, create_var_symbol, dummy_span,
    func_from_symbol, int_type, var_from_symbol,
};

#[test]
fn global_scope_declaration_and_lookup() {
    let mut table = SymbolTable::new();
    let var_symbol = create_var_symbol("x", true);

    table.declare("x", var_symbol.clone()).unwrap();
    assert_eq!(table.lookup("x"), Some(var_symbol));
}

#[test]
fn shadowing_across_scopes() {
    let mut table = SymbolTable::new();
    let global_var = create_var_symbol("x", false);
    let local_var = create_var_symbol("x", true);

    table.declare("x", global_var.clone()).unwrap();
    table.push_scope(ScopeKind::Block, None);
    table.declare("x", local_var.clone()).unwrap();

    // Should find local variable in inner scope
    assert_eq!(table.lookup("x"), Some(local_var));
    table.pop_scope();

    // Should find global variable after popping scope
    assert_eq!(table.lookup("x"), Some(global_var));
}

#[test]
fn duplicate_declaration_same_scope() {
    let mut table = SymbolTable::new();
    let var1 = create_var_symbol("x", true);
    let var2 = create_var_symbol("x", false);

    table.declare("x", var1).unwrap();
    let result = table.declare("x", var2);

    // Fixed pattern matching without guard
    assert!(matches!(result, Err(CompileError::TypeError { .. })));

    // Verify error message content
    if let Err(CompileError::TypeError { message, .. }) = result {
        assert!(message.contains("already declared in this Global scope"));
    } else {
        panic!("Expected TypeError");
    }
}

#[test]
fn function_symbol_tracking() {
    let mut table = SymbolTable::new();
    let func = FunctionSymbol {
        name: "foo".to_string(),
        parameters: vec![Parameter {
            name: "arg".into(),
            type_annotation: Type::I8, // Use correct field name
            span: dummy_span(),
        }],
        return_type: Type::Void,
        defined_at: dummy_span(),
    };

    assert_eq!(table.current_function(), None);

    table.enter_function(func.clone());
    assert_eq!(table.current_function(), Some(&func));

    table.exit_function();
    assert_eq!(table.current_function(), None);
}

#[test]
fn lookup_non_existent_symbol() {
    let table = SymbolTable::new();
    assert_eq!(table.lookup("ghost"), None);
    assert_eq!(table.lookup_variable("phantom"), None);
    assert_eq!(table.lookup_function("specter"), None);
}

#[test]
fn scope_isolation() {
    let mut table = SymbolTable::new();
    table.declare("a", create_var_symbol("a", true)).unwrap();

    table.push_scope(ScopeKind::Block, None);
    assert_eq!(table.lookup("a"), Some(create_var_symbol("a", true))); // Can see parent
    table.declare("b", create_var_symbol("b", false)).unwrap();

    table.push_scope(ScopeKind::Block, None);
    assert_eq!(table.lookup("b"), Some(create_var_symbol("b", false))); // Can see grandparent

    table.pop_scope();
    table.pop_scope();
    assert_eq!(table.lookup("b"), None); // b not in global scope
}

#[test]
fn cannot_pop_global_scope() {
    let mut table = SymbolTable::new();
    table.pop_scope(); // Should not panic
    table.pop_scope(); // Multiple pops should be safe
    assert_eq!(table.scope_count(), 1);
}

#[test]
fn mixed_symbol_types() {
    let mut table = SymbolTable::new();
    let var = create_var_symbol("var", true);
    let func = create_func_symbol("func");

    table.declare("var", var.clone()).unwrap();
    table.declare("func", func.clone()).unwrap();

    // Compare inner values instead of Symbol wrappers
    assert_eq!(table.lookup_variable("var"), var_from_symbol(var.clone()));
    assert_eq!(
        table.lookup_function("func"),
        func_from_symbol(func.clone())
    );
}

#[test]
fn precise_error_span_reporting() {
    let mut table = SymbolTable::new();
    let span1 = create_span("test_file", 10, 20, 10, 30);
    let span2 = create_span("test_file", 15, 25, 15, 35);

    let first_var = Symbol::Variable(VariableSymbol {
        name: "x".into(),
        ty: int_type(),
        mutable: true,
        defined_at: span1.clone(),
        last_assignment: None,
    });

    let second_var = Symbol::Variable(VariableSymbol {
        name: "x".into(),
        ty: int_type(),
        mutable: false,
        defined_at: span2.clone(),
        last_assignment: None,
    });

    table.declare("x", first_var).unwrap();
    let err = table.declare("x", second_var).unwrap_err();

    match err {
        CompileError::TypeError { span, .. } => assert_eq!(span, span1),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn function_symbol_in_nested_scopes() {
    let mut table = SymbolTable::new();
    let global_func = create_func_symbol("foo");
    let local_func = create_func_symbol("foo");

    table.declare("foo", global_func.clone()).unwrap();
    table.push_scope(ScopeKind::Block, None);
    table.declare("foo", local_func.clone()).unwrap();

    // Compare inner function symbols
    assert_eq!(
        table.lookup_function("foo"),
        func_from_symbol(local_func.clone())
    );
    table.pop_scope();

    assert_eq!(
        table.lookup_function("foo"),
        func_from_symbol(global_func.clone())
    );
}

#[test]
fn lookup_specific_symbol_types() {
    let mut table = SymbolTable::new();
    let var = create_var_symbol("x", true);
    let func = create_func_symbol("y");

    table.declare("x", var.clone()).unwrap();
    table.declare("y", func.clone()).unwrap();

    // Compare inner values
    assert_eq!(table.lookup_variable("x"), var_from_symbol(var.clone()));
    assert_eq!(table.lookup_variable("y"), None);
    assert_eq!(table.lookup_function("x"), None);
    assert_eq!(table.lookup_function("y"), func_from_symbol(func.clone()));
}

#[test]
fn duplicate_variable_error_span() {
    let mut table = SymbolTable::new();
    let span1 = create_span("file", 1, 1, 1, 5);
    let span2 = create_span("file", 2, 1, 2, 5);

    let first_var = Symbol::Variable(VariableSymbol {
        name: "x".into(),
        ty: int_type(),
        mutable: true,
        defined_at: span1.clone(),
        last_assignment: None,
    });

    let second_var = Symbol::Variable(VariableSymbol {
        name: "x".into(),
        ty: int_type(),
        mutable: false,
        defined_at: span2.clone(),
        last_assignment: None,
    });

    table.declare("x", first_var).unwrap();
    let err = table.declare("x", second_var).unwrap_err();

    // Verify error type and that it uses the first declaration's span
    match err {
        CompileError::TypeError { message, span, help: _ } => {
            assert!(message.contains("Identifier 'x' already declared in this Global scope"));
            assert_eq!(span, span1);
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn duplicate_function_error_span() {
    let mut table = SymbolTable::new();
    let span1 = create_span("file", 5, 1, 5, 10);
    let span2 = create_span("file", 10, 1, 10, 10);

    let first_func = Symbol::Function(FunctionSymbol {
        name: "func".into(),
        parameters: Vec::new(),
        return_type: Type::Void,
        defined_at: span1.clone(),
    });

    let second_func = Symbol::Function(FunctionSymbol {
        name: "func".into(),
        parameters: Vec::new(),
        return_type: Type::Void,
        defined_at: span2.clone(),
    });

    table.declare("func", first_func).unwrap();
    let err = table.declare("func", second_func).unwrap_err();

    // Verify error type and that it uses the first declaration's span
    match err {
        CompileError::TypeError { message, span, help: _ } => {
            assert!(message.contains("Identifier 'func' already declared in this Global scope"));
            assert_eq!(span, span1);
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn duplicate_unknown_symbol_type_uses_default_span() {
    let mut table = SymbolTable::new();

    // Create an unknown symbol type (TypeAlias in this case)
    let unknown_symbol = Symbol::TypeAlias(Type::I32);

    // Create a variable symbol for duplicate declaration
    let var_symbol = Symbol::Variable(VariableSymbol {
        name: "x".into(),
        ty: int_type(),
        mutable: true,
        defined_at: create_span("file", 5, 1, 5, 2),
        last_assignment: None,
    });

    // Declare the unknown symbol
    table.declare("x", unknown_symbol).unwrap();

    // Attempt to declare duplicate symbol
    let err = table.declare("x", var_symbol).unwrap_err();

    // Verify error type and default span
    match err {
        CompileError::TypeError { message, span, help: _ } => {
            assert!(message.contains("Identifier 'x' already declared in this Global scope"));
            assert_eq!(span, SourceSpan::default());
        }
        _ => panic!("Expected TypeError with default span"),
    }
}

#[test]
fn test_current_scope_kind() {
    let mut table = SymbolTable::new();
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Global));

    table.push_scope(ScopeKind::Function, None);
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Function));

    table.push_scope(ScopeKind::Block, None);
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Block));

    table.pop_scope();
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Function));

    table.pop_scope();
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Global));
}

#[test]
fn test_current_function_return_type() {
    let mut table = SymbolTable::new();
    assert_eq!(table.current_function_return_type(), None);

    let void_func = FunctionSymbol {
        name: "void_func".into(),
        parameters: Vec::new(),
        return_type: Type::Void,
        defined_at: dummy_span(),
    };

    let int_func = FunctionSymbol {
        name: "int_func".into(),
        parameters: vec![Parameter {
            name: "arg".into(),
            type_annotation: Type::I32,
            span: dummy_span(),
        }],
        return_type: Type::I32,
        defined_at: dummy_span(),
    };

    table.enter_function(void_func);
    assert_eq!(table.current_function_return_type(), Some(Type::Void));

    table.exit_function();
    assert_eq!(table.current_function_return_type(), None);

    table.enter_function(int_func);
    assert_eq!(table.current_function_return_type(), Some(Type::I32));
}

#[test]
fn test_scope_stack_management() {
    let mut table = SymbolTable::new();
    assert_eq!(table.scope_count(), 1);

    table.push_scope(ScopeKind::Function, None);
    table.push_scope(ScopeKind::Block, None);
    assert_eq!(table.scope_count(), 3);

    table.pop_scope();
    assert_eq!(table.scope_count(), 2);
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Function));

    table.pop_scope();
    assert_eq!(table.scope_count(), 1);
    assert_eq!(table.current_scope_kind(), Some(ScopeKind::Global));

    // Try popping global scope (should be no-op)
    table.pop_scope();
    table.pop_scope();
    table.pop_scope();
    assert_eq!(table.scope_count(), 1);
}

#[test]
fn test_lookup_in_specific_scope() {
    let mut table = SymbolTable::new();
    let global_var = create_var_symbol("a", true);
    let func_var = create_var_symbol("b", false);
    let block_var = create_var_symbol("c", true);

    table.declare("a", global_var.clone()).unwrap();

    table.push_scope(ScopeKind::Function, None);
    table.declare("b", func_var.clone()).unwrap();

    table.push_scope(ScopeKind::Block, None);
    table.declare("c", block_var.clone()).unwrap();

    // Verify lookups in deepest scope
    assert_eq!(table.lookup("a"), Some(global_var.clone()));
    assert_eq!(table.lookup("b"), Some(func_var.clone()));
    assert_eq!(table.lookup("c"), Some(block_var.clone()));

    // Verify lookups after popping scopes
    table.pop_scope();
    assert_eq!(table.lookup("a"), Some(global_var.clone()));
    assert_eq!(table.lookup("b"), Some(func_var.clone()));
    assert_eq!(table.lookup("c"), None);

    table.pop_scope();
    assert_eq!(table.lookup("a"), Some(global_var));
    assert_eq!(table.lookup("b"), None);
    assert_eq!(table.lookup("c"), None);
}