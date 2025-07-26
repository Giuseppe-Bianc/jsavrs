use jsavrs::nir::{
    BasicBlock, Cfg, Function, FunctionAttributes, Instruction, InstructionKind, IrLiteralValue,
    IrParameter, IrType, ParamAttributes, Terminator, TerminatorKind, Value,
};
use jsavrs::utils::dummy_span;

fn create_dummy_value() -> Value {
    Value::new_literal(IrLiteralValue::I32(42))
}

#[test]
fn test_cfg_creation() {
    let cfg = Cfg::new("entry");
    assert_eq!(cfg.entry_label, "entry");
    assert_eq!(cfg.blocks.len(), 1);
    assert!(cfg.blocks.contains_key("entry"));
    assert_eq!(cfg.successors.get("entry").unwrap().len(), 0);
    assert_eq!(cfg.predecessors.get("entry").unwrap().len(), 0);
}

#[test]
fn test_cfg_add_block() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    assert_eq!(cfg.blocks.len(), 2);
    assert!(cfg.blocks.contains_key("block1"));
    assert_eq!(cfg.successors.get("block1").unwrap().len(), 0);
    assert_eq!(cfg.predecessors.get("block1").unwrap().len(), 0);
}

#[test]
fn test_cfg_add_edge() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);
    cfg.add_edge("entry", "block1");

    assert_eq!(cfg.successors.get("entry").unwrap().len(), 1);
    assert!(cfg.successors.get("entry").unwrap().contains("block1"));
    assert_eq!(cfg.predecessors.get("block1").unwrap().len(), 1);
    assert!(cfg.predecessors.get("block1").unwrap().contains("entry"));

    let block = cfg.blocks.get("block1").unwrap();
    assert_eq!(block.predecessors, vec!["entry"]);
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

    assert_eq!(func.name, "test");
    assert_eq!(func.parameters, params);
    assert_eq!(func.return_type, IrType::Void);
    assert_eq!(func.cfg.entry_label, "entry_test");
    assert!(func.cfg.blocks.contains_key("entry_test"));
    assert_eq!(func.local_vars.len(), 0);
}

#[test]
fn test_function_add_block() {
    let mut func = Function::new("test", vec![], IrType::Void);
    let block = BasicBlock::new("block1", dummy_span());
    func.add_block(block);

    assert!(func.cfg.blocks.contains_key("block1"));
}

#[test]
fn test_function_add_local() {
    let mut func = Function::new("test", vec![], IrType::Void);
    func.add_local("var1".to_string(), IrType::I32);

    assert_eq!(func.local_vars.len(), 1);
    assert_eq!(func.local_vars.get("var1").unwrap(), &IrType::I32);
}

#[test]
fn test_function_add_edge() {
    let mut func = Function::new("test", vec![], IrType::Void);
    let block = BasicBlock::new("block1", dummy_span());
    func.add_block(block);
    func.add_edge("entry_test", "block1");

    let cfg = &func.cfg;
    assert!(cfg.successors.get("entry_test").unwrap().contains("block1"));
    assert!(cfg.predecessors.get("block1").unwrap().contains("entry_test"));
}

#[test]
fn test_basic_block_creation() {
    let block = BasicBlock::new("block1", dummy_span());

    assert_eq!(block.label, "block1");
    assert_eq!(block.instructions.len(), 0);
    assert_eq!(
        block.terminator.kind,
        TerminatorKind::Unreachable
    );
    assert_eq!(block.predecessors.len(), 0);
    assert!(block.dominator_info.is_none());
}

#[test]
fn test_basic_block_add_predecessor() {
    let mut block = BasicBlock::new("block1", dummy_span());
    block.add_predecessor("pred1".to_string());
    block.add_predecessor("pred2".to_string());

    assert_eq!(block.predecessors.len(), 2);
    assert!(block.predecessors.contains(&"pred1".to_string()));
    assert!(block.predecessors.contains(&"pred2".to_string()));

    // Test deduplication
    block.add_predecessor("pred1".to_string());
    assert_eq!(block.predecessors.len(), 2);
}

#[test]
fn test_basic_block_display() {
    let mut block = BasicBlock::new("block1", dummy_span());

    // Empty block
    assert_eq!(
        format!("{}", block),
        "block1:\n  unreachable"
    );

    // With predecessors
    block.add_predecessor("pred1".to_string());
    block.add_predecessor("pred2".to_string());
    assert_eq!(
        format!("{}", block),
        "// Predecessors: pred1, pred2\nblock1:\n  unreachable"
    );

    // With instructions
    let value = create_dummy_value();
    let inst =  Instruction::new(InstructionKind::Load {
        src: value.clone(),
        ty: IrType::I32,
    }, dummy_span());
    block.instructions.push(inst);

    block.terminator = Terminator::new(
        TerminatorKind::Branch("exit".to_string()),
        dummy_span(),
    );

    assert_eq!(
        format!("{}", block),
        "// Predecessors: pred1, pred2
block1:
  load i32 from 42i32
  br exit"
    );
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
        TerminatorKind::Branch("block1".to_string()),
        dummy_span(),
    );
    block1.terminator = Terminator::new(
        TerminatorKind::Branch("exit".to_string()),
        dummy_span(),
    );
    exit_block.terminator = Terminator::new(
        TerminatorKind::Return{value:create_dummy_value(),ty: IrType::Void},
        dummy_span(),
    );

    // Add blocks to function
    func.add_block(entry_block);
    func.add_block(block1);
    func.add_block(exit_block);

    // Add edges
    func.add_edge("entry_test", "block1");
    func.add_edge("block1", "exit");

    let expected_output = r#"function test () -> void:
