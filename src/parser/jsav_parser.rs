// src/parser/jsav_parser.rs
use crate::error::compile_error::CompileError;
use crate::location::source_span::SourceSpan;
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
        let token = self.peek()?;
        match token.kind {
            TokenKind::KeywordFun => self.parse_function(),
            TokenKind::KeywordMain => self.parse_main_function(),
            TokenKind::KeywordIf => self.parse_if(),
            TokenKind::KeywordVar | TokenKind::KeywordConst => self.parse_var_declaration(),
            TokenKind::KeywordReturn => self.parse_return(),
            TokenKind::KeywordWhile => self.parse_while(),
            TokenKind::KeywordFor => self.parse_for(),
            TokenKind::KeywordBreak | TokenKind::KeywordContinue => self.parse_break_continue(),
            TokenKind::OpenBrace => self.parse_block_stmt(),
            _ => self.parse_expression_stmt(),
        }
    }
    fn parse_main_function(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // 'main'
        let body = self.parse_block_stmt()?;
        let end_span = body.span();

        let function_span = start_token
            .span
            .merged(end_span)
            .unwrap_or_else(|| start_token.span.clone());

        Some(Stmt::MainFunction {
            body: vec![body],
            span: function_span,
        })
    }

    fn parse_break_continue(&mut self) -> Option<Stmt> {
        let token = self.advance()?; // Use reference
        let span = token.span.clone();
        match token.kind {
            TokenKind::KeywordBreak => Some(Stmt::Break { span }),
            TokenKind::KeywordContinue => Some(Stmt::Continue { span }),
            _ => None,
        }
    }

    fn parse_block_stmt(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // '{'
        let mut statements = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }

        self.expect(&TokenKind::CloseBrace, "end of block");
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

        self.expect(&TokenKind::OpenParen, "after function name");
        let mut params = Vec::new();
        while !self.check(&TokenKind::CloseParen) && !self.is_at_end() {
            let param_start = self.peek()?.clone();
            let name = self.consume_identifier()?;
            let name_span = self.previous()?.span.clone();
            self.expect(&TokenKind::Colon, "after parameter name");
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
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::CloseParen, "after parameter list");

        let return_type = if self.match_token(&TokenKind::Colon) {
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
        self.expect(&TokenKind::OpenParen, "after 'if'");
        let condition = self.parse_expr(0)?;
        self.expect(&TokenKind::CloseParen, "after the condition");
        let then_branch = self.parse_block_stmt()?;

        let else_branch = if self.match_token(&TokenKind::KeywordElse) {
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

    fn parse_while(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // 'while'
        self.expect(&TokenKind::OpenParen, "after 'while'");
        let condition = self.parse_expr(0)?;
        self.expect(&TokenKind::CloseParen, "after the condition");
        let body = self.parse_block_stmt()?;
        let end_span = body.span();
        let function_span = start_token
            .span
            .merged(end_span)
            .unwrap_or_else(|| start_token.span.clone());
        Some(Stmt::While {
            condition,
            body: vec![body],
            span: function_span,
        })
    }

    fn parse_for(&mut self) -> Option<Stmt> {
        let start_token = self.advance()?.clone(); // 'for'
        self.expect(&TokenKind::OpenParen, "after 'for'");

        // 1. Parse initializer (può essere var/const, espressione o vuoto)
        let initializer = if self.match_token(&TokenKind::Semicolon) {
            None // Inizializzatore vuoto
        } else if self.check(&TokenKind::KeywordVar) {
            // Dichiarazione var/const
            let stmt = self.parse_var_declaration();
            self.expect(&TokenKind::Semicolon, "after for loop initializer");
            stmt.map(Box::new)
        } else {
            // Espressione
            let stmt = self.parse_expression_stmt();
            self.expect(&TokenKind::Semicolon, "after for loop initializer");
            stmt.map(Box::new)
        };

        // 2. Parse condition (opzionale)
        let condition = if self.check(&TokenKind::Semicolon) {
            self.advance(); // Consuma il punto e virgola
            None
        } else {
            let expr = self.parse_expr(0);
            self.expect(&TokenKind::Semicolon, "after for loop condition");
            expr
        };

        // 3. Parse increment (opzionale)
        let increment = if self.check(&TokenKind::CloseParen) {
            None
        } else {
            // Non c'è punto e virgola dopo l'incremento
            self.parse_expr(0)
        };

        // 4. Chiudi parentesi
        self.expect(&TokenKind::CloseParen, "after for loop clauses");

        // 5. Parse body
        let body_stmt = self.parse_stmt()?;
        let body = if let Stmt::Block { statements, .. } = body_stmt {
            statements // Usa le dichiarazioni direttamente
        } else {
            vec![body_stmt] // Avvolgi in un vettore
        };

        // Calcola lo span totale (dal token 'for' alla fine del body)
        let end_span = body.last().map(|s| s.span()).cloned().unwrap_or_else(|| {
            self.previous()
                .map(|t| t.span.clone())
                .unwrap_or_else(|| start_token.span.clone())
        });

        let span = start_token.span.merged(&end_span).unwrap_or(start_token.span);

        Some(Stmt::For {
            initializer,
            condition,
            increment,
            body,
            span,
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

        while self.match_token(&TokenKind::OpenBracket) {
            let size_expr = self.parse_expr(0)?;
            self.expect(&TokenKind::CloseBracket, "after array size");
            type_ = Type::Array(Box::new(type_), Box::new(size_expr));
        }

        #[allow(clippy::collapsible_if)]
        if let Type::Custom(name) = &type_ {
            if name == "vector" && self.match_token(&TokenKind::Less) {
                let inner_type = self.parse_type()?;
                self.expect(&TokenKind::Greater, "after vector inner type");
                type_ = Type::Vector(Box::new(inner_type));
            }
        }

        Some(type_)
    }

    #[allow(clippy::if_same_then_else)]
    fn parse_var_declaration(&mut self) -> Option<Stmt> {
        let (start_token, is_mutable) = if self.match_token(&TokenKind::KeywordConst) {
            (self.previous().unwrap().clone(), false)
        } else if self.match_token(&TokenKind::KeywordVar) {
            (self.previous().unwrap().clone(), true)
        } else {
            let token = self.previous().unwrap().clone();
            self.syntax_error("Expected 'const' or 'var'", &token);
            return None;
        };

        let mut variables = Vec::new();
        while let Some(name) = self.consume_identifier() {
            variables.push(name);
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }

        if variables.is_empty() {
            self.syntax_error("Expected at least one variable name", &start_token);
            return None;
        }

        self.expect(&TokenKind::Colon, "after variable name(s)");
        let type_ann = match self.parse_type() {
            Some(t) => t,
            None => {
                self.report_peek_error("Invalid type specification");
                Type::Void
            }
        };

        self.expect(&TokenKind::Equal, "after type annotation");
        let mut initializers = Vec::new();
        loop {
            match self.parse_expr(0) {
                Some(expr) => initializers.push(expr),
                None => {
                    self.report_peek_error("Expected initializer expression");
                    break;
                }
            }
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }

        if variables.len() != initializers.len() {
            self.syntax_error("Number of initializers does not match number of variables", &start_token);
        }

        Some(Stmt::VarDeclaration {
            variables,
            type_annotation: type_ann,
            is_mutable,
            initializers,
            span: self.merged_span(&start_token),
        })
    }

    fn report_peek_error(&mut self, message: &str) {
        if let Some(token) = &self.peek().cloned() {
            self.syntax_error(message, token);
        }
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
            left = self.led(left)?;
        }

        Some(left)
    }

    fn nud(&mut self) -> Option<Expr> {
        let token = self.advance()?.clone();
        match token.kind {
            // Literals
            TokenKind::Numeric(n) => Expr::new_number_literal(n, token.span),
            TokenKind::KeywordBool(b) => Expr::new_bool_literal(b, token.span),
            TokenKind::KeywordNullptr => Expr::new_nullptr_literal(token.span),
            TokenKind::StringLiteral(s) => Expr::new_string_literal(s, token.span),
            TokenKind::CharLiteral(c) => Expr::new_char_literal(c, token.span),
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

    fn led(&mut self, left: Expr) -> Option<Expr> {
        let token = match self.advance() {
            Some(t) => t.clone(),
            None => return None,
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
                None
            }
        }
    }

    // Parsing operations
    fn parse_unary(&mut self, op: UnaryOp, token: Token) -> Expr {
        let (_, rbp) = unary_binding_power(&token);
        let expr = self
            .parse_expr(rbp)
            .unwrap_or_else(|| Expr::null_expr(token.span.clone()));
        Expr::Unary {
            op,
            expr: Box::new(expr),
            span: token.span,
        }
    }

    fn parse_array_literal(&mut self, start_token: Token) -> Option<Expr> {
        let mut elements = Vec::new();
        self.extract_elements(TokenKind::CloseBrace, &mut elements);
        if !self.expect(&TokenKind::CloseBrace, "end of array literal") {
            return None;
        }
        Some(Expr::ArrayLiteral {
            elements,
            span: self.merged_span(&start_token),
        })
    }

    fn extract_elements(&mut self, kind: TokenKind, elements: &mut Vec<Expr>) {
        while !self.check(&kind.clone()) && !self.is_at_end() {
            if let Some(expr) = self.parse_expr(0) {
                elements.push(expr);
            }
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
    }

    fn parse_binary(&mut self, left: Expr, token: Token) -> Option<Expr> {
        let op = match BinaryOp::get_op(&token) {
            Ok(op) => op,
            Err(e) => {
                self.errors.push(e);
                return None;
            }
        };

        let right = self
            .parse_expr(binding_power(&token).1)
            .unwrap_or_else(|| Expr::null_expr(token.span.clone()));
        Some(Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
            span: token.span,
        })
    }

    fn parse_grouping(&mut self, start_token: Token) -> Option<Expr> {
        let expr = self.parse_expr(0);
        if !self.expect(&TokenKind::CloseParen, "end of grouping") {
            return None;
        }
        Some(Expr::Grouping {
            expr: Box::new(expr?),
            span: self.merged_span(&start_token),
        })
    }

    fn parse_assignment(&mut self, left: Expr, token: Token) -> Option<Expr> {
        let value = self
            .parse_expr(1)
            .unwrap_or_else(|| Expr::null_expr(token.span.clone()));

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
            return None;
        }

        Some(Expr::Assign {
            target: Box::new(left),
            value: Box::new(value),
            span,
        })
    }

    fn parse_call(&mut self, callee: Expr, start_token: Token) -> Option<Expr> {
        let mut arguments = Vec::new();
        self.extract_elements(TokenKind::CloseParen, &mut arguments);

        // Check if we successfully found the closing parenthesis
        if !self.expect(&TokenKind::CloseParen, "end of function call arguments") {
            return None;
        }

        Some(Expr::Call {
            callee: Box::new(callee),
            arguments,
            span: self.merged_span(&start_token),
        })
    }

    fn parse_array_access(&mut self, array: Expr, start_token: Token) -> Option<Expr> {
        let index = self
            .parse_expr(0)
            .unwrap_or_else(|| Expr::null_expr(start_token.span.clone()));
        if !self.expect(&TokenKind::CloseBracket, "end of array access") {
            return None;
        }
        Some(Expr::ArrayAccess {
            array: Box::new(array),
            index: Box::new(index),
            span: self.merged_span(&start_token),
        })
    }

    fn merged_span(&self, start_token: &Token) -> SourceSpan {
        self.previous()
            .and_then(|end| start_token.span.merged(&end.span))
            .unwrap_or(start_token.span.clone())
    }

    // Improved syntax_error for clearer messages
    fn syntax_error(&mut self, message: &str, token: &Token) {
        self.errors.push(CompileError::SyntaxError {
            message: format!("{}: {}", message, &token.kind),
            span: token.span.clone(),
        });
    }

    /// Improved `expect` to provide more context about expected vs found tokens
    fn expect(&mut self, kind: &TokenKind, context: &str) -> bool {
        if self.match_token(&kind.clone()) {
            true
        } else {
            let current_token = self.peek().cloned();
            let expected = &kind.clone();
            let found_str = current_token
                .as_ref()
                .map(|t| t.kind.to_string())
                .unwrap_or_else(|| "end of input".to_string());

            let span = current_token
                .as_ref()
                .map(|t| t.span.clone())
                .unwrap_or_default();

            let error_message = format!("Expected {expected} in {context} but found {found_str}");
            self.errors.push(CompileError::SyntaxError {
                message: error_message,
                span,
            });
            false
        }
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
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

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().map(|t| &t.kind == kind).unwrap_or(false)
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .map(|t| t.kind == TokenKind::Eof)
            .unwrap_or(true)
    }
}