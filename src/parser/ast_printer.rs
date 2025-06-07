use crate::parser::ast::{Expr, LiteralValue, Stmt, Type};
use console::Style;

#[derive(Debug, Clone, PartialEq)]
struct StyleManager {
    pub operator: Style,
    pub literal: Style,
    pub variable: Style,
    pub structure: Style,
    pub punctuation: Style,
    pub keyword: Style,
    pub type_style: Style,
}

impl StyleManager {
    fn new() -> Self {
        Self {
            operator: Style::new().blue(),
            literal: Style::new().green(),
            variable: Style::new().yellow(),
            structure: Style::new().cyan(),
            punctuation: Style::new().magenta(),
            keyword: Style::new().blue(),
            type_style: Style::new().green(),
        }
    }
}

pub fn pretty_print(expr: &Expr) -> String {
    let mut output = String::new();
    let styles = StyleManager::new();
    print_expr(expr, "", true, &mut output, &styles);
    output
}

fn print_expr(
    expr: &Expr,
    indent: &str,
    is_last: bool,
    output: &mut String,
    styles: &StyleManager,
) {
    match expr {
        Expr::Binary {
            left, op, right, ..
        } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().operator,
                &format!("BinaryOp {op:?}"),
            );

            // Left child
            let left_indent = get_indent(indent, is_last);
            append_line(
                output,
                &left_indent,
                false,
                styles.structure.clone(),
                "Left:",
            );
            let left_child_indent = get_indent(left_indent.as_str(), false);
            print_expr(left, &left_child_indent, true, output, styles);

            // Right child
            let right_indent = get_indent(indent, is_last);
            append_line(
                output,
                &right_indent,
                true,
                styles.structure.clone(),
                "Right:",
            );
            let right_child_indent = get_indent(right_indent.as_str(), true);
            print_expr(right, &right_child_indent, true, output, styles);
        }
        Expr::Unary { op, expr, .. } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().operator,
                &format!("UnaryOp {op:?}"),
            );
            let new_indent = get_indent(indent, is_last);
            append_line(output, &new_indent, true, styles.structure.clone(), "Expr:");
            let expr_indent = get_indent(new_indent.as_str(), true);
            print_expr(expr, &expr_indent, true, output, styles);
        }
        Expr::Grouping { expr, .. } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().punctuation,
                "Grouping",
            );
            let new_indent = get_indent(indent, is_last);
            append_line(output, &new_indent, true, styles.structure.clone(), "Expr:");
            let expr_indent = get_indent(new_indent.as_str(), true);
            print_expr(expr, &expr_indent, true, output, styles);
        }
        Expr::Literal { value, .. } => {
            let val_str = match value {
                LiteralValue::Number(n) => format!("{n}"),
                LiteralValue::StringLit(s) => format!("\"{s}\""),
                LiteralValue::CharLit(c) => format!("'{c}'"),
                LiteralValue::Bool(b) => format!("{b}"),
                LiteralValue::Nullptr => "nullptr".to_string(),
            };
            append_line(
                output,
                indent,
                is_last,
                styles.clone().literal,
                &format!("Literal {val_str}"),
            );
        }
        Expr::Variable { name, .. } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().variable,
                &format!("Variable '{name}'"),
            );
        }
        Expr::Assign { target, value, .. } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().variable,
                "Assignment",
            );
            let new_indent = get_indent(indent, is_last);

            // Target
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Target:",
            );
            let target_indent = get_indent(new_indent.as_str(), false);
            print_expr(target, &target_indent, true, output, styles);

            // Value
            append_line(
                output,
                &new_indent,
                true,
                styles.structure.clone(),
                "Value:",
            );
            let value_indent = get_indent(new_indent.as_str(), true);
            print_expr(value, &value_indent, true, output, styles);
        }
        Expr::Call {
            callee, arguments, ..
        } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().punctuation,
                "Function Call",
            );
            let new_indent = get_indent(indent, is_last);

            // Callee
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Callee:",
            );
            let callee_indent = get_indent(new_indent.as_str(), false);
            print_expr(callee, &callee_indent, true, output, styles);

            // Arguments
            append_line(
                output,
                &new_indent,
                true,
                styles.structure.clone(),
                "Arguments:",
            );
            let args_indent = get_indent(new_indent.as_str(), true);

            for (i, arg) in arguments.iter().enumerate() {
                let is_last_arg = i == arguments.len() - 1;
                let arg_indent = get_indent(&args_indent, is_last); // FIX: Use is_last_arg
                append_line(
                    output,
                    &arg_indent,
                    is_last_arg,
                    styles.structure.clone(),
                    "Arg:",
                );
                let child_indent = get_indent(&arg_indent, is_last_arg); // FIX: Use is_last_arg
                print_expr(arg, &child_indent, true, output, styles);
            }
        }
        Expr::ArrayAccess { array, index, .. } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().punctuation,
                "Array Access",
            );
            let new_indent = get_indent(indent, is_last);

            // Array
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Array:",
            );
            let array_indent = get_indent(new_indent.as_str(), false);
            print_expr(array, &array_indent, true, output, styles);

            // Index
            append_line(
                output,
                &new_indent,
                true,
                styles.structure.clone(),
                "Index:",
            );
            let index_indent = get_indent(new_indent.as_str(), true);
            print_expr(index, &index_indent, true, output, styles);
        }
        Expr::ArrayLiteral { elements, .. } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().punctuation,
                "Array Literal",
            );
            let new_indent = get_indent(indent, is_last);
            append_line(
                output,
                &new_indent,
                true,
                styles.structure.clone(),
                "Elements:",
            );

            for (i, elem) in elements.iter().enumerate() {
                let is_last_elem = i == elements.len() - 1;
                let elem_indent = get_indent(&new_indent, true);
                print_expr(elem, &elem_indent, is_last_elem, output, styles);
            }
        }
    }
}

