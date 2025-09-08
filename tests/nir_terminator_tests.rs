use jsavrs::nir::{IrConstantValue, IrLiteralValue, IrType, Terminator, TerminatorKind, Value};
use jsavrs::utils::dummy_span;

// Helper per creare valori di test
fn create_i32_value(v: i32) -> Value {
    Value::new_literal(IrLiteralValue::I32(v))
}

fn create_bool_value(v: bool) -> Value {
    Value::new_literal(IrLiteralValue::Bool(v))
}

#[test]
fn return_terminator_edge_cases() {
    // Valore con caratteri speciali
    let string_val = Value::new_constant(IrConstantValue::String { string: "\n\t\\\"".into() }, IrType::String);

    let term = Terminator::new(TerminatorKind::Return { value: string_val, ty: IrType::String }, dummy_span());

    assert!(term.is_terminator());
    assert!(term.get_targets().is_empty());
    assert_eq!(format!("{}", term), "ret \"\\n\\t\\\\\\\"\" string");

    // Valori numerici edge
    let edge_cases = [(i32::MIN, "ret -2147483648i32 i32"), (i32::MAX, "ret 2147483647i32 i32"), (0, "ret 0i32 i32")];

    for (val, expected) in edge_cases {
        let value = create_i32_value(val);
        let term = Terminator::new(TerminatorKind::Return { value: value.clone(), ty: IrType::I32 }, dummy_span());

        assert_eq!(format!("{}", term), expected);
    }
}

#[test]
fn branch_terminator_edge_cases() {
    // Label vuota
    let term = Terminator::new(TerminatorKind::Branch { label: "".into() }, dummy_span());

    assert!(term.is_terminator());
    assert_eq!(term.get_targets(), vec![""]);
    assert_eq!(format!("{}", term), "br ");

    // Caratteri speciali
    let term = Terminator::new(TerminatorKind::Branch { label: "label$@1".into() }, dummy_span());

    assert_eq!(format!("{}", term), "br label$@1");
}

#[test]
fn conditional_branch_edge_cases() {
    // Stessa label per entrambi i rami
    let term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_bool_value(true),
            true_label: "shared".into(),
            false_label: "shared".into(),
        },
        dummy_span(),
    );

    assert!(term.is_terminator());
    assert_eq!(term.get_targets(), vec!["shared", "shared"]);
    assert_eq!(format!("{}", term), "br true ? shared : shared");

    // Label vuote
    let term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_bool_value(false),
            true_label: "".into(),
            false_label: "".into(),
        },
        dummy_span(),
    );

    assert_eq!(format!("{}", term), "br false ?  : ");
}

#[test]
fn switch_terminator_edge_cases() {
    // Zero cases
    let term = Terminator::new(
        TerminatorKind::Switch {
            value: create_i32_value(0),
            ty: IrType::I32,
            default_label: "default".to_string(),
            cases: Vec::new(),
        },
        dummy_span(),
    );

    assert!(term.is_terminator());
    assert_eq!(term.get_targets(), vec!["default"]);
    assert_eq!(format!("{}", term), "switch 0i32 i32: , default default");

    // Single case
    let term = Terminator::new(
        TerminatorKind::Switch {
            value: create_i32_value(42),
            ty: IrType::I32,
            default_label: "default".to_string(),
            cases: vec![(create_i32_value(1), "case1".to_string())],
        },
        dummy_span(),
    );

    assert_eq!(term.get_targets(), vec!["case1", "default"]);
    assert_eq!(format!("{}", term), "switch 42i32 i32: 1i32 => case1, default default");

    // Valori estremi nei cases
    let term = Terminator::new(
        TerminatorKind::Switch {
            value: create_i32_value(i32::MIN),
            ty: IrType::I32,
            default_label: "default".to_string(),
            cases: vec![
                (create_i32_value(i32::MIN), "min".to_string()),
                (create_i32_value(i32::MAX), "max".to_string()),
            ],
        },
        dummy_span(),
    );

    assert_eq!(
        format!("{}", term),
        "switch -2147483648i32 i32: -2147483648i32 => min, 2147483647i32 => max, default default"
    );
}

#[test]
fn indirect_branch_edge_cases() {
    // Zero labels
    let term = Terminator::new(
        TerminatorKind::IndirectBranch { address: create_i32_value(0xABCD), possible_labels: Vec::new() },
        dummy_span(),
    );

    assert!(term.is_terminator());
    assert!(term.get_targets().is_empty());
    assert_eq!(format!("{}", term), "ibr 43981i32 []");

    // Label vuote
    let term = Terminator::new(
        TerminatorKind::IndirectBranch {
            address: create_i32_value(0),
            possible_labels: vec!["".to_string(), "label".to_string()],
        },
        dummy_span(),
    );

    assert_eq!(term.get_targets(), vec!["", "label"]);
    assert_eq!(format!("{}", term), "ibr 0i32 [, label]");
}

#[test]
fn unreachable_terminator() {
    let term = Terminator::new(TerminatorKind::Unreachable, dummy_span());

    assert!(!term.is_terminator());
    assert!(term.get_targets().is_empty());
    assert_eq!(format!("{}", term), "unreachable");
}

#[test]
fn get_targets_edge_cases() {
    // Switch con target duplicati
    let switch_term = Terminator::new(
        TerminatorKind::Switch {
            value: create_i32_value(1),
            ty: IrType::I32,
            default_label: "target".to_string(),
            cases: vec![(create_i32_value(1), "target".to_string()), (create_i32_value(2), "target".to_string())],
        },
        dummy_span(),
    );

    assert_eq!(switch_term.get_targets(), vec!["target", "target", "target"]);

    // ConditionalBranch con target vuoti
    let cond_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_bool_value(true),
            true_label: "".into(),
            false_label: "".into(),
        },
        dummy_span(),
    );

    assert_eq!(cond_term.get_targets(), vec!["", ""]);
}

#[allow(clippy::approx_constant)]
#[test]
fn display_floating_point_edge_cases() {
    // Verifica formattazione float interi
    let float_val = Value::new_literal(IrLiteralValue::F32(42.0));
    let term = Terminator::new(TerminatorKind::Return { value: float_val, ty: IrType::F32 }, dummy_span());

    assert_eq!(format!("{}", term), "ret 42.0f32 f32");

    // Valori float non interi
    let float_val = Value::new_literal(IrLiteralValue::F64(3.14159));
    let term = Terminator::new(TerminatorKind::Return { value: float_val, ty: IrType::F64 }, dummy_span());

    assert_eq!(format!("{}", term), "ret 3.14159f64 f64");
}
