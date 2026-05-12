use std::{cell::Cell, fmt::{Display}};

use gc_arena::{Collect, Gc, lock::{GcRefLock}};

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


#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum ObjPtr<'gc> {
    Obj   (Object   <'gc>),
    ObjMut(ObjectMut<'gc>),
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum Object<'gc> {
    String  (Gc<'gc, ObjString>),
    Function(Gc<'gc, ObjFunction<'gc>>),
    NativeFn(Gc<'gc, ObjNativeFn<'gc>>),
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum ObjectMut<'gc> {
    Class   (GcRefLock<'gc, ObjClass   <'gc>>),
    Instance(GcRefLock<'gc, ObjInstance<'gc>>),
    Closure (GcRefLock<'gc, ObjClosure <'gc>>),
    Value   (GcRefLock<'gc, ObjValue   <'gc>>),
}



const ID: Cell<usize> = Cell::new(0);



impl<'gc> Display for ObjPtr<'gc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ObjPtr::Obj   (obj) => obj.fmt(f),
            ObjPtr::ObjMut(obj) => obj.fmt(f),
        }
    }
}


impl<'gc> Display for Object<'gc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Object::String  (str)   => write!(f, "{}",             str.string),
            Object::Function(func)  => write!(f, "<fn {}>",        func .name),
            Object::NativeFn(func)  => write!(f, "<native fn {}>", func .name),
        }
    }
}

impl<'gc> Display for ObjectMut<'gc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ObjectMut::Class   (class) => write!(f, "<class {}>",     class.borrow().name),
            ObjectMut::Instance(inst)  => write!(f, "<{} instance>",  inst .borrow().class.borrow().name),
            ObjectMut::Closure (func)  => write!(f, "<closure {}>",   func .borrow().function.name),
            ObjectMut::Value   (val)   => write!(f, "{}",             val  .borrow().value),
        }
    }
}
