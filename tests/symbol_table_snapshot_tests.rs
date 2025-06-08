use insta::assert_debug_snapshot;
use jsavrs::error::compile_error::CompileError;
use jsavrs::parser::ast::{Parameter, Type};
use jsavrs::semantic::symbol_table::{FunctionSymbol, Symbol, SymbolTable, VariableSymbol};
use jsavrs::utils::{create_func_symbol, create_span, create_var_symbol, dummy_span};

#[test]
fn global_scope_declaration_and_lookup() {
    let mut table = SymbolTable::new();
    let var_symbol = create_var_symbol("x", true);

    table.declare("x", var_symbol.clone()).unwrap();
    assert_debug_snapshot!(table.lookup("x"));
}

#[test]
fn shadowing_across_scopes() {
    let mut table = SymbolTable::new();
    let global_var = create_var_symbol("x", false);
    let local_var = create_var_symbol("x", true);

    table.declare("x", global_var.clone()).unwrap();
    table.push_scope();
    table.declare("x", local_var.clone()).unwrap();

    // Should find local variable in inner scope
    assert_debug_snapshot!(table.lookup("x"));
    table.pop_scope();

    // Should find global variable after popping scope
    assert_debug_snapshot!(table.lookup("x"));
}

#[test]
fn duplicate_declaration_same_scope() {
    let mut table = SymbolTable::new();
    let var1 = create_var_symbol("x", true);
    let var2 = create_var_symbol("x", false);

    table.declare("x", var1).unwrap();
    let result = table.declare("x", var2);

    // Fixed pattern matching without guard
    assert_debug_snapshot!(result);
}

#[test]
fn function_symbol_tracking() {
    let mut table = SymbolTable::new();
    let func = FunctionSymbol {
        name: "foo".to_string(),
        parameters: vec![Parameter {
            name: "arg".to_string(),
            type_annotation: Type::I8,  // Use correct field name
            span: dummy_span(),
        }],
        return_type: Type::Void,
        defined_at: dummy_span(),
    };

    assert_debug_snapshot!(table.current_function());

    table.enter_function(func.clone());
    assert_debug_snapshot!(table.current_function());

    table.exit_function();
    assert_debug_snapshot!(table.current_function());
}

#[test]
fn lookup_non_existent_symbol() {
    let table = SymbolTable::new();
    assert_debug_snapshot!(table.lookup("ghost"));
    assert_debug_snapshot!(table.lookup_variable("phantom"));
    assert_debug_snapshot!(table.lookup_function("specter"));
}

#[test]
fn scope_isolation() {
    let mut table = SymbolTable::new();
    table.declare("a", create_var_symbol("a", true)).unwrap();

    table.push_scope();
    assert_debug_snapshot!(table.lookup("a")); // Can see parent
    table.declare("b", create_var_symbol("b", false)).unwrap();

    table.push_scope();
    assert_debug_snapshot!(table.lookup("b")); // Can see grandparent

    table.pop_scope();
    table.pop_scope();
    assert_debug_snapshot!(table.lookup("b")); // b not in global scope
}

#[test]
fn cannot_pop_global_scope() {
    let mut table = SymbolTable::new();
    table.pop_scope(); // Should not panic
    table.pop_scope(); // Multiple pops should be safe
    assert_debug_snapshot!(table.scopes.len());
}

#[test]
fn mixed_symbol_types() {
    let mut table = SymbolTable::new();
    let var = create_var_symbol("var", true);
    let func = create_func_symbol("func");

    table.declare("var", var.clone()).unwrap();
    table.declare("func", func.clone()).unwrap();

    // Compare inner values instead of Symbol wrappers
    assert_debug_snapshot!(table.lookup_variable("var"));
    assert_debug_snapshot!(table.lookup_function("func"));
}

#[test]
fn precise_error_span_reporting() {
    let mut table = SymbolTable::new();
    let span1 = create_span("test_file", 10, 20, 10, 30);
    let span2 = create_span("test_file", 15, 25, 15, 35);

    let first_var = Symbol::Variable(VariableSymbol {
        name: "x".to_string(),
        ty: int_type(),
        mutable: true,
        defined_at: span1.clone(),
        last_assignment: None,
    });

    let second_var = Symbol::Variable(VariableSymbol {
        name: "x".to_string(),
        ty: int_type(),
        mutable: false,
        defined_at: span2.clone(),
        last_assignment: None,
    });

    table.declare("x", first_var).unwrap();
    assert_debug_snapshot!(table.declare("x", second_var).unwrap_err());
}

#[test]
fn function_symbol_in_nested_scopes() {
    let mut table = SymbolTable::new();
    let global_func = create_func_symbol("foo");
    let local_func = create_func_symbol("foo");

    table.declare("foo", global_func.clone()).unwrap();
    table.push_scope();
    table.declare("foo", local_func.clone()).unwrap();

    // Compare inner function symbols
    assert_debug_snapshot!(table.lookup_function("foo"));
    table.pop_scope();

    assert_debug_snapshot!(table.lookup_function("foo"));
}

#[test]
fn lookup_specific_symbol_types() {
    let mut table = SymbolTable::new();
    let var = create_var_symbol("x", true);
    let func = create_func_symbol("y");

    table.declare("x", var.clone()).unwrap();
    table.declare("y", func.clone()).unwrap();

    // Compare inner values
    assert_debug_snapshot!(table.lookup_variable("x"));
    assert_debug_snapshot!(table.lookup_variable("y"));
    assert_debug_snapshot!(table.lookup_function("x"));
    assert_debug_snapshot!(table.lookup_function("y"));
}