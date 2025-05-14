use logos::Logos;
use crate::tokens::number::Number;

fn parse_number(lex: &mut logos::Lexer<TokenKind>) -> Option<Number> {
    let slice = lex.slice();

    if let Some(num) = parse_scientific(slice) {
        return Some(num);
    }

    if let Some(num) = parse_integer(slice) {
        return Some(num);
    }

    if let Some(num) = parse_float(slice) {
        return Some(num);
    }
    None
}

fn parse_scientific(slice: &str) -> Option<Number> {
    let pos = slice.find(['e', 'E'])?;
    let (base_str, exp_str) = slice.split_at(pos);
    let base = base_str.parse::<f64>().ok()?;
    let exp = exp_str[1..].parse::<i32>().ok()?; // Skip 'e' or 'E'
    Some(Number::Scientific(base, exp))
}

fn parse_integer(slice: &str) -> Option<Number> {
    slice.parse::<i64>().ok().map(Number::Integer)
}

fn parse_float(slice: &str) -> Option<Number> {
    slice.parse::<f64>().ok().map(Number::Float)
}

fn parse_binary(lex: &mut logos::Lexer<TokenKind>) -> Option<i64> {
    let num_str = &lex.slice()[2..]; // Skip "#b"
    i64::from_str_radix(num_str, 2).ok()
}

fn parse_octal(lex: &mut logos::Lexer<TokenKind>) -> Option<i64> {
    let num_str = &lex.slice()[2..]; // Skip "#o"
    i64::from_str_radix(num_str, 8).ok()
}

fn parse_hex(lex: &mut logos::Lexer<TokenKind>) -> Option<i64> {
    let num_str = &lex.slice()[2..]; // Skip "#x" prefix
    i64::from_str_radix(num_str, 16).ok()
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

    #[regex(r"(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?", parse_number, priority = 4)]
    Number(Number),

    #[regex(r"#b[01]+", parse_binary, priority = 3)]
    Binary(i64),

    #[regex(r"#o[0-7]+", parse_octal, priority = 3)]
    Octal(i64),

    // Hexadecimal numbers (medium priority)
    #[regex(r"#x[0-9a-fA-F]+", parse_hex, priority = 2)]
    Hexadecimal(i64),

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
        r"[ \t\n\f\u{00A0}\u{1680}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]+",
        logos::skip
    )]
    Whitespace,
    Eof,
}