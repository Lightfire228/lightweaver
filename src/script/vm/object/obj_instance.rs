use std::collections::HashMap;

use gc_arena::{Collect, Gc};

use crate::script::vm::{object::{Obj, ObjClass, ObjType}, value::Value};

pub type Fields<'gc> = HashMap<String, Value<'gc>>;

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjInstance<'gc> {
    pub class:  Gc<'gc, ObjClass>,
    pub fields: Fields<'gc>,
}

impl<'gc> ObjInstance<'gc> {
    pub fn new(class: Gc<'gc, ObjClass>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
}


impl<'gc> Obj<'gc> {
    pub fn new_instance(class: Gc<'gc, ObjClass>) -> Obj<'gc> {
        Obj::new(ObjInstance::new(class).into())
    }


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
