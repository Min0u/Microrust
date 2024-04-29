/////////////////////////////////////////////
// see https://pest.rs/book/ and https://pest.rs/book/examples/calculator.html
use pest::iterators::{Pairs, Pair};
use pest::pratt_parser::PrattParser;

#[derive(pest_derive::Parser)]
#[grammar = "parsing/grammar.pest"]
pub struct PestParser;

use super::instruction::Instruction;
use super::expression::Expression;
use super::leftexpression::LeftExpression;
use super::binop::Binop;
use super::ptr_kind::PtrKind;
use super::parsedvalue::ParsedValue;

use crate::identifier::Identifier;
use crate::parser::ParseError;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(AND, Left) | Op::infix(OR, Left))
            .op(Op::infix(EQQUALS, Left) | Op::infix(NEQ, Left) | Op::infix(GEQ, Left) | Op::infix(LEQ, Left) | Op::infix(GREATER, Left) | Op::infix(LOWER, Left))
            .op(Op::infix(ADD, Left) | Op::infix(SUBTRACT, Left))
            .op(Op::infix(MULTIPLY, Left) | Op::infix(DIVIDE, Left) | Op::infix(MODULO, Left))
//            .op(Op::prefix(unary_minus))
    };
}

pub fn parse_lexpr(mut pairs: Pairs<Rule>) -> LeftExpression {
    let first_rule = pairs.next().unwrap();
    match first_rule.as_rule() {
        Rule::identifier => LeftExpression::Identifier(Identifier::from(first_rule.as_str())),
        Rule::deref => LeftExpression::Star(Box::new(parse_lexpr(first_rule.into_inner()))),
        Rule::lexpr => parse_lexpr(first_rule.into_inner()),
        _ => unreachable!()
    }
}


pub fn parse_binop(mut pairs: Pairs<Rule>) -> Binop {
    let first_rule = pairs.next().unwrap();
    parse_binop_rule(first_rule)
}

pub fn parse_binop_rule(first_rule: Pair<'_, Rule>) -> Binop {
    match first_rule.as_rule() {
        Rule::ADD => Binop::Add,
        Rule::SUBTRACT => Binop::Sub,
        Rule::MULTIPLY => Binop::Mul,
        Rule::DIVIDE => Binop::Div,
        Rule::MODULO => Binop::Mod,
        Rule::EQQUALS => Binop::Eq,
        Rule::NEQ => Binop::Neq,
        Rule::GEQ => Binop::Geq,
        Rule::LEQ => Binop::Leq,
        Rule::GREATER => Binop::Gt,
        Rule::LOWER => Binop::Lt,
        Rule::AND => Binop::And,
        Rule::OR => Binop::Or,
        _ => unreachable!()
    }
}

pub fn parse_boxkind(mut pairs: Pairs<Rule>) -> PtrKind {
    parse_boxkind_rule(pairs.next().unwrap())
}

pub fn parse_boxkind_rule(first_rule: Pair<'_, Rule>) -> PtrKind {
    match first_rule.as_rule() {
        Rule::BOX => PtrKind::Box,
        Rule::RC => PtrKind::Rc,
        Rule::RAWPTR => PtrKind::RawPtr,
        _ => unreachable!()
    }
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expression::Const(ParsedValue::Integer(primary.as_str().parse::<isize>().unwrap())),
            Rule::boolean => Expression::Const(ParsedValue::Boolean(match primary.as_str() {
                "true" => true,
                "false" => false,
                _ => unreachable!()
            })),
            Rule::lexpr => Expression::ValueAt(parse_lexpr(primary.into_inner())),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::boxexpr => {
                let mut rules = primary.into_inner();
                let boxkind = parse_boxkind(rules.next().unwrap().into_inner());
                let expr = parse_expr(rules.next().unwrap().into_inner());
                Expression::NewPtr(boxkind, Box::new(expr))
            },
            Rule::atom => parse_expr(primary.into_inner()),
            Rule::conditional_expr => {
                let mut rules = primary.into_inner();
                let cond = Box::new(parse_expr(rules.next().unwrap().into_inner()));
                let cond_true = Box::new(parse_expr(rules.next().unwrap().into_inner()));
                let cond_false = Box::new(parse_expr(rules.next().unwrap().into_inner()));
                Expression::Conditional{cond, cond_true, cond_false}
            },
            Rule::unit => Expression::Const(ParsedValue::Unit),
            rule => unreachable!("parse_expr expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op2 = parse_binop_rule(op);
            Expression::BinOp(Box::new(lhs), op2, Box::new(rhs))
        })
        .parse(pairs)
}

