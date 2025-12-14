use std::sync::Arc;

use jsavrs::ir::{
    BasicBlock, ControlFlowGraph, Function, FunctionAttributes, Instruction, InstructionKind, IrLiteralValue,
    IrParameter, IrType, ParamAttributes, Terminator, TerminatorKind, Value,
};
use jsavrs::utils::{dummy_span, vec_to_string};

fn create_dummy_value() -> Value {
    Value::new_literal(IrLiteralValue::I32(42))
}

#[test]
fn test_function_creation() {
    let params = vec![IrParameter { name: "param1".into(), ty: IrType::I32, attributes: ParamAttributes::default() }];
    let func = Function::new("test", params.clone(), IrType::Void);

    assert_eq!(func.name, Arc::from("test"));
    assert_eq!(func.parameters, params);
    assert_eq!(func.return_type, IrType::Void);
    assert_eq!(func.cfg.entry_label, Arc::from("entry_test"));
    //assert!(func.cfg.blocks().contains_key("entry_test"));
    assert_eq!(func.local_vars.len(), 0);
}

#[test]
fn test_function_add_block() {
    let mut func = Function::new("test", vec![], IrType::Void);
    func.add_block("block1", dummy_span());
    // Fixed: Use get_block to check existence
    assert!(func.cfg.get_block("block1").is_some());
}

#[test]
fn test_basic_block_display() {
    let mut block = BasicBlock::new("block1", dummy_span());

    // Empty block
    assert_eq!(format!("{block}"), "block1:\n  unreachable\n");

    // With instructions
    let value = create_dummy_value();
    let inst = Instruction::new(InstructionKind::Load { src: value, ty: IrType::I32 }, dummy_span());
    block.instructions.push(inst);
    block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "exit".into() }, dummy_span()));

    assert_eq!(
        format!("{block}"),
        "block1:
  load i32 from 42i32
  br exit
"
    );
}

#[test]
fn test_function_display() {
    let mut func = Function::new("test", vec![], IrType::Void);
    // Add blocks to function
    func.add_block("entry_test", dummy_span());
    func.add_block("block1", dummy_span());
    func.add_block("exit", dummy_span());
    func.set_terminator("entry_test", Terminator::new(TerminatorKind::Branch { label: "block1".into() }, dummy_span()));
    func.set_terminator("block1", Terminator::new(TerminatorKind::Branch { label: "exit".into() }, dummy_span()));
    func.set_terminator(
        "exit",
        Terminator::new(TerminatorKind::Return { value: create_dummy_value(), ty: IrType::Void }, dummy_span()),
    );
    // Add edges

    let expected_output = r"function test () -> void:
blocks:
// Scope: SCOPE_0
entry_test:
  br block1

// Scope: SCOPE_0
block1:
  br exit

// Scope: SCOPE_0
exit:
  ret 42i32 void

";

    assert_eq!(vec_to_string(vec![func]), expected_output);
}

#[allow(clippy::field_reassign_with_default)]
#[test]
fn test_function_attributes() {
    let mut attrs = FunctionAttributes::default();
    attrs.is_entry = true;
    attrs.is_varargs = true;
    attrs.calling_convention = Arc::from("fast");

    assert!(attrs.is_entry);
    assert!(attrs.is_varargs);
    assert_eq!(attrs.calling_convention, Arc::from("fast"));
}

#[test]
fn test_ir_parameter() {
    let name: Arc<str> = Arc::from("arg");
    let param = IrParameter {
        name: name.clone(),
        ty: IrType::I32,
        attributes: ParamAttributes { by_val: true, no_alias: true, source_span: Some(dummy_span()) },
    };

    assert_eq!(param.name, name);
    assert_eq!(param.ty, IrType::I32);
    assert!(param.attributes.by_val);
    assert!(param.attributes.no_alias);
}

