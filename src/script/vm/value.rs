use std::{cell::{Ref, RefMut}, ops::{Deref, DerefMut}};

use gc_arena::{Collect, Gc, Mutation, lock::RefLock};

use crate::script::vm::object::{ObjPtr, ObjPtrWritable, ObjString};

use super::object::{Obj, ObjType};


#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    Number(f64),
    Bool  (bool),
    Obj   (Gc<'gc, Obj<'gc>>),
    ObjMut(Gc<'gc, RefLock<Obj<'gc>>>),
    Nil,
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum ObjRef<'gc> {
    Obj   (Gc<'gc, Obj<'gc>>),
    ObjMut(Gc<'gc, RefLock<Obj<'gc>>>),
}

impl<'gc> Value<'gc> {

    pub fn new_obj(obj: Gc<'gc, Obj<'gc>>) -> Self {
        Value::Obj(obj)
    }

    pub fn new_obj_mut(obj: Gc<'gc, RefLock<Obj<'gc>>>) -> Self {
        Value::ObjMut(obj)
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
            Value::ObjMut(_) => false,
        }
    }

    pub fn as_obj(&'gc self) -> Option<&'gc Obj<'gc>> {
        match self {
            Value::Obj(obj) => Some(obj),
            _               => None,
        }
    }

    pub fn as_obj_ref(&'gc self) -> Option<Ref<Obj<'gc>>> {
        match self {
            Value::ObjMut(obj) => Some(obj.borrow()),
            _                  => None,
        }
    }

    pub fn as_obj_mut<'a>(&'a mut self, ctx: &Mutation<'gc>) -> Option<RefMut<Obj<'gc>>> {
        match self {
            Value::ObjMut(obj) => Some(obj.borrow_mut(ctx)),
            _                  => None,
        }
    }

    pub fn to_lw_str<'a, 'b>(&'a self) -> Option<&'a ObjString> {
        let Value::Obj(obj) = self else { None? };

        let ObjType::String(obj) = &obj.type_ else { None? };

        Some(&obj)
    }

    pub fn display(&self) -> String {
        match self {
            Value::Obj   (x) => x.as_string(),
            Value::ObjMut(x) => x.borrow().as_string(),
            Value::Number(x) => x.to_string(),
            Value::Bool  (x) => x.to_string(),
            Value::Nil       => "nil".to_owned(),
        }
    }

    pub fn display_type(&self) -> String {
        match self {
            Value::Obj   (_) => "Object" .to_owned(),
            Value::ObjMut(_) => "Object" .to_owned(),
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
            (Value::Obj   (a), Value::Obj   (b)) => a == b,
            (Value::ObjMut(a), Value::ObjMut(b)) => a == b,
            (Value::Nil,       Value::Nil)       => true,
            _                                    => false,
        }
    }
}
