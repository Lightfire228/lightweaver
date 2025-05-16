use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    pub value: Token,
}

impl Literal {
    pub fn new(value: Token) -> Expr {
        Expr::Literal(Self {
            value,
        })
    }
}
