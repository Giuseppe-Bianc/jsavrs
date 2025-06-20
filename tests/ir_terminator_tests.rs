use jsavrs::ir::{ImmediateValue, IrType, Terminator, Value, ValueKind};

// Helper functions to create values
fn i32_val(n: i32) -> Value {
    Value::new_immediate(ImmediateValue::I32(n))
}

fn bool_val(b: bool) -> Value {
    Value::new_immediate(ImmediateValue::Bool(b))
}

fn string_val(s: &str) -> Value {
    Value::new_immediate(ImmediateValue::String(s.to_string()))
}

fn local_val(name: &str) -> Value {
    Value {
        kind: ValueKind::Local(name.to_string()),
        ty: IrType::I32,
    }
}

#[test]
fn test_return_terminator() {
    let term = Terminator::Return(i32_val(42), IrType::I32);
    assert_eq!(term.to_string(), "ret 42i32 i32");

    let term = Terminator::Return(Value::new_immediate(ImmediateValue::F64(3.14)), IrType::F64);
    assert_eq!(term.to_string(), "ret 3.14f64 f64");

    let term = Terminator::Return(string_val("hello"), IrType::String);
    assert_eq!(term.to_string(), r#"ret "hello" string"#);

    // Edge case: empty string
    let term = Terminator::Return(string_val(""), IrType::String);
    assert_eq!(term.to_string(), r#"ret "" string"#);
}

#[test]
fn test_branch_terminator() {
    let term = Terminator::Branch("label1".to_string());
    assert_eq!(term.to_string(), "br label1");

    // Edge case: empty label
    let term = Terminator::Branch("".to_string());
    assert_eq!(term.to_string(), "br ");
}

#[test]
fn test_conditional_branch_terminator() {
    let term = Terminator::ConditionalBranch {
        condition: bool_val(true),
        true_label: "if_true".to_string(),
        false_label: "if_false".to_string(),
    };
    assert_eq!(term.to_string(), "br true ? if_true : if_false");

    // With complex condition
    let term = Terminator::ConditionalBranch {
        condition: Value {
            kind: ValueKind::Temporary("cond".to_string()),
            ty: IrType::Bool,
        },
        true_label: "block$1".to_string(),
        false_label: "block#2".to_string(),
    };
    assert_eq!(term.to_string(), "br tcond ? block$1 : block#2");

    // Edge case: empty labels
    let term = Terminator::ConditionalBranch {
        condition: bool_val(false),
        true_label: "".to_string(),
        false_label: "".to_string(),
    };
    assert_eq!(term.to_string(), "br false ?  : ");
}

#[test]
fn test_switch_terminator() {
    // Single case
    let term = Terminator::Switch {
        value: i32_val(5),
        ty: IrType::I32,
        default_label: "default".to_string(),
        cases: vec![(i32_val(1), "case1".to_string())],
    };
    assert_eq!(
        term.to_string(),
        "switch 5i32 i32: 1i32 => case1 default default"
    );

    // Multiple cases
    let term = Terminator::Switch {
        value: local_val("input"),
        ty: IrType::I32,
        default_label: "default_block".to_string(),
        cases: vec![
            (i32_val(0), "zero".to_string()),
            (i32_val(1), "one".to_string()),
            (i32_val(2), "two".to_string()),
        ],
    };
    assert_eq!(
        term.to_string(),
        "switch %input i32: 0i32 => zero, 1i32 => one, 2i32 => two default default_block"
    );

    // No cases
    let term = Terminator::Switch {
        value: i32_val(100),
        ty: IrType::I32,
        default_label: "only_default".to_string(),
        cases: vec![],
    };
    assert_eq!(term.to_string(), "switch 100i32 i32:  default only_default");

    // Mixed value types
    let term = Terminator::Switch {
        value: Value {
            kind: ValueKind::Global("global".to_string()),
            ty: IrType::I32,
        },
        ty: IrType::I32,
        default_label: "default".to_string(),
        cases: vec![
            (i32_val(10), "ten".to_string()),
            (local_val("val"), "local_case".to_string()),
        ],
    };
    assert_eq!(
        term.to_string(),
        "switch @global i32: 10i32 => ten, %val => local_case default default"
    );

    // Special characters in labels
    let term = Terminator::Switch {
        value: i32_val(3),
        ty: IrType::I32,
        default_label: "block#special".to_string(),
        cases: vec![
            (i32_val(1), "case@1".to_string()),
            (i32_val(2), "case$2".to_string()),
        ],
    };
    assert_eq!(
        term.to_string(),
        "switch 3i32 i32: 1i32 => case@1, 2i32 => case$2 default block#special"
    );
}

#[test]
fn test_unreachable_terminator() {
    let term = Terminator::Unreachable;
    assert_eq!(term.to_string(), "unreachable");
}

#[test]
fn test_is_terminator() {
    assert!(Terminator::Return(i32_val(0), IrType::I32).is_terminator());
    assert!(Terminator::Branch("label".to_string()).is_terminator());
    assert!(
        Terminator::ConditionalBranch {
            condition: bool_val(true),
            true_label: "t".to_string(),
            false_label: "f".to_string(),
        }
        .is_terminator()
    );
    assert!(
        Terminator::Switch {
            value: i32_val(0),
            ty: IrType::I32,
            default_label: "d".to_string(),
            cases: vec![],
        }
        .is_terminator()
    );
    assert!(!Terminator::Unreachable.is_terminator());
}

#[test]
fn test_edge_cases() {
    // Extreme integer values
    let large_num = i64::MAX;
    let term = Terminator::Return(
        Value::new_immediate(ImmediateValue::I64(large_num)),
        IrType::I64,
    );
    assert_eq!(term.to_string(), format!("ret {}i64 i64", large_num));

    // Special float values
    let term = Terminator::Return(
        Value::new_immediate(ImmediateValue::F32(f32::NAN)),
        IrType::F32,
    );
    assert_eq!(term.to_string(), "ret NaNf32 f32");

    // Empty strings in labels and values
    let term = Terminator::Switch {
        value: Value::new_immediate(ImmediateValue::String("".to_string())),
        ty: IrType::String,
        default_label: "".to_string(),
        cases: vec![(string_val(""), "".to_string())],
    };
    assert_eq!(term.to_string(), r#"switch "" string: "" =>  default "#);

    // Long strings
    let long_str = "a".repeat(1000);
    let term = Terminator::Return(
        Value::new_immediate(ImmediateValue::String(long_str.clone())),
        IrType::String,
    );
    assert_eq!(
        term.to_string(),
        format!(r#"ret "{}" string"#, long_str.escape_default())
    );
}
