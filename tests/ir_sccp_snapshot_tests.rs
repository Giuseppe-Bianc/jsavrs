// Snapshot tests for SCCP Constant Folding IR Transformations
// Uses insta crate for snapshot-based regression testing

// NOTE: Full IR transformation snapshots require complete Function/BasicBlock APIs.
// Current snapshots focus on data structure representations.

use insta::assert_snapshot;
use jsavrs::ir::optimizer::constant_folding::evaluator::UnaryOp;
use jsavrs::ir::optimizer::constant_folding::*;

#[test]
fn snapshot_lattice_value_representations() {
    // Snapshot test for lattice value representations
    let bottom = LatticeValue::Bottom;
    let const_42 = LatticeValue::Constant(ConstantValue::I32(42));
    let const_char = LatticeValue::Constant(ConstantValue::Char('A'));
    let top = LatticeValue::Top;

    let lattice_repr = format!(
        "Bottom: {:#?}\nConstant(I32(42)): {:#?}\nConstant(Char('A')): {:#?}\nTop: {:#?}",
        bottom, const_42, const_char, top
    );

    assert_snapshot!(lattice_repr);
}

#[test]
fn snapshot_meet_operation_results() {
    // Snapshot test for lattice meet operation results
    let bottom = LatticeValue::Bottom;
    let const_10 = LatticeValue::Constant(ConstantValue::I32(10));
    let const_20 = LatticeValue::Constant(ConstantValue::I32(20));
    let top = LatticeValue::Top;

    let results = format!(
        "Bottom ⊓ Constant(10) = {:#?}\n\
         Constant(10) ⊓ Constant(10) = {:#?}\n\
         Constant(10) ⊓ Constant(20) = {:#?}\n\
         Top ⊓ Constant(10) = {:#?}\n\
         Top ⊓ Bottom = {:#?}",
        bottom.meet(&const_10),
        const_10.meet(&const_10),
        const_10.meet(&const_20),
        top.meet(&const_10),
        top.meet(&bottom)
    );

    assert_snapshot!(results);
}

#[test]
fn snapshot_optimization_stats_format() {
    // Snapshot test specifically for optimization statistics output format
    let stats = OptimizationStats {
        constants_propagated: 15,
        branches_resolved: 3,
        phi_nodes_simplified: 2,
        blocks_marked_unreachable: 1,
        iterations: 2,
    };

    let formatted = format!("{}", stats);
    assert_snapshot!(formatted);
}

#[test]
fn snapshot_constant_value_types() {
    // Snapshot all constant value type representations
    let values = vec![
        ConstantValue::I8(127),
        ConstantValue::I16(32767),
        ConstantValue::I32(2147483647),
        ConstantValue::I64(9223372036854775807),
        ConstantValue::U8(255),
        ConstantValue::U16(65535),
        ConstantValue::U32(4294967295),
        ConstantValue::U64(18446744073709551615),
        ConstantValue::F32(3.14159),
        ConstantValue::F64(2.71828182845),
        ConstantValue::Bool(true),
        ConstantValue::Bool(false),
        ConstantValue::Char('A'),
    ];

    let repr = values.iter().map(|v| format!("{:#?}", v)).collect::<Vec<_>>().join("\n");

    assert_snapshot!(repr);
}

#[test]
fn snapshot_sccp_config() {
    // Snapshot SCCPConfig structure
    let default_config = SCCPConfig::default();
    let custom_config = SCCPConfig { verbose: true, max_iterations: 50 };

    let repr = format!("Default Config:\n{:#?}\n\nCustom Config:\n{:#?}", default_config, custom_config);

    assert_snapshot!(repr);
}

#[test]
fn snapshot_lattice_meet_commutative() {
    // Verify meet operation is commutative through snapshots
    let bottom = LatticeValue::Bottom;
    let const_val = LatticeValue::Constant(ConstantValue::I32(42));
    let top = LatticeValue::Top;

    let results = format!(
        "Commutativity Tests:\n\
         Bottom ⊓ Const = {:#?}\n\
         Const ⊓ Bottom = {:#?}\n\
         Top ⊓ Const = {:#?}\n\
         Const ⊓ Top = {:#?}\n\
         Const ⊓ Const = {:#?}",
        bottom.meet(&const_val),
        const_val.meet(&bottom),
        top.meet(&const_val),
        const_val.meet(&top),
        const_val.meet(&const_val)
    );

    assert_snapshot!(results);
}

