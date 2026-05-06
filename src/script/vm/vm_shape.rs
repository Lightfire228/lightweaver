use std::{fmt::Display, marker::PhantomData};

use gc_arena::Collect;


#[derive(Debug, Clone, Copy, Collect, PartialEq, Eq)]
#[collect(no_drop)]
pub enum VmShape {
    Rect,

}


impl Display for VmShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmShape::Rect => f.write_str("Rect"),
        }
    }
}
