use std::{cell::{Ref, RefMut}, ops::DerefMut};

use gc_arena::{Collect, Gc, Mutation, lock::RefLock};

use crate::script::vm::{object::{ObjPtr, ObjRef, ObjRefMut}};

use super::object::{Obj, ObjType};


#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    Number(f64),
    Bool  (bool),
    Obj   (ValueObj<'gc>),
    Closed(ObjPtr<'gc>),
    Nil,
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum ValueObj<'gc> {
    Obj   (Gc<'gc, Obj<'gc>>),
    ObjMut(Gc<'gc, RefLock<Obj<'gc>>>),
}

impl<'gc> Value<'gc> {

    pub fn new_obj(obj: Gc<'gc, Obj<'gc>>) -> Self {
        Value::Obj(ValueObj::Obj(obj))
    }

    pub fn new_obj_mut(obj: Gc<'gc, RefLock<Obj<'gc>>>) -> Self {
        Value::Obj(ValueObj::ObjMut(obj))
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

    pub fn as_obj<'a>(&'a self) -> Option<ObjRef<'gc>> {


        match self {
            Value::Obj(o)    => Some(o.borrow()),
            Value::Closed(o) => Some(o.borrow()),
            _                => None,
        }
    }

    pub fn as_obj_mut<'a>(&'a mut self, ctx: &Mutation<'gc>) -> Option<ObjRefMut<'gc>> {

        match self {
            Value::Obj(o)    => Some(o.borrow_mut(ctx)),
            Value::Closed(o) => Some(o.borrow_mut(ctx)),
            _                => None,
        }
    }

    pub fn to_str<'a, 'b>(&'a self) -> Option<&'a str> {

        todo!();
        // match self {
        //     Value::Obj(obj) => Some({

        //         match &obj.type_ {
        //             ObjType::String  (s)   =>  s.string.as_str(),
        //             _                        => None?
        //         }
        //     }),
        //     _ => None,
        // }
    }

    pub fn is_lw_string(&self) -> bool {

        match self {
            Value::Obj(obj) => {

                match obj.borrow().type_ {
                    ObjType::String(_) => true,
                    _                  => false,
                }
            }
            _                  => false,
        }

    }

    pub fn display(&self) -> String {
        match self {
            Value::Obj   (x) => x.borrow().as_string(),
            Value::Closed(x) => x.borrow().as_string(),
            Value::Number(x) => x.to_string(),
            Value::Bool  (x) => x.to_string(),
            Value::Nil       => "nil".to_owned(),
        }
    }

    pub fn display_type(&self) -> String {
        match self {
            Value::Obj   (_) => "Object" .to_owned(),
            Value::Closed(_) => "Closed" .to_owned(),
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
            (Value::Closed(a), Value::Closed(b)) => a == b,
            (Value::Nil,       Value::Nil)       => true,
            _                                    => false,
        }
    }
}
