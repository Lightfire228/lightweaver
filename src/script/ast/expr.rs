// Auto generated code. Edit scripts/generate_ast.py instead

use crate::script::tokens::Token;

pub enum Expr {
    Assign       (Assign),
    Grouping     (Grouping),
    Instantiation(Instantiation),
    Body         (Body),
    Property     (Property),
    Literal      (Literal),
    Logical      (Logical),
    Variable     (Variable),
}

pub struct Assign {
    name:  Token,
    value: Box<Expr>,
}

pub struct Grouping {
    expression: Box<Expr>,
}

pub struct Instantiation {
    type_: Token,
    body:  Box<Body>,
}

pub struct Body {
    properties: Vec<Property>,
}

pub struct Property {
    name:        Token,
    initializer: Box<Expr>,
}

pub struct Literal {
}

pub struct Logical {
    left:     Box<Expr>,
    operator: Token,
    right:    Box<Expr>,
}

pub struct Variable {
    name: Token,
}


pub trait Visitor<T> {
    fn visit_assign       (&mut self, x: &Assign)        -> T;
    fn visit_grouping     (&mut self, x: &Grouping)      -> T;
    fn visit_instantiation(&mut self, x: &Instantiation) -> T;
    fn visit_body         (&mut self, x: &Body)          -> T;
    fn visit_property     (&mut self, x: &Property)      -> T;
    fn visit_literal      (&mut self, x: &Literal)       -> T;
    fn visit_logical      (&mut self, x: &Logical)       -> T;
    fn visit_variable     (&mut self, x: &Variable)      -> T;
}