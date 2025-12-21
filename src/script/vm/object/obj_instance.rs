use std::{cell::RefMut, collections::HashMap};

use gc_arena::{Collect, Gc, Mutation, lock::RefLock};

use crate::script::vm::{object::{Obj, ObjClass, ObjectMut}, value::Value};

pub type Fields<'gc> = HashMap<String, Value<'gc>>;

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjInstance<'gc> {
    pub class:  Gc<'gc, ObjClass<'gc>>,
    pub fields: Fields<'gc>,
}

impl<'gc> ObjInstance<'gc> {
    pub fn new(class: Gc<'gc, ObjClass>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
}

// TODO: Macro this
impl<'gc> ObjectMut<'gc> {
    pub fn new_instance(class: Gc<'gc, ObjClass>, ctx: &Mutation<'gc>) -> Self {
        ObjectMut::Instance(
            Gc::new(
                ctx,
                RefLock::new(
                    ObjInstance::new(class)
                )
            )
        )
    }

    pub fn to_instance(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjInstance>> {
        match self {
            ObjectMut::Instance(inst) => Some(inst.borrow_mut(ctx)),
            _                         => None,
        }
    }
}

impl<'gc> Obj<'gc> {
    pub fn new_instance(class: Gc<'gc, ObjClass>, ctx: &Mutation<'gc>) -> Self {
        Obj::ObjMut(ObjectMut::new_instance(class, ctx))
    }



    pub fn to_instance(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjInstance>> {
        match self {
            Obj::Obj   (_)   => None,
            Obj::ObjMut(obj) => obj.to_instance(ctx)
        }

    }
}
