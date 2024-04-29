use crate::error::EvalError;
use crate::identifier::Identifier;
use crate::memory::Address;
use crate::namespace::NameSpace;
use crate::parsing::value::Value;

#[derive(Debug, Clone)]
pub struct NameSpaceStack { stack: Vec<NameSpace> }

impl NameSpaceStack {
    pub fn new() -> Self {
        NameSpaceStack { stack: vec![] }
    }

    pub(crate) fn push(&mut self, ns: NameSpace) {
        self.stack.push(ns);
    }

    pub(crate) fn pop(&mut self) -> Option<NameSpace> {
        self.stack.pop()
    }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        for ns in self.stack.iter().rev() {
            if let Ok(v) = ns.find(id) {
                return Ok(v);
            }
        }
        Err(EvalError::Undefined(id.clone()))
    }

    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> {
        self.stack.last_mut().unwrap().declare(id, mutable, value)
    }

    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        for ns in self.stack.iter_mut().rev() {
            let res = ns.set(id, value.clone());
            match res {
                Ok(_) => return Ok(()),
                Err(EvalError::Undefined(_)) => (),
                Err(EvalError::NotMutable(_)) => return res,
                Err(EvalError::TypeMismatch{..}) => return res,
                _ => unreachable!(),
            }
        }
        Err(EvalError::Undefined(id.clone()))
    }

    pub fn get_address(&self, id: &Identifier) -> Result<Address, EvalError> {
        for index in  (0..self.stack.len()).rev() {
            if self.stack[index].find(id).is_ok() {
                return Ok(Address::StackAddress(index, id.clone()));
            }
        }
        Err(EvalError::Undefined(id.clone()))

    }
}