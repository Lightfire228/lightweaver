use std::marker::PhantomData;

use gc_arena::{Arena, Collect, Gc, Mutation, Rootable};

use crate::script::vm::{chunk::Chunk};


#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjFunction<'gc> {
    pub arity: usize,
    pub chunk: Gc<'gc, Chunk<'gc>>,
    pub name:  String,
}



impl<'gc> ObjFunction<'gc> {
    pub fn new(name: String, arity: usize, ctx: &'gc Mutation<'gc>) -> Gc<'gc, Self> {

        let chunk = Gc::new(ctx, Chunk::new(name.clone()));

        Gc::new(ctx, Self {
            arity,
            chunk,
            name:  name,
        })
    }
}

impl<'gc> Eq for ObjFunction<'gc> {}

impl<'gc> PartialEq for ObjFunction<'gc> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
