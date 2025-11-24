use crate::script::vm::{chunk::Chunk};


#[derive(Debug, Clone)]
pub struct ObjFunction {
    pub arity: usize,
    pub chunk: Chunk,
    pub name:  String,
}



impl ObjFunction {
    pub fn new(name: String, arity: usize) -> Self {
        Self {
            arity,
            chunk: Chunk::new(name.clone()),
            name:  name,
        }
    }
}

impl Eq for ObjFunction {}

impl PartialEq for ObjFunction {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
