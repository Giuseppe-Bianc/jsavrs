// src/tokens/token_kind.rs
use crate::tokens::number::Number;
use logos::Logos;
use std::fmt;
use std::sync::Arc;

/// Parses a numeric literal token into a structured [`Number`] representation.
///
/// Handles:
/// - Integer vs float detection
/// - Scientific notation
/// - Multi-character suffixes (i8, i16, i32, u8, u16, u32, u, f, d)
///
/// # Arguments
/// * `lex` - Lexer context from Logos
///
/// # Returns
/// Parsed [`Number`] or `None` for invalid literals
pub fn parse_number(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (numeric_part, suffix) = split_numeric_and_suffix(slice);
    handle_suffix(numeric_part, suffix)
}

/// Splits a numeric literal string into its numeric part and optional suffix.
///
/// # Arguments
/// * `slice` - Full numeric literal string
///
/// # Returns
/// Tuple containing:
/// - Numeric portion (without suffix)
/// - Optional suffix (normalized to lowercase)
///
/// # Examples
/// ```
/// use jsavrs::tokens::token_kind::split_numeric_and_suffix;
/// assert_eq!(split_numeric_and_suffix("42u"), ("42", Some("u".to_string())));
/// assert_eq!(split_numeric_and_suffix("3.14F"), ("3.14", Some("f".to_string())));
/// assert_eq!(split_numeric_and_suffix("100i16"), ("100", Some("i16".to_string())));
/// assert_eq!(split_numeric_and_suffix("6.022e23u32"), ("6.022e23", Some("u32".to_string())));
/// assert_eq!(split_numeric_and_suffix("100"), ("100", None));
/// ```
pub fn split_numeric_and_suffix(slice: &str) -> (&str, Option<&str>) {
    if slice.is_empty() {
        return (slice, None);
    }

    let bytes = slice.as_bytes();
    let len = bytes.len();
    
    // Fast path: check last character first
    let last_char = bytes[len - 1].to_ascii_lowercase();
    
    // Single-char suffixes: 'u', 'f', 'd'
    match last_char {
        b'u' | b'f' | b'd' => {
            return (&slice[..len - 1], Some(&slice[len - 1..]));
        }
        _ => {}
    }
    
    // Multi-char suffixes: check if we have at least 3 chars
    if len < 3 {
        return (slice, None);
    }
    
    // Check 3-char suffixes (i16, i32, u16, u32)
    if len >= 3 {
        let last_three = &bytes[len - 3..];
        let suffix_lower = [
            last_three[0].to_ascii_lowercase(),
            last_three[1].to_ascii_lowercase(),
            last_three[2].to_ascii_lowercase(),
        ];
        
        match suffix_lower {
            [b'i', b'1', b'6'] | [b'i', b'3', b'2'] |
            [b'u', b'1', b'6'] | [b'u', b'3', b'2'] => {
                return (&slice[..len - 3], Some(&slice[len - 3..]));
            }
            _ => {}
        }
    }
    
    // Check 2-char suffixes (i8, u8)
    if len >= 2 {
        let last_two = &bytes[len - 2..];
        let suffix_lower = [
            last_two[0].to_ascii_lowercase(),
            last_two[1].to_ascii_lowercase(),
        ];
        
        match suffix_lower {
            [b'i', b'8'] | [b'u', b'8'] => {
                return (&slice[..len - 2], Some(&slice[len - 2..]));
            }
            _ => {}
        }
    }
    
    (slice, None)
}

fn parse_integer<T>(numeric_part: &str, map_fn: fn(T) -> Number) -> Option<Number>
where
    T: std::str::FromStr,
{
    if is_valid_integer_literal(numeric_part) { numeric_part.parse::<T>().ok().map(map_fn) } else { None }
}

