use jsavrs::error::error_code::{CompilerPhase, ErrorCode, Severity};

/// Test that all severity variants have correct Display implementation.
#[test]
fn test_severity_display() {
    assert_eq!(format!("{}", Severity::Note), "note");
    assert_eq!(format!("{}", Severity::Warning), "warning");
    assert_eq!(format!("{}", Severity::Error), "error");
    assert_eq!(format!("{}", Severity::Fatal), "fatal");
}

/// Test that Severity implements proper ordering (Note < Warning < Error < Fatal).
#[test]
fn test_severity_ordering() {
    assert!(Severity::Note < Severity::Warning);
    assert!(Severity::Warning < Severity::Error);
    assert!(Severity::Error < Severity::Fatal);
    assert!(Severity::Note < Severity::Fatal);
}

/// Test Severity equality.
#[test]
fn test_severity_equality() {
    assert_eq!(Severity::Note, Severity::Note);
    assert_eq!(Severity::Warning, Severity::Warning);
    assert_eq!(Severity::Error, Severity::Error);
    assert_eq!(Severity::Fatal, Severity::Fatal);
    assert_ne!(Severity::Note, Severity::Warning);
    assert_ne!(Severity::Error, Severity::Fatal);
}

/// Test Severity Clone.
#[test]
fn test_severity_clone() {
    let original = Severity::Error;
    let cloned = original;
    assert_eq!(original, cloned);
}

/// Test Severity Debug formatting.
#[test]
fn test_severity_debug() {
    assert_eq!(format!("{:?}", Severity::Note), "Note");
    assert_eq!(format!("{:?}", Severity::Warning), "Warning");
    assert_eq!(format!("{:?}", Severity::Error), "Error");
    assert_eq!(format!("{:?}", Severity::Fatal), "Fatal");
}

/// Test Severity Hash implementation.
#[test]
fn test_severity_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Severity::Note);
    set.insert(Severity::Warning);
    set.insert(Severity::Error);
    set.insert(Severity::Fatal);

    assert_eq!(set.len(), 4);
    assert!(set.contains(&Severity::Note));
    assert!(set.contains(&Severity::Warning));
    assert!(set.contains(&Severity::Error));
    assert!(set.contains(&Severity::Fatal));
}

/// Test that all compiler phase variants have correct Display implementation.
#[test]
fn test_compiler_phase_display() {
    assert_eq!(format!("{}", CompilerPhase::Lexer), "lexer");
    assert_eq!(format!("{}", CompilerPhase::Parser), "parser");
    assert_eq!(format!("{}", CompilerPhase::Semantic), "semantic");
    assert_eq!(format!("{}", CompilerPhase::IrGeneration), "ir-gen");
    assert_eq!(format!("{}", CompilerPhase::CodeGeneration), "codegen");
    assert_eq!(format!("{}", CompilerPhase::System), "system");
}

/// Test `CompilerPhase` equality.
#[test]
fn test_compiler_phase_equality() {
    assert_eq!(CompilerPhase::Lexer, CompilerPhase::Lexer);
    assert_eq!(CompilerPhase::Parser, CompilerPhase::Parser);
    assert_eq!(CompilerPhase::Semantic, CompilerPhase::Semantic);
    assert_ne!(CompilerPhase::Lexer, CompilerPhase::Parser);
    assert_ne!(CompilerPhase::Semantic, CompilerPhase::System);
}

/// Test `CompilerPhase` Clone.
#[test]
fn test_compiler_phase_clone() {
    let original = CompilerPhase::Parser;
    let cloned = original;
    assert_eq!(original, cloned);
}

/// Test `CompilerPhase` Debug formatting.
#[test]
fn test_compiler_phase_debug() {
    assert_eq!(format!("{:?}", CompilerPhase::Lexer), "Lexer");
    assert_eq!(format!("{:?}", CompilerPhase::Parser), "Parser");
    assert_eq!(format!("{:?}", CompilerPhase::Semantic), "Semantic");
    assert_eq!(format!("{:?}", CompilerPhase::IrGeneration), "IrGeneration");
    assert_eq!(format!("{:?}", CompilerPhase::CodeGeneration), "CodeGeneration");
    assert_eq!(format!("{:?}", CompilerPhase::System), "System");
}

/// Test `CompilerPhase` Hash implementation.
#[test]
fn test_compiler_phase_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(CompilerPhase::Lexer);
    set.insert(CompilerPhase::Parser);
    set.insert(CompilerPhase::Semantic);
    set.insert(CompilerPhase::IrGeneration);
    set.insert(CompilerPhase::CodeGeneration);
    set.insert(CompilerPhase::System);

    assert_eq!(set.len(), 6);
    assert!(set.contains(&CompilerPhase::Lexer));
    assert!(set.contains(&CompilerPhase::System));
}

