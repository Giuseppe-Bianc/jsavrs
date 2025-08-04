use jsavrs::nir::{
    BasicBlock, Cfg, Function, FunctionAttributes, Instruction, InstructionKind, IrLiteralValue,
    IrParameter, IrType, ParamAttributes, Terminator, TerminatorKind, Value,
};
use jsavrs::utils::dummy_span;
use insta::assert_snapshot;
use insta::assert_debug_snapshot;

fn create_dummy_value() -> Value {
    Value::new_literal(IrLiteralValue::I32(42))
}

#[test]
fn test_cfg_creation() {
    let cfg = Cfg::new("entry");
    assert_snapshot!(format!("{cfg:?}"));
}

#[test]
fn test_cfg_add_block() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    assert_snapshot!(format!("{cfg:?}"));
}

#[test]
fn test_cfg_add_edge() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);
    cfg.add_edge("entry", "block1");

    let block = cfg.blocks.get("block1").unwrap();
    assert_snapshot!(format!("{block:?}"));
}

#[test]
fn test_function_creation() {
    let params = vec![
        IrParameter {
            name: "param1".to_string(),
            ty: IrType::I32,
            attributes: ParamAttributes::default(),
        }
    ];
    let func = Function::new("test", params.clone(), IrType::Void);

    assert_debug_snapshot!(func)
}

#[test]
fn test_function_add_block() {
    let mut func = Function::new("test", vec![], IrType::Void);
    let block = BasicBlock::new("block1", dummy_span());
    func.add_block(block);

    assert_snapshot!(func.cfg.blocks.contains_key("block1"));
}

#[test]
fn test_function_add_local() {
    let mut func = Function::new("test", vec![], IrType::Void);
    func.add_local("var1".to_string(), IrType::I32);

    assert_debug_snapshot!(func.local_vars.get("var1"));
}

#[test]
fn test_function_add_edge() {
    let mut func = Function::new("test", vec![], IrType::Void);
    let block = BasicBlock::new("block1", dummy_span());
    func.add_block(block);
    func.add_edge("entry_test", "block1");

    let cfg = &func.cfg;
    assert_debug_snapshot!(cfg);
}

#[test]
fn test_basic_block_creation() {
    let block = BasicBlock::new("block1", dummy_span());

    assert_debug_snapshot!(block);
}

#[test]
fn test_basic_block_add_predecessor() {
    let mut block = BasicBlock::new("block1", dummy_span());
    block.add_predecessor("pred1".to_string());
    block.add_predecessor("pred2".to_string());
    assert_debug_snapshot!(block);
    block.add_predecessor("pred1".to_string());
    assert_debug_snapshot!(block);
}

#[test]
fn test_basic_block_display() {
    let mut block = BasicBlock::new("block1", dummy_span());

    // With predecessors
    block.add_predecessor("pred1".to_string());
    block.add_predecessor("pred2".to_string());

    assert_debug_snapshot!(block);

    // With instructions
    let value = create_dummy_value();
    let inst = Instruction::new(InstructionKind::Load {
        src: value.clone(),
        ty: IrType::I32,
    }, dummy_span());
    block.instructions.push(inst);

    block.terminator = Terminator::new(
        TerminatorKind::Branch { label: "exit".to_string() },
        dummy_span(),
    );

    assert_debug_snapshot!(block);
}

#[test]
fn test_function_display() {
    let mut func = Function::new("test", vec![], IrType::Void);

    // Create blocks
    let mut entry_block = BasicBlock::new("entry_test", dummy_span());
    let mut block1 = BasicBlock::new("block1", dummy_span());
    let mut exit_block = BasicBlock::new("exit", dummy_span());

    // Add terminators
    entry_block.terminator = Terminator::new(
        TerminatorKind::Branch { label: "block1".to_string() },
        dummy_span(),
    );
    block1.terminator = Terminator::new(
        TerminatorKind::Branch { label: "exit".to_string() },
        dummy_span(),
    );
    exit_block.terminator = Terminator::new(
        TerminatorKind::Return { value: create_dummy_value(), ty: IrType::Void },
        dummy_span(),
    );

    // Add blocks to function
    func.add_block(entry_block);
    func.add_block(block1);
    func.add_block(exit_block);

    // Add edges
    func.add_edge("entry_test", "block1");
    func.add_edge("block1", "exit");

    assert_debug_snapshot!(func);
}

#[test]
fn test_function_attributes() {
    let mut attrs = FunctionAttributes::default();
    attrs.is_entry = true;
    attrs.is_varargs = true;
    attrs.calling_convention = "fast".to_string();
    assert_debug_snapshot!(attrs);
}

#[test]
fn test_ir_parameter() {
    let param = IrParameter {
        name: "arg".to_string(),
        ty: IrType::I32,
        attributes: ParamAttributes {
            by_val: true,
            no_alias: true,
            source_span: Some(dummy_span()),
        },
    };

    assert_debug_snapshot!(param);
}

#[test]
fn test_complex_cfg() {
    let mut func = Function::new("complex", vec![], IrType::Void);

    // Create blocks
    let blocks = vec![
        ("entry", vec!["a", "b"]),
        ("a", vec!["c"]),
        ("b", vec!["c"]),
        ("c", vec!["exit"]),
        ("exit", vec![]),
    ];

    // Add blocks
    for (label, _) in &blocks {
        let block = BasicBlock::new(*label, dummy_span());
        func.add_block(block);
    }

    // Add edges
    for (src, dests) in blocks {
        for dest in dests {
            func.add_edge(src, dest);
        }
    }

    // Verify CFG
    let cfg = &func.cfg;
    assert_debug_snapshot!(cfg);
}

#[test]
fn test_terminator_targets() {
    let return_term = Terminator::new(
        TerminatorKind::Return { value: create_dummy_value(), ty: IrType::Void },
        dummy_span(),
    );
    assert_debug_snapshot!(return_term);

    let branch_term = Terminator::new(
        TerminatorKind::Branch { label: "target".to_string() },
        dummy_span(),
    );
    assert_debug_snapshot!(branch_term);

    let cond_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_dummy_value(),
            true_label: "true".to_string(),
            false_label: "false".to_string(),
        },
        dummy_span(),
    );

    assert_debug_snapshot!(cond_term);

    let switch_term = Terminator::new(
        TerminatorKind::Switch {
            value: create_dummy_value(),
            ty: IrType::I32,
            default_label: "default".to_string(),
            cases: vec![
                (create_dummy_value(), "case1".to_string()),
                (create_dummy_value(), "case2".to_string()),
            ],
        },
        dummy_span(),
    );
    let mut targets = switch_term.get_targets();
    targets.sort();
    assert_debug_snapshot!(targets);

    let indirect_term = Terminator::new(
        TerminatorKind::IndirectBranch {
            address: create_dummy_value(),
            possible_labels: vec!["l1".to_string(), "l2".to_string()],
        },
        dummy_span(),
    );
    assert_debug_snapshot!(indirect_term);
}

#[test]
fn test_function_with_parameters() {
    let params = vec![
        IrParameter {
            name: "a".to_string(),
            ty: IrType::I32,
            attributes: ParamAttributes::default(),
        },
        IrParameter {
            name: "b".to_string(),
            ty: IrType::F64,
            attributes: ParamAttributes::default(),
        },
    ];

    let func = Function::new("func", params, IrType::Bool);
    let output = format!("{}", func);

    assert_debug_snapshot!(output);
}