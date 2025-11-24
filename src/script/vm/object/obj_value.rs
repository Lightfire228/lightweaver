use crate::script::vm::{object::{ Obj, ObjType}, value::Value};


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

impl From<ObjValue> for ObjType {
    fn from(value: ObjValue) -> Self {
        ObjType::Value(value)
    }
}


impl<'a> From<&'a Obj> for &'a ObjValue {
    fn from(value: &'a Obj) -> Self {
        match &value.type_ {
            ObjType::Value(obj) => &obj,
            _                     => panic!("Unable to cast {:?} as Value", value.type_)
        }
    }
}

impl<'a> From<&'a mut Obj> for &'a mut ObjValue {
    fn from(value: &'a mut Obj) -> Self {
        let typename = format!("{:?}", &value.type_);

        match &mut value.type_ {
            ObjType::Value(obj) => obj,
            _                     => panic!("Unable to cast {typename} as Value")
        }
    }
}
