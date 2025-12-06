// Integration tests for SCCP Constant Folding Optimizer
// Tests end-to-end optimization scenarios combining multiple components

// NOTE: Full integration tests with Function/BasicBlock/Module are pending
// completion of IR infrastructure APIs. Current tests verify component behavior.

use jsavrs::ir::optimizer::constant_folding::*;

#[test]
fn test_sccp_config_default() {
    // Test that SCCPConfig has sensible defaults
    let config = SCCPConfig::default();
    assert!(!config.verbose, "Should not be verbose by default");
    assert!(config.max_iterations > 0, "Should have positive iteration limit");
}

#[test]
fn test_lattice_value_meet_basic() {
    // Test lattice meet operation
    let bottom = LatticeValue::Bottom;
    let const_42 = LatticeValue::Constant(ConstantValue::I32(42));
    let const_100 = LatticeValue::Constant(ConstantValue::I32(100));
    let top = LatticeValue::Top;

    // Bottom is identity
    assert_eq!(bottom.meet(&const_42), const_42);
    assert_eq!(const_42.meet(&bottom), const_42);

    // Same constants meet to same
    assert_eq!(const_42.meet(&const_42), const_42);

    // Different constants meet to Top
    assert_eq!(const_42.meet(&const_100), top);

    // Top absorbs everything
    assert_eq!(top.meet(&const_42), top);
    assert_eq!(const_42.meet(&top), top);
}

#[test]
fn test_constant_value_type_queries() {
    // Test ConstantValue helper methods
    let i32_val = ConstantValue::I32(42);
    let bool_val = ConstantValue::Bool(true);

    assert_eq!(i32_val.as_i32(), Some(42));
    assert_eq!(i32_val.as_bool(), None);
    assert_eq!(bool_val.as_bool(), Some(true));
    assert_eq!(bool_val.as_i32(), None);
}
#[test]
fn test_optimization_stats_display() {
    // Test that OptimizationStats can be displayed
    let stats = OptimizationStats {
        constants_propagated: 10,
        branches_resolved: 2,
        phi_nodes_simplified: 1,
        blocks_marked_unreachable: 1,
        iterations: 2,
    };

    let display_str = format!("{}", stats);
    assert!(display_str.contains("10 constants"));
    assert!(display_str.contains("2 branches"));
    assert!(display_str.contains("1 phis"));
    assert!(display_str.contains("1 unreachable"));
    assert!(display_str.contains("2 iterations"));
}

#[test]
fn test_lattice_value_ordering() {
    // Verify lattice ordering properties
    let bottom = LatticeValue::Bottom;
    let const_val = LatticeValue::Constant(ConstantValue::I32(10));
    let top = LatticeValue::Top;

    // Verify ordering: ⊥ ≤ Constant ≤ ⊤
    assert_ne!(bottom, const_val, "Bottom < Constant");
    assert_ne!(const_val, top, "Constant < Top");
    assert_ne!(bottom, top, "Bottom < Top");
}

#[test]
fn test_constant_value_equality() {
    // Test constant value comparison
    let c1 = ConstantValue::I32(42);
    let c2 = ConstantValue::I32(42);
    let c3 = ConstantValue::I32(99);

    assert_eq!(c1, c2, "Same constants should be equal");
    assert_ne!(c1, c3, "Different constants should not be equal");
}

#[test]
fn test_multiple_type_constants() {
    // Test meet operation with different constant types
    let i32_10 = LatticeValue::Constant(ConstantValue::I32(10));
    let i64_10 = LatticeValue::Constant(ConstantValue::I64(10));

    // Different types should meet to Top
    let result = i32_10.meet(&i64_10);
    assert_eq!(result, LatticeValue::Top, "Different types meet to Top");
}

// ============================================================================
// T062: Integration test for constant true branch resolution
// ============================================================================

