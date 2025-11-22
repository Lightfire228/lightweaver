use crate::script::vm::{gc::ObjectId, object::{ Obj, ObjType}, value::Value};


#[derive(Debug, Clone)]
pub struct ObjClosure {
    pub arity:       usize,
    pub function:    ObjectId,
    pub closed_vals: Vec<Value>,
}



impl ObjClosure {
    pub fn new(function: ObjectId, arity: usize, closed_vals: Vec<Value>) -> Self {
        Self {
            arity,
            function,
            closed_vals,
        }
    }
}

impl From<ObjClosure> for ObjType {
    fn from(value: ObjClosure) -> Self {
        ObjType::Closure(value)
    }
}


impl<'a> From<&'a Obj> for &'a ObjClosure {
    fn from(value: &'a Obj) -> Self {
        match &value.type_ {
            ObjType::Closure(obj) => &obj,
            _                     => panic!("Unable to cast {:?} as Closure", value.type_)
        }
    }
}

impl<'a> From<&'a mut Obj> for &'a mut ObjClosure {
    fn from(value: &'a mut Obj) -> Self {
        let typename = format!("{:?}", &value.type_);

        match &mut value.type_ {
            ObjType::Closure(obj) => obj,
            _                     => panic!("Unable to cast {typename} as Closure")
        }
    }
}
