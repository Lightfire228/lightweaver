use crate::script::vm::object::ObjType;


#[derive(Debug, Clone)]
pub struct ObjClass {
    pub name: String,
}

impl ObjClass {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

impl From<ObjClass> for ObjType {
    fn from(value: ObjClass) -> Self {
        Self::Class(value)
    }
}
