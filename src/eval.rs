use std::ops::Deref;
use crate::Identifier;
use crate::parsing::binop::Binop::*;
use crate::error::EvalError;
use crate::error::EvalError::{DivisionByZero, Undefined};
use crate::parsing::expression::Expression;
use crate::parsing::expression::Expression::*;
use crate::parsing::instruction::Instruction;
use crate::namespace::NameSpace;
use crate::namespacestack::NameSpaceStack;
use crate::parsing::leftexpression::LeftExpression;
use crate::r#type::Type;
use crate::parsing::value::Value;
use crate::parsing::value::Value::Unit;

impl Expression {
    fn eval_and_cast_to_int(&self, nss: &mut NameSpaceStack) -> Result<isize, EvalError> {
        let v = self.eval(nss)?;
        v.to_int()
            .map_err(|_| EvalError::TypeMismatch{
                expression: self.clone(),
                expected: Type::Int,
                found: Some(Type::from(&v))})
    }

    pub fn eval(&self, nss : &mut NameSpaceStack) -> Result<Value, EvalError> {
        match self {
            Const(v) => Ok(Value::from(*v)),
            BinOp(e1, op, e2) => {
                match op {
                    Add => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Integer(v1 + v2))
                    },
                    Sub => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Integer(v1 - v2))
                    },
                    Mul => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Integer(v1 * v2))
                    },
                    Div => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        if v2 == 0 {
                            Err(DivisionByZero(*e2.clone()))
                        } else {
                            Ok(Value::Integer(v1 / v2))
                        }
                    }
                    Mod => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        if v2 == 0 {
                            Err(DivisionByZero(self.clone()))
                        } else {
                            Ok(Value::Integer(v1 % v2))
                        }
                    }
                    Leq => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Boolean(v1 <= v2))
                    }
                    Geq => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Boolean(v1 >= v2))
                    }
                    Lt => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Boolean(v1 < v2))
                    }
                    Gt => {
                        let v1 = e1.eval_and_cast_to_int(nss)?;
                        let v2 = e2.eval_and_cast_to_int(nss)?;
                        Ok(Value::Boolean(v1 > v2))
                    }
                    Eq => {
                        let v1 = e1.eval(nss)?;
                        let v2 = e2.eval(nss)?;
                        match (v1, v2) {
                            (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Boolean(i1 == i2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 == b2)),
                            (Value::Boolean(b), Value::Integer(i)) => Err(EvalError::TypeMismatch{
                                expression: *e2.clone(),
                                expected: Type::Bool,
                                found: Some(Type::Int)
                            }),
                            (Value::Integer(i), Value::Boolean(b)) => Err(EvalError::TypeMismatch{
                                expression: *e1.clone(),
                                expected: Type::Int,
                                found: Some(Type::Bool)
                            }),
                            _ => Err(EvalError::Undefined(Identifier::from("Eq")))
                        }
                    }
                    Neq => {
                        let v1 = e1.eval(nss)?;
                        let v2 = e2.eval(nss)?;
                        match (v1, v2) {
                            (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Boolean(i1 != i2)),
                            (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 != b2)),
                            (Value::Boolean(b), Value::Integer(i)) => Err(EvalError::TypeMismatch{
                                expression: *e2.clone(),
                                expected: Type::Bool,
                                found: Some(Type::Int)
                            }),
                            (Value::Integer(i), Value::Boolean(b)) => Err(EvalError::TypeMismatch{
                                expression: *e1.clone(),
                                expected: Type::Int,
                                found: Some(Type::Bool)
                            }),
                            _ => Err(EvalError::Undefined(Identifier::from("Neq")))
                        }
                    }
                    And => {
                        let v1 = e1.eval(nss)?;
                        let v2 = e2.eval(nss)?;
                        let b1 = v1.to_bool();
                        let b2 = v2.to_bool();
                        match (b1, b2) {
                            (Ok(true), Ok(true)) => Ok(Value::Boolean(true)),
                            (Ok(false), _) => Ok(Value::Boolean(false)),
                            (_, Ok(false)) => Ok(Value::Boolean(false)),
                            _ => Err(EvalError::Undefined(Identifier::from("And")))
                        }
                    }
                    Or => {
                        let v1 = e1.eval(nss)?;
                        let b1 = v1.to_bool();
                        if let Ok(true) = b1 {
                            return Ok(Value::Boolean(true))
                        }
                        let v2 = e2.eval(nss)?;
                        let b2 = v2.to_bool();
                        if let Ok(true) = b2 {
                            Ok(Value::Boolean(true))
                        } else {
                            Ok(Value::Boolean(false))
                        }
                    }
                }
            },
            Identifier(id) => {
                nss.find(id.into())
            }
            Conditional {cond, cond_true, cond_false} => {
                let v = cond.eval(nss)?;
                let b = v.to_bool();
                match b {
                    Ok(true) => cond_true.eval(nss),
                    Ok(false) => cond_false.eval(nss),
                    _ => Err(EvalError::Undefined(Identifier::from("Conditional")))
                }
            }
            ValueAt (lexpr) => {
                eval_lexpr(lexpr, nss)
            }
            NewPtr(boxkind, expr) => {
                todo!()
            }
            Deref(_) => todo!(),
            AmpersAnd(_) => {
                let addr = self.eval_to_address(nss)?;
                Ok(Value::Pointer(addr))
            }
        }
    }
}

