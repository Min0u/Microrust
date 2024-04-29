use std::fmt::{self, Display};

use crate::identifier::Identifier;

#[derive(Debug, Clone)]
pub enum LeftExpression {
    Identifier(Identifier),
    Star(Box<LeftExpression>),
}

impl Display for LeftExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LeftExpression::*;
        match self {
            Identifier(id) => {write!(f, "{}", id)? },
            Star(lexpr) => {write!(f, "*{}", lexpr)?},
        };
        Ok(())
    }
}
