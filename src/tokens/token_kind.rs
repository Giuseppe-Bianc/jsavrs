// src/tokens/token_kind.rs
use crate::tokens::number::Number;
use logos::Logos;

pub fn parse_number(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (numeric_part, suffix) = split_numeric_and_suffix(slice);
    handle_suffix(numeric_part, suffix)
}

/// Splits the input string into numeric part and possible suffix
pub fn split_numeric_and_suffix(slice: &str) -> (&str, Option<String>) {
    if slice.is_empty() {
        return (slice, None);
    }

    let last_char = slice.chars().last().unwrap();
    match last_char {
        'u' | 'U' | 'f' | 'F' | 'd' | 'D' => {
            let (num_part, suffix_part) = slice.split_at(slice.len() - 1);
            (num_part, Some(suffix_part.to_ascii_lowercase()))
        }
        _ => (slice, None),
    }
}

/// Main suffix handling router
pub fn handle_suffix(numeric_part: &str, suffix: Option<String>) -> Option<Number> {
    match suffix.as_deref() {
        Some("u") => handle_unsigned_suffix(numeric_part),
        Some("f") => handle_float_suffix(numeric_part),
        Some("d") | None => handle_default_suffix(numeric_part),
        _ => None,
    }
}

/// Handles unsigned integer suffix case
pub fn handle_unsigned_suffix(numeric_part: &str) -> Option<Number> {
    if is_valid_unsigned(numeric_part) {
        numeric_part
            .parse::<u64>()
            .ok()
            .map(Number::UnsignedInteger)
    } else {
        None
    }
}

/// Validates numeric part for unsigned integers
pub fn is_valid_unsigned(numeric_part: &str) -> bool {
    !numeric_part.contains(['.', 'e', 'E'])
}
/// Handles float suffix case
pub fn handle_float_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, true)
        .or_else(|| numeric_part.parse::<f32>().ok().map(Number::Float32))
}

/// Handles default suffix cases (double or no suffix)
pub fn handle_default_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, false).or_else(|| handle_non_scientific(numeric_part))
}

/// Handles non-scientific notation numbers
pub fn handle_non_scientific(numeric_part: &str) -> Option<Number> {
    if numeric_part.contains('.') {
        numeric_part.parse::<f64>().ok().map(Number::Float64)
    } else {
        numeric_part.parse::<i64>().ok().map(Number::Integer)
    }
}

pub fn parse_scientific(s: &str, is_f32: bool) -> Option<Number> {
    let pos = s.find(['e', 'E'])?;
    let (base_str, exp_str) = s.split_at(pos);
    let exp = exp_str[1..].parse::<i32>().ok()?;

    if is_f32 {
        let base = base_str.parse::<f32>().ok()?;
        Some(Number::Scientific32(base, exp))
    } else {
        let base = base_str.parse::<f64>().ok()?;
        Some(Number::Scientific64(base, exp))
    }
}

// Generic parser for base-specific numbers
pub fn parse_base_number(radix: u32, lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (_, num_part) = slice.split_at(2); // Split off "#b", "#o", or "#x"
    let (num_str, suffix) = match num_part.chars().last() {
        Some('u') | Some('U') => (&num_part[..num_part.len() - 1], true),
        _ => (num_part, false),
    };

    if suffix {
        u64::from_str_radix(num_str, radix)
            .ok()
            .map(Number::UnsignedInteger)
    } else {
        i64::from_str_radix(num_str, radix)
            .ok()
            .map(Number::Integer)
    }
}

pub fn parse_binary(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(2, lex)
}

pub fn parse_octal(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(8, lex)
}

pub fn parse_hex(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(16, lex)
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Operator tokens with correct ordering (longest first)
    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("||")]
    OrOr,
    #[token("&&")]
    AndAnd,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token("%=")]
    PercentEqual,
    #[token("^=")]
    XorEqual,

    // Single-character operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("!")]
    Not,
    #[token("^")]
    Xor,
    #[token("%")]
    Percent,
    #[token("|")]
    Or,
    #[token("&")]
    And,
    #[token("=")]
    Equal,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,

    // Keywords
    #[token("fun")]
    KeywordFun,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("return")]
    KeywordReturn,
    #[token("while")]
    KeywordWhile,
    #[token("for")]
    KeywordFor,
    #[token("main")]
    KeywordMain,
    #[token("var")]
    KeywordVar,
    #[token("const")]
    KeywordConst,
    #[token("nullptr")]
    KeywordNullptr,
    #[token("break")]
    KeywordBreak,
    #[token("continue")]
    KeywordContinue,

    // Literals
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    KeywordBool(bool),

    // Identifiers
    // ASCII identifiers (including underscores)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    IdentifierAscii(String),

    // Unicode identifiers (including underscores)
    #[regex(r"[\p{Letter}\p{Mark}_][\p{Letter}\p{Mark}\p{Number}_]*", |lex| lex.slice().to_string(), priority = 1)]
    IdentifierUnicode(String),

    #[regex(
        r"(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?[ufdUF]?",
        parse_number,
        priority = 4
    )]
    Numeric(Number),

    #[regex(r"#b[01]+[uU]?", parse_binary, priority = 3)]
    Binary(Number),

    #[regex(r"#o[0-7]+[uU]?", parse_octal, priority = 3)]
    Octal(Number),

    // Hexadecimal numbers (medium priority)
    #[regex(r"#x[0-9a-fA-F]+[uU]?", parse_hex, priority = 2)]
    Hexadecimal(Number),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLiteral(String),

    #[regex(r#"'([^'\\]|\\.)'"#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    CharLiteral(String),

    // Parentheses
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    // Square brackets
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    // Curly brackets
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    // Types
    #[token("i8")]
    TypeI8,
    #[token("i16")]
    TypeI16,
    #[token("i32")]
    TypeI32,
    #[token("i64")]
    TypeI64,
    #[token("u8")]
    TypeU8,
    #[token("u16")]
    TypeU16,
    #[token("u32")]
    TypeU32,
    #[token("u64")]
    TypeU64,
    #[token("f32")]
    TypeF32,
    #[token("f64")]
    TypeF64,
    #[token("char")]
    TypeChar,
    #[token("string")]
    TypeString,
    #[token("bool")]
    TypeBool,

    // Whitespace (including Unicode spaces)
    #[regex(
        r"[ \t\r\n\f\u{00A0}\u{1680}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]+",
        logos::skip
    )]
    #[regex(r";")]
    Semicolon,
    Whitespace,
    /// Matches both single-line and multi-line comments
    #[regex(r"//[^\n\r]*",logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/",logos::skip)]
    Comment,
    Eof,
}


// src/tokens/token_kind.rs
impl TokenKind {
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            TokenKind::TypeI8 |
            TokenKind::TypeI16 |
            TokenKind::TypeI32 |
            TokenKind::TypeI64 |
            TokenKind::TypeU8 |
            TokenKind::TypeU16 |
            TokenKind::TypeU32 |
            TokenKind::TypeU64 |
            TokenKind::TypeF32 |
            TokenKind::TypeF64 |
            TokenKind::TypeChar |
            TokenKind::TypeString |
            TokenKind::TypeBool
        )
    }
}