/// Routes numeric literal parsing based on suffix type.
///
/// # Arguments
/// * `numeric_part` - Numeric portion without suffix
/// * `suffix` - Optional suffix indicating type
///
/// # Returns
/// Parsed [`Number`] variant matching suffix, or `None` for invalid formats
fn handle_suffix(numeric_part: &str, suffix: Option<&str>) -> Option<Number> {
    match suffix.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("u") => parse_integer::<u64>(numeric_part, Number::UnsignedInteger),
        Some("u8") => parse_integer::<u8>(numeric_part, Number::U8),
        Some("u16") => parse_integer::<u16>(numeric_part, Number::U16),
        Some("u32") => parse_integer::<u32>(numeric_part, Number::U32),
        Some("i8") => parse_integer::<i8>(numeric_part, Number::I8),
        Some("i16") => parse_integer::<i16>(numeric_part, Number::I16),
        Some("i32") => parse_integer::<i32>(numeric_part, Number::I32),
        Some("f") => handle_float_suffix(numeric_part),
        Some("d") | None => handle_default_suffix(numeric_part),
        _ => None,
    }
}

/// Helper to check if a string represents a valid pure-integer literal:
/// no decimal point, no 'e'/'E', no sign (we assume negative sign is a separate token).
///
/// # Arguments
/// * `numeric_part` - Numeric string to validate
///
/// # Returns
/// `true` if it contains only digits (0–9)
pub fn is_valid_integer_literal(numeric_part: &str) -> bool {
    if numeric_part.contains('.') || numeric_part.contains('e') || numeric_part.contains('E') {
        return false;
    }
    numeric_part.chars().all(|c| c.is_ascii_digit())
}

/// Parses numeric string with float32 suffix ('f').
///
/// # Arguments
/// * `numeric_part` - Numeric string without suffix
///
/// # Returns
/// [`Number::Float32`] or [`Number::Scientific32`] if valid
pub fn handle_float_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, true).or_else(|| numeric_part.parse::<f32>().ok().map(Number::Float32))
}

/// Parses numeric strings with default suffix (no suffix or 'd').
///
/// # Arguments
/// * `numeric_part` - Numeric string without suffix
///
/// # Returns
/// [`Number::Integer`], [`Number::Float64`], or [`Number::Scientific64`]
pub fn handle_default_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, false).or_else(|| handle_non_scientific(numeric_part))
}

/// Parses non-scientific notation numbers.
///
/// # Arguments
/// * `numeric_part` - Numeric string to parse
///
/// # Returns
/// [`Number::Integer`] if no decimal point, [`Number::Float64`] otherwise
pub fn handle_non_scientific(numeric_part: &str) -> Option<Number> {
    if numeric_part.contains('.') {
        numeric_part.parse::<f64>().ok().map(Number::Float64)
    } else {
        numeric_part.parse::<i64>().ok().map(Number::Integer)
    }
}

/// Parses scientific notation numbers (e.g., "6.022e23").
///
/// # Arguments
/// * `s` - Full numeric string
/// * `is_f32` - Whether to parse as 32-bit float
///
/// # Returns
/// [`Number::Scientific32`] or [`Number::Scientific64`] if valid
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

/// Generic parser for base-specific numbers (binary, octal, hex).
///
/// # Arguments
/// * `radix` - Numeric base (2, 8, or 16)
/// * `lex` - Lexer context
///
/// # Returns
/// Parsed [`Number`] with optional unsigned suffix
#[inline]
pub fn parse_base_number(radix: u32, lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (_, num_part) = slice.split_at(2); // Remove prefix ("#b", "#o", or "#x")
    let (num_str, suffix_u) = match num_part.chars().last() {
        Some('u') | Some('U') => (&num_part[..num_part.len() - 1], true),
        _ => (num_part, false),
    };

    if suffix_u {
        u64::from_str_radix(num_str, radix).ok().map(Number::UnsignedInteger)
    } else {
        i64::from_str_radix(num_str, radix).ok().map(Number::Integer)
    }
}

/// Parses binary literals (e.g., "#b1010u").
pub fn parse_binary(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(2, lex)
}

/// Parses octal literals (e.g., "#o755").
pub fn parse_octal(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(8, lex)
}

/// Parses hexadecimal literals (e.g., "#xdeadbeefu").
pub fn parse_hex(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(16, lex)
}

