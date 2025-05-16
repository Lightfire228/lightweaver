use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Get {
    pub expr: Box<Expr>,
    pub name: Token,
}


impl Get {
    pub fn new(expr: Expr, name: Token) -> Expr {
        Expr::Get(Self {
            expr: Box::new(expr),
            name,
        })
    }
}
