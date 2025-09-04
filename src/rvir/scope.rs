//src/rvir/scope.rs
use std::collections::HashMap;
use std::sync::Arc;
use super::{RScopeId, RValue};

#[derive(Debug, Clone, PartialEq)]
pub struct RScope {
    pub symbols: HashMap<Arc<str>, RValue>,
    pub parent: Option<RScopeId>,
    pub children: Vec<RScopeId>,
    pub depth: usize,
}

impl RScope {
    pub fn new(parent: Option<RScopeId>, depth: usize) -> Self {
        RScope {
            symbols: HashMap::new(),
            parent,
            children: Vec::new(),
            depth,
        }
    }

    pub fn insert(&mut self, name: Arc<str>, value: RValue) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&RValue> {
        self.symbols.get(name)
    }
}