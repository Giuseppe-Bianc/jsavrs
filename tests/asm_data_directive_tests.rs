use jsavrs::asm::*;

#[test]
fn test_data_directive_db() {
    let bytes = vec![0x10, 0x20, 0x30];
    let directive = DataDirective::Db(bytes);

    if let DataDirective::Db(values) = directive {
        assert_eq!(values, vec![0x10, 0x20, 0x30]);
    } else {
        panic!("Expected Db variant");
    }
}

#[test]
fn test_data_directive_dword() {
    let words = vec![0x1234, 0x5678];
    let directive = DataDirective::Dw(words);

    if let DataDirective::Dw(values) = directive {
        assert_eq!(values, vec![0x1234, 0x5678]);
    } else {
        panic!("Expected Dw variant");
    }
}

#[test]
fn test_data_directive_dd() {
    let dwords = vec![0x12345678, 0x9ABCDEF0];
    let directive = DataDirective::Dd(dwords);

    if let DataDirective::Dd(values) = directive {
        assert_eq!(values, vec![0x12345678, 0x9ABCDEF0]);
    } else {
        panic!("Expected Dd variant");
    }
}

#[test]
fn test_data_directive_dq() {
    let qwords = vec![0x123456789ABCDEF0, 0xFEDCBA9876543210];
    let directive = DataDirective::Dq(qwords);

    if let DataDirective::Dq(values) = directive {
        assert_eq!(values, vec![0x123456789ABCDEF0, 0xFEDCBA9876543210]);
    } else {
        panic!("Expected Dq variant");
    }
}

#[test]
fn test_data_directive_asciz() {
    let directive = DataDirective::new_asciz("Hello");

    if let DataDirective::Asciz(string, terminator) = directive {
        assert_eq!(string, "Hello");
        assert_eq!(terminator, 0x00);
    } else {
        panic!("Expected Asciz variant");
    }
}

#[test]
fn test_data_directive_asciz_with_terminator() {
    let directive = DataDirective::new_asciiz_with_terminator("Hello", 0x0A);

    if let DataDirective::Asciz(string, terminator) = directive {
        assert_eq!(string, "Hello");
        assert_eq!(terminator, 0x0A);
    } else {
        panic!("Expected Asciz variant");
    }
}

#[test]
fn test_data_directive_ascii() {
    let directive = DataDirective::Ascii("Hello".to_string());

    if let DataDirective::Ascii(string) = directive {
        assert_eq!(string, "Hello");
    } else {
        panic!("Expected Ascii variant");
    }
}

#[test]
fn test_data_directive_resb() {
    let directive = DataDirective::Resb(256);

    if let DataDirective::Resb(size) = directive {
        assert_eq!(size, 256);
    } else {
        panic!("Expected Resb variant");
    }
}

#[test]
fn test_data_directive_resw() {
    let directive = DataDirective::Resw(100);

    if let DataDirective::Resw(size) = directive {
        assert_eq!(size, 100);
    } else {
        panic!("Expected Resw variant");
    }
}

#[test]
fn test_data_directive_resd() {
    let directive = DataDirective::Resd(50);

    if let DataDirective::Resd(size) = directive {
        assert_eq!(size, 50);
    } else {
        panic!("Expected Resd variant");
    }
}

#[test]
fn test_data_directive_resq() {
    let directive = DataDirective::Resq(25);

    if let DataDirective::Resq(size) = directive {
        assert_eq!(size, 25);
    } else {
        panic!("Expected Resq variant");
    }
}

#[test]
fn test_data_directive_equ_constant() {
    let directive = DataDirective::new_equ_constant(42);

    if let DataDirective::Equ(EquExpression::Constant(value)) = directive {
        assert_eq!(value, 42);
    } else {
        panic!("Expected Equ with Constant variant");
    }
}

#[test]
fn test_data_directive_equ_length_of() {
    let directive = DataDirective::new_equ_length_of("msg");

    if let DataDirective::Equ(EquExpression::LengthOf(label)) = directive {
        assert_eq!(label, "msg");
    } else {
        panic!("Expected Equ with LengthOf variant");
    }
}

#[test]
fn test_data_directive_equ_generic() {
    let directive = DataDirective::new_equ_generic("BUFFER_SIZE * 2");

    if let DataDirective::Equ(EquExpression::Generic(expr)) = directive {
        assert_eq!(expr, "BUFFER_SIZE * 2");
    } else {
        panic!("Expected Equ with Generic variant");
    }
}

#[test]
fn test_equ_expression_constants() {
    let constant_expr = EquExpression::Constant(-42);
    if let EquExpression::Constant(value) = constant_expr {
        assert_eq!(value, -42);
    }

    let length_expr = EquExpression::LengthOf("start".to_string());
    if let EquExpression::LengthOf(label) = length_expr {
        assert_eq!(label, "start");
    }

    let generic_expr = EquExpression::Generic("expr".to_string());
    if let EquExpression::Generic(expr) = generic_expr {
        assert_eq!(expr, "expr");
    }
}

#[test]
fn test_data_directive_display_db() {
    let directive = DataDirective::Db(vec![0x10, 0x20]);
    let display = format!("{}", directive);
    assert_eq!(display, "db 0x10, 0x20");
}

#[test]
fn test_data_directive_display_dw() {
    let directive = DataDirective::Dw(vec![0x1234, 0x5678]);
    let display = format!("{}", directive);
    assert_eq!(display, "dw 0x1234, 0x5678");
}

