//! Unit tests for Boolean and comparison operations in SCCP evaluator
//!
//! Tests T059-T060 from User Story 2

use jsavrs::ir::optimizer::constant_folding::evaluator::{BinaryOp, ConstantEvaluator, UnaryOp};
use jsavrs::ir::optimizer::constant_folding::{ConstantValue, LatticeValue};

// ============================================================================
// T059: Unit tests for Boolean operations
// ============================================================================

#[test]
fn test_bool_and_true_true() {
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::And, true, true);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "true AND true should be true");
}

#[test]
fn test_bool_and_true_false() {
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::And, true, false);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "true AND false should be false");
}

#[test]
fn test_bool_and_false_false() {
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::And, false, false);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "false AND false should be false");
}

#[test]
fn test_bool_or_true_true() {
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::Or, true, true);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "true OR true should be true");
}

#[test]
fn test_bool_or_true_false() {
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::Or, true, false);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "true OR false should be true");
}

#[test]
fn test_bool_or_false_false() {
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::Or, false, false);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "false OR false should be false");
}

#[test]
fn test_bool_not_true() {
    let result = ConstantEvaluator::eval_unary_bool(UnaryOp::Not, true);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "NOT true should be false");
}

#[test]
fn test_bool_not_false() {
    let result = ConstantEvaluator::eval_unary_bool(UnaryOp::Not, false);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "NOT false should be true");
}

#[test]
fn test_bool_invalid_binary_op() {
    // Non-boolean operations should return Top
    let result = ConstantEvaluator::eval_binary_bool(BinaryOp::Add, true, false);
    assert_eq!(result, LatticeValue::Top, "Add operation on booleans should return Top");
}

#[test]
fn test_bool_invalid_unary_op() {
    // Non-boolean unary operations should return Top
    let result = ConstantEvaluator::eval_unary_bool(UnaryOp::Neg, true);
    assert_eq!(result, LatticeValue::Top, "Neg operation on boolean should return Top");
}

// ============================================================================
// T060: Unit tests for comparison operations
// ============================================================================

#[test]
fn test_i32_eq_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 42, 42);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "42 == 42 should be true");
}

#[test]
fn test_i32_eq_not_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 42, 43);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "42 == 43 should be false");
}

#[test]
fn test_i32_ne_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Ne, 42, 42);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "42 != 42 should be false");
}

#[test]
fn test_i32_ne_not_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Ne, 42, 43);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "42 != 43 should be true");
}

#[test]
fn test_i32_lt_less() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 10, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "10 < 20 should be true");
}

#[test]
fn test_i32_lt_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 20, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "20 < 20 should be false");
}

#[test]
fn test_i32_lt_greater() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 30, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "30 < 20 should be false");
}

#[test]
fn test_i32_le_less() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Le, 10, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "10 <= 20 should be true");
}

#[test]
fn test_i32_le_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Le, 20, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "20 <= 20 should be true");
}

#[test]
fn test_i32_le_greater() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Le, 30, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "30 <= 20 should be false");
}

#[test]
fn test_i32_gt_less() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 10, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "10 > 20 should be false");
}

#[test]
fn test_i32_gt_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 20, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "20 > 20 should be false");
}

#[test]
fn test_i32_gt_greater() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 30, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "30 > 20 should be true");
}

#[test]
fn test_i32_ge_less() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Ge, 10, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(false)), "10 >= 20 should be false");
}

#[test]
fn test_i32_ge_equal() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Ge, 20, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "20 >= 20 should be true");
}

#[test]
fn test_i32_ge_greater() {
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Ge, 30, 20);
    assert_eq!(result, LatticeValue::Constant(ConstantValue::Bool(true)), "30 >= 20 should be true");
}

#[test]
fn test_i32_compare_negative_values() {
    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, -10, -5),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "-10 < -5 should be true"
    );

    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, -5, -10),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "-5 > -10 should be true"
    );
}

#[test]
fn test_i32_compare_zero() {
    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 0, 0),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "0 == 0 should be true"
    );

    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 0, 1),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "0 < 1 should be true"
    );

    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 0, -1),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "0 > -1 should be true"
    );
}

#[test]
fn test_i32_compare_invalid_op() {
    // Non-comparison operations should return Top
    let result = ConstantEvaluator::eval_compare_i32(BinaryOp::Add, 10, 20);
    assert_eq!(result, LatticeValue::Top, "Add operation in comparison context should return Top");
}

#[test]
fn test_i32_compare_extremes() {
    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, i32::MIN, i32::MAX),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "i32::MIN < i32::MAX should be true"
    );

    assert_eq!(
        ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, i32::MAX, i32::MAX),
        LatticeValue::Constant(ConstantValue::Bool(true)),
        "i32::MAX == i32::MAX should be true"
    );
}

// ============================================================================
// Combined Boolean and Comparison Tests
// ============================================================================

#[test]
fn test_comparison_result_in_boolean_and() {
    // Simulate: (x < 10) && (y > 5) where x=3, y=8
    let cmp1 = ConstantEvaluator::eval_compare_i32(BinaryOp::Lt, 3, 10);
    let cmp2 = ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 8, 5);

    assert!(matches!(cmp1, LatticeValue::Constant(ConstantValue::Bool(true))));
    assert!(matches!(cmp2, LatticeValue::Constant(ConstantValue::Bool(true))));

    let final_result = ConstantEvaluator::eval_binary_bool(BinaryOp::And, true, true);
    assert_eq!(final_result, LatticeValue::Constant(ConstantValue::Bool(true)), "Combined comparison should be true");
}

#[test]
fn test_comparison_result_in_boolean_or() {
    // Simulate: (x == 0) || (y != 0) where x=5, y=0
    let cmp1 = ConstantEvaluator::eval_compare_i32(BinaryOp::Eq, 5, 0);
    let cmp2 = ConstantEvaluator::eval_compare_i32(BinaryOp::Ne, 0, 0);

    assert!(matches!(cmp1, LatticeValue::Constant(ConstantValue::Bool(false))));
    assert!(matches!(cmp2, LatticeValue::Constant(ConstantValue::Bool(false))));

    let final_result = ConstantEvaluator::eval_binary_bool(BinaryOp::Or, false, false);
    assert_eq!(final_result, LatticeValue::Constant(ConstantValue::Bool(false)), "Combined comparison should be false");
}

#[test]
fn test_negated_comparison() {
    // Simulate: !(x > 10) where x=5
    let cmp = ConstantEvaluator::eval_compare_i32(BinaryOp::Gt, 5, 10);

    assert!(matches!(cmp, LatticeValue::Constant(ConstantValue::Bool(false))));

    let negated = ConstantEvaluator::eval_unary_bool(UnaryOp::Not, false);
    assert_eq!(negated, LatticeValue::Constant(ConstantValue::Bool(true)), "Negated comparison should be true");
}
