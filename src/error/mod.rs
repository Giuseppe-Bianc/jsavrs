/// # Error Module
///
/// The error module handles error definition and reporting throughout the compilation
///process. It provides structured error types and reporting mechanisms for all
/// compiler phases.
///
/// ## Phase-specific responsibilities:
/// * Initialization: Sets up error type definitions and reporting infrastructure
/// * Runtime: Captures and categorizes errors from different compilation phases
/// * Termination: Provides comprehensive error reporting before compilation failure
pub mod compile_error;
pub mod error_reporter;
