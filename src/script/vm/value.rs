use crate::script::vm::gc::{Context, ObjectId};

use super::object::{Obj, ObjType};


#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number(f64),
    Bool  (bool),
    Obj   (ObjectId),
    Nil,
}

impl Value {

    pub fn new_obj(obj_id: ObjectId) -> Self {
        Value::Obj(obj_id)
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

    pub fn as_obj<'a>(&'a self, ctx: &'a Context) -> Option<&'a Obj> {

        match self {
            Value::Obj(o) => {
                Some(ctx.get(*o))
            },
            _             => None,
        }
    }

    pub fn as_obj_mut<'a>(&'a mut self, ctx: &'a mut Context) -> Option<&'a mut Obj> {

        match self {
            Value::Obj(o) => {
                Some(ctx.get_mut(*o))
            },
            _             => None,
        }
    }

    pub fn to_str<'a, 'b: 'a>(&'a self, ctx: &'b Context) -> Option<&'b str> {

        match self {
            Value::Obj(obj_id) => Some({

                let obj = ctx.get(*obj_id);

                match &obj.type_ {
                    ObjType::String  (obj)   => &obj.string,
                    _                        => None?
                }
            }),
            _ => None,
        }
    }

    pub fn is_lw_string(&self, ctx: &Context) -> bool {

        match self {
            Value::Obj(obj_id) => {
                let obj = ctx.get(*obj_id);

                match obj.type_ {
                    ObjType::String(_) => true,
                    _                  => false,
                }
            }
            _                  => false,
        }

    }

    pub fn display(&self, ctx: &Context) -> String {
        match self {
            Value::Obj   (x) => ctx.get(*x).as_string(ctx),
            Value::Number(x) => x.to_string(),
            Value::Bool  (x) => x.to_string(),
            Value::Nil       => "nil".to_owned(),
        }
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
