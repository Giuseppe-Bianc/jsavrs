// src/nir/scope_manager.rs
use super::scope::Scope;
use super::types::ScopeId;
use crate::nir::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeManager {
    scopes: HashMap<ScopeId, Scope>,
    current_scope: ScopeId,
    root_scope: ScopeId,
}

impl ScopeManager {
    pub fn new() -> Self {
        let root_id = ScopeId::new();
        let root_scope = Scope::new(None, 0);

        let mut scopes = HashMap::new();
        scopes.insert(root_id, root_scope);

        ScopeManager {
            scopes,
            current_scope: root_id,
            root_scope: root_id,
        }
    }

    pub fn enter_scope(&mut self) -> ScopeId {
        let new_id = ScopeId::new();
        let depth = self.scopes[&self.current_scope].depth + 1;
        let new_scope = Scope::new(Some(self.current_scope), depth);

        self.scopes
            .get_mut(&self.current_scope)
            .unwrap()
            .children
            .push(new_id);

        self.scopes.insert(new_id, new_scope);
        self.current_scope = new_id;
        new_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[&self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    // Nuovo metodo pubblico per ottenere una copia della mappa degli scope
    pub fn get_scopes(&self) -> HashMap<ScopeId, Scope> {
        self.scopes.clone()
    }

    pub fn current_scope(&self) -> ScopeId {
        self.current_scope
    }

    pub fn add_symbol(&mut self, name: Arc<str>, mut value: Value) {
        value.scope = Some(self.current_scope);
        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .insert(name, value);
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        let mut current = self.current_scope;

        loop {
            if let Some(value) = self.scopes[&current].get(name) {
                return Some(value);
            }

            if let Some(parent) = self.scopes[&current].parent {
                current = parent;
            } else {
                return None;
            }
        }
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Value> {
        let mut current = self.current_scope;

        loop {
            if self.scopes[&current].symbols.contains_key(name) {
                return self.scopes.get_mut(&current).unwrap().symbols.get_mut(name);
            }

            if let Some(parent) = self.scopes[&current].parent {
                current = parent;
            } else {
                return None;
            }
        }
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}
