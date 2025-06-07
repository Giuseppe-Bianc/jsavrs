/*use crate::{parser::ast::{Expr, Stmt}, semantic::type_checker::TypeChecker};

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_stmt(&mut self, stmt: &Stmt);
}

impl Visitor<()> for TypeChecker {
    fn visit_expr(&mut self, expr: &Expr) -> () {
        self.visit_expr(expr);
    }
    
    fn visit_stmt(&mut self, stmt: &Stmt) -> () {
        self.visit_stmt(stmt);
    }
}*/