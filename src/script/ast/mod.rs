

mod expr;
mod stmt;


pub use expr::*;
pub use stmt::*;
pub mod eq;


#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>
}