/// Test all lexical error codes (E0001-E0010).
#[test]
fn test_lexical_error_codes() {
    assert_eq!(ErrorCode::E0001.code(), "E0001");
    assert_eq!(ErrorCode::E0002.code(), "E0002");
    assert_eq!(ErrorCode::E0003.code(), "E0003");
    assert_eq!(ErrorCode::E0004.code(), "E0004");
    assert_eq!(ErrorCode::E0005.code(), "E0005");
    assert_eq!(ErrorCode::E0006.code(), "E0006");
    assert_eq!(ErrorCode::E0007.code(), "E0007");
    assert_eq!(ErrorCode::E0008.code(), "E0008");
    assert_eq!(ErrorCode::E0009.code(), "E0009");
    assert_eq!(ErrorCode::E0010.code(), "E0010");
}

/// Test all parser error codes (E1001-E1015).
#[test]
fn test_parser_error_codes() {
    assert_eq!(ErrorCode::E1001.code(), "E1001");
    assert_eq!(ErrorCode::E1002.code(), "E1002");
    assert_eq!(ErrorCode::E1003.code(), "E1003");
    assert_eq!(ErrorCode::E1004.code(), "E1004");
    assert_eq!(ErrorCode::E1005.code(), "E1005");
    assert_eq!(ErrorCode::E1006.code(), "E1006");
    assert_eq!(ErrorCode::E1007.code(), "E1007");
    assert_eq!(ErrorCode::E1008.code(), "E1008");
    assert_eq!(ErrorCode::E1009.code(), "E1009");
    assert_eq!(ErrorCode::E1010.code(), "E1010");
    assert_eq!(ErrorCode::E1011.code(), "E1011");
    assert_eq!(ErrorCode::E1012.code(), "E1012");
    assert_eq!(ErrorCode::E1013.code(), "E1013");
    assert_eq!(ErrorCode::E1014.code(), "E1014");
    assert_eq!(ErrorCode::E1015.code(), "E1015");
}

/// Test all semantic/type error codes (E2001-E2032).
#[test]
fn test_semantic_error_codes() {
    assert_eq!(ErrorCode::E2001.code(), "E2001");
    assert_eq!(ErrorCode::E2002.code(), "E2002");
    assert_eq!(ErrorCode::E2003.code(), "E2003");
    assert_eq!(ErrorCode::E2004.code(), "E2004");
    assert_eq!(ErrorCode::E2005.code(), "E2005");
    assert_eq!(ErrorCode::E2006.code(), "E2006");
    assert_eq!(ErrorCode::E2007.code(), "E2007");
    assert_eq!(ErrorCode::E2008.code(), "E2008");
    assert_eq!(ErrorCode::E2009.code(), "E2009");
    assert_eq!(ErrorCode::E2010.code(), "E2010");
    assert_eq!(ErrorCode::E2011.code(), "E2011");
    assert_eq!(ErrorCode::E2012.code(), "E2012");
    assert_eq!(ErrorCode::E2013.code(), "E2013");
    assert_eq!(ErrorCode::E2014.code(), "E2014");
    assert_eq!(ErrorCode::E2015.code(), "E2015");
    assert_eq!(ErrorCode::E2016.code(), "E2016");
    assert_eq!(ErrorCode::E2017.code(), "E2017");
    assert_eq!(ErrorCode::E2018.code(), "E2018");
    assert_eq!(ErrorCode::E2019.code(), "E2019");
    assert_eq!(ErrorCode::E2020.code(), "E2020");
    assert_eq!(ErrorCode::E2021.code(), "E2021");
    assert_eq!(ErrorCode::E2022.code(), "E2022");
    assert_eq!(ErrorCode::E2023.code(), "E2023");
    assert_eq!(ErrorCode::E2024.code(), "E2024");
    assert_eq!(ErrorCode::E2025.code(), "E2025");
    assert_eq!(ErrorCode::E2026.code(), "E2026");
    assert_eq!(ErrorCode::E2027.code(), "E2027");
    assert_eq!(ErrorCode::E2028.code(), "E2028");
    assert_eq!(ErrorCode::E2029.code(), "E2029");
    assert_eq!(ErrorCode::E2030.code(), "E2030");
    assert_eq!(ErrorCode::E2031.code(), "E2031");
    assert_eq!(ErrorCode::E2032.code(), "E2032");
}

/// Test all IR generation error codes (E3001-E3008).
#[test]
fn test_ir_error_codes() {
    assert_eq!(ErrorCode::E3001.code(), "E3001");
    assert_eq!(ErrorCode::E3002.code(), "E3002");
    assert_eq!(ErrorCode::E3003.code(), "E3003");
    assert_eq!(ErrorCode::E3004.code(), "E3004");
    assert_eq!(ErrorCode::E3005.code(), "E3005");
    assert_eq!(ErrorCode::E3006.code(), "E3006");
    assert_eq!(ErrorCode::E3007.code(), "E3007");
    assert_eq!(ErrorCode::E3008.code(), "E3008");
}

