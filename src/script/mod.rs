use std::{fs, path::Path};

use ast::{Ast, AstNode, DisplayArgs, WalkArgs};
use parser::{parse_ast, ParseErrorType};
use scanner::{scan_tokens, ScannerErrorType};


pub mod tokens;
pub mod scanner;
pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod vm;

mod test;

use vm::{compiler::Compiler, RuntimeError, Vm};

type ScanErrorList  = Vec<scanner::ScannerError>;
type ParseErrorList = Vec<parser ::ParseError>;
type RunResult      = Result<(), RunError>;

pub enum RunError {
    IOError,
    ScannerError(ScanErrorList),
    ParserError (ParseErrorList),
    RuntimeError(RuntimeError)
}

type Re = RunError;


pub fn run_file(path: &Path) -> &str {

    match (|| {

        let source = fs::read_to_string(path).map_err(|_|   Re::IOError)?;

        let tokens = scan_tokens(&source)    .map_err(|err| Re::ScannerError(err))?;

        let ast    = parse_ast(tokens)       .map_err(|err| Re::ParserError(err))?;

        display_ast(&ast);
        let chunks  = Compiler::compile(ast);

        let mut vm = Vm::new();
        vm.interpret(chunks).map_err(|err| Re::RuntimeError(err))?;

        Ok("test")
    })() {
        Err(err) => display_error(err),

        Ok (val) => val,
    }
}


fn display_error(err: RunError) -> ! {
    match err {
        Re::IOError => panic!("Unable to read source file"),
        Re::ScannerError(err) => display_scanner_err(err),
        Re::ParserError (err) => display_parser_err (err),
        Re::RuntimeError(err) => display_runtime_err(err),
    }
}

fn display_scanner_err(err: ScanErrorList) -> ! {
    type Se = ScannerErrorType;

    for e in err.iter() {
        eprint!("Compile Error: Line {} - Col {} - \n> ", e.line, e.col);

        match &e.type_ {
            Se::UnterminatedString      => eprintln!("Unterminated String"),
            Se::UnexpectedCharacter(ch) => eprintln!("Unexpected character: '{}'", ch),
        }
    }

    panic!()
}

fn display_parser_err(err: ParseErrorList) -> ! {
    type Pe = ParseErrorType;

    for e in err.iter() {
        eprint!("Compile Error: Line {} - Col {} - \n> ", e.token.line, e.token.col);

        match &e.type_ {
            Pe::MissingClassIdentifier                  => eprintln!("Expect class name"),
            Pe::MissingSuperclassIdentifier             => eprintln!("Expect superclass name"),
            Pe::MissingClassOpenCurly                   => eprintln!("Expect '{{' before class body"),
            Pe::MissingClassCloseCurly                  => eprintln!("Expect '}}' after class body"),
            Pe::MissingFunctionIdentifier(type_)        => eprintln!("Expect {} name",             type_.to_string()),
            Pe::MissingFunctionOpenParen (type_)        => eprintln!("Expect '(' after {} name",   type_.to_string()),
            Pe::MissingFunctionOpenBrace (type_)        => eprintln!("Expect '}}' before {} name", type_.to_string()),
            Pe::MissingFunctionCloseParen               => eprintln!("Expect ')' after parameters"),
            Pe::FunctionTooManyParameters               => eprintln!("Can't have more than 255 parameters"),
            Pe::MissingParameterIdentifier              => eprintln!("Expect parameter name"),
            Pe::MissingVariableIdentifier               => eprintln!("Expect variable name"),
            Pe::MissingVariableSemicolon                => eprintln!("Expect ';' after variable declaration"),
            Pe::MissingForOpenParen                     => eprintln!("Expect '(' after 'for'"),
            Pe::MissingForCloseParen                    => eprintln!("Expect ')' after for clauses"),
            Pe::MissingForConditionDelimiter            => eprintln!("Expect ';' after loop condition"),
            Pe::MissingIfOpenParen                      => eprintln!("Expect '(' after 'if'"),
            Pe::MissingIfCloseParen                     => eprintln!("Expect ')' after if contition"),
            Pe::MissingPrintSemicolon                   => eprintln!("Expect ';' after print"),
            Pe::MissingReturnSemicolon                  => eprintln!("Expect ';' after return value"),
            Pe::MissingWhileOpenParen                   => eprintln!("Expect '(' after while"),
            Pe::MissingWhileCloseParen                  => eprintln!("Expect ')' after condition"),
            Pe::MissingExpressionStmtSemicolon          => eprintln!("Expect ';' after expression"),
            Pe::MissingBlockCloseBrace                  => eprintln!("Expect '}}' after block"),
            Pe::InvalidAssignmentTarget                 => eprintln!("Invalid assignment target"),
            Pe::MissingPropertyIdentifier               => eprintln!("Expect property name after '.'"),
            Pe::MissingSuperDot                         => eprintln!("Expect '.' after super"),
            Pe::MissingSuperPropertyIdentifier          => eprintln!("Expect superclass method name"),
            Pe::MissingGroupingCloseParen               => eprintln!("Expect ')' after expression"),
            Pe::MissingExpression(token)                => eprintln!("Expect expression ({})", token),
        }
    }

    panic!()
}

fn display_runtime_err(err: RuntimeError) -> ! {
    panic!("Runtime error: [line {}] {}", err.line, err.msg)
}



fn display_ast(ast: &Ast) {

    let args = DisplayArgs { depth: 0 };

    let disp = ast.display(args);
    println!("{}", disp.primary);

    let args = WalkArgs;
    for node in ast.walk(args) {
        display(node, 1, None);
    }

}


// This is a little kludgey.
// The idea is to walk the AST and display each node at a particular indent level
// while also allowing for the previous node to optionally label it's children
fn display(node: Box<&dyn AstNode>, depth: usize, prefix: Option<String>) {
    let args = DisplayArgs {
        depth,
    };
    let disp   = node.display(args);
    let spaces = spaces(disp.depth);

    println!(
        "{}{}{}",
        spaces,
        prefix.unwrap_or("".to_owned()),
        disp.primary,
    );

    let args     = WalkArgs;
    let children = node.walk(args);

    let depth = depth +1;

    match disp.labels {
        Some(fields) => {
            assert_eq!(children.len(), fields.len(), "The number of display field labels must match the number of node children");

            for (child, prefix) in children.into_iter().zip(fields) {
                display(child, depth, Some(prefix));
            }
        }
        None => {
            for child in children {
                display(child, depth, None);
            }
        }
    }
}

fn spaces(depth: usize) -> String {
    " ".repeat(depth * 4)
}
