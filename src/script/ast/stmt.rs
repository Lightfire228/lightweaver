use super::expr::{self, Expr};
use crate::script::tokens::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    Block     (Block),
    Class     (Class),
    Expression(ExpressionStmt),
    Function  (FunctionStmt),
    If        (IfStmt),
    Print     (PrintStmt),
    Return    (ReturnStmt),
    Var       (VarStmt),
    While     (WhileStmt),
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    stmts: Box::<Vec<Stmt>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    name:       Token,
    superclass: expr::Variable,
    methods:    Box<Vec<FunctionStmt>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionStmt {
    expr: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionStmt {
    name:   Token,
    params: Vec<Token>,
    body:   Box<Vec<Stmt>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStmt {
    condition:   Expr,
    then_branch: Box<Stmt>,
    else_branch: Box<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrintStmt {
    expr: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmt {
    keyword: Token,
    value:   Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarStmt {
    name:        Token,
    initializer: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WhileStmt {
    condition: Expr,
    body:      Box<Stmt>,
}