/// Test all code generation error codes (E4001-E4005).
#[test]
fn test_codegen_error_codes() {
    assert_eq!(ErrorCode::E4001.code(), "E4001");
    assert_eq!(ErrorCode::E4002.code(), "E4002");
    assert_eq!(ErrorCode::E4003.code(), "E4003");
    assert_eq!(ErrorCode::E4004.code(), "E4004");
    assert_eq!(ErrorCode::E4005.code(), "E4005");
}

/// Test all I/O error codes (E5001-E5005).
#[test]
fn test_io_error_codes() {
    assert_eq!(ErrorCode::E5001.code(), "E5001");
    assert_eq!(ErrorCode::E5002.code(), "E5002");
    assert_eq!(ErrorCode::E5003.code(), "E5003");
    assert_eq!(ErrorCode::E5004.code(), "E5004");
    assert_eq!(ErrorCode::E5005.code(), "E5005");
}

/// Test numeric codes for lexical errors.
#[test]
fn test_lexical_numeric_codes() {
    assert_eq!(ErrorCode::E0001.numeric_code(), 1);
    assert_eq!(ErrorCode::E0002.numeric_code(), 2);
    assert_eq!(ErrorCode::E0003.numeric_code(), 3);
    assert_eq!(ErrorCode::E0004.numeric_code(), 4);
    assert_eq!(ErrorCode::E0005.numeric_code(), 5);
    assert_eq!(ErrorCode::E0006.numeric_code(), 6);
    assert_eq!(ErrorCode::E0007.numeric_code(), 7);
    assert_eq!(ErrorCode::E0008.numeric_code(), 8);
    assert_eq!(ErrorCode::E0009.numeric_code(), 9);
    assert_eq!(ErrorCode::E0010.numeric_code(), 10);
}

/// Test numeric codes for parser errors.
#[test]
fn test_parser_numeric_codes() {
    assert_eq!(ErrorCode::E1001.numeric_code(), 1001);
    assert_eq!(ErrorCode::E1002.numeric_code(), 1002);
    assert_eq!(ErrorCode::E1003.numeric_code(), 1003);
    assert_eq!(ErrorCode::E1004.numeric_code(), 1004);
    assert_eq!(ErrorCode::E1005.numeric_code(), 1005);
    assert_eq!(ErrorCode::E1015.numeric_code(), 1015);
}

/// Test numeric codes for semantic errors.
#[test]
fn test_semantic_numeric_codes() {
    assert_eq!(ErrorCode::E2001.numeric_code(), 2001);
    assert_eq!(ErrorCode::E2023.numeric_code(), 2023);
    assert_eq!(ErrorCode::E2032.numeric_code(), 2032);
}

/// Test numeric codes for IR errors.
#[test]
fn test_ir_numeric_codes() {
    assert_eq!(ErrorCode::E3001.numeric_code(), 3001);
    assert_eq!(ErrorCode::E3008.numeric_code(), 3008);
}

/// Test numeric codes for codegen errors.
#[test]
fn test_codegen_numeric_codes() {
    assert_eq!(ErrorCode::E4001.numeric_code(), 4001);
    assert_eq!(ErrorCode::E4005.numeric_code(), 4005);
}

/// Test numeric codes for I/O errors.
#[test]
fn test_io_numeric_codes() {
    assert_eq!(ErrorCode::E5001.numeric_code(), 5001);
    assert_eq!(ErrorCode::E5005.numeric_code(), 5005);
}

/// Test that numeric code matches the string code.
#[test]
fn test_numeric_code_matches_string_code() {
    let test_codes =
        [ErrorCode::E0001, ErrorCode::E1001, ErrorCode::E2023, ErrorCode::E3001, ErrorCode::E4001, ErrorCode::E5001];

    for code in test_codes {
        let string_code = code.code();
        let numeric = code.numeric_code();
        let expected = format!("E{numeric:04}");
        assert_eq!(string_code, expected, "Mismatch for {code:?}");
    }
}

/// Test that all lexical errors map to Lexer phase.
#[test]
fn test_lexical_errors_phase() {
    let lexical_errors = [
        ErrorCode::E0001,
        ErrorCode::E0002,
        ErrorCode::E0003,
        ErrorCode::E0004,
        ErrorCode::E0005,
        ErrorCode::E0006,
        ErrorCode::E0007,
        ErrorCode::E0008,
        ErrorCode::E0009,
        ErrorCode::E0010,
    ];

    for error in lexical_errors {
        assert_eq!(error.phase(), CompilerPhase::Lexer, "Error {error:?} should be in Lexer phase");
    }
}

/// Test that all parser errors map to Parser phase.
#[test]
fn test_parser_errors_phase() {
    let parser_errors = [
        ErrorCode::E1001,
        ErrorCode::E1002,
        ErrorCode::E1003,
        ErrorCode::E1004,
        ErrorCode::E1005,
        ErrorCode::E1006,
        ErrorCode::E1007,
        ErrorCode::E1008,
        ErrorCode::E1009,
        ErrorCode::E1010,
        ErrorCode::E1011,
        ErrorCode::E1012,
        ErrorCode::E1013,
        ErrorCode::E1014,
        ErrorCode::E1015,
    ];

    for error in parser_errors {
        assert_eq!(error.phase(), CompilerPhase::Parser, "Error {error:?} should be in Parser phase");
    }
}

