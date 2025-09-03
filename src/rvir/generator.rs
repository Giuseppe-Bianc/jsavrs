// src/rvir/generator.rs
use super::*;
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::tokens::number::Number;
use std::collections::HashMap;

pub struct RIrGenerator {
    current_block: Option<RBasicBlock>,
    current_block_label: Option<String>,
    scope_manager: RScopeManager,
    temp_counter: u64,
    block_counter: usize,
    errors: Vec<CompileError>,
    break_stack: Vec<String>,
    continue_stack: Vec<String>,
    type_context: TypeContext,
    access_controller: RAccessController,
    root_scope: Option<RScopeId>,
}

#[derive(Debug, Default)]
struct TypeContext {
    structs: HashMap<String, (Vec<(String, RIrType)>, SourceSpan)>,
    _aliases: HashMap<String, RIrType>,
}

impl RIrGenerator {
    pub fn new() -> Self {
        let scope_manager = RScopeManager::new();
        let access_controller = RAccessController::new(&scope_manager);
        Self {
            current_block: None,
            current_block_label: None,
            scope_manager: scope_manager.clone(),
            temp_counter: 0,
            block_counter: 0,
            errors: Vec::new(),
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            access_controller,
            type_context: TypeContext::default(),
            root_scope: scope_manager.root_scope(),
        }
    }

    fn new_error(&mut self, message: impl Into<String>, span: SourceSpan) {
        self.errors.push(CompileError::IrGeneratorError {
            message: message.into(),
            span,
            help: None,
        });
    }

    pub fn generate(&mut self, stmts: Vec<Stmt>, module_name: &str) -> (Module, Vec<CompileError>) {
        let mut module = Module::new(module_name, self.root_scope);
        for stmt in stmts {
            self.visit_top_stmt(&stmt, &mut module); // Process the statement in the  global scope
        }
        (module, std::mem::take(&mut self.errors))
    }

    fn visit_top_stmt(&mut self, stmt: &Stmt, module: &mut Module) {
        match stmt {
            Stmt::Function {
                name,
                parameters,
                return_type,
                body,
                span,
            } => {
                let mut func =
                    self.create_function(name, parameters, return_type.clone(), span.clone());
                self.generate_function_body(&mut func, body.clone(), span.clone());
                module.add_function(func.clone());
            }
            Stmt::MainFunction { body, span } => {
                let mut func = self.create_function("main", &[], Type::Void, span.clone());
                self.generate_function_body(&mut func, body.clone(), span.clone());
                module.add_function(func);
            }
            other => {
                self.new_error(
                    format!("Unsupported top-level statement: {:?}", other),
                    other.span().clone(),
                );
            }
        }
    }

    fn create_function(
        &mut self,
        name: &str,
        parameters: &[Parameter],
        return_type: Type,
        span: SourceSpan,
    ) -> Function {
        let ir_params = parameters
            .iter()
            .map(|param| {
                let ty = self.map_type(&param.type_annotation);
                IrParameter {
                    name: param.name.clone(),
                    ty: ty.clone(),
                    attributes: ParamAttributes {
                        source_span: Some(param.span.clone()),
                        ..Default::default()
                    },
                }
            })
            .collect();
        let ir_return_type = self.map_type(&return_type);

        let mut func = Function::new(name, ir_params, ir_return_type);
        func.attributes.source_span = Some(span);
        func
    }

    fn map_type(&self, ty: &Type) -> RIrType {
        match ty {
            Type::I8 => RIrType::I8,
            Type::I16 => RIrType::I16,
            Type::I32 => RIrType::I32,
            Type::I64 => RIrType::I64,
            Type::U8 => RIrType::U8,
            Type::U16 => RIrType::U16,
            Type::U32 => RIrType::U32,
            Type::U64 => RIrType::U64,
            Type::F32 => RIrType::F32,
            Type::F64 => RIrType::F64,
            Type::Char => RIrType::Char,
            Type::String => RIrType::String,
            Type::Bool => RIrType::Bool,
            Type::Custom(name) => {
                if let Some((fields, span)) = self.type_context.structs.get(name) {
                    RIrType::Struct(name.clone(), fields.clone(), span.clone())
                } else {
                    RIrType::Custom(name.clone(), SourceSpan::default())
                }
            }
            Type::Array(element_type, size_expr) => {
                if let Expr::Literal {
                    value: LiteralValue::Number(Number::Integer(size)),
                    ..
                } = **size_expr
                {
                    RIrType::Array(Box::new(self.map_type(element_type)), size as usize)
                } else {
                    RIrType::Pointer(Box::new(self.map_type(element_type)))
                }
            }
            Type::Vector(element_type) => RIrType::Pointer(Box::new(self.map_type(element_type))),
            Type::Void => RIrType::Void,
            Type::NullPtr => RIrType::Pointer(Box::new(RIrType::I8)),
        }
    }

    fn generate_function_body(&mut self, func: &mut Function, _body: Vec<Stmt>, _span: SourceSpan) {
        self.break_stack.clear();
        self.continue_stack.clear();
        func.enter_scope();
        // Here you would generate the actual body of the function
        let entry_label = format!("entry_{}", func.name);
        func.add_block(&entry_label, _span.clone());
        self.current_block_label = Some(entry_label.clone());
        self.current_block = func.cfg.get_block_mut(&entry_label).cloned();

        if let Some(block) = &self.current_block
            && matches!(block.terminator.kind, RTerminatorKind::Unreachable) {
            let return_value = match func.return_type {
                RIrType::Void => RValue::new_literal(RIrLiteralValue::I32(0)),
                _ => RValue::new_literal(RIrLiteralValue::I32(0)),
            };
            func.set_terminator(self.current_block_label.clone().unwrap().as_str(), RTerminator::new(
                    RTerminatorKind::Return {
                        value: return_value,
                        ty: func.return_type.clone(),
                    },
                    SourceSpan::default(),
                ));
        }

        func.exit_scope();
        self.scope_manager.append_manager(&func.scope_manager);
    }
}

impl Default for RIrGenerator {
    fn default() -> Self {
        Self::new()
    }
}
