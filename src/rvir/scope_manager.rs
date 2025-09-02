// src/rvir/scope_manager.rs
use super::scope::RScope;
use super::types::RScopeId;
use crate::rvir::RValue;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RScopeManager {
    scopes: HashMap<RScopeId, RScope>,
    current_scope: RScopeId,
    root_scope: RScopeId,
}

impl RScopeManager {
    pub fn new() -> Self {
        let root_id = RScopeId::new();
        let root_scope = RScope::new(None, 0);

        let mut scopes = HashMap::new();
        scopes.insert(root_id, root_scope);

        RScopeManager {
            scopes,
            current_scope: root_id,
            root_scope: root_id,
        }
    }

    pub fn enter_scope(&mut self) -> RScopeId {
        let new_id = RScopeId::new();
        let depth = self.scopes[&self.current_scope].depth + 1;
        let new_scope = RScope::new(Some(self.current_scope), depth);

        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
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
    pub fn get_scopes(&self) -> &HashMap<RScopeId, RScope> {
        &self.scopes
    }

    pub fn current_scope(&self) -> RScopeId {
        self.current_scope
    }

    pub fn add_symbol(&mut self, name: impl Into<String>, mut value: RValue) {
        value.scope = Some(self.current_scope);
        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&RValue> {
        let mut current = self.current_scope;

        loop {
            let scope = self.scopes.get(&current).expect("scope id must exist");
            if let Some(value) = scope.get(name) {
                return Some(value);
            }

            if let Some(parent) = scope.parent {
                current = parent;
            } else {
                return None;
            }
        }
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut RValue> {
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

impl Default for RScopeManager {
    fn default() -> Self {
        Self::new()
    }
}
