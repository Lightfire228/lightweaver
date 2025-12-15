
use gc_arena::{Collect, Gc};

use crate::script::vm::{value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFn<'gc>(fn(&[Value<'gc>]) -> Value<'gc>);

#[derive(Debug, Clone, Eq, Collect)]
#[collect(no_drop)]
pub struct ObjNative<'gc> {
    pub func: NativeFn<'gc>,
    pub name: String,
}


impl<'gc> ObjNative<'gc> {
    pub fn new(name: String, func: NativeFn<'gc>) -> Self {
        Self {
            func,
            name,
        }
    }
}

impl<'gc> PartialEq for ObjNative<'gc> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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