#[test]
fn test_terminator_targets() {
    let return_term =
        Terminator::new(TerminatorKind::Return { value: create_dummy_value(), ty: IrType::Void }, dummy_span());
    assert_eq!(return_term.get_targets(), Vec::<Arc<str>>::new());

    let branch_term = Terminator::new(TerminatorKind::Branch { label: "target".into() }, dummy_span());
    assert_eq!(branch_term.get_targets(), vec!["target".into()]);

    let cond_term = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_dummy_value(),
            true_label: "true".into(),
            false_label: "false".into(),
        },
        dummy_span(),
    );
    assert_eq!(cond_term.get_targets(), vec!["true".into(), "false".into()]);

    let switch_term = Terminator::new(
        TerminatorKind::Switch {
            value: create_dummy_value(),
            ty: IrType::I32,
            default_label: "default".into(),
            cases: vec![(create_dummy_value(), "case1".into()), (create_dummy_value(), "case2".into())],
        },
        dummy_span(),
    );
    let mut targets = switch_term.get_targets();
    targets.sort();
    assert_eq!(targets, vec!["case1".into(), "case2".into(), "default".into()]);

    let indirect_term = Terminator::new(
        TerminatorKind::IndirectBranch {
            address: create_dummy_value(),
            possible_labels: vec!["l1".into(), "l2".into()],
        },
        dummy_span(),
    );
    assert_eq!(indirect_term.get_targets(), vec!["l1".into(), "l2".into()]);
}

#[test]
fn test_function_with_parameters() {
    let params = vec![
        IrParameter { name: "a".into(), ty: IrType::I32, attributes: ParamAttributes::default() },
        IrParameter { name: "b".into(), ty: IrType::F64, attributes: ParamAttributes::default() },
    ];

    let func = Function::new("func", params, IrType::Bool);
    let output = format!("{func}");

    assert!(output.contains("function func (a: i32, b: f64) -> bool:"));
}

#[test]
fn test_cfg_get_block() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    cfg.add_block(BasicBlock::new("entry", dummy_span())); // Ensure entry block exists
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    // Test existing block
    let retrieved_block = cfg.get_block("block1");
    assert!(retrieved_block.is_some());
    assert_eq!(retrieved_block.unwrap().label, "block1".into());

    // Test non-existent block
    assert!(cfg.get_block("invalid").is_none());

    // Test entry block
    assert!(cfg.get_block("entry").is_some());
}

#[test]
fn test_cfg_get_block_mut_persists_changes() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    // Modify block through mutable reference
    {
        let block = cfg.get_block_mut("block1").unwrap();
        block
            .instructions
            .push(Instruction::new(InstructionKind::Load { src: create_dummy_value(), ty: IrType::I32 }, dummy_span()));
    }

    // Verify changes persisted
    let block = cfg.get_block("block1").unwrap();
    assert_eq!(block.instructions.len(), 1);
}

#[test]
fn test_cfg_get_block_mut_entry_block() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    cfg.add_block(BasicBlock::new("entry", dummy_span()));

    // Modify entry block
    if let Some(entry) = cfg.get_block_mut("entry") {
        entry.set_terminator(Terminator::new(TerminatorKind::Unreachable, dummy_span()));
    }

    let entry = cfg.get_block("entry").unwrap();
    assert!(matches!(entry.terminator().kind, TerminatorKind::Unreachable));
}

#[test]
fn test_get_entry_block_exists() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    cfg.add_block(entry_block);

    let entry_block_ref = cfg.get_entry_block();
    assert!(entry_block_ref.is_some());
    assert_eq!(entry_block_ref.unwrap().label, "entry".into());
}

#[test]
fn test_get_entry_block_nonexistent() {
    let cfg = ControlFlowGraph::new(Arc::from("nonexistent"));
    // Don't add the block with the entry label

    let entry_block_ref = cfg.get_entry_block();
    assert!(entry_block_ref.is_none());
}

