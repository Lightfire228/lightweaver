use std::ops::{Deref, DerefMut};

use crate::script::vm::object::{Obj, ObjString, ObjType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectId(pub usize);

#[derive(Debug)]
pub struct Context {
    id:   ObjectId,
    objs: Vec<Box<Obj>>,
}

impl Context {

    pub fn new() -> Self {
        Self {
            id:    0.into(),
            objs:  vec![],
        }
    }

    pub fn new_obj(&mut self, type_: ObjType) -> ObjectId {

        let id  = self.next_id();
        let obj = Obj::new(type_, id);

        self.objs.push(Box::new(obj));

        id
    }

    pub fn add_string(&mut self, str: &str) -> ObjectId {
        let obj = ObjString::new(str.to_owned()).into();

        self.new_obj(obj)
    }

    pub fn get(&self, id: ObjectId) -> &Obj {
        &self.objs[*id]
    }

    pub fn get_mut(&mut self, id: ObjectId) -> &mut Obj {
        &mut self.objs[*id]
    }

    fn next_id(&mut self) -> ObjectId {
        let id = self.id;
        *self.id += 1;

        id
    }
}


impl From<usize> for ObjectId {
    fn from(value: usize) -> Self {
        ObjectId(value)
    }
}

impl Deref for ObjectId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ObjectId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
