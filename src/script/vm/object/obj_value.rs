use crate::script::vm::{value::Value};


#[derive(Debug, Clone)]
pub struct ObjValue {
    pub value: Value,
}



impl ObjValue {
    pub fn new(value: Value) -> Self {
        Self {
            value,
        }
    }
}
