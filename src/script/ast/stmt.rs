use super::expr::Expr;
use crate::script::tokens::Token;

#[derive(Debug)]
pub enum Stmt {
    #[allow(dead_code)]
    Block     (Block),
    Expression(ExpressionStmt),
    VarDecl   (VarDecl),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: Token,
    pub initializer: Option<Box<Expr>>
}

impl ExpressionStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Expression(ExpressionStmt { 
            expression: Box::new(expr),
        })
    }
}

impl VarDecl {
    pub fn new(name: Token, initializer: Option<Expr>) -> Stmt {
        Stmt::VarDecl(Self {
            name,
            initializer: initializer.map(|x| Box::new(x))
        })
    }
}