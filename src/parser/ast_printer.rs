use crate::parser::ast::{Expr, LiteralValue, Stmt};
use console::Style;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BranchType {
    Last,
    Middle,
}

impl BranchType {
    fn symbol(&self) -> &'static str {
        match self {
            BranchType::Last => "└── ",
            BranchType::Middle => "├── ",
        }
    }

    fn indent_continuation(&self) -> &'static str {
        match self {
            BranchType::Last => "    ",
            BranchType::Middle => "│   ",
        }
    }
}

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
    print_expr(expr, "", BranchType::Last, &mut output, &styles);
    output
}

fn print_expr(expr: &Expr, indent: &str, branch_type: BranchType, output: &mut String, styles: &StyleManager) {
    match expr {
        Expr::Binary { left, op, right, .. } => {
            append_line(output, indent, branch_type, styles.clone().operator, &format!("BinaryOp {op:?}"));
            // Left child
            let left_indent = get_indent(indent, &branch_type);
            append_line(output, &left_indent, BranchType::Middle, styles.structure.clone(), "Left:");
            let left_child_indent = get_indent(left_indent.as_str(), &BranchType::Middle);
            print_expr(left, &left_child_indent, BranchType::Last, output, styles);
            // Right child
            let right_indent = get_indent(indent, &branch_type);
            append_line(output, &right_indent, BranchType::Last, styles.structure.clone(), "Right:");
            let right_child_indent = get_indent(right_indent.as_str(), &BranchType::Last);
            print_expr(right, &right_child_indent, BranchType::Last, output, styles);
        }
        Expr::Unary { op, expr, .. } => {
            append_line(output, indent, branch_type, styles.clone().operator, &format!("UnaryOp {op:?}"));
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Expr:");
            let expr_indent = get_indent(new_indent.as_str(), &BranchType::Last);
            print_expr(expr, &expr_indent, BranchType::Last, output, styles);
        }
        Expr::Grouping { expr, .. } => {
            append_line(output, indent, branch_type, styles.clone().punctuation, "Grouping");
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Expr:");
            let expr_indent = get_indent(new_indent.as_str(), &BranchType::Last);
            print_expr(expr, &expr_indent, BranchType::Last, output, styles);
        }
        Expr::Literal { value, .. } => {
            let val_str = match value {
                LiteralValue::Number(n) => format!("{n}"),
                LiteralValue::StringLit(s) => format!("\"{s}\""),
                LiteralValue::CharLit(c) => format!("'{c}'"),
                LiteralValue::Bool(b) => format!("{b}"),
                LiteralValue::Nullptr => "nullptr".to_string(),
            };
            append_line(output, indent, branch_type, styles.clone().literal, &format!("Literal {val_str}"));
        }
        Expr::Variable { name, .. } => {
            append_line(output, indent, branch_type, styles.clone().variable, &format!("Variable '{name}'"));
        }
        Expr::Assign { target, value, .. } => {
            append_line(output, indent, branch_type, styles.clone().variable, "Assignment");
            let new_indent = get_indent(indent, &branch_type);
            // Target
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Target:");
            let target_indent = get_indent(new_indent.as_str(), &BranchType::Middle);
            print_expr(target, &target_indent, BranchType::Last, output, styles);
            // Value
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Value:");
            let value_indent = get_indent(new_indent.as_str(), &BranchType::Last);
            print_expr(value, &value_indent, BranchType::Last, output, styles);
        }
        Expr::Call { callee, arguments, .. } => {
            append_line(output, indent, branch_type, styles.clone().punctuation, "Function Call");
            let new_indent = get_indent(indent, &branch_type);
            // Callee
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Callee:");
            let callee_indent = get_indent(new_indent.as_str(), &BranchType::Middle);
            print_expr(callee, &callee_indent, BranchType::Last, output, styles);
            // Arguments
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Arguments:");
            let args_indent = get_indent(new_indent.as_str(), &BranchType::Last);
            for (i, arg) in arguments.iter().enumerate() {
                let arg_branch_type = if i == arguments.len() - 1 { BranchType::Last } else { BranchType::Middle };
                let arg_indent = get_indent(&args_indent, &BranchType::Last);
                append_line(output, &arg_indent, arg_branch_type, styles.structure.clone(), "Arg:");
                let child_indent = get_indent(&arg_indent, &arg_branch_type);
                print_expr(arg, &child_indent, BranchType::Last, output, styles);
            }
        }
        Expr::ArrayAccess { array, index, .. } => {
            append_line(output, indent, branch_type, styles.clone().punctuation, "Array Access");
            let new_indent = get_indent(indent, &branch_type);
            // Array
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Array:");
            let array_indent = get_indent(new_indent.as_str(), &BranchType::Middle);
            print_expr(array, &array_indent, BranchType::Last, output, styles);
            // Index
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Index:");
            let index_indent = get_indent(new_indent.as_str(), &BranchType::Last);
            print_expr(index, &index_indent, BranchType::Last, output, styles);
        }
        Expr::ArrayLiteral { elements, .. } => {
            append_line(output, indent, branch_type, styles.clone().punctuation, "Array Literal");
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Elements:");
            for (i, elem) in elements.iter().enumerate() {
                let elem_branch_type = if i == elements.len() - 1 { BranchType::Last } else { BranchType::Middle };
                let elem_indent = get_indent(&new_indent, &BranchType::Last);
                print_expr(elem, &elem_indent, elem_branch_type, output, styles);
            }
        }
    }
}

