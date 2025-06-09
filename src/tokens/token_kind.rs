// src/tokens/token_kind.rs
use crate::tokens::number::Number;
use logos::Logos;
use std::fmt;

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
pub fn split_numeric_and_suffix(slice: &str) -> (&str, Option<String>) {
    if slice.is_empty() {
        return (slice, None);
    }
    let lowered = slice.to_ascii_lowercase();
    // Supported suffixes, longest first:
    const SUFFIXES: [&str; 9] = [
        "i16", "i32", "u16", "u32", // length == 3
        "i8", "u8", // length == 2
        "u", "f", "d", // length == 1
    ];
    for &suf in SUFFIXES.iter() {
        if lowered.ends_with(suf) {
            let cut_pos = slice.len() - suf.len();
            let numeric_part = &slice[..cut_pos];
            let suffix_part = slice[cut_pos..].to_ascii_lowercase();
            return (numeric_part, Some(suffix_part));
        }
    }
    (slice, None)
}

fn parse_integer<T>(numeric_part: &str, map_fn: fn(T) -> Number) -> Option<Number>
where
    T: std::str::FromStr,
{
    if is_valid_integer_literal(numeric_part) {
        numeric_part.parse::<T>().ok().map(map_fn)
    } else {
        None
    }
}

/// Routes numeric literal parsing based on suffix type.
///
/// # Arguments
/// * `numeric_part` - Numeric portion without suffix
/// * `suffix` - Optional suffix indicating type
///
/// # Returns
/// Parsed [`Number`] variant matching suffix, or `None` for invalid formats
pub fn handle_suffix(numeric_part: &str, suffix: Option<String>) -> Option<Number> {
    match suffix.as_deref() {
        // Unsigned‐integer suffixes
        Some("u") => parse_integer::<u64>(numeric_part, Number::UnsignedInteger),
        Some("u8") => parse_integer::<u8>(numeric_part, Number::U8),
        Some("u16") => parse_integer::<u16>(numeric_part, Number::U16),
        Some("u32") => parse_integer::<u32>(numeric_part, Number::U32),

        // Signed‐integer suffixes
        Some("i8") => parse_integer::<i8>(numeric_part, Number::I8),
        Some("i16") => parse_integer::<i16>(numeric_part, Number::I16),
        Some("i32") => parse_integer::<i32>(numeric_part, Number::I32),

        // Float32 suffix
        Some("f") => handle_float_suffix(numeric_part),

        // Double (f64) o nessun suffisso
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
    parse_scientific(numeric_part, true)
        .or_else(|| numeric_part.parse::<f32>().ok().map(Number::Float32))
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
pub fn parse_base_number(radix: u32, lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (_, num_part) = slice.split_at(2); // Remove prefix ("#b", "#o", or "#x")
    let (num_str, suffix_u) = match num_part.chars().last() {
        Some('u') | Some('U') => (&num_part[..num_part.len() - 1], true),
        _ => (num_part, false),
    };

    if suffix_u {
        u64::from_str_radix(num_str, radix)
            .ok()
            .map(Number::UnsignedInteger)
    } else {
        i64::from_str_radix(num_str, radix)
            .ok()
            .map(Number::Integer)
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
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    IdentifierAscii(String),

    /// Unicode identifiers (supports international characters)
    #[regex(r"[\p{Letter}\p{Mark}_][\p{Letter}\p{Mark}\p{Number}_]*", |lex| lex.slice().to_string(), priority = 1
    )]
    IdentifierUnicode(String),

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
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLiteral(String),

    /// Character literals (captures content without quotes)
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
    #[regex(
        r"[ \t\r\n\f\u{00A0}\u{1680}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]+",
        logos::skip
    )]
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
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Star => write!(f, "'*'"),
            TokenKind::Slash => write!(f, "'/'"),
            TokenKind::PlusEqual => write!(f, "'+='"),
            TokenKind::MinusEqual => write!(f, "'-='"),
            TokenKind::EqualEqual => write!(f, "'=='"),
            TokenKind::NotEqual => write!(f, "'!='"),
            TokenKind::Less => write!(f, "'<'"),
            TokenKind::Greater => write!(f, "'>'"),
            TokenKind::LessEqual => write!(f, "'<='"),
            TokenKind::GreaterEqual => write!(f, "'>='"),
            TokenKind::PlusPlus => write!(f, "'++'"),
            TokenKind::MinusMinus => write!(f, "'--'"),
            TokenKind::OrOr => write!(f, "'||'"),
            TokenKind::AndAnd => write!(f, "'&&'"),
            TokenKind::ShiftLeft => write!(f, "'<<'"),
            TokenKind::ShiftRight => write!(f, "'>>'"),
            TokenKind::PercentEqual => write!(f, "'%='"),
            TokenKind::XorEqual => write!(f, "'^='"),
            TokenKind::Not => write!(f, "'!'"),
            TokenKind::Xor => write!(f, "'^'"),
            TokenKind::Percent => write!(f, "'%'"),
            TokenKind::Or => write!(f, "'|'"),
            TokenKind::And => write!(f, "'&'"),
            TokenKind::Equal => write!(f, "'='"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Dot => write!(f, "'.'"),
            TokenKind::Semicolon => write!(f, "';'"),

            // Keywords
            TokenKind::KeywordFun => write!(f, "'fun'"),
            TokenKind::KeywordIf => write!(f, "'if'"),
            TokenKind::KeywordElse => write!(f, "'else'"),
            TokenKind::KeywordReturn => write!(f, "'return'"),
            TokenKind::KeywordWhile => write!(f, "'while'"),
            TokenKind::KeywordFor => write!(f, "'for'"),
            TokenKind::KeywordMain => write!(f, "'main'"),
            TokenKind::KeywordVar => write!(f, "'var'"),
            TokenKind::KeywordConst => write!(f, "'const'"),
            TokenKind::KeywordNullptr => write!(f, "'nullptr'"),
            TokenKind::KeywordBreak => write!(f, "'break'"),
            TokenKind::KeywordContinue => write!(f, "'continue'"),
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
            TokenKind::OpenParen => write!(f, "'('"),
            TokenKind::CloseParen => write!(f, "')'"),
            TokenKind::OpenBracket => write!(f, "'['"),
            TokenKind::CloseBracket => write!(f, "']'"),
            TokenKind::OpenBrace => write!(f, "'{{'"),
            TokenKind::CloseBrace => write!(f, "'}}'"),

            // Types
            TokenKind::TypeI8 => write!(f, "'i8'"),
            TokenKind::TypeI16 => write!(f, "'i16'"),
            TokenKind::TypeI32 => write!(f, "'i32'"),
            TokenKind::TypeI64 => write!(f, "'i64'"),
            TokenKind::TypeU8 => write!(f, "'u8'"),
            TokenKind::TypeU16 => write!(f, "'u16'"),
            TokenKind::TypeU32 => write!(f, "'u32'"),
            TokenKind::TypeU64 => write!(f, "'u64'"),
            TokenKind::TypeF32 => write!(f, "'f32'"),
            TokenKind::TypeF64 => write!(f, "'f64'"),
            TokenKind::TypeChar => write!(f, "'char'"),
            TokenKind::TypeString => write!(f, "'string'"),
            TokenKind::TypeBool => write!(f, "'bool'"),

            // Special tokens
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Eof => write!(f, "end of file"),
        }
    }
}
