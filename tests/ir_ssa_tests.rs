use jsavrs::ir::function::Function;
use jsavrs::ir::ssa::SsaTransformer;
//use jsavrs::ir::cfg::ControlFlowGraph;
use jsavrs::ir::basic_block::BasicBlock;
use jsavrs::ir::instruction::{Instruction, InstructionKind};
use jsavrs::ir::terminator::{Terminator, TerminatorKind};
use jsavrs::ir::types::IrType;
use jsavrs::ir::value::{/*ValueKind,*/ IrLiteralValue, Value};
use jsavrs::location::source_span::SourceSpan;
use std::sync::Arc;

#[test]
fn test_ssa_transformer_new() {
    let _transformer = SsaTransformer::new(None);
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
    let alloca_inst = Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default());
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
        TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::Void },
        SourceSpan::default(),
    );
    block.set_terminator(return_term);

    // Add the block to the function's CFG
    let _node_idx = func.cfg.add_block(block);
    func.cfg.entry_label = "entry".to_string();

    // Transform to SSA form
    let mut transformer = SsaTransformer::new(None);
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
    let alloca_inst = Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default());
    entry_block.instructions.push(alloca_inst);
    let branch_term = Terminator::new(TerminatorKind::Branch { label: Arc::from("condition") }, SourceSpan::default());
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
    let branch_term2 = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
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
    let branch_term3 = Terminator::new(TerminatorKind::Branch { label: Arc::from("merge") }, SourceSpan::default());
    else_block.set_terminator(branch_term3);
    func.cfg.add_block(else_block);

    // Merge block
    let mut merge_block = BasicBlock::new("merge", SourceSpan::default());
    let return_term = Terminator::new(
        TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::Void },
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
    let mut transformer = SsaTransformer::new(None);
    let result = transformer.transform_function(&mut func);

    // The transformation should succeed
    assert!(result.is_ok());
}

#[test]
fn test_simple_if_else_transformation() {
    // Create a simple function with an if-else structure
    let mut func = Function::new("test", vec![], IrType::Void);

    // Create blocks
    let _entry_block = BasicBlock::new("entry", SourceSpan::default());
    let _then_block = BasicBlock::new("then", SourceSpan::default());
    let _else_block = BasicBlock::new("else", SourceSpan::default());
    let _merge_block = BasicBlock::new("merge", SourceSpan::default());

    // Add blocks to function
    func.add_block("entry", SourceSpan::default());
    func.add_block("then", SourceSpan::default());
    func.add_block("else", SourceSpan::default());
    func.add_block("merge", SourceSpan::default());

    // Set entry block
    func.cfg.entry_label = "entry".to_string();

    // Create a variable x
    let x_var = Value::new_temporary(0, IrType::I32).with_debug_info(Some("x".into()), SourceSpan::default());

    // Add an alloca instruction for x in entry block
    let alloca_inst =
        Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default()).with_result(x_var.clone());

    func.add_instruction("entry", alloca_inst);

    // Add store instructions in then and else blocks
    let const_10 = Value::new_literal(IrLiteralValue::I32(10));
    let store_then =
        Instruction::new(InstructionKind::Store { value: const_10, dest: x_var.clone() }, SourceSpan::default());
    func.add_instruction("then", store_then);

    let const_20 = Value::new_literal(IrLiteralValue::I32(20));
    let store_else =
        Instruction::new(InstructionKind::Store { value: const_20, dest: x_var.clone() }, SourceSpan::default());
    func.add_instruction("else", store_else);

    // Transform to SSA
    let mut transformer = SsaTransformer::new(None);
    let result = transformer.transform_function(&mut func);

    // The transformation should succeed
    if let Err(e) = &result {
        eprintln!("SSA transformation error in test_simple_if_else_transformation: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_loop_transformation() {
    // Create a simple function with a loop structure
    let mut func = Function::new("test_loop", vec![], IrType::Void);

    // Create blocks
    let _entry_block = BasicBlock::new("entry", SourceSpan::default());
    let _loop_header = BasicBlock::new("loop_header", SourceSpan::default());
    let _loop_body = BasicBlock::new("loop_body", SourceSpan::default());
    let _loop_exit = BasicBlock::new("loop_exit", SourceSpan::default());

    // Add blocks to function
    func.add_block("entry", SourceSpan::default());
    func.add_block("loop_header", SourceSpan::default());
    func.add_block("loop_body", SourceSpan::default());
    func.add_block("loop_exit", SourceSpan::default());

    // Set entry block
    func.cfg.entry_label = "entry".to_string();

    // Create a variable i
    let i_var = Value::new_temporary(0, IrType::I32).with_debug_info(Some("i".into()), SourceSpan::default());

    // Add an alloca instruction for i in entry block
    let alloca_inst =
        Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default()).with_result(i_var.clone());

    func.add_instruction("entry", alloca_inst);

    // Add store instruction to initialize i in loop header
    let const_0 = Value::new_literal(IrLiteralValue::I32(0));
    let store_init =
        Instruction::new(InstructionKind::Store { value: const_0, dest: i_var.clone() }, SourceSpan::default());
    func.add_instruction("loop_header", store_init);

    // Add store instruction to increment i in loop body
    let const_1 = Value::new_literal(IrLiteralValue::I32(1));
    let _load_i =
        Instruction::new(InstructionKind::Load { src: i_var.clone(), ty: IrType::I32 }, SourceSpan::default());
    // For simplicity, we'll just add a store that overwrites i in the loop body
    let store_inc =
        Instruction::new(InstructionKind::Store { value: const_1, dest: i_var.clone() }, SourceSpan::default());
    func.add_instruction("loop_body", store_inc);

    // Transform to SSA
    let mut transformer = SsaTransformer::new(None);
    let result = transformer.transform_function(&mut func);

    // The transformation should succeed
    if let Err(e) = &result {
        eprintln!("SSA transformation error in test_loop_transformation: {}", e);
    }
    assert!(result.is_ok());
}
