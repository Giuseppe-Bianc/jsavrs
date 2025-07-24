use jsavrs::nir::{BasicBlock, Instruction, InstructionKind, IrType, IrUnaryOp, Terminator, TerminatorKind, Value, ValueKind};

#[test]
fn test_new_block() {
    let block = BasicBlock::new("entry", Default::default());
    assert_eq!(block.label, "entry");
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator.kind, TerminatorKind::Unreachable);
}

#[test]
fn test_new_block_predecessors() {
    let mut block = BasicBlock::new("entry", Default::default());
    block.add_predecessor("prev".to_string());
    assert_eq!(block.label, "entry");
    assert_eq!(block.predecessors.len(), 1);
    assert!(block.instructions.is_empty());
    assert_eq!(block.terminator.kind, TerminatorKind::Unreachable);
}

#[test]
fn test_block_display_empty() {
    let block: BasicBlock = BasicBlock::new("entry", Default::default());
    assert_eq!(block.to_string(), "entry:\n  unreachable");
}

#[test]
fn test_block_display_whit_predecessor() {
    let mut block: BasicBlock = BasicBlock::new("entry", Default::default());
    block.add_predecessor("prev".to_string());
    assert_eq!(block.to_string(), "// Predecessors: prev\nentry:\n  unreachable");
}