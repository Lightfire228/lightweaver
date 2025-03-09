
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

pub enum Stmt {
    Expr(Expr),
}

pub enum Expr {
    Assignment,
    Connection,
}

pub struct Logical {
    
}