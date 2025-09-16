// src/ir/scope_manager.rs
use super::scope::Scope;
use super::types::ScopeId;
use crate::ir::Value;
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

        ScopeManager { scopes, current_scope: root_id, root_scope: root_id }
    }

    pub fn enter_scope(&mut self) -> ScopeId {
        let new_id = ScopeId::new();
        let depth = self.scopes[&self.current_scope].depth + 1;
        let new_scope = Scope::new(Some(self.current_scope), depth);

        self.scopes.get_mut(&self.current_scope).expect("current scope must exist in scopes map").children.push(new_id);

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
    pub fn get_scopes(&self) -> &HashMap<ScopeId, Scope> {
        &self.scopes
    }

    pub fn current_scope(&self) -> ScopeId {
        self.current_scope
    }

    pub fn add_symbol(&mut self, name: impl Into<Arc<str>>, mut value: Value) {
        value.scope = Some(self.current_scope);
        self.scopes
            .get_mut(&self.current_scope)
            .expect("current scope must exist in scopes map")
            .insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
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

    pub fn root_scope(&self) -> Option<ScopeId> {
        Some(self.root_scope)
    }

    pub fn append_manager(&mut self, other: &ScopeManager) {
        let root_id = self.root_scope;

        // Create a mapping from old scope IDs to new scope IDs to avoid collisions
        let mut id_mapping: HashMap<ScopeId, ScopeId> = HashMap::new();

        // First, create new IDs for all scopes in the other manager (except its root)
        for (scope_id, _) in other.scopes.iter() {
            if *scope_id != other.root_scope {
                id_mapping.insert(*scope_id, ScopeId::new());
            }
        }

        // Now process each scope from the other manager
        for (old_scope_id, scope) in other.scopes.iter() {
            // Skip the root scope of the other manager
            if *old_scope_id == other.root_scope {
                continue;
            }

            // Get the new ID for this scope
            let new_scope_id = *id_mapping.get(old_scope_id).unwrap();

            let mut new_scope = scope.clone();

            // Update parent references using the mapping
            if let Some(old_parent_id) = new_scope.parent {
                if old_parent_id == other.root_scope {
                    // If parent was the other manager's root, make it our root
                    new_scope.parent = Some(root_id);
                    new_scope.depth = self.scopes[&root_id].depth + 1;
                } else {
                    // If parent was another scope in the other manager, map it to the new ID
                    if let Some(new_parent_id) = id_mapping.get(&old_parent_id) {
                        new_scope.parent = Some(*new_parent_id);
                        // Update depth based on new parent
                        if let Some(parent_scope) = self.scopes.get(new_parent_id) {
                            new_scope.depth = parent_scope.depth + 1;
                        } else {
                            new_scope.depth = self.scopes[&root_id].depth + 1;
                        }
                    } else {
                        // Fallback: connect to our root
                        new_scope.parent = Some(root_id);
                        new_scope.depth = self.scopes[&root_id].depth + 1;
                    }
                }
            } else {
                // Scope without parent: connect it to our root
                new_scope.parent = Some(root_id);
                new_scope.depth = self.scopes[&root_id].depth + 1;
            }

            // Update children references using the mapping
            let mut new_children = Vec::new();
            for child_id in &new_scope.children {
                if let Some(new_child_id) = id_mapping.get(child_id) {
                    new_children.push(*new_child_id);
                }
            }
            new_scope.children = new_children;

            self.scopes.insert(new_scope_id, new_scope);
        }

        // Update our root's children to include the top-level scopes from the other manager
        for (old_scope_id, scope) in other.scopes.iter() {
            if *old_scope_id == other.root_scope {
                continue;
            }

            // Check if this scope was a direct child of the other manager's root
            if let Some(parent_id) = scope.parent
                && parent_id == other.root_scope
            {
                let new_scope_id = *id_mapping.get(old_scope_id).unwrap();
                self.scopes.get_mut(&root_id).unwrap().children.push(new_scope_id);
            }
        }

        // Update current_scope to the mapped version of the other manager's current scope
        // Map the other manager's current scope to the new ID
        if other.current_scope != other.root_scope
            && let Some(new_current_scope_id) = id_mapping.get(&other.current_scope)
        {
            self.current_scope = *new_current_scope_id;
        }
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}
