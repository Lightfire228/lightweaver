use crate::script::{ast::Expr, tokens::Token};

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value:   Option<Expr>,
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Option<Expr>) -> Stmt {
        Stmt::Return(Self {
            keyword,
            value,
        })
    }
}
