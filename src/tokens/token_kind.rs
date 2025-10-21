// src/tokens/token_kind.rs
use crate::tokens::number::Number;
use logos::Logos;
use std::fmt;
use std::sync::Arc;

/// Parses a numeric literal token into a structured [`Number`] representation.
///
/// This function is called by the Logos lexer when it encounters a numeric literal.
/// It handles all numeric formats including:
/// - Pure integers (e.g., `42`)
/// - Floating-point numbers (e.g., `3.14`)
/// - Scientific notation (e.g., `6.022e23`)
/// - Type suffixes (e.g., `42u8`, `3.14f`, `100i16`)
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context, providing access to
///           the matched slice via `lex.slice()`
///
/// # Returns
///
/// * `Some(Number)` - Successfully parsed numeric literal
/// * `None` - Invalid numeric format or overflow/underflow
///
/// # Implementation Strategy
///
/// 1. Extract the full matched slice from the lexer
/// 2. Split into numeric part and optional type suffix
/// 3. Route to appropriate parser based on suffix type
///
/// # Examples
///
/// ```ignore
/// // Called internally by Logos for patterns like:
/// // "42"      -> Some(Number::Integer(42))
/// // "3.14f"   -> Some(Number::Float32(3.14))
/// // "100u16"  -> Some(Number::U16(100))
/// // "6.022e23" -> Some(Number::Scientific64(6.022, 23))
/// ```
pub fn parse_number(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();
    let (numeric_part, suffix) = split_numeric_and_suffix(slice);
    handle_suffix(numeric_part, suffix)
}

/// Splits a numeric literal string into its numeric part and optional type suffix.
///
/// This function performs efficient suffix detection using byte-level pattern matching
/// with optimized fast paths for common cases.
///
/// # Supported Suffixes
///
/// - Single-character: `u`, `f`, `d`
/// - Two-character: `i8`, `u8`
/// - Three-character: `i16`, `i32`, `u16`, `u32`
///
/// # Arguments
///
/// * `slice` - Full numeric literal string including any suffix
///
/// # Returns
///
/// A tuple containing:
/// - Numeric portion (without suffix)
/// - Optional suffix (original case preserved, but matching is case-insensitive)
///
/// # Performance Notes
///
/// Uses byte-level operations and fast-path checking for optimal performance:
/// 1. First checks the last character for single-char suffixes
/// 2. Then checks for 3-char suffixes if string is long enough
/// 3. Finally checks for 2-char suffixes
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::token_kind::split_numeric_and_suffix;
///
/// assert_eq!(split_numeric_and_suffix("42u"), ("42", Some("u")));
/// assert_eq!(split_numeric_and_suffix("3.14F"), ("3.14", Some("F")));
/// assert_eq!(split_numeric_and_suffix("100i16"), ("100", Some("i16")));
/// assert_eq!(split_numeric_and_suffix("6.022e23u32"), ("6.022e23", Some("u32")));
/// assert_eq!(split_numeric_and_suffix("100"), ("100", None));
/// ```
pub fn split_numeric_and_suffix(slice: &str) -> (&str, Option<&str>) {
    if slice.is_empty() {
        return (slice, None);
    }

    let bytes = slice.as_bytes();
    let len = bytes.len();

    // Fast path: check last character first for single-char suffixes
    let last_char = bytes[len - 1].to_ascii_lowercase();

    // Single-char suffixes: 'u' (unsigned), 'f' (float32), 'd' (float64)
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
            [b'i', b'1', b'6'] | [b'i', b'3', b'2'] | [b'u', b'1', b'6'] | [b'u', b'3', b'2'] => {
                return (&slice[..len - 3], Some(&slice[len - 3..]));
            }
            _ => {}
        }
    }

    // Check 2-char suffixes (i8, u8)
    if len >= 2 {
        let last_two = &bytes[len - 2..];
        let suffix_lower = [last_two[0].to_ascii_lowercase(), last_two[1].to_ascii_lowercase()];

        match suffix_lower {
            [b'i', b'8'] | [b'u', b'8'] => {
                return (&slice[..len - 2], Some(&slice[len - 2..]));
            }
            _ => {}
        }
    }

    (slice, None)
}

