

mod expr;
mod stmt;


pub use expr::*;
pub use stmt::*;


#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>
}

