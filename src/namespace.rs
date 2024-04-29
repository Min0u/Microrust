use std::collections::HashMap;
use crate::error::EvalError;
use crate::identifier::Identifier;
use crate::parsing::expression::Expression;
use crate::parsing::value::Value;

#[derive(Debug, Clone)]
pub struct NameSpace (
    HashMap<Identifier, (bool, Value)>,
);

impl NameSpace {
    pub fn new() -> Self {
        NameSpace(HashMap::new())
    }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        match self.0.get(id) {
            Some((_, v)) => Ok(v.clone()),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> {
        if self.0.contains_key(&id) {
            Err(EvalError::AlreadyDefined(id.clone()))
        } else {
            self.0.insert(id.clone(), (mutable, value));
            Ok(())
        }
    }

    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        match self.0.get_mut(id) {
            Some((true, v)) => {
                *v = value;
                Ok(())
            },
            Some((false, _)) => Err(EvalError::NotMutable(Some(Expression::Identifier(id.clone())))),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }
}