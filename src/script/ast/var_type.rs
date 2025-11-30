use std::fmt::Display;

use crate::script::vm::chunk::{StackOffset, UpvalueIndex};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    Global,
    Upvalue(UpvalueIndex),
    Local  (StackOffset),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarDeclType {
    Global,
    Upvalue,
    Local,
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

impl Display for VarDeclType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            VarDeclType::Global  => "Global Decl",
            VarDeclType::Upvalue => "Upvalue Decl",
            VarDeclType::Local   => "Local Decl",
        })
    }
}
