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
    name:  Token,
    value: Box::<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Binary {
    left:     Box<Expr>,
    operator: Token,
    right:    Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Call {
    callee: Box<Expr>,
    paren:  Token,
    args:   Box<Vec<Expr>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Get {
    expr: Box<Expr>,
    name: Token,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grouping {
    expr: Box<Expr>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Literal {
    value: Token,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Logical {
    left:     Box<Expr>,
    operator: Token,
    right:    Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Set {
    target: Box<Expr>,
    name:   Token,
    value:  Box<Expr>
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Super {
    keyword: Token,
    method:  Token,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct This {
    keyword: Token,
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Unary {
    operator: Token,
    right:    Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    name: Token,
}