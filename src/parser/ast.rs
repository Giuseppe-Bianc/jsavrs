//src/parser/ast.rs
use console::Style;
use crate::location::source_span::SourceSpan;
use crate::tokens::number::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>, op: BinaryOp, right: Box<Expr>, span: SourceSpan,
    },
    Unary {
        op: UnaryOp, expr: Box<Expr>, span: SourceSpan,
    },
    Grouping {
        expr: Box<Expr>, span: SourceSpan,
    },
    Literal {
        value: LiteralValue, span: SourceSpan,
    },
    Variable {
        name: String, span: SourceSpan,
    },
    Assign {
        name: String, value: Box<Expr>, span: SourceSpan,
    },
    Call {
        callee: Box<Expr>, arguments: Vec<Expr>, span: SourceSpan,
    },
    ArrayAccess {
        array: Box<Expr>, index: Box<Expr>, span: SourceSpan,
    },
    // Additional expressions as needed
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(Number),
    StringLit(String),
    CharLit(String),
    Bool(bool),
    Nullptr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression {
        expr: Expr,
    },
    VarDeclaration {
        variables: Vec<String>, type_annotation: Type, initializers: Vec<Expr>, span: SourceSpan,
    },
    Function {
        name: String, parameters: Vec<Parameter>, return_type: Type, body: Vec<Stmt>, span: SourceSpan,
    },
    If {
        condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>>, span: SourceSpan,
    },
    Block {
        statements: Vec<Stmt>, span: SourceSpan,
    },
    Return {
        value: Option<Expr>, span: SourceSpan,
    },
    // Additional statements as needed
}

impl Expr {
    pub fn span(&self) -> &SourceSpan {
        match self {
            Expr::Binary { span, .. } => span,
            Expr::Unary { span, .. } => span,
            Expr::Grouping { span, .. } => span,
            Expr::Literal { span, .. } => span,
            Expr::Variable { span, .. } => span,
            Expr::Assign { span, .. } => span,
            Expr::Call { span, .. } => span,
            Expr::ArrayAccess { span, .. } => span,
        }
    }
    
    pub fn new_nullptr(span: SourceSpan) -> Self {
        Expr::Literal {
            value: LiteralValue::Nullptr,
            span,
        }
    }
}


impl Stmt {
    pub fn span(&self) -> &SourceSpan {
        match self {
            Stmt::Expression { expr } => expr.span(),
            Stmt::VarDeclaration { span, .. } => span,
            Stmt::Function { span, .. } => span,
            Stmt::If { span, .. } => span,
            Stmt::Block { span, .. } => span,
            Stmt::Return { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char,
    String,
    Bool,
    Array(Box<Type>, Box<Expr>),
    Vector(Box<Type>),
    Void,
}

pub fn pretty_print(expr: &Expr) -> String {
    let mut output = String::new();
    print_expr(expr, "", true, &mut output);
    output
}

fn print_expr(expr: &Expr, indent: &str, is_last: bool, output: &mut String) {
    let operator_style = Style::new().blue();
    let literal_style = Style::new().green();
    let variable_style = Style::new().yellow();
    let structure_style = Style::new().cyan();
    let punctuation_style = Style::new().magenta();

    match expr {
        Expr::Binary { left, op, right, .. } => {
            append_line(output, indent, is_last, operator_style, &format!("BinaryOp {op:?}"));

            // Left child
            let left_indent = get_indent(indent, is_last);
            append_line(output, &left_indent, false, structure_style.clone(), "Left:");
            let left_child_indent = get_indent(left_indent.as_str(), false);
            print_expr(left, &left_child_indent, true, output);

            // Right child
            let right_indent = get_indent(indent, is_last);
            append_line(output, &right_indent, true, structure_style.clone(), "Right:");
            let right_child_indent = get_indent(right_indent.as_str(), true);
            print_expr(right, &right_child_indent, true, output);
        }
        Expr::Unary { op, expr, .. } => {
            append_line(output, indent, is_last, operator_style, &format!("UnaryOp {op:?}"));
            let new_indent = get_indent(indent, is_last);
            append_line(output, &new_indent, true, structure_style.clone(), "Expr:");
            let expr_indent = get_indent(new_indent.as_str(), true);
            print_expr(expr, &expr_indent, true, output);
        }
        Expr::Grouping { expr, .. } => {
            append_line(output, indent, is_last, punctuation_style, "Grouping");
            let new_indent = get_indent(indent, is_last);
            append_line(output, &new_indent, true, structure_style.clone(), "Expr:");
            let expr_indent = get_indent(new_indent.as_str(), true);
            print_expr(expr, &expr_indent, true, output);
        }
        Expr::Literal { value, .. } => {
            let val_str = match value {
                LiteralValue::Number(n) => format!("{n}"),
                LiteralValue::StringLit(s) => format!("\"{s}\""),
                LiteralValue::CharLit(c) => format!("'{c}'"),
                LiteralValue::Bool(b) => format!("{b}"),
                LiteralValue::Nullptr => "nullptr".to_string(),
            };
            append_line(output, indent, is_last, literal_style, &format!("Literal {val_str}"));
        }
        Expr::Variable { name, .. } => {
            append_line(output, indent, is_last, variable_style, &format!("Variable '{name}'"));
        }
        Expr::Assign { name, value, .. } => {
            append_line(output, indent, is_last, variable_style, &format!("Assign to '{name}'"));
            let new_indent = get_indent(indent, is_last);
            append_line(output, &new_indent, true, structure_style.clone(), "Value:");
            let value_indent = get_indent(new_indent.as_str(), true);
            print_expr(value, &value_indent, true, output);
        }
        Expr::Call { callee, arguments, .. } => {
            append_line(output, indent, is_last, punctuation_style, "Function Call");
            let new_indent = get_indent(indent, is_last);

            // Callee
            append_line(output, &new_indent, false, structure_style.clone(), "Callee:");
            let callee_indent = get_indent(new_indent.as_str(), false);
            print_expr(callee, &callee_indent, true, output);

            // Arguments
            append_line(output, &new_indent, true, structure_style.clone(), "Arguments:");
            let args_indent = get_indent(new_indent.as_str(), true);

            for (i, arg) in arguments.iter().enumerate() {
                let is_last_arg = i == arguments.len() - 1;
                let arg_indent = get_indent(&args_indent, is_last); // FIX: Use is_last_arg
                append_line(output, &arg_indent, is_last_arg, structure_style.clone(), "Arg:");
                let child_indent = get_indent(&arg_indent, is_last_arg); // FIX: Use is_last_arg
                print_expr(arg, &child_indent, true, output);
            }
        }
        Expr::ArrayAccess { array, index, .. } => {
            append_line(output, indent, is_last, punctuation_style, "Array Access");
            let new_indent = get_indent(indent, is_last);

            // Array
            append_line(output, &new_indent, false, structure_style.clone(), "Array:");
            let array_indent = get_indent(new_indent.as_str(), false);
            print_expr(array, &array_indent, true, output);

            // Index
            append_line(output, &new_indent, true, structure_style.clone(), "Index:");
            let index_indent = get_indent(new_indent.as_str(), true);
            print_expr(index, &index_indent, true, output);
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
