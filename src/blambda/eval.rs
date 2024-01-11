use crate::blambda::syntax::*;
use num_bigint::BigUint;

pub fn evaluate_expr(expr: Expr) -> Option<bool> {
    match expr {
        Expr::Value(v) => Some(v),
        Expr::Unary { op, arg } => match op {
            UnOp::Not => evaluate_expr((*arg).clone()).map(|v| !v),
        },
        Expr::Binary {
            op: BinOp::Condition,
            arg1: condition,
            arg2: branches,
        } => {
            let condition = evaluate_expr((*condition).clone());
            match (condition, (*branches).clone()) {
                (
                    Some(condition),
                    Expr::Binary {
                        op: BinOp::Branch,
                        arg1: branch1,
                        arg2: branch2,
                    },
                ) => {
                    if condition {
                        evaluate_expr((*branch1).clone())
                    } else {
                        evaluate_expr((*branch2).clone())
                    }
                }
                _ => None,
            }
        }
        Expr::Binary { op, arg1, arg2 } => match op {
            BinOp::Or => {
                let v1 = evaluate_expr((*arg1).clone());
                let v2 = evaluate_expr((*arg2).clone());
                match (v1, v2) {
                    (Some(v1), Some(v2)) => Some(v1 || v2),
                    _ => None,
                }
            }
            BinOp::And => {
                let v1 = evaluate_expr((*arg1).clone());
                let v2 = evaluate_expr((*arg2).clone());
                match (v1, v2) {
                    (Some(v1), Some(v2)) => Some(v1 && v2),
                    _ => None,
                }
            }
            _ => None,
        },
    }
}

pub fn evaluate_program(program: Program) -> Option<BigUint> {
    program
        .exprs
        .iter()
        .map(|expr| evaluate_expr(expr.clone()))
        .fold(Some(BigUint::from(0u32)), |acc, v| match (acc, v) {
            (Some(b), Some(true)) => Some(2u32 * b + 1u32),
            (Some(b), Some(false)) => Some(2u32 * b),
            (_, _) => None,
        })
}
