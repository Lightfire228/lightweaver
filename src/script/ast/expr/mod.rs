use crate::script::{ast::{AstNode, AstNodeList, CompileArgs, DisplayArgs, WalkArgs}};

mod assign_expr;
mod binary_expr;
mod call_expr;
mod get_expr;
mod grouping_expr;
mod literal_expr;
mod logical_expr;
mod set_expr;
mod super_expr;
mod this_expr;
mod unary_expr;
mod variable_expr;

pub use assign_expr   ::*;
pub use binary_expr   ::*;
pub use call_expr     ::*;
pub use get_expr      ::*;
pub use grouping_expr ::*;
pub use literal_expr  ::*;
pub use logical_expr  ::*;
pub use set_expr      ::*;
pub use super_expr    ::*;
pub use this_expr     ::*;
pub use unary_expr    ::*;
pub use variable_expr ::*;


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

impl AstNode for Expr {
    fn display(&self, args: DisplayArgs) {
        self.as_ast().display(args)
    }

    fn compile(&self, args: CompileArgs) -> crate::script::ast::ByteCode {
        self.as_ast().compile(args)
    }

    fn walk   (&self, args: WalkArgs)    -> AstNodeList {
        self.as_ast().walk(args)
    }
}


impl Expr {
    pub fn as_ast(&self) -> Box<&dyn AstNode> {
        match self {
            Expr::Assign   (expr) => Box::new(expr),
            Expr::Binary   (expr) => Box::new(expr),
            Expr::Call     (expr) => Box::new(expr),
            Expr::Get      (expr) => Box::new(expr),
            Expr::Grouping (expr) => Box::new(expr),
            Expr::Literal  (expr) => Box::new(expr),
            Expr::Logical  (expr) => Box::new(expr),
            Expr::Set      (expr) => Box::new(expr),
            Expr::Super    (expr) => Box::new(expr),
            Expr::This     (expr) => Box::new(expr),
            Expr::Unary    (expr) => Box::new(expr),
            Expr::Variable (expr) => Box::new(expr),
        }
    }
}
