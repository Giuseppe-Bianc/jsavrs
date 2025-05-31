use console::style;
use crate::error::compile_error::CompileError;
use crate::location::line_tracker::LineTracker;
use crate::location::source_span::SourceSpan;

/// Enhanced error reporter with source context display
pub struct ErrorReporter {
    line_tracker: LineTracker,
}

impl ErrorReporter {
    pub fn new(line_tracker: LineTracker) -> Self {
        Self { line_tracker }
    }

    /// Reports all compile errors with source context
    pub fn report_errors(&self, errors: Vec<CompileError>) {
        for error in errors {
            match error {
                CompileError::LexerError { message, span } => {
                    self.print_error("LEX", &message, &span)
                }
                CompileError::SyntaxError { message, span } => {
                    self.print_error("SYNTAX", &message, &span)
                }
                CompileError::IoError(e) => {
                    eprintln!(
                        "{} {}: {}",
                        style("ERROR:").red().bold(),
                        style("I/O").red(),
                        style(e).yellow()
                    );
                }
            }
        }
    }

    /// Prints an error with source context and visual indicators
    fn print_error(&self, category: &str, message: &str, span: &SourceSpan) {
        // Extract source location information
        let start_line = span.start.line;
        let start_col = span.start.column;
        let end_line = span.end.line;
        let end_col = span.end.column;

        // Get the source line where the error starts
        let source_line = self.line_tracker.get_line(start_line).unwrap_or_default();

        // Print error header
        eprintln!(
            "{} {}: {}\n{} {}",
            style("ERROR").red().bold(),
            style(category).red(),
            style(message).yellow(),
            style("Location:").blue(),
            style(span).cyan()
        );

        // Print source context only if we have the source line
        if !source_line.is_empty() {
            // Print source line with line number
            eprintln!("{:4} │ {}", start_line, source_line);

            // Calculate underline positions
            let underline_start = if start_line == end_line {
                // Single line error
                let length = (end_col - start_col).max(1);
                " ".repeat(start_col - 1) + &"^".repeat(length)
            } else {
                // Multi-line error
                " ".repeat(start_col - 1) + "^"
            };

            // Print underline indicator
            eprintln!("     │ {}", style(underline_start).red().bold());

            // Add note for multi-line errors
            if start_line != end_line {
                eprintln!(
                    "     │ {} (error spans lines {}-{})",
                    style("...").blue(),
                    start_line,
                    end_line
                );
            }
        }
    }
}