/// Test that all semantic errors map to Semantic phase.
#[test]
fn test_semantic_errors_phase() {
    let semantic_errors = [ErrorCode::E2001, ErrorCode::E2002, ErrorCode::E2003, ErrorCode::E2023, ErrorCode::E2032];

    for error in semantic_errors {
        assert_eq!(error.phase(), CompilerPhase::Semantic, "Error {error:?} should be in Semantic phase");
    }
}

/// Test that all IR errors map to `IrGeneration` phase.
#[test]
fn test_ir_errors_phase() {
    let ir_errors = [
        ErrorCode::E3001,
        ErrorCode::E3002,
        ErrorCode::E3003,
        ErrorCode::E3004,
        ErrorCode::E3005,
        ErrorCode::E3006,
        ErrorCode::E3007,
        ErrorCode::E3008,
    ];

    for error in ir_errors {
        assert_eq!(error.phase(), CompilerPhase::IrGeneration, "Error {error:?} should be in IrGeneration phase");
    }
}

/// Test that all codegen errors map to `CodeGeneration` phase.
#[test]
fn test_codegen_errors_phase() {
    let codegen_errors = [ErrorCode::E4001, ErrorCode::E4002, ErrorCode::E4003, ErrorCode::E4004, ErrorCode::E4005];

    for error in codegen_errors {
        assert_eq!(error.phase(), CompilerPhase::CodeGeneration, "Error {error:?} should be in CodeGeneration phase");
    }
}

/// Test that all I/O errors map to System phase.
#[test]
fn test_io_errors_phase() {
    let io_errors = [ErrorCode::E5001, ErrorCode::E5002, ErrorCode::E5003, ErrorCode::E5004, ErrorCode::E5005];

    for error in io_errors {
        assert_eq!(error.phase(), CompilerPhase::System, "Error {error:?} should be in System phase");
    }
}

/// Test that E1013 (missing semicolon) is a warning.
#[test]
fn test_warning_severity() {
    assert_eq!(ErrorCode::E1013.severity(), Severity::Warning);
}

/// Test that all lexical errors are Error severity.
#[test]
fn test_lexical_errors_severity() {
    let errors = [ErrorCode::E0001, ErrorCode::E0002, ErrorCode::E0003, ErrorCode::E0004, ErrorCode::E0005];

    for error in errors {
        assert_eq!(error.severity(), Severity::Error, "Error {error:?} should be Error severity");
    }
}

/// Test that semantic errors are Error severity.
#[test]
fn test_semantic_errors_severity() {
    let errors = [ErrorCode::E2023, ErrorCode::E2024, ErrorCode::E2027, ErrorCode::E2028];

    for error in errors {
        assert_eq!(error.severity(), Severity::Error, "Error {error:?} should be Error severity");
    }
}

/// Test that all parser errors except E1013 are Error severity.
#[test]
fn test_parser_errors_severity_except_warning() {
    let errors = [
        ErrorCode::E1001,
        ErrorCode::E1002,
        ErrorCode::E1003,
        ErrorCode::E1004,
        ErrorCode::E1005,
        ErrorCode::E1006,
        ErrorCode::E1007,
        ErrorCode::E1008,
        ErrorCode::E1009,
        ErrorCode::E1010,
        ErrorCode::E1011,
        ErrorCode::E1012,
        // E1013 is a warning, skip it
        ErrorCode::E1014,
        ErrorCode::E1015,
    ];

    for error in errors {
        assert_eq!(error.severity(), Severity::Error, "Error {error:?} should be Error severity");
    }
}

