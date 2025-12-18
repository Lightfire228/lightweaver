use gc_arena::Collect;

use crate::script::vm::{object::Obj, value::Value};


#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjValue<'gc> {
    pub value: Value<'gc>,
}



impl<'gc> ObjValue<'gc> {
    pub fn new(value: Value<'gc>) -> Self {
        Self {
            value,
        }
    }
}

impl<'gc> Obj<'gc> {
    pub fn new_value(value: Value<'gc>) -> Obj<'gc> {
        Obj::new(ObjValue::new(value).into())
    }
}