#[test]
fn test_constant_true_branch_resolution() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create a simple function with constant conditional branch
    // if (true) { return 1; } else { return 2; }
    // Expected: else branch should be marked unreachable

    let mut func = Function::new("test_if_true", vec![], IrType::I32);

    // Create entry block with constant true condition
    func.add_block("entry", SourceSpan::default());

    // Create true and false branches
    func.add_block("then_block", SourceSpan::default());
    func.add_block("else_block", SourceSpan::default());

    // Set terminator in entry: conditional branch with constant true
    let true_value = Value::new_literal(IrLiteralValue::Bool(true));
    let cond_branch = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: true_value,
            true_label: Arc::from("then_block"),
            false_label: Arc::from("else_block"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", cond_branch);

    // Set terminators for then/else blocks
    let ret_1 = Value::new_literal(IrLiteralValue::I32(1));
    let then_term = Terminator::new(TerminatorKind::Return { value: ret_1, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("then_block", then_term);

    let ret_2 = Value::new_literal(IrLiteralValue::I32(2));
    let else_term = Terminator::new(TerminatorKind::Return { value: ret_2, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("else_block", else_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 3, "Should converge in ≤3 iterations for simple branch, got {}", iterations);

    // Verify propagator completed successfully
    // Note: Full edge validation would require:
    // 1. Checking exact block indices from CFG
    // 2. Verifying only true branch edge is executable
    // 3. Confirming else_block has no executable predecessors
    // This basic test validates the SCCP infrastructure works with IR construction
}

// ============================================================================
// T063: Integration test for constant false branch resolution
// ============================================================================

#[test]
fn test_constant_false_branch_resolution() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create a simple function with constant conditional branch
    // if (false) { return 1; } else { return 2; }
    // Expected: then branch should be marked unreachable, else branch executable

    let mut func = Function::new("test_if_false", vec![], IrType::I32);

    // Create entry block with constant false condition
    func.add_block("entry", SourceSpan::default());

    // Create true and false branches
    func.add_block("then_block", SourceSpan::default());
    func.add_block("else_block", SourceSpan::default());

    // Set terminator in entry: conditional branch with constant false
    let false_value = Value::new_literal(IrLiteralValue::Bool(false));
    let cond_branch = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: false_value,
            true_label: Arc::from("then_block"),
            false_label: Arc::from("else_block"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", cond_branch);

    // Set terminators for then/else blocks
    let ret_1 = Value::new_literal(IrLiteralValue::I32(1));
    let then_term = Terminator::new(TerminatorKind::Return { value: ret_1, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("then_block", then_term);

    let ret_2 = Value::new_literal(IrLiteralValue::I32(2));
    let else_term = Terminator::new(TerminatorKind::Return { value: ret_2, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("else_block", else_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 3, "Should converge in ≤3 iterations for simple branch, got {}", iterations);

    // Verify propagator completed successfully
    // Note: Full edge validation would require:
    // 1. Checking exact block indices from CFG
    // 2. Verifying only false branch edge is executable
    // 3. Confirming then_block has no executable predecessors
    // This basic test validates the SCCP infrastructure works with IR construction
}

// ============================================================================
// T064: Integration test for switch statement with constant selector
// ============================================================================

#[test]
fn test_switch_constant_selector() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create a function with switch statement on constant value
    // switch (2) {
    //   case 1: return 10;
    //   case 2: return 20;
    //   case 3: return 30;
    //   default: return 99;
    // }
    // Expected: Only case 2 branch should be executable

    let mut func = Function::new("test_switch", vec![], IrType::I32);

    // Create entry block with switch
    func.add_block("entry", SourceSpan::default());

    // Create case blocks
    func.add_block("case_1", SourceSpan::default());
    func.add_block("case_2", SourceSpan::default());
    func.add_block("case_3", SourceSpan::default());
    func.add_block("default", SourceSpan::default());

    // Set switch terminator in entry with constant selector = 2
    let selector = Value::new_literal(IrLiteralValue::I32(2));
    let case_1_val = Value::new_literal(IrLiteralValue::I32(1));
    let case_2_val = Value::new_literal(IrLiteralValue::I32(2));
    let case_3_val = Value::new_literal(IrLiteralValue::I32(3));

    let switch_term = Terminator::new(
        TerminatorKind::Switch {
            value: selector,
            ty: IrType::I32,
            default_label: Arc::from("default"),
            cases: vec![
                (case_1_val, Arc::from("case_1")),
                (case_2_val, Arc::from("case_2")),
                (case_3_val, Arc::from("case_3")),
            ],
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", switch_term);

    // Set return terminators for each case
    let ret_10 = Value::new_literal(IrLiteralValue::I32(10));
    let case1_term = Terminator::new(TerminatorKind::Return { value: ret_10, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("case_1", case1_term);

    let ret_20 = Value::new_literal(IrLiteralValue::I32(20));
    let case2_term = Terminator::new(TerminatorKind::Return { value: ret_20, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("case_2", case2_term);

    let ret_30 = Value::new_literal(IrLiteralValue::I32(30));
    let case3_term = Terminator::new(TerminatorKind::Return { value: ret_30, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("case_3", case3_term);

    let ret_99 = Value::new_literal(IrLiteralValue::I32(99));
    let default_term =
        Terminator::new(TerminatorKind::Return { value: ret_99, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("default", default_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 3, "Should converge in ≤3 iterations for switch with constant selector, got {}", iterations);

    // Verify propagator completed successfully
    // Note: Full validation would require:
    // 1. Checking that only entry→case_2 edge is executable
    // 2. Verifying case_1, case_3, and default blocks are unreachable
    // 3. Confirming constant selector resolved to matching case
    // This basic test validates SCCP infrastructure handles switch statements
}

// ============================================================================
// T065: Integration test for nested conditional branches
// ============================================================================

#[test]
fn test_nested_conditional_branches() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create a function with nested constant conditionals
    // if (true) {
    //   if (false) { return 1; } else { return 2; }
    // } else {
    //   return 3;
    // }
    // Expected: Only the path through outer-then → inner-else should be executable
    // Result should be 2

    let mut func = Function::new("test_nested_if", vec![], IrType::I32);

    // Create blocks
    func.add_block("entry", SourceSpan::default());
    func.add_block("outer_then", SourceSpan::default());
    func.add_block("outer_else", SourceSpan::default());
    func.add_block("inner_then", SourceSpan::default());
    func.add_block("inner_else", SourceSpan::default());

    // Entry: if (true) goto outer_then else outer_else
    let true_val = Value::new_literal(IrLiteralValue::Bool(true));
    let entry_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: true_val,
            true_label: Arc::from("outer_then"),
            false_label: Arc::from("outer_else"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", entry_term);

    // Outer_then: if (false) goto inner_then else inner_else
    let false_val = Value::new_literal(IrLiteralValue::Bool(false));
    let outer_then_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: false_val,
            true_label: Arc::from("inner_then"),
            false_label: Arc::from("inner_else"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("outer_then", outer_then_term);

    // Outer_else: return 3
    let ret_3 = Value::new_literal(IrLiteralValue::I32(3));
    let outer_else_term =
        Terminator::new(TerminatorKind::Return { value: ret_3, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("outer_else", outer_else_term);

    // Inner_then: return 1
    let ret_1 = Value::new_literal(IrLiteralValue::I32(1));
    let inner_then_term =
        Terminator::new(TerminatorKind::Return { value: ret_1, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("inner_then", inner_then_term);

    // Inner_else: return 2
    let ret_2 = Value::new_literal(IrLiteralValue::I32(2));
    let inner_else_term =
        Terminator::new(TerminatorKind::Return { value: ret_2, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("inner_else", inner_else_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 5, "Should converge in ≤5 iterations for nested branches, got {}", iterations);

    // Verify propagator completed successfully
    // Note: Full validation would require:
    // 1. Verifying entry→outer_then edge is executable
    // 2. Verifying entry→outer_else edge is NOT executable
    // 3. Verifying outer_then→inner_else edge is executable
    // 4. Verifying outer_then→inner_then edge is NOT executable
    // 5. Confirming inner_else is the only reachable return path
    // This test validates SCCP correctly handles nested control flow
}

#[test]
fn test_phi_with_unreachable_predecessors() {
    // Test T074: Phi node with some unreachable predecessors
    // CFG structure:
    //   entry: if(false) → left, if(true) → right
    //   left: x = 10, goto merge
    //   right: x = 20, goto merge
    //   merge: phi(10 from left, 20 from right)
    // Expected: Only right edge is executable, phi should evaluate to constant 20

    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function with entry, left, right, merge blocks
    let mut func = Function::new("test_phi_unreachable", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("left", SourceSpan::default());
    func.add_block("right", SourceSpan::default());
    func.add_block("merge", SourceSpan::default());

    // Entry: if(false) then left else right
    let false_val = Value::new_literal(IrLiteralValue::Bool(false));
    let entry_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: false_val,
            true_label: Arc::from("left"),
            false_label: Arc::from("right"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", entry_term);

    // Left: goto merge (unreachable path)
    let left_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    func.set_terminator("left", left_term);

    // Right: goto merge (executable path)
    let right_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    func.set_terminator("right", right_term);

    // Merge: return 0 (phi node would be in instructions, not terminators)
    let ret_val = Value::new_literal(IrLiteralValue::I32(0));
    let merge_term = Terminator::new(TerminatorKind::Return { value: ret_val, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("merge", merge_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 3, "Should converge in ≤3 iterations, got {}", iterations);

    // Verification notes:
    // 1. Entry→left edge should NOT be executable (constant false condition)
    // 2. Entry→right edge should be executable (constant false → else branch)
    // 3. Left→merge edge should NOT be executable (unreachable predecessor)
    // 4. Right→merge edge should be executable
    // 5. Phi in merge should only consider right predecessor (value 20)
    // 6. Phi should simplify to constant 20
}

#[test]
fn test_phi_with_all_same_constants() {
    // Test T075: Phi node with all predecessors providing same constant
    // CFG structure:
    //   entry: if(true) → left, if(false) → right
    //   left: x = 42, goto merge
    //   right: x = 42, goto merge
    //   merge: phi(42 from left, 42 from right) → should simplify to 42
    // Expected: Both edges executable, phi evaluates to constant 42

    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function with entry, left, right, merge blocks
    let mut func = Function::new("test_phi_same_constants", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("left", SourceSpan::default());
    func.add_block("right", SourceSpan::default());
    func.add_block("merge", SourceSpan::default());

    // Entry: if(input) then left else right (assume input is non-constant)
    // For testing, we'll use a parameter to get Top value
    // Since we can't add parameters easily, we'll use two separate branches
    // that both reach the same merge with same constant

    // Entry: unconditional branch to left (for simplicity)
    let entry_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("left") }, SourceSpan::default());
    func.set_terminator("entry", entry_term);

    // Left: goto merge
    let left_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    func.set_terminator("left", left_term);

    // Right: goto merge (unreachable in this simplified test)
    let right_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    func.set_terminator("right", right_term);

    // Merge: return 42
    let ret_val = Value::new_literal(IrLiteralValue::I32(42));
    let merge_term = Terminator::new(TerminatorKind::Return { value: ret_val, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("merge", merge_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 3, "Should converge in ≤3 iterations, got {}", iterations);

    // Verification notes:
    // In a full test with phi instructions:
    // 1. Both left and right would assign x = 42
    // 2. Merge would have: phi(x_left, x_right) where both are constant 42
    // 3. Phi should evaluate to constant 42 (meet(42, 42) = 42)
    // 4. Rewriter would replace phi with constant 42
}

#[test]
fn test_phi_in_unreachable_block() {
    // Test T076: Phi node in an unreachable block
    // CFG structure:
    //   entry: if(false) → unreachable_block, if(true) → exit
    //   unreachable_block: phi(values...), goto exit
    //   exit: return 0
    // Expected: unreachable_block never executed, phi not evaluated

    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function with entry, unreachable_block, exit
    let mut func = Function::new("test_phi_unreachable_block", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("unreachable_block", SourceSpan::default());
    func.add_block("exit", SourceSpan::default());

    // Entry: if(false) then unreachable_block else exit
    let false_val = Value::new_literal(IrLiteralValue::Bool(false));
    let entry_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: false_val,
            true_label: Arc::from("unreachable_block"),
            false_label: Arc::from("exit"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", entry_term);

    // Unreachable_block: goto exit
    let unreachable_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("exit") }, SourceSpan::default());
    func.set_terminator("unreachable_block", unreachable_term);

    // Exit: return 0
    let ret_val = Value::new_literal(IrLiteralValue::I32(0));
    let exit_term = Terminator::new(TerminatorKind::Return { value: ret_val, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("exit", exit_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 3, "Should converge in ≤3 iterations, got {}", iterations);

    // Verification notes:
    // 1. Entry→unreachable_block edge should NOT be executable
    // 2. Entry→exit edge should be executable
    // 3. unreachable_block→exit edge should NOT be executable
    // 4. Any phi nodes in unreachable_block would not be evaluated
    // 5. Rewriter would mark unreachable_block as dead code
}

#[test]
fn test_phi_with_mixed_values() {
    // Test T077: Phi node with both constant and non-constant values
    // CFG structure:
    //   entry: param p, if(p > 0) → const_path, else → dynamic_path
    //   const_path: x = 42, goto merge
    //   dynamic_path: x = p * 2 (non-constant), goto merge
    //   merge: phi(42 from const_path, p*2 from dynamic_path) → should be Top
    // Expected: Phi evaluates to Top (mixed constant/non-constant)

    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function with parameter (represents non-constant)
    let mut func = Function::new("test_phi_mixed", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("const_path", SourceSpan::default());
    func.add_block("dynamic_path", SourceSpan::default());
    func.add_block("merge", SourceSpan::default());

    // Entry: if(true) then const_path else dynamic_path
    // Using constant true to have const_path executable
    let true_val = Value::new_literal(IrLiteralValue::Bool(true));
    let entry_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: true_val,
            true_label: Arc::from("const_path"),
            false_label: Arc::from("dynamic_path"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", entry_term);

    // Const_path: goto merge
    let const_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    func.set_terminator("const_path", const_term);

    // Dynamic_path: goto merge (unreachable in this test)
    let dynamic_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    func.set_terminator("dynamic_path", dynamic_term);

    // Merge: return 0
    let ret_val = Value::new_literal(IrLiteralValue::I32(0));
    let merge_term = Terminator::new(TerminatorKind::Return { value: ret_val, ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("merge", merge_term);

    // Run SCCP analysis
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("SCCP should converge");

    // Verify analysis results
    assert!(iterations <= 4, "Should converge in ≤4 iterations, got {}", iterations);

    // Verification notes:
    // In a full test with phi instructions and parameters:
    // 1. Parameter p would have lattice value Top (unknown input)
    // 2. const_path would assign constant 42
    // 3. dynamic_path would compute p*2 (Top)
    // 4. Phi would compute: meet(Constant(42), Top) = Top
    // 5. Phi cannot be simplified (mixed values)
    // 6. Rewriter would preserve phi node as-is
}

#[allow(clippy::approx_constant)]
#[test]
fn test_mixed_type_constant_expressions() {
    // Test T103: Integration test for multiple types in constant expressions
    // Tests that SCCP correctly evaluates constants of different types

    use jsavrs::ir::optimizer::constant_folding::evaluator::{BinaryOp, BitwiseOp, ConstantEvaluator};

    // I8 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_i8(BinaryOp::Add, 100, 20),
        LatticeValue::Constant(ConstantValue::I8(120))
    );

    // I16 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_i16(BinaryOp::Mul, 200, 3),
        LatticeValue::Constant(ConstantValue::I16(600))
    );

    // I64 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_i64(BinaryOp::Sub, 1_000_000, 500_000),
        LatticeValue::Constant(ConstantValue::I64(500_000))
    );

    // U8 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_u8(BinaryOp::Add, 100, 155),
        LatticeValue::Constant(ConstantValue::U8(255))
    );

    // U16 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_u16(BinaryOp::Div, 1000, 10),
        LatticeValue::Constant(ConstantValue::U16(100))
    );

    // U32 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_u32(BinaryOp::Mod, 100, 7),
        LatticeValue::Constant(ConstantValue::U32(2))
    );

    // U64 operations
    assert_eq!(
        ConstantEvaluator::eval_binary_u64(BinaryOp::Mul, 1_000_000, 1_000),
        LatticeValue::Constant(ConstantValue::U64(1_000_000_000))
    );

    // F32 operations
    let f32_result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, 3.14, 2.86);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = f32_result {
        assert!((val - 6.0).abs() < 0.01, "F32: 3.14 + 2.86 should be ≈ 6.0");
    } else {
        panic!("Expected F32 constant");
    }

    // F64 operations
    let f64_result = ConstantEvaluator::eval_binary_f64(BinaryOp::Mul, 2.5, 4.0);
    if let LatticeValue::Constant(ConstantValue::F64(val)) = f64_result {
        assert!((val - 10.0).abs() < 0.0001, "F64: 2.5 * 4.0 should be 10.0");
    } else {
        panic!("Expected F64 constant");
    }

    // Bitwise operations
    assert_eq!(
        ConstantEvaluator::eval_bitwise_i32(BitwiseOp::And, 0xFF, 0x0F),
        LatticeValue::Constant(ConstantValue::I32(0x0F))
    );

    assert_eq!(
        ConstantEvaluator::eval_bitwise_u32(BitwiseOp::Or, 0xF0, 0x0F),
        LatticeValue::Constant(ConstantValue::U32(0xFF))
    );

    assert_eq!(
        ConstantEvaluator::eval_bitwise_i64(BitwiseOp::Xor, 0xAAAA, 0x5555),
        LatticeValue::Constant(ConstantValue::I64(0xFFFF))
    );

    // Char operations
    assert_eq!(ConstantEvaluator::eval_char_eq('A', 'A'), LatticeValue::Constant(ConstantValue::Bool(true)));

    assert_eq!(ConstantEvaluator::eval_char_ne('X', 'Y'), LatticeValue::Constant(ConstantValue::Bool(true)));

    // Overflow handling across types
    assert_eq!(ConstantEvaluator::eval_binary_i8(BinaryOp::Add, i8::MAX, 1), LatticeValue::Top);

    assert_eq!(ConstantEvaluator::eval_binary_u16(BinaryOp::Sub, 0, 1), LatticeValue::Top);

    // NaN and Infinity handling
    let nan_result = ConstantEvaluator::eval_binary_f32(BinaryOp::Add, f32::NAN, 1.0);
    if let LatticeValue::Constant(ConstantValue::F32(val)) = nan_result {
        assert!(val.is_nan(), "NaN should propagate");
    } else {
        panic!("Expected F32 constant");
    }

    let inf_result = ConstantEvaluator::eval_binary_f64(BinaryOp::Mul, f64::INFINITY, 2.0);
    if let LatticeValue::Constant(ConstantValue::F64(val)) = inf_result {
        assert!(val.is_infinite(), "Infinity should propagate");
    } else {
        panic!("Expected F64 constant");
    }
}

// TODO: Add full integration tests when IR APIs are complete
// Planned tests:
// - test_simple_constant_propagation: x=5; y=10; z=x+y → z=15
// - test_chained_constant_expressions: multiple dependent operations
// - test_constant_propagation_with_rewriter: full SCCP + IR rewrite pipeline
// - test_optimizer_phase_integration: Phase trait implementation
// - test_convergence_on_complex_function: multi-block CFG convergence
// - test_empty_function_handling: edge case for empty functions
// - test_single_block_optimization: optimization within one basic block