/// Helper function to parse integer literals with generic type support.
///
/// This function validates that the numeric string represents a valid integer
/// (no decimal point or exponent), then attempts to parse it into the target type.
///
/// # Type Parameters
///
/// * `T` - The target integer type (must implement `FromStr`)
///
/// # Arguments
///
/// * `numeric_part` - The numeric string without suffix
/// * `map_fn` - Function to wrap the parsed value in a [`Number`] variant
///
/// # Returns
///
/// * `Some(Number)` - Successfully parsed and wrapped integer
/// * `None` - Invalid format or value out of range for type `T`
///
/// # Examples
///
/// ```ignore
/// parse_integer::<i8>("42", Number::I8)     // Some(Number::I8(42))
/// parse_integer::<i8>("999", Number::I8)    // None (overflow)
/// parse_integer::<u64>("42", Number::UnsignedInteger)  // Some(Number::UnsignedInteger(42))
/// ```
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

/// Routes numeric literal parsing based on type suffix.
///
/// This function dispatches to the appropriate parser based on the suffix,
/// implementing the language's type inference rules:
/// - No suffix: defaults to i64 for integers, f64 for floats
/// - 'u': unsigned 64-bit integer
/// - 'f': 32-bit float
/// - 'd': 64-bit float (explicit)
/// - Sized suffixes (i8, u32, etc.): specific type
///
/// # Arguments
///
/// * `numeric_part` - Numeric portion without suffix
/// * `suffix` - Optional type suffix (case-insensitive)
///
/// # Returns
///
/// * `Some(Number)` - Parsed number matching the suffix type
/// * `None` - Invalid format or unsupported suffix
///
/// # Type Resolution Table
///
/// | Suffix | Type | Example |
/// |--------|------|---------|
/// | None | i64/f64 | `42` → Integer(42), `3.14` → Float64(3.14) |
/// | `u` | u64 | `42u` → UnsignedInteger(42) |
/// | `i8` | i8 | `42i8` → I8(42) |
/// | `u16` | u16 | `1000u16` → U16(1000) |
/// | `f` | f32 | `3.14f` → Float32(3.14) |
/// | `d` | f64 | `3.14d` → Float64(3.14) |
///
/// # Examples
///
/// ```ignore
/// handle_suffix("42", Some("u"))     // Some(Number::UnsignedInteger(42))
/// handle_suffix("42", Some("i8"))    // Some(Number::I8(42))
/// handle_suffix("3.14", Some("f"))   // Some(Number::Float32(3.14))
/// handle_suffix("42", None)          // Some(Number::Integer(42))
/// ```
pub fn handle_suffix(numeric_part: &str, suffix: Option<&str>) -> Option<Number> {
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
        _ => None, // Unknown suffix
    }
}

/// Validates that a string represents a pure integer literal.
///
/// A valid integer literal must:
/// - Contain only ASCII digits (0-9)
/// - Have no decimal point (`.`)
/// - Have no exponent marker (`e` or `E`)
/// - Have no sign character (handled as separate token by lexer)
///
/// # Arguments
///
/// * `numeric_part` - Numeric string to validate
///
/// # Returns
///
/// `true` if the string is a valid integer literal, `false` otherwise
///
/// # Examples
///
/// ```
/// use jsavrs::tokens::token_kind::is_valid_integer_literal;
///
/// assert!(is_valid_integer_literal("42"));
/// assert!(is_valid_integer_literal("1234567890"));
/// assert!(!is_valid_integer_literal("3.14"));      // Has decimal point
/// assert!(!is_valid_integer_literal("6.022e23"));  // Has exponent
/// assert!(!is_valid_integer_literal("-42"));       // Has sign
/// ```
pub fn is_valid_integer_literal(numeric_part: &str) -> bool {
    if numeric_part.contains('.') || numeric_part.contains('e') || numeric_part.contains('E') {
        return false;
    }
    numeric_part.chars().all(|c| c.is_ascii_digit())
}

