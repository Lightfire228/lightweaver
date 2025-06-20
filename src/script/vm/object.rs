use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Obj {
    pub type_: ObjType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjType {
    String(ObjString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjString {
    pub string: String,
}

pub type ObjRef = Rc<RefCell<Obj>>;

impl Obj {
    fn new(type_: ObjType) -> ObjRef {
        Rc::new(RefCell::new(Self {
            type_,
        }))
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        match (&self.type_, &other.type_) {
            (ObjType::String(a), ObjType::String(b)) => a.string == b.string
        }
    }
}

impl Eq for Obj {}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", match &self.type_ {
            ObjType::String(str) => &str.string,
        })
    }
}

impl ObjString {
    pub fn new(string: String) -> ObjRef {
        Obj::new(ObjType::String(Self {
            string,
        }))
    }
}
