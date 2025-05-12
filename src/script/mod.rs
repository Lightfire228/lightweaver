use std::{fs, path::Path};

use scanner::{scan_tokens, ScannerErrorType};


pub mod tokens;
pub mod scanner;
pub mod ast;
pub mod parser;
pub mod interpreter;

mod test;

type ScanErrorList = Vec<scanner::ScannerError>;

pub enum RunError {
    IOError,
    ScannerError(ScanErrorList),
}
use RunError::*;


type RunResult = Result<(), RunError>;

pub fn run_file(path: &Path) -> &str {

    match (|| {

        let source = fs::read_to_string(path).map_err(|_| IOError)?;

        let tokens = scan_tokens(&source).map_err(|err| ScannerError(err))?;

        dbg!(tokens);

        Ok("test")
    })() {
        Err(err) => display_error(err),

        Ok (val) => val,
    }
}


fn display_error(err: RunError) -> ! {
    match err {
        IOError => panic!("Unable to read source file"),

        ScannerError(err) => display_scanner_err(err),
    }
}

fn display_scanner_err(err: ScanErrorList) -> ! {
    use ScannerErrorType::*;

    for e in err.iter() {
        eprint!("Compile Error: Line {} - Col {} - \n> ", e.line, e.col);

        match &e.type_ {
            UnterminatedString      => eprintln!("Unterminated String"),
            UnexpectedCharacter(ch) => eprintln!("Unexpected character: '{}'", ch),
        }
    }

    panic!()
}

