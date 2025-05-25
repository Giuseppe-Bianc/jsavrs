// src/parser/jsav_parser.rs

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

    /// Entry point: parse a sequence of statements until EOF
    pub fn parse(mut self) -> (Vec<Stmt>, Vec<CompileError>) {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                // Synchronize on error: skip one token
                self.advance();
            }
        }
        (statements, self.errors)
    }

    /// Parse a single statement (recursive descent)
    fn parse_statement(&mut self) -> Option<Stmt> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::KeywordFun) => self.parse_function(),
            Some(TokenKind::KeywordIf) => self.parse_if(),
            Some(TokenKind::KeywordReturn) => self.parse_return(),
            Some(TokenKind::OpenBrace) => self.parse_block(),
            Some(TokenKind::KeywordVar) => self.parse_var_declaration(),
            _ => self.parse_expression_stmt(),
        }
    }

    /// Parse a function definition:
    /// fun <name>(<params>) [: <return_type>] { <body> }
    fn parse_function(&mut self) -> Option<Stmt> {
        // consume 'fun'
        let fun_token = self.advance().unwrap().clone();

        // require identifier
        let (name, name_span) = if let Some(Token {
                                                kind: TokenKind::IdentifierAscii(s),
                                                span,
                                                ..
                                            }) = self.advance()
        {
            (s.clone(), span.clone())
        } else {
            self.syntax_error("Expected function name", &fun_token);
            return None;
        };

        // parameters
        self.expect(TokenKind::OpenParen, "Expected '(' after function name");
        let mut parameters = Vec::new();
        if !self.check(TokenKind::CloseParen) {
            loop {
                // param name
                let (param_name, param_span) = if let Some(Token {
                                                               kind: TokenKind::IdentifierAscii(s),
                                                               span,
                                                               ..
                                                           }) = self.advance()
                {
                    (s.clone(), span.clone())
                } else {
                    self.syntax_error("Expected parameter name", &fun_token);
                    return None;
                };
                // colon
                self.expect(TokenKind::Colon, "Expected ':' after parameter name");
                // type
                let type_annotation = self.parse_type().unwrap_or(Type::Void);

                parameters.push(Parameter {
                    name: param_name,
                    type_annotation,
                    span: param_span,
                });

                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::CloseParen, "Expected ')' after parameters");

        // optional return type
        let return_type = if self.match_token(TokenKind::Colon) {
            self.parse_type().unwrap_or(Type::Void)
        } else {
            Type::Void
        };

        // body block
        let body_start_span = if let Some(Token { span, .. }) = self.peek() {
            span.clone()
        } else {
            fun_token.span.clone()
        };
        let body = if let Some(Stmt::Block { statements, .. }) = self.parse_block() {
            statements
        } else {
            Vec::new()
        };

        Some(Stmt::Function {
            name,
            parameters,
            return_type,
            body,
            span: name_span.merged(&body_start_span).unwrap_or(name_span),
        })
    }

    /// Parse an if statement:
    /// if (<condition>) { <then_branch> } [else { <else_branch> }]
    fn parse_if(&mut self) -> Option<Stmt> {
        let if_token = self.advance().unwrap().clone();
        self.expect(TokenKind::OpenParen, "Expected '(' after 'if'");
        let condition = self.parse_expr(0)?;
        self.expect(TokenKind::CloseParen, "Expected ')' after if condition");
        let then_branch = if let Some(Stmt::Block { statements, span: _then_span }) = self.parse_block() {
            statements
        } else {
            Vec::new()
        };

        let else_branch = if self.match_token(TokenKind::KeywordElse) {
            if let Some(Stmt::Block { statements, .. }) = self.parse_block() {
                Some(statements)
            } else {
                Some(Vec::new())
            }
        } else {
            None
        };

        Some(Stmt::If {
            condition,
            then_branch,
            else_branch,
            span: if_token.span.clone(),
        })
    }

    /// Parse a return statement:
    /// return [<expr>]
    fn parse_return(&mut self) -> Option<Stmt> {
        let return_token = self.advance().unwrap().clone();
        let value = if self.check_expression_start() {
            Some(self.parse_expr(0)?)
        } else {
            None
        };
        Some(Stmt::Return {
            value,
            span: return_token.span.clone(),
        })
    }

    /// Parse a block: { <statements> }
    fn parse_block(&mut self) -> Option<Stmt> {
        let start_token = self.advance().unwrap().clone(); // consume '{'
        let mut statements = Vec::new();
        while !self.check(TokenKind::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }
        self.expect(TokenKind::CloseBrace, "Expected '}' after block");
        Some(Stmt::Block {
            statements,
            span: start_token.span.clone(),
        })
    }

    /// Parse a variable declaration:
    /// var <name> : <type> [= <initializer>]
    fn parse_var_declaration(&mut self) -> Option<Stmt> {
        let var_token = self.advance().unwrap().clone(); // consume 'var'

        // parse variable name
        let (name, name_span) = if let Some(Token {
                                                kind: TokenKind::IdentifierAscii(s),
                                                span,
                                                ..
                                            }) = self.advance()
        {
            (s.clone(), span.clone())
        } else {
            self.syntax_error("Expected variable name", &var_token);
            return None;
        };

        // colon
        self.expect(TokenKind::Colon, "Expected ':' after variable name");
        // parse type
        let type_annotation = self.parse_type().unwrap_or(Type::Void);

        // initializer (optional)
        let mut initializers = Vec::new();
        if self.match_token(TokenKind::Equal) {
            let init_expr = self.parse_expr(0)?;
            initializers.push(init_expr);
        }

        Some(Stmt::VarDeclaration {
            variables: vec![name],
            type_annotation,
            initializers,
            span: name_span,
        })
    }

    /// Parse an expression statement (just an Expr wrapped in Stmt::Expression)
    fn parse_expression_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr(0)?;
        Some(Stmt::Expression { expr })
    }

    /// Helpers to detect start of an expression
    fn check_expression_start(&self) -> bool {
        matches!(
            self.peek().map(|t| &t.kind),
            Some(TokenKind::Minus)
                | Some(TokenKind::Not)
                | Some(TokenKind::OpenParen)
                | Some(TokenKind::IdentifierAscii(_))
                | Some(TokenKind::IdentifierUnicode(_))
                | Some(TokenKind::Numeric(_))
                | Some(TokenKind::KeywordBool(_))
                | Some(TokenKind::KeywordNullptr)
                | Some(TokenKind::StringLiteral(_))
                | Some(TokenKind::CharLiteral(_))
        )
    }

    /// Parse a type annotation (TypeI8, TypeI16, etc.)
    fn parse_type(&mut self) -> Option<Type> {
        let token = match self.advance() {
            Some(t) => t.clone(),
            None => return None,
        };

        match token.kind {
            TokenKind::TypeI8 => Some(Type::I8),
            TokenKind::TypeI16 => Some(Type::I16),
            TokenKind::TypeI32 => Some(Type::I32),
            TokenKind::TypeI64 => Some(Type::I64),
            TokenKind::TypeU8 => Some(Type::U8),
            TokenKind::TypeU16 => Some(Type::U16),
            TokenKind::TypeU32 => Some(Type::U32),
            TokenKind::TypeU64 => Some(Type::U64),
            TokenKind::TypeF32 => Some(Type::F32),
            TokenKind::TypeF64 => Some(Type::F64),
            TokenKind::TypeChar => Some(Type::Char),
            TokenKind::TypeString => Some(Type::String),
            TokenKind::TypeBool => Some(Type::Bool),
            _ => {
                self.syntax_error("Expected type annotation but found", &token);
                None
            }
        }
    }

    // ------------------------------------------------------------------------
    // Expression parsing (Pratt) from the existing JsavParser

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
            None => return left,
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

    // Literal helpers
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
        let index = self
            .parse_expr(0)
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
            let found = self
                .peek()
                .map(|t| format!("{:?}", t.kind))
                .unwrap_or_else(|| "end of input".to_string());
            self.errors.push(CompileError::SyntaxError {
                message: format!("{context}: Expected '{kind:?}' but found {found}"),
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
