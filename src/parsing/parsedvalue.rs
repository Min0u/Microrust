use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParsedValue {
    Integer(isize),
    Boolean(bool),
    Unit,
}

impl fmt::Display for ParsedValue {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedValue::Integer(i) => write!(f, "{}", i),
            ParsedValue::Boolean(b) => write!(f, "{}", b),
            ParsedValue::Unit => write!(f, "()"),
        }
    }
}