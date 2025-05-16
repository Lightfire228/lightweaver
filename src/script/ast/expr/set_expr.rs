use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Set {
    pub target: Box<Expr>,
    pub name:   Token,
    pub value:  Box<Expr>
}

impl Set {
    pub fn new(target: Expr, name: Token, value: Expr) -> Expr {
        Expr::Set(Self {
            target: Box::new(target),
            name,
            value: Box::new(value),
        })
    }
}
