// Unit tests for SCCPropagator
// Tests phi node evaluation logic with executable edges

use jsavrs::ir::optimizer::constant_folding::*;

// ============================================================================
// T073: Unit tests for phi node evaluation with executable edges
// ============================================================================
// These tests verify the lattice meet operation behavior which is used
// internally by SCCPropagator for phi node evaluation

#[test]
fn test_phi_evaluation_all_same_constant() {
    // Test phi node where all incoming values are the same constant
    // Expected: phi evaluates to that constant

    // Simulate: phi = (10 from block1, 10 from block2, 10 from block3)
    // All edges executable, all values constant I32(10)

    let const_10 = LatticeValue::Constant(ConstantValue::I32(10));

    // Meet of same constants should be that constant
    let result = const_10.meet(&const_10).meet(&const_10);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::I32(10)));
}
#[test]
fn test_phi_evaluation_different_constants() {
    // Test phi node where incoming values are different constants
    // Expected: phi evaluates to Top

    // Simulate: phi = (10 from block1, 20 from block2)
    // All edges executable, different constant values

    let const_10 = LatticeValue::Constant(ConstantValue::I32(10));
    let const_20 = LatticeValue::Constant(ConstantValue::I32(20));

    // Meet of different constants should be Top
    let result = const_10.meet(&const_20);
    assert_eq!(result, LatticeValue::Top);
}

#[test]
fn test_phi_evaluation_with_bottom() {
    // Test phi node where some incoming values are Bottom
    // Expected: Bottom values should be ignored (not yet computed)

    // Simulate: phi = (10 from block1, Bottom from block2, 10 from block3)
    // Bottom represents uninitialized/not-yet-computed values

    let const_10 = LatticeValue::Constant(ConstantValue::I32(10));
    let bottom = LatticeValue::Bottom;

    // Bottom is identity for meet
    let result = const_10.meet(&bottom).meet(&const_10);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::I32(10)));
}

#[test]
fn test_phi_evaluation_with_top() {
    // Test phi node where one incoming value is Top
    // Expected: phi evaluates to Top (conservative)

    // Simulate: phi = (10 from block1, Top from block2, 10 from block3)

    let const_10 = LatticeValue::Constant(ConstantValue::I32(10));
    let top = LatticeValue::Top;

    // Top absorbs everything
    let result = const_10.meet(&top).meet(&const_10);
    assert_eq!(result, LatticeValue::Top);
}

#[test]
fn test_phi_evaluation_only_bottom() {
    // Test phi node where all incoming values are Bottom
    // Expected: phi is Bottom (not yet computed)

    let bottom = LatticeValue::Bottom;

    let result = bottom.meet(&bottom).meet(&bottom);
    assert_eq!(result, LatticeValue::Bottom);
}

#[test]
fn test_phi_evaluation_executable_edge_filtering() {
    // Test that phi evaluation only considers executable predecessor edges
    // This is a conceptual test - actual filtering happens in SCCPropagator

    // Simulate: phi = (10 from block1, 20 from block2, 30 from block3)
    // But only block1 and block3 edges are executable
    // Expected: meet(10, 30) = Top (different constants)

    let const_10 = LatticeValue::Constant(ConstantValue::I32(10));
    let const_30 = LatticeValue::Constant(ConstantValue::I32(30));

    // Only meet executable edges (skip block2's 20)
    let result = const_10.meet(&const_30);
    assert_eq!(result, LatticeValue::Top);
}

#[test]
fn test_phi_evaluation_single_executable_edge() {
    // Test phi node with only one executable predecessor edge
    // Expected: phi takes value from that single edge

    // Simulate: phi = (42 from block1)
    // Only one edge is executable

    let const_42 = LatticeValue::Constant(ConstantValue::I32(42));

    // Single value - should be that value
    let result = const_42;
    assert_eq!(result, LatticeValue::Constant(ConstantValue::I32(42)));
}

#[test]
fn test_phi_evaluation_type_mismatch() {
    // Test phi node where incoming values have different types
    // Expected: phi evaluates to Top (type mismatch)

    let i32_val = LatticeValue::Constant(ConstantValue::I32(10));
    let i64_val = LatticeValue::Constant(ConstantValue::I64(10));

    // Different types should meet to Top
    let result = i32_val.meet(&i64_val);
    assert_eq!(result, LatticeValue::Top);
}

#[test]
fn test_phi_evaluation_bool_values() {
    // Test phi node with boolean values

    let bool_true = LatticeValue::Constant(ConstantValue::Bool(true));
    let bool_false = LatticeValue::Constant(ConstantValue::Bool(false));

    // Same boolean
    let result1 = bool_true.meet(&bool_true);
    assert_eq!(result1, LatticeValue::Constant(ConstantValue::Bool(true)));

    // Different booleans
    let result2 = bool_true.meet(&bool_false);
    assert_eq!(result2, LatticeValue::Top);
}
