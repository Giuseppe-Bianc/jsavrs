use std::io;
use jsavrs::error::compile_error::CompileError;
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
use jsavrs::location::line_tracker::LineTracker;
use jsavrs::utils::{create_span, strip_ansi_codes};

// Test: Errore Lexer su singola riga
#[test]
fn lexer_error_single_line() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::LexerError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    let expected = "\
ERROR LEX: Invalid character '#'
Location: test:line 1:column 5 - line 1:column 6
   1 │ fn main() { let x = 42; }
     │     ^
";
    assert_eq!(stripped, expected);
}

#[test]
fn type_error_single_line() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::TypeError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    let expected = "\
ERROR TYPE: Invalid character '#'
Location: test:line 1:column 5 - line 1:column 6
   1 │ fn main() { let x = 42; }
     │     ^
";
    assert_eq!(stripped, expected);
}

#[test]
fn syntax_error_multi_line() {
    let source = "fn main() {\n    let x = 42;\n    println!(\"hello\");\n}";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::SyntaxError {
        message: "Mismatched brackets".to_string(),
        span: create_span("test", 1, 12, 3, 5),

    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    let expected = "\
ERROR SYNTAX: Mismatched brackets
Location: test:line 1:column 12 - line 3:column 5
   1 │ fn main() {
     │            ^
     │ ... (error spans lines 1-3)
";
    assert_eq!(stripped, expected);
}

#[test]
fn io_error() {
    let line_tracker = LineTracker::new("test", "".to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::IoError(
        io::Error::new(io::ErrorKind::NotFound, "File not found")
    )];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_eq!(stripped, "ERROR: I/O: File not found\n");
}

// Test: Errori multipli (Lex + Syntax + IO)
#[test]
fn multiple_errors() {
    let source = "let x = 42;\nprint x";
    let line_tracker = LineTracker::new("test",source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![
        CompileError::IoError(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied")),
        CompileError::LexerError {
            message: "Unterminated string".to_string(),
            span: create_span("test",2, 7, 2, 8),
        },
        CompileError::SyntaxError {
            message: "Expected semicolon".to_string(),
            span: create_span("test",1, 10, 1, 11),
        },
    ];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    let expected = "\
ERROR: I/O: Access denied
ERROR LEX: Unterminated string
Location: test:line 2:column 7 - line 2:column 8
   2 │ print x
     │       ^
ERROR SYNTAX: Expected semicolon
Location: test:line 1:column 10 - line 1:column 11
   1 │ let x = 42;
     │          ^
";
    assert_eq!(stripped, expected);
}

// Test: Gestione righe non esistenti
#[test]
fn line_out_of_bounds() {
    let source = "single line";
    let line_tracker = LineTracker::new("test",source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::LexerError {
        message: "Invalid token".to_string(),
        span: create_span("test",5, 1, 5, 2), // Linea inesistente
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);
    assert!(stripped.contains("Location: test:line 5:column 1 - line 5:column 2"));
    assert!(!stripped.contains("│")); // Non deve mostrare codice
}

#[test]
fn report_error_from_lexer() {
    let mut lexer = Lexer::new("test", &"@");
    let reporter = ErrorReporter::new(lexer.get_line_tracker());
    let (_tokens, errors) = lexer_tokenize_with_errors(&mut lexer);

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    let expected = "\
ERROR LEX: Invalid token: \"@\"
Location: test:line 1:column 1 - line 1:column 2
   1 │ @
     │ ^
";
    assert_eq!(stripped, expected);
}