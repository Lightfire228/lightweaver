
// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html

mod expr;
mod stmt;


pub use expr::*;
pub use stmt::*;
pub mod eq;


#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>
}

