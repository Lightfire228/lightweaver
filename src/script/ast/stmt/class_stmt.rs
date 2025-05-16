use crate::script::{ast::Variable, tokens::Token};

use super::{FunctionStmt, Stmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub name:       Token,
    pub superclass: Option<Variable>,
    pub methods:    Box<Vec<FunctionStmt>>
}

impl Class {
    pub fn new(
        name:       Token,
        superclass: Option<Variable>,
        methods:    Vec<FunctionStmt>
    ) -> Stmt {
        Stmt::Class(Self {
            name,
            superclass,
            methods: Box::new(methods)
        })
    }
}
