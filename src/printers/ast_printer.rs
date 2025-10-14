use crate::parser::ast::{Expr, Stmt};
use crate::printers::branch_type::{BranchConfig, BranchType, StyleManager, append_line, get_indent, print_children};

/// Pretty-print an expression AST into a styled, tree-like string.
/// Optimized with capacity preallocation.
pub fn pretty_print(expr: &Expr) -> String {
    let node_count = count_expr_nodes(expr);
    // Estimate ~45 chars per node (branch chars + label + styling)
    let mut output = String::with_capacity(node_count * 45);
    let styles = StyleManager::new();
    print_expr(expr, "", BranchType::Last, &mut output, &styles);
    output
}

/// Count total nodes in expression tree for capacity estimation
fn count_expr_nodes(expr: &Expr) -> usize {
    1 + match expr {
        Expr::Binary { left, right, .. } => {
            count_expr_nodes(left) + count_expr_nodes(right)
        }
        Expr::Unary { expr, .. } | Expr::Grouping { expr, .. } => {
            count_expr_nodes(expr)
        }
        Expr::Assign { target, value, .. } => {
            count_expr_nodes(target) + count_expr_nodes(value)
        }
        Expr::Call { callee, arguments, .. } => {
            count_expr_nodes(callee) + arguments.iter().map(count_expr_nodes).sum::<usize>()
        }
        Expr::ArrayAccess { array, index, .. } => {
            count_expr_nodes(array) + count_expr_nodes(index)
        }
        Expr::ArrayLiteral { elements, .. } => {
            elements.iter().map(count_expr_nodes).sum::<usize>()
        }
        Expr::Literal { .. } | Expr::Variable { .. } => 0,
    }
}

/// Unified function to print labeled branches
fn print_branch(
    label: &str, expr: &Expr, parent_indent: &str, branch_config: BranchConfig, output: &mut String,
    styles: &StyleManager,
) {
    let indent = get_indent(parent_indent, &branch_config.parent_type);
    append_line(output, &indent, branch_config.current_type, styles.structure.clone(), label);
    print_expr(expr, &get_indent(&indent, &branch_config.current_type), branch_config.child_type, output, styles);
}

