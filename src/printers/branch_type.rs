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