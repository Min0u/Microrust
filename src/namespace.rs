use std::collections::HashMap;
use crate::error::EvalError;
use crate::identifier::Identifier;
use crate::memorycell::MemoryCell;
use crate::parsing::expression::Expression;
use crate::parsing::value::Value;

#[derive(Debug, Clone)]
pub struct NameSpace(HashMap<Identifier, MemoryCell>);



impl NameSpace {
    pub fn new() -> Self {
        NameSpace(HashMap::new())
    }

    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> {
//        self.0.try_insert(id, value).map_err(|_| EvalError::AlreadyDefined(id))
        if self.0.contains_key(&id) {
            Err(EvalError::AlreadyDefined(id.clone()))
        } else {
            self.0.insert(id.clone(), MemoryCell::new(mutable, value));
            Ok(())
        }
    }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        match self.0.get(id) {
            // get_value
            Some(get_value) => get_value.get_value().map(|v| v.clone()),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        match self.0.get_mut(id) {
            //appelle la fonction is_mutable et get_value
            Some(memoryCell) => {
                let mutable = memoryCell.is_mutable();
                if !mutable {
                    return Err(EvalError::NotMutable(Some(Expression::Identifier(id.clone()))));
                }
                let _ = memoryCell.set_value(value);
                Ok(())

            },
            None => Err(EvalError::Undefined(id.clone())),
        }
    }
}
