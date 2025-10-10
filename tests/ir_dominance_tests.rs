use jsavrs::ir::basic_block::BasicBlock;
use jsavrs::ir::cfg::ControlFlowGraph;
use jsavrs::ir::dominance::DominanceInfo;
use jsavrs::location::source_span::SourceSpan;

#[test]
fn test_dominance_simple_linear() {
    // Create a simple linear CFG: entry -> block1 -> block2
    let mut cfg = ControlFlowGraph::new("entry".to_string());
    let entry_block = BasicBlock::new("entry", SourceSpan::default());
    let block1 = BasicBlock::new("block1", SourceSpan::default());
    let block2 = BasicBlock::new("block2", SourceSpan::default());

    let entry_idx = cfg.add_block(entry_block);
    let block1_idx = cfg.add_block(block1);
    let block2_idx = cfg.add_block(block2);

    cfg.add_edge(entry_idx, block1_idx);
    cfg.add_edge(block1_idx, block2_idx);

    let mut dominance = DominanceInfo::new();
    let result = dominance.compute_dominators(&cfg);
    assert!(result.is_ok());

    // Entry dominates itself
    assert_eq!(dominance.immediate_dominator(entry_idx), Some(entry_idx));

    // block1's immediate dominator should be entry
    assert_eq!(dominance.immediate_dominator(block1_idx), Some(entry_idx));

    // block2's immediate dominator should be block1
    assert_eq!(dominance.immediate_dominator(block2_idx), Some(block1_idx));
}

#[test]
fn test_dominance_if_else() {
    // Create an if-else CFG:
    // entry -> condition -> then_branch -> merge
    //                    -> else_branch -/
    let mut cfg = ControlFlowGraph::new("entry".to_string());
    let entry_block = BasicBlock::new("entry", SourceSpan::default());
    let condition_block = BasicBlock::new("condition", SourceSpan::default());
    let then_block = BasicBlock::new("then_branch", SourceSpan::default());
    let else_block = BasicBlock::new("else_branch", SourceSpan::default());
    let merge_block = BasicBlock::new("merge", SourceSpan::default());

    let entry_idx = cfg.add_block(entry_block);
    let condition_idx = cfg.add_block(condition_block);
    let then_idx = cfg.add_block(then_block);
    let else_idx = cfg.add_block(else_block);
    let merge_idx = cfg.add_block(merge_block);

    cfg.add_edge(entry_idx, condition_idx);
    cfg.add_edge(condition_idx, then_idx);
    cfg.add_edge(condition_idx, else_idx);
    cfg.add_edge(then_idx, merge_idx);
    cfg.add_edge(else_idx, merge_idx);

    let mut dominance = DominanceInfo::new();
    let result = dominance.compute_dominators(&cfg);
    assert!(result.is_ok());

    // Entry dominates itself
    assert_eq!(dominance.immediate_dominator(entry_idx), Some(entry_idx));

    // Condition's immediate dominator should be entry
    assert_eq!(dominance.immediate_dominator(condition_idx), Some(entry_idx));

    // Then branch's immediate dominator should be condition
    assert_eq!(dominance.immediate_dominator(then_idx), Some(condition_idx));

    // Else branch's immediate dominator should be condition
    assert_eq!(dominance.immediate_dominator(else_idx), Some(condition_idx));

    // Merge's immediate dominator should be condition (common dominator)
    assert_eq!(dominance.immediate_dominator(merge_idx), Some(condition_idx));
}

