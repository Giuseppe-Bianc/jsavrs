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
    for (i, child) in children.iter().enumerate() {
        let branch_type = if i == children.len() - 1 { BranchType::Last } else { BranchType::Middle };
        print_fn(child, indent, branch_type, output, styles);
    }
}