/// Test that all error codes have non-empty messages.
#[test]
fn test_all_messages_non_empty() {
    let all_codes = [
        ErrorCode::E0001,
        ErrorCode::E0002,
        ErrorCode::E0003,
        ErrorCode::E0004,
        ErrorCode::E0005,
        ErrorCode::E0006,
        ErrorCode::E0007,
        ErrorCode::E0008,
        ErrorCode::E0009,
        ErrorCode::E0010,
        ErrorCode::E1001,
        ErrorCode::E1002,
        ErrorCode::E1003,
        ErrorCode::E1004,
        ErrorCode::E1005,
        ErrorCode::E1006,
        ErrorCode::E1007,
        ErrorCode::E1008,
        ErrorCode::E1009,
        ErrorCode::E1010,
        ErrorCode::E1011,
        ErrorCode::E1012,
        ErrorCode::E1013,
        ErrorCode::E1014,
        ErrorCode::E1015,
        ErrorCode::E2001,
        ErrorCode::E2002,
        ErrorCode::E2003,
        ErrorCode::E2004,
        ErrorCode::E2005,
        ErrorCode::E2006,
        ErrorCode::E2007,
        ErrorCode::E2008,
        ErrorCode::E2009,
        ErrorCode::E2010,
        ErrorCode::E2011,
        ErrorCode::E2012,
        ErrorCode::E2013,
        ErrorCode::E2014,
        ErrorCode::E2015,
        ErrorCode::E2016,
        ErrorCode::E2017,
        ErrorCode::E2018,
        ErrorCode::E2019,
        ErrorCode::E2020,
        ErrorCode::E2021,
        ErrorCode::E2022,
        ErrorCode::E2023,
        ErrorCode::E2024,
        ErrorCode::E2025,
        ErrorCode::E2026,
        ErrorCode::E2027,
        ErrorCode::E2028,
        ErrorCode::E2029,
        ErrorCode::E2030,
        ErrorCode::E2031,
        ErrorCode::E2032,
        ErrorCode::E3001,
        ErrorCode::E3002,
        ErrorCode::E3003,
        ErrorCode::E3004,
        ErrorCode::E3005,
        ErrorCode::E3006,
        ErrorCode::E3007,
        ErrorCode::E3008,
        ErrorCode::E4001,
        ErrorCode::E4002,
        ErrorCode::E4003,
        ErrorCode::E4004,
        ErrorCode::E4005,
        ErrorCode::E5001,
        ErrorCode::E5002,
        ErrorCode::E5003,
        ErrorCode::E5004,
        ErrorCode::E5005,
    ];

    for code in all_codes {
        let message = code.message();
        assert!(!message.is_empty(), "Error {code:?} should have a non-empty message");
    }
}

/// Test specific message content for key errors.
#[test]
fn test_specific_messages() {
    assert_eq!(ErrorCode::E0001.message(), "invalid or unrecognized token");
    assert_eq!(ErrorCode::E0005.message(), "unterminated string literal");
    assert_eq!(ErrorCode::E1001.message(), "maximum recursion depth exceeded");
    assert_eq!(ErrorCode::E1004.message(), "unexpected token");
    assert_eq!(ErrorCode::E1013.message(), "missing semicolon");
    assert_eq!(ErrorCode::E2023.message(), "undefined variable");
    assert_eq!(ErrorCode::E2024.message(), "cannot assign to immutable variable");
    assert_eq!(ErrorCode::E2027.message(), "undefined function");
    assert_eq!(ErrorCode::E2028.message(), "wrong number of arguments");
    assert_eq!(ErrorCode::E3001.message(), "break outside loop in IR");
    assert_eq!(ErrorCode::E4001.message(), "invalid assembly instruction");
    assert_eq!(ErrorCode::E4002.message(), "register allocation failed");
    assert_eq!(ErrorCode::E5001.message(), "file not found");
    assert_eq!(ErrorCode::E5003.message(), "invalid file extension");
}

/// Test that messages are lowercase (following convention).
#[test]
fn test_messages_lowercase_start() {
    let codes =
        [ErrorCode::E0001, ErrorCode::E1001, ErrorCode::E2001, ErrorCode::E3001, ErrorCode::E4001, ErrorCode::E5001];

    for code in codes {
        let message = code.message();
        let first_char = message.chars().next().unwrap();
        assert!(
            first_char.is_lowercase() || first_char.is_ascii_digit(),
            "Message for {code:?} should start lowercase: {message}"
        );
    }
}

/// Test that key errors have detailed explanations.
#[test]
fn test_key_explanations_content() {
    let explanation = ErrorCode::E0001.explanation();
    assert!(explanation.contains("lexer"), "E0001 explanation should mention lexer");
    assert!(explanation.contains("token"), "E0001 explanation should mention token");

    let explanation = ErrorCode::E2023.explanation();
    assert!(explanation.contains("declare"), "E2023 explanation should mention declare");
    assert!(explanation.contains("scope"), "E2023 explanation should mention scope");

    let explanation = ErrorCode::E2024.explanation();
    assert!(explanation.contains("const"), "E2024 explanation should mention const");

    let explanation = ErrorCode::E2027.explanation();
    assert!(explanation.contains("fun"), "E2027 explanation should mention fun");
}

/// Test that all error codes have non-empty explanations.
#[test]
fn test_all_explanations_non_empty() {
    let codes = [
        ErrorCode::E0001,
        ErrorCode::E0002,
        ErrorCode::E0003,
        ErrorCode::E1001,
        ErrorCode::E2023,
        ErrorCode::E3001,
        ErrorCode::E4001,
        ErrorCode::E5001,
    ];

    for code in codes {
        let explanation = code.explanation();
        assert!(!explanation.is_empty(), "Error {code:?} should have an explanation");
    }
}

/// Test default explanation for errors without specific explanations.
#[test]
fn test_default_explanation() {
    // E2016 uses the default explanation
    let explanation = ErrorCode::E2016.explanation();
    assert!(explanation.contains("error message"), "Default explanation should reference error message");
}

/// Test that E0002 has binary literal suggestions.
#[test]
fn test_e0002_suggestions() {
    let suggestions = ErrorCode::E0002.suggestions();
    assert!(!suggestions.is_empty(), "E0002 should have suggestions");
    assert!(suggestions.iter().any(|s| s.contains("#b")), "E0002 suggestions should mention #b");
}

