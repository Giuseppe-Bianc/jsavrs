use crate::location::source_span::SourceSpan;
use crate::tokens::token_kind::TokenKind;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
}
