use crate::script::tokens::Token;

use super::Expr;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    pub name:  Token,
    pub value: Box::<Expr>,
}


impl Assign {
    pub fn new(name: Token, value: Expr) -> Expr {
        Expr::Assign(Self {
            name,
            value: Box::new(value),
        })
    }
}
