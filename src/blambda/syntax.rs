use std::sync::Arc;

use serde::{ser::SerializeMap, Serialize};

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

impl Serialize for Expr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Expr::Value(v) => serializer.serialize_bool(*v),
            Expr::Unary { op, arg } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("op", op)?;
                map.serialize_entry("arg", arg)?;
                map.end()
            }
            Expr::Binary { op, arg1, arg2 } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("op", op)?;
                map.serialize_entry("arg1", arg1)?;
                map.serialize_entry("arg2", arg2)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub exprs: Vec<Expr>,
}

impl Serialize for Program {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("exprs", &self.exprs)?;
        map.end()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnOp {
    Not,
}

impl Serialize for UnOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UnOp::Not => serializer.serialize_str("not"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinOp {
    Or,
    And,
    Condition,
    Branch,
}

impl Serialize for BinOp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BinOp::Or => serializer.serialize_str("or"),
            BinOp::And => serializer.serialize_str("and"),
            BinOp::Condition => serializer.serialize_str("condition"),
            BinOp::Branch => serializer.serialize_str("branch"),
        }
    }
}
