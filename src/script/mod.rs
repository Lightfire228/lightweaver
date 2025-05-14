use std::{fs, path::Path};

use parser::{parse_ast, ParseErrorType};
use scanner::{scan_tokens, ScannerErrorType};


pub mod tokens;
pub mod scanner;
pub mod ast;
pub mod parser;
pub mod interpreter;

mod test;

type ScanErrorList  = Vec<scanner::ScannerError>;
type ParseErrorList = Vec<parser ::ParseError>;

pub enum RunError {
    IOError,
    ScannerError(ScanErrorList),
    ParserError (ParseErrorList),
}
use RunError::*;


type RunResult = Result<(), RunError>;

pub fn run_file(path: &Path) -> &str {

    match (|| {

        let source = fs::read_to_string(path).map_err(|_|   IOError)?;

        let tokens = scan_tokens(&source)    .map_err(|err| ScannerError(err))?;

        let ast    = parse_ast(tokens)       .map_err(|err| ParserError(err))?;

        dbg!(ast);

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
        ParserError (err) => display_parser_err (err),
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

fn display_parser_err(err: ParseErrorList) -> ! {
    use ParseErrorType::*;

    for e in err.iter() {
        eprint!("Compile Error: Line {} - Col {} - \n> ", e.token.line, e.token.col);

        match &e.type_ {
            MissingClassIdentifier                  => panic!("Expect class name"),
            MissingSuperclassIdentifier             => panic!("Expect superclass name"),
            MissingClassOpenCurly                   => panic!("Expect '{{' before class body"),
            MissingClassCloseCurly                  => panic!("Expect '}}' after class body"),
            MissingFunctionIdentifier(type_)        => panic!("Expect {} name",             type_.to_string()),
            MissingFunctionOpenParen (type_)        => panic!("Expect '(' after {} name",   type_.to_string()),
            MissingFunctionOpenBrace (type_)        => panic!("Expect '}}' before {} name", type_.to_string()),
            MissingFunctionCloseParen               => panic!("Expect ')' after parameters"),
            FunctionTooManyParameters               => panic!("Can't have more than 255 parameters"),
            MissingParameterIdentifier              => panic!("Expect parameter name"),
            MissingVariableIdentifier               => panic!("Expect variable name"),
            MissingVariableSemicolon                => panic!("Expect ';' after variable declaration"),
            MissingForOpenParen                     => panic!("Expect '(' after 'for'"),
            MissingForCloseParen                    => panic!("Expect ')' after for clauses"),
            MissingForConditionDelimiter            => panic!("Expect ';' after loop condition"),
            MissingIfOpenParen                      => panic!("Expect '(' after 'if'"),
            MissingIfCloseParen                     => panic!("Expect ')' after if contition"),
            MissingPrintSemicolon                   => panic!("Expect ';' after value"),
            MissingReturnSemicolon                  => panic!("Expect ';' after return value"),
            MissingWhileOpenParen                   => panic!("Expect '(' after while"),
            MissingWhileCloseParen                  => panic!("Expect ')' after condition"),
            MissingExpressionStmtSemicolon          => panic!("Expect ';' after value"),
            MissingBlockCloseBrace                  => panic!("Expect '}}' after block"),
            InvalidAssignmentTarget                 => panic!("Invalid assignment target"),
            MissingPropertyIdentifier               => panic!("Expect property name after '.'"),
            MissingSuperDot                         => panic!("Expect '.' after super"),
            MissingSuperPropertyIdentifier          => panic!("Expect superclass method name"),
            MissingGroupingCloseParen               => panic!("Expect ')' after expression"),
            MissingExpression                       => panic!("Expect expression"),
        }
    }

    panic!()
}
