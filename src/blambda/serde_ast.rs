use serde::{ser::SerializeMap, Serialize};

use crate::blambda::syntax::{BinOp, Expr, Program, UnOp};

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
