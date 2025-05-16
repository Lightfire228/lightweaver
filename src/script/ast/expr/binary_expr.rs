use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Binary {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}


impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Self {
            left:     Box::new(left),
            operator,
            right:    Box::new(right),
        })
    }
}
