use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right:    Box<Expr>,
}


impl Unary {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Self {
            operator,
            right: Box::new(right),
        })
    }
}
