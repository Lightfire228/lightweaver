use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}, tokens::Token};

use super::Stmt;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionStmt {
    pub name:   Token,
    pub params: Vec<FunctionParam>,
    pub body:   Box<Vec<Stmt>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionParam {
    pub name:   Token,
    pub closed: bool,
}

impl FunctionStmt {

    pub fn new(
        name:   Token,
        params: Vec<Token>,
        body:   Vec<Stmt>,
    ) -> Self {
        Self {
            name,
            params: params.into_iter().map(|p| FunctionParam::new(p)).collect(),
            body:   Box::new(body),
        }
    }
}

impl FunctionParam {
    fn new(name: Token) -> Self {
        Self {
            name,
            closed: false,
        }
    }
}

impl AstNode for FunctionStmt {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Function ({})", self.name.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        self.body.iter().map(Stmt::as_ast).collect()
    }
}