entry_test:
  br block1

// Predecessors: entry_test
block1:
  br exit

// Predecessors: block1
exit:
  ret 42i32 void

"#;

    assert_eq!(format!("{}", func), expected_output);
}

#[test]
fn test_function_attributes() {
    let mut attrs = FunctionAttributes::default();
    attrs.is_entry = true;
    attrs.is_varargs = true;
    attrs.calling_convention = "fast".to_string();

    assert!(attrs.is_entry);
    assert!(attrs.is_varargs);
    assert_eq!(attrs.calling_convention, "fast");
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

    assert_eq!(param.name, "arg");
    assert_eq!(param.ty, IrType::I32);
    assert!(param.attributes.by_val);
    assert!(param.attributes.no_alias);
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
    assert_eq!(cfg.successors.get("entry").unwrap().len(), 2);
    assert!(cfg.successors.get("entry").unwrap().contains("a"));
    assert!(cfg.successors.get("entry").unwrap().contains("b"));

    assert_eq!(cfg.predecessors.get("c").unwrap().len(), 2);
    assert!(cfg.predecessors.get("c").unwrap().contains("a"));
    assert!(cfg.predecessors.get("c").unwrap().contains("b"));

    assert_eq!(cfg.predecessors.get("exit").unwrap().len(), 1);
    assert!(cfg.predecessors.get("exit").unwrap().contains("c"));
}

#[test]
fn test_terminator_targets() {
    let return_term = Terminator::new(
        TerminatorKind::Return{value: create_dummy_value(),ty: IrType::Void},
        dummy_span(),
    );
    assert_eq!(return_term.get_targets(), Vec::<String>::new());

    let branch_term = Terminator::new(
        TerminatorKind::Branch("target".to_string()),
        dummy_span(),
    );
    assert_eq!(branch_term.get_targets(), vec!["target"]);

    let cond_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_dummy_value(),
            true_label: "true".to_string(),
            false_label: "false".to_string(),
        },
        dummy_span(),
    );
    assert_eq!(cond_term.get_targets(), vec!["true", "false"]);

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
    assert_eq!(targets, vec!["case1", "case2", "default"]);

    let indirect_term = Terminator::new(
        TerminatorKind::IndirectBranch {
            address: create_dummy_value(),
            possible_labels: vec!["l1".to_string(), "l2".to_string()],
        },
        dummy_span(),
    );
    assert_eq!(indirect_term.get_targets(), vec!["l1", "l2"]);
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

    assert!(output.contains("function func (a: i32, b: f64) -> bool:"));
}

#[test]
fn test_cfg_get_block() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    // Test existing block
    let retrieved_block = cfg.get_block("block1");
    assert!(retrieved_block.is_some());
    assert_eq!(retrieved_block.unwrap().label, "block1");

    // Test non-existent block
    assert!(cfg.get_block("invalid").is_none());

    // Test entry block
    assert!(cfg.get_block("entry").is_some());
}

#[test]
fn test_cfg_get_block_mut() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    // Modify existing block
    if let Some(block) = cfg.get_block_mut("block1") {
        block.add_predecessor("new_pred".to_string());
    }

    let retrieved_block = cfg.get_block("block1").unwrap();
    assert_eq!(retrieved_block.predecessors, vec!["new_pred"]);

    // Try to modify non-existent block
    assert!(cfg.get_block_mut("invalid").is_none());

    // Modify entry block
    if let Some(entry) = cfg.get_block_mut("entry") {
        entry.terminator = Terminator::new(
            TerminatorKind::Branch("new_target".to_string()),
            dummy_span(),
        );
    }

    let entry = cfg.get_block("entry").unwrap();
    match &entry.terminator.kind {
        TerminatorKind::Branch(label) => assert_eq!(label, "new_target"),
        _ => panic!("Terminator not modified correctly"),
    }
}

#[test]
fn test_function_cfg_accessors() {
    let mut func = Function::new("test", vec![], IrType::Void);
    let block = BasicBlock::new("block1", dummy_span());
    func.add_block(block);

    // Test get_block through function's cfg
    assert!(func.cfg.get_block("block1").is_some());
    assert!(func.cfg.get_block("entry_test").is_some());
    assert!(func.cfg.get_block("invalid").is_none());

    // Test get_block_mut through function's cfg
    if let Some(block) = func.cfg.get_block_mut("block1") {
        block.add_predecessor("func_pred".to_string());
    }

    let block = func.cfg.get_block("block1").unwrap();
    assert!(block
        .predecessors
        .contains(&"func_pred".to_string()));
}

#[test]
fn test_cfg_get_block_mut_persists_changes() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    // Modify block through mutable reference
    {
        let block = cfg.get_block_mut("block1").unwrap();
        block.instructions.push(Instruction::new(
            InstructionKind::Load {
                src: create_dummy_value(),
                ty: IrType::I32,
            },
            dummy_span(),
        ));
    }

    // Verify changes persisted
    let block = cfg.get_block("block1").unwrap();
    assert_eq!(block.instructions.len(), 1);
}

#[test]
fn test_cfg_get_block_mut_entry_block() {
    let mut cfg = Cfg::new("entry");

    // Modify entry block
    if let Some(entry) = cfg.get_block_mut("entry") {
        entry.terminator = Terminator::new(
            TerminatorKind::Unreachable,
            dummy_span(),
        );
    }

    let entry = cfg.get_block("entry").unwrap();
    assert!(matches!(
        entry.terminator.kind,
        TerminatorKind::Unreachable
    ));
}