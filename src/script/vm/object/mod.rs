use std::cell::{Cell, Ref, RefMut};

use ast_macro::{ObjTryFrom};
use gc_arena::{Collect, Gc, lock::RefLock};

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

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct Obj<'gc> {
    pub id:    usize,
    pub type_: ObjType<'gc>,
}

pub type ObjPtrWritable<'gc> = Gc<'gc, RefLock<Obj<'gc>>>;
pub type ObjPtr        <'gc> = Gc<'gc, Obj<'gc>>;

#[derive(Debug, Clone, Collect, ObjTryFrom)]
#[collect(no_drop)]
pub enum ObjType<'gc> {
    String  (ObjString),
    Function(ObjFunction<'gc>),
    NativeFn(ObjNative  <'gc>),
    Class   (ObjClass),
    Instance(ObjInstance<'gc>),
    Closure (ObjClosure <'gc>),
    Value   (ObjValue   <'gc>),
}

const ID: Cell<usize> = Cell::new(0);

impl<'gc> Obj<'gc> {
    pub fn new(type_: ObjType<'gc>) -> Obj<'gc> {
        let id = ID.get();
        ID.set(id +1);

        Self {
            id,
            type_,
        }
    }

    pub fn as_string(&self) -> String {
        match &self.type_ {
            ObjType::String  (str)   => str.string.clone(),
            ObjType::Function(func)  => format!("<fn {}>",        func .name),
            ObjType::NativeFn(func)  => format!("<native fn {}>", func .name),
            ObjType::Class   (class) => format!("<class {}>",     class.name),
            ObjType::Instance(inst)  => format!("<{} instance>",  inst.as_str()),
            ObjType::Closure (func)  => format!("<closure {}>",   func.as_str()),
            ObjType::Value   (val)   => format!("{}",             val.value.display()),
        }
    }
}

impl<'gc> PartialEq for Obj<'gc> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.type_, &other.type_) {
            (ObjType::String  (a), ObjType::String  (b)) => a.string == b.string,
            (ObjType::Function(a), ObjType::Function(b)) => a == b,
            _                                            => false,
        }
    }
}

impl<'gc> Eq for Obj<'gc> {}

impl<'gc> ObjInstance<'gc> {
    fn as_str<'a>(&'a self) -> &'a str {
        &self.class.name
    }
}

impl<'gc> ObjClosure<'gc> {
    fn as_str<'a>(&'a self) -> &'a str {
        &self.function.name
    }
}
