use ast_macro::{ObjTryFrom};

use crate::script::vm::gc::{Context, ObjectId};

mod obj_native;
mod obj_string;
mod obj_function;
mod obj_class;
mod obj_instance;
mod obj_closure;
mod obj_value;

pub use obj_native  ::*;
pub use obj_string  ::*;
pub use obj_function::*;
pub use obj_class   ::*;
pub use obj_instance::*;
pub use obj_closure ::*;
pub use obj_value   ::*;

#[derive(Debug, Clone)]
pub struct Obj {
    pub id:    ObjectId,
    pub type_: ObjType,
}

#[derive(Debug, Clone, ObjTryFrom)]
pub enum ObjType {
    String  (ObjString),
    Function(ObjFunction),
    NativeFn(ObjNative),
    Class   (ObjClass),
    Instance(ObjInstance),
    Closure (ObjClosure),
    Value   (ObjValue),
}


impl Obj {
    pub fn new(type_: ObjType, id: ObjectId) -> Obj {
        Self {
            id,
            type_,
        }
    }

    pub fn as_string(&self, ctx: &Context) -> String {
        match &self.type_ {
            ObjType::String  (str)   => str.string.clone(),
            ObjType::Function(func)  => format!("<fn {}>",        func .name),
            ObjType::NativeFn(func)  => format!("<native fn {}>", func .name),
            ObjType::Class   (class) => format!("<class {}>",     class.name),
            ObjType::Instance(inst)  => format!("<{} instance>",  inst.as_str(ctx)),
            ObjType::Closure (func)  => format!("<closure {}>",   func.as_str(ctx)),
            ObjType::Value   (val)   => format!("{}",             val.value.display(ctx)),
        }
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        match (&self.type_, &other.type_) {
            (ObjType::String  (a), ObjType::String  (b)) => a.string == b.string,
            (ObjType::Function(a), ObjType::Function(b)) => a == b,
            _                                            => false,
        }
    }
}

impl Eq for Obj {}

impl ObjInstance {
    fn as_str<'a>(&'a self, ctx: &'a Context) -> &'a str {
        let class: &ObjClass = ctx.get(self.class).try_into().unwrap();

        &class.name
    }
}

impl ObjClosure {
    fn as_str<'a>(&'a self, ctx: &'a Context) -> &'a str {
        let func: &ObjFunction = ctx.get(self.function).try_into().unwrap();

        &func.name
    }
}
