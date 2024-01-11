use std::sync::Arc;

use crate::blambda::syntax::{BinOp, Expr, Program, UnOp};
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
#[allow(unused_imports)]
use pest::Parser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            // lowest precidence is ternary operator components
            .op(Op::infix(condition, Right) | Op::infix(branch, Left))
            // second-lowest precedence is binary ops
            .op(Op::infix(or, Left) | Op::infix(and, Left))
            // highest precedence is unary op
            .op(Op::prefix(neg))
    };
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar/blambda.pest"]
pub struct BlambdaParser;

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::boolval => Expr::Value(primary.as_str().to_lowercase() == "t"),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, got {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::or => BinOp::Or,
                Rule::and => BinOp::And,
                Rule::condition => BinOp::Condition,
                Rule::branch => BinOp::Branch,
                _ => unreachable!(),
            };
            Expr::Binary {
                op,
                arg1: Arc::new(lhs),
                arg2: Arc::new(rhs),
            }
        })
        .map_prefix(|op, arg| {
            let op = match op.as_rule() {
                Rule::neg => UnOp::Not,
                _ => unreachable!(),
            };
            Expr::Unary {
                op,
                arg: Arc::new(arg),
            }
        })
        .parse(pairs)
}

pub fn parse_program(pairs: Pairs<Rule>) -> Program {
    Program {
        exprs: pairs
            .flat_map(|pair| pair.into_inner())
            .filter(|pair| pair.as_rule() == Rule::expr)
            .map(|pair| parse_expr(pair.into_inner()))
            .collect(),
    }
}
