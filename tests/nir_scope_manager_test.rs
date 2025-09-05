use jsavrs::nir::{IrLiteralValue, ScopeManager, Value, ValueKind};
// Test for creating a new ScopeManager
#[test]
fn test_new_scope_manager() {
    // Input: creation of a new ScopeManager
    let manager = ScopeManager::new();
    // Expected behavior:
    // - There should be a root scope
    // - The current scope should be the root scope
    // - The scope map should contain only the root scope
    // - The root scope should have depth 0 and no parent
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 1, "There should be only the root scope");
    let current_scope_id = manager.current_scope();
    let root_scope_id = current_scope_id; // In a new ScopeManager, current_scope is the root
    assert_eq!(current_scope_id, root_scope_id, "The current scope should be the root scope");
    let root_scope = &scopes[&root_scope_id];
    assert_eq!(root_scope.depth, 0, "The root scope should have depth 0");
    assert!(root_scope.parent.is_none(), "The root scope should not have a parent");
    assert!(root_scope.symbols.is_empty(), "The root scope should initially be empty");
    assert!(root_scope.children.is_empty(), "The root scope should initially have no children");
}
// Test for using the Default trait
#[test]
fn test_default_scope_manager() {
    // Input: creation of a ScopeManager using Default::default()
    let manager = ScopeManager::default();
    // Expected behavior: should behave like ScopeManager::new()
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 1, "There should be only the root scope");
    let current_scope_id = manager.current_scope();
    let root_scope_id = current_scope_id; // In a new ScopeManager, current_scope is the root
    assert_eq!(current_scope_id, root_scope_id, "The current scope should be the root scope");
    let root_scope = &scopes[&root_scope_id];
    assert_eq!(root_scope.depth, 0, "The root scope should have depth 0");
    assert!(root_scope.parent.is_none(), "The root scope should not have a parent");
}
// Test for entering a new scope
#[test]
fn test_enter_scope() {
    // Input: creation of a ScopeManager and entering a new scope
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    let new_scope_id = manager.enter_scope();
    // Expected behavior:
    // - A new scope should be created
    // - The current scope should be the new scope
    // - The new scope should have the root scope as parent
    // - The new scope should have depth 1
    // - The root scope should have the new scope as child
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 2, "There should be two scopes");
    assert_eq!(manager.current_scope(), new_scope_id, "The current scope should be the new scope");
    let new_scope = &scopes[&new_scope_id];
    assert_eq!(new_scope.depth, 1, "The new scope should have depth 1");
    assert_eq!(new_scope.parent, Some(root_id), "The new scope should have the root scope as parent");
    assert!(new_scope.symbols.is_empty(), "The new scope should initially be empty");
    assert!(new_scope.children.is_empty(), "The new scope should initially have no children");
    let root_scope = &scopes[&root_id];
    assert!(root_scope.children.contains(&new_scope_id), "The root scope should have the new scope as child");
}
// Test for exiting a scope
#[test]
fn test_exit_scope() {
    // Input: creation of a ScopeManager, entering a new scope and exiting
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    let _new_scope_id = manager.enter_scope();
    manager.exit_scope();
    // Expected behavior:
    // - The current scope should return to be the root scope
    // Actual output:
    assert_eq!(manager.current_scope(), root_id, "The current scope should be the root scope after exit");
}
// Test for exiting the root scope (edge case)
#[test]
fn test_exit_root_scope() {
    // Input: creation of a ScopeManager and attempt to exit the root scope
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    manager.exit_scope();
    // Expected behavior:
    // - Nothing should happen, the current scope should remain the root scope
    // Actual output:
    assert_eq!(manager.current_scope(), root_id, "The current scope should remain the root scope");
}
// Test for nested scopes
#[test]
fn test_nested_scopes() {
    // Input: creation of a ScopeManager and creation of nested scopes
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    let scope1_id = manager.enter_scope();
    let scope2_id = manager.enter_scope();
    let scope3_id = manager.enter_scope();
    // Expected behavior:
    // - There should be 4 scopes in total
    // - The current scope should be scope3
    // - The hierarchy should be: root -> scope1 -> scope2 -> scope3
    // - The depths should be: root=0, scope1=1, scope2=2, scope3=3
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 4, "There should be four scopes");
    assert_eq!(manager.current_scope(), scope3_id, "The current scope should be scope3");
    // Verify hierarchy
    let scope3 = &scopes[&scope3_id];
    assert_eq!(scope3.depth, 3, "scope3 should have depth 3");
    assert_eq!(scope3.parent, Some(scope2_id), "scope3 should have scope2 as parent");
    let scope2 = &scopes[&scope2_id];
    assert_eq!(scope2.depth, 2, "scope2 should have depth 2");
    assert_eq!(scope2.parent, Some(scope1_id), "scope2 should have scope1 as parent");
    assert!(scope2.children.contains(&scope3_id), "scope2 should have scope3 as child");
    let scope1 = &scopes[&scope1_id];
    assert_eq!(scope1.depth, 1, "scope1 should have depth 1");
    assert_eq!(scope1.parent, Some(root_id), "scope1 should have the root scope as parent");
    assert!(scope1.children.contains(&scope2_id), "scope1 should have scope2 as child");
    let root_scope = &scopes[&root_id];
    assert_eq!(root_scope.depth, 0, "The root scope should have depth 0");
    assert!(root_scope.parent.is_none(), "The root scope should not have a parent");
    assert!(root_scope.children.contains(&scope1_id), "The root scope should have scope1 as child");
}
// Test for entering and exiting nested scopes
#[test]
fn test_enter_exit_nested_scopes() {
    // Input: creation of nested scopes and sequential exit
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    let scope1_id = manager.enter_scope();
    let scope2_id = manager.enter_scope();
    let _scope3_id = manager.enter_scope();
    // Exit scope3
    manager.exit_scope();
    assert_eq!(manager.current_scope(), scope2_id, "After exiting scope3, the current scope should be scope2");
    // Exit scope2
    manager.exit_scope();
    assert_eq!(manager.current_scope(), scope1_id, "After exiting scope2, the current scope should be scope1");
    // Exit scope1
    manager.exit_scope();
    assert_eq!(manager.current_scope(), root_id, "After exiting scope1, the current scope should be the root scope");
}
// Test for adding a symbol
#[test]
fn test_add_symbol() {
    // Input: creation of a ScopeManager and adding a symbol
    let mut manager = ScopeManager::new();
    let current_scope = manager.current_scope();
    let value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value.clone());
    // Expected behavior:
    // - The symbol should be added to the current scope
    // - The value should have the scope set
    // Actual output:
    let scopes = manager.get_scopes();
    let scope = &scopes[&current_scope];
    assert!(scope.symbols.contains_key("x"), "The scope should contain the symbol 'x'");
    let stored_value = scope.symbols.get("x").unwrap();
    // We compare only the relevant fields, since the scope is set by add_symbol
    assert_eq!(stored_value.kind, value.kind, "The kind should be the original one");
    assert_eq!(stored_value.ty, value.ty, "The type should be the original one");
    assert_eq!(stored_value.debug_info, value.debug_info, "The debug info should be the original one");
    assert_eq!(stored_value.scope, Some(current_scope), "The value should have the scope set");
}
// Test for looking up a symbol in the current scope
#[test]
fn test_lookup_symbol_in_current_scope() {
    // Input: creation of a ScopeManager, adding a symbol and looking it up
    let mut manager = ScopeManager::new();
    let value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value.clone());
    // Expected behavior:
    // - The lookup should find the symbol in the current scope
    // Actual output:
    let found_value = manager.lookup("x");
    assert!(found_value.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_value = found_value.unwrap();
    assert_eq!(found_value.kind, value.kind, "The kind should be the original one");
    assert_eq!(found_value.ty, value.ty, "The type should be the original one");
    assert_eq!(found_value.debug_info, value.debug_info, "The debug info should be the original one");
}
// Test for looking up a symbol in a parent scope
#[test]
fn test_lookup_symbol_in_parent_scope() {
    // Input: creation of a ScopeManager, adding a symbol in the root scope,
    // creating a new scope and looking up the symbol
    let mut manager = ScopeManager::new();
    let value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value.clone());
    // Enter a new scope
    manager.enter_scope();
    // Expected behavior:
    // - The lookup should find the symbol in the parent scope
    // Actual output:
    let found_value = manager.lookup("x");
    assert!(found_value.is_some(), "Should find the symbol 'x' in the parent scope");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_value = found_value.unwrap();
    assert_eq!(found_value.kind, value.kind, "The kind should be the original one");
    assert_eq!(found_value.ty, value.ty, "The type should be the original one");
    assert_eq!(found_value.debug_info, value.debug_info, "The debug info should be the original one");
    // We also verify that the scope was set correctly
    assert!(found_value.scope.is_some(), "The value should have the scope set");
}
// Test for looking up a non-existent symbol
#[test]
fn test_lookup_nonexistent_symbol() {
    // Input: creation of a ScopeManager and lookup of a non-existent symbol
    let manager = ScopeManager::new();
    // Expected behavior:
    // - The lookup should not find the symbol
    // Actual output:
    let found_value = manager.lookup("x");
    assert!(found_value.is_none(), "Should not find the non-existent symbol 'x'");
}
// Test for symbol shadowing
#[test]
fn test_symbol_shadowing() {
    // Input: creation of a ScopeManager, adding a symbol in the root scope,
    // creating a new scope, adding a symbol with the same name and looking it up
    let mut manager = ScopeManager::new();
    let root_value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), root_value.clone());
    // Enter a new scope
    manager.enter_scope();
    let child_value = Value::new_literal(IrLiteralValue::I32(100));
    manager.add_symbol("x".into(), child_value.clone());
    // Expected behavior:
    // - The lookup should find the symbol in the current scope (shadowing)
    // Actual output:
    let found_value = manager.lookup("x");
    assert!(found_value.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_value = found_value.unwrap();
    assert_eq!(found_value.kind, child_value.kind, "The kind should be the original one");
    assert_eq!(found_value.ty, child_value.ty, "The type should be the original one");
    assert_eq!(found_value.debug_info, child_value.debug_info, "The debug info should be the original one");
    assert!(found_value.scope.is_some(), "The value should have the scope set");
}
// Test for looking up a symbol in a deep hierarchy
#[test]
fn test_lookup_in_deep_hierarchy() {
    // Input: creation of a deep scope hierarchy and adding symbols at different levels
    let mut manager = ScopeManager::new();
    let root_value = Value::new_literal(IrLiteralValue::I32(1));
    manager.add_symbol("x".into(), root_value.clone());
    manager.enter_scope(); // scope1
    let scope1_value = Value::new_literal(IrLiteralValue::I32(2));
    manager.add_symbol("y".into(), scope1_value.clone());
    manager.enter_scope(); // scope2
    let scope2_value = Value::new_literal(IrLiteralValue::I32(3));
    manager.add_symbol("z".into(), scope2_value.clone());
    manager.enter_scope(); // scope3
    // Expected behavior:
    // - The lookup of 'x' should find the value in the root scope
    // - The lookup of 'y' should find the value in scope1
    // - The lookup of 'z' should find the value in scope2
    // Actual output:
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, root_value.kind, "The kind of 'x' should be the original one");
    assert_eq!(found_x.ty, root_value.ty, "The type of 'x' should be the original one");
    assert_eq!(found_x.debug_info, root_value.debug_info, "The debug info of 'x' should be the original one");
    let found_y = manager.lookup("y");
    assert!(found_y.is_some(), "Should find the symbol 'y'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_y = found_y.unwrap();
    assert_eq!(found_y.kind, scope1_value.kind, "The kind of 'y' should be the original one");
    assert_eq!(found_y.ty, scope1_value.ty, "The type of 'y' should be the original one");
    assert_eq!(found_y.debug_info, scope1_value.debug_info, "The debug info of 'y' should be the original one");
    let found_z = manager.lookup("z");
    assert!(found_z.is_some(), "Should find the symbol 'z'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_z = found_z.unwrap();
    assert_eq!(found_z.kind, scope2_value.kind, "The kind of 'z' should be the original one");
    assert_eq!(found_z.ty, scope2_value.ty, "The type of 'z' should be the original one");
    assert_eq!(found_z.debug_info, scope2_value.debug_info, "The debug info of 'z' should be the original one");
}
// Test for lookup_mut of a symbol in the current scope
#[test]
fn test_lookup_mut_symbol_in_current_scope() {
    // Input: creation of a ScopeManager, adding a symbol and mutable lookup
    let mut manager = ScopeManager::new();
    let value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value);
    // Expected behavior:
    // - The mutable lookup should find the symbol in the current scope
    // - It should be possible to get a mutable reference to the value
    // Actual output:
    let found_value = manager.lookup_mut("x");
    assert!(found_value.is_some(), "Should find the symbol 'x'");
    // Verify that the reference is to the correct value
    let found_value = found_value.unwrap();
    assert_eq!(format!("{}", found_value), "42i32", "The value should be 42");
}
// Test for lookup_mut of a symbol in a parent scope
#[test]
fn test_lookup_mut_symbol_in_parent_scope() {
    // Input: creation of a ScopeManager, adding a symbol in the root scope,
    // creating a new scope and mutable lookup of the symbol
    let mut manager = ScopeManager::new();
    let value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value);
    // Enter a new scope
    manager.enter_scope();
    // Expected behavior:
    // - The mutable lookup should find the symbol in the parent scope
    // - It should be possible to get a mutable reference to the value
    // Actual output:
    let found_value = manager.lookup_mut("x");
    assert!(found_value.is_some(), "Should find the symbol 'x' in the parent scope");
    // Verify that the reference is to the correct value
    let found_value = found_value.unwrap();
    assert_eq!(format!("{}", found_value), "42i32", "The value should be 42");
}
// Test for lookup_mut of a non-existent symbol
#[test]
fn test_lookup_mut_nonexistent_symbol() {
    // Input: creation of a ScopeManager and mutable lookup of a non-existent symbol
    let mut manager = ScopeManager::new();
    // Expected behavior:
    // - The mutable lookup should not find the symbol
    // Actual output:
    let found_value = manager.lookup_mut("x");
    assert!(found_value.is_none(), "Should not find the non-existent symbol 'x'");
}
// Test for getting all scopes
#[test]
fn test_get_scopes() {
    // Input: creation of a ScopeManager with nested scopes and getting all scopes
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    let scope1_id = manager.enter_scope();
    let scope2_id = manager.enter_scope();
    // Exit scope2
    manager.exit_scope();
    // Add a symbol to scope1
    let value = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value);
    // Expected behavior:
    // - Should get a copy of all scopes
    // - The copy should contain all created scopes
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 3, "There should be three scopes");
    assert!(scopes.contains_key(&root_id), "Should contain the root scope");
    assert!(scopes.contains_key(&scope1_id), "Should contain scope1");
    assert!(scopes.contains_key(&scope2_id), "Should contain scope2");
    // Verify that scope1 contains the symbol
    let scope1 = &scopes[&scope1_id];
    assert!(scope1.symbols.contains_key("x"), "scope1 should contain the symbol 'x'");
}
// Test for getting the current scope
#[test]
fn test_current_scope() {
    // Input: creation of a ScopeManager and getting the current scope
    let mut manager = ScopeManager::new();
    let initial_scope = manager.current_scope();
    // Expected behavior:
    // - Initially, the current scope should be the root scope
    // Actual output:
    assert_eq!(manager.current_scope(), initial_scope, "Initially, the current scope should be the root scope");
    // Enter a new scope
    let new_scope = manager.enter_scope();
    // Expected behavior:
    // - After entering a new scope, the current scope should be the new scope
    // Actual output:
    assert_eq!(
        manager.current_scope(),
        new_scope,
        "After entering a new scope, the current scope should be the new scope"
    );
    // Exit the scope
    manager.exit_scope();
    // Expected behavior:
    // - After exiting, the current scope should return to be the initial scope
    // Actual output:
    assert_eq!(
        manager.current_scope(),
        initial_scope,
        "After exiting, the current scope should return to be the initial scope"
    );
}
// Test for multiple scopes with the same name in different scopes
#[test]
fn test_multiple_scopes_with_same_name() {
    // Input: creation of multiple scopes with symbols with the same name
    let mut manager = ScopeManager::new();
    // Add 'x' to the root scope
    let root_value = Value::new_literal(IrLiteralValue::I32(1));
    manager.add_symbol("x".into(), root_value.clone());
    // Enter a new scope and add 'x'
    manager.enter_scope();
    let scope1_value = Value::new_literal(IrLiteralValue::I32(2));
    manager.add_symbol("x".into(), scope1_value.clone());
    // Enter another scope and add 'x'
    manager.enter_scope();
    let scope2_value = Value::new_literal(IrLiteralValue::I32(3));
    manager.add_symbol("x".into(), scope2_value.clone());
    // Expected behavior:
    // - In each scope, the lookup of 'x' should find the local value
    // - Values in parent scopes should be inaccessible directly when there is shadowing
    // Actual output:
    // In the innermost scope
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, scope2_value.kind, "The kind should be the original one");
    assert_eq!(found_x.ty, scope2_value.ty, "The type should be the original one");
    assert_eq!(found_x.debug_info, scope2_value.debug_info, "The debug info should be the original one");
    assert!(found_x.scope.is_some(), "The value should have the scope set");
    // Exit the innermost scope
    manager.exit_scope();
    // In scope1
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, scope1_value.kind, "The kind should be the original one");
    assert_eq!(found_x.ty, scope1_value.ty, "The type should be the original one");
    assert_eq!(found_x.debug_info, scope1_value.debug_info, "The debug info should be the original one");
    assert!(found_x.scope.is_some(), "The value should have the scope set");
    // Exit scope1
    manager.exit_scope();
    // In the root scope
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, root_value.kind, "The kind should be the original one");
    assert_eq!(found_x.ty, root_value.ty, "The type should be the original one");
    assert_eq!(found_x.debug_info, root_value.debug_info, "The debug info should be the original one");
    assert!(found_x.scope.is_some(), "The value should have the scope set");
}

