use super::expr::Expr;
use crate::script::tokens::Token;

#[derive(Debug)]
pub enum Stmt {
    Block     (Block),
    Expression(Expression),
    Let       (Let),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Expression {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct Let {
    pub name: Token,
    pub initializer: Option<Box<Expr>>
}

impl Expression {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Expression(Expression { 
            expression: Box::new(expr),
        })
    }
}

impl Let {
    pub fn new(name: &Token, initializer: Option<Expr>) -> Stmt {
        Stmt::Let(Self {
            name:        name.clone(),
            initializer: initializer.map(|x| Box::new(x))
        })
    }
}