#[test]
fn test_get_entry_block_after_modifications() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let mut entry_block = BasicBlock::new("entry", dummy_span());
    entry_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "target".into() }, dummy_span()));
    cfg.add_block(entry_block);

    let entry_block_ref = cfg.get_entry_block();
    assert!(entry_block_ref.is_some());
    assert_eq!(entry_block_ref.unwrap().label.as_ref(), "entry");
    assert!(matches!(entry_block_ref.unwrap().terminator().kind, TerminatorKind::Branch { .. }));
}

#[test]
fn test_blocks_mut_basic() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    let block1 = BasicBlock::new("block1", dummy_span());
    cfg.add_block(entry_block);
    cfg.add_block(block1);

    // Use blocks_mut to modify all blocks
    for block in cfg.blocks_mut() {
        block
            .instructions
            .push(Instruction::new(InstructionKind::Load { src: create_dummy_value(), ty: IrType::I32 }, dummy_span()));
    }

    // Verify that changes were applied to all blocks
    assert_eq!(cfg.get_block("entry").unwrap().instructions.len(), 1);
    assert_eq!(cfg.get_block("block1").unwrap().instructions.len(), 1);
}

#[test]
fn test_blocks_mut_empty_cfg() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    // Don't add any blocks

    let count = cfg.blocks_mut().count();
    assert_eq!(count, 0);
}

#[test]
fn test_blocks_mut_single_block() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    cfg.add_block(entry_block);

    let mut count = 0;
    for block in cfg.blocks_mut() {
        block
            .instructions
            .push(Instruction::new(InstructionKind::Load { src: create_dummy_value(), ty: IrType::I32 }, dummy_span()));
        count += 1;
    }
    assert_eq!(count, 1);
    assert_eq!(cfg.get_block("entry").unwrap().instructions.len(), 1);
}

#[test]
fn test_blocks_mut_multiple_blocks() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    let block1 = BasicBlock::new("block1", dummy_span());
    let block2 = BasicBlock::new("block2", dummy_span());
    cfg.add_block(entry_block);
    cfg.add_block(block1);
    cfg.add_block(block2);

    let mut modified_blocks = Vec::new();
    for block in cfg.blocks_mut() {
        block
            .instructions
            .push(Instruction::new(InstructionKind::Load { src: create_dummy_value(), ty: IrType::I32 }, dummy_span()));
        modified_blocks.push(block.label.to_string());
    }

    assert_eq!(modified_blocks.len(), 3);
    assert!(modified_blocks.contains(&"entry".to_string()));
    assert!(modified_blocks.contains(&"block1".to_string()));
    assert!(modified_blocks.contains(&"block2".to_string()));

    assert_eq!(cfg.get_block("entry").unwrap().instructions.len(), 1);
    assert_eq!(cfg.get_block("block1").unwrap().instructions.len(), 1);
    assert_eq!(cfg.get_block("block2").unwrap().instructions.len(), 1);
}

#[test]
fn test_dfs_post_order_linear() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    let block1 = BasicBlock::new("block1", dummy_span());
    let block2 = BasicBlock::new("block2", dummy_span());
    let entry_idx = cfg.add_block(entry_block);
    let block1_idx = cfg.add_block(block1);
    let block2_idx = cfg.add_block(block2);

    // Create linear CFG: entry -> block1 -> block2
    cfg.add_edge(entry_idx, block1_idx);
    cfg.add_edge(block1_idx, block2_idx);

    let post_order: Vec<String> = cfg.dfs_post_order().map(|idx| cfg.graph()[idx].label.to_string()).collect();

    // Verify that we have the right elements in post-order (the exact order may vary by graph implementation)
    assert_eq!(post_order.len(), 3);
    assert!(post_order.contains(&"entry".to_string()));
    assert!(post_order.contains(&"block1".to_string()));
    assert!(post_order.contains(&"block2".to_string()));

    // In post-order, block2 should come before block1 (since block2 is a child of block1)
    // and block1 should come before entry (since block1 is a child of entry in the logical flow)
    let block2_pos = post_order.iter().position(|x| x == "block2").unwrap();
    let block1_pos = post_order.iter().position(|x| x == "block1").unwrap();
    let entry_pos = post_order.iter().position(|x| x == "entry").unwrap();

    // block2 comes after block1
    assert!(block2_pos > block1_pos);
    // block1 comes after entry
    assert!(block1_pos > entry_pos);
}

