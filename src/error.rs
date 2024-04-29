use std::fmt::{self, Display};

use crate::parsing::expression::Expression;
use crate::identifier::Identifier;
use crate::memory::Address;
use crate::parser::ParseError;
use crate::r#type::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    ParseError(ParseError),
    EvalError(EvalError),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum EvalError {
    DivisionByZero(Expression),
    Undefined(Identifier),
    AlreadyDefined(Identifier),
    NotMutable(Option<Expression>),
    TypeMismatch{expression: Expression, expected: Type, found: Option<Type>},
    NonAllocatedCell(Option<Expression>),
    NonInitializedValue(Option<Expression>),
    UseAfterFree(Option<Expression>),
    MemoryLeak(Address),
    MovedValue(Option<Expression>),
    CannotMoveOwnedValue(Option<Expression>),
    CannotFreeOwnedValue(Option<Expression>),
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl From<EvalError> for Error {
    fn from(e: EvalError) -> Self {
        Error::EvalError(e)
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EvalError::*;
        match self {
            DivisionByZero(e) => write!(f, "Division by zero, `{}` evaluates to 0", e),
            Undefined(id) => write!(f, "Undefined identifier `{}`.", id,),
            AlreadyDefined(id) => write!(f, "Identifier `{}` already defined.", id),
            NotMutable(e) => write!(f, "Value {}is not mutable.", e.as_ref().map(|e| format!("at `{}` ", e)).unwrap_or("".to_string())),
            TypeMismatch { expression, expected, found} => {
                write!(f, "Type mismatch in expression `{}`. Expected: {}. {}", expression, expected, found.map(|f| format!("Found: {}", f)).unwrap_or("".to_string()))
            },
            NonAllocatedCell(e) => write!(f, "Cell {}is not allocated.", e.as_ref().map(|e| format!("at `{}` ", e)).unwrap_or("".to_string())),
            NonInitializedValue(e) => write!(f, "Value {} is not initialized.", e.as_ref().map(|e| format!("in `{}` ", e)).unwrap_or("".to_string())),
            UseAfterFree(e) => write!(f, "{}use after free.", e.as_ref().map(|e| format!("`{}` is a ", e)).unwrap_or("".to_string())),
            MemoryLeak(a) => write!(f, "leaking {}", a),
            MovedValue(e) => write!(f, "{} has been moved", e.as_ref().map(|e| format!("`{}`", e)).unwrap_or("value".to_string())),
            CannotMoveOwnedValue(e) => write!(f, "cannot move {}, owned value with move semantics", e.as_ref().map(|e| format!("`{}`", e)).unwrap_or("this value".to_string())),
            CannotFreeOwnedValue(e) => write!(f, "cannot free {}, owned value", e.as_ref().map(|e| format!("`{}`", e)).unwrap_or("this value".to_string())),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            ParseError(e) => write!(f, "Parse Error: {}", e),
            EvalError(e) => write!(f, "Evaluation Error: {}", e),
        }
    }
}

impl EvalError {
    pub fn with_expression_info(&self, e: Expression) -> Self {
        use EvalError::*;
        match self {
            NotMutable(None) => NotMutable(Some(e)),
            NonAllocatedCell(None) => NonAllocatedCell(Some(e)),
            NonInitializedValue(None) => NonInitializedValue(Some(e)),
            UseAfterFree(None) => UseAfterFree(Some(e)),
            _ => self.clone(),
        }
    }
}
