use std::{marker::PhantomData};

use gc_arena::{Collect, Gc, Mutation, lock::{GcRefLock, RefLock}};

use crate::script::vm::object::{ObjPtr, ObjectMut};



#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjClass<'gc> {
    pub name: String,
    p:        PhantomData<&'gc ()>
}

impl<'gc> ObjClass<'gc> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            p: PhantomData
        }
    }
}

// TODO: Macro this
impl<'gc> ObjectMut<'gc> {
    pub fn new_class(name: String, ctx: &Mutation<'gc>) -> Self {
        ObjectMut::Class(
            Gc::new(
                ctx,
                RefLock::new(
                    ObjClass::new(name)
                )
            )
        )
    }

    pub fn to_class(&self) -> Option<GcRefLock<'gc, ObjClass<'gc>>> {
        match self {
            ObjectMut::Class(class) => Some(*class),
            _                       => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_class(name: String, ctx: &Mutation<'gc>) -> Self {
        ObjPtr::ObjMut(ObjectMut::new_class(name, ctx))
    }


    pub fn to_class(&self) -> Option<GcRefLock<'gc, ObjClass<'gc>>> {
        match self {
            ObjPtr::Obj   (_)   => None,
            ObjPtr::ObjMut(obj) => obj.to_class()
        }
    }
}
