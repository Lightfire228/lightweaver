
use gc_arena::{Collect, Gc, Mutation};

use crate::script::vm::{object::{Obj, Object}, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFn<'gc>(pub fn(&[Value<'gc>]) -> Value<'gc>);

#[derive(Debug, Clone, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub struct ObjNativeFn<'gc> {
    pub func: NativeFn<'gc>,
    pub name: String,
}


impl<'gc> ObjNativeFn<'gc> {
    pub fn new(name: String, func: NativeFn<'gc>) -> Self {
        Self {
            func,
            name,
        }
    }
}

// TODO: Macro this
impl<'gc> Object<'gc> {
    pub fn new_native_fn(name: String, func: NativeFn<'gc>, ctx: &Mutation<'gc>) -> Self {
        Object::NativeFn(Gc::new(ctx, ObjNativeFn::new(name, func)))
    }

    pub fn to_native_fn(&'gc self) -> Option<&ObjNativeFn> {
        match self {
            Object::NativeFn(func) => Some(func),
            _                      => None,
        }

    }
}

impl<'gc> Obj<'gc> {
    pub fn new_native_fn(name: String, func: NativeFn<'gc>, ctx: &Mutation<'gc>) -> Self {
        Obj::Obj(Object::new_native_fn(name, func, ctx))
    }

    pub fn to_native_fn(&'gc self) -> Option<&ObjNativeFn> {
        match self {
            Obj::Obj   (obj) => Some(obj.to_native_fn()?),
            Obj::ObjMut(_)   => None
        }
    }
}

// SAFETY: It's not possible for this function to squirrel away a GC reference outside of
//         the current GC mutation, because of lifetime branding.
//         And it's not possible for collection to run during function execution, because a mutation
//         immutably borrows the GC context.
unsafe impl<'gc> Collect for NativeFn<'gc> {
    #[inline]
    fn needs_trace() -> bool where Self: Sized {
        false
    }
}
