
use std::{fs, io, path::PathBuf, process::exit};

use super::{interpreter::Interpreter, parser::Parser, scanner::Scanner, token::{Token, TokenType}};

pub struct ScriptRuntime {
    had_error:         bool,
    had_runtime_error: bool,
}

impl ScriptRuntime {
    pub fn new() -> ScriptRuntime {
        ScriptRuntime {
            had_error:         false,
            had_runtime_error: false,
        }
    }

    pub fn run_file(&mut self, path_str: &str) {
        let path = PathBuf::from(path_str);
        let file = fs::canonicalize(path).unwrap();

        let source = file_to_str(&file).unwrap();

        self.run(&source);

        if self.had_error {
            exit(65);
        }
        if self.had_runtime_error {
            exit(70);
        }
    }

    pub fn run_prompt(&mut self) {
        // TODO:
    }

    fn run(&mut self, source: &str) {
        let tokens = Scanner::scan_tokens(self, &source);
        let stmts  = Parser ::parse      (self, tokens);

        if self.had_error {
            return
        }

        Interpreter::interpret(&self, stmts);
    }

    pub fn error_tokenizing(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn error_parsing(&mut self, token: Token, message: &str) {

        let loc     = format!(" at '{}'", message);
        let mut loc = loc.as_ref();

        if token.type_ == TokenType::EOF {
            loc = " at end of file";
        }

        self.report(token.line, loc, message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, where_, message);

        self.had_error = true;
    }
}


fn file_to_str(path: &PathBuf) -> io::Result<String> {
    fs::read_to_string(path)
}
