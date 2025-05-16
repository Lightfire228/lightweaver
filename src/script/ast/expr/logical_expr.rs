use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Logical {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Logical(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}
