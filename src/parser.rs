use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ParseError {
    CannotParse,
    SyntaxNotSupported,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        match self {
            CannotParse => write!(f, "Cannot parse"),
            SyntaxNotSupported => write!(f, "Syntax not supported"),
        }
    }
}

pub trait Parse {
    fn parse(input: &str) -> Result<Self, ParseError> where Self: Sized;
}