/// Parses numeric strings with 32-bit float suffix ('f').
///
/// Handles both regular floating-point notation and scientific notation,
/// producing either [`Number::Float32`] or [`Number::Scientific32`] respectively.
///
/// # Arguments
///
/// * `numeric_part` - Numeric string without the 'f' suffix
///
/// # Returns
///
/// * `Some(Number::Float32)` - For regular float literals
/// * `Some(Number::Scientific32)` - For scientific notation
/// * `None` - If parsing fails
///
/// # Examples
///
/// ```ignore
/// handle_float_suffix("3.14")      // Some(Number::Float32(3.14))
/// handle_float_suffix("6.022e23")  // Some(Number::Scientific32(6.022, 23))
/// handle_float_suffix("invalid")   // None
/// ```
pub fn handle_float_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, true)
        .or_else(|| numeric_part.parse::<f32>().ok().map(Number::Float32))
}

/// Parses numeric strings with default or 'd' suffix.
///
/// Implements the default type inference rules:
/// - Integer literals (no decimal/exponent) → i64
/// - Floating-point literals → f64
/// - Scientific notation → Scientific64
///
/// # Arguments
///
/// * `numeric_part` - Numeric string without suffix (or with 'd' suffix removed)
///
/// # Returns
///
/// * `Some(Number::Integer)` - For integer literals
/// * `Some(Number::Float64)` - For floating-point literals
/// * `Some(Number::Scientific64)` - For scientific notation
/// * `None` - If parsing fails
///
/// # Examples
///
/// ```ignore
/// handle_default_suffix("42")        // Some(Number::Integer(42))
/// handle_default_suffix("3.14")      // Some(Number::Float64(3.14))
/// handle_default_suffix("6.022e23")  // Some(Number::Scientific64(6.022, 23))
/// ```
pub fn handle_default_suffix(numeric_part: &str) -> Option<Number> {
    parse_scientific(numeric_part, false)
        .or_else(|| handle_non_scientific(numeric_part))
}

/// Parses non-scientific notation numbers (integers and simple floats).
///
/// Determines the appropriate type based on the presence of a decimal point:
/// - No decimal point → i64 integer
/// - Has decimal point → f64 float
///
/// # Arguments
///
/// * `numeric_part` - Numeric string to parse
///
/// # Returns
///
/// * `Some(Number::Integer)` - For literals without decimal point
/// * `Some(Number::Float64)` - For literals with decimal point
/// * `None` - If parsing fails (overflow, underflow, or invalid format)
///
/// # Examples
///
/// ```ignore
/// handle_non_scientific("42")     // Some(Number::Integer(42))
/// handle_non_scientific("3.14")   // Some(Number::Float64(3.14))
/// handle_non_scientific(".5")     // Some(Number::Float64(0.5))
/// ```
pub fn handle_non_scientific(numeric_part: &str) -> Option<Number> {
    if numeric_part.contains('.') {
        numeric_part.parse::<f64>().ok().map(Number::Float64)
    } else {
        numeric_part.parse::<i64>().ok().map(Number::Integer)
    }
}

/// Parses scientific notation numbers (e.g., "6.022e23").
///
/// Scientific notation format: `base[e|E][+|-]exponent`
/// where the base can be an integer or floating-point number.
///
/// # Arguments
///
/// * `s` - Full numeric string in scientific notation
/// * `is_f32` - If `true`, parses as 32-bit float; if `false`, as 64-bit float
///
/// # Returns
///
/// * `Some(Number::Scientific32)` - For 32-bit scientific notation
/// * `Some(Number::Scientific64)` - For 64-bit scientific notation
/// * `None` - If not in scientific notation or parsing fails
///
/// # Format Details
///
/// - Exponent marker: `e` or `E` (case-insensitive)
/// - Optional sign: `+` or `-` before exponent
/// - Base: can be integer or floating-point
/// - Exponent: must be valid i32 integer
///
/// # Examples
///
/// ```ignore
/// parse_scientific("6.022e23", false)   // Some(Number::Scientific64(6.022, 23))
/// parse_scientific("1.5e-10", true)     // Some(Number::Scientific32(1.5, -10))
/// parse_scientific("3E+8", false)       // Some(Number::Scientific64(3.0, 8))
/// parse_scientific("42", false)         // None (no exponent marker)
/// ```
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

