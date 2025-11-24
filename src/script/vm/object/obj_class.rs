

#[derive(Debug, Clone)]
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