/// Represents all possible token types in the language.
///
/// Generated by the lexer and consumed by the parser. Variants include:
/// - Operators
/// - Keywords
/// - Identifiers
/// - Literals
/// - Punctuation
/// - Types
///
/// Uses Logos lexer generation with regex patterns and custom parsers.
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

    // Boolean literals (captures value)
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    KeywordBool(bool),

    // Identifiers
    /// ASCII identifiers (letters, digits, underscores)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Arc::from(lex.slice()), priority = 2)]
    IdentifierAscii(Arc<str>),

    /// Unicode identifiers (supports international characters)
    #[regex(
    r"[\p{Letter}\p{Mark}_][\p{Letter}\p{Mark}\p{Number}_]*",
    |lex| Arc::from(lex.slice()),
    priority = 1
    )]
    IdentifierUnicode(Arc<str>),

    /// Numeric literals (supports integer, float, scientific, and multi-char suffixes)
    #[regex(
        r"(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?([uU]|[fF]|[dD]|[iI](8|16|32)|[uU](8|16|32))?",
        parse_number,
        priority = 4
    )]
    Numeric(Number),

    /// Binary literals (e.g., "#b1010u")
    #[regex(r"#b[01]+[uU]?", parse_binary, priority = 3)]
    Binary(Number),

    /// Octal literals (e.g., "#o755")
    #[regex(r"#o[0-7]+[uU]?", parse_octal, priority = 3)]
    Octal(Number),

    /// Hexadecimal literals (e.g., "#xdeadbeefu")
    #[regex(r"#x[0-9a-fA-F]+[uU]?", parse_hex, priority = 2)]
    Hexadecimal(Number),

    /// String literals (captures content without quotes)
    #[regex(r#""([^"\\]|\\.)*""#, |lex| Arc::from(&lex.slice()[1..lex.slice().len()-1]))]
    StringLiteral(Arc<str>),

    /// Character literals (captures content without quotes)
    #[regex(r#"'([^'\\]|\\.)'"#, |lex| {
        let s = lex.slice();
        Arc::from(&s[1..s.len()-1])
    })]
    CharLiteral(Arc<str>),

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

    // Type keywords
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

    // Whitespace and comments (skipped by lexer)
    #[regex(r"[ \t\r\n\f\u{00A0}\u{1680}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]+", logos::skip)]
    #[regex(r";")]
    Semicolon,
    Whitespace,
    /// Matches both single-line and multi-line comments
    #[regex(r"//[^\n\r]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Comment,

    /// End-of-file marker
    Eof,
}

