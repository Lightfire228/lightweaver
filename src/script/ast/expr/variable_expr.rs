use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, VarType, WalkArgs}, tokens::Token};
use super::Expr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Variable {
    pub name:     Token,
    pub var_type: VarType,
}


impl Variable {
    pub fn new(name: Token) -> Variable {
        Self {
            name,
            var_type: VarType::Global,
        }
    }

    pub fn as_expr(self) -> Expr {
        Expr::Variable(self)
    }
}

impl AstNode for Variable {
    fn display(&self, args: DisplayArgs) -> AstDisplay {

        let msg = format!("Variable ({}, type: {}{})", self.name.lexeme, self.var_type, get(&self.var_type));

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        vec![]
    }
}

fn get(var_type: &VarType) -> String {
    match var_type {
        VarType::Global         => format!(""),
        VarType::Upvalue(index) => format!(", index: {}", **index),
        VarType::Local  (index) => format!(", index: {}", **index),
    }
}