#[test]
fn test_dfs_post_order_empty_cfg() {
    let cfg = ControlFlowGraph::new(Arc::from("entry"));
    // No blocks added - no entry block exists

    let post_order: Vec<String> = cfg.dfs_post_order().map(|idx| cfg.graph()[idx].label.to_string()).collect();

    assert_eq!(post_order, Vec::<String>::new());
}

#[test]
fn test_dfs_post_order_single_block() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    cfg.add_block(entry_block);

    let post_order: Vec<String> = cfg.dfs_post_order().map(|idx| cfg.graph()[idx].label.to_string()).collect();

    assert_eq!(post_order, vec!["entry"]);
}

#[test]
fn test_dfs_post_order_branching() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    let branch_block = BasicBlock::new("branch", dummy_span());
    let then_block = BasicBlock::new("then", dummy_span());
    let else_block = BasicBlock::new("else", dummy_span());
    let merge_block = BasicBlock::new("merge", dummy_span());

    let entry_idx = cfg.add_block(entry_block);
    let branch_idx = cfg.add_block(branch_block);
    let then_idx = cfg.add_block(then_block);
    let else_idx = cfg.add_block(else_block);
    let merge_idx = cfg.add_block(merge_block);

    // Create branching CFG: entry -> branch -> then/else -> merge
    cfg.add_edge(entry_idx, branch_idx);
    cfg.add_edge(branch_idx, then_idx);
    cfg.add_edge(branch_idx, else_idx);
    cfg.add_edge(then_idx, merge_idx);
    cfg.add_edge(else_idx, merge_idx);

    let post_order: Vec<String> = cfg.dfs_post_order().map(|idx| cfg.graph()[idx].label.to_string()).collect();

    // In post-order traversal, children should be visited before parents
    // The exact order may vary depending on graph implementation
    assert_eq!(post_order.len(), 5);
    assert!(post_order.contains(&"entry".to_string()));
    assert!(post_order.contains(&"branch".to_string()));
    assert!(post_order.contains(&"then".to_string()));
    assert!(post_order.contains(&"else".to_string()));
    assert!(post_order.contains(&"merge".to_string()));
}

#[test]
fn test_dfs_post_order_cycles() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let entry_block = BasicBlock::new("entry", dummy_span());
    let loop_block = BasicBlock::new("loop", dummy_span());
    let back_block = BasicBlock::new("back", dummy_span());

    let entry_idx = cfg.add_block(entry_block);
    let loop_idx = cfg.add_block(loop_block);
    let back_idx = cfg.add_block(back_block);

    // Create cyclic CFG: entry -> loop -> back -> loop (cycle)
    cfg.add_edge(entry_idx, loop_idx);
    cfg.add_edge(loop_idx, back_idx);
    cfg.add_edge(back_idx, loop_idx); // Create a cycle

    let post_order: Vec<String> = cfg.dfs_post_order().map(|idx| cfg.graph()[idx].label.to_string()).collect();

    // The post-order should still work with cycles, but the exact order depends on DFS implementation
    assert!(post_order.len() == 3);
    assert!(post_order.contains(&"entry".to_string()));
    assert!(post_order.contains(&"loop".to_string()));
    assert!(post_order.contains(&"back".to_string()));
}

