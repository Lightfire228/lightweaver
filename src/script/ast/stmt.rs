use super::expr::{self, Expr};
use crate::script::tokens::Token;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    None,
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
    pub stmts: Box::<Vec<Stmt>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub name:       Token,
    pub superclass: Option<expr::Variable>,
    pub methods:    Box<Vec<FunctionStmt>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionStmt {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionStmt {
    pub name:   Token,
    pub params: Vec<Token>,
    pub body:   Box<Vec<Stmt>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStmt {
    pub condition:   Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrintStmt {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value:   Option<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarStmt {
    pub name:        Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body:      Box<Stmt>,
}

impl Class {
    pub fn new(
        name:       Token,
        superclass: Option<expr::Variable>,
        methods:    Vec<FunctionStmt>
    ) -> Stmt {
        Stmt::Class(Self {
            name,
            superclass,
            methods: Box::new(methods)
        })
    }
}

impl FunctionStmt {

    pub fn new(
        name:   Token,
        params: Vec<Token>,
        body:   Vec<Stmt>,
    ) -> Self {
        Self {
            name,
            params,
            body: Box::new(body),
        }
    }
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Stmt {
        Stmt::Block(Block {
            stmts: Box::new(stmts),
        })
    }
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Expr>) -> Stmt {
        Stmt::Var(VarStmt {
            name,
            initializer,
        })
    }
}

impl ExpressionStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Expression(Self {
            expr,
        })
    }
}

impl WhileStmt {
    pub fn new(condition: Expr, body: Stmt) -> Stmt {
        Stmt::While(Self {
            condition,
            body: Box::new(body),
        })
    }
}

impl IfStmt {
    pub fn new(
        condition:   Expr,
        then_branch: Stmt,
        else_branch: Option<Stmt>
    ) -> Stmt {
        Stmt::If(Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|x| Box::new(x)),
        })

    }
}

impl PrintStmt {
    pub fn new(expr: Expr) -> Stmt {
        Stmt::Print(Self {
            expr,
        })
    }
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Option<Expr>) -> Stmt {
        Stmt::Return(Self {
            keyword,
            value,
        })
    }
}