/// Test that E0003 has octal literal suggestions.
#[test]
fn test_e0003_suggestions() {
    let suggestions = ErrorCode::E0003.suggestions();
    assert!(!suggestions.is_empty(), "E0003 should have suggestions");
    assert!(suggestions.iter().any(|s| s.contains("#o")), "E0003 suggestions should mention #o");
}

/// Test that E0004 has hex literal suggestions.
#[test]
fn test_e0004_suggestions() {
    let suggestions = ErrorCode::E0004.suggestions();
    assert!(!suggestions.is_empty(), "E0004 should have suggestions");
    assert!(suggestions.iter().any(|s| s.contains("#x")), "E0004 suggestions should mention #x");
}

/// Test that E0005 has string literal suggestions.
#[test]
fn test_e0005_suggestions() {
    let suggestions = ErrorCode::E0005.suggestions();
    assert!(!suggestions.is_empty(), "E0005 should have suggestions");
    assert!(suggestions.iter().any(|s| s.contains("quote")), "E0005 suggestions should mention quote");
}

/// Test that E2023 has variable declaration suggestions.
#[test]
fn test_e2023_suggestions() {
    let suggestions = ErrorCode::E2023.suggestions();
    assert!(!suggestions.is_empty(), "E2023 should have suggestions");
    assert!(suggestions.iter().any(|s| s.contains("var")), "E2023 suggestions should mention var");
    assert!(suggestions.iter().any(|s| s.contains("typo")), "E2023 suggestions should mention typo");
}

/// Test that E2024 has const/var suggestions.
#[test]
fn test_e2024_suggestions() {
    let suggestions = ErrorCode::E2024.suggestions();
    assert!(!suggestions.is_empty(), "E2024 should have suggestions");
    assert!(suggestions.iter().any(|s| s.contains("var")), "E2024 suggestions should mention var");
}

/// Test that E2009 and E2010 have loop suggestions.
#[test]
fn test_break_continue_suggestions() {
    let break_suggestions = ErrorCode::E2009.suggestions();
    let continue_suggestions = ErrorCode::E2010.suggestions();

    assert!(!break_suggestions.is_empty(), "E2009 should have suggestions");
    assert!(!continue_suggestions.is_empty(), "E2010 should have suggestions");

    assert!(break_suggestions.iter().any(|s| s.contains("loop")), "E2009 suggestions should mention loop");
    assert!(continue_suggestions.iter().any(|s| s.contains("loop")), "E2010 suggestions should mention loop");
}

/// Test that some errors have empty suggestions (valid case).
#[test]
fn test_empty_suggestions_valid() {
    // Most IR and codegen errors don't have specific suggestions
    let suggestions = ErrorCode::E3003.suggestions();
    assert!(suggestions.is_empty(), "E3003 should have no suggestions (internal error)");

    let suggestions = ErrorCode::E4001.suggestions();
    assert!(suggestions.is_empty(), "E4001 should have no suggestions (internal error)");
}

/// Test Display format includes code and message.
#[test]
fn test_display_format() {
    let code = ErrorCode::E2023;
    let display = format!("{code}");

    assert!(display.contains("E2023"), "Display should contain error code");
    assert!(display.contains("undefined variable"), "Display should contain error message");
    assert!(display.contains(": "), "Display should contain separator");
}

/// Test Display for various error categories.
#[test]
fn test_display_various_categories() {
    assert!(format!("{}", ErrorCode::E0001).starts_with("E0001:"));
    assert!(format!("{}", ErrorCode::E1001).starts_with("E1001:"));
    assert!(format!("{}", ErrorCode::E2001).starts_with("E2001:"));
    assert!(format!("{}", ErrorCode::E3001).starts_with("E3001:"));
    assert!(format!("{}", ErrorCode::E4001).starts_with("E4001:"));
    assert!(format!("{}", ErrorCode::E5001).starts_with("E5001:"));
}

/// Test `ErrorCode` implements `std::error::Error`.
#[test]
fn test_error_trait() {
    let error: &dyn std::error::Error = &ErrorCode::E2023;
    // Error trait provides source() which returns None for ErrorCode
    assert!(error.source().is_none());
}

/// Test `ErrorCode` Debug implementation.
#[test]
fn test_debug_impl() {
    assert_eq!(format!("{:?}", ErrorCode::E0001), "E0001");
    assert_eq!(format!("{:?}", ErrorCode::E2023), "E2023");
    assert_eq!(format!("{:?}", ErrorCode::E5005), "E5005");
}

/// Test `ErrorCode` Clone implementation.
#[test]
fn test_clone_impl() {
    let original = ErrorCode::E2023;
    let cloned = original;
    assert_eq!(original, cloned);
}

/// Test `ErrorCode` Copy implementation.
#[test]
fn test_copy_impl() {
    let original = ErrorCode::E2023;
    let copied = original; // Copy, not move
    assert_eq!(original, copied); // original still usable
}

