use serde::{ser::SerializeMap, Serialize};

use crate::blambda::syntax::{BinOp, Expr, Program, UnOp};

use super::error::BlambdaError;

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

trait BlambdaRepr {
    fn blambda_repr(&self) -> String;
}

impl BlambdaRepr for bool {
    fn blambda_repr(&self) -> String {
        format!("{}", if *self { "t" } else { "f" })
    }
}

impl BlambdaRepr for UnOp {
    fn blambda_repr(&self) -> String {
        match self {
            UnOp::Not => "~".to_string(),
        }
    }
}

impl BlambdaRepr for BinOp {
    fn blambda_repr(&self) -> String {
        match self {
            BinOp::Or => "|".to_string(),
            BinOp::And => "&".to_string(),
            BinOp::Condition => "?".to_string(),
            BinOp::Branch => ":".to_string(),
        }
    }
}

pub fn serialize_expr(expr: &Expr) -> Result<String, BlambdaError> {
    match expr {
        Expr::Value(v) => Ok(v.blambda_repr()),
        Expr::Unary { op, arg } => {
            let arg = serialize_expr(arg)?;
            Ok(format!("({} {})", op.blambda_repr(), arg))
        }
        Expr::Binary { op, arg1, arg2 } => {
            let arg1 = serialize_expr(arg1)?;
            let arg2 = serialize_expr(arg2)?;
            Ok(format!("({} {} {})", arg1, op.blambda_repr(), arg2))
        }
    }
}

pub fn serialize_program(program: &Program) -> Result<String, BlambdaError> {
    let mut exprs = Vec::new();
    for expr in &program.exprs {
        exprs.push(serialize_expr(expr)?);
    }
    Ok(format!(
        "{}",
        &program
            .exprs
            .iter()
            .flat_map(|expr| serialize_expr(expr))
            .collect::<Vec<String>>()
            .join(" ")
    ))
}
