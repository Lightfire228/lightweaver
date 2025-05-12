use crate::script::tokens::Token;


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Assign   (Assign),
    Binary   (Binary),
    Call     (Call),
    Get      (Get),
    Grouping (Grouping),
    Literal  (Literal),
    Logical  (Logical),
    Set      (Set),
    Super    (Super),
    This     (This),
    Unary    (Unary),
    Variable (Variable),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    pub name:  Token,
    pub value: Box::<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Binary {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren:  Token,
    pub args:   Box<Vec<Expr>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Get {
    pub expr: Box<Expr>,
    pub name: Token,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grouping {
    pub expr: Box<Expr>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    pub value: Token,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Logical {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Set {
    pub target: Box<Expr>,
    pub name:   Token,
    pub value:  Box<Expr>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Super {
    pub keyword: Token,
    pub method:  Token,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct This {
    pub keyword: Token,
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right:    Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    pub name: Token,
}


impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Self {
            left:     Box::new(left),
            operator,
            right:    Box::new(right),
        })
    }
}

impl Variable {
    pub fn new(name: Token) -> Expr {
        Expr::Variable(Self { name })
    }
}