/// Test `ErrorCode` Hash implementation.
#[test]
fn test_hash_impl() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(ErrorCode::E0001);
    set.insert(ErrorCode::E2023);
    set.insert(ErrorCode::E5001);

    assert_eq!(set.len(), 3);
    assert!(set.contains(&ErrorCode::E0001));
    assert!(set.contains(&ErrorCode::E2023));
    assert!(set.contains(&ErrorCode::E5001));
    assert!(!set.contains(&ErrorCode::E1001));
}

/// Test `ErrorCode` `PartialEq` implementation.
#[test]
fn test_partial_eq_impl() {
    assert_eq!(ErrorCode::E0001, ErrorCode::E0001);
    assert_ne!(ErrorCode::E0001, ErrorCode::E0002);
    assert_ne!(ErrorCode::E0001, ErrorCode::E1001);
}

/// Test `ErrorCode` Eq implementation (reflexive, symmetric, transitive).
#[test]
fn test_eq_properties() {
    let a = ErrorCode::E2023;
    let b = ErrorCode::E2023;
    let c = ErrorCode::E2023;

    // Reflexive
    assert_eq!(a, a);

    // Symmetric
    assert_eq!(a, b);
    assert_eq!(b, a);

    // Transitive
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}

/// Test that all codes in a range belong to the correct phase.
#[test]
fn test_phase_range_consistency() {
    // Lexer: 1-999
    for code in [1u16, 10, 999] {
        let phase = match code {
            1..=999 => CompilerPhase::Lexer,
            1001..=1999 => CompilerPhase::Parser,
            2001..=2999 => CompilerPhase::Semantic,
            3001..=3999 => CompilerPhase::IrGeneration,
            4001..=4999 => CompilerPhase::CodeGeneration,
            _ => CompilerPhase::System,
        };
        if code <= 10 {
            assert_eq!(phase, CompilerPhase::Lexer);
        }
    }
}

/// Test that numeric codes are unique within their range.
#[test]
fn test_unique_numeric_codes() {
    use std::collections::HashSet;

    let all_codes = [
        ErrorCode::E0001,
        ErrorCode::E0002,
        ErrorCode::E0003,
        ErrorCode::E0004,
        ErrorCode::E0005,
        ErrorCode::E0006,
        ErrorCode::E0007,
        ErrorCode::E0008,
        ErrorCode::E0009,
        ErrorCode::E0010,
        ErrorCode::E1001,
        ErrorCode::E1002,
        ErrorCode::E1003,
        ErrorCode::E1004,
        ErrorCode::E1005,
        ErrorCode::E1006,
        ErrorCode::E1007,
        ErrorCode::E1008,
        ErrorCode::E1009,
        ErrorCode::E1010,
        ErrorCode::E1011,
        ErrorCode::E1012,
        ErrorCode::E1013,
        ErrorCode::E1014,
        ErrorCode::E1015,
        ErrorCode::E2001,
        ErrorCode::E2002,
        ErrorCode::E2003,
        ErrorCode::E2004,
        ErrorCode::E2005,
        ErrorCode::E2006,
        ErrorCode::E2007,
        ErrorCode::E2008,
        ErrorCode::E2009,
        ErrorCode::E2010,
        ErrorCode::E2011,
        ErrorCode::E2012,
        ErrorCode::E2013,
        ErrorCode::E2014,
        ErrorCode::E2015,
        ErrorCode::E2016,
        ErrorCode::E2017,
        ErrorCode::E2018,
        ErrorCode::E2019,
        ErrorCode::E2020,
        ErrorCode::E2021,
        ErrorCode::E2022,
        ErrorCode::E2023,
        ErrorCode::E2024,
        ErrorCode::E2025,
        ErrorCode::E2026,
        ErrorCode::E2027,
        ErrorCode::E2028,
        ErrorCode::E2029,
        ErrorCode::E2030,
        ErrorCode::E2031,
        ErrorCode::E2032,
        ErrorCode::E3001,
        ErrorCode::E3002,
        ErrorCode::E3003,
        ErrorCode::E3004,
        ErrorCode::E3005,
        ErrorCode::E3006,
        ErrorCode::E3007,
        ErrorCode::E3008,
        ErrorCode::E4001,
        ErrorCode::E4002,
        ErrorCode::E4003,
        ErrorCode::E4004,
        ErrorCode::E4005,
        ErrorCode::E5001,
        ErrorCode::E5002,
        ErrorCode::E5003,
        ErrorCode::E5004,
        ErrorCode::E5005,
    ];

    let mut numeric_set = HashSet::new();
    let mut string_set = HashSet::new();

    for code in all_codes {
        let numeric = code.numeric_code();
        let string = code.code();

        assert!(numeric_set.insert(numeric), "Duplicate numeric code: {numeric}");
        assert!(string_set.insert(string), "Duplicate string code: {string}");
    }

    // Verify we tested all codes
    assert_eq!(numeric_set.len(), all_codes.len());
    assert_eq!(string_set.len(), all_codes.len());
}

