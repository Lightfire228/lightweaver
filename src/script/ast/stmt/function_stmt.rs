use crate::script::tokens::Token;

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionStmt {
    pub name:   Token,
    pub params: Vec<Token>,
    pub body:   Box<Vec<Stmt>>
}


impl FunctionStmt {

    pub fn new(
        name:   Token,
        params: Vec<Token>,
        body:   Vec<Stmt>,
    ) -> Self {
        Self {
            name,
            params,
            body: Box::new(body),
        }
    }
}
