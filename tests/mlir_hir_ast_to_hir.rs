use std::sync::Arc;
use jsavrs::mlir::hir::ast_to_hir::AstToHirTransformer;
use jsavrs::mlir::hir::hirimp::{HIRExpr, HIRStmt, HIRType};
use jsavrs::parser::ast::{BinaryOp, Expr, LiteralValue, Parameter, Stmt, Type};
use jsavrs::tokens::number::Number;
use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
    fn create_test_span() -> SourceSpan {
        let start = SourceLocation::new(1, 1, 0);
        let end = SourceLocation::new(1, 10, 9);
        SourceSpan::new(Arc::from("test.rs"), start, end)
    }

    #[test]
    fn test_transform_simple_literal() {
        let mut transformer = AstToHirTransformer::new();
        let span = create_test_span();
        
        let ast_expr = Expr::Literal {
            value: LiteralValue::Number(Number::Integer(42)),
            span: span.clone(),
        };
        
        let hir_expr = transformer.transform_expr(ast_expr).unwrap();
        
        match hir_expr {
            HIRExpr::Literal { value: LiteralValue::Number(Number::Integer(42)), .. } => {},
            _ => panic!("Expected HIR literal with number 42"),
        }
    }

    #[test]
    fn test_transform_binary_expression() {
        let mut transformer = AstToHirTransformer::new();
        let span = create_test_span();
        
        let left = Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(1)),
            span: span.clone(),
        });
        let right = Box::new(Expr::Literal {
            value: LiteralValue::Number(Number::Integer(2)),
            span: span.clone(),
        });
        
        let ast_expr = Expr::Binary {
            left,
            op: BinaryOp::Add,
            right,
            span: span.clone(),
        };
        
        let hir_expr = transformer.transform_expr(ast_expr).unwrap();
        
        match hir_expr {
            HIRExpr::Binary { op: BinaryOp::Add, .. } => {},
            _ => panic!("Expected HIR binary expression with Add operator"),
        }
    }

    #[test]
    fn test_transform_variable_declaration() {
        let mut transformer = AstToHirTransformer::new();
        let span = create_test_span();
        
        let ast_stmt = Stmt::VarDeclaration {
            variables: vec![Arc::from("x")],
            type_annotation: Type::I32,
            is_mutable: false,
            initializers: vec![Expr::Literal {
                value: LiteralValue::Number(Number::Integer(10)),
                span: span.clone(),
            }],
            span: span.clone(),
        };
        
        let hir_stmt = transformer.transform_stmt(ast_stmt).unwrap();
        
        match hir_stmt {
            HIRStmt::VarDeclaration { 
                variables, 
                type_annotation: HIRType::I32,
                is_mutable: false,
                ..
            } => {
                assert_eq!(variables.len(), 1);
                assert_eq!(variables[0].as_ref(), "x");
            },
            _ => panic!("Expected HIR variable declaration"),
        }
    }

    #[test]
    fn test_parent_child_relationships() {
        let mut transformer = AstToHirTransformer::new();
        let span = create_test_span();
        
        // Create a binary expression to test parent-child relationships
        let left = Box::new(Expr::Variable {
            name: Arc::from("a"),
            span: span.clone(),
        });
        let right = Box::new(Expr::Variable {
            name: Arc::from("b"),
            span: span.clone(),
        });
        
        let ast_expr = Expr::Binary {
            left,
            op: BinaryOp::Add,
            right,
            span: span.clone(),
        };
        
        let hir_expr = transformer.transform_expr(ast_expr).unwrap();
        
        // Verify that the transformation succeeded and has proper structure
        match hir_expr {
            HIRExpr::Binary { left, right, .. } => {
                // Verify left and right operands are transformed correctly
                match (left.as_ref(), right.as_ref()) {
                    (HIRExpr::Variable { name: left_name, .. }, HIRExpr::Variable { name: right_name, .. }) => {
                        assert_eq!(left_name.as_ref(), "a");
                        assert_eq!(right_name.as_ref(), "b");
                    },
                    _ => panic!("Expected left and right to be variables"),
                }
            },
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_transform_function_with_body() {
        let mut transformer = AstToHirTransformer::new();
        let span = create_test_span();
        
        let param = Parameter {
            name: Arc::from("x"),
            type_annotation: Type::I32,
            span: span.clone(),
        };
        
        let return_stmt = Stmt::Return {
            value: Some(Expr::Variable {
                name: Arc::from("x"),
                span: span.clone(),
            }),
            span: span.clone(),
        };
        
        let ast_stmt = Stmt::Function {
            name: Arc::from("identity"),
            parameters: vec![param],
            return_type: Type::I32,
            body: vec![return_stmt],
            span: span.clone(),
        };
        
        let hir_stmt = transformer.transform_stmt(ast_stmt).unwrap();
        
        match hir_stmt {
            HIRStmt::Function { name, parameters, return_type, body, .. } => {
                assert_eq!(name.as_ref(), "identity");
                assert_eq!(parameters.len(), 1);
                assert_eq!(parameters[0].name.as_ref(), "x");
                assert_eq!(return_type, HIRType::I32);
                assert_eq!(body.len(), 1);
                
                // Verify the return statement was transformed correctly
                match &body[0] {
                    HIRStmt::Return { value: Some(HIRExpr::Variable { name, .. }), .. } => {
                        assert_eq!(name.as_ref(), "x");
                    },
                    _ => panic!("Expected return statement with variable"),
                }
            },
            _ => panic!("Expected function declaration"),
        }
    }
