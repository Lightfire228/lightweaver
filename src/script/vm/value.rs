
use std::{fmt::Display};

use gc_arena::{Collect, Gc};

use crate::script::vm::object::{ObjString, ObjectMut};

use super::object::{ObjPtr};


#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    Number(f64),
    Bool  (bool),
    Obj   (ObjPtr<'gc>),
    Nil,
}

impl<'gc> Value<'gc> {

    pub fn new_obj(obj: ObjPtr<'gc>) -> Self {
        Value::Obj(obj)
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
            Value::Number(_) => false,
            Value::Obj   (_) => false,
        }
    }

    pub fn as_obj<'a>(&'a self) -> Option<&'a ObjPtr<'gc>> {
        match self {
            Value::Obj(obj) => Some(obj),
            _               => None,
        }
    }

    pub fn to_obj(self) -> Option<ObjPtr<'gc>> {
        match self {
            Value::Obj(obj) => Some(obj),
            _               => None,
        }
    }

    pub fn as_obj_mut(&'gc mut self) -> Option<&'gc mut ObjectMut<'gc>> {
        let Value ::Obj   (obj) = self else { None? };
        let ObjPtr::ObjMut(obj) = obj  else { None? };

        Some(obj)
    }

    pub fn to_obj_mut(self) -> Option<ObjectMut<'gc>> {
        let Value ::Obj   (obj) = self else { None? };
        let ObjPtr::ObjMut(obj) = obj  else { None? };

        Some(obj)
    }

    pub fn as_str<'a>(&'a self) -> Option<Gc<'gc, ObjString>> {
        let str = self.as_obj()?;
        let str = str.to_string()?;

        Some(str)
    }


    pub fn display_type(&self) -> String {
        match self {
            Value::Obj   (_) => "Object" .to_owned(),
            Value::Number(_) => "Number" .to_owned(),
            Value::Bool  (_) => "Boolean".to_owned(),
            Value::Nil       => "Nil"    .to_owned(),
        }
    }
}


impl<'gc> PartialEq for Value<'gc> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool  (a), Value::Bool  (b)) => a == b,
            (Value::Obj   (_), Value::Obj   (_)) => todo!(),
            (Value::Nil,       Value::Nil)       => true,
            _                                    => false,
        }
    }
}

impl<'gc> Display for Value<'gc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Obj   (x) => x.fmt(f),
            Value::Number(x) => x.fmt(f),
            Value::Bool  (x) => x.fmt(f),
            Value::Nil       => f.write_str("nil"),
        }
    }
}
