use jsavrs::ir::ssa::SsaTransformer;
use jsavrs::ir::function::Function;
use jsavrs::ir::cfg::ControlFlowGraph;
use jsavrs::ir::basic_block::BasicBlock;
use jsavrs::ir::instruction::{Instruction, InstructionKind};
use jsavrs::ir::terminator::{Terminator, TerminatorKind};
use jsavrs::ir::value::{Value, ValueKind, IrLiteralValue};
use jsavrs::ir::types::IrType;
use jsavrs::location::source_span::SourceSpan;
use std::sync::Arc;

#[test]
fn test_ssa_transformer_new() {
    let transformer = SsaTransformer::new();
    // Just check that we can create it
    assert!(true);
}

#[test]
fn test_simple_ssa_transformation() {
    // Create a simple function with a single block
    let mut func = Function::new("test", vec![], IrType::Void);
    
    // Create a basic block with some instructions
    let mut block = BasicBlock::new("entry", SourceSpan::default());
    
    // Add an alloca instruction
    let alloca_inst = Instruction::new(
        InstructionKind::Alloca { ty: IrType::I32 },
        SourceSpan::default(),
    );
    block.instructions.push(alloca_inst);
    
    // Add a store instruction
    let store_inst = Instruction::new(
        InstructionKind::Store {
            value: Value::new_literal(IrLiteralValue::I32(42)),
            dest: Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))),
        },
        SourceSpan::default(),
    );
    block.instructions.push(store_inst);
    
    // Add a return terminator
    let return_term = Terminator::new(
        TerminatorKind::Return {
            value: Value::new_literal(IrLiteralValue::I32(0)),
            ty: IrType::Void,
        },
        SourceSpan::default(),
    );
    block.set_terminator(return_term);
    
    // Add the block to the function's CFG
    let node_idx = func.cfg.add_block(block);
    func.cfg.entry_label = "entry".to_string();
    
    // Transform to SSA form
    let mut transformer = SsaTransformer::new();
    let result = transformer.transform_function(&mut func);
    
    // The transformation should succeed
    assert!(result.is_ok());
    
    // The function should still have one block
    assert_eq!(func.cfg.graph().node_count(), 1);
}

#[test]
fn test_ssa_with_if_else() {
    // Create a function with if-else control flow
    let mut func = Function::new("test", vec![], IrType::Void);
    
    // Entry block
    let mut entry_block = BasicBlock::new("entry", SourceSpan::default());
    let alloca_inst = Instruction::new(
        InstructionKind::Alloca { ty: IrType::I32 },
        SourceSpan::default(),
    );
    entry_block.instructions.push(alloca_inst);
    let branch_term = Terminator::new(
        TerminatorKind::Branch {
            label: Arc::from("condition"),
        },
        SourceSpan::default(),
    );
    entry_block.set_terminator(branch_term);
    func.cfg.add_block(entry_block);
    
    // Condition block
    let mut condition_block = BasicBlock::new("condition", SourceSpan::default());
    let cond_value = Value::new_literal(IrLiteralValue::Bool(true));
    let cond_branch = Terminator::new(
        TerminatorKind::ConditionalBranch {
            condition: cond_value,
            true_label: Arc::from("then_branch"),
            false_label: Arc::from("else_branch"),
        },
        SourceSpan::default(),
    );
    condition_block.set_terminator(cond_branch);
    func.cfg.add_block(condition_block);
    
    // Then branch
    let mut then_block = BasicBlock::new("then_branch", SourceSpan::default());
    let store_inst1 = Instruction::new(
        InstructionKind::Store {
            value: Value::new_literal(IrLiteralValue::I32(20)),
            dest: Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))),
        },
        SourceSpan::default(),
    );
    then_block.instructions.push(store_inst1);
    let branch_term2 = Terminator::new(
        TerminatorKind::Branch {
            label: Arc::from("merge"),
        },
        SourceSpan::default(),
    );
    then_block.set_terminator(branch_term2);
    func.cfg.add_block(then_block);
    
    // Else branch
    let mut else_block = BasicBlock::new("else_branch", SourceSpan::default());
    let store_inst2 = Instruction::new(
        InstructionKind::Store {
            value: Value::new_literal(IrLiteralValue::I32(30)),
            dest: Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))),
        },
        SourceSpan::default(),
    );
    else_block.instructions.push(store_inst2);
    let branch_term3 = Terminator::new(
        TerminatorKind::Branch {
            label: Arc::from("merge"),
        },
        SourceSpan::default(),
    );
    else_block.set_terminator(branch_term3);
    func.cfg.add_block(else_block);
    
    // Merge block
    let mut merge_block = BasicBlock::new("merge", SourceSpan::default());
    let return_term = Terminator::new(
        TerminatorKind::Return {
            value: Value::new_literal(IrLiteralValue::I32(0)),
            ty: IrType::Void,
        },
        SourceSpan::default(),
    );
    merge_block.set_terminator(return_term);
    func.cfg.add_block(merge_block);
    
    // Connect the blocks
    func.cfg.connect_blocks("entry", "condition");
    func.cfg.connect_blocks("condition", "then_branch");
    func.cfg.connect_blocks("condition", "else_branch");
    func.cfg.connect_blocks("then_branch", "merge");
    func.cfg.connect_blocks("else_branch", "merge");
    
    func.cfg.entry_label = "entry".to_string();
    
    // Transform to SSA form
    let mut transformer = SsaTransformer::new();
    let result = transformer.transform_function(&mut func);
    
    // The transformation should succeed
    assert!(result.is_ok());
}