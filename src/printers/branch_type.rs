//! Branch type printer utilities for tree-like output structures.
//!
//! This module provides types and functions for printing hierarchical data
//! with proper indentation and branch symbols (├──, └──, etc.).
//!
//! # Key Types
//! - [`BranchType`]: Represents the position of a node (Last or Middle)
//! - [`BranchConfig`]: Configuration for parent/current/child branch types
//! - [`StyleManager`]: Console styling configuration for different element types
//!
//! # Usage
//! Use [`print_children`] to iterate over child elements with correct branch symbols,
//! and [`append_line`] to add formatted lines with proper indentation.

use console::Style;
use std::fmt::Write;

#[repr(u8)]
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

/// Prints a list of children with correct branch symbols and indentation.
///
/// Iterates through all children, assigning [`BranchType::Middle`] to all but the last child,
/// and [`BranchType::Last`] to the final child. For each child, calls the provided closure
/// with the appropriate branch type.
///
/// # Parameters
/// - `children`: Slice of child elements to print
/// - `indent`: Current indentation string
/// - `output`: Mutable string buffer to append formatted output
/// - `styles`: Style configuration for formatting
/// - `print_fn`: Closure invoked for each child with signature `(child, indent, branch_type, output, styles)`
///
/// # Behavior
/// Returns early without calling `print_fn` if `children` is empty.
pub fn print_children<T, F>(children: &[T], indent: &str, output: &mut String, styles: &StyleManager, mut print_fn: F)
where
    F: FnMut(&T, &str, BranchType, &mut String, &StyleManager),
{
    let len = children.len();
    if len == 0 {
        return;
    }

    for child in &children[..len - 1] {
        print_fn(child, indent, BranchType::Middle, output, styles);
    }
    print_fn(&children[len - 1], indent, BranchType::Last, output, styles);
}

// Keep existing helper functions
pub fn get_indent(indent: &str, branch_type: &BranchType) -> String {
    let continuation = branch_type.indent_continuation();
    let mut result = String::with_capacity(indent.len() + continuation.len());
    result.push_str(indent);
    result.push_str(continuation);
    result
}

pub fn append_line(output: &mut String, indent: &str, branch_type: BranchType, style: Style, text: &str) {
    let branch = branch_type.symbol();
    let styled_text = style.apply_to(text);
    write!(output, "{}{}{}\n", indent, branch, styled_text).unwrap();
}
