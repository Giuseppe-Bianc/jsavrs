use console::Style;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchType {
    Last,
    Middle,
}

impl BranchType {
    pub fn symbol(&self) -> &'static str {
        match self {
            BranchType::Last => "└── ",
            BranchType::Middle => "├── ",
        }
    }
    pub fn indent_continuation(&self) -> &'static str {
        match self {
            BranchType::Last => "    ",
            BranchType::Middle => "│   ",
        }
    }
}

pub struct BranchConfig {
    pub parent_type: BranchType,
    pub current_type: BranchType,
    pub child_type: BranchType,
}

impl BranchConfig {
    pub fn new(parent_type: BranchType, current_type: BranchType, child_type: BranchType) -> Self {
        Self { parent_type, current_type, child_type }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleManager {
    pub operator: Style,
    pub literal: Style,
    pub variable: Style,
    pub structure: Style,
    pub punctuation: Style,
    pub keyword: Style,
    pub type_style: Style,
    pub metadata: Style,
}

impl StyleManager {
    pub fn new() -> Self {
        Self {
            operator: Style::new().blue(),
            literal: Style::new().green(),
            variable: Style::new().yellow(),
            structure: Style::new().cyan(),
            punctuation: Style::new().magenta(),
            keyword: Style::new().blue(),
            type_style: Style::new().green(),
            metadata: Style::new().dim().italic(),
        }
    }
}

/// Helper function to print lists of children with correct indentation
pub fn print_children<T, F>(children: &[T], indent: &str, output: &mut String, styles: &StyleManager, mut print_fn: F)
where
    F: FnMut(&T, &str, BranchType, &mut String, &StyleManager),
{
    let last_idx = children.len().saturating_sub(1);
    for (i, child) in children.iter().enumerate() {
        let branch_type = if i == last_idx { BranchType::Last } else { BranchType::Middle };
        print_fn(child, indent, branch_type, output, styles);
    }
}

// Keep existing helper functions
pub fn get_indent(indent: &str, branch_type: &BranchType) -> String {
    format!("{}{}", indent, branch_type.indent_continuation())
}

pub fn append_line(output: &mut String, indent: &str, branch_type: BranchType, style: Style, text: &str) {
    let branch = branch_type.symbol();
    let styled_text = style.apply_to(text);
    output.push_str(&format!("{indent}{branch}{styled_text}\n"));
}
