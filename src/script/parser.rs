use crate::ScriptRuntime;

use super::{ast::Stmt, token::Token};


pub struct Parser <'a> {
    runtime: &'a mut ScriptRuntime,

    tokens:  Vec<Token>,
    current: usize,
}

impl <'a> Parser <'a> {
    pub fn new(runtime: &'a mut ScriptRuntime, tokens: Vec<Token>) -> Parser <'a> {
        Parser {
            runtime,

            tokens,
            current: 0,
        }
    }

    pub fn parse(runtime: &'a mut ScriptRuntime, tokens: Vec<Token>) -> Vec<Stmt> {

        let mut parser = Parser::new(runtime, tokens);

        // TODO:
        Vec::new()
    }
}
