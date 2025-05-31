use crate::error::compile_error::CompileError;
use crate::location::line_tracker::LineTracker;
use crate::location::source_span::SourceSpan;
use console::style;

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
        let mut output = String::new();
        for error in errors {
            match error {
                CompileError::LexerError { message, span } => {
                    output.push_str(&self.format_error("LEX", &message, &span));
                }
                CompileError::SyntaxError { message, span } => {
                    output.push_str(&self.format_error("SYNTAX", &message, &span));
                }
                CompileError::IoError(e) => {
                    output.push_str(&format!(
                        "{} {}: {}",
                        style("ERROR:").red().bold(),
                        style("I/O").red(),
                        style(e).yellow()
                    ));
                }
            }
        }
        output
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

        // Header with error category and message
        output.push_str(&format!(
            "{} {}: {}\n{} {}\n",
            style("ERROR").red().bold(),
            style(category).red(),
            style(message).yellow(),
            style("Location:").blue(),
            style(span).cyan()
        ));

        if !source_line.is_empty() {
            // Source line with line number
            output.push_str(&format!("{:4} │ {}\n", start_line, source_line));
            let stat_mo = start_col - 1;

            // Generate underline indicators
            let underline = if start_line == end_line {
                // Single line error
                let length = (end_col - start_col).max(1);
                " ".repeat(stat_mo) + &"^".repeat(length)
            } else {
                // Multi-line error
                " ".repeat(stat_mo) + "^"
            };

            // Underline indicator
            output.push_str(&format!("     │ {}\n", style(underline).red().bold()));

            // Multi-line note
            if start_line != end_line {
                output.push_str(&format!(
                    "     │ {} (error spans lines {}-{})\n",
                    style("...").blue(),
                    start_line,
                    end_line
                ));
            }
        }

        output
    }
}
