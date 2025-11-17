use std::collections::HashMap;

use crate::script::vm::{gc::ObjectId, object::{Obj, ObjType}, value::Value};

pub type Fields = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct ObjInstance {
    pub class:      ObjectId,
    pub class_name: String, //TODO: this is stupid
    pub fields:     Fields,
}

impl ObjInstance {
    pub fn new(class: ObjectId, class_name: String) -> Self {
        Self {
            class,
            class_name,
            fields: HashMap::new(),
        }
    }
}

impl From<ObjInstance> for ObjType {
    fn from(value: ObjInstance) -> Self {
        Self::Instance(value)
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