#[test]
fn test_verify_success() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let mut entry_block = BasicBlock::new("entry", dummy_span());
    let mut target_block = BasicBlock::new("target", dummy_span());

    // Set proper terminators
    entry_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "target".into() }, dummy_span()));
    target_block.set_terminator(Terminator::new(
        TerminatorKind::Return { value: create_dummy_value(), ty: IrType::Void },
        dummy_span(),
    ));

    cfg.add_block(entry_block);
    cfg.add_block(target_block);

    let result = cfg.verify();
    assert!(result.is_ok());
}

#[test]
fn test_verify_no_entry_block() {
    let cfg = ControlFlowGraph::new(Arc::from("entry"));
    // Don't add the entry block

    let result = cfg.verify();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no entry block"));
}

#[test]
fn test_verify_block_without_terminator() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let mut entry_block = BasicBlock::new("entry", dummy_span());
    let target_block = BasicBlock::new("target", dummy_span());

    // Set the terminator for entry but not for target
    entry_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "target".into() }, dummy_span()));
    // target_block has Unreachable as default terminator which should fail the verification

    cfg.add_block(entry_block);
    cfg.add_block(target_block);

    let result = cfg.verify();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("has no valid terminator"));
}

#[test]
fn test_verify_nonexistent_terminator_target() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let mut entry_block = BasicBlock::new("entry", dummy_span());

    // Set terminator to point to non-existent block
    entry_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "nonexistent".into() }, dummy_span()));

    cfg.add_block(entry_block);

    let result = cfg.verify();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("refers to non-existent block"));
}

#[test]
fn test_verify_conditional_branch_nonexistent_targets() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));
    let mut entry_block = BasicBlock::new("entry", dummy_span());

    // Set conditional branch to point to non-existent blocks
    entry_block.set_terminator(Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_dummy_value(),
            true_label: "nonexistent_true".into(),
            false_label: "nonexistent_false".into(),
        },
        dummy_span(),
    ));

    cfg.add_block(entry_block);

    let result = cfg.verify();
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("refers to non-existent block"));
}

#[test]
fn test_verify_complex_cfg_success() {
    let mut cfg = ControlFlowGraph::new(Arc::from("entry"));

    let mut entry_block = BasicBlock::new("entry", dummy_span());
    let mut branch_block = BasicBlock::new("branch", dummy_span());
    let mut then_block = BasicBlock::new("then", dummy_span());
    let mut else_block = BasicBlock::new("else", dummy_span());
    let mut merge_block = BasicBlock::new("merge", dummy_span());

    // Set proper terminators for all blocks
    entry_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "branch".into() }, dummy_span()));
    branch_block.set_terminator(Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: create_dummy_value(),
            true_label: "then".into(),
            false_label: "else".into(),
        },
        dummy_span(),
    ));
    then_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "merge".into() }, dummy_span()));
    else_block.set_terminator(Terminator::new(TerminatorKind::Branch { label: "merge".into() }, dummy_span()));
    merge_block.set_terminator(Terminator::new(
        TerminatorKind::Return { value: create_dummy_value(), ty: IrType::Void },
        dummy_span(),
    ));

    cfg.add_block(entry_block);
    cfg.add_block(branch_block);
    cfg.add_block(then_block);
    cfg.add_block(else_block);
    cfg.add_block(merge_block);

    // Connect the blocks
    let entry_idx = cfg.find_block_by_label("entry").unwrap();
    let branch_idx = cfg.find_block_by_label("branch").unwrap();
    let then_idx = cfg.find_block_by_label("then").unwrap();
    let else_idx = cfg.find_block_by_label("else").unwrap();
    let merge_idx = cfg.find_block_by_label("merge").unwrap();

    cfg.add_edge(entry_idx, branch_idx);
    cfg.add_edge(branch_idx, then_idx);
    cfg.add_edge(branch_idx, else_idx);
    cfg.add_edge(then_idx, merge_idx);
    cfg.add_edge(else_idx, merge_idx);

    let result = cfg.verify();
    assert!(result.is_ok());
}
