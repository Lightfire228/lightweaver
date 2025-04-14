use crate::script::tokens::Token;

#[derive(Debug)]
pub enum Expr {
    Assign       (Assign),
    Instantiation(Instantiation),
    Connection   (Connection),
    Variable     (Variable),
}

#[derive(Debug)]
pub struct Assign {
    pub name:  Token,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct Instantiation {
    pub type_: Token,
    // pub body:  Box<Body>,
}

// #[derive(Debug)]
// pub struct Body {
//     pub properties: Vec<Property>,
// }

// #[derive(Debug)]
// pub struct Property {
//     pub name:        Token,
//     pub initializer: Box<Expr>,
// }

#[derive(Debug)]
pub struct Literal {
}

#[derive(Debug)]
pub struct Connection {
    pub left:     Box<Expr>,
    pub operator: Token,
    pub right:    Box<Expr>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}


impl Assign {
    pub fn new(name: &Token, expr: Expr) -> Expr {
        Expr::Assign(Self { 
            name:  name.clone(),
            value: Box::new(expr),
        })
    }
}


impl Instantiation {
    pub fn new(type_: &Token) -> Expr {
        Expr::Instantiation(Self { 
            type_:  type_.clone(),
            // expr: Box::new(expr),
        })
    }
}

impl Variable {
    pub fn new(name: &Token) -> Expr {
        Expr::Variable(Self {
            name: name.clone(),
        })
    }
}