use gc_arena::Collect;

use crate::script::vm::{value::Value};


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
