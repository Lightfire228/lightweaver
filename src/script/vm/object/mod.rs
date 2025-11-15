use std::{fmt::Display};

use crate::script::vm::gc::ObjectId;

mod obj_native;
mod obj_string;
mod obj_function;

pub use obj_native  ::*;
pub use obj_string  ::*;
pub use obj_function::*;

#[derive(Debug, Clone)]
pub struct Obj {
    pub id:    ObjectId,
    pub type_: ObjType,
}

#[derive(Debug, Clone)]
pub enum ObjType {
    String  (ObjString),
    Function(ObjFunction),
    NativeFn(ObjNative)
}


impl Obj {
    pub fn new(type_: ObjType, id: ObjectId) -> Obj {
        Self {
            id,
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
            ObjType::String  (str)  => str.string.clone(),
            ObjType::Function(func) => format!("<fn {}>",        func.name),
            ObjType::NativeFn(func) => format!("<native fn {}>", func.name),
        })
    }
}
