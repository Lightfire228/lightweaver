

mod expr;
mod stmt;
mod var_type;


use std::fmt::Display;

pub use expr    ::*;
pub use stmt    ::*;
pub use var_type::*;

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

    fn walk   (&self, args: WalkArgs)    -> AstNodeList<'_>;

    // fn display_spaces(&self, msg: &str, args: DisplayArgs) {
    //     println!("{}{}", " ".repeat(args.depth * 4), msg)
    // }
}


impl AstNode for Ast {
    fn display(&self, args: DisplayArgs) -> AstDisplay {
        AstDisplay {
            depth:   args.depth,
            primary: "Ast".to_owned(),
            labels:  None,
        }
    }

    fn walk   (&self, _: WalkArgs)    -> AstNodeList<'_> {
        self.stmts.iter().map(Stmt::as_ast).collect()
    }
}


pub struct AstDisplay {
    pub depth:   usize,
    pub primary: String,
    pub labels:  Option<Vec<String>>,
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let args = DisplayArgs { depth: 0 };

        let disp = self.display(args);
        writeln!(f, "{}", disp.primary)?;

        let args = WalkArgs;
        for node in self.walk(args) {
            display(f, node, 1, None)?;
        }

        Ok(())

    }
}


// This is a little kludgey.
// The idea is to walk the AST and display each node at a particular indent level
// while also allowing for the previous node to optionally label it's children
fn display(f: &mut std::fmt::Formatter<'_>, node: Box<&dyn AstNode>, depth: usize, prefix: Option<String>) -> std::fmt::Result {
    let args = DisplayArgs {
        depth,
    };
    let disp   = node.display(args);
    let spaces = spaces(disp.depth);

    writeln!(f,
        "{}{}{}",
        spaces,
        prefix.unwrap_or("".to_owned()),
        disp.primary,
    )?;

    let args     = WalkArgs;
    let children = node.walk(args);

    let depth = depth +1;

    match disp.labels {
        Some(fields) => {
            assert_eq!(children.len(), fields.len(), "The number of display field labels must match the number of node children");

            for (child, prefix) in children.into_iter().zip(fields) {
                display(f, child, depth, Some(prefix))?;
            }
        }
        None => {
            for child in children {
                display(f, child, depth, None)?;
            }
        }
    }
    Ok(())
}

fn spaces(depth: usize) -> String {
    " ".repeat(depth * 4)
}
