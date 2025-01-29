use crate::ScriptRuntime;

use super::ast::Stmt;



pub struct Interpreter <'a> {
    runtime: &'a ScriptRuntime,

    stmts: Vec<Stmt>,
}

impl <'a> Interpreter <'a> {
    pub fn new(runtime: &'a ScriptRuntime, stmts: Vec<Stmt>) -> Interpreter<'a> {
        Interpreter {
            runtime,
            stmts,
        }
    }

    pub fn interpret(runtime: &'a ScriptRuntime, stmts: Vec<Stmt>) {
        let mut interpreter = Interpreter::new(runtime, stmts);
    } 
}