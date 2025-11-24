use crate::script::vm::{gc::ObjectId, value::Value};


#[derive(Debug, Clone)]
pub struct ObjClosure {
    pub arity:       usize,
    pub function:    ObjectId,
    pub closed_vals: Vec<Value>,
}



impl ObjClosure {
    pub fn new(function: ObjectId, arity: usize, closed_vals: Vec<Value>) -> Self {
        Self {
            arity,
            function,
            closed_vals,
        }
    }
}
