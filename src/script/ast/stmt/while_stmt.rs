use crate::script::ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs};
use crate::script::ast::Expr;

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body:      Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Expr, body: Stmt) -> Stmt {
        Stmt::While(Self {
            condition,
            body: Box::new(body),
        })
    }
}


impl AstNode for WhileStmt {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "While Stmt".to_owned(),
            labels:  Some(vec![
                "Condition: ".to_owned(),
                "Body:      ".to_owned(),
            ]),
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        vec![
            self.condition.as_ast(),
            self.body     .as_ast(),
        ]
    }
}
