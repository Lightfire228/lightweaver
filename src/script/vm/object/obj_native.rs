
use crate::script::vm::{value::Value};

pub type NativeFn = fn(&[Value]) -> Value;

#[derive(Debug, Clone, Eq)]
pub struct ObjNative {
    pub func: NativeFn,
    pub name: String,
}


impl ObjNative {
    pub fn new(name: String, func: NativeFn) -> Self {
        Self {
            func,
            name,
        }
    }
}

impl PartialEq for ObjNative {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
