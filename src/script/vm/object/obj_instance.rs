use std::collections::HashMap;

use crate::script::vm::{gc::ObjectId, object::ObjType};

pub type Fields = HashMap<String, ObjectId>;

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