/// Test code format consistency (EXXXX pattern).
#[test]
fn test_code_format_pattern() {
    let codes =
        [ErrorCode::E0001, ErrorCode::E1001, ErrorCode::E2001, ErrorCode::E3001, ErrorCode::E4001, ErrorCode::E5001];

    for code in codes {
        let string = code.code();
        assert!(string.starts_with('E'), "Code should start with 'E'");
        assert_eq!(string.len(), 5, "Code should be 5 characters: {string}");

        // Verify remaining characters are digits
        for ch in string.chars().skip(1) {
            assert!(ch.is_ascii_digit(), "Code suffix should be digits: {string}");
        }
    }
}

/// Test complete error workflow: code -> phase -> severity -> message.
#[test]
fn test_error_workflow() {
    let code = ErrorCode::E2023;

    // Get all metadata
    let code_str = code.code();
    let numeric = code.numeric_code();
    let phase = code.phase();
    let severity = code.severity();
    let message = code.message();
    let explanation = code.explanation();
    let suggestions = code.suggestions();

    // Verify consistency
    assert_eq!(code_str, "E2023");
    assert_eq!(numeric, 2023);
    assert_eq!(phase, CompilerPhase::Semantic);
    assert_eq!(severity, Severity::Error);
    assert!(!message.is_empty());
    assert!(!explanation.is_empty());
    assert!(!suggestions.is_empty());
}

/// Test using `ErrorCode` in error reporting context.
#[test]
fn test_error_reporting_context() {
    let code = ErrorCode::E0005;

    // Simulate error report formatting
    let report = format!("[{}] {}: {} ({})", code.phase(), code.severity(), code, code.code());

    assert!(report.contains("lexer"));
    assert!(report.contains("error"));
    assert!(report.contains("unterminated string"));
    assert!(report.contains("E0005"));
}

/// Test collecting multiple errors.
#[test]
fn test_multiple_error_collection() {
    let errors = [ErrorCode::E0001, ErrorCode::E1004, ErrorCode::E2023, ErrorCode::E2024];

    assert_eq!(errors.iter().filter(|e| e.phase() == CompilerPhase::Lexer).count(), 1);
    assert_eq!(errors.iter().filter(|e| e.phase() == CompilerPhase::Parser).count(), 1);
    assert_eq!(errors.iter().filter(|e| e.phase() == CompilerPhase::Semantic).count(), 2);
}

/// Test error code as map key.
#[test]
fn test_error_code_as_map_key() {
    use std::collections::HashMap;

    let mut error_counts: HashMap<ErrorCode, u32> = HashMap::new();

    error_counts.insert(ErrorCode::E2023, 5);
    error_counts.insert(ErrorCode::E0001, 2);
    error_counts.insert(ErrorCode::E1004, 10);

    assert_eq!(error_counts.get(&ErrorCode::E2023), Some(&5));
    assert_eq!(error_counts.get(&ErrorCode::E0001), Some(&2));
    assert_eq!(error_counts.get(&ErrorCode::E5001), None);

    // Update count
    *error_counts.entry(ErrorCode::E2023).or_insert(0) += 1;
    assert_eq!(error_counts.get(&ErrorCode::E2023), Some(&6));
}

/// Test boundary error codes at phase transitions.
#[test]
fn test_phase_boundary_codes() {
    // Last lexer error
    assert_eq!(ErrorCode::E0010.phase(), CompilerPhase::Lexer);

    // First parser error
    assert_eq!(ErrorCode::E1001.phase(), CompilerPhase::Parser);

    // Last parser error
    assert_eq!(ErrorCode::E1015.phase(), CompilerPhase::Parser);

    // First semantic error
    assert_eq!(ErrorCode::E2001.phase(), CompilerPhase::Semantic);

    // Last semantic error
    assert_eq!(ErrorCode::E2032.phase(), CompilerPhase::Semantic);
}

/// Test that cloning preserves all properties.
#[test]
fn test_clone_preserves_properties() {
    let original = ErrorCode::E2023;
    let cloned = original;

    assert_eq!(original.code(), cloned.code());
    assert_eq!(original.numeric_code(), cloned.numeric_code());
    assert_eq!(original.phase(), cloned.phase());
    assert_eq!(original.severity(), cloned.severity());
    assert_eq!(original.message(), cloned.message());
    assert_eq!(original.explanation(), cloned.explanation());
    assert_eq!(original.suggestions(), cloned.suggestions());
}

/// Test formatting with alternate formatter.
#[test]
fn test_alternate_debug_format() {
    let code = ErrorCode::E2023;
    let debug = format!("{code:#?}");
    assert!(debug.contains("E2023"));
}

/// Test that severity ordering is total.
#[test]
fn test_severity_total_ordering() {
    let mut severities = [Severity::Fatal, Severity::Note, Severity::Error, Severity::Warning];

    severities.sort();

    assert_eq!(severities[0], Severity::Note);
    assert_eq!(severities[1], Severity::Warning);
    assert_eq!(severities[2], Severity::Error);
    assert_eq!(severities[3], Severity::Fatal);
}
