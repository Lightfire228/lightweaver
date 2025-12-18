
use gc_arena::{Collect, Gc};

use crate::script::vm::{chunk::Chunk, object::Obj};


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

impl<'gc> Obj<'gc> {
    pub fn new_function(name: String, arity: usize, chunk: Gc<'gc, Chunk<'gc>>) -> Obj<'gc> {
        Obj::new(ObjFunction::new(name, arity, chunk).into())
    }
}


impl<'gc> Eq for ObjFunction<'gc> {}

impl<'gc> PartialEq for ObjFunction<'gc> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
