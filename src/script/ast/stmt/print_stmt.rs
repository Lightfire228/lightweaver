use crate::script::ast::Expr;

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrintStmt {
    pub expr: Expr,
}

impl PrintStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Print(Self {
            expr,
        })
    }
}
