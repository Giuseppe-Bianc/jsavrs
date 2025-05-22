use jsavrs::error::compile_error::CompileError;
use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
use std::sync::Arc;

#[test]
fn test_io_error_display() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error: CompileError = io_error.into();
    assert_eq!(format!("{}", error), "I/O error: File not found");
}

macro_rules! generate_display_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            let span = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($line, 1, 0),
                SourceLocation::new($line, 2, 1),
            );
            let error = CompileError::$error_type {
                message: "Unexpected token \"@\"".to_string(),
                span,
            };
            assert_eq!(
                format!("{} at {}", error.message().unwrap(), error.span().unwrap()),
                format!(
                    "Unexpected token \"@\" at test_file:line {}:column 1 - line {}:column 2",
                    $line, $line
                )
            );
        }
    };
}

generate_display_test!(test_lexer_error_display, LexerError, 1);
generate_display_test!(test_parser_error_display, SyntaxError, 2);

macro_rules! generate_message_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            let span = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($line, 1, 0),
                SourceLocation::new($line, 2, 1),
            );
            let error = CompileError::$error_type {
                message: "Unexpected token \"@\"".to_string(),
                span,
            };
            assert_eq!(error.message(), Some("Unexpected token \"@\""));
        }
    };
}

generate_message_test!(test_lexer_error_message, LexerError, 1);
generate_message_test!(test_parser_error_message, SyntaxError, 2);

macro_rules! generate_span_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            let span = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($line, 1, 0),
                SourceLocation::new($line, 2, 1),
            );
            let error = CompileError::$error_type {
                message: "Unexpected token \"@\"".to_string(),
                span,
            };
            assert_eq!(error.span().unwrap().start.line, $line);
            assert_eq!(error.span().unwrap().end.line, $line);
            assert_eq!(error.span().unwrap().start.column, 1);
            assert_eq!(error.span().unwrap().end.column, 2);
            assert_eq!(error.span().unwrap().start.absolute_pos, 0);
            assert_eq!(error.span().unwrap().end.absolute_pos, 1);
        }
    };
}

generate_span_test!(test_lexer_error_span, LexerError, 1);
generate_span_test!(test_parser_error_span, SyntaxError, 2);

macro_rules! generate_set_message_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            let span = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($line, 1, 0),
                SourceLocation::new($line, 2, 1),
            );
            let mut error = CompileError::$error_type {
                message: "Unexpected token \"@\"".to_string(),
                span,
            };
            error.set_message("New message".to_string());
            assert_eq!(error.message(), Some("New message"));
        }
    };
}

generate_set_message_test!(test_set_message, LexerError, 1);
generate_set_message_test!(test_set_message_parser, SyntaxError, 2);

macro_rules! generate_set_span_test {
    ($test_name:ident, $error_type:ident, $initial_line:expr, $new_line:expr) => {
        #[test]
        fn $test_name() {
            let span1 = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($initial_line, 1, 0),
                SourceLocation::new($initial_line, 2, 1),
            );
            let span2 = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($new_line, 1, 2),
                SourceLocation::new($new_line, 2, 3),
            );
            let mut error = CompileError::$error_type {
                message: "Unexpected token \"@\"".to_string(),
                span: span1,
            };
            error.set_span(span2);
            assert_eq!(error.span().unwrap().start.line, $new_line);
            assert_eq!(error.span().unwrap().end.line, $new_line);
            assert_eq!(error.span().unwrap().start.column, 1);
            assert_eq!(error.span().unwrap().end.column, 2);
            assert_eq!(error.span().unwrap().start.absolute_pos, 2);
            assert_eq!(error.span().unwrap().end.absolute_pos, 3);
        }
    };
}

generate_set_span_test!(test_set_span, LexerError, 1, 2);
generate_set_span_test!(test_set_span_parser, SyntaxError, 2, 3);

#[test]
fn test_set_message_not_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let mut error: CompileError = io_error.into();
    error.set_message("New message".to_string());
    assert_eq!(error.message(), None);
}

#[test]
fn test_set_span_not_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let mut error: CompileError = io_error.into();
    let span = SourceSpan::new(
        Arc::from("test_file"),
        SourceLocation::new(1, 1, 0),
        SourceLocation::new(1, 2, 1),
    );
    error.set_span(span);
    assert_eq!(error.span(), None);
}

#[test]
fn test_get_span_non_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error: CompileError = io_error.into();
    assert_eq!(error.span(), None);
}

#[test]
fn test_get_message_non_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error: CompileError = io_error.into();
    assert_eq!(error.message(), None);
}