use gc_arena::Collect;

use crate::script::vm::object::{Obj, ObjString};



#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjClass {
    pub name: String,
}

impl ObjClass {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}
impl<'gc> Obj<'gc> {
    pub fn new_class(name: String) -> Obj<'gc> {
        Obj::new(ObjString::new(name).into())
    }
}
