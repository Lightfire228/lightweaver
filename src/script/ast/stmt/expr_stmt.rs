use crate::script::ast::Expr;

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionStmt {
    pub expr: Expr,
}

impl ExpressionStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Expression(Self {
            expr,
        })
    }
}
