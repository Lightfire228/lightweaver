use std::{cell::RefMut, marker::PhantomData};

use gc_arena::{Collect, Gc, Mutation, lock::RefLock};

use crate::script::vm::object::{Obj, ObjectMut};



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

    pub fn to_class(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjClass>> {
        match self {
            ObjectMut::Class(class) => Some(class.borrow_mut(ctx)),
            _                       => None,
        }
    }
}

impl<'gc> Obj<'gc> {
    pub fn new_class(name: String, ctx: &Mutation<'gc>) -> Self {
        Obj::ObjMut(ObjectMut::new_class(name, ctx))
    }



    pub fn to_class(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjClass>> {
        match self {
            Obj::Obj   (_)   => None,
            Obj::ObjMut(obj) => obj.to_class(ctx)
        }

    }
}
