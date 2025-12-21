use std::cell::RefMut;

use gc_arena::{Collect, Gc, Mutation, lock::RefLock};

use crate::script::vm::{object::{Obj, ObjectMut}, value::Value};


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

    pub fn to_value(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjValue>> {
        match self {
            ObjectMut::Value(value) => Some(value.borrow_mut(ctx)),
            _                       => None,
        }
    }
}

impl<'gc> Obj<'gc> {
    pub fn new_value(value: Value<'gc>, ctx: &Mutation<'gc>) -> Self {
        Obj::ObjMut(ObjectMut::new_value(value, ctx))
    }



    pub fn to_value(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjValue>> {
        match self {
            Obj::Obj   (_)   => None,
            Obj::ObjMut(obj) => obj.to_value(ctx)
        }

    }
}