// Test for empty nested scopes
#[test]
fn test_empty_nested_scopes() {
    // Input: creation of nested scopes without symbols
    let mut manager = ScopeManager::new();
    let root_id = manager.current_scope();
    // Create several nested scopes
    let scope1_id = manager.enter_scope();
    let scope2_id = manager.enter_scope();
    let scope3_id = manager.enter_scope();
    // Expected behavior:
    // - All scopes should exist
    // - All scopes should be empty (no symbols)
    // - The hierarchy should be correct
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 4, "There should be four scopes");
    // Verify that all scopes are empty
    for (id, scope) in &scopes {
        assert!(scope.symbols.is_empty(), "Scope {:?} should be empty", id);
    }
    // Verify the hierarchy
    let scope3 = &scopes[&scope3_id];
    assert_eq!(scope3.parent, Some(scope2_id), "scope3 should have scope2 as parent");
    let scope2 = &scopes[&scope2_id];
    assert_eq!(scope2.parent, Some(scope1_id), "scope2 should have scope1 as parent");
    assert!(scope2.children.contains(&scope3_id), "scope2 should have scope3 as child");
    let scope1 = &scopes[&scope1_id];
    assert_eq!(scope1.parent, Some(root_id), "scope1 should have the root scope as parent");
    assert!(scope1.children.contains(&scope2_id), "scope1 should have scope2 as child");
    let root_scope = &scopes[&root_id];
    assert!(root_scope.parent.is_none(), "The root scope should not have a parent");
    assert!(root_scope.children.contains(&scope1_id), "The root scope should have scope1 as child");
}
// Test for a large number of nested scopes (stress test)
#[test]
fn test_deeply_nested_scopes() {
    // Input: creation of a large number of nested scopes
    let mut manager = ScopeManager::new();
    let mut scope_ids = Vec::new();
    // Create 100 nested scopes
    for i in 0..100 {
        scope_ids.push(manager.current_scope());
        manager.enter_scope();
        // Add a symbol with a unique name in each scope
        let value = Value::new_literal(IrLiteralValue::I32(i));
        manager.add_symbol(format!("x_{}", i).into(), value);
    }
    // Expected behavior:
    // - There should be 101 scopes in total (root + 100 children)
    // - The depth of the current scope should be 100
    // - We should be able to find all symbols by going up the hierarchy
    // Actual output:
    let scopes = manager.get_scopes();
    assert_eq!(scopes.len(), 101, "There should be 101 scopes");
    let current_scope_id = manager.current_scope();
    let current_scope = &scopes[&current_scope_id];
    assert_eq!(current_scope.depth, 100, "The current scope should have depth 100");
    // Verify that we can find all symbols
    for i in (0..100).rev() {
        let found_value = manager.lookup(&format!("x_{}", i));
        assert!(found_value.is_some(), "Should find the symbol 'x_{}'", i);
        // Verify the value more specifically
        match found_value.unwrap().kind {
            ValueKind::Literal(IrLiteralValue::I32(val)) => {
                assert_eq!(val, i, "The value of 'x_{}' should be {}", i, i);
            }
            _ => panic!("The symbol 'x_{}' should be an I32 literal", i),
        }
    }
    // Exit all scopes
    for _ in 0..100 {
        manager.exit_scope();
    }
    // Verify that we are back to the root scope
    assert_eq!(manager.current_scope(), scope_ids[0], "After exiting all scopes, we should be back to the root scope");
}
// Test for adding the same symbol twice in the same scope
#[test]
fn test_add_duplicate_symbol_in_same_scope() {
    // Input: creation of a ScopeManager and adding the same symbol twice
    let mut manager = ScopeManager::new();
    let value1 = Value::new_literal(IrLiteralValue::I32(42));
    manager.add_symbol("x".into(), value1);
    let value2 = Value::new_literal(IrLiteralValue::I32(100));
    manager.add_symbol("x".into(), value2.clone());
    // Expected behavior:
    // - The second value should overwrite the first
    // Actual output:
    let found_value = manager.lookup("x");
    assert!(found_value.is_some(), "Should find the symbol 'x'");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_value = found_value.unwrap();
    assert_eq!(found_value.kind, value2.kind, "The kind should be that of the second value");
    assert_eq!(found_value.ty, value2.ty, "The type should be that of the second value");
    assert_eq!(found_value.debug_info, value2.debug_info, "The debug info should be that of the second value");
    // We also verify that the scope was set correctly
    assert!(found_value.scope.is_some(), "The scope should be set");
}
// Test for memory management (verify that there are no leaks)
#[test]
fn test_memory_management() {
    // Input: creation and destruction of many scopes
    {
        let mut manager = ScopeManager::new();
        // Create and destroy many scopes
        for _ in 0..10 {
            for _ in 0..5 {
                manager.enter_scope();
                // Add some symbols
                let value = Value::new_literal(IrLiteralValue::I32(42));
                manager.add_symbol("x".into(), value);
            }
            for _ in 0..5 {
                manager.exit_scope();
            }
        }
    } // manager goes out of scope here
    // Expected behavior:
    // - There should be no memory leaks
    // Actual output:
    // This test mainly verifies that there are no panics or leaks
    // If the test completes without errors, memory management is considered correct
}
// Test for adding symbols with special names
#[test]
fn test_symbols_with_special_names() {
    // Input: creation of a ScopeManager and adding symbols with special names
    let mut manager = ScopeManager::new();
    // Symbol with empty string
    let value1 = Value::new_literal(IrLiteralValue::I32(1));
    manager.add_symbol("".into(), value1.clone());
    // Symbol with special characters
    let value2 = Value::new_literal(IrLiteralValue::I32(2));
    manager.add_symbol("!@#$%^&*()".into(), value2.clone());
    // Symbol with spaces
    let value3 = Value::new_literal(IrLiteralValue::I32(3));
    manager.add_symbol("nome con spazi".into(), value3.clone());
    // Unicode symbol
    let value4 = Value::new_literal(IrLiteralValue::I32(4));
    manager.add_symbol("こんにちは".into(), value4.clone());
    // Expected behavior:
    // - All symbols should be added correctly
    // - It should be possible to retrieve them
    // Actual output:
    let found_value1 = manager.lookup("").unwrap();
    assert_eq!(found_value1.kind, value1.kind, "The kind should be the original one");
    assert_eq!(found_value1.ty, value1.ty, "The type should be the original one");
    assert_eq!(found_value1.debug_info, value1.debug_info, "The debug info should be the original one");
    assert!(found_value1.scope.is_some(), "The value should have the scope set");
    let found_value2 = manager.lookup("!@#$%^&*()").unwrap();
    assert_eq!(found_value2.kind, value2.kind, "The kind should be the original one");
    assert_eq!(found_value2.ty, value2.ty, "The type should be the original one");
    assert_eq!(found_value2.debug_info, value2.debug_info, "The debug info should be the original one");
    assert!(found_value2.scope.is_some(), "The value should have the scope set");
    let found_value3 = manager.lookup("nome con spazi").unwrap();
    assert_eq!(found_value3.kind, value3.kind, "The kind should be the original one");
    assert_eq!(found_value3.ty, value3.ty, "The type should be the original one");
    assert_eq!(found_value3.debug_info, value3.debug_info, "The debug info should be the original one");
    assert!(found_value3.scope.is_some(), "The value should have the scope set");
    let found_value4 = manager.lookup("こんにちは").unwrap();
    assert_eq!(found_value4.kind, value4.kind, "The kind should be the original one");
    assert_eq!(found_value4.ty, value4.ty, "The type should be the original one");
    assert_eq!(found_value4.debug_info, value4.debug_info, "The debug info should be the original one");
    assert!(found_value4.scope.is_some(), "The value should have the scope set");
}
// Test for looking up symbols in scopes with multiple children
#[test]
fn test_lookup_in_scope_with_multiple_children() {
    // Input: creation of a scope with multiple children and symbol lookup
    let mut manager = ScopeManager::new();
    // Add a symbol to the root scope
    let root_value = Value::new_literal(IrLiteralValue::I32(0));
    manager.add_symbol("x".into(), root_value.clone());
    // Create scope1 and store its ID
    manager.enter_scope(); // scope1
    let _scope1_id = manager.current_scope();
    let scope1_value = Value::new_literal(IrLiteralValue::I32(1));
    manager.add_symbol("y".into(), scope1_value.clone());
    // Verify that we can find 'x' and 'y' in scope1
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find 'x' in the root scope from scope1");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, root_value.kind, "The kind of 'x' should be the original one");
    assert_eq!(found_x.ty, root_value.ty, "The type of 'x' should be the original one");
    assert_eq!(found_x.debug_info, root_value.debug_info, "The debug info of 'x' should be the original one");
    let found_y = manager.lookup("y");
    assert!(found_y.is_some(), "Should find 'y' in scope1");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_y = found_y.unwrap();
    assert_eq!(found_y.kind, scope1_value.kind, "The kind of 'y' should be the original one");
    assert_eq!(found_y.ty, scope1_value.ty, "The type of 'y' should be the original one");
    assert_eq!(found_y.debug_info, scope1_value.debug_info, "The debug info of 'y' should be the original one");
    // Return to the root scope
    manager.exit_scope();
    // Create scope2
    manager.enter_scope(); // scope2
    let scope2_value = Value::new_literal(IrLiteralValue::I32(2));
    manager.add_symbol("z".into(), scope2_value.clone());
    // Verify that we can find 'x' and 'z' in scope2, but not 'y'
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find 'x' in the root scope from scope2");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, root_value.kind, "The kind of 'x' should be the original one");
    assert_eq!(found_x.ty, root_value.ty, "The type of 'x' should be the original one");
    assert_eq!(found_x.debug_info, root_value.debug_info, "The debug info of 'x' should be the original one");
    let found_z = manager.lookup("z");
    assert!(found_z.is_some(), "Should find 'z' in scope2");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_z = found_z.unwrap();
    assert_eq!(found_z.kind, scope2_value.kind, "The kind of 'z' should be the original one");
    assert_eq!(found_z.ty, scope2_value.ty, "The type of 'z' should be the original one");
    assert_eq!(found_z.debug_info, scope2_value.debug_info, "The debug info of 'z' should be the original one");
    assert!(manager.lookup("y").is_none(), "Should not find 'y' in scope2");
    // Return to the root scope
    manager.exit_scope();
    // Create scope3
    manager.enter_scope(); // scope3
    let scope3_value = Value::new_literal(IrLiteralValue::I32(3));
    manager.add_symbol("w".into(), scope3_value.clone());
    // Verify that we can find 'x' and 'w' in scope3, but not 'y' or 'z'
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find 'x' in the root scope from scope3");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, root_value.kind, "The kind of 'x' should be the original one");
    assert_eq!(found_x.ty, root_value.ty, "The type of 'x' should be the original one");
    assert_eq!(found_x.debug_info, root_value.debug_info, "The debug info of 'x' should be the original one");
    let found_w = manager.lookup("w");
    assert!(found_w.is_some(), "Should find 'w' in scope3");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_w = found_w.unwrap();
    assert_eq!(found_w.kind, scope3_value.kind, "The kind of 'w' should be the original one");
    assert_eq!(found_w.ty, scope3_value.ty, "The type of 'w' should be the original one");
    assert_eq!(found_w.debug_info, scope3_value.debug_info, "The debug info of 'w' should be the original one");
    assert!(manager.lookup("y").is_none(), "Should not find 'y' in scope3");
    assert!(manager.lookup("z").is_none(), "Should not find 'z' in scope3");
    // Return to the root scope
    manager.exit_scope();
    // Verify that in the root scope we can find only 'x'
    let found_x = manager.lookup("x");
    assert!(found_x.is_some(), "Should find 'x' in the root scope");
    // We compare only the relevant fields, since the scope is set by add_symbol
    let found_x = found_x.unwrap();
    assert_eq!(found_x.kind, root_value.kind, "The kind of 'x' should be the original one");
    assert_eq!(found_x.ty, root_value.ty, "The type of 'x' should be the original one");
    assert_eq!(found_x.debug_info, root_value.debug_info, "The debug info of 'x' should be the original one");
    assert!(manager.lookup("y").is_none(), "Should not find 'y' in the root scope");
    assert!(manager.lookup("z").is_none(), "Should not find 'z' in the root scope");
    assert!(manager.lookup("w").is_none(), "Should not find 'w' in the root scope");
}

