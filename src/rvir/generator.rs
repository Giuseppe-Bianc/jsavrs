// src/rvir/generator.rs
use super::*;
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
//use crate::tokens::number::Number;
//use std::collections::HashMap;
///use crate::nir::{AccessController, ScopeManager};

pub struct RIrGenerator {
    current_block: Option<RBasicBlock>,
    current_block_label: Option<String>,
    scope_manager: RScopeManager,
    temp_counter: u64,
    block_counter: usize,
    errors: Vec<CompileError>,
    break_stack: Vec<String>,
    continue_stack: Vec<String>,
    access_controller: RAccessController,
}

impl RIrGenerator {
    pub fn new() -> Self {
        let scope_manager = RScopeManager::new();
        let access_controller = RAccessController::new(&scope_manager);
        Self {
            current_block: None,
            current_block_label: None,
            scope_manager,
            temp_counter: 0,
            block_counter: 0,
            errors: Vec::new(),
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            access_controller,
        }
    }

    fn new_error(&mut self, message: impl Into<String>, span: SourceSpan) {
        self.errors.push(CompileError::IrGeneratorError { message: message.into(), span, help: None });
    }

    pub fn generate(&mut self, stmts: Vec<Stmt>, module_name: &str) -> (Module, Vec<CompileError>) {
        let mut module = Module::new(module_name);
        for stmt in stmts {
            self.visit_stmt(&stmt, &mut module);
        }
        (module, std::mem::take(&mut self.errors))
    }

    fn visit_stmt(&mut self, stmt: &Stmt, module: &mut Module) {
        match stmt {
            Stmt::Function { name, parameters, return_type, body, span} => {
                let mut func: Function =
                    self.create_function(&name, &parameters, return_type, span.clone());
            }
            Stmt::MainFunction {body, span } => {}
            other => {
                self.new_error(
                    "Unsupported top-level statement",
                    other.span().clone(),
                );
            }
        }
    }

    fn create_function(&self, name: &&String, parameters: &&Vec<Parameter>, return_type: &Type, span: SourceSpan) -> Function {
        todo!()
    }
}