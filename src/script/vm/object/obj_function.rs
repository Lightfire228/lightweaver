
use gc_arena::{Collect, Gc, Mutation};

use crate::script::vm::{chunk::Chunk, object::{Obj, Object}};


#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjFunction<'gc> {
    pub arity: usize,
    pub chunk: Gc<'gc, Chunk<'gc>>,
    pub name:  String,
}


// TODO: Macro this
impl<'gc> ObjFunction<'gc> {
    pub fn new(name: String, arity: usize, chunk: Gc<'gc, Chunk<'gc>>) -> Self {

        Self {
            arity,
            chunk,
            name:  name,
        }
    }
}


impl<'gc> Object<'gc> {
    pub fn new_func(name: String, arity: usize, chunk: Gc<'gc, Chunk<'gc>>, ctx: &Mutation<'gc>) -> Self {
        Object::Function(Gc::new(ctx, ObjFunction::new(name, arity, chunk)))
    }

    pub fn to_func(&'gc self) -> Option<&ObjFunction> {
        match self {
            Object::Function(func) => Some(func),
            _                      => None,
        }

    }
}

impl<'gc> Obj<'gc> {
    pub fn new_func(name: String, arity: usize, chunk: Gc<'gc, Chunk<'gc>>, ctx: &Mutation<'gc>) -> Self {
        Obj::Obj(Object::new_func(name, arity, chunk, ctx))
    }

    pub fn to_func(&'gc self) -> Option<&ObjFunction> {
        match self {
            Obj::Obj   (obj) => Some(obj.to_func()?),
            Obj::ObjMut(_)   => None
        }
    }
}
