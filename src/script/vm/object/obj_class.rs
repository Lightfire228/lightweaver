use gc_arena::Collect;



#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ObjClass {
    pub name: String,
}

impl ObjClass {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}