fn parse_block(pairs: &mut Pairs<Rule>) -> Result<Vec<Instruction>, ParseError> {
    let first_rule = pairs.next().unwrap();
    let mut res = vec![];
    match first_rule.as_rule() {
        Rule::empty_block => { },
        Rule::non_empty_block => {
            let mut rules = first_rule.into_inner();
            while let Some(rule) = rules.next(){
                if rule.as_rule() == Rule::instr {
                    let instr = parse_instr(&mut rule.into_inner())?;
                    res.push(instr)
                }
            }
        },
        _ => unreachable!("parse_block expected instrs, found {:?}", first_rule),
    };
    Ok(res)
}

pub fn parse_instr(pairs: &mut Pairs<Rule>) -> Result<Instruction, ParseError> {
    let first_rule = pairs.next().unwrap();
    match first_rule.as_rule() {
        Rule::expr => Ok(Instruction::Expr(parse_expr(first_rule.into_inner()))),
        Rule::let_equals => {
            let mut rules = first_rule.into_inner();
            let id = rules.next().unwrap().as_span().as_str().to_string();
            let id = Identifier::from(id.as_str());
            let expr = rules.next().unwrap().into_inner();
            Ok(Instruction::Let{id, mutable: false, expr: parse_expr(expr) })
        },
        Rule::let_mut_equals => {
            let mut rules = first_rule.into_inner();
            let id = rules.next().unwrap().as_span().as_str().to_string();
            let id = Identifier::from(id.as_str());
            let expr = rules.next().unwrap().into_inner();
            Ok(Instruction::Let{id, mutable: true, expr: parse_expr(expr) })
        },
        Rule::update_instr => {
            let mut rules = first_rule.into_inner();
            let lexpr = parse_lexpr(rules.next().unwrap().into_inner());
            let expr = parse_expr(rules.next().unwrap().into_inner());
            Ok(Instruction::WriteAt(lexpr, expr))
        },
        Rule::if_instr => {
            let mut rules = first_rule.into_inner();
            let cond = parse_expr(rules.next().unwrap().into_inner());
            let cond_true = Box::new(Instruction::Block(parse_block(&mut rules.next().unwrap().into_inner())?));
            let cond_false = Box::new(Instruction::Block(parse_block(&mut rules.next().unwrap().into_inner())?));
            Ok(Instruction::IfElse{cond, cond_true, cond_false})
        },
        Rule::while_instr => {
            let mut rules = first_rule.into_inner();
            let cond = parse_expr(rules.next().unwrap().into_inner());
            let instr = Instruction::Block(parse_block(&mut rules.next().unwrap().into_inner())?);
            Ok(Instruction::While(cond, Box::new(instr)))
        },
        Rule::instrs => {
            Ok(Instruction::Block(parse_block(&mut first_rule.into_inner())?))
        },
        Rule::free_instr => {
            let lexpr = parse_lexpr(first_rule.into_inner());
            Ok(Instruction::Free(lexpr))
        },
        Rule::drop_instr => {
            let lexpr = parse_lexpr(first_rule.into_inner());
            Ok(Instruction::Drop(lexpr))
        },
        _ => unreachable!("parse_instr expected instr, found {:?}", first_rule),
    }
}