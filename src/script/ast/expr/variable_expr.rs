use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    pub name: Token,
}


impl Variable {
    pub fn new(name: Token) -> Expr {
        Expr::Variable(Self { name })
    }
}
