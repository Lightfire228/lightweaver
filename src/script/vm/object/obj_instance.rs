use std::collections::HashMap;

use gc_arena::{Collect, Gc, Mutation};

use crate::script::vm::{object::{Obj, ObjClass, ObjType}, value::Value};

pub type Fields<'gc> = HashMap<String, Value<'gc>>;

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjInstance<'gc> {
    pub class:  Gc<'gc, ObjClass>,
    pub fields: Gc<'gc, Fields<'gc>>,
}

impl<'gc> ObjInstance<'gc> {
    pub fn new(class: Gc<'gc, ObjClass>, ctx: &Mutation<'gc>) -> Self {
        Self {
            class,
            fields: Gc::new(ctx, HashMap::new()),
        }
    }
}


impl<'gc> Obj<'gc> {
    pub fn to_instance(&self) -> Option<&'gc ObjInstance> {
        type T<'a> = ObjType<'a>;

        match &self.type_ {
            T::Instance(inst) => Some(inst),
            _                 => None,
        }
    }

    pub fn to_instance_mut(&mut self) -> Option<&'gc mut ObjInstance> {
        type T<'a> = ObjType<'a>;

        match &mut self.type_ {
            T::Instance(inst) => Some(inst),
            _                 => None,
        }
    }
}