fn get_indent(indent: &str, is_last: bool) -> String {
    format!("{}{}", indent, if is_last { "    " } else { "│   " })
}

fn append_line(output: &mut String, indent: &str, is_last: bool, style: Style, text: &str) {
    let branch = if is_last { "└── " } else { "├── " };
    let styled_text = style.apply_to(text);
    output.push_str(&format!("{indent}{branch}{styled_text}\n"));
}

// Add the following functions after the print_expr function

pub fn pretty_print_stmt(stmt: &Stmt) -> String {
    let mut output = String::new();
    let styles = StyleManager::new();
    print_stmt(stmt, "", true, &mut output, &styles);
    output
}

fn print_stmt(
    stmt: &Stmt,
    indent: &str,
    is_last: bool,
    output: &mut String,
    styles: &StyleManager,
) {
    match stmt {
        Stmt::Expression { expr } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().keyword,
                "Expression",
            );
            let new_indent = get_indent(indent, is_last);
            append_line(output, &new_indent, true, styles.structure.clone(), "Expr:");
            print_expr(expr, &get_indent(&new_indent, true), true, output, styles);
        }
        Stmt::VarDeclaration {
            variables,
            type_annotation,
            is_mutable,
            initializers,
            span: _span,
        } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().keyword,
                "VarDeclaration",
            );
            let new_indent = get_indent(indent, is_last);

            // Variables
            if *is_mutable {
                append_line(
                    output,
                    &new_indent,
                    false,
                    styles.variable.clone(),
                    "Variables:",
                );
            } else {
                append_line(
                    output,
                    &new_indent,
                    false,
                    styles.variable.clone(),
                    "Constants:",
                );
            }
            let vars_indent = get_indent(&new_indent, false);
            for (i, var) in variables.iter().enumerate() {
                let is_last_var = i == variables.len() - 1;
                append_line(
                    output,
                    &vars_indent,
                    is_last_var,
                    styles.variable.clone(),
                    var,
                );
            }

            // Type
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Type:",
            );
            let type_indent = get_indent(&new_indent, false);
            let type_str = format_type(type_annotation);
            append_line(
                output,
                &type_indent,
                true,
                styles.clone().type_style,
                &type_str,
            );

            // Initializers
            append_line(
                output,
                &new_indent,
                true,
                styles.structure.clone(),
                "Initializers:",
            );
            let init_indent = get_indent(&new_indent, true);
            for (i, init) in initializers.iter().enumerate() {
                let is_last = i == initializers.len() - 1;
                print_expr(init, &init_indent, is_last, output, styles);
            }
        }
        Stmt::Function {
            name,
            parameters,
            return_type,
            body,
            span: _span,
        } => {
            append_line(output, indent, is_last, styles.clone().keyword, "Function");
            let new_indent = get_indent(indent, is_last);

            // Name
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Name:",
            );
            let name_indent = get_indent(&new_indent, false);
            append_line(output, &name_indent, true, styles.clone().variable, name);

            // Parameters
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Parameters:",
            );
            let params_indent = get_indent(&new_indent, false);
            for (i, param) in parameters.iter().enumerate() {
                let is_last_param = i == parameters.len() - 1;
                append_line(
                    output,
                    &params_indent,
                    is_last_param,
                    styles.structure.clone(),
                    &format!("Parameter '{}'", param.name),
                );
                let param_indent = get_indent(&params_indent, is_last_param);
                append_line(
                    output,
                    &param_indent,
                    true,
                    styles.type_style.clone(),
                    &format!("Type: {}", format_type(&param.type_annotation)),
                );
            }

            // Return Type
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Return Type:",
            );
            let return_indent = get_indent(&new_indent, false);
            append_line(
                output,
                &return_indent,
                true,
                styles.clone().type_style,
                &format_type(return_type),
            );

            // Body
            append_line(output, &new_indent, true, styles.structure.clone(), "Body:");
            let body_indent = get_indent(&new_indent, true);
            for (i, stmt) in body.iter().enumerate() {
                let is_last_stmt = i == body.len() - 1;
                print_stmt(stmt, &body_indent, is_last_stmt, output, styles);
            }
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            span: _span,
        } => {
            append_line(output, indent, is_last, styles.clone().keyword, "If");
            let new_indent = get_indent(indent, is_last);

            // Condition
            append_line(
                output,
                &new_indent,
                false,
                styles.structure.clone(),
                "Condition:",
            );
            let cond_indent = get_indent(&new_indent, false);
            print_expr(condition, &cond_indent, true, output, styles);

            // Then Branch
            if then_branch.is_empty() {
                append_line(
                    output,
                    &new_indent,
                    true,
                    styles.structure.clone(),
                    "Then: (empty)",
                );
                return;
            } else {
                append_line(
                    output,
                    &new_indent,
                    else_branch.is_none(),
                    styles.structure.clone(),
                    "Then:",
                );
                let then_indent = get_indent(&new_indent, else_branch.is_none());
                for (i, stmt) in then_branch.iter().enumerate() {
                    let is_last_then = i == then_branch.len() - 1;
                    print_stmt(stmt, &then_indent, is_last_then, output, styles);
                }
            }

            // Else Branch
            if let Some(else_branch) = else_branch {
                append_line(output, &new_indent, true, styles.structure.clone(), "Else:");
                let else_indent = get_indent(&new_indent, true);
                for (i, stmt) in else_branch.iter().enumerate() {
                    let is_last_else = i == else_branch.len() - 1;
                    print_stmt(stmt, &else_indent, is_last_else, output, styles);
                }
            }
        }
        Stmt::MainFunction { body, span: _span } => {
            append_line(
                output,
                indent,
                is_last,
                styles.clone().keyword,
                "MainFunction",
            );
            let new_indent = get_indent(indent, is_last);
            for (i, stmt) in body.iter().enumerate() {
                let is_last_stmt = i == body.len() - 1;
                print_stmt(stmt, &new_indent, is_last_stmt, output, styles);
            }
        }
        Stmt::Block {
            statements,
            span: _span,
        } => {
            append_line(output, indent, is_last, styles.clone().keyword, "Block");
            let new_indent = get_indent(indent, true);
            for (i, stmt) in statements.iter().enumerate() {
                let is_last_stmt = i == statements.len() - 1;
                print_stmt(stmt, &new_indent, is_last_stmt, output, styles);
            }
        }
        Stmt::Return { value, span: _span } => {
            append_line(output, indent, is_last, styles.clone().keyword, "Return");
            let new_indent = get_indent(indent, is_last);
            if let Some(expr) = value {
                append_line(
                    output,
                    &new_indent,
                    true,
                    styles.structure.clone(),
                    "Value:",
                );
                print_expr(expr, &get_indent(&new_indent, true), true, output, styles);
            }
        }
        Stmt::Break { span: _span } => {
            append_line(output, indent, is_last, styles.clone().keyword, "Break");
        }
        Stmt::Continue { span: _span } => {
            append_line(output, indent, is_last, styles.clone().keyword, "Continue");
        }
    }
}

// Helper function to format Type for display
fn format_type(ty: &Type) -> String {
    match ty {
        Type::I8 => "i8".to_string(),
        Type::I16 => "i16".to_string(),
        Type::I32 => "i32".to_string(),
        Type::I64 => "i64".to_string(),
        Type::U8 => "u8".to_string(),
        Type::U16 => "u16".to_string(),
        Type::U32 => "u32".to_string(),
        Type::U64 => "u64".to_string(),
        Type::F32 => "f32".to_string(),
        Type::F64 => "f64".to_string(),
        Type::Char => "char".to_string(),
        Type::String => "string".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Custom(type_s) => type_s.clone(),
        Type::Array(inner, _size_expr) => {
            let inner_type = format_type(inner);
            format!("[{inner_type}; <expr>]")
        }
        Type::Vector(inner) => {
            let inner_type = format_type(inner);
            format!("Vector<{inner_type}>")
        }
        Type::Void => "void".to_string(),
    }
}
