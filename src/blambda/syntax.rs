use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    // Terminal values
    Value(bool),
    // Unary operator(s)
    Unary {
        op: UnOp,
        arg: Arc<Expr>,
    },
    // Binary operator(s)
    Binary {
        op: BinOp,
        arg1: Arc<Expr>,
        arg2: Arc<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub exprs: Vec<Expr>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnOp {
    Not,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinOp {
    Or,
    And,
    Condition,
    Branch,
}
