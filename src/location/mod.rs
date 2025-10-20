/// # Location Module
///
/// The location module handles source code position tracking for error reporting.
/// It provides utilities to track line numbers, columns, and source spans throughout
/// the compilation process.
///
/// ## Phase-specific responsibilities:
/// * Initialization: Sets up position tracking for source files
/// * Runtime: Maintains and updates location information as code is processed
/// * Termination: Provides final location information for error reporting
pub mod line_tracker;
pub mod source_location;
pub mod source_span;
