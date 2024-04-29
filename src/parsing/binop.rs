#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Leq,
    Geq,
    Lt,
    Gt,
    Eq,
    Neq,
    And,
    Or
}

use std::fmt::{self, Display};

impl Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Binop::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Mod => write!(f, "%"),
            Leq => write!(f, "<="),
            Geq => write!(f, ">="),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
        }
    }
}


use pest::Parser;

use super::utils::{PestParser, Rule, parse_binop};
use crate::parser::{Parse, ParseError};


impl Parse for Binop {

    fn parse(input: &str) -> Result<Self, ParseError> {
        match PestParser::parse(Rule::start_rule_binop, &input) {
            Ok(mut pairs) => {
                let first_rule = pairs.next().unwrap();
                match first_rule.as_rule() {
                    Rule::bin_op => {
                        Ok(parse_binop(first_rule.into_inner()))
                    }
                    _ => { panic!("the grammar is not as expected") }
                }                
            },
            Err(_e) => { Err(ParseError::CannotParse) }
        }
    }
}