use crate::script::vm::object::{Obj, ObjType};


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

impl TryFrom<Obj> for ObjClass {
    type Error = ();

    fn try_from(obj: Obj) -> Result<Self, Self::Error> {
        match obj.type_ {
            ObjType::Class(class) => Ok(class),
            _                     => Err(())
        }
    }
}

impl<'a> TryFrom<&'a Obj> for &'a ObjClass {
    type Error = ();

    fn try_from(obj: &'a Obj) -> Result<Self, Self::Error> {
        match &obj.type_ {
            ObjType::Class(class) => Ok(class),
            _                     => Err(())
        }
    }
}
