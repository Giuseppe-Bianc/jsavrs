use crate::location::source_span::SourceSpan;
use crate::tokens::token_kind::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
}
