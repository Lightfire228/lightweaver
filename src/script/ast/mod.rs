

mod expr;
mod stmt;


pub use expr::*;
pub use stmt::*;

use super::parser::{ParseResult, Parser};

pub trait ParseStmt {
    fn parse(parser: &mut Parser) -> ParseResult<Stmt>;
}

pub trait ParseExpr {
    fn parse(parser: &mut Parser) -> ParseResult<Expr>;
}


#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>
}

pub struct DisplayArgs {
    pub depth: usize
}

pub struct CompileArgs;
pub struct WalkArgs;

pub struct ByteCode;

pub type AstNodeList<'a> = Vec<Box<&'a dyn AstNode>>;

pub trait AstNode {

    fn display(&self, args: DisplayArgs) -> AstDisplay;

    fn compile(&self, args: CompileArgs) -> ByteCode;

    fn walk   (&self, args: WalkArgs)    -> AstNodeList;

    fn display_spaces(&self, msg: &str, args: DisplayArgs) {
        println!("{}{}", " ".repeat(args.depth * 4), msg)
    }
}


impl AstNode for Ast {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Ast".to_owned(),
            fields:  None,
        }
    }

    fn compile(&self, _: CompileArgs) -> ByteCode {
        todo!()
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList {
        self.stmts.iter().map(Stmt::as_ast).collect()
    }
}


pub struct AstDisplay {
    pub depth:   usize,
    pub primary: String,
    pub fields:  Option<Vec<String>>,
}
