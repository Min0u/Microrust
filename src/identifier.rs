use std::{fmt::{Display, Debug}, rc::Rc};

////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier (Rc<str>);
////////////////////////////////////////////////////////////////////////////


impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier(Rc::from(s))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}