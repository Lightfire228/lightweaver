use std::marker::PhantomData;

use gc_arena::{Arena, Collect, Gc, Mutation, Rootable, lock::RefLock};

use crate::script::vm::{chunk::Chunk};


#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjFunction<'gc> {
    pub arity: usize,
    pub chunk: Gc<'gc, Chunk<'gc>>,
    pub name:  String,
}



impl<'gc> ObjFunction<'gc> {
    pub fn new(name: String, arity: usize, chunk: Gc<'gc, Chunk<'gc>>) -> Self {

        Self {
            arity,
            chunk,
            name:  name,
        }
    }
}



impl<'gc> Eq for ObjFunction<'gc> {}

impl<'gc> PartialEq for ObjFunction<'gc> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