fn get_indent(indent: &str, branch_type: &BranchType) -> String {
    format!("{}{}", indent, branch_type.indent_continuation())
}

fn append_line(output: &mut String, indent: &str, branch_type: BranchType, style: Style, text: &str) {
    let branch = branch_type.symbol();
    let styled_text = style.apply_to(text);
    output.push_str(&format!("{indent}{branch}{styled_text}\n"));
}

// Add the following functions after the print_expr function

pub fn pretty_print_stmt(stmt: &Stmt) -> String {
    let mut output = String::new();
    let styles = StyleManager::new();
    print_stmt(stmt, "", BranchType::Last, &mut output, &styles);
    output
}

fn print_stmt(stmt: &Stmt, indent: &str, branch_type: BranchType, output: &mut String, styles: &StyleManager) {
    match stmt {
        Stmt::Expression { expr } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Expression");
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Expr:");
            print_expr(expr, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
        }
        Stmt::VarDeclaration { variables, type_annotation, is_mutable, initializers, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "VarDeclaration");
            let new_indent = get_indent(indent, &branch_type);
            // Variables
            let vars_label = if *is_mutable { "Variables:" } else { "Constants:" };
            append_line(output, &new_indent, BranchType::Middle, styles.variable.clone(), vars_label);
            let vars_indent = get_indent(&new_indent, &BranchType::Middle);
            for (i, var) in variables.iter().enumerate() {
                let var_branch_type = if i == variables.len() - 1 { BranchType::Last } else { BranchType::Middle };
                append_line(output, &vars_indent, var_branch_type, styles.variable.clone(), var);
            }
            // Type
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Type:");
            let type_indent = get_indent(&new_indent, &BranchType::Middle);
            let type_str = format!("{type_annotation}");
            append_line(output, &type_indent, BranchType::Last, styles.clone().type_style, &type_str);
            // Initializers
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Initializers:");
            let init_indent = get_indent(&new_indent, &BranchType::Last);
            for (i, init) in initializers.iter().enumerate() {
                let init_branch_type = if i == initializers.len() - 1 { BranchType::Last } else { BranchType::Middle };
                print_expr(init, &init_indent, init_branch_type, output, styles);
            }
        }
        Stmt::Function { name, parameters, return_type, body, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Function");
            let new_indent = get_indent(indent, &branch_type);
            // Name
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Name:");
            let name_indent = get_indent(&new_indent, &BranchType::Middle);
            append_line(output, &name_indent, BranchType::Last, styles.clone().variable, name);
            // Parameters
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Parameters:");
            let params_indent = get_indent(&new_indent, &BranchType::Middle);
            for (i, param) in parameters.iter().enumerate() {
                let param_branch_type = if i == parameters.len() - 1 { BranchType::Last } else { BranchType::Middle };
                append_line(
                    output,
                    &params_indent,
                    param_branch_type,
                    styles.structure.clone(),
                    &format!("Parameter '{}'", param.name),
                );
                let param_indent = get_indent(&params_indent, &param_branch_type);
                append_line(
                    output,
                    &param_indent,
                    BranchType::Last,
                    styles.type_style.clone(),
                    &format!("Type: {}", &param.type_annotation),
                );
            }
            // Return Type
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Return Type:");
            let return_indent = get_indent(&new_indent, &BranchType::Middle);
            append_line(output, &return_indent, BranchType::Last, styles.clone().type_style, &format!("{return_type}"));
            // Body
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            let body_indent = get_indent(&new_indent, &BranchType::Last);
            for (i, stmt) in body.iter().enumerate() {
                let stmt_branch_type = if i == body.len() - 1 { BranchType::Last } else { BranchType::Middle };
                print_stmt(stmt, &body_indent, stmt_branch_type, output, styles);
            }
        }
        Stmt::If { condition, then_branch, else_branch, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "If");
            let new_indent = get_indent(indent, &branch_type);
            // Condition
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Condition:");
            let cond_indent = get_indent(&new_indent, &BranchType::Middle);
            print_expr(condition, &cond_indent, BranchType::Last, output, styles);
            // Then Branch
            if then_branch.is_empty() {
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Then: (empty)");
                return;
            } else {
                let then_branch_type = if else_branch.is_none() { BranchType::Last } else { BranchType::Middle };
                append_line(output, &new_indent, then_branch_type, styles.structure.clone(), "Then:");
                let then_indent = get_indent(&new_indent, &then_branch_type);
                for (i, stmt) in then_branch.iter().enumerate() {
                    let stmt_branch_type =
                        if i == then_branch.len() - 1 { BranchType::Last } else { BranchType::Middle };
                    print_stmt(stmt, &then_indent, stmt_branch_type, output, styles);
                }
            }
            // Else Branch
            if let Some(else_branch) = else_branch {
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Else:");
                let else_indent = get_indent(&new_indent, &BranchType::Last);
                for (i, stmt) in else_branch.iter().enumerate() {
                    let stmt_branch_type =
                        if i == else_branch.len() - 1 { BranchType::Last } else { BranchType::Middle };
                    print_stmt(stmt, &else_indent, stmt_branch_type, output, styles);
                }
            }
        }
        Stmt::MainFunction { body, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "MainFunction");
            let new_indent = get_indent(indent, &branch_type);
            for (i, stmt) in body.iter().enumerate() {
                let stmt_branch_type = if i == body.len() - 1 { BranchType::Last } else { BranchType::Middle };
                print_stmt(stmt, &new_indent, stmt_branch_type, output, styles);
            }
        }
        Stmt::Block { statements, span: _span } => {
            if statements.is_empty() {
                append_line(output, indent, branch_type, styles.clone().keyword, "Block: (empty)");
            } else {
                append_line(output, indent, branch_type, styles.clone().keyword, "Block");
                let new_indent = get_indent(indent, &branch_type);
                for (i, stmt) in statements.iter().enumerate() {
                    let stmt_branch_type =
                        if i == statements.len() - 1 { BranchType::Last } else { BranchType::Middle };
                    print_stmt(stmt, &new_indent, stmt_branch_type, output, styles);
                }
            }
        }
        Stmt::Return { value, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Return");
            let new_indent = get_indent(indent, &branch_type);
            if let Some(expr) = value {
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Value:");
                print_expr(expr, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
            }
        }
        Stmt::While { condition, body, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "While");
            let new_indent = get_indent(indent, &branch_type);
            // Condition
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Condition:");
            let cond_indent = get_indent(&new_indent, &BranchType::Middle);
            print_expr(condition, &cond_indent, BranchType::Last, output, styles);
            // Body
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            let body_indent = get_indent(&new_indent, &BranchType::Last);
            for (i, stmt) in body.iter().enumerate() {
                let stmt_branch_type = if i == body.len() - 1 { BranchType::Last } else { BranchType::Middle };
                print_stmt(stmt, &body_indent, stmt_branch_type, output, styles);
            }
        }
        Stmt::For { initializer, condition, increment, body, span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "For");
            let new_indent = get_indent(indent, &branch_type);
            // Initializer
            if let Some(init) = initializer {
                append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Initializer:");
                print_stmt(init, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            }

            // Condition
            if let Some(cond) = condition {
                append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Condition:");
                print_expr(cond, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            }

            // Increment
            if let Some(inc) = increment {
                append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Increment:");
                print_expr(inc, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            }

            // Body
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            let body_indent = get_indent(&new_indent, &BranchType::Last);
            for (i, stmt) in body.iter().enumerate() {
                let stmt_branch_type = if i == body.len() - 1 { BranchType::Last } else { BranchType::Middle };
                print_stmt(stmt, &body_indent, stmt_branch_type, output, styles);
            }
        }
        Stmt::Break { span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Break");
        }
        Stmt::Continue { span: _span } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Continue");
        }
    }
}
