use std::fmt::Display;
use crate::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    StackAddress(usize, Identifier),
    HeapAddress(usize)
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::StackAddress(offset, id) => write!(f, "@[{}, {}]", offset, id),
            Address::HeapAddress(offset) => write!(f, "@[{}]", offset),
        }
    }
}