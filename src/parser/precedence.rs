use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;

#[must_use]
pub const fn binding_power(token: &Token) -> (u8, u8) {
    match token.kind {
        // Assignment operators (right-associative)
        TokenKind::Equal
        | TokenKind::PlusEqual
        | TokenKind::MinusEqual
        | TokenKind::PercentEqual
        | TokenKind::XorEqual => (2, 1),

        // Logical OR (left-associative)
        TokenKind::OrOr => (4, 3),

        // Logical AND (left-associative)
        TokenKind::AndAnd => (6, 5),

        // Equality (left-associative)
        TokenKind::EqualEqual | TokenKind::NotEqual => (8, 7),

        // Comparison (left-associative)
        TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual => (10, 9),

        // Bitwise OR (left-associative)
        TokenKind::Or => (12, 11),

        // Bitwise XOR (left-associative)
        TokenKind::Xor => (14, 13),

        // Bitwise AND (left-associative)
        TokenKind::And => (16, 15),

        // Shift (left-associative)
        TokenKind::ShiftLeft | TokenKind::ShiftRight => (18, 17),

        // Add/Subtract (left-associative)
        TokenKind::Plus | TokenKind::Minus => (20, 19),

        // Multiply/Divide/Mod (left-associative)
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => (22, 21),

        // Function call, array access, member access
        TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::Dot => (27, 26),

        _ => (0, 0),
    }
}

#[must_use]
pub const fn unary_binding_power(token: &Token) -> (u8, u8) {
    match token.kind {
        TokenKind::Not | TokenKind::Minus => (24, 23),
        _ => (0, 0),
    }
}
