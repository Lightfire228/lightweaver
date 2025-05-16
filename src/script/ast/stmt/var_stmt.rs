use crate::script::{ast::Expr, tokens::Token};

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarStmt {
    pub name:        Token,
    pub initializer: Option<Expr>,
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Expr>) -> Stmt {
        Stmt::Var(VarStmt {
            name,
            initializer,
        })
    }
}