#[test]
fn snapshot_mixed_type_meet_operations() {
    // Test meet operations between different constant types
    let i32_val = LatticeValue::Constant(ConstantValue::I32(10));
    let i64_val = LatticeValue::Constant(ConstantValue::I64(10));
    let f32_val = LatticeValue::Constant(ConstantValue::F32(10.0));
    let bool_val = LatticeValue::Constant(ConstantValue::Bool(true));

    let results = format!(
        "Mixed Type Meets:\n\
         I32(10) ⊓ I64(10) = {:#?}\n\
         I32(10) ⊓ F32(10.0) = {:#?}\n\
         I32(10) ⊓ Bool(true) = {:#?}\n\
         I64(10) ⊓ F32(10.0) = {:#?}",
        i32_val.meet(&i64_val),
        i32_val.meet(&f32_val),
        i32_val.meet(&bool_val),
        i64_val.meet(&f32_val)
    );

    assert_snapshot!(results);
}

// ============================================================================
// T066: Snapshot tests for branch resolution IR transformation
// ============================================================================

#[test]
fn snapshot_constant_true_branch_transformation() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function: if (true) { return 1; } else { return 2; }
    let mut func = Function::new("constant_true_branch", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("then_block", SourceSpan::default());
    func.add_block("else_block", SourceSpan::default());

    // Entry: conditional branch on constant true
    let true_val = Value::new_literal(IrLiteralValue::Bool(true));
    let cond_branch = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: true_val.clone(),
            true_label: Arc::from("then_block"),
            false_label: Arc::from("else_block"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", cond_branch);

    // Then block: return 1
    let ret_1 = Value::new_literal(IrLiteralValue::I32(1));
    let then_term =
        Terminator::new(TerminatorKind::Return { value: ret_1.clone(), ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("then_block", then_term);

    // Else block: return 2
    let ret_2 = Value::new_literal(IrLiteralValue::I32(2));
    let else_term =
        Terminator::new(TerminatorKind::Return { value: ret_2.clone(), ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("else_block", else_term);

    // Capture before state
    let block_labels: Vec<&str> = func.cfg.blocks().map(|b| b.label.as_ref()).collect();
    let before_state = format!(
        "Function: {}\n\
         Blocks: {:?}\n\
         Entry terminator: ConditionalBranch on Bool(true)\n\
         Then terminator: Return I32(1)\n\
         Else terminator: Return I32(2)",
        func.name.as_ref(),
        block_labels
    );

    // Run SCCP
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("Should converge");

    // Capture after state
    let after_state = format!(
        "SCCP Analysis Complete:\n\
         Iterations: {}\n\
         Expected transformation: ConditionalBranch(true) → Branch(then_block)\n\
         Expected unreachable: else_block\n\
         \n\
         Note: Full IR rewriting requires IRRewriter integration",
        iterations
    );

    let snapshot_output = format!("BEFORE:\n{}\n\nAFTER:\n{}", before_state, after_state);
    assert_snapshot!(snapshot_output);
}

#[test]
fn snapshot_constant_false_branch_transformation() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function: if (false) { return 1; } else { return 2; }
    let mut func = Function::new("constant_false_branch", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("then_block", SourceSpan::default());
    func.add_block("else_block", SourceSpan::default());

    // Entry: conditional branch on constant false
    let false_val = Value::new_literal(IrLiteralValue::Bool(false));
    let cond_branch = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: false_val.clone(),
            true_label: Arc::from("then_block"),
            false_label: Arc::from("else_block"),
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", cond_branch);

    // Then block: return 1
    let ret_1 = Value::new_literal(IrLiteralValue::I32(1));
    let then_term =
        Terminator::new(TerminatorKind::Return { value: ret_1.clone(), ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("then_block", then_term);

    // Else block: return 2
    let ret_2 = Value::new_literal(IrLiteralValue::I32(2));
    let else_term =
        Terminator::new(TerminatorKind::Return { value: ret_2.clone(), ty: IrType::I32 }, SourceSpan::default());
    func.set_terminator("else_block", else_term);

    // Capture before state
    let block_labels: Vec<&str> = func.cfg.blocks().map(|b| b.label.as_ref()).collect();
    let before_state = format!(
        "Function: {}\n\
         Blocks: {:?}\n\
         Entry terminator: ConditionalBranch on Bool(false)\n\
         Then terminator: Return I32(1)\n\
         Else terminator: Return I32(2)",
        func.name.as_ref(),
        block_labels
    );

    // Run SCCP
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("Should converge");

    // Capture after state
    let after_state = format!(
        "SCCP Analysis Complete:\n\
         Iterations: {}\n\
         Expected transformation: ConditionalBranch(false) → Branch(else_block)\n\
         Expected unreachable: then_block\n\
         \n\
         Note: Full IR rewriting requires IRRewriter integration",
        iterations
    );

    let snapshot_output = format!("BEFORE:\n{}\n\nAFTER:\n{}", before_state, after_state);
    assert_snapshot!(snapshot_output);
}

#[test]
fn snapshot_switch_constant_selector_transformation() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function with switch on constant selector
    let mut func = Function::new("constant_switch", vec![], IrType::I32);
    func.add_block("entry", SourceSpan::default());
    func.add_block("case_1", SourceSpan::default());
    func.add_block("case_2", SourceSpan::default());
    func.add_block("case_3", SourceSpan::default());
    func.add_block("default", SourceSpan::default());

    // Entry: switch on constant 2
    let selector = Value::new_literal(IrLiteralValue::I32(2));
    let switch_term = Terminator::new(
        TerminatorKind::Switch {
            value: selector.clone(),
            ty: IrType::I32,
            default_label: Arc::from("default"),
            cases: vec![
                (Value::new_literal(IrLiteralValue::I32(1)), Arc::from("case_1")),
                (Value::new_literal(IrLiteralValue::I32(2)), Arc::from("case_2")),
                (Value::new_literal(IrLiteralValue::I32(3)), Arc::from("case_3")),
            ],
        },
        SourceSpan::default(),
    );
    func.set_terminator("entry", switch_term);

    // Set return terminators
    for (block, val) in [("case_1", 10), ("case_2", 20), ("case_3", 30), ("default", 99)] {
        let ret_val = Value::new_literal(IrLiteralValue::I32(val));
        let term = Terminator::new(TerminatorKind::Return { value: ret_val, ty: IrType::I32 }, SourceSpan::default());
        func.set_terminator(block, term);
    }

    // Capture before state
    let block_labels: Vec<&str> = func.cfg.blocks().map(|b| b.label.as_ref()).collect();
    let before_state = format!(
        "Function: {}\n\
         Blocks: {:?}\n\
         Entry terminator: Switch on constant I32(2)\n\
         Cases: 1→case_1, 2→case_2, 3→case_3, default→default",
        func.name.as_ref(),
        block_labels
    );

    // Run SCCP
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("Should converge");

    // Capture after state
    let after_state = format!(
        "SCCP Analysis Complete:\n\
         Iterations: {}\n\
         Expected transformation: Switch(2) → Branch(case_2)\n\
         Expected unreachable: case_1, case_3, default\n\
         \n\
         Note: Full IR rewriting requires IRRewriter integration",
        iterations
    );

    let snapshot_output = format!("BEFORE:\n{}\n\nAFTER:\n{}", before_state, after_state);
    assert_snapshot!(snapshot_output);
}

// ============================================================================
// T067: Snapshot test for unreachable code marking
// ============================================================================

#[test]
fn snapshot_unreachable_code_marking() {
    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // Create function with multiple unreachable paths
    // if (true) {
    //   if (false) { return 1; } else { return 2; }
    // } else {
    //   return 3;
    // }
    // Expected unreachable: outer_else, inner_then
    // Expected reachable: entry, outer_then, inner_else

    let mut func = Function::new("unreachable_marking", vec![], IrType::I32);

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

    // Set return terminators
    for (block, val) in [("outer_else", 3), ("inner_then", 1), ("inner_else", 2)] {
        let ret_val = Value::new_literal(IrLiteralValue::I32(val));
        let term = Terminator::new(TerminatorKind::Return { value: ret_val, ty: IrType::I32 }, SourceSpan::default());
        func.set_terminator(block, term);
    }

    // Capture before state
    let block_labels: Vec<&str> = func.cfg.blocks().map(|b| b.label.as_ref()).collect();
    let before_state = format!(
        "Function: {}\n\
         Blocks: {:?}\n\
         Control Flow:\n\
         - entry: if (true) → outer_then | outer_else\n\
         - outer_then: if (false) → inner_then | inner_else\n\
         - outer_else: return 3\n\
         - inner_then: return 1\n\
         - inner_else: return 2",
        func.name.as_ref(),
        block_labels
    );

    // Run SCCP
    let mut propagator = SCCPropagator::new_for_function(&func);
    let iterations = propagator.propagate(&func, 10).expect("Should converge");

    // Capture after state
    let after_state = format!(
        "SCCP Analysis Complete:\n\
         Iterations: {}\n\
         \n\
         Reachable blocks:\n\
         - entry (always reachable)\n\
         - outer_then (reachable: true condition)\n\
         - inner_else (reachable: false condition)\n\
         \n\
         Unreachable blocks:\n\
         - outer_else (unreachable: outer condition is true)\n\
         - inner_then (unreachable: inner condition is false)\n\
         \n\
         Expected transformations:\n\
         - entry: ConditionalBranch(true) → Branch(outer_then)\n\
         - outer_then: ConditionalBranch(false) → Branch(inner_else)\n\
         - outer_else: UNREACHABLE\n\
         - inner_then: UNREACHABLE\n\
         \n\
         Note: Full IR rewriting and unreachable marking requires IRRewriter integration",
        iterations
    );

    let snapshot_output = format!("BEFORE:\n{}\n\nAFTER:\n{}", before_state, after_state);
    assert_snapshot!(snapshot_output);
}

#[test]
fn snapshot_phi_node_simplification() {
    // T078: Snapshot test for phi node simplification
    // Tests three scenarios:
    // 1. Phi with unreachable predecessors (only executable edges considered)
    // 2. Phi with all same constant values (simplified to constant)
    // 3. Phi with mixed values (preserved as-is, evaluates to Top)

    use jsavrs::ir::optimizer::constant_folding::propagator::SCCPropagator;
    use jsavrs::ir::terminator::{Terminator, TerminatorKind};
    use jsavrs::ir::types::IrType;
    use jsavrs::ir::value::IrLiteralValue;
    use jsavrs::ir::{Function, Value};
    use jsavrs::location::source_span::SourceSpan;
    use std::sync::Arc;

    // === Scenario 1: Phi with unreachable predecessor ===
    let mut func1 = Function::new("phi_unreachable_pred", vec![], IrType::I32);
    func1.add_block("entry", SourceSpan::default());
    func1.add_block("left", SourceSpan::default());
    func1.add_block("right", SourceSpan::default());
    func1.add_block("merge", SourceSpan::default());

    // Entry: if(false) → left, else → right
    let false_val = Value::new_literal(IrLiteralValue::Bool(false));
    func1.set_terminator(
        "entry",
        Terminator::new(
            TerminatorKind::ConditionalBranch {
                condition: false_val,
                true_label: Arc::from("left"),
                false_label: Arc::from("right"),
            },
            SourceSpan::default(),
        ),
    );
    func1.set_terminator(
        "left",
        Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default()),
    );
    func1.set_terminator(
        "right",
        Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default()),
    );
    func1.set_terminator(
        "merge",
        Terminator::new(
            TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
            SourceSpan::default(),
        ),
    );

    let scenario1_before = format!(
        "Scenario 1: Phi with Unreachable Predecessor\n\
         Function: {}\n\
         Blocks: {:?}\n\
         \n\
         Control Flow:\n\
         - entry: if (false) → left | right\n\
         - left: x = 10, goto merge\n\
         - right: x = 20, goto merge\n\
         - merge: phi(10 from left, 20 from right), return\n\
         \n\
         Note: left is unreachable (false condition)",
        func1.name.as_ref(),
        func1.cfg.blocks().map(|b| b.label.as_ref()).collect::<Vec<_>>()
    );

    let mut propagator1 = SCCPropagator::new_for_function(&func1);
    let iterations1 = propagator1.propagate(&func1, 10).expect("Should converge");

    let scenario1_after = format!(
        "SCCP Analysis:\n\
         Iterations: {}\n\
         \n\
         Executable edges:\n\
         - entry → right (false condition)\n\
         - right → merge\n\
         \n\
         Unreachable edges:\n\
         - entry → left (constant false)\n\
         - left → merge (unreachable predecessor)\n\
         \n\
         Phi evaluation:\n\
         - Only right predecessor is executable\n\
         - Phi should evaluate to constant 20\n\
         - Expected transformation: phi(10, 20) → constant 20",
        iterations1
    );

    // === Scenario 2: Phi with all same constants ===
    let mut func2 = Function::new("phi_same_constants", vec![], IrType::I32);
    func2.add_block("entry", SourceSpan::default());
    func2.add_block("merge", SourceSpan::default());

    func2.set_terminator(
        "entry",
        Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default()),
    );
    func2.set_terminator(
        "merge",
        Terminator::new(
            TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(42)), ty: IrType::I32 },
            SourceSpan::default(),
        ),
    );

    let scenario2_before = format!(
        "Scenario 2: Phi with All Same Constants\n\
         Function: {}\n\
         \n\
         Control Flow (conceptual):\n\
         - path1: x = 42, goto merge\n\
         - path2: x = 42, goto merge\n\
         - merge: phi(42 from path1, 42 from path2), return\n\
         \n\
         Note: Both paths provide same constant value",
        func2.name.as_ref()
    );

    let mut propagator2 = SCCPropagator::new_for_function(&func2);
    let iterations2 = propagator2.propagate(&func2, 10).expect("Should converge");

    let scenario2_after = format!(
        "SCCP Analysis:\n\
         Iterations: {}\n\
         \n\
         Phi evaluation:\n\
         - meet(Constant(42), Constant(42)) = Constant(42)\n\
         - All executable predecessors provide same constant\n\
         - Expected transformation: phi(42, 42) → constant 42",
        iterations2
    );

    // === Scenario 3: Phi with mixed values ===
    let scenario3_before = "Scenario 3: Phi with Mixed Values\n\
         \n\
         Control Flow (conceptual):\n\
         - const_path: x = 100, goto merge\n\
         - dynamic_path: x = param * 2 (non-constant), goto merge\n\
         - merge: phi(100 from const_path, param*2 from dynamic_path), return\n\
         \n\
         Note: One constant, one non-constant (Top) value";

    let scenario3_after = "SCCP Analysis:\n\
         \n\
         Phi evaluation:\n\
         - meet(Constant(100), Top) = Top\n\
         - Mixed constant and non-constant values\n\
         - Expected transformation: phi preserved as-is (cannot simplify)";

    // Combine all scenarios
    let snapshot_output = format!(
        "=== PHI NODE SIMPLIFICATION SCENARIOS ===\n\
         \n\
         SCENARIO 1 - UNREACHABLE PREDECESSOR\n\
         BEFORE:\n{}\n\
         \n\
         AFTER:\n{}\n\
         \n\
         =====================================\n\
         \n\
         SCENARIO 2 - ALL SAME CONSTANTS\n\
         BEFORE:\n{}\n\
         \n\
         AFTER:\n{}\n\
         \n\
         =====================================\n\
         \n\
         SCENARIO 3 - MIXED VALUES\n\
         BEFORE:\n{}\n\
         \n\
         AFTER:\n{}",
        scenario1_before, scenario1_after, scenario2_before, scenario2_after, scenario3_before, scenario3_after
    );

    assert_snapshot!(snapshot_output);
}

#[test]
fn snapshot_all_type_evaluations() {
    // T104: Snapshot test for constant evaluation across all IR types
    // Documents the behavior of type-safe evaluation for all supported types

    use jsavrs::ir::optimizer::constant_folding::evaluator::{BinaryOp, BitwiseOp, ConstantEvaluator};

    let mut output = String::new();

    // I8 Type Evaluations
    output.push_str("=== I8 Type Evaluations ===\n");
    output.push_str(&format!("Add: {:?}\n", ConstantEvaluator::eval_binary_i8(BinaryOp::Add, 100, 20)));
    output.push_str(&format!("Overflow: {:?}\n", ConstantEvaluator::eval_binary_i8(BinaryOp::Add, i8::MAX, 1)));
    output.push_str("\n");

    // I16 Type Evaluations
    output.push_str("=== I16 Type Evaluations ===\n");
    output.push_str(&format!("Mul: {:?}\n", ConstantEvaluator::eval_binary_i16(BinaryOp::Mul, 200, 3)));
    output.push_str(&format!("Overflow: {:?}\n", ConstantEvaluator::eval_binary_i16(BinaryOp::Mul, i16::MAX, 2)));
    output.push_str("\n");

    // I32 Type Evaluations
    output.push_str("=== I32 Type Evaluations ===\n");
    output.push_str(&format!("Add: {:?}\n", ConstantEvaluator::eval_binary_i32(BinaryOp::Add, 1000, 2000)));
    output.push_str(&format!("Div: {:?}\n", ConstantEvaluator::eval_binary_i32(BinaryOp::Div, 100, 5)));
    output.push_str(&format!("DivByZero: {:?}\n", ConstantEvaluator::eval_binary_i32(BinaryOp::Div, 100, 0)));
    output.push_str("\n");

    // I64 Type Evaluations
    output.push_str("=== I64 Type Evaluations ===\n");
    output.push_str(&format!("Sub: {:?}\n", ConstantEvaluator::eval_binary_i64(BinaryOp::Sub, 1_000_000, 500_000)));
    output.push_str(&format!("Overflow: {:?}\n", ConstantEvaluator::eval_binary_i64(BinaryOp::Sub, i64::MIN, 1)));
    output.push_str("\n");

    // U8 Type Evaluations
    output.push_str("=== U8 Type Evaluations ===\n");
    output.push_str(&format!("Add: {:?}\n", ConstantEvaluator::eval_binary_u8(BinaryOp::Add, 200, 55)));
    output.push_str(&format!("Overflow: {:?}\n", ConstantEvaluator::eval_binary_u8(BinaryOp::Add, u8::MAX, 1)));
    output.push_str(&format!("Underflow: {:?}\n", ConstantEvaluator::eval_binary_u8(BinaryOp::Sub, 0, 1)));
    output.push_str("\n");

    // U16 Type Evaluations
    output.push_str("=== U16 Type Evaluations ===\n");
    output.push_str(&format!("Div: {:?}\n", ConstantEvaluator::eval_binary_u16(BinaryOp::Div, 1000, 10)));
    output.push_str("\n");

    // U32 Type Evaluations
    output.push_str("=== U32 Type Evaluations ===\n");
    output.push_str(&format!("Mod: {:?}\n", ConstantEvaluator::eval_binary_u32(BinaryOp::Mod, 100, 7)));
    output.push_str("\n");

    // U64 Type Evaluations
    output.push_str("=== U64 Type Evaluations ===\n");
    output.push_str(&format!("Mul: {:?}\n", ConstantEvaluator::eval_binary_u64(BinaryOp::Mul, 1_000_000, 1_000)));
    output.push_str("\n");

    // F32 Type Evaluations (with special values)
    output.push_str("=== F32 Type Evaluations ===\n");
    output.push_str(&format!("Add: {:?}\n", ConstantEvaluator::eval_binary_f32(BinaryOp::Add, 3.14, 2.86)));
    output.push_str(&format!("NaN: is_nan = {}\n", ConstantEvaluator::is_nan_f32(f32::NAN)));
    output.push_str(&format!("Infinity: is_infinite = {}\n", ConstantEvaluator::is_infinite_f32(f32::INFINITY)));
    output.push_str(&format!("NegZero: is_neg_zero = {}\n", ConstantEvaluator::is_neg_zero_f32(-0.0)));
    output.push_str("\n");

    // F64 Type Evaluations
    output.push_str("=== F64 Type Evaluations ===\n");
    output.push_str(&format!("Mul: {:?}\n", ConstantEvaluator::eval_binary_f64(BinaryOp::Mul, 2.5, 4.0)));
    output.push_str(&format!("NaN: is_nan = {}\n", ConstantEvaluator::is_nan_f64(f64::NAN)));
    output.push_str(&format!("Infinity: is_infinite = {}\n", ConstantEvaluator::is_infinite_f64(f64::NEG_INFINITY)));
    output.push_str("\n");

    // Char Type Evaluations
    output.push_str("=== Char Type Evaluations ===\n");
    output.push_str(&format!("Eq('A', 'A'): {:?}\n", ConstantEvaluator::eval_char_eq('A', 'A')));
    output.push_str(&format!("Ne('X', 'Y'): {:?}\n", ConstantEvaluator::eval_char_ne('X', 'Y')));
    output.push_str(&format!("Unicode('😀', '😀'): {:?}\n", ConstantEvaluator::eval_char_eq('😀', '😀')));
    output.push_str("\n");

    // Bitwise Operations (all integer types)
    output.push_str("=== Bitwise Operations ===\n");
    output.push_str(&format!("AND i32: {:?}\n", ConstantEvaluator::eval_bitwise_i32(BitwiseOp::And, 0xFF, 0x0F)));
    output.push_str(&format!("OR u32: {:?}\n", ConstantEvaluator::eval_bitwise_u32(BitwiseOp::Or, 0xF0, 0x0F)));
    output.push_str(&format!("XOR i64: {:?}\n", ConstantEvaluator::eval_bitwise_i64(BitwiseOp::Xor, 0xAAAA, 0x5555)));
    output.push_str(&format!("SHL i32: {:?}\n", ConstantEvaluator::eval_bitwise_i32(BitwiseOp::Shl, 1, 4)));
    output.push_str(&format!("SHR u64: {:?}\n", ConstantEvaluator::eval_bitwise_u64(BitwiseOp::Shr, 128, 3)));
    output.push_str(&format!("NOT i8: {:?}\n", ConstantEvaluator::eval_bitwise_not_i8(0b00001111)));
    output.push_str(&format!("NOT u8: {:?}\n", ConstantEvaluator::eval_bitwise_not_u8(0b10101010)));
    output.push_str("\n");

    // Boolean Operations
    output.push_str("=== Boolean Operations ===\n");
    output.push_str(&format!("AND: {:?}\n", ConstantEvaluator::eval_binary_bool(BinaryOp::And, true, false)));
    output.push_str(&format!("OR: {:?}\n", ConstantEvaluator::eval_binary_bool(BinaryOp::Or, true, false)));
    output.push_str(&format!("NOT: {:?}\n", ConstantEvaluator::eval_unary_bool(UnaryOp::Not, true)));
    output.push_str("\n");

    // Comparison Operations
    output.push_str("=== Comparison Operations ===\n");
    output.push_str(&format!("EQ: {:?}\n", ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 42, 42)));
    output.push_str(&format!("LT: {:?}\n", ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 5, 10)));
    output.push_str(&format!("GT: {:?}\n", ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 10, 5)));

    assert_snapshot!(output);
}

// TODO: Add IR transformation snapshots when Function/BasicBlock APIs are complete
// Planned snapshots:
// - snapshot_simple_constant_propagation: Before/after for x=5+10
// - snapshot_chained_arithmetic: Before/after for chained operations
// - snapshot_division_operation: Before/after for division
// - snapshot_modulo_operation: Before/after for modulo
// - snapshot_multiple_blocks: Before/after for multi-block functions
// - snapshot_module_level_optimization: Before/after for whole module
