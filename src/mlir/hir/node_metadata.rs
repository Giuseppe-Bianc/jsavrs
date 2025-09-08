// src/mlir/hir/node_metadata.rs
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        NodeId::new()
    }
}

// Display implementation for NodeId
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeMetadata {
    parent: Option<NodeId>,
    id: NodeId,
}
impl NodeMetadata {
	#[inline]
    pub fn new(parent: Option<NodeId>) -> Self {
        NodeMetadata { parent, id: NodeId::new() }
    }
	
	#[inline]
    pub fn node_id(&self) -> NodeId {
        self.id
    }
}

impl fmt::Display for NodeMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parent {
            Some(p) => write!(f, "NodeMetadata(id: {}, parent: {})", self.id, p),
            None => write!(f, "NodeMetadata(id: {}, parent: None)", self.id),
        }
    }
}
impl Default for NodeMetadata {
    fn default() -> Self {
        NodeMetadata { parent: None, id: NodeId::new() }
    }
}
