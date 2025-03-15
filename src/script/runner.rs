
use std::{fs::{self, File}, io::{self, BufWriter, Read}, path::{Path, PathBuf}};

use super::{scanner::{Scanner, ScannerError}, tokens::Token};

pub enum RunnerError {
    ParserError(Vec<ScannerError>),
    RuntimeError,
}


pub fn run_file(path: &Path) -> Result<(), RunnerError> {

    let source = fs::read_to_string(path).expect("Couldn't find the file specified");

    match run(&source) {
        Ok(()) => (),

        Err(error) => {
            display_error(error);
            panic!("Unable to run file")
        }
    }

    Ok(())
}

pub fn run_repl() -> Result<(), RunnerError> {

    let mut buffer = String::new();
    let     stdin  = io::stdin();

    loop {
        buffer.clear();

        println!("> ");
        stdin.read_line(&mut buffer).expect("Unable to read from stdin");

        if buffer.is_empty() {
            break;
        }

        match run(&buffer) {
            Ok(())   => (),
            Err(err) => display_error(err),
        };

    }

    Ok(())
}

fn run(source: &str) -> Result<(), RunnerError> {

    let res = Scanner::scan_tokens(source);

    match res {
        Ok(tokens) => {
            display_tokens(tokens);
            Ok(())
        },
        Err(errs)  => Err(RunnerError::ParserError(errs)),
    }
}



fn display_error(err: RunnerError) {
    use self::RunnerError::*;
    match err {
        ParserError(parser_err) => display_scanner_errors(parser_err),
        RuntimeError            => display_runtime_errors(),
    }
}

fn display_scanner_errors(errs: Vec<ScannerError>) {

    for e in errs {
        eprintln!("[Scanner Error, Line: {}, Col: {}] {}", e.line, e.col, e.msg);
    }
}

fn display_runtime_errors() {
    //TODO:
}


fn display_tokens(tokens: Vec<Token>) {
    for t in tokens {
        println!("{}", t);
    }
}