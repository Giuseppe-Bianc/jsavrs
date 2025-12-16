//! Unit tests for `ExecutableEdgeSet` in SCCP propagator
//!
//! Tests T061 from User Story 2

use jsavrs::ir::optimizer::constant_folding::propagator::{CFGEdge, ExecutableEdgeSet};

// ============================================================================
// T061: Unit tests for ExecutableEdgeSet operations
// ============================================================================

#[test]
fn test_executable_edge_set_new() {
    let edge_set = ExecutableEdgeSet::new();
    assert!(!edge_set.has_executable_predecessor(0), "Newly created ExecutableEdgeSet should have no edges");
}

#[test]
fn test_executable_edge_set_with_capacity() {
    let edge_set = ExecutableEdgeSet::with_capacity(100);
    assert!(!edge_set.has_executable_predecessor(0), "ExecutableEdgeSet with capacity should have no edges initially");
}

#[test]
fn test_mark_executable_first_time() {
    let mut edge_set = ExecutableEdgeSet::new();
    let edge = CFGEdge::new(0, 1);

    let first_time = edge_set.mark_executable(edge);
    assert!(first_time, "Marking an edge executable for the first time should return true");
}

#[test]
fn test_mark_executable_second_time() {
    let mut edge_set = ExecutableEdgeSet::new();
    let edge = CFGEdge::new(0, 1);

    edge_set.mark_executable(edge);
    let second_time = edge_set.mark_executable(edge);

    assert!(!second_time, "Marking an edge executable a second time should return false");
}

#[test]
fn test_is_executable_marked_edge() {
    let mut edge_set = ExecutableEdgeSet::new();
    let edge = CFGEdge::new(0, 1);

    edge_set.mark_executable(edge);
    assert!(edge_set.is_executable(&edge), "Marked edge should be executable");
}

#[test]
fn test_is_executable_unmarked_edge() {
    let edge_set = ExecutableEdgeSet::new();
    let edge = CFGEdge::new(0, 1);

    assert!(!edge_set.is_executable(&edge), "Unmarked edge should not be executable");
}

#[test]
fn test_has_executable_predecessor_no_edges() {
    let edge_set = ExecutableEdgeSet::new();

    assert!(!edge_set.has_executable_predecessor(1), "Block with no predecessors should return false");
}

#[test]
fn test_has_executable_predecessor_with_edge() {
    let mut edge_set = ExecutableEdgeSet::new();
    let edge = CFGEdge::new(0, 1);

    edge_set.mark_executable(edge);
    assert!(edge_set.has_executable_predecessor(1), "Block 1 should have executable predecessor from block 0");
}

#[test]
fn test_has_executable_predecessor_wrong_block() {
    let mut edge_set = ExecutableEdgeSet::new();
    let edge = CFGEdge::new(0, 1);

    edge_set.mark_executable(edge);
    assert!(!edge_set.has_executable_predecessor(2), "Block 2 should have no executable predecessors");
}

#[test]
fn test_multiple_predecessors() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Block 2 has predecessors from blocks 0 and 1
    edge_set.mark_executable(CFGEdge::new(0, 2));
    edge_set.mark_executable(CFGEdge::new(1, 2));

    assert!(edge_set.has_executable_predecessor(2), "Block 2 should have executable predecessors");

    let predecessors: Vec<usize> = edge_set.executable_predecessors(2).collect();
    assert_eq!(predecessors.len(), 2, "Block 2 should have exactly 2 executable predecessors");
    assert!(predecessors.contains(&0), "Block 0 should be a predecessor of block 2");
    assert!(predecessors.contains(&1), "Block 1 should be a predecessor of block 2");
}

#[test]
fn test_executable_predecessors_empty() {
    let edge_set = ExecutableEdgeSet::new();

    assert!(
        edge_set.executable_predecessors(1).next().is_none(),
        "Block with no executable edges should have no predecessors"
    );
}

#[test]
fn test_executable_predecessors_single() {
    let mut edge_set = ExecutableEdgeSet::new();
    edge_set.mark_executable(CFGEdge::new(0, 1));

    let predecessors: Vec<usize> = edge_set.executable_predecessors(1).collect();

    assert_eq!(predecessors.len(), 1, "Block 1 should have 1 predecessor");
    assert_eq!(predecessors[0], 0, "Predecessor should be block 0");
}

#[test]
fn test_executable_predecessors_multiple() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Block 3 has predecessors from blocks 0, 1, 2
    edge_set.mark_executable(CFGEdge::new(0, 3));
    edge_set.mark_executable(CFGEdge::new(1, 3));
    edge_set.mark_executable(CFGEdge::new(2, 3));

    let predecessors: Vec<usize> = edge_set.executable_predecessors(3).collect();

    assert_eq!(predecessors.len(), 3, "Block 3 should have 3 predecessors");
    assert!(predecessors.contains(&0));
    assert!(predecessors.contains(&1));
    assert!(predecessors.contains(&2));
}

