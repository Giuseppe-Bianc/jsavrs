use insta::assert_snapshot;
use jsavrs::nir::{BasicBlock, Instruction, InstructionKind, IrBinaryOp, IrLiteralValue, IrType, TerminatorKind, Value};
use jsavrs::utils::dummy_span;

#[test]
fn test_new_block() {
    let block = BasicBlock::new("entry", Default::default());
    assert_snapshot!(block.to_string())
}

#[test]
fn test_new_block_predecessors() {
    let mut block = BasicBlock::new("entry", Default::default());
    block.add_predecessor("prev".to_string());
    assert_snapshot!(block.to_string())
}

#[test]
fn test_block_display_empty() {
    let block: BasicBlock = BasicBlock::new("entry", Default::default());
    assert_snapshot!(block.to_string())
}

#[test]
fn test_block_display_whit_predecessor() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    block.add_predecessor("prev".to_string());
    assert_snapshot!(block.to_string())
}

#[test]
fn test_block_display_whit_instruction() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    let left = Value::new_literal(IrLiteralValue::I32(100i32));
    let right = Value::new_literal(IrLiteralValue::I32(200i32));

    let inst = Instruction::new(
        InstructionKind::Binary {
            op: IrBinaryOp::Add,
            left: left.clone(),
            right: right.clone(),
            ty: IrType::I32,
        },
        dummy_span(),
    ).with_result(Value::new_temporary(1000, IrType::I32));
    block.instructions.push(inst);
    assert_snapshot!(block.to_string())
}
#[test]
fn test_block_display_whit_instruction_and_predecessor() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    let left = Value::new_literal(IrLiteralValue::I32(100i32));
    let right = Value::new_literal(IrLiteralValue::I32(200i32));

    let inst = Instruction::new(
        InstructionKind::Binary {
            op: IrBinaryOp::Add,
            left: left.clone(),
            right: right.clone(),
            ty: IrType::I32,
        },
        dummy_span(),
    ).with_result(Value::new_temporary(1000, IrType::I32));
    block.add_predecessor("prev".to_string());
    block.instructions.push(inst);
    assert_snapshot!(block.to_string())
}