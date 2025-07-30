//src/semantic/symbol_table.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::{Parameter, Type};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Variable(VariableSymbol),
    Function(FunctionSymbol),
    TypeAlias(Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableSymbol {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
    pub defined_at: SourceSpan,
    pub last_assignment: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSymbol {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub defined_at: SourceSpan,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    current_function: Option<FunctionSymbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Global scope
            current_function: None,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn declare(&mut self, name: &str, symbol: Symbol) -> Result<(), CompileError> {
        let current_scope = self.scopes.last_mut().expect("Always at least one scope");

        if current_scope.contains_key(name) {
            return Err(CompileError::TypeError {
                message: format!("Duplicate identifier '{name}' in same scope"),
                span: match current_scope.get(name) {
                    Some(Symbol::Variable(v)) => v.defined_at.clone(),
                    Some(Symbol::Function(f)) => f.defined_at.clone(),
                    _ => SourceSpan::default(),
                },
            });
        }

        current_scope.insert(name.to_string(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym.clone());
            }
        }
        None
    }

    pub fn lookup_function(&self, name: &str) -> Option<FunctionSymbol> {
        self.lookup(name).and_then(|sym| match sym {
            Symbol::Function(f) => Some(f),
            _ => None,
        })
    }

    pub fn lookup_variable(&self, name: &str) -> Option<VariableSymbol> {
        self.lookup(name).and_then(|sym| match sym {
            Symbol::Variable(v) => Some(v),
            _ => None,
        })
    }

    pub fn enter_function(&mut self, func: FunctionSymbol) {
        self.current_function = Some(func);
    }

    pub fn exit_function(&mut self) {
        self.current_function = None;
    }

    pub fn current_function(&self) -> Option<&FunctionSymbol> {
        self.current_function.as_ref()
    }
}
