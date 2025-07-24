use jsavrs::nir::{BasicBlock, Instruction, InstructionKind, IrType, IrUnaryOp, Terminator, TerminatorKind, Value, ValueKind};

#[test]
fn test_new_block() {
    let block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry");
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator.kind, TerminatorKind::Unreachable);
}