use insta::assert_debug_snapshot;
use jsavrs::tokens::number::Number;
use jsavrs::tokens::token_kind::{split_numeric_and_suffix, TokenKind};
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

#[test]
fn test_eof_and_ignored() {
    assert_debug_snapshot!("eof", TokenKind::Eof.to_string());
    assert_debug_snapshot!("comment", TokenKind::Comment.to_string());
    assert_debug_snapshot!("whitespace", TokenKind::Whitespace.to_string());
}

#[test]
fn test_identifier_ascii_normal() {
    let ident = "foo".to_string();
    assert_debug_snapshot!(TokenKind::IdentifierAscii(ident.clone()).to_string());
}

#[test]
fn test_identifier_ascii_empty() {
    let ident = "".to_string();
    assert_debug_snapshot!(TokenKind::IdentifierAscii(ident.clone()).to_string());
}

#[test]
fn test_identifier_unicode() {
    let ident = "προεδομή".to_string(); // qualche stringa Unicode
    assert_debug_snapshot!(TokenKind::IdentifierUnicode(ident.clone()).to_string());
}

#[test]
fn test_numeric_integer() {
    // Qui assumiamo che Number::Integer(i64) sia un modo valido per costruire un Number
    let num = Number::Integer(123);
    assert_debug_snapshot!("numeric", TokenKind::Numeric(num.clone()).to_string());
    assert_debug_snapshot!(
        "hexadecimal",
        TokenKind::Hexadecimal(num.clone()).to_string()
    );
    assert_debug_snapshot!("octal", TokenKind::Octal(num.clone()).to_string());
    assert_debug_snapshot!("binary", TokenKind::Binary(num.clone()).to_string());
}

#[test]
fn test_string_literal_simple() {
    let s = "hello".to_string();
    assert_debug_snapshot!(TokenKind::StringLiteral(s.clone()).to_string());
}

#[test]
fn test_string_literal_with_quotes_inside() {
    let s = "he said \"ciao\"".to_string();
    assert_debug_snapshot!(TokenKind::StringLiteral(s.clone()).to_string());
}

#[test]
fn test_char_literal_simple() {
    let c = "x".to_string();
    assert_debug_snapshot!(TokenKind::CharLiteral(c.clone()).to_string());
}

#[test]
fn test_char_literal_unicode() {
    let c = "ψ".to_string();
    assert_debug_snapshot!(TokenKind::CharLiteral(c.clone()).to_string());
}

#[test]
fn test_keyword_bool_true_false() {
    assert_debug_snapshot!("true", TokenKind::KeywordBool(true).to_string());
    assert_debug_snapshot!("false", TokenKind::KeywordBool(false).to_string());
}

#[test]
fn test_keyword_nullptr() {
    assert_debug_snapshot!(TokenKind::KeywordNullptr.to_string());
}

// ——— Test per tutte le keyword principali ———
#[test]
fn test_all_keywords() {
    let mapping = vec![
        TokenKind::KeywordFun,
        TokenKind::KeywordIf,
        TokenKind::KeywordElse,
        TokenKind::KeywordVar,
        TokenKind::KeywordConst,
        TokenKind::KeywordReturn,
        TokenKind::KeywordWhile,
        TokenKind::KeywordFor,
        TokenKind::KeywordBreak,
        TokenKind::KeywordContinue,
        TokenKind::KeywordMain,
    ];

    let input_result: Vec<(TokenKind, String)> = mapping
        .iter()
        .map(|kind| (kind.clone(), kind.to_string()))
        .collect();

    assert_debug_snapshot!(input_result);
}

// ——— Test per tutti i tipi primari ———
#[test]
fn test_all_primitive_types() {
    let mapping = vec![
        TokenKind::TypeI8,
        TokenKind::TypeI16,
        TokenKind::TypeI32,
        TokenKind::TypeI64,
        TokenKind::TypeU8,
        TokenKind::TypeU16,
        TokenKind::TypeU32,
        TokenKind::TypeU64,
        TokenKind::TypeF32,
        TokenKind::TypeF64,
        TokenKind::TypeChar,
        TokenKind::TypeString,
        TokenKind::TypeBool,
    ];

    let input_result: Vec<(TokenKind, String)> = mapping
        .iter()
        .map(|kind| (kind.clone(), kind.to_string()))
        .collect();

    assert_debug_snapshot!(input_result);
}

// ——— Test per punteggiatura e simboli singoli ———
#[test]
fn test_punctuation() {
    let mapping = vec![
        TokenKind::OpenParen,
        TokenKind::CloseParen,
        TokenKind::OpenBrace,
        TokenKind::CloseBrace,
        TokenKind::OpenBracket,
        TokenKind::CloseBracket,
        TokenKind::Semicolon,
        TokenKind::Colon,
        TokenKind::Comma,
        TokenKind::Dot,
    ];

    let input_result: Vec<(TokenKind, String)> = mapping
        .iter()
        .map(|kind| (kind.clone(), kind.to_string()))
        .collect();

    assert_debug_snapshot!(input_result);
}

// ——— Test per operatori semplici e composti ———
#[test]
fn test_operators_single_and_multi_char() {
    let mapping = vec![
        TokenKind::Plus,
        TokenKind::PlusPlus,
        TokenKind::MinusMinus,
        TokenKind::PlusEqual,
        TokenKind::MinusEqual,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::Percent,
        TokenKind::PercentEqual,
        TokenKind::Equal,
        TokenKind::EqualEqual,
        TokenKind::NotEqual,
        TokenKind::Less,
        TokenKind::LessEqual,
        TokenKind::Greater,
        TokenKind::GreaterEqual,
        TokenKind::AndAnd,
        TokenKind::OrOr,
        TokenKind::Not,
        TokenKind::And,
        TokenKind::Or,
        TokenKind::Xor,
        TokenKind::XorEqual,
        TokenKind::ShiftLeft,
        TokenKind::ShiftRight,
    ];

    let input_result: Vec<(TokenKind, String)> = mapping
        .iter()
        .map(|kind| (kind.clone(), kind.to_string()))
        .collect();

    assert_debug_snapshot!(input_result);
}
