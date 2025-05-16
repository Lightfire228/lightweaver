use super::*;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub stmts: Box::<Vec<Stmt>>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Stmt {
        Stmt::Block(Block {
            stmts: Box::new(stmts),
        })
    }
}
