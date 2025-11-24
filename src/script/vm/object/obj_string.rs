use crate::script::vm::object::ObjType;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjString {
    pub string: String,
}


impl ObjString {
    pub fn new(string: String) -> ObjString {
        Self {
            string,
        }
    }
}

impl From<String> for ObjType {
    fn from(value: String) -> Self {
        ObjType::String(ObjString::new(value))
    }
}
