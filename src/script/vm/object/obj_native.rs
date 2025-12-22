
use gc_arena::{Collect, Gc, Mutation};

use crate::script::vm::{object::{ObjPtr, Object}, value::Value};

#[derive(Debug, Clone)]
pub struct NativeFn<'gc>(pub fn(&[Value<'gc>]) -> Value<'gc>);

#[derive(Debug, Clone, Collect)]
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

    pub fn to_native_fn(&self) -> Option<Gc<'gc, ObjNativeFn<'gc>>> {
        match self {
            Object::NativeFn(func) => Some(*func),
            _                      => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_native_fn(name: String, func: NativeFn<'gc>, ctx: &Mutation<'gc>) -> Self {
        ObjPtr::Obj(Object::new_native_fn(name, func, ctx))
    }

    pub fn to_native_fn(&self) -> Option<Gc<'gc, ObjNativeFn<'gc>>> {
        match self {
            ObjPtr::Obj   (obj) => Some(obj.to_native_fn()?),
            ObjPtr::ObjMut(_)   => None
        }
    }
}

impl<'gc> PartialEq for ObjNativeFn<'gc> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'gc> Eq for ObjNativeFn<'gc> {}

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
