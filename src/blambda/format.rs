use crate::blambda::serde_ast;
use crate::blambda::syntax::{Expr, Program};

pub fn format_expr(expr: &Expr) -> String {
    serde_ast::serialize_expr(expr).unwrap()
}

pub fn format_program(program: &Program) -> String {
    serde_ast::serialize_program(program).unwrap()
}
