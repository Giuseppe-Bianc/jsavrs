use jsavrs::nir::{
    BasicBlock, Instruction, InstructionKind, IrBinaryOp, IrLiteralValue, IrType, TerminatorKind, Value,
};
use jsavrs::utils::dummy_span;

#[test]
fn test_new_block() {
    let block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry".into());
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator().kind, TerminatorKind::Unreachable);
}

/*#[test]
fn test_new_block_predecessors() {
    let mut block = BasicBlock::new("entry", Default::default());
    block.add_predecessor("prev".to_string());
    assert_eq!(block.label, "entry");
    assert_eq!(block.predecessors.len(), 1);
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator().kind, TerminatorKind::Unreachable);
}*/

#[test]
fn test_block_display_empty() {
    let block: BasicBlock = BasicBlock::new("entry", Default::default());
    assert_eq!(block.to_string(), "entry:\n  unreachable\n");
}

/*#[test]
fn test_block_display_whit_predecessor() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    block.add_predecessor("prev".to_string());
    assert_eq!(block.to_string(), "// Predecessors: prev\nentry:\n  unreachable");
}*/

#[test]
fn test_block_display_whit_instruction() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    let left = Value::new_literal(IrLiteralValue::I32(100i32));
    let right = Value::new_literal(IrLiteralValue::I32(200i32));

    let inst = Instruction::new(
        InstructionKind::Binary { op: IrBinaryOp::Add, left: left.clone(), right: right.clone(), ty: IrType::I32 },
        dummy_span(),
    )
    .with_result(Value::new_temporary(1000, IrType::I32));
    block.instructions.push(inst);
    assert_eq!(block.to_string(), "entry:\n  t1000 = add 100i32 200i32, i32\n  unreachable\n");
}
/*#[test]
fn test_block_display_whit_instruction_and_predecessor() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    let left = Value::new_literal(IrLiteralValue::I32(100i32));
    let right = Value::new_literal(IrLiteralValue::I32(200i32));

    let inst = Instruction::new(
        InstructionKind::Binary { op: IrBinaryOp::Add, left: left.clone(), right: right.clone(), ty: IrType::I32 },
        dummy_span(),
    )
    .with_result(Value::new_temporary(1000, IrType::I32));
    block.add_predecessor("prev".to_string());
    block.instructions.push(inst);
    assert_eq!(block.to_string(), "// Predecessors: prev\nentry:\n  t1000 = add 100i32 200i32, i32\n  unreachable");
}
*/