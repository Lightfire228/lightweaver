use std::cell::RefMut;

use gc_arena::{Collect, Gc, Mutation, lock::RefLock};

use crate::script::vm::{object::{Obj, ObjFunction, ObjectMut}, value::Value};


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

    pub fn to_closure(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjClosure>> {
        match self {
            ObjectMut::Closure(class) => Some(class.borrow_mut(ctx)),
            _                       => None,
        }
    }
}

impl<'gc> Obj<'gc> {
    pub fn new_closure(name: String, ctx: &Mutation<'gc>) -> Self {
        Obj::ObjMut(ObjectMut::new_class(name, ctx))
    }



    pub fn to_closure(&'gc self, ctx: &Mutation<'gc>) -> Option<RefMut<ObjClosure>> {
        match self {
            Obj::Obj   (_)   => None,
            Obj::ObjMut(obj) => obj.to_closure(ctx)
        }

    }
}