#[test]
fn test_data_directive_display_dd() {
    let directive = DataDirective::Dd(vec![0x12345678, 0x9ABCDEF0]);
    let display = format!("{}", directive);
    assert_eq!(display, "dd 0x12345678, 0x9abcdef0");
}

#[test]
fn test_data_directive_display_dq() {
    let directive = DataDirective::Dq(vec![0x123456789ABCDEF0]);
    let display = format!("{}", directive);
    assert_eq!(display, "dq 0x123456789abcdef0");
}

#[test]
fn test_data_directive_display_asciz() {
    let directive = DataDirective::new_asciz("Hello");
    let display = format!("{}", directive);
    assert_eq!(display, "db \"Hello\", 0");
}

#[test]
fn test_data_directive_display_ascii() {
    let directive = DataDirective::Ascii("Hello".to_string());
    let display = format!("{}", directive);
    assert_eq!(display, "db \"Hello\"");
}

#[test]
fn test_data_directive_display_resb() {
    let directive = DataDirective::Resb(256);
    let display = format!("{}", directive);
    assert_eq!(display, "resb 256");
}

#[test]
fn test_data_directive_display_resw() {
    let directive = DataDirective::Resw(100);
    let display = format!("{}", directive);
    assert_eq!(display, "resw 100");
}

#[test]
fn test_data_directive_display_resd() {
    let directive = DataDirective::Resd(50);
    let display = format!("{}", directive);
    assert_eq!(display, "resd 50");
}

#[test]
fn test_data_directive_display_resq() {
    let directive = DataDirective::Resq(25);
    let display = format!("{}", directive);
    assert_eq!(display, "resq 25");
}

#[test]
fn test_data_directive_display_equ() {
    let directive = DataDirective::new_equ_constant(42);
    let display = format!("{}", directive);
    assert_eq!(display, "equ 42");

    let length_directive = DataDirective::new_equ_length_of("msg");
    let length_display = format!("{}", length_directive);
    assert_eq!(length_display, "equ $ - msg");

    let generic_directive = DataDirective::new_equ_generic("BUFFER_SIZE * 2");
    let generic_display = format!("{}", generic_directive);
    assert_eq!(generic_display, "equ BUFFER_SIZE * 2");
}

#[test]
fn test_equ_expression_display() {
    let constant = EquExpression::Constant(42);
    assert_eq!(format!("{}", constant), "42");

    let length = EquExpression::LengthOf("label".to_string());
    assert_eq!(format!("{}", length), "$ - label");

    let generic = EquExpression::Generic("expr".to_string());
    assert_eq!(format!("{}", generic), "expr");
}

#[test]
fn test_data_directive_empty_vectors() {
    let empty_db = DataDirective::Db(vec![]);
    assert_eq!(format!("{}", empty_db), "db ");

    let empty_dw = DataDirective::Dw(vec![]);
    assert_eq!(format!("{}", empty_dw), "dw ");

    let empty_dd = DataDirective::Dd(vec![]);
    assert_eq!(format!("{}", empty_dd), "dd ");

    let empty_dq = DataDirective::Dq(vec![]);
    assert_eq!(format!("{}", empty_dq), "dq ");
}

#[test]
fn test_data_directive_single_elements() {
    let single_db = DataDirective::Db(vec![0xFF]);
    assert_eq!(format!("{}", single_db), "db 0xff");

    let single_dq = DataDirective::Dq(vec![0x123456789ABCDEF0]);
    assert_eq!(format!("{}", single_dq), "dq 0x123456789abcdef0");
}

#[test]
fn test_data_directive_with_special_chars_in_string() {
    let directive = DataDirective::Ascii("Hello\nWorld\t\"Test\"".to_string());
    let display = format!("{}", directive);
    assert_eq!(display, "db \"Hello\\\\nWorld\\\\t\\\"Test\\\"\"");

    let directive2 = DataDirective::new_asciz("Test\\Backslash");
    let display2 = format!("{}", directive2);
    assert_eq!(display2, "db \"Test\\\\Backslash\", 0");
}

#[test]
fn test_data_directive_large_values() {
    let max_byte = DataDirective::Db(vec![u8::MAX]);
    assert_eq!(format!("{}", max_byte), "db 0xff");

    let max_word = DataDirective::Dw(vec![u16::MAX]);
    assert_eq!(format!("{}", max_word), "dw 0xffff");

    let max_dword = DataDirective::Dd(vec![u32::MAX]);
    assert_eq!(format!("{}", max_dword), "dd 0xffffffff");

    let max_qword = DataDirective::Dq(vec![u64::MAX]);
    assert_eq!(format!("{}", max_qword), "dq 0xffffffffffffffff");
}

#[test]
fn test_equ_expression_negative_values() {
    let negative = DataDirective::new_equ_constant(-123);
    let display = format!("{}", negative);
    assert_eq!(display, "equ -123");
}

#[test]
fn test_data_directive_clone() {
    let original = DataDirective::new_asciz("test");
    let cloned = original.clone();
    assert_eq!(format!("{}", original), format!("{}", cloned));
}

#[test]
fn test_data_directive_debug() {
    let directive = DataDirective::new_asciz("test");
    let debug_str = format!("{:?}", directive);
    assert!(debug_str.contains("Asciz"));
    assert!(debug_str.contains("test"));
}

#[test]
fn test_equ_expression_clone() {
    let original = EquExpression::LengthOf("label".to_string());
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_equ_expression_debug() {
    let expr = EquExpression::Generic("test".to_string());
    let debug_str = format!("{:?}", expr);
    assert!(debug_str.contains("Generic"));
    assert!(debug_str.contains("test"));
}
