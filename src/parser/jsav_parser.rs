use crate::error::compile_error::CompileError;
use crate::parser::ast::*;
use crate::parser::precedence::*;
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
        let token_kind = token.kind.clone();
        let token_span = token.span.clone();

        match &token_kind {
            // Literals
            TokenKind::Numeric(n) => Some(Expr::Literal {
                value: LiteralValue::Number(n.clone()),
                span: token_span,
            }),
            TokenKind::KeywordBool(b) => Some(Expr::Literal {
                value: LiteralValue::Bool(*b),
                span: token_span,
            }),
            TokenKind::KeywordNullptr => Some(Expr::Literal {
                value: LiteralValue::Nullptr,
                span: token_span,
            }),
            TokenKind::StringLiteral(s) => Some(Expr::Literal {
                value: LiteralValue::StringLit(s.clone()),
                span: token_span,
            }),
            TokenKind::CharLiteral(c) => Some(Expr::Literal {
                value: LiteralValue::CharLit(c.clone()),
                span: token_span,
            }),

            // Unary operators
            TokenKind::Minus => Some(self.parse_unary(UnaryOp::Negate, token)),
            TokenKind::Not => Some(self.parse_unary(UnaryOp::Not, token)),

            // Grouping
            TokenKind::OpenParen => self.parse_grouping(token),

            // Variables
            TokenKind::IdentifierAscii(name) | TokenKind::IdentifierUnicode(name) => {
                Some(Expr::Variable {
                    name: name.clone(),
                    span: token_span,
                })
            }

            _ => {
                self.errors.push(CompileError::SyntaxError {
                    message: format!("Unexpected token: {:?}", token.kind),
                    span: token.span,
                });
                None
            }
        }
    }

    fn led(&mut self, left: Expr) -> Expr {
        let token = self.advance().expect("Expected operator").clone();
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
                self.errors.push(CompileError::SyntaxError {
                    message: format!("Unexpected operator: {:?}", token.kind),
                    span: token.span,
                });
                left
            }
        }
    }

    fn parse_unary(&mut self, op: UnaryOp, token: Token) -> Expr {
        let (_, rbp) = unary_binding_power(&token);
        let expr = self.parse_expr(rbp);
        Expr::Unary {
            op,
            expr: Box::new(expr.unwrap_or_else(||Expr::new_nullptr(token.span.clone()))),
            span: token.span,
        }
    }

    fn parse_binary(&mut self, left: Expr, token: Token) -> Expr {
        match BinaryOp::get_op(&token) {
            Ok(op) => {
                let right = self.parse_expr(binding_power(&token).1);
                Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right.unwrap_or_else(|| Expr::new_nullptr(token.span.clone()))),
                    span: token.span,
                }
            }
            Err(error) => {
                self.errors.push(error);
                left
            }
        }
    }

    fn parse_grouping(&mut self, start_token: Token) -> Option<Expr> {
        let expr = self.parse_expr(0);
        self.expect(TokenKind::CloseParen, "Unclosed parenthesis");
        Some(Expr::Grouping {
            expr: Box::new(expr?),
            span: start_token.span.merged(&self.previous().unwrap().span).unwrap_or(start_token.span),
        })
    }

    fn parse_assignment(&mut self, left: Expr, token: Token) -> Expr {
        let value = self.parse_expr(1);
        if let Expr::Variable { name, span } = left {
            Expr::Assign {
                name,
                value: Box::new(value.unwrap_or_else(|| Expr::new_nullptr(token.span.clone()))),
                span: span.merged(&token.span).unwrap_or(span),
            }
        } else {
            self.errors.push(CompileError::SyntaxError {
                message: "Invalid assignment target".to_string(),
                span: token.span,
            });
            left
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
            span: start_token.span.merged(&self.previous().unwrap().span).unwrap_or(start_token.span),
        }
    }

    fn parse_array_access(&mut self, array: Expr, start_token: Token) -> Expr {
        let index = self.parse_expr(0);
        self.expect(TokenKind::CloseBracket, "Unclosed array access");
        Expr::ArrayAccess {
            array: Box::new(array),
            index: Box::new(index.unwrap_or_else(||Expr::new_nullptr(start_token.span.clone()))),
            span: start_token.span.merged(&self.previous().unwrap().span).unwrap_or(start_token.span),
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    fn expect(&mut self, kind: TokenKind, message: &str) {
        if !self.match_token(kind) {
            self.errors.push(CompileError::SyntaxError {
                message: message.to_string(),
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