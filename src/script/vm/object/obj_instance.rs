use std::collections::HashMap;

use crate::script::vm::{gc::ObjectId, object::{Obj, ObjType}, value::Value};

pub type Fields = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct ObjInstance {
    pub class:  ObjectId,
    pub fields: Fields,
}

impl ObjInstance {
    pub fn new(class: ObjectId) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
}


impl Obj {
    pub fn to_instance(&self) -> Option<&'_ ObjInstance> {
        type T = ObjType;

        match &self.type_ {
            T::Instance(inst) => Some(inst),
            _                 => None,
        }
    }

    pub fn to_instance_mut(&mut self) -> Option<&'_ mut ObjInstance> {
        type T = ObjType;

        match &mut self.type_ {
            T::Instance(inst) => Some(inst),
            _                 => None,
        }
    }
}
