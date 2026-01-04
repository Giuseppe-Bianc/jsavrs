use crate::error::compile_error::CompileError;
use crate::error::error_code::ErrorCode;
use crate::location::line_tracker::LineTracker;
use crate::location::source_span::SourceSpan;
use console::style;
use std::fmt::Write;

/// Enhanced error reporter with source context display
pub struct ErrorReporter {
    line_tracker: LineTracker,
}

fn format_simple_error(error_type: &str, message: impl std::fmt::Display, code: Option<ErrorCode>) -> String {
    format!(
        "{}{}{}: {}\n",
        style("ERROR").red().bold(),
        code.map(|c| format!(" [{}] ", style(c.code()).red().bold())).unwrap_or_else(|| ": ".to_string()),
        style(error_type).red(),
        style(message).yellow()
    )
}

impl ErrorReporter {
    #[must_use]
    pub const fn new(line_tracker: LineTracker) -> Self {
        Self { line_tracker }
    }

    /// Returns a formatted string containing all compile errors with source context
    #[must_use]
    pub fn report_errors(&self, errors: Vec<CompileError>) -> String {
        let mut output = String::with_capacity(errors.len() * 500);
        for error in errors {
            let formatted = match error {
                CompileError::LexerError { message, span, help, code } => {
                    self.format_error("LEX", &message, &span, help.as_deref(), code)
                }
                CompileError::SyntaxError { message, span, help, code } => {
                    self.format_error("SYNTAX", &message, &span, help.as_deref(), code)
                }
                CompileError::TypeError { message, span, help, code } => {
                    self.format_error("TYPE", &message, &span, help.as_deref(), code)
                }
                CompileError::IrGeneratorError { message, span, help, code } => {
                    self.format_error("IR GEN", &message, &span, help.as_deref(), code)
                }
                CompileError::AsmGeneratorError { message, code } => format_simple_error("ASM GEN", &message, code),
                CompileError::IoError(e) => format_simple_error("I/O", &e, None),
            };
            output.push_str(&formatted);
        }
        output
    }

    /// Formats an error with source context and visual indicators
    fn format_error(
        &self, category: &str, message: &str, span: &SourceSpan, help: Option<&str>, code: Option<ErrorCode>,
    ) -> String {
        let start_line = span.start.line;
        let start_col = span.start.column;
        let end_line = span.end.line;
        let end_col = span.end.column;

        // Get the source line where the error starts
        let source_line = self.line_tracker.get_line(start_line).unwrap_or_default();

        let estimated_capacity =
            100 + message.len() + category.len() + source_line.len() + help.map_or(0, |h| h.len() + 20) + 50;
        let mut output = String::with_capacity(estimated_capacity);

        // Header with error information
        let _ = writeln!(
            &mut output,
            "{}{}{}: {}\n{} {}",
            style("ERROR").red().bold(),
            code.map(|c| format!(" [{}] ", style(c.code()).red().bold())).unwrap_or_else(|| " ".to_string()),
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
                format!("{:>width$}{}", "", "^".repeat(length), width = start_offset)
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

        if let Some(help) = help {
            let _ = writeln!(&mut output, "{} {}", style("help:").blue().bold(), style(help).green());
        }

        output
    }
}
