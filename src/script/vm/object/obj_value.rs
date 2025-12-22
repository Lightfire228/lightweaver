use gc_arena::{Collect, Gc, Mutation, lock::{GcRefLock, RefLock}};

use crate::script::vm::{object::{ObjPtr, ObjectMut}, value::Value};


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


// TODO: Macro this
impl<'gc> ObjectMut<'gc> {
    pub fn new_value(value: Value<'gc>, ctx: &Mutation<'gc>) -> Self {
        ObjectMut::Value(
            Gc::new(
                ctx,
                RefLock::new(
                    ObjValue::new(value)
                )
            )
        )
    }

    pub fn to_value(&self) -> Option<GcRefLock<'gc, ObjValue<'gc>>> {
        match self {
            ObjectMut::Value(value) => Some(*value),
            _                       => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_value(value: Value<'gc>, ctx: &Mutation<'gc>) -> Self {
        ObjPtr::ObjMut(ObjectMut::new_value(value, ctx))
    }

    pub fn to_value(&self) -> Option<GcRefLock<'gc, ObjValue<'gc>>> {
        match self {
            ObjPtr::Obj   (_)   => None,
            ObjPtr::ObjMut(obj) => Some(obj.to_value()?)
        }

    }
}
