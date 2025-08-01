use insta::assert_debug_snapshot;
use insta::assert_snapshot;
use jsavrs::error::compile_error::CompileError;
use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
use jsavrs::{make_error, make_error_lineless};
use jsavrs::utils::t_span;
use std::sync::Arc;

#[test]
fn test_io_error_display() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error: CompileError = io_error.into();
    assert_snapshot!(error);
}

#[test]
fn test_asm_generator_error_display() {
    let error = CompileError::AsmGeneratorError {
        message: "Invalid assembly code".to_string(),
    };
    assert_snapshot!(error);
}

macro_rules! generate_display_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(error, $error_type, $line);
            let display = format!("{} at {}", error.message().unwrap(), error.span().unwrap());
            assert_snapshot!(display);
        }
    };
}

generate_display_test!(test_lexer_error_display, LexerError, 1);
generate_display_test!(test_parser_error_display, SyntaxError, 2);
generate_display_test!(test_type_error_display, TypeError, 3);
generate_display_test!(test_ir_error_display, IrGeneratorError, 4);

macro_rules! generate_message_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(error, $error_type, $line);
            assert_snapshot!(error.message().unwrap());
        }
    };
}

generate_message_test!(test_lexer_error_message, LexerError, 1);
generate_message_test!(test_parser_error_message, SyntaxError, 2);
generate_message_test!(test_type_error_message, TypeError, 3);
generate_message_test!(test_ir_error_message, IrGeneratorError, 4);

#[test]
fn test_asm_generator() {
    make_error_lineless!(error, AsmGeneratorError);
    assert_snapshot!(error.message().unwrap());
}

macro_rules! generate_span_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(error, $error_type, $line);
            assert_debug_snapshot!(error.span().unwrap());
        }
    };
}

generate_span_test!(test_lexer_error_span, LexerError, 1);
generate_span_test!(test_parser_error_span, SyntaxError, 2);
generate_span_test!(test_type_error_span, TypeError, 3);
generate_span_test!(test_ir_error_span, IrGeneratorError, 4);

macro_rules! generate_set_message_test {
    ($test_name:ident, $error_type:ident, $line:expr) => {
        #[test]
        fn $test_name() {
            make_error!(mut error, $error_type, $line);
            error.set_message("New message".to_string());
            assert_snapshot!(error.message().unwrap());
        }
    };
}

#[test]
fn test_set_message_asm_generator() {
    make_error_lineless!(mut error, AsmGeneratorError);
    error.set_message("New message".to_string());
    assert_snapshot!(error.message().unwrap());
}

generate_set_message_test!(test_set_message, LexerError, 1);
generate_set_message_test!(test_set_message_parser, SyntaxError, 2);
generate_set_message_test!(test_set_message_type, TypeError, 3);
generate_set_message_test!(test_set_message_ir_generator, IrGeneratorError, 4);

macro_rules! generate_set_span_test {
    ($test_name:ident, $error_type:ident, $initial_line:expr, $new_line:expr) => {
        #[test]
        fn $test_name() {
            // Create the initial error
            make_error!(mut error, $error_type, $initial_line);

            // Create the new span
            let new_span = SourceSpan::new(
                Arc::from("test_file"),
                SourceLocation::new($new_line, 1, 2),
                SourceLocation::new($new_line, 2, 3),
            );
            error.set_span(new_span);

            assert_debug_snapshot!(error.span().unwrap());
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
    assert_debug_snapshot!(error.message());
}

// Tests for calling `set_span` on a non-lexer error
#[test]
fn test_set_span_not_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let mut error: CompileError = io_error.into();
    let span = t_span(1);
    error.set_span(span);
    assert_debug_snapshot!(error.span());
}

// Tests for retrieving span/message on a non-lexer error
#[test]
fn test_get_span_non_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error: CompileError = io_error.into();
    assert_debug_snapshot!(error.span());
}

#[test]
fn test_get_message_non_lexer_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let error: CompileError = io_error.into();
    assert_debug_snapshot!(error.message());
}