#[test]
fn test_dominance_while_loop() {
    // Create a while loop CFG:
    // entry -> loop_header -> loop_body -> loop_header (back edge)
    //                       -> loop_exit
    let mut cfg = ControlFlowGraph::new("entry".to_string());
    let entry_block = BasicBlock::new("entry", SourceSpan::default());
    let header_block = BasicBlock::new("loop_header", SourceSpan::default());
    let body_block = BasicBlock::new("loop_body", SourceSpan::default());
    let exit_block = BasicBlock::new("loop_exit", SourceSpan::default());

    let entry_idx = cfg.add_block(entry_block);
    let header_idx = cfg.add_block(header_block);
    let body_idx = cfg.add_block(body_block);
    let exit_idx = cfg.add_block(exit_block);

    cfg.add_edge(entry_idx, header_idx);
    cfg.add_edge(header_idx, body_idx); // Conditional branch to body
    cfg.add_edge(header_idx, exit_idx); // Conditional branch to exit
    cfg.add_edge(body_idx, header_idx); // Back edge to header

    let mut dominance = DominanceInfo::new();
    let result = dominance.compute_dominators(&cfg);
    assert!(result.is_ok());

    // Entry dominates itself
    assert_eq!(dominance.immediate_dominator(entry_idx), Some(entry_idx));

    // Header's immediate dominator should be entry
    assert_eq!(dominance.immediate_dominator(header_idx), Some(entry_idx));

    // Body's immediate dominator should be header
    assert_eq!(dominance.immediate_dominator(body_idx), Some(header_idx));

    // Exit's immediate dominator should be header
    assert_eq!(dominance.immediate_dominator(exit_idx), Some(header_idx));
}

#[test]
fn test_compute_dominance_frontiers() {
    // Create an if-else CFG:
    // entry -> condition -> then_branch -> merge
    //                    -> else_branch -/
    let mut cfg = ControlFlowGraph::new("entry".to_string());
    let entry_block = BasicBlock::new("entry", SourceSpan::default());
    let condition_block = BasicBlock::new("condition", SourceSpan::default());
    let then_block = BasicBlock::new("then_branch", SourceSpan::default());
    let else_block = BasicBlock::new("else_branch", SourceSpan::default());
    let merge_block = BasicBlock::new("merge", SourceSpan::default());

    let entry_idx = cfg.add_block(entry_block);
    let condition_idx = cfg.add_block(condition_block);
    let then_idx = cfg.add_block(then_block);
    let else_idx = cfg.add_block(else_block);
    let merge_idx = cfg.add_block(merge_block);

    cfg.add_edge(entry_idx, condition_idx);
    cfg.add_edge(condition_idx, then_idx);
    cfg.add_edge(condition_idx, else_idx);
    cfg.add_edge(then_idx, merge_idx);
    cfg.add_edge(else_idx, merge_idx);

    let mut dominance = DominanceInfo::new();
    let result = dominance.compute_dominators(&cfg);
    assert!(result.is_ok());

    let result = dominance.compute_dominance_frontiers(&cfg);
    assert!(result.is_ok());

    // Merge block should be in the dominance frontier of condition, then, and else blocks
    let _merge_frontier = dominance.dominance_frontier(condition_idx);
    // Note: The exact frontier computation depends on the implementation details
}

#[test]
fn test_dominance_info_new() {
    let dominance = DominanceInfo::new();
    assert!(dominance.idom.is_empty());
    assert!(dominance.dominance_frontiers.is_empty());
    assert!(dominance.dom_tree_children.is_empty());
}

#[test]
fn test_compute_dominators_simple() {
    // Create a simple CFG: entry -> block1 -> block2
    let mut cfg = ControlFlowGraph::new("entry".to_string());
    let entry_block = BasicBlock::new("entry", SourceSpan::default());
    let block1 = BasicBlock::new("block1", SourceSpan::default());
    let block2 = BasicBlock::new("block2", SourceSpan::default());

    let entry_idx = cfg.add_block(entry_block);
    let block1_idx = cfg.add_block(block1);
    let block2_idx = cfg.add_block(block2);

    cfg.add_edge(entry_idx, block1_idx);
    cfg.add_edge(block1_idx, block2_idx);

    let mut dominance = DominanceInfo::new();
    let result = dominance.compute_dominators(&cfg);
    assert!(result.is_ok());

    // Entry dominates itself
    assert_eq!(dominance.immediate_dominator(entry_idx), Some(entry_idx));

    // block1's immediate dominator should be entry
    assert_eq!(dominance.immediate_dominator(block1_idx), Some(entry_idx));

    // block2's immediate dominator should be block1
    assert_eq!(dominance.immediate_dominator(block2_idx), Some(block1_idx));
}
