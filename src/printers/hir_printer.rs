use console::Style;
use crate::mlir::hir::hirimp::{HIRExpr, HIRStmt};
use crate::mlir::hir::node_metadata::NodeMetadata;
use crate::printers::branch_type::{append_line, get_indent, print_children, BranchConfig, BranchType, StyleManager};

pub fn pretty_print_hir(expr: &HIRExpr) -> String {
    let mut output = String::new();
    let styles = StyleManager::new();
    print_expr(expr, "", BranchType::Last, &mut output, &styles);
    output
}

// Unified function to print labeled branches
fn print_branch_hir(
    label: &str, expr: &HIRExpr, parent_indent: &str, branch_config: BranchConfig, output: &mut String,
    styles: &StyleManager,
) {
    let indent = get_indent(parent_indent, &branch_config.parent_type);
    append_line(output, &indent, branch_config.current_type, styles.structure.clone(), label);
    print_expr(expr, &get_indent(&indent, &branch_config.current_type), branch_config.child_type, output, styles);
}


fn print_expr(expr: &HIRExpr, indent: &str, branch_type: BranchType, output: &mut String, styles: &StyleManager) {
    match expr {
        HIRExpr::Binary { left, op, right, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().operator, &format!("BinaryOp {op:?}"), node_metadata, styles);
            print_branch_hir("Left:", left, indent, BranchConfig::new(branch_type, BranchType::Middle, BranchType::Last), output, styles);
            print_branch_hir("Right:", right, indent, BranchConfig::new(branch_type, BranchType::Last, BranchType::Last), output, styles);
        }
        HIRExpr::Unary { op, expr, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().operator, &format!("UnaryOp {op:?}"), node_metadata, styles);
            print_branch_hir("Expr:", expr, indent, BranchConfig::new(branch_type, BranchType::Last, BranchType::Last), output, styles);
        }
        HIRExpr::Grouping { expr, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().punctuation, "Grouping", node_metadata, styles);
            print_branch_hir("Expr:", expr, indent, BranchConfig::new(branch_type, BranchType::Last, BranchType::Last), output, styles);
        }
        HIRExpr::Literal { value, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().literal, &format!("Literal {value}"), node_metadata, styles);
        }
        HIRExpr::Variable { name, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().variable, &format!("Variable '{name}'"), node_metadata, styles);
        }
        HIRExpr::Assign { target, value, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().variable, "Assignment", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            // Target
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Target:");
            print_expr(target, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Value:");
            print_expr(value, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
        }
        HIRExpr::Call { callee, arguments, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().punctuation, "Function Call", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            // Callee
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Callee:");
            print_expr(callee, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Arguments:");
            let args_indent = get_indent(&new_indent, &BranchType::Last);
            print_children(
                arguments,
                &get_indent(&args_indent, &BranchType::Last),
                output,
                styles,
                |arg, child_indent, branch_type, output, styles| {
                    append_line(output, child_indent, branch_type, styles.structure.clone(), "Arg:");
                    print_expr(arg, &get_indent(child_indent, &branch_type), BranchType::Last, output, styles);
                },
            );
        }
        HIRExpr::ArrayAccess { array, index, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().punctuation, "Array Access", node_metadata, styles);
            print_branch_hir("Array:", array, indent, BranchConfig::new(branch_type, BranchType::Middle, BranchType::Last), output, styles);
            print_branch_hir("Index:", index, indent, BranchConfig::new(branch_type, BranchType::Last, BranchType::Last), output, styles);
        }
        HIRExpr::ArrayLiteral { elements, node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().punctuation, "Array Literal", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Elements:");
            let elems_indent = get_indent(&new_indent, &BranchType::Last);
            print_children(elements, &elems_indent, output, styles, print_expr);
        }
    }
}
fn append_line_with_metadata(
    output: &mut String,
    indent: &str,
    branch_type: BranchType,
    style: Style,
    text: &str,
    node_metadata: &NodeMetadata,
    styles: &StyleManager
) {
    let branch = branch_type.symbol();
    let styled_text = style.apply_to(text);
    let metadata_string = format!(" [{}]", node_metadata);
    let metadata_text = styles.metadata.apply_to(&metadata_string);
    output.push_str(&format!("{indent}{branch}{styled_text}{metadata_text}\n"));
}

