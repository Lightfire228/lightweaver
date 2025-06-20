use std::{cell::RefMut, fmt::Display};

use crate::script::vm::object::ObjRef;

use super::object::{Obj, ObjString, ObjType};


#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool  (bool),
    Obj   (ObjRef),
    Nil,
}

impl Value {

    pub fn new_string(string: String) -> Self {
        Value::Obj(ObjString::new(string))
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(x) => Some(*x),
            _                => None,
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil       => true,
            Value::Bool  (x) => !(*x),
            _                => false,
        }
    }

    pub fn as_obj(&self) -> Option<ObjRef> {
        match self {
            Value::Obj(o) => Some(o.clone()),
            _             => None,
        }
    }

    pub fn as_obj_mut(&mut self) -> Option<RefMut<Obj>> {
        match self {
            Value::Obj(o) => Some(o.borrow_mut()),
            _             => None,
        }
    }

    pub fn as_obj_string(&self) -> Option<ObjString> {
        match self {
            Value::Obj(obj) => Some({

                match &(*obj).borrow().type_ {
                    ObjType::String(obj) => obj.clone()
                }

            }),
            _             => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::Obj(obj) => Some({

                match &obj.as_ref().borrow().type_ {
                    ObjType::String(obj) => obj.string.clone()
                }
            }),
            _             => None,
        }
    }

    pub fn is_string(&self) -> bool {

        if let Value::Obj(obj) = &self {
            #[allow(irrefutable_let_patterns)]
            if let ObjType::String(_) = obj.as_ref().borrow().type_ {
                return true;
            }
        }

        false

    }
}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_str(self))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool  (a), Value::Bool  (b)) => a == b,
            (Value::Obj   (a), Value::Obj   (b)) => a == b,
            (Value::Nil,       Value::Nil)       => true,
            _                                    => false,
        }
    }
}

fn to_str(value: &Value) -> String {
    match value {
        Value::Obj   (x) => x.as_ref().borrow().to_string(),
        Value::Number(x) => x.to_string(),
        Value::Bool  (x) => x.to_string(),
        Value::Nil       => "nil".to_owned(),

    }
}
