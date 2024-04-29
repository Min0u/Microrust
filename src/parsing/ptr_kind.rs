#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtrKind {
    RawPtr,
    Box,
    Rc,
}


use std::fmt::Display;

impl Display for PtrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PtrKind::*;
        match self {
            RawPtr => write!(f, "RawPtr"),
            Box => write!(f, "Box"),
            Rc => write!(f, "Rc"),
        }
    }
}