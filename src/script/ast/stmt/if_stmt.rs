use crate::script::ast::Expr;

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStmt {
    pub condition:   Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl IfStmt {
    pub fn new(
        condition:   Expr,
        then_branch: Stmt,
        else_branch: Option<Stmt>
    ) -> Stmt {
        Stmt::If(Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|x| Box::new(x)),
        })

    }
}