/// Prints an expression with the given indentation and branch type.
fn print_expr(expr: &Expr, indent: &str, branch_type: BranchType, output: &mut String, styles: &StyleManager) {
    match expr {
        Expr::Binary { left, op, right, .. } => {
            append_line(output, indent, branch_type, styles.operator.clone(), &format!("BinaryOp {op:?}"));
            print_branch(
                "Left:",
                left,
                indent,
                BranchConfig::new(branch_type, BranchType::Middle, BranchType::Last),
                output,
                styles,
            );
            print_branch(
                "Right:",
                right,
                indent,
                BranchConfig::new(branch_type, BranchType::Last, BranchType::Last),
                output,
                styles,
            );
        }
        Expr::Unary { op, expr, .. } => {
            append_line(output, indent, branch_type, styles.operator.clone(), &format!("UnaryOp {op:?}"));
            print_branch(
                "Expr:",
                expr,
                indent,
                BranchConfig::new(branch_type, BranchType::Last, BranchType::Last),
                output,
                styles,
            );
        }
        Expr::Grouping { expr, .. } => {
            append_line(output, indent, branch_type, styles.clone().punctuation, "Grouping");
            print_branch(
                "Expr:",
                expr,
                indent,
                BranchConfig::new(branch_type, BranchType::Last, BranchType::Last),
                output,
                styles,
            );
        }
        Expr::Literal { value, .. } => {
            append_line(output, indent, branch_type, styles.literal.clone(), &format!("Literal {value}"));
        }
        Expr::Variable { name, .. } => {
            append_line(output, indent, branch_type, styles.variable.clone(), &format!("Variable '{name}'"));
        }
        Expr::Assign { target, value, .. } => {
            append_line(output, indent, branch_type, styles.clone().variable, "Assignment");
            let new_indent = get_indent(indent, &branch_type);
            // Target
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Target:");
            print_expr(target, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Value:");
            print_expr(value, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
        }
        Expr::Call { callee, arguments, .. } => {
            append_line(output, indent, branch_type, styles.punctuation.clone(), "Function Call");
            let new_indent = get_indent(indent, &branch_type);
            // Callee
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Callee:");
            print_expr(callee, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Arguments:");
            let args_indent = get_indent(&new_indent, &BranchType::Last);
            print_children(
                arguments,
                &args_indent,
                output,
                styles,
                |arg, child_indent, branch_type, output, styles| {
                    append_line(output, child_indent, branch_type, styles.structure.clone(), "Arg:");
                    print_expr(arg, &get_indent(child_indent, &branch_type), BranchType::Last, output, styles);
                },
            );
        }
        Expr::ArrayAccess { array, index, .. } => {
            append_line(output, indent, branch_type, styles.punctuation.clone(), "Array Access");
            print_branch(
                "Array:",
                array,
                indent,
                BranchConfig::new(branch_type, BranchType::Middle, BranchType::Last),
                output,
                styles,
            );
            print_branch(
                "Index:",
                index,
                indent,
                BranchConfig::new(branch_type, BranchType::Last, BranchType::Last),
                output,
                styles,
            );
        }
        Expr::ArrayLiteral { elements, .. } => {
            append_line(output, indent, branch_type, styles.punctuation.clone(), "Array Literal");
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Elements:");
            print_children(elements, &get_indent(&new_indent, &BranchType::Last), output, styles, print_expr);
        }
    }
}

/// Pretty-print a single statement AST into a styled, tree-like string.
/// Mirrors `pretty_print` for expressions.
pub fn pretty_print_stmt(stmt: &Stmt) -> String {
    let node_count = count_stmt_nodes(stmt);
    // Statements typically have longer labels, estimate ~50 chars per node
    let mut output = String::with_capacity(node_count * 50);
    let styles = StyleManager::new();
    print_stmt(stmt, "", BranchType::Last, &mut output, &styles);
    output
}

/// Count total nodes in statement tree for capacity estimation
fn count_stmt_nodes(stmt: &Stmt) -> usize {
    1 + match stmt {
        Stmt::Expression { expr } => count_expr_nodes(expr),
        Stmt::VarDeclaration { variables, initializers, .. } => {
            variables.len() + initializers.iter().map(count_expr_nodes).sum::<usize>()
        }
        Stmt::Function { parameters, body, .. } => {
            parameters.len() + body.iter().map(count_stmt_nodes).sum::<usize>()
        }
        Stmt::If { condition, then_branch, else_branch, .. } => {
            count_expr_nodes(condition)
                + then_branch.iter().map(count_stmt_nodes).sum::<usize>()
                + else_branch.as_ref().map_or(0, |branch| {
                    branch.iter().map(count_stmt_nodes).sum::<usize>()
                })
        }
        Stmt::MainFunction { body, .. } | Stmt::Block { statements: body, .. } => {
            body.iter().map(count_stmt_nodes).sum::<usize>()
        }
        Stmt::Return { value, .. } => {
            value.as_ref().map_or(0, count_expr_nodes)
        }
        Stmt::While { condition, body, .. } => {
            count_expr_nodes(condition) + body.iter().map(count_stmt_nodes).sum::<usize>()
        }
        Stmt::For { initializer, condition, increment, body, .. } => {
            initializer.as_ref().map_or(0, |s| count_stmt_nodes(s))
                + condition.as_ref().map_or(0, count_expr_nodes)
                + increment.as_ref().map_or(0, count_expr_nodes)
                + body.iter().map(count_stmt_nodes).sum::<usize>()
        }
        Stmt::Break { .. } | Stmt::Continue { .. } => 0,
    }
}

fn print_stmt(stmt: &Stmt, indent: &str, branch_type: BranchType, output: &mut String, styles: &StyleManager) {
    match stmt {
        Stmt::Expression { expr } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Expression");
            let new_indent = get_indent(indent, &branch_type);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Expr:");
            print_expr(expr, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
        }
        Stmt::VarDeclaration { variables, type_annotation, is_mutable, initializers, span: _ } => {
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
            append_line(
                output,
                &type_indent,
                BranchType::Last,
                styles.type_style.clone(),
                &format!("{type_annotation}"),
            );
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Initializers:");
            let init_indent = get_indent(&new_indent, &BranchType::Last);
            print_children(initializers, &init_indent, output, styles, print_expr);
        }
        Stmt::Function { name, parameters, return_type, body, span: _ } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "Function");
            let new_indent = get_indent(indent, &branch_type);

            // Name
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Name:");
            append_line(
                &mut *output,
                &get_indent(&new_indent, &BranchType::Middle),
                BranchType::Last,
                styles.variable.clone(),
                name,
            );

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
                styles.type_style.clone(),
                &format!("{return_type}"),
            );

            // Body
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            print_children(body, &get_indent(&new_indent, &BranchType::Last), output, styles, print_stmt);
        }
        Stmt::If { condition, then_branch, else_branch, span: _ } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "If");
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
        Stmt::MainFunction { body, span: _ } => {
            append_line(output, indent, branch_type, styles.keyword.clone(), "MainFunction");
            print_children(body, &get_indent(indent, &branch_type), output, styles, print_stmt);
        }
        Stmt::Block { statements, span: _ } => {
            if statements.is_empty() {
                append_line(output, indent, branch_type, styles.keyword.clone(), "Block: (empty)");
            } else {
                append_line(output, indent, branch_type, styles.keyword.clone(), "Block");
                print_children(statements, &get_indent(indent, &branch_type), output, styles, print_stmt);
            }
        }
        Stmt::Return { value, span: _ } => {
            append_line(output, indent, branch_type, styles.keyword.clone(), "Return");
            if let Some(expr) = value {
                let new_indent = get_indent(indent, &branch_type);
                append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Value:");
                print_expr(expr, &get_indent(&new_indent, &BranchType::Last), BranchType::Last, output, styles);
            }
        }
        Stmt::While { condition, body, span: _ } => {
            append_line(output, indent, branch_type, styles.clone().keyword, "While");
            let new_indent = get_indent(indent, &branch_type);
            // Condition
            append_line(output, &new_indent, BranchType::Middle, styles.structure.clone(), "Condition:");
            print_expr(condition, &get_indent(&new_indent, &BranchType::Middle), BranchType::Last, output, styles);
            append_line(output, &new_indent, BranchType::Last, styles.structure.clone(), "Body:");
            print_children(body, &get_indent(&new_indent, &BranchType::Last), output, styles, print_stmt);
        }
        Stmt::For { initializer, condition, increment, body, span: _ } => {
            append_line(output, indent, branch_type, styles.keyword.clone(), "For");
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
        Stmt::Break { .. } => append_line(output, indent, branch_type, styles.keyword.clone(), "Break"),
        Stmt::Continue { .. } => append_line(output, indent, branch_type, styles.keyword.clone(), "Continue"),
    }
}