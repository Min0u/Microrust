use crate::error::EvalError;
use crate::parsing::value::Value;

#[derive(Debug, Clone)]
pub enum MemoryCell {
    NotAllocated,
    AllocatedCell(AllocatedCell),
}

#[derive(Clone, Debug)]
pub struct AllocatedCell {
    mutable: bool,
    value: Option<Value>,
}



// is_mutable
impl MemoryCell {
    pub fn new(mutable: bool, value: Value) -> Self {
        MemoryCell::AllocatedCell(AllocatedCell {
            mutable,
            value: Some(value),
        })
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            MemoryCell::NotAllocated => false,
            MemoryCell::AllocatedCell(cell) => cell.mutable,
        }
    }

    pub fn get_value(&self) -> Result<&Value, EvalError> {
        match self {
            MemoryCell::NotAllocated => Err(EvalError::NonAllocatedCell(None)),
            MemoryCell::AllocatedCell(cell) => match &cell.value {
                Some(value) => Ok(value),
                None => Err(EvalError::NonInitializedValue(None)),
            },
        }
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), EvalError> {
        match self {
            MemoryCell::NotAllocated => Err(EvalError::NonAllocatedCell(None)),
            MemoryCell::AllocatedCell(cell) => {
                if cell.mutable {
                    cell.value = Some(value);
                    Ok(())
                } else {
                    Err(EvalError::NotMutable(None))
                }
            }
        }
    }

    pub fn is_allocated(&self) -> bool {
        match self {
            MemoryCell::NotAllocated => false,
            MemoryCell::AllocatedCell(_) => true,
        }
    }
}

impl AllocatedCell {
    pub fn new(mutable: bool, value: Value) -> Self {
        AllocatedCell {
            mutable,
            value: Some(value),
        }
    }

    pub fn get_value(&self) -> Option<&Value> {
        self.value.as_ref()
    }
}