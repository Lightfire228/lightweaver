use crate::script::tokens::Token;

use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren:  Token,
    pub args:   Box<Vec<Expr>>,
}


impl Call {
    pub fn new(
        callee: Expr,
        paren:  Token,
        args:   Vec<Expr>,
    ) -> Expr {
        Expr::Call(Self {
            callee: Box::new(callee),
            paren,
            args:   Box::new(args),
        })
    }
}