/// Generic parser for base-specific number literals (binary, octal, hexadecimal).
///
/// Handles literals with prefixes:
/// - Binary: `#b` (e.g., `#b1010`)
/// - Octal: `#o` (e.g., `#o755`)
/// - Hexadecimal: `#x` (e.g., `#xDEADBEEF`)
///
/// Supports optional unsigned suffix (`u` or `U`).
///
/// # Arguments
///
/// * `radix` - Numeric base (2 for binary, 8 for octal, 16 for hexadecimal)
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed literals (no 'u' suffix)
/// * `Some(Number::UnsignedInteger)` - For unsigned literals (with 'u' suffix)
/// * `None` - If parsing fails or contains invalid digits for the radix
///
/// # Implementation Notes
///
/// - Strips the 2-character prefix (`#b`, `#o`, or `#x`)
/// - Checks for optional trailing `u` or `U` suffix
/// - Uses `i64::from_str_radix` or `u64::from_str_radix` for parsing
///
/// # Examples
///
/// ```ignore
/// // Binary literals
/// parse_base_number(2, lex_for("#b1010"))    // Some(Number::Integer(10))
/// parse_base_number(2, lex_for("#b1111u"))   // Some(Number::UnsignedInteger(15))
///
/// // Octal literals
/// parse_base_number(8, lex_for("#o755"))     // Some(Number::Integer(493))
///
/// // Hexadecimal literals
/// parse_base_number(16, lex_for("#xDEAD"))   // Some(Number::Integer(57005))
/// ```
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

/// Parses binary literals prefixed with `#b`.
///
/// Binary literals use base-2 representation with digits 0 and 1.
/// Supports optional unsigned suffix.
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed binary literals
/// * `Some(Number::UnsignedInteger)` - For unsigned binary literals (with 'u' suffix)
/// * `None` - If parsing fails or contains non-binary digits
///
/// # Format
///
/// - Prefix: `#b` (required)
/// - Digits: `0`, `1` only
/// - Suffix: `u` or `U` (optional, for unsigned)
///
/// # Examples
///
/// ```ignore
/// // In source code:
/// #b1010    // → Number::Integer(10)
/// #b1111u   // → Number::UnsignedInteger(15)
/// #b0       // → Number::Integer(0)
/// ```
pub fn parse_binary(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(2, lex)
}

/// Parses octal literals prefixed with `#o`.
///
/// Octal literals use base-8 representation with digits 0-7.
/// Supports optional unsigned suffix.
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed octal literals
/// * `Some(Number::UnsignedInteger)` - For unsigned octal literals (with 'u' suffix)
/// * `None` - If parsing fails or contains non-octal digits
///
/// # Format
///
/// - Prefix: `#o` (required)
/// - Digits: `0-7` only
/// - Suffix: `u` or `U` (optional, for unsigned)
///
/// # Examples
///
/// ```ignore
/// // In source code:
/// #o755     // → Number::Integer(493)  (Unix file permissions)
/// #o77u     // → Number::UnsignedInteger(63)
/// #o10      // → Number::Integer(8)
/// ```
pub fn parse_octal(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    parse_base_number(8, lex)
}

/// Parses hexadecimal literals prefixed with `#x`.
///
/// Hexadecimal literals use base-16 representation with digits 0-9 and A-F (case-insensitive).
/// Supports optional unsigned suffix.
///
/// # Arguments
///
/// * `lex` - Mutable reference to the Logos lexer context
///
/// # Returns
///
/// * `Some(Number::Integer)` - For signed hexadecimal literals
/// * `Some(Number::UnsignedInteger)` - For unsigned hexadecimal literals (with 'u' suffix)
/// * `None` - If parsing fails or contains non-hexadecimal digits
///
/// # Format
///
/// - Prefix: `#x` (required)
/// - Digits: `0-9`, `A-F`, `a-f`
/// - Suffix: `u` or `U` (optional, for unsigned)
///
/// # Examples
///
/// ```ignore
/// // In source code:
/// #xDEADBEEF    // → Number::Integer(3735928559)
/// #xFFu         // → Number::UnsignedInteger(255)
/// #x1A2B        // → Number::Integer(6699)
/// ```
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
    #[regex(r"(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?([uUfFdD]|[iIuU](?:8|16|32))?", parse_number, priority = 4)]
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
    #[regex(r"\p{White_Space}+", logos::skip)]
    #[regex(r";")]
    Semicolon,
    Whitespace,
    /// Matches both single-line and multi-line comments
    #[regex(r"//[^\n\r]*", logos::skip)]
    #[regex(r"/\*[^*]*\*+(?:[^*/][^*]*\*+)*/", logos::skip)]
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
