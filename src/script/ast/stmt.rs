use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}};
use super::{AstDisplay, ByteCode};

mod expr_stmt;
mod function_stmt;
mod block_stmt;
mod class_stmt;
mod if_stmt;
mod print_stmt;
mod return_stmt;
mod var_stmt;
mod while_stmt;

use ast_macro::AstTryFrom;
pub use expr_stmt     ::*;
pub use function_stmt ::*;
pub use block_stmt    ::*;
pub use class_stmt    ::*;
pub use if_stmt       ::*;
pub use print_stmt    ::*;
pub use return_stmt   ::*;
pub use var_stmt      ::*;
pub use while_stmt    ::*;



#[derive(Debug, PartialEq, Eq, Clone, AstTryFrom)]
pub enum Stmt {
    Block     (Block),
    Class     (Class),
    Expression(ExpressionStmt),
    Function  (FunctionStmt),
    If        (IfStmt),
    Print     (PrintStmt),
    Return    (ReturnStmt),
    Var       (VarStmt),
    While     (WhileStmt),
}

impl AstNode for Stmt {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        self.as_ast().display(args)
    }

    fn compile(&self, args: CompileArgs) -> ByteCode {
        self.as_ast().compile(args)
    }

    fn walk   (&self, args: WalkArgs)    -> AstNodeList<'_> {
        self.as_ast().walk(args)
    }
}

impl Stmt {
    pub fn as_ast(&self) -> Box<&dyn AstNode> {
        match self {
            Stmt::Block      (stmt) => Box::new(stmt),
            Stmt::Class      (stmt) => Box::new(stmt),
            Stmt::Expression (stmt) => Box::new(stmt),
            Stmt::Function   (stmt) => Box::new(stmt),
            Stmt::If         (stmt) => Box::new(stmt),
            Stmt::Print      (stmt) => Box::new(stmt),
            Stmt::Return     (stmt) => Box::new(stmt),
            Stmt::Var        (stmt) => Box::new(stmt),
            Stmt::While      (stmt) => Box::new(stmt),
        }
    }
}
