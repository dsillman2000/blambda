#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::blambda::eval::evaluate_expr;
    use crate::blambda::format::format_expr;
    use crate::blambda::format::format_program;
    use crate::blambda::parse::*;
    use crate::blambda::syntax::*;
    use crate::fallible_parse;

    #[test]
    fn test_serialize_value_expr() {
        // Serialize true as yaml
        let expr = Expr::Value(true);
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "true\n");

        // Serialize false as yaml
        let expr = Expr::Value(false);
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "false\n");
    }

    #[test]
    fn test_serialize_unary_expr() {
        // Serialize unary NOT as yaml
        let expr = Expr::Unary {
            op: UnOp::Not,
            arg: Arc::new(Expr::Value(true)),
        };
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "op: not\narg: true\n");
    }

    #[test]
    fn test_serialize_binary_expr() {
        // Serialize binary OR as yaml
        let expr = Expr::Binary {
            op: BinOp::Or,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "op: or\narg1: true\narg2: false\n");

        // Serialize binary AND as yaml
        let expr = Expr::Binary {
            op: BinOp::And,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "op: and\narg1: true\narg2: false\n");

        // Serialize binary CONDITION as yaml
        let expr = Expr::Binary {
            op: BinOp::Condition,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "op: condition\narg1: true\narg2: false\n");

        // Serialize binary BRANCH as yaml
        let expr = Expr::Binary {
            op: BinOp::Branch,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let serialized = serde_yaml::to_string(&expr).unwrap();
        assert_eq!(serialized, "op: branch\narg1: true\narg2: false\n");
    }

    #[test]
    fn test_serialize_program() {
        // Serialize program with one value expression as yaml
        let program = Program {
            exprs: vec![Expr::Value(true)],
        };
        let serialized = serde_yaml::to_string(&program).unwrap();
        assert_eq!(serialized, "exprs:\n- true\n");

        // Serialize program with two unary expressions as yaml
        let program = Program {
            exprs: vec![
                Expr::Unary {
                    op: UnOp::Not,
                    arg: Arc::new(Expr::Value(true)),
                },
                Expr::Unary {
                    op: UnOp::Not,
                    arg: Arc::new(Expr::Value(false)),
                },
            ],
        };
        let serialized = serde_yaml::to_string(&program).unwrap();
        assert_eq!(
            serialized,
            "exprs:\n- op: not\n  arg: true\n- op: not\n  arg: false\n"
        );
    }

    #[test]
    fn test_parse_value_expr() {
        // Parse true from blambda script
        let input = "t";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(expr, Expr::Value(true));

        // Parse false from blambda script
        let input = "f";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(expr, Expr::Value(false));
    }

    #[test]
    fn test_parse_unary_expr() {
        // Parse unary NOT from blambda script
        let input = "~t";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Unary {
                op: UnOp::Not,
                arg: Arc::new(Expr::Value(true)),
            }
        );
    }

    #[test]
    fn test_parse_binary_expr() {
        // Parse binary OR from blambda script
        let input = "t | f";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinOp::Or,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Value(false)),
            }
        );

        // Parse binary AND from blambda script
        let input = "t & f";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinOp::And,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Value(false)),
            }
        );

        // Parse binary CONDITION from blambda script
        let input = "t ? f";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinOp::Condition,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Value(false)),
            }
        );

        // Parse binary BRANCH from blambda script
        let input = "t : f";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinOp::Branch,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Value(false)),
            }
        );
    }

    #[test]
    fn test_parse_ternary_operator() {
        // Parse ternary operator from blambda script
        let input = "t ? f : t";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinOp::Condition,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Binary {
                    op: BinOp::Branch,
                    arg1: Arc::new(Expr::Value(false)),
                    arg2: Arc::new(Expr::Value(true)),
                }),
            }
        );

        // Parse nested ternary operator from blambda script
        let input = "t ? (f ? t : f) : t";
        let pairs = fallible_parse(Rule::expr, input).unwrap();
        let expr = parse_expr(pairs);
        assert_eq!(
            expr,
            Expr::Binary {
                op: BinOp::Condition,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Binary {
                    op: BinOp::Branch,
                    arg1: Arc::new(Expr::Binary {
                        op: BinOp::Condition,
                        arg1: Arc::new(Expr::Value(false)),
                        arg2: Arc::new(Expr::Binary {
                            op: BinOp::Branch,
                            arg1: Arc::new(Expr::Value(true)),
                            arg2: Arc::new(Expr::Value(false)),
                        }),
                    }),
                    arg2: Arc::new(Expr::Value(true)),
                }),
            }
        );
    }

    #[test]
    fn test_eval_expr_value() {
        // Evaluate true value expression
        let expr = Expr::Value(true);
        let value = evaluate_expr(expr);
        assert_eq!(value, Some(true));

        // Evaluate false value expression
        let expr = Expr::Value(false);
        let value = evaluate_expr(expr);
        assert_eq!(value, Some(false));
    }

    #[test]
    fn test_eval_expr_unary() {
        // Evaluate unary NOT expression
        let expr = Expr::Unary {
            op: UnOp::Not,
            arg: Arc::new(Expr::Value(true)),
        };
        let value = evaluate_expr(expr);
        assert_eq!(value, Some(false));
    }

    #[test]
    fn test_eval_expr_binary() {
        // Evaluate binary OR expression
        let expr = Expr::Binary {
            op: BinOp::Or,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let value = evaluate_expr(expr);
        assert_eq!(value, Some(true));

        // Evaluate binary AND expression
        let expr = Expr::Binary {
            op: BinOp::And,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let value = evaluate_expr(expr);
        assert_eq!(value, Some(false));

        // Evaluate ternary operator
        let expr = Expr::Binary {
            op: BinOp::Condition,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Binary {
                op: BinOp::Branch,
                arg1: Arc::new(Expr::Value(false)),
                arg2: Arc::new(Expr::Value(true)),
            }),
        };
        let value = evaluate_expr(expr);
        assert_eq!(value, Some(false));
    }

    #[test]
    fn test_eval_bad_expr() {
        // Fail to evaluate single condition expression
        let expr = Expr::Binary {
            op: BinOp::Condition,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(true)),
        };
        let value = evaluate_expr(expr);
        assert_eq!(value, None);

        // Fail to evaluate single branch expression
        let expr = Expr::Binary {
            op: BinOp::Branch,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(true)),
        };
        let value = evaluate_expr(expr);
        assert_eq!(value, None);
    }

    #[test]
    fn test_format_expr_value() {
        // Format true value expression
        let expr = Expr::Value(true);
        let formatted = format_expr(&expr);
        assert_eq!(formatted, "t");

        // Format false value expression
        let expr = Expr::Value(false);
        let formatted = format_expr(&expr);
        assert_eq!(formatted, "f");
    }

    #[test]
    fn test_format_expr_unary() {
        // Format unary NOT expression
        let expr = Expr::Unary {
            op: UnOp::Not,
            arg: Arc::new(Expr::Value(true)),
        };
        let formatted = format_expr(&expr);
        assert_eq!(formatted, "(~ t)");
    }

    #[test]
    fn test_format_expr_binary() {
        // Format binary OR expression
        let expr = Expr::Binary {
            op: BinOp::Or,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let formatted = format_expr(&expr);
        assert_eq!(formatted, "(t | f)");

        // Format binary AND expression
        let expr = Expr::Binary {
            op: BinOp::And,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Value(false)),
        };
        let formatted = format_expr(&expr);
        assert_eq!(formatted, "(t & f)");
    }

    #[test]
    fn test_format_expr_ternary() {
        // Format ternary operator
        let expr = Expr::Binary {
            op: BinOp::Condition,
            arg1: Arc::new(Expr::Value(true)),
            arg2: Arc::new(Expr::Binary {
                op: BinOp::Branch,
                arg1: Arc::new(Expr::Value(false)),
                arg2: Arc::new(Expr::Value(true)),
            }),
        };
        let formatted = format_expr(&expr);
        assert_eq!(formatted, "(t ? (f : t))");
    }

    #[test]
    fn test_format_program() {
        // Test format singleton program
        let program = Program {
            exprs: vec![Expr::Binary {
                op: BinOp::Or,
                arg1: Arc::new(Expr::Value(true)),
                arg2: Arc::new(Expr::Value(false)),
            }],
        };
        let formatted = format_program(&program);
        assert_eq!(formatted, "(t | f)");

        // Test format program with multiple expressions
        let program = Program {
            exprs: vec![
                Expr::Binary {
                    op: BinOp::Or,
                    arg1: Arc::new(Expr::Value(true)),
                    arg2: Arc::new(Expr::Value(false)),
                },
                Expr::Binary {
                    op: BinOp::And,
                    arg1: Arc::new(Expr::Value(true)),
                    arg2: Arc::new(Expr::Value(false)),
                },
            ],
        };
        let formatted = format_program(&program);
        assert_eq!(formatted, "(t | f) (t & f)");
    }

    #[test]
    fn test_parse_format_expr_commutes() {
        // Test that parsing and formatting an expression commutes
        let inputs = vec!["t", "f", "(~ t)", "(f | f)", "(t & t)", "(t ? (f : t))"];
        inputs.iter().for_each(|&input| {
            let pairs = fallible_parse(Rule::expr, input).unwrap();
            let expr = parse_expr(pairs);
            let formatted = format_expr(&expr);
            assert_eq!(formatted, input);
        });
    }

    #[test]
    fn test_parse_format_program_commutes() {
        // Test that parsing and formatting a program commutes
        let inputs = vec![
            "(t | f)",
            "(t | f) (t & f)",
            "(t | f) (t & f) (t ? (f : t))",
        ];
        inputs.iter().for_each(|&input| {
            let pairs = fallible_parse(Rule::program, input).unwrap();
            let program = parse_program(pairs);
            let formatted = format_program(&program);
            assert_eq!(formatted, input);
        });
    }
}
