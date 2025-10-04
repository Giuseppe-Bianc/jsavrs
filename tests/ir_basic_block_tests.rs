use jsavrs::ir::{
    BasicBlock, Instruction, InstructionKind, IrBinaryOp, IrLiteralValue, IrType, ScopeId, TerminatorKind, Value,
};
use jsavrs::utils::dummy_span;

#[test]
fn test_new_block() {
    let block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry".into());
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator().kind, TerminatorKind::Unreachable);
}

#[test]
fn test_block_terminator_mut() {
    let mut block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry".into());
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator_mut().kind, TerminatorKind::Unreachable);
}

#[test]
fn test_block_scope() {
    let block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry".into());
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator().kind, TerminatorKind::Unreachable);
    assert_eq!(block.scope(), None);
}

#[test]
fn test_block_set_scope() {
    let mut block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry".into());
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator().kind, TerminatorKind::Unreachable);
    assert_eq!(block.scope(), None);
    block.set_scope(ScopeId::new());
    assert_eq!(block.scope().is_some(), true);
}

#[test]
fn test_block_clear_scope() {
    let mut block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry".into());
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator().kind, TerminatorKind::Unreachable);
    assert_eq!(block.scope(), None);
    block.set_scope(ScopeId::new());
    assert_eq!(block.scope().is_some(), true);
    block.clear_scope();
    assert_eq!(block.scope(), None);
}

#[test]
fn test_block_display_empty() {
    let block: BasicBlock = BasicBlock::new("entry", Default::default());
    assert_eq!(block.to_string(), "entry:\n  unreachable\n");
}

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
