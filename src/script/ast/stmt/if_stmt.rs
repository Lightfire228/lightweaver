use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}};
use crate::script::ast::Expr;

use super::Stmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStmt {
    pub condition:   Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
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

impl AstNode for IfStmt {
    fn display(&self, _: DisplayArgs) {
        println!("If Stmt")
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        let mut results = vec![
            self.condition  .as_ast(),
            self.then_branch.as_ast(),
        ];

        if let Some(else_branch) = &self.else_branch {
            results.push(else_branch.as_ast());
        }

        results
    }
}
