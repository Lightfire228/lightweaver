use std::fmt::Display;

use crate::script::vm::chunk::{StackIndex, UpvalueIndex};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    Global,
    Upvalue(UpvalueIndex),
    Local  (StackIndex),
}


impl Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            VarType::Global     => "Global",
            VarType::Upvalue(_) => "Upvalue",
            VarType::Local  (_) => "Local",
        })
    }
}
