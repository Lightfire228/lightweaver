use gc_arena::{Collect, Gc};

use crate::script::vm::{object::ObjFunction, value::Value};


#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjClosure<'gc> {
    pub arity:       usize,
    pub function:    Gc<'gc, ObjFunction<'gc>>,
    pub closed_vals: Vec<Value<'gc>>,
}



impl<'gc> ObjClosure<'gc> {
    pub fn new(function: Gc<'gc, ObjFunction<'gc>>, arity: usize, closed_vals: Vec<Value<'gc>>) -> Self {
        Self {
            arity,
            function,
            closed_vals,
        }
    }
}