#[test]
fn test_executable_predecessors_excludes_successors() {
    let mut edge_set = ExecutableEdgeSet::new();

    // 0 → 1 → 2
    edge_set.mark_executable(CFGEdge::new(0, 1));
    edge_set.mark_executable(CFGEdge::new(1, 2));

    let predecessors: Vec<usize> = edge_set.executable_predecessors(1).collect();

    assert_eq!(predecessors.len(), 1, "Block 1 should have 1 predecessor");
    assert_eq!(predecessors[0], 0, "Only block 0 should be predecessor of 1");
    assert!(!predecessors.contains(&2), "Block 2 is a successor, not a predecessor");
}

#[test]
fn test_cfg_edge_equality() {
    let edge1 = CFGEdge::new(0, 1);
    let edge2 = CFGEdge::new(0, 1);
    let edge3 = CFGEdge::new(1, 0);

    assert_eq!(edge1, edge2, "Edges with same from/to should be equal");
    assert_ne!(edge1, edge3, "Edges with different from/to should not be equal");
}

#[test]
fn test_cfg_edge_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    let edge1 = CFGEdge::new(0, 1);
    let edge2 = CFGEdge::new(0, 1);

    set.insert(edge1);
    assert!(set.contains(&edge2), "HashSet should find equivalent edge");
    assert_eq!(set.len(), 1, "Duplicate edges should not increase set size");
}

#[test]
fn test_complex_cfg_diamond() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Diamond CFG:
    //     0
    //    / \
    //   1   2
    //    \ /
    //     3
    edge_set.mark_executable(CFGEdge::new(0, 1));
    edge_set.mark_executable(CFGEdge::new(0, 2));
    edge_set.mark_executable(CFGEdge::new(1, 3));
    edge_set.mark_executable(CFGEdge::new(2, 3));

    // Verify entry block
    assert!(!edge_set.has_executable_predecessor(0), "Entry block should have no predecessors");

    // Verify branch blocks
    assert!(edge_set.has_executable_predecessor(1));
    assert!(edge_set.has_executable_predecessor(2));

    // Verify merge block
    let predecessors: Vec<usize> = edge_set.executable_predecessors(3).collect();
    assert_eq!(predecessors.len(), 2, "Merge block should have 2 predecessors");
    assert!(predecessors.contains(&1));
    assert!(predecessors.contains(&2));
}

#[test]
fn test_unreachable_path_in_diamond() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Diamond CFG with one unreachable path:
    //     0
    //    / \
    //   1   2 (unreachable)
    //    \ /
    //     3
    edge_set.mark_executable(CFGEdge::new(0, 1));
    // Note: CFGEdge(0, 2) NOT marked - path unreachable
    edge_set.mark_executable(CFGEdge::new(1, 3));
    edge_set.mark_executable(CFGEdge::new(2, 3)); // Edge exists but 2 is unreachable

    assert!(!edge_set.has_executable_predecessor(2), "Block 2 should be unreachable");

    let predecessors: Vec<usize> = edge_set.executable_predecessors(3).collect();
    assert_eq!(predecessors.len(), 2, "Block 3 should have 2 edges");

    // However, block 2 is not reachable from entry, so only path from 1 is truly executable
    assert!(predecessors.contains(&1), "Path from block 1 should be executable");
}

#[test]
fn test_loop_edges() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Simple loop: 0 → 1 → 2 → 1 (back edge)
    edge_set.mark_executable(CFGEdge::new(0, 1));
    edge_set.mark_executable(CFGEdge::new(1, 2));
    edge_set.mark_executable(CFGEdge::new(2, 1)); // Back edge

    let predecessors: Vec<usize> = edge_set.executable_predecessors(1).collect();
    assert_eq!(predecessors.len(), 2, "Loop header should have 2 predecessors");
    assert!(predecessors.contains(&0), "Entry edge to loop");
    assert!(predecessors.contains(&2), "Back edge from loop body");
}

#[test]
fn test_self_loop() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Block 1 loops to itself
    edge_set.mark_executable(CFGEdge::new(1, 1));

    assert!(edge_set.has_executable_predecessor(1), "Self-loop block should have itself as predecessor");

    let predecessors: Vec<usize> = edge_set.executable_predecessors(1).collect();
    assert_eq!(predecessors.len(), 1);
    assert_eq!(predecessors[0], 1, "Self-loop predecessor should be itself");
}

#[test]
fn test_large_cfg() {
    let mut edge_set = ExecutableEdgeSet::new();

    // Create a larger CFG: 0 → 1 → 2 → ... → 99 → 100
    for i in 0..100 {
        edge_set.mark_executable(CFGEdge::new(i, i + 1));
    }

    // Verify chain structure
    for i in 1..=100 {
        assert!(edge_set.has_executable_predecessor(i), "Block {i} should have executable predecessor");
        let predecessors: Vec<usize> = edge_set.executable_predecessors(i).collect();
        assert_eq!(predecessors.len(), 1);
        assert_eq!(predecessors[0], i - 1);
    }

    assert!(!edge_set.has_executable_predecessor(0), "Entry block should have no predecessors");
}
