use std::{collections::HashMap};

use gc_arena::{Collect, Gc, Mutation, lock::{GcRefLock, RefLock}};

use crate::script::vm::{object::{ObjPtr, ObjClass, ObjectMut}, value::Value};

pub type Fields<'gc> = HashMap<String, Value<'gc>>;

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjInstance<'gc> {
    pub class:  GcRefLock<'gc, ObjClass<'gc>>,
    pub fields: Fields<'gc>,
}

impl<'gc> ObjInstance<'gc> {
    pub fn new(class: GcRefLock<'gc, ObjClass<'gc>>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
}

// TODO: Macro this
impl<'gc> ObjectMut<'gc> {
    pub fn new_instance(class: GcRefLock<'gc, ObjClass<'gc>>, ctx: &Mutation<'gc>) -> Self {
        ObjectMut::Instance(
            Gc::new(
                ctx,
                RefLock::new(
                    ObjInstance::new(class)
                )
            )
        )
    }

    pub fn to_instance(&self) -> Option<GcRefLock<'gc, ObjInstance<'gc>>> {
        match self {
            ObjectMut::Instance(inst) => Some(*inst),
            _                         => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_instance(class: GcRefLock<'gc, ObjClass<'gc>>, ctx: &Mutation<'gc>) -> Self {
        ObjPtr::ObjMut(ObjectMut::new_instance(class, ctx))
    }


    pub fn to_instance(&self) -> Option<GcRefLock<'gc, ObjInstance<'gc>>> {
        match self {
            ObjPtr::Obj   (_)   => None,
            ObjPtr::ObjMut(obj) => obj.to_instance()
        }

    }
}
