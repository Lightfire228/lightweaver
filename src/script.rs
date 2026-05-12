use std::{fs, path::Path};

use parser::{parse_ast, ParseErrorType};
use scanner::{scan_tokens, ScannerErrorType};


pub mod tokens;
pub mod scanner;
pub mod ast;
pub mod parser;
pub mod vm;
pub mod resolver;

mod test;

use vm::{compiler::compile, RuntimeError};

use crate::{script::{
    parser::AssignmentTarget,
    resolver::resolve,
    vm::{
        ArenaRoot, State
    }
}, shapes::Shape};

type ScanErrorList  = Vec<scanner::ScannerError>;
type ParseErrorList = Vec<parser ::ParseError>;
type RunResult      = Result<(), RunError>;

#[derive(Debug)]
pub enum RunError {
    IOError,
    ScannerError(ScanErrorList),
    ParserError (ParseErrorList),
    RuntimeError(RuntimeError)
}

type Re = RunError;


pub fn run_file(path: &Path) -> Result<Vec<Shape>, RunError> {


    let source  = fs::read_to_string(path).map_err(|_|   Re::IOError)?;

    let tokens  = scan_tokens(&source)    .map_err(|err| Re::ScannerError(err))?;

    let mut ast = parse_ast(tokens)       .map_err(|err| Re::ParserError(err))?;
    resolve(&mut ast);
    println!("{}", ast);

    let mut root = ArenaRoot::new(|_ctx| { State::new() });

    root.mutate_root(|ctx, root| {
        compile(ast, root, ctx).unwrap();
    });

    root.mutate(|_ctx, root| {
        root.dbg_funcs();
    });

    vm::interpret(root).map_err(|err| Re::RuntimeError(err))

}


fn display_error(err: RunError) -> ! {
    match err {
        Re::IOError           => panic!("Unable to read source file"),
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
            Pe::MissingPropertyIdentifier               => eprintln!("Expect property name after '.'"),
            Pe::MissingSuperDot                         => eprintln!("Expect '.' after super"),
            Pe::MissingSuperPropertyIdentifier          => eprintln!("Expect superclass method name"),
            Pe::MissingGroupingCloseParen               => eprintln!("Expect ')' after expression"),
            Pe::MissingExpression(token)                => eprintln!("Expect expression ({})", token),
            Pe::InvalidAssignmentTarget(target)         => {
                type T = AssignmentTarget;
                match target {
                    T::Dot  => eprintln!("Invalid assignment target for '.'"),
                    T::Expr => eprintln!("Invalid assignment target for expression"),
                }
            },
        }
    }

    panic!()
}


fn display_runtime_err(err: RuntimeError) -> ! {
    panic!("Runtime error: [line {}] {} \n{}", err.line, err.msg, err.stack_trace)
}
