use crate::script::vm::{chunk::Chunk, object::{Obj, ObjType}};


#[derive(Debug, Clone)]
pub struct ObjFunction {
    pub arity: usize,
    pub chunk: Chunk,
    pub name:  String,
}



impl ObjFunction {
    pub fn new(name: String, arity: usize) -> Self {
        Self {
            arity,
            chunk: Chunk::new(name.clone()),
            name:  name,
        }
    }
}

impl From<ObjFunction> for ObjType {
    fn from(value: ObjFunction) -> Self {
        ObjType::Function(value)
    }
}

impl<'a> From<&'a Obj> for &'a ObjFunction {
    fn from(value: &'a Obj) -> Self {
        match &value.type_ {
            ObjType::Function(obj) => &obj,
            _                      => panic!("Unable to cast {:?} as Function", value.type_)
        }
    }
}

impl<'a> From<&'a mut Obj> for &'a mut ObjFunction {
    fn from(value: &'a mut Obj) -> Self {
        let typename = format!("{:?}", &value.type_);

        match &mut value.type_ {
            ObjType::Function(obj) => obj,
            _                      => panic!("Unable to cast {typename} as Function")
        }
    }
}

impl Eq for ObjFunction {}

impl PartialEq for ObjFunction {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