// Add the following functions after the print_expr function

/// Pretty-print a single statement AST into a styled, tree-like string.
/// Mirrors `pretty_print` for expressions.
pub fn pretty_print_stmt_hir(stmt: &HIRStmt) -> String {
    let mut output = String::new();
    let styles = StyleManager::new();
    print_stmt(stmt, "", BranchType::Last, &mut output, &styles);
    output
}

fn print_stmt(stmt: &HIRStmt, indent: &str, branch_type: BranchType, output: &mut String, styles: &StyleManager) {
    match stmt {
        HIRStmt::Expression { expr, node_metadata } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Expression", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Expr:");
            print_expr(expr, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
        }
        HIRStmt::VarDeclaration { variables, type_annotation, is_mutable, initializers, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "VarDeclaration", node_metadata, styles);
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
            append_line(
                output,
                &type_indent,
                BranchType::Last,
                styles.clone().type_style,
                &format!("{type_annotation}"),
            );
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Initializers:");
            let init_indent = get_indent(&new_indent, &BranchType::Last);
            print_children(initializers, &init_indent, output, styles, print_expr);
        }
        HIRStmt::Function { name, parameters, return_type, body, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Function", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            // Name
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Name:");
            append_line(
                &mut *output,
                &get_indent(&new_indent, &BranchType::Middle),
                BranchType::Last,
                styles.clone().variable,
                name,
            );
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
                append_line(
                    output,
                    &get_indent(&params_indent, &param_branch_type),
                    BranchType::Last,
                    styles.type_style.clone(),
                    &format!("Type: {}", param.type_annotation),
                );
            }
            // Return Type
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Return Type:");
            append_line(
                output,
                &get_indent(&new_indent, &BranchType::Middle),
                BranchType::Last,
                styles.clone().type_style,
                &format!("{return_type}"),
            );
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            print_children(body, &get_indent(&new_indent, &BranchType::Last), output, styles, print_stmt);
        }
        HIRStmt::If { condition, then_branch, else_branch, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "If", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            // Condition
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Condition:");
            print_expr(condition, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            if then_branch.is_empty() {
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Then: (empty)");
            } else {
                let then_branch_type = if else_branch.is_none() { BranchType::Last } else { BranchType::Middle };
                append_line(output, &new_indent, then_branch_type, styles.structure.clone(), "Then:");
                print_children(then_branch, &get_indent(&new_indent, &then_branch_type), output, styles, print_stmt);
            }
            // Else Branch
            if let Some(else_branch) = else_branch {
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Else:");
                print_children(else_branch, &get_indent(&new_indent, &BranchType::Last), output, styles, print_stmt);
            }
        }
        HIRStmt::MainFunction { body, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "MainFunction", node_metadata, styles);
            print_children(body, &get_indent(indent, &branch_type), output, styles, print_stmt);
        }
        HIRStmt::Block { statements, node_metadata, span: _ } => {
            if statements.is_empty() {
                append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Block: (empty)", node_metadata, styles);
            } else {
                append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Block", node_metadata, styles);
                print_children(statements, &get_indent(indent, &branch_type), output, styles, print_stmt);
            }
        }
        HIRStmt::Return { value, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Return", node_metadata, styles);
            if let Some(expr) = value {
                let new_indent = get_indent(indent, &branch_type);
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Value:");
                print_expr(expr, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
            }
        }
        HIRStmt::While { condition, body, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "While", node_metadata, styles);
            let new_indent = get_indent(indent, &branch_type);
            // Condition
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Condition:");
            print_expr(condition, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            print_children(body, &get_indent(&new_indent, &BranchType::Last), output, styles, print_stmt);
        }
        HIRStmt::For { initializer, condition, increment, body, node_metadata, span: _ } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "For", node_metadata, styles);
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
            print_children(body, &get_indent(&new_indent, &BranchType::Last), output, styles, print_stmt);
        }
        HIRStmt::Break { node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Break", node_metadata, styles);
        }
        HIRStmt::Continue { node_metadata, .. } => {
            append_line_with_metadata(output, indent, branch_type, styles.clone().keyword, "Continue", node_metadata, styles);
        }
    }
}