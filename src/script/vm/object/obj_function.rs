
use gc_arena::{Collect, Gc, Mutation, lock::GcRefLock};

use crate::script::vm::{chunk::Chunk, object::{ObjPtr, Object}};


#[derive(Debug, Clone, Collect, PartialEq, Eq)]
#[collect(no_drop)]
pub struct ObjFunction<'gc> {
    pub arity: usize,
    pub chunk: GcRefLock<'gc, Chunk<'gc>>,
    pub name:  String,
}


// TODO: Macro this
impl<'gc> ObjFunction<'gc> {
    pub fn new(name: String, arity: usize, chunk: GcRefLock<'gc, Chunk<'gc>>) -> Self {

        Self {
            arity,
            chunk,
            name:  name,
        }
    }
}


impl<'gc> Object<'gc> {
    pub fn new_func(name: String, arity: usize, chunk: GcRefLock<'gc, Chunk<'gc>>, ctx: &Mutation<'gc>) -> Self {
        Object::Function(Gc::new(ctx, ObjFunction::new(name, arity, chunk)))
    }

    pub fn to_func(&self) -> Option<Gc<'gc, ObjFunction<'gc>>> {
        match self {
            Object::Function(func) => Some(*func),
            _                      => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_func(name: String, arity: usize, chunk: GcRefLock<'gc, Chunk<'gc>>, ctx: &Mutation<'gc>) -> Self {
        ObjPtr::Obj(Object::new_func(name, arity, chunk, ctx))
    }

    pub fn to_func(&self) -> Option<Gc<'gc, ObjFunction<'gc>>> {
        match self {
            ObjPtr::Obj   (obj) => Some(obj.to_func()?),
            ObjPtr::ObjMut(_)   => None
        }
    }
}
