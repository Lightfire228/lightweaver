use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct This {
    pub keyword: Token,
}


impl This {
    pub fn new(
        keyword: Token,
    ) -> Expr {
        Expr::This(Self {
            keyword,
        })
    }
}