impl TokenKind {
    /// Checks if the token represents a type keyword.
    ///
    /// # Returns
    /// `true` for all type variants (i8, u8, f32, etc.), `false` otherwise
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            TokenKind::TypeI8
                | TokenKind::TypeI16
                | TokenKind::TypeI32
                | TokenKind::TypeI64
                | TokenKind::TypeU8
                | TokenKind::TypeU16
                | TokenKind::TypeU32
                | TokenKind::TypeU64
                | TokenKind::TypeF32
                | TokenKind::TypeF64
                | TokenKind::TypeChar
                | TokenKind::TypeString
                | TokenKind::TypeBool
        )
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Operators
            TokenKind::Plus => f.write_str("'+'"),
            TokenKind::Minus => f.write_str("'-'"),
            TokenKind::Star => f.write_str("'*'"),
            TokenKind::Slash => f.write_str("'/'"),
            TokenKind::PlusEqual => f.write_str("'+='"),
            TokenKind::MinusEqual => f.write_str("'-='"),
            TokenKind::EqualEqual => f.write_str("'=='"),
            TokenKind::NotEqual => f.write_str("'!='"),
            TokenKind::Less => f.write_str("'<'"),
            TokenKind::Greater => f.write_str("'>'"),
            TokenKind::LessEqual => f.write_str("'<='"),
            TokenKind::GreaterEqual => f.write_str("'>='"),
            TokenKind::PlusPlus => f.write_str("'++'"),
            TokenKind::MinusMinus => f.write_str("'--'"),
            TokenKind::OrOr => f.write_str("'||'"),
            TokenKind::AndAnd => f.write_str("'&&'"),
            TokenKind::ShiftLeft => f.write_str("'<<'"),
            TokenKind::ShiftRight => f.write_str("'>>'"),
            TokenKind::PercentEqual => f.write_str("'%='"),
            TokenKind::XorEqual => f.write_str("'^='"),
            TokenKind::Not => f.write_str("'!'"),
            TokenKind::Xor => f.write_str("'^'"),
            TokenKind::Percent => f.write_str("'%'"),
            TokenKind::Or => f.write_str("'|'"),
            TokenKind::And => f.write_str("'&'"),
            TokenKind::Equal => f.write_str("'='"),
            TokenKind::Colon => f.write_str("':'"),
            TokenKind::Comma => f.write_str("','"),
            TokenKind::Dot => f.write_str("'.'"),
            TokenKind::Semicolon => f.write_str("';'"),

            // Keywords
            TokenKind::KeywordFun => f.write_str("'fun'"),
            TokenKind::KeywordIf => f.write_str("'if'"),
            TokenKind::KeywordElse => f.write_str("'else'"),
            TokenKind::KeywordReturn => f.write_str("'return'"),
            TokenKind::KeywordWhile => f.write_str("'while'"),
            TokenKind::KeywordFor => f.write_str("'for'"),
            TokenKind::KeywordMain => f.write_str("'main'"),
            TokenKind::KeywordVar => f.write_str("'var'"),
            TokenKind::KeywordConst => f.write_str("'const'"),
            TokenKind::KeywordNullptr => f.write_str("'nullptr'"),
            TokenKind::KeywordBreak => f.write_str("'break'"),
            TokenKind::KeywordContinue => f.write_str("'continue'"),
            TokenKind::KeywordBool(b) => write!(f, "boolean '{b}'"),

            // Identifiers
            TokenKind::IdentifierAscii(s) | TokenKind::IdentifierUnicode(s) => {
                write!(f, "identifier '{s}'")
            }

            // Numeric literals
            TokenKind::Numeric(n) => write!(f, "number '{n}'"),
            TokenKind::Binary(n) => write!(f, "binary '{n}'"),
            TokenKind::Octal(n) => write!(f, "octal '{n}'"),
            TokenKind::Hexadecimal(n) => write!(f, "hexadecimal '{n}'"),

            // String/char literals
            TokenKind::StringLiteral(s) => write!(f, "string literal \"{s}\""),
            TokenKind::CharLiteral(c) => write!(f, "character literal '{c}'"),

            // Brackets
            TokenKind::OpenParen => f.write_str("'('"),
            TokenKind::CloseParen => f.write_str("')'"),
            TokenKind::OpenBracket => f.write_str("'['"),
            TokenKind::CloseBracket => f.write_str("']'"),
            TokenKind::OpenBrace => f.write_str("'{'"),
            TokenKind::CloseBrace => f.write_str("'}'"),
            TokenKind::TypeI8 => f.write_str("'i8'"),
            TokenKind::TypeI16 => f.write_str("'i16'"),
            TokenKind::TypeI32 => f.write_str("'i32'"),
            TokenKind::TypeI64 => f.write_str("'i64'"),
            TokenKind::TypeU8 => f.write_str("'u8'"),
            TokenKind::TypeU16 => f.write_str("'u16'"),
            TokenKind::TypeU32 => f.write_str("'u32'"),
            TokenKind::TypeU64 => f.write_str("'u64'"),
            TokenKind::TypeF32 => f.write_str("'f32'"),
            TokenKind::TypeF64 => f.write_str("'f64'"),
            TokenKind::TypeChar => f.write_str("'char'"),
            TokenKind::TypeString => f.write_str("'string'"),
            TokenKind::TypeBool => f.write_str("'bool'"),

            // Special tokens
            TokenKind::Whitespace => f.write_str("whitespace"),
            TokenKind::Comment => f.write_str("comment"),
            TokenKind::Eof => f.write_str("end of file"),
        }
    }
}
