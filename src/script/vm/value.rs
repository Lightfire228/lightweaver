use std::fmt::Display;

use super::object::{Obj, ObjString, ObjType};


#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool  (bool),
    Obj   (Box<Obj>),
    Nil,
}

impl Value {

    pub fn new_string(string: String) -> Self {
        Value::Obj(Box::new(ObjString::new(string)))
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

    pub fn as_obj(&self) -> Option<&Obj> {
        match self {
            Value::Obj(o) => Some(o),
            _             => None,
        }
    }

    pub fn as_obj_mut(&mut self) -> Option<&mut Obj> {
        match self {
            Value::Obj(o) => Some(o),
            _             => None,
        }
    }

    pub fn as_string(self) -> Option<ObjString> {
        match self {
            Value::Obj(obj) => Some(
                match obj.type_ {
                    ObjType::String(obj) => obj
                }
            ),
            _             => None,
        }
    }

    pub fn is_string(&self) -> bool {

        if let Value::Obj(obj) = &self {
            if let ObjType::String(_) = obj.type_ {
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
        Value::Obj   (x) => x.to_string(),
        Value::Number(x) => x.to_string(),
        Value::Bool  (x) => x.to_string(),
        Value::Nil       => "nil".to_owned(),

    }
}
