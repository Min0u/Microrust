use crate::parsing::expression::Expression;
use crate::parsing::leftexpression::LeftExpression;
use crate::identifier::Identifier;

#[derive(Debug)]
pub enum Instruction {
    Expr(Expression),
    Let{id:Identifier, mutable:bool, expr:Expression},
    Block(Vec<Instruction>),
    IfElse{
        cond: Expression,
        cond_true: Box<Instruction>,
        cond_false: Box<Instruction>,
    },
    WriteAt(LeftExpression, Expression),
    While(Expression, Box<Instruction>),
    Free(LeftExpression),
}

use std::fmt::Display;
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Expr(expr) => write!(f, "{}", expr),
            Let{id, mutable, expr} => {
                if *mutable {
                    write!(f, "let mut {} = {}", id, expr)
                } else {
                    write!(f, "let {} = {}", id, expr)
                }
            },
            Block(instrs) => {
                write!(f, "{{{}}}", instrs.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";"))
            },
            IfElse{cond, cond_true, cond_false} => {
                write!(f, "if {} {} else {}", cond, cond_true, cond_false)
            },
            WriteAt(lexpr, expr) => {
                write!(f, "{} = {}", lexpr, expr)
            },
            While(cond, instr) => {
                write!(f, "while {} {}", cond, instr)
            },
            Free(lexpr) => {
                write!(f, "free {}", lexpr)
            },
        }
    }
}

impl From<Instruction> for Result<Instruction, ParseError> {
    fn from(instr: Instruction) -> Self {
        match instr {
            Instruction::Expr(expr) => Ok(Instruction::Expr(Expression::parse(&expr.to_string())?)),
            Instruction::Let{id, mutable, expr} => {
                Ok(Instruction::Let{id, mutable, expr: Expression::parse(&expr.to_string())?})
            },
            Instruction::Block(instrs) => {
                let instrs: Result<Vec<Instruction>, ParseError> = instrs.into_iter().map(|x| <_>::from(x)).collect();
                Ok(Instruction::Block(instrs?))
            },
            Instruction::WriteAt(lexpr, expr) => {
                Ok(Instruction::WriteAt(lexpr, Expression::parse(&expr.to_string())?))
            },
            Instruction::IfElse{cond, cond_true, cond_false} => {
                let cond = Expression::parse(&cond.to_string())?;
                let cond_true = Box::new(Instruction::parse(&cond_true.to_string())?);
                let cond_false = Box::new(Instruction::parse(&cond_false.to_string())?);
                Ok(Instruction::IfElse{cond, cond_true, cond_false})
            },
            Instruction::While(cond, instr) => {
                let cond = Expression::parse(&cond.to_string())?;
                let instr = Box::new(Instruction::parse(&instr.to_string())?);
                Ok(Instruction::While(cond, instr))
            },
            _ => { Err(ParseError::SyntaxNotSupported) }
        }
    }
}

use pest::Parser;
use crate::parser::{ParseError, Parse};
use super::utils::{PestParser, Rule, parse_instr};

impl Parse for Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match PestParser::parse(Rule::start_rule_instr, &input) {
            Ok(mut pairs) => {
                let first_rule = pairs.next().unwrap();
                match first_rule.as_rule() {
                    Rule::instr => {
                        parse_instr(&mut first_rule.into_inner())
                    }
                    _ => { panic!("the grammar is not as expected") }
                }
            },
            Err(_e) => { Err(ParseError::CannotParse) }
        }
    }
}