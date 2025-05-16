use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Super {
    pub keyword: Token,
    pub method:  Token,
}


impl Super {
    pub fn new(
        keyword: Token,
        method:  Token,

    ) -> Expr {
        Expr::Super(Self {
            keyword,
            method,
        })
    }
}
