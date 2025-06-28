use std::{fmt::Display};

use crate::script::vm::{chunk::Chunk, gc::ObjectId};

#[derive(Debug, Clone)]
pub struct Obj {
    pub id:    ObjectId,
    pub type_: ObjType,
}

#[derive(Debug, Clone)]
pub enum ObjType {
    String  (ObjString),
    Function(ObjFunction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjString {
    pub string: String,
}

#[derive(Debug, Clone)]
pub struct ObjFunction {
    pub arity: usize,
    pub chunk: Chunk,
    pub name:  String,
}

impl Obj {
    fn new(type_: ObjType) -> Obj {
        Self {
            id:    usize::MAX.into(),
            type_,
        }
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        match (&self.type_, &other.type_) {
            (ObjType::String  (a), ObjType::String  (b)) => a.string == b.string,
            (ObjType::Function(a), ObjType::Function(b)) => a == b,
            _                                            => false,
        }
    }
}

impl Eq for Obj {}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", match &self.type_ {
            ObjType::String  (str)  => &str.string,
            ObjType::Function(_unc) => "<fn>",
        })
    }
}

impl ObjString {
    pub fn new(string: String) -> ObjString {
        Self {
            string,
        }
    }
}

impl From<ObjString> for Obj {
    fn from(value: ObjString) -> Self {
        Self::new(ObjType::String(value))
    }
}

impl PartialEq for ObjFunction {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for ObjFunction {}
