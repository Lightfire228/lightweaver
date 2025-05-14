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

impl Literal {
    pub fn new(value: Token) -> Expr {
        Expr::Literal(Self {
            value,
        })
    }
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Expr {
        Expr::Assign(Self {
            name,
            value: Box::new(value),
        })
    }
}

impl Set {
    pub fn new(target: Expr, name: Token, value: Expr) -> Expr {
        Expr::Set(Self {
            target: Box::new(target),
            name,
            value: Box::new(value),
        })
    }
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Logical(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Self {
            operator,
            right: Box::new(right),
        })
    }
}

impl Get {
    pub fn new(expr: Expr, name: Token) -> Expr {
        Expr::Get(Self {
            expr: Box::new(expr),
            name,
        })
    }
}

impl Call {
    pub fn new(
        callee: Expr,
        paren:  Token,
        args:   Vec<Expr>,
    ) -> Expr {
        Expr::Call(Self {
            callee: Box::new(callee),
            paren,
            args:   Box::new(args),
        })
    }
}

impl This {
    pub fn new(
        keyword: Token,
    ) -> Expr {
        Expr::This(Self {
            keyword,
        })
    }
}

impl Grouping {
    pub fn new(
        expr: Expr,
    ) -> Expr {
        Expr::Grouping(Self {
            expr: Box::new(expr)
        })
    }
}

impl Super {
    pub fn new(
        keyword: Token,
        method:  Token,

    ) -> Expr {
        Expr::Super(Self {
            keyword,
            method,
        })
    }
}
