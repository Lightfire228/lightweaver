
mod expr_stmt;
mod function_stmt;
mod block_stmt;
mod class_stmt;
mod if_stmt;
mod print_stmt;
mod return_stmt;
mod var_stmt;
mod while_stmt;

pub use expr_stmt     ::*;
pub use function_stmt ::*;
pub use block_stmt    ::*;
pub use class_stmt    ::*;
pub use if_stmt       ::*;
pub use print_stmt    ::*;
pub use return_stmt   ::*;
pub use var_stmt      ::*;
pub use while_stmt    ::*;



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    None,
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
