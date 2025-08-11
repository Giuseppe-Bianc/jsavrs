// src/ir/symbol_table.rs
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use crate::ir::types::Type;
use crate::ir::values::{Value, /*GlobalRef, ArgumentRef*/};

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable {
        name: String,
        ty: Type,
        value: Value,
        is_mutable: bool,
        is_global: bool,
        alignment: Option<u32>,
    },
    Function {
        name: String,
        ty: Type,
        parameters: Vec<Parameter>,
        is_variadic: bool,
    },
    Type {
        name: String,
        ty: Type,
        is_opaque: bool,
    },
    Label {
        name: String,
        block: String,
    },
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
    pub attributes: Vec<ParameterAttribute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterAttribute {
    ByVal,
    NoAlias,
    NoCapture,
    NonNull,
    ReadOnly,
    WriteOnly,
    InReg,
    SRet,
    Align(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeKind {
    Global,
    Function,
    Block,
    Loop,
    Conditional,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Weak<RefCell<Scope>>>,
    pub level: u32,
    pub kind: ScopeKind,
}

impl Scope {
    pub fn new(kind: ScopeKind, level: u32) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
            level,
            kind,
        }
    }

    pub fn add_symbol(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            return Err(format!("Symbol '{}' already defined in this scope", name));
        }
        self.symbols.insert(name, symbol);
        Ok(())
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn get_symbol_recursive(&self, name: &str) -> Option<Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            return Some(symbol.clone());
        }

        if let Some(parent_weak) = &self.parent {
            if let Some(parent) = parent_weak.upgrade() {
                return parent.borrow().get_symbol_recursive(name);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Rc<RefCell<Scope>>>,
    current_level: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global_scope = Rc::new(RefCell::new(Scope::new(ScopeKind::Global, 0)));
        Self {
            scopes: vec![global_scope],
            current_level: 0,
        }
    }

    pub fn enter_scope(&mut self, kind: ScopeKind) {
        self.current_level += 1;
        let new_scope = Rc::new(RefCell::new(Scope::new(kind, self.current_level)));

        // Set parent to current scope
        if let Some(current) = self.scopes.last() {
            new_scope.borrow_mut().parent = Some(Rc::downgrade(current));
        }

        self.scopes.push(new_scope);
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.current_level -= 1;
        }
    }

    pub fn current_scope(&self) -> Option<Rc<RefCell<Scope>>> {
        self.scopes.last().cloned()
    }

    pub fn add_symbol(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        if let Some(scope) = self.current_scope() {
            scope.borrow_mut().add_symbol(name, symbol)
        } else {
            Err("No current scope".to_string())
        }
    }

    pub fn get_symbol(&self, name: &str) -> Option<Symbol> {
        if let Some(scope) = self.current_scope() {
            scope.borrow().get_symbol_recursive(name)
        } else {
            None
        }
    }

    pub fn get_symbol_in_current_scope(&self, name: &str) -> Option<Symbol> {
        if let Some(scope) = self.current_scope() {
            scope.borrow().get_symbol(name).cloned()
        } else {
            None
        }
    }

    pub fn add_variable(&mut self, name: String, ty: Type, value: Value, is_mutable: bool, is_global: bool) -> Result<(), String> {
        let symbol = Symbol::Variable {
            name: name.clone(),
            ty,
            value,
            is_mutable,
            is_global,
            alignment: None,
        };
        self.add_symbol(name, symbol)
    }

    pub fn add_function(&mut self, name: String, ty: Type, parameters: Vec<Parameter>, is_variadic: bool) -> Result<(), String> {
        let symbol = Symbol::Function {
            name: name.clone(),
            ty,
            parameters,
            is_variadic,
        };
        self.add_symbol(name, symbol)
    }

    pub fn add_type(&mut self, name: String, ty: Type, is_opaque: bool) -> Result<(), String> {
        let symbol = Symbol::Type {
            name: name.clone(),
            ty,
            is_opaque,
        };
        self.add_symbol(name, symbol)
    }

    pub fn add_label(&mut self, name: String, block: String) -> Result<(), String> {
        let symbol = Symbol::Label {
            name: name.clone(),
            block,
        };
        self.add_symbol(name, symbol)
    }

    pub fn get_variable(&self, name: &str) -> Option<(Type, Value, bool)> {
        if let Some(Symbol::Variable { ty, value, is_mutable, .. }) = self.get_symbol(name) {
            Some((ty, value, is_mutable))
        } else {
            None
        }
    }

    pub fn get_function(&self, name: &str) -> Option<(Type, Vec<Parameter>, bool)> {
        if let Some(Symbol::Function { ty, parameters, is_variadic, .. }) = self.get_symbol(name) {
            Some((ty, parameters.clone(), is_variadic))
        } else {
            None
        }
    }

    pub fn get_type(&self, name: &str) -> Option<Type> {
        if let Some(Symbol::Type { ty, .. }) = self.get_symbol(name) {
            Some(ty)
        } else {
            None
        }
    }

    pub fn get_label(&self, name: &str) -> Option<String> {
        if let Some(Symbol::Label { block, .. }) = self.get_symbol(name) {
            Some(block)
        } else {
            None
        }
    }
}