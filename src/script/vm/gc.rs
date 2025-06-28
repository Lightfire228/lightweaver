use crate::script::vm::object::{Obj, ObjString};

#[derive(Debug)]
pub struct Context {
    id:   usize,
    objs: Vec<Obj>,
}

impl Context {

    pub fn new() -> Self {
        Self {
            id:    0,
            objs:  vec![],
        }
    }

    pub fn add(&mut self, mut obj: Obj) -> usize {

        let id = self.next_id();
        obj.id = id;

        self.objs.push(obj);

        id
    }

    pub fn add_string(&mut self, str: &str) -> usize {
        let obj = ObjString::new(str.to_owned()).into();

        self.add(obj)
    }

    pub fn get(&self, id: usize) -> &Obj {
        &self.objs[id]
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Obj {
        &mut self.objs[id]
    }

    fn next_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;

        id
    }
}
