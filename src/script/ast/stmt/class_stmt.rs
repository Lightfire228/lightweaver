use crate::script::{ast::{AstDisplay, AstNode, AstNodeList, CompileArgs, DisplayArgs, VarType, WalkArgs}, tokens::Token};
use crate::script::{ast::Variable};

use super::{FunctionStmt, Stmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub name:       Token,
    pub superclass: Option<Variable>,
    pub methods:    Box<Vec<FunctionStmt>>,
    pub var_type:   VarType,
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
            methods: Box::new(methods),
            var_type: VarType::Global,
        })
    }
}

impl AstNode for Class {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        let msg = format!("Class ({})", self.name.lexeme);

        AstDisplay {
            depth:   args.depth,
            primary: msg,
            labels:  Some(
                self.methods.iter().map(|_| "Method: ".to_owned()).collect()
            ),
        }
    }

    fn compile(&self, _: CompileArgs) -> crate::script::ast::ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        let mut results: AstNodeList = vec![];

        if let Some(superclass) = &self.superclass {
            results.push(Box::new(superclass));
        }

        results.extend(self.methods.iter().map(|f| Box::new(f as &dyn AstNode)));

        results
    }
}
