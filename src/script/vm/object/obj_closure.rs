use gc_arena::{Collect, Gc, Mutation, lock::{GcRefLock, RefLock}};

use crate::script::vm::{object::{ObjPtr, ObjFunction, ObjectMut}, value::Value};


#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjClosure<'gc> {
    pub arity:       usize,
    pub function:    Gc<'gc, ObjFunction<'gc>>,
    pub closed_vals: Vec<Value<'gc>>,
}


// TODO: Macro this
impl<'gc> ObjClosure<'gc> {
    pub fn new(
        function:    Gc<'gc, ObjFunction<'gc>>,
        arity:       usize,
        closed_vals: Vec<Value<'gc>>
    )
        -> Self
    {
        Self {
            arity,
            function,
            closed_vals,
        }
    }
}

impl<'gc> ObjectMut<'gc> {
    pub fn new_closure(
        function:    Gc<'gc, ObjFunction<'gc>>,
        arity:       usize,
        closed_vals: Vec<Value<'gc>>,
        ctx:         &Mutation<'gc>
    ) -> Self {
        ObjectMut::Closure(
            Gc::new(
                ctx,
                RefLock::new(
                    ObjClosure::new(
                        function,
                        arity,
                        closed_vals
                    )
                )
            )
        )
    }

    pub fn to_closure(&self) -> Option<GcRefLock<'gc, ObjClosure<'gc>>> {
        match self {
            ObjectMut::Closure(class) => Some(*class),
            _                         => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_closure(
        function:    Gc<'gc, ObjFunction<'gc>>,
        arity:       usize,
        closed_vals: Vec<Value<'gc>>,
        ctx:         &Mutation<'gc>
    ) -> Self {
        ObjPtr::ObjMut(ObjectMut::new_closure(function, arity, closed_vals, ctx,))
    }


    pub fn to_closure(&self) -> Option<GcRefLock<'gc, ObjClosure<'gc>>> {
        match self {
            ObjPtr::Obj   (_)   => None,
            ObjPtr::ObjMut(obj) => Some(obj.to_closure()?)
        }

    }
}