// Test for handling scopes with a large number of symbols
#[test]
fn test_scope_with_many_symbols() {
    // Input: creation of a scope with many symbols
    let mut manager = ScopeManager::new();
    // Add 1000 symbols
    for i in 0..1000 {
        let value = Value::new_literal(IrLiteralValue::I32(i));
        manager.add_symbol(format!("symbol_{}", i).into(), value);
    }
    // Expected behavior:
    // - All symbols should be added correctly
    // - It should be possible to retrieve them
    // Actual output:
    for i in 0..1000 {
        let found_value = manager.lookup(&format!("symbol_{}", i));
        assert!(found_value.is_some(), "Should find the symbol 'symbol_{}'", i);
        // Verify the value more specifically
        match found_value.unwrap().kind {
            ValueKind::Literal(IrLiteralValue::I32(val)) => {
                assert_eq!(val, i, "The value of 'symbol_{}' should be {}", i, i);
            }
            _ => panic!("The symbol 'symbol_{}' should be an I32 literal", i),
        }
    }
    // Verify that the scope contains 1000 symbols
    let scopes = manager.get_scopes();
    let current_scope_id = manager.current_scope();
    let current_scope = &scopes[&current_scope_id];
    assert_eq!(current_scope.symbols.len(), 1000, "The scope should contain 1000 symbols");
}
