use crate::script::ast::Expr;

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body:      Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Expr, body: Stmt) -> Stmt {
        Stmt::While(Self {
            condition,
            body: Box::new(body),
        })
    }
}
