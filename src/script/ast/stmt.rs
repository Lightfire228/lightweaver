// Auto generated code. Edit scripts/generate_ast.py instead

use super::expr::Expr;
use crate::script::tokens::Token;

pub enum Stmt {
    Block     (Block),
    Expression(Expression),
    Let       (Let),
}

pub struct Block {
    statements: Vec<Stmt>,
}

pub struct Expression {
    expression: Box<Expr>,
}

pub struct Let {
    name:        Token,
    initializer: Box<Expr>,
}


pub trait Visitor<T> {
    fn visit_block     (&mut self, x: &Block)      -> T;
    fn visit_expression(&mut self, x: &Expression) -> T;
    fn visit_let       (&mut self, x: &Let)        -> T;
}