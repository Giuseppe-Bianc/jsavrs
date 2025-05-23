use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
use crate::parser::ast::*;
use crate::parser::precedence::*;
use crate::tokens::number::Number;
use crate::tokens::token::Token;
use crate::tokens::token_kind::TokenKind;

pub struct JsavParser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<CompileError>,
}

impl JsavParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> (Option<Expr>, Vec<CompileError>) {
        let expr = self.parse_expr(0);
        (expr, self.errors)
    }

    fn parse_expr(&mut self, min_bp: u8) -> Option<Expr> {
        let mut left = self.nud()?;

        while let Some(token) = self.peek() {
            let (lbp, _) = binding_power(token);
            if lbp <= min_bp {
                break;
            }

            left = self.led(left);
        }

        Some(left)
    }

    fn nud(&mut self) -> Option<Expr> {
        let token = self.advance()?.clone();
        match token.kind {
            // Literals
            TokenKind::Numeric(n) => self.new_number_literal(n, token.span),
            TokenKind::KeywordBool(b) => self.new_bool_literal(b, token.span),
            TokenKind::KeywordNullptr => self.new_nullptr_literal(token.span),
            TokenKind::StringLiteral(s) => self.new_string_literal(s, token.span),
            TokenKind::CharLiteral(c) => self.new_char_literal(c, token.span),

            // Unary operators
            TokenKind::Minus => Some(self.parse_unary(UnaryOp::Negate, token)),
            TokenKind::Not => Some(self.parse_unary(UnaryOp::Not, token)),

            // Grouping
            TokenKind::OpenParen => self.parse_grouping(token),

            // Variables
            TokenKind::IdentifierAscii(name) | TokenKind::IdentifierUnicode(name) => {
                Some(Expr::Variable {
                    name,
                    span: token.span,
                })
            }

            _ => {
                self.syntax_error("Unexpected token", &token);
                None
            }
        }
    }

    fn led(&mut self, left: Expr) -> Expr {
        let token = match self.advance() {
            Some(t) => t.clone(),
            None => return left, // Early return if no token
        };

        match token.kind {
            // Binary operators
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::EqualEqual
            | TokenKind::NotEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::AndAnd
            | TokenKind::OrOr
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Xor
            | TokenKind::ShiftLeft
            | TokenKind::ShiftRight => self.parse_binary(left, token),

            // Assignment
            TokenKind::Equal => self.parse_assignment(left, token),
            // Function call
            TokenKind::OpenParen => self.parse_call(left, token),
            // Array access
            TokenKind::OpenBracket => self.parse_array_access(left, token),

            _ => {
                self.syntax_error("Unexpected operator", &token);
                left
            }
        }
    }

    // Helper methods for literals
    fn new_number_literal(&self, value: Number, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::Number(value),
            span,
        })
    }

    fn new_bool_literal(&self, value: bool, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::Bool(value),
            span,
        })
    }

    fn new_nullptr_literal(&self, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::Nullptr,
            span,
        })
    }

    fn new_string_literal(&self, value: String, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::StringLit(value),
            span,
        })
    }

    fn new_char_literal(&self, value: String, span: SourceSpan) -> Option<Expr> {
        Some(Expr::Literal {
            value: LiteralValue::CharLit(value),
            span,
        })
    }

    // Parsing operations
    fn parse_unary(&mut self, op: UnaryOp, token: Token) -> Expr {
        let (_, rbp) = unary_binding_power(&token);
        let expr = self.parse_expr(rbp).unwrap_or_else(|| self.null_expr(token.span.clone()));
        Expr::Unary {
            op,
            expr: Box::new(expr),
            span: token.span,
        }
    }

    fn parse_binary(&mut self, left: Expr, token: Token) -> Expr {
        let op = match BinaryOp::get_op(&token) {
            Ok(op) => op,
            Err(e) => {
                self.errors.push(e);
                return left;
            }
        };

        let right = self.parse_expr(binding_power(&token).1)
            .unwrap_or_else(|| self.null_expr(token.span.clone()));

        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
            span: token.span,
        }
    }

    fn parse_grouping(&mut self, start_token: Token) -> Option<Expr> {
        let expr = self.parse_expr(0);
        self.expect(TokenKind::CloseParen, "Unclosed parenthesis");
        Some(Expr::Grouping {
            expr: Box::new(expr?),
            span: self.merged_span(&start_token),
        })
    }

    fn parse_assignment(&mut self, left: Expr, token: Token) -> Expr {
        match left {
            Expr::Variable { name, span } => {
                let value = self.parse_expr(1)
                    .unwrap_or_else(|| self.null_expr(token.span.clone()));
                Expr::Assign {
                    name,
                    value: Box::new(value),
                    span: span.merged(&token.span).unwrap_or(span),
                }
            }
            _ => {
                self.syntax_error("Invalid assignment target", &token);
                left
            }
        }
    }

    fn parse_call(&mut self, callee: Expr, start_token: Token) -> Expr {
        let mut arguments = Vec::new();
        while !self.check(TokenKind::CloseParen) && !self.is_at_end() {
            if let Some(arg) = self.parse_expr(0) {
                arguments.push(arg);
            }
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::CloseParen, "Unclosed function call");
        Expr::Call {
            callee: Box::new(callee),
            arguments,
            span: self.merged_span(&start_token),
        }
    }

    fn parse_array_access(&mut self, array: Expr, start_token: Token) -> Expr {
        let index = self.parse_expr(0)
            .unwrap_or_else(|| self.null_expr(start_token.span.clone()));
        self.expect(TokenKind::CloseBracket, "Unclosed array access");
        Expr::ArrayAccess {
            array: Box::new(array),
            index: Box::new(index),
            span: self.merged_span(&start_token),
        }
    }

    // Utility methods
    fn merged_span(&self, start_token: &Token) -> SourceSpan {
        self.previous()
            .and_then(|end| start_token.span.merged(&end.span))
            .unwrap_or(start_token.span.clone())
    }

    fn null_expr(&self, span: SourceSpan) -> Expr {
        Expr::Literal {
            value: LiteralValue::Nullptr,
            span,
        }
    }

    fn syntax_error(&mut self, message: &str, token: &Token) {
        self.errors.push(CompileError::SyntaxError {
            message: format!("{}: {:?}", message, token.kind),
            span: token.span.clone(),
        });
    }

    // Token management
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current.saturating_sub(1))
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    fn expect(&mut self, kind: TokenKind, context: &str) {
        if !self.match_token(kind.clone()) {
            let found = self.peek()
                .map(|t| format!("{:?}", t.kind))
                .unwrap_or_else(|| "end of input".to_string());
            self.errors.push(CompileError::SyntaxError {
                message: format!("{}: Expected '{:?}' but found {}", context, kind, found),
                span: self.peek().map(|t| t.span.clone()).unwrap_or_default(),
            });
        }
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().map(|t| t.kind == TokenKind::Eof).unwrap_or(true)
    }
}