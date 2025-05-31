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

    pub fn parse(mut self) -> (Vec<Stmt>, Vec<CompileError>) {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                // Error recovery: skip the problematic token
                self.advance();
            }
        }
        (statements, self.errors)
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        let token = self.peek()?.clone();
        match token.kind {
            TokenKind::KeywordFun => self.parse_function(),
            TokenKind::KeywordIf => self.parse_if(),
            TokenKind::KeywordVar | TokenKind::KeywordConst => self.parse_var_declaration(),
            TokenKind::KeywordReturn => self.parse_return(),
            // TokenKind::KeywordWhile => self.parse_while(),
            // TokenKind::KeywordFor => self.parse_for(),
            TokenKind::KeywordBreak => self.parse_break(),
            TokenKind::KeywordContinue => self.parse_continue(),
            TokenKind::OpenBrace => self.parse_block_stmt(),
            _ => self.parse_expression_stmt(),
        }
    }

    fn parse_break(&mut self) -> Option<Stmt> {
        let token = self.advance()?; // Consume 'break'
        Some(Stmt::Break {
            span: token.span.clone(),
        })
    }

    fn parse_continue(&mut self) -> Option<Stmt> {
        let token = self.advance()?; // Consume 'continue'
        Some(Stmt::Continue {
            span: token.span.clone(),
        })
    }

    fn parse_block_stmt(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // '{'
        let mut statements = Vec::new();

        while !self.check(TokenKind::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }

        self.expect(TokenKind::CloseBrace, "end of block");
        Some(Stmt::Block {
            statements,
            span: self.merged_span(&start_token),
        })
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // 'return'
        let return_value = if !self.is_end_of_statement() {
            Some(self.parse_expr(0)?)
        } else {
            None
        };

        Some(Stmt::Return {
            value: return_value.clone(),
            span: self.calculate_return_span(&start_token, &return_value),
        })
    }

    fn is_end_of_statement(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind.clone()),
            Some(TokenKind::CloseBrace) | Some(TokenKind::Eof) | Some(TokenKind::Semicolon)
        )
    }

    fn calculate_return_span(&self, start: &Token, value: &Option<Expr>) -> SourceSpan {
        value
            .as_ref()
            .and_then(|v| start.span.merged(v.span()))
            .unwrap_or_else(|| start.span.clone())
    }

    fn parse_function(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // 'fun'
        let name = self.consume_identifier()?;
        let _name_span = self.previous()?.span.clone();

        self.expect(TokenKind::OpenParen, "after function name");
        let mut params = Vec::new();
        while !self.check(TokenKind::CloseParen) && !self.is_at_end() {
            let param_start = self.peek()?.clone();
            let name = self.consume_identifier()?;
            let name_span = self.previous()?.span.clone();
            self.expect(TokenKind::Colon, "after parameter name");
            let type_ann = self.parse_type()?;
            let type_span = self.previous()?.span.clone();
            let param_span = name_span
                .merged(&type_span)
                .unwrap_or_else(|| param_start.span.clone());
            params.push(Parameter {
                name,
                type_annotation: type_ann,
                span: param_span,
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::CloseParen, "after parameter list");

        let return_type = if self.match_token(TokenKind::Colon) {
            let type_ann = self.parse_type()?;
            Some(type_ann)
        } else {
            None
        };

        let body = self.parse_block_stmt()?;
        let end_span = body.span();
        let function_span = start_token
            .span
            .merged(end_span)
            .unwrap_or_else(|| start_token.span.clone());

        Some(Stmt::Function {
            name,
            parameters: params,
            return_type: return_type.unwrap_or(Type::Void),
            body: vec![body],
            span: function_span,
        })
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // 'if'
        let condition = self.parse_expr(0)?;
        let then_branch = self.parse_block_stmt()?;

        let else_branch = if self.match_token(TokenKind::KeywordElse) {
            Some(vec![self.parse_stmt()?])
        } else {
            None
        };

        Some(Stmt::If {
            condition,
            then_branch: vec![then_branch],
            else_branch,
            span: self.merged_span(&start_token),
        })
    }

    fn parse_type(&mut self) -> Option<Type> {
        let token = self.advance()?.clone();
        let mut type_ = match &token.kind {
            TokenKind::TypeI8 => Type::I8,
            TokenKind::TypeI16 => Type::I16,
            TokenKind::TypeI32 => Type::I32,
            TokenKind::TypeI64 => Type::I64,
            TokenKind::TypeU8 => Type::U8,
            TokenKind::TypeU16 => Type::U16,
            TokenKind::TypeU32 => Type::U32,
            TokenKind::TypeU64 => Type::U64,
            TokenKind::TypeF32 => Type::F32,
            TokenKind::TypeF64 => Type::F64,
            TokenKind::TypeChar => Type::Char,
            TokenKind::TypeString => Type::String,
            TokenKind::TypeBool => Type::Bool,
            TokenKind::IdentifierAscii(name) | TokenKind::IdentifierUnicode(name) => {
                Type::Custom(name.clone())
            }
            _ => {
                self.syntax_error("Invalid type", &token);
                return None;
            }
        };

        while self.match_token(TokenKind::OpenBracket) {
            let size_expr = self.parse_expr(0)?;
            self.expect(TokenKind::CloseBracket, "after array size");
            type_ = Type::Array(Box::new(type_), Box::new(size_expr));
        }

        #[allow(clippy::collapsible_if)]
        if let Type::Custom(name) = &type_ {
            if name == "vector" && self.match_token(TokenKind::Less) {
                let inner_type = self.parse_type()?;
                self.expect(TokenKind::Greater, "after vector inner type");
                type_ = Type::Vector(Box::new(inner_type));
            }
        }

        Some(type_)
    }

    fn parse_var_declaration(&mut self) -> Option<Stmt> {
        let is_const = self.match_token(TokenKind::KeywordConst);
        if !is_const {
            self.match_token(TokenKind::KeywordVar);
        }
        let start_token = self.previous()?.clone();

        let mut variables = Vec::new();
        while let Some(name) = self.consume_identifier() {
            variables.push(name);
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        if variables.is_empty() {
            self.syntax_error("Expected at least one variable name", &start_token);
            return None;
        }

        self.expect(TokenKind::Colon, "after variable name(s)");
        let type_ann = match self.parse_type() {
            Some(t) => t,
            None => {
                let err_token = self.peek().cloned();
                if let Some(t) = &err_token {
                    self.syntax_error("Invalid type specification", t);
                }
                Type::Void
            }
        };

        self.expect(TokenKind::Equal, "after type annotation");
        let mut initializers = Vec::new();
        loop {
            match self.parse_expr(0) {
                Some(expr) => initializers.push(expr),
                None => {
                    let err_token = self.peek().cloned();
                    if let Some(t) = &err_token {
                        self.syntax_error("Expected initializer expression", t);
                    }
                    break;
                }
            }
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        Some(Stmt::VarDeclaration {
            variables,
            type_annotation: type_ann,
            initializers,
            span: self.merged_span(&start_token),
        })
    }

    fn consume_identifier(&mut self) -> Option<String> {
        // Capture token first to avoid overlapping borrows
        let token = self.peek()?.clone();

        match &token.kind {
            TokenKind::IdentifierAscii(s) | TokenKind::IdentifierUnicode(s) => {
                self.advance();
                Some(s.clone())
            }
            _ => {
                self.syntax_error("Expected identifier after the 'var' or 'const'", &token);
                None
            }
        }
    }

    fn parse_expression_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expr(0)?;
        Some(Stmt::Expression { expr })
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
            TokenKind::OpenBrace => self.parse_array_literal(token),
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
        let expr = self
            .parse_expr(rbp)
            .unwrap_or_else(|| self.null_expr(token.span.clone()));
        Expr::Unary {
            op,
            expr: Box::new(expr),
            span: token.span,
        }
    }

    fn parse_array_literal(&mut self, start_token: Token) -> Option<Expr> {
        let mut elements = Vec::new();
        self.extract_elements(TokenKind::CloseBrace, &mut elements);
        self.expect(TokenKind::CloseBrace, "end of array literal");
        Some(Expr::ArrayLiteral {
            elements,
            span: self.merged_span(&start_token),
        })
    }

    fn extract_elements(&mut self, kind: TokenKind, elements: &mut Vec<Expr>) {
        while !self.check(kind.clone()) && !self.is_at_end() {
            if let Some(expr) = self.parse_expr(0) {
                elements.push(expr);
            }
            if !self.match_token(TokenKind::Comma) {
                break;
            }
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

        let right = self
            .parse_expr(binding_power(&token).1)
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
        self.expect(TokenKind::CloseParen, "end of grouping");
        Some(Expr::Grouping {
            expr: Box::new(expr?),
            span: self.merged_span(&start_token),
        })
    }

    //src/parser/jsav_parser.rs
    fn parse_assignment(&mut self, left: Expr, token: Token) -> Expr {
        let value = self
            .parse_expr(1)
            .unwrap_or_else(|| self.null_expr(token.span.clone()));

        let span = left
            .span()
            .merged(value.span())
            .unwrap_or(token.span.clone());

        // Check if left is valid l-value (variable or array access)
        let valid = matches!(&left, Expr::Variable { .. } | Expr::ArrayAccess { .. });

        if !valid {
            self.errors.push(CompileError::SyntaxError {
                message: "Invalid left-hand side in assignment".to_string(),
                span: left.span().clone(),
            });
        }

        Expr::Assign {
            target: Box::new(left),
            value: Box::new(value),
            span,
        }
    }

    fn parse_call(&mut self, callee: Expr, start_token: Token) -> Expr {
        let mut arguments = Vec::new();
        self.extract_elements(TokenKind::CloseParen, &mut arguments);
        self.expect(TokenKind::CloseParen, "end of function call arguments");
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
        self.expect(TokenKind::CloseBracket, "end of array access");
        Expr::ArrayAccess {
            array: Box::new(array),
            index: Box::new(index),
            span: self.merged_span(&start_token),
        }
    }

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

    // Improved syntax_error for clearer messages
    fn syntax_error(&mut self, message: &str, token: &Token) {
        self.errors.push(CompileError::SyntaxError {
            message: format!("{}: {}", message, self.token_kind_to_string(&token.kind)),
            span: token.span.clone(),
        });
    }

    /// Improved `expect` to provide more context about expected vs found tokens
    fn expect(&mut self, kind: TokenKind, context: &str) {
        if !self.match_token(kind.clone()) {
            let current_token = self.peek().cloned();
            let expected_str = self.token_kind_to_string(&kind);
            let found_str = current_token
                .as_ref()
                .map(|t| self.token_kind_to_string(&t.kind))
                .unwrap_or_else(|| "end of input".to_string());

            let span = current_token
                .as_ref()
                .map(|t| t.span.clone())
                .unwrap_or_default();

            let error_message =
                format!("Expected {expected_str} in {context} but found {found_str}");
            self.errors.push(CompileError::SyntaxError {
                message: error_message,
                span,
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

    pub fn token_kind_to_string(&self, kind: &TokenKind) -> String {
        match kind {
            TokenKind::Eof => "end of file".to_string(),
            TokenKind::IdentifierAscii(s) => format!("identifier '{s}'"),
            TokenKind::IdentifierUnicode(s) => format!("identifier '{s}'"),
            TokenKind::Numeric(n) => format!("number '{n}'"),
            TokenKind::StringLiteral(s) => format!("string literal \"{s}\""),
            TokenKind::CharLiteral(c) => format!("character literal '{c}'"),
            TokenKind::KeywordBool(b) => format!("boolean '{b}'"),
            TokenKind::KeywordNullptr => "nullptr".to_string(),
            // Keywords
            TokenKind::KeywordFun => "'fun'".to_string(),
            TokenKind::KeywordIf => "'if'".to_string(),
            TokenKind::KeywordElse => "'else'".to_string(),
            TokenKind::KeywordVar => "'var'".to_string(),
            TokenKind::KeywordConst => "'const'".to_string(),
            TokenKind::KeywordReturn => "'return'".to_string(),
            TokenKind::KeywordWhile => "'while'".to_string(),
            TokenKind::KeywordFor => "'for'".to_string(),
            TokenKind::KeywordBreak => "'break'".to_string(),
            TokenKind::KeywordContinue => "'continue'".to_string(),
            // Types
            TokenKind::TypeI8 => "'i8'".to_string(),
            TokenKind::TypeI16 => "'i16'".to_string(),
            TokenKind::TypeI32 => "'i32'".to_string(),
            TokenKind::TypeI64 => "'i64'".to_string(),
            TokenKind::TypeU8 => "'u8'".to_string(),
            TokenKind::TypeU16 => "'u16'".to_string(),
            TokenKind::TypeU32 => "'u32'".to_string(),
            TokenKind::TypeU64 => "'u64'".to_string(),
            TokenKind::TypeF32 => "'f32'".to_string(),
            TokenKind::TypeF64 => "'f64'".to_string(),
            TokenKind::TypeChar => "'char'".to_string(),
            TokenKind::TypeString => "'string'".to_string(),
            TokenKind::TypeBool => "'bool'".to_string(),
            // Punctuation
            TokenKind::OpenParen => "'('".to_string(),
            TokenKind::CloseParen => "')'".to_string(),
            TokenKind::OpenBrace => "'{'".to_string(),
            TokenKind::CloseBrace => "'}'".to_string(),
            TokenKind::OpenBracket => "'['".to_string(),
            TokenKind::CloseBracket => "']'".to_string(),
            TokenKind::Semicolon => "';'".to_string(),
            TokenKind::Colon => "':'".to_string(),
            TokenKind::Comma => "','".to_string(),
            TokenKind::Dot => "'.'".to_string(),

            // Operators
            TokenKind::Plus => "'+'".to_string(),
            TokenKind::PlusPlus => "'++'".to_string(),
            TokenKind::MinusMinus => "'--'".to_string(),
            TokenKind::PlusEqual => "'+='".to_string(),
            TokenKind::Minus => "'-'".to_string(),
            TokenKind::Star => "'*'".to_string(),
            TokenKind::Slash => "'/'".to_string(),
            TokenKind::Percent => "'%'".to_string(),
            TokenKind::Equal => "'='".to_string(),
            TokenKind::EqualEqual => "'=='".to_string(),
            TokenKind::NotEqual => "'!='".to_string(),
            TokenKind::Less => "'<'".to_string(),
            TokenKind::LessEqual => "'<='".to_string(),
            TokenKind::Greater => "'>'".to_string(),
            TokenKind::GreaterEqual => "'>='".to_string(),
            TokenKind::AndAnd => "'&&'".to_string(),
            TokenKind::OrOr => "'||'".to_string(),
            TokenKind::Not => "'!'".to_string(),
            TokenKind::And => "'&'".to_string(),
            TokenKind::Or => "'|'".to_string(),
            TokenKind::Xor => "'^'".to_string(),
            TokenKind::ShiftLeft => "'<<'".to_string(),
            TokenKind::ShiftRight => "'>>'".to_string(),

            // Fallback for any unhandled variants
            _ => format!("'{kind:?}'"),
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .map(|t| t.kind == TokenKind::Eof)
            .unwrap_or(true)
    }
}
