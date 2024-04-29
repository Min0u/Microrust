#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Address,
}

use std::fmt::{self, Display};

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Type::*;
        match self {
            Int => write!(f, "isize"),
            Bool => write!(f, "bool"),
            Unit => write!(f, "unit"),
            Address => write!(f, "address"),
        }
    }
}

