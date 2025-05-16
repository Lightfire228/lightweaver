

mod expr;
mod stmt;


pub use expr::*;
pub use stmt::*;

use super::parser::Parser;

pub trait ParseStmt {
    fn parse(parser: Parser) -> Stmt;
}

pub trait ParseExpr {
    fn parse(parser: Parser) -> Expr;
}


#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>
}
