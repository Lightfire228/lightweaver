
use gc_arena::{Collect, Gc, Mutation};

use crate::script::vm::object::{ObjPtr, Object};


#[derive(Debug, Clone, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub struct ObjString {
    pub string: String,
}


impl ObjString {
    pub fn new(string: String) -> ObjString {
        Self {
            string,
        }
    }
}


// TODO: Macro this
impl<'gc> Object<'gc> {
    pub fn new_string(string: String, ctx: &Mutation<'gc>) -> Self {
        Object::String(Gc::new(ctx, ObjString::new(string)))
    }

    pub fn to_string(&self) -> Option<Gc<'gc, ObjString>> {
        match self {
            Object::String (str) => Some(*str),
            _                    => None,
        }
    }
}

impl<'gc> ObjPtr<'gc> {
    pub fn new_string(string: String, ctx: &Mutation<'gc>) -> Self {
        ObjPtr::Obj(Object::new_string(string, ctx))
    }

    pub fn to_string(&self) -> Option<Gc<'gc, ObjString>> {
        match self {
            ObjPtr::Obj   (obj) => Some(obj.to_string()?),
            ObjPtr::ObjMut(_)   => None
        }
    }
}

// impl<'gc> TryFrom<&'gc Obj<'gc>> for &Object<'gc> {
//     type Error = ();

//     fn try_from(value: &'gc Obj<'gc>) -> Result<Self, Self::Error> {
//         match value {
//             Obj::Obj   (obj) => Ok (obj.as_ref()),
//             Obj::ObjMut(_)   => Err(()),
//         }
//     }
// }

// impl<'gc> Object<'gc> {

//     fn try_from_obj(value: &'gc mut Obj<'gc>, ctx: &Mutation<'gc>) -> Option<RefMut<'gc, ObjectMut<'gc>>> {
//         match value {
//             Obj::Obj   (_)   => None,
//             Obj::ObjMut(obj) => Some(obj.borrow_mut(ctx)),
//         }
//     }
// }