fn eval_lexpr(lexpr: &LeftExpression, nss: &mut NameSpaceStack) -> Result<Value, EvalError> {
    match lexpr {
        LeftExpression::Identifier(id) => {
            nss.find(id)
        }
        _ => todo!()
    }
}

impl Instruction {
    pub fn exec(&self, nss: &mut NameSpaceStack) -> Result<(Option<Identifier>, Value), EvalError> {
        match self {
            Instruction::Expr(expr) => {
                let v = expr.eval(nss);
                Ok((None, v?))
            },

            Instruction::Let { id, mutable, expr } => {
                let v = expr.eval(nss)?;
                nss.declare(id, *mutable, v.clone())?;
                Ok((Some(id.clone()), v))
            }

            Instruction::Block(instrs) => {
                let new_nss = NameSpace::new();
                nss.push(new_nss);
                let mut res = Unit;
                for instr in instrs {
                    let (_id, v) = instr.exec(nss).map_err(|err| {nss.pop(); err})?;
                    res = v;
                }
                nss.pop();
                Ok((None, res))
            }
            Instruction::WriteAt(lexpr, expr) => {
                let v = expr.eval(nss)?;
                match lexpr {
                    LeftExpression::Identifier(id) => {
                        nss.set(id, v.clone())?;
                        Ok((None, v))
                    }
                    _ => Err(EvalError::Undefined(Identifier::from("WriteAt")))
                }
            }
            Instruction::IfElse{cond, cond_true, cond_false} => {
                let v = cond.eval(nss)?;
                let b = v.to_bool();
                match b {
                    Ok(true) => cond_true.exec(nss),
                    Ok(false) => cond_false.exec(nss),
                    _ => Err(EvalError::Undefined(Identifier::from("IfElse")))
                }
            }
            Instruction::While(cond, instr) => {
                let mut res = Value::Unit;
                loop {
                    let v = cond.eval(nss)?;
                    let b = v.to_bool();
                    match b {
                        Ok(true) => {
                            let (_id, v) = instr.exec(nss).map_err(|err| {nss.pop(); err})?;
                            res = v;
                        }
                        Ok(false) => break,
                        _ => return Err(EvalError::Undefined(Identifier::from("While")))
                    }
                }
                Ok((None, Unit))
            }
            Instruction::Free(lexpr) => {
                todo!()
            }
            Instruction::Drop(lexpr) => {
                todo!()
            }
        }
    }
}
