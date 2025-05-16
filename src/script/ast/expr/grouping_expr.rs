
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>
}


impl Grouping {
    pub fn new(
        expr: Expr,
    ) -> Expr {
        Expr::Grouping(Self {
            expr: Box::new(expr)
        })
    }
}
