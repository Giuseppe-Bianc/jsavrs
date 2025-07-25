use jsavrs::nir::{BasicBlock, Cfg};
use jsavrs::utils::dummy_span;

#[test]
fn test_cfg_creation() {
    let cfg = Cfg::new("entry");
    assert_eq!(cfg.entry_label, "entry");
    assert_eq!(cfg.blocks.len(), 1);
    assert!(cfg.blocks.contains_key("entry"));
    assert_eq!(cfg.successors.get("entry").unwrap().len(), 0);
    assert_eq!(cfg.predecessors.get("entry").unwrap().len(), 0);
}

#[test]
fn test_cfg_add_block() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);

    assert_eq!(cfg.blocks.len(), 2);
    assert!(cfg.blocks.contains_key("block1"));
    assert_eq!(cfg.successors.get("block1").unwrap().len(), 0);
    assert_eq!(cfg.predecessors.get("block1").unwrap().len(), 0);
}

#[test]
fn test_cfg_add_edge() {
    let mut cfg = Cfg::new("entry");
    let block = BasicBlock::new("block1", dummy_span());
    cfg.add_block(block);
    cfg.add_edge("entry", "block1");

    assert_eq!(cfg.successors.get("entry").unwrap().len(), 1);
    assert!(cfg.successors.get("entry").unwrap().contains("block1"));
    assert_eq!(cfg.predecessors.get("block1").unwrap().len(), 1);
    assert!(cfg.predecessors.get("block1").unwrap().contains("entry"));

    let block = cfg.blocks.get("block1").unwrap();
    assert_eq!(block.predecessors, vec!["entry"]);
}