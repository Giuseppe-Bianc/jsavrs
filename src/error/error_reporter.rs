use crate::error::compile_error::CompileError;
use crate::location::line_tracker::LineTracker;
use crate::location::source_span::SourceSpan;
use console::style;
use std::fmt::Write;

/// Enhanced error reporter with source context display
pub struct ErrorReporter {
    line_tracker: LineTracker,
}

impl ErrorReporter {
    pub fn new(line_tracker: LineTracker) -> Self {
        Self { line_tracker }
    }

    /// Returns a formatted string containing all compile errors with source context
    pub fn report_errors(&self, errors: Vec<CompileError>) -> String {
        errors
            .into_iter()
            .map(|error| match error {
                CompileError::LexerError { message, span } => {
                    self.format_error("LEX", &message, &span)
                }
                CompileError::SyntaxError { message, span } => {
                    self.format_error("SYNTAX", &message, &span)
                }
                CompileError::TypeError { message, span } => {
                    self.format_error("TYPE", &message, &span)
                }
                CompileError::IrGeneratorError { message, span } => {
                    self.format_error("IR GEN", &message, &span)
                }
                CompileError::AsmGeneratorError(mesage) => format!(
                    "{} {}: {}\n",
                    style("ERROR:").red().bold(),
                    style("ASM GEN").red(),
                    style(mesage).yellow()
                ),
                CompileError::IoError(e) => format!(
                    "{} {}: {}\n",
                    style("ERROR:").red().bold(),
                    style("I/O").red(),
                    style(e).yellow()
                ),
            })
            .collect()
    }

    /// Formats an error with source context and visual indicators
    fn format_error(&self, category: &str, message: &str, span: &SourceSpan) -> String {
        let start_line = span.start.line;
        let start_col = span.start.column;
        let end_line = span.end.line;
        let end_col = span.end.column;

        // Get the source line where the error starts
        let source_line = self.line_tracker.get_line(start_line).unwrap_or_default();

        let mut output = String::new();

        // Header with error information
        let _ = writeln!(
            &mut output,
            "{} {}: {}\n{} {}",
            style("ERROR").red().bold(),
            style(category).red(),
            style(message).yellow(),
            style("Location:").blue(),
            style(span).cyan()
        );

        if !source_line.is_empty() {
            // Source line with line number
            let _ = writeln!(&mut output, "{start_line:4} │ {source_line}");
            let start_offset = start_col.saturating_sub(1);

            // Generate underline indicators
            let underline = if start_line == end_line {
                // Single line error
                let length = (end_col - start_col).max(1);
                format!(
                    "{space:>start$}{marker}",
                    space = "",
                    start = start_offset,
                    marker = "^".repeat(length)
                )
            } else {
                format!("{:>width$}^", "", width = start_offset)
            };

            // Underline indicator
            let _ = writeln!(&mut output, "     │ {}", style(underline).red().bold());

            // Multi-line note
            if start_line != end_line {
                let _ = writeln!(
                    &mut output,
                    "     │ {} (error spans lines {}-{})",
                    style("...").blue(),
                    start_line,
                    end_line
                );
            }
        }

        output
    }
}
