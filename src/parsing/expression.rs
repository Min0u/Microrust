use super::leftexpression::LeftExpression;
use super::binop::Binop;
use super::ptr_kind::PtrKind;
use super::parsedvalue::ParsedValue;

#[derive(Debug, Clone)]
pub enum Expression {
    Const(ParsedValue),
    Identifier(Identifier),
    NewPtr(PtrKind, Box<Expression>),
    ValueAt(LeftExpression),
    BinOp(Box<Expression>, Binop, Box<Expression>),
    Conditional{
        cond: Box<Expression>,
        cond_true: Box<Expression>,
        cond_false: Box<Expression>,
    },
    Deref(Box<Expression>),
    AmpersAnd(Box<Expression>),
}


use std::fmt::Display;

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            Const(i) => write!(f, "{}", i),
            NewPtr(boxkind, expr) => write!(f, "{}::new({})", boxkind, expr),
            ValueAt(lexpr) => write!(f, "{}", lexpr),
            BinOp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            Conditional { cond, cond_true, cond_false } =>
                write!(f, "({}) ? {}  : {} ", cond, cond_true, cond_false),
            Identifier(i) => write!(f, "{}", i),
            Deref(e) => write!(f, "*{}", e),
            AmpersAnd(e) => write!(f, "&{}", e),
        }
    }
}

impl From<Expression> for Result<Expression, ParseError> {
    fn from(expr: Expression) -> Self {
        match expr {
            Expression::Const(v) =>
                Ok(Expression::Const(v)),
            Expression::ValueAt(LeftExpression::Identifier(id)) =>
                Ok(Expression::Identifier(id.clone())),
            Expression::BinOp(lhs, binop, rhs) => {
                let lhs = Box::new(Self::from(*lhs)?);
                let rhs = Box::new(Self::from(*rhs)?);
                let binop: Result<Binop, ParseError> = Ok(binop);
                Ok(Expression::BinOp(lhs, binop?, rhs))
            },
            Expression::Conditional{ cond, cond_true, cond_false } => {
                Ok(Expression::Conditional {
                    cond: Box::new(Self::from(*cond)?) ,
                    cond_true: Box::new(Self::from(*cond_true)?),
                    cond_false: Box::new(Self::from(*cond_false)?)
                })
            }
            _ => { Err(ParseError::SyntaxNotSupported) }
        }
    }
}

use pest::Parser;
use crate::identifier::Identifier;

use super::utils::{ PestParser, Rule, parse_expr};
use crate::parser::{ Parse, ParseError};


impl Parse for Expression {

    fn parse(input: &str) -> Result<Self, ParseError> {
        match PestParser::parse(Rule::start_rule_expr, &input) {
            Ok(mut pairs) => {
                let first_rule = pairs.next().unwrap();
                match first_rule.as_rule() {
                    Rule::expr => {
                        Ok(parse_expr(first_rule.into_inner()))
                    }
                    _ => { panic!("the grammar is not as expected") }
                }
            },
            Err(_e) => { Err(ParseError::CannotParse) }
        }
    }
}