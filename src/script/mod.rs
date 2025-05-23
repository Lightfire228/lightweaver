use std::{fs, path::Path};

use ast::{Ast, AstNode, DisplayArgs, WalkArgs};
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


        display_ast(ast);

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
            MissingClassIdentifier                  => eprintln!("Expect class name"),
            MissingSuperclassIdentifier             => eprintln!("Expect superclass name"),
            MissingClassOpenCurly                   => eprintln!("Expect '{{' before class body"),
            MissingClassCloseCurly                  => eprintln!("Expect '}}' after class body"),
            MissingFunctionIdentifier(type_)        => eprintln!("Expect {} name",             type_.to_string()),
            MissingFunctionOpenParen (type_)        => eprintln!("Expect '(' after {} name",   type_.to_string()),
            MissingFunctionOpenBrace (type_)        => eprintln!("Expect '}}' before {} name", type_.to_string()),
            MissingFunctionCloseParen               => eprintln!("Expect ')' after parameters"),
            FunctionTooManyParameters               => eprintln!("Can't have more than 255 parameters"),
            MissingParameterIdentifier              => eprintln!("Expect parameter name"),
            MissingVariableIdentifier               => eprintln!("Expect variable name"),
            MissingVariableSemicolon                => eprintln!("Expect ';' after variable declaration"),
            MissingForOpenParen                     => eprintln!("Expect '(' after 'for'"),
            MissingForCloseParen                    => eprintln!("Expect ')' after for clauses"),
            MissingForConditionDelimiter            => eprintln!("Expect ';' after loop condition"),
            MissingIfOpenParen                      => eprintln!("Expect '(' after 'if'"),
            MissingIfCloseParen                     => eprintln!("Expect ')' after if contition"),
            MissingPrintSemicolon                   => eprintln!("Expect ';' after print"),
            MissingReturnSemicolon                  => eprintln!("Expect ';' after return value"),
            MissingWhileOpenParen                   => eprintln!("Expect '(' after while"),
            MissingWhileCloseParen                  => eprintln!("Expect ')' after condition"),
            MissingExpressionStmtSemicolon          => eprintln!("Expect ';' after expression"),
            MissingBlockCloseBrace                  => eprintln!("Expect '}}' after block"),
            InvalidAssignmentTarget                 => eprintln!("Invalid assignment target"),
            MissingPropertyIdentifier               => eprintln!("Expect property name after '.'"),
            MissingSuperDot                         => eprintln!("Expect '.' after super"),
            MissingSuperPropertyIdentifier          => eprintln!("Expect superclass method name"),
            MissingGroupingCloseParen               => eprintln!("Expect ')' after expression"),
            MissingExpression(token)                => eprintln!("Expect expression ({})", token),
        }
    }

    panic!()
}


fn display_ast(ast: Ast) {

    let args = WalkArgs;
    for node in ast.walk(args) {
        display(node);
    }

}


fn display(node: Box<&dyn AstNode>) {
    let args = DisplayArgs {
        depth: 0
    };
    node.display(args);

    let args = WalkArgs;
    for child in node.walk(args) {
        display(child);
    }
}
