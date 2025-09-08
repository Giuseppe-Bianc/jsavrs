//src/ir/scope.rs
use super::{ScopeId, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub symbols: HashMap<Arc<str>, Value>,
    pub parent: Option<ScopeId>,
    pub children: Vec<ScopeId>,
    pub depth: usize,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>, depth: usize) -> Self {
        Scope { symbols: HashMap::new(), parent, children: Vec::new(), depth }
    }

    pub fn insert(&mut self, name: Arc<str>, value: Value) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.symbols.get(name)
    }
}
