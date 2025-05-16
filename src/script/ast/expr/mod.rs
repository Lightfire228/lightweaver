
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
