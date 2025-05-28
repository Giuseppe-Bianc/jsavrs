use insta::assert_debug_snapshot;
use jsavrs::tokens::token_kind::{TokenKind, split_numeric_and_suffix};
use logos::Logos;

#[test]
fn test_empty_slice_for_split_numeric_and_suffix() {
    let result = split_numeric_and_suffix("");

    assert_debug_snapshot!("empty_slice_for_split_numeric_and_suffix", result);
}
/// Helper function to tokenize input into a Vec<TokenKind>
#[allow(clippy::while_let_on_iterator)]
fn tokenize(input: &str) -> Vec<Result<TokenKind, ()>> {
    let mut lex = TokenKind::lexer(input);
    let mut tokens = Vec::new();
    while let Some(token) = lex.next() {
        tokens.push(token);
    }
    tokens
}

#[test]
fn test_operators() {
    let input = r#"
		+= -= == = != <= >= ++ -- || && << >> %= ^= 
		+ - * / < > ! ^ % | & = : , .
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_keywords() {
    let input = r#"
		fun if else return while for main var const nullptr 
		break continue true false
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_identifiers() {
    let input = r#"
		ascii_ident _under_score unicode_ñññ var123 
		\u{9876}_unicode
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_numeric_literals() {
    let input = r#"
		123 45u 6.7f 8.9e10 3.0d 1e3f 123.456 
		#b1010u #o777 #x1AFU #xdeadbeef
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_string_char_literals() {
    let input = r#"
		"hello\nworld" 'a' '\t' "\"escaped\"" '\''
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_brackets_and_punctuation() {
    let input = "() [] {} ;";
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_types() {
    let input = r#"
		i8 i16 i32 i64 u8 u16 u32 u64 
		f32 f64 char string bool
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_comments_and_whitespace() {
    let input = r#"
		// This is a comment
		/* Multi-line
		   comment */
		valid_after_comment
	"#;
    let tokens = tokenize(input);
    assert_debug_snapshot!(tokens);
}
