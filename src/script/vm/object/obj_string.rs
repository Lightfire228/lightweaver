use gc_arena::Collect;

use crate::script::vm::object::{Obj, ObjType};


#[derive(Debug, Clone, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub struct ObjString {
    pub string: String,
}


impl ObjString {
    pub fn new(string: String) -> ObjString {
        Self {
            string,
        }
    }
}

impl<'gc> Obj<'gc> {
    pub fn new_string(string: String) -> Obj<'gc> {
        Obj::new(ObjString::new(string).into())
    }
}

impl<'gc> From<String> for ObjType<'gc> {
    fn from(value: String) -> Self {
        ObjType::String(ObjString::new(value))
    }
}
