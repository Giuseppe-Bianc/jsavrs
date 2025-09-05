use insta::assert_snapshot;
use jsavrs::error::compile_error::CompileError;
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
use jsavrs::location::line_tracker::LineTracker;
use jsavrs::utils::{create_span, strip_ansi_codes};
use std::io;

// Test: Errore Lexer su singola riga
#[test]
fn lexer_error_single_line() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::LexerError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
        help: None,
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn lexer_error_single_line_whit_error() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::LexerError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
        help: Some("This is a test error".to_string()),
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn type_error_single_line() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::TypeError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
        help: None,
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn type_error_single_line_whit_error() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::TypeError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
        help: Some("This is a test error".to_string()),
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);
    assert_snapshot!(stripped);
}

#[test]
fn ir_gen_error_single_line() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::IrGeneratorError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
        help: None,
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn ir_gen_error_single_line_whit_error() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::IrGeneratorError {
        message: "Invalid character '#'".to_string(),
        span: create_span("test", 1, 5, 1, 6),
        help: Some("This is a test error".to_string()),
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn asm_gen_error_single_line() {
    let source = "fn main() { let x = 42; }";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::AsmGeneratorError { message: "invalid asm".to_string() }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    let expected = "\
ERROR: ASM GEN: invalid asm
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
        help: None,
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn syntax_error_multi_line_whit_error() {
    let source = "fn main() {\n    let x = 42;\n    println!(\"hello\");\n}";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::SyntaxError {
        message: "Mismatched brackets".to_string(),
        span: create_span("test", 1, 12, 3, 5),
        help: Some("Check your brackets".to_string()),
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

#[test]
fn io_error() {
    let line_tracker = LineTracker::new("test", "".to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::IoError(io::Error::new(io::ErrorKind::NotFound, "File not found"))];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

// Test: Errori multipli (Lex + Syntax + IO)
#[test]
fn multiple_errors() {
    let source = "let x = 42;\nprint x";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![
        CompileError::IoError(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied")),
        CompileError::LexerError {
            message: "Unterminated string".to_string(),
            span: create_span("test", 2, 7, 2, 8),
            help: None,
        },
        CompileError::SyntaxError {
            message: "Expected semicolon".to_string(),
            span: create_span("test", 1, 10, 1, 11),
            help: None,
        },
    ];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);

    assert_snapshot!(stripped);
}

// Test: Gestione righe non esistenti
#[test]
fn line_out_of_bounds() {
    let source = "single line";
    let line_tracker = LineTracker::new("test", source.to_string());
    let reporter = ErrorReporter::new(line_tracker);

    let errors = vec![CompileError::LexerError {
        message: "Invalid token".to_string(),
        span: create_span("test", 5, 1, 5, 2), // Linea inesistente
        help: None,
    }];

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);
    assert_snapshot!(stripped); // Non deve mostrare codice
}

#[test]
fn report_error_from_lexer() {
    let mut lexer = Lexer::new("test", &"@");
    let reporter = ErrorReporter::new(lexer.get_line_tracker());
    let (_tokens, errors) = lexer_tokenize_with_errors(&mut lexer);

    let report = reporter.report_errors(errors);
    let stripped = strip_ansi_codes(&report);
    assert_snapshot!(stripped); // Non deve mostrare codice
}
