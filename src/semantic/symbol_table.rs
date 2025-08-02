// src/semantic/symbol_table.rs
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Global,
    Function,
    Block,
    //Struct,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub kind: ScopeKind,
    pub symbols: HashMap<String, Symbol>,
    pub defined_at: Option<SourceSpan>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_function: Option<FunctionSymbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope {
                kind: ScopeKind::Global,
                symbols: HashMap::new(),
                defined_at: None,
            }],
            current_function: None,
        }
    }

    pub fn push_scope(&mut self, kind: ScopeKind, defined_at: Option<SourceSpan>) {
        self.scopes.push(Scope {
            kind,
            symbols: HashMap::new(),
            defined_at,
        });
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    pub fn current_scope(&self) -> Option<&Scope> {
        self.scopes.last()
    }

    pub fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.scopes.last_mut()
    }

    pub fn current_scope_kind(&self) -> Option<ScopeKind> {
        self.current_scope().map(|s| s.kind)
    }

    pub fn declare(&mut self, name: &str, symbol: Symbol) -> Result<(), CompileError> {
        let current_scope = self.current_scope_mut().expect("At least one scope");

        if current_scope.symbols.contains_key(name) {
            return Err(CompileError::TypeError {
                message: format!(
                    "Identifier '{}' already declared in this {:?} scope",
                    name, current_scope.kind
                ),
                span: match current_scope.symbols.get(name) {
                    Some(Symbol::Variable(v)) => v.defined_at.clone(),
                    Some(Symbol::Function(f)) => f.defined_at.clone(),
                    _ => SourceSpan::default(),
                },
            });
        }

        current_scope.symbols.insert(name.to_string(), symbol);
        Ok(())
    }

    // Helper method to find symbols with a custom filter
    #[allow(clippy::collapsible_if)]
    fn find_symbol<F, T>(&self, name: &str, filter: F) -> Option<T>
    where
        F: Fn(&Symbol) -> Option<T>,
    {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                if let Some(result) = filter(sym) {
                    return Some(result);
                }
            }
        }
        None
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        self.find_symbol(name, |sym| Some(sym.clone()))
    }

    pub fn lookup_function(&self, name: &str) -> Option<FunctionSymbol> {
        self.find_symbol(name, |sym| match sym {
            Symbol::Function(f) => Some(f.clone()),
            _ => None,
        })
    }

    pub fn lookup_variable(&self, name: &str) -> Option<VariableSymbol> {
        self.find_symbol(name, |sym| match sym {
            Symbol::Variable(v) => Some(v.clone()),
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

    pub fn current_function_return_type(&self) -> Option<Type> {
        self.current_function().map(|f| f.return_type.clone())
    }
}
