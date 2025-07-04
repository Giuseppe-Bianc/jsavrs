use jsavrs::error::compile_error::CompileError;
use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
use jsavrs::make_error;
use jsavrs::utils::t_span;
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
            make_error!(error, $error_type, $line);
            let expected = format!(
                "Unexpected token \"@\" at test_file:line {}:column 1 - line {}:column 2",
                $line, $line
            );
            assert_eq!(
                format!("{} at {}", error.message().unwrap(), error.span().unwrap()),
                expected
            );
        }
    };
}

generate_display_test!(test_lexer_error_display, LexerError, 1);
generate_display_test!(test_parser_error_display, SyntaxError, 2);
generate_display_test!(test_type_error_display, TypeError, 3);
generate_display_test!(test_io_error_display, IrGeneratorError, 4);

macro_rules! generate_message_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(error, $error_type, $line);
            assert_eq!(error.message(), Some("Unexpected token \"@\""));
        }
    };
}

generate_message_test!(test_lexer_error_message, LexerError, 1);
generate_message_test!(test_parser_error_message, SyntaxError, 2);
generate_message_test!(test_type_error_message, TypeError, 3);
generate_message_test!(test_io_error_message, IrGeneratorError, 4);

macro_rules! generate_span_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(error, $error_type, $line);
            let span = error.span().unwrap();
            assert_eq!(span.start.line, $line);
            assert_eq!(span.start.column, 1);
            assert_eq!(span.start.absolute_pos, 0);
            assert_eq!(span.end.line, $line);
            assert_eq!(span.end.column, 2);
            assert_eq!(span.end.absolute_pos, 1);
        }
    };
}

generate_span_test!(test_lexer_error_span, LexerError, 1);
generate_span_test!(test_parser_error_span, SyntaxError, 2);
generate_span_test!(test_type_error_span, TypeError, 3);
generate_span_test!(test_io_error_span, IrGeneratorError, 4);

macro_rules! generate_set_message_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(mut error, $error_type, $line);
            error.set_message("New message".to_string());
            assert_eq!(error.message(), Some("New message"));
        }
    };
}

generate_set_message_test!(test_set_message, LexerError, 1);
generate_set_message_test!(test_set_message_parser, SyntaxError, 2);
generate_set_message_test!(test_set_message_type, TypeError, 3);
generate_set_message_test!(test_set_message_ir_generator, IrGeneratorError, 4);

macro_rules! generate_set_span_test {
    ($test_name:ident, $error_type:ident, $initial_line:expr, $new_line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(mut error, $error_type, $initial_line);

            let new_span = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($new_line, 1, 2),
                SourceLocation::new($new_line, 2, 3),
            );
            error.set_span(new_span);

            let span = error.span().unwrap();
            assert_eq!(span.start.line, $new_line);
            assert_eq!(span.start.column, 1);
            assert_eq!(span.start.absolute_pos, 2);
            assert_eq!(span.end.line, $new_line);
            assert_eq!(span.end.column, 2);
            assert_eq!(span.end.absolute_pos, 3);
        }
    };
}

generate_set_span_test!(test_set_span, LexerError, 1, 2);
generate_set_span_test!(test_set_span_parser, SyntaxError, 2, 3);
generate_set_span_test!(test_set_span_type, TypeError, 3, 4);
generate_set_span_test!(test_set_span_ir_generator, IrGeneratorError, 4, 5);

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
    let span = t_span(1);
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
