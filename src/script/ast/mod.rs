

mod expr;
mod stmt;


pub use expr::*;
pub use stmt::*;

use super::parser::{ParseResult, Parser};

pub trait ParseStmt {
    fn parse(parser: &mut Parser) -> ParseResult<Stmt>;
}

pub trait ParseExpr {
    fn parse(parser: &mut Parser) -> ParseResult<Expr>;
}


#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>
}
