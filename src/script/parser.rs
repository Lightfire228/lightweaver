use std::collections::HashMap;

use crate::script::ast::*;

use crate::script::tokens::TokenType::*;

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse_ast(tokens: Vec<Token>) -> Result<Ast, Vec<ParseError>> {
    let mut parser = Parser::new(tokens);

    let mut statements = vec![];
    let mut errors     = vec![];

    while !parser.is_eof() {
        match parser.parse_declaration() {
            Ok (stmt) => statements.push(stmt),
            Err(err)  => errors    .push(err),
        }
    }

    if errors.len() > 0 {
        Err(errors)
    }
    else {
        Ok(Ast { stmts: statements })
    }
}

pub struct Parser {
    tokens:  Vec<Token>,
    current: usize,

    parse_table: HashMap<TokenType, ParseRule>,

    // debug
    parse_stack: Vec<String>,
    depth:       usize,
    
}


#[derive(Debug)]
pub struct ParseError {
    pub type_: ParseErrorType,
    pub token: Token,

}

#[derive(Debug)]
pub enum ParseErrorType {
    MissingClassIdentifier,
    MissingSuperclassIdentifier,
    MissingClassOpenCurly,
    MissingClassCloseCurly,
    MissingFunctionIdentifier(FunctionType),
    MissingFunctionOpenParen (FunctionType),
    MissingFunctionOpenBrace (FunctionType),
    MissingFunctionCloseParen,
    FunctionTooManyParameters,
    MissingParameterIdentifier,
    MissingVariableIdentifier,
    MissingVariableSemicolon,
    MissingForOpenParen,
    MissingForCloseParen,
    MissingForConditionDelimiter,
    MissingIfOpenParen,
    MissingIfCloseParen,
    MissingPrintSemicolon,
    MissingReturnSemicolon,
    MissingWhileOpenParen,
    MissingWhileCloseParen,
    MissingExpressionStmtSemicolon,
    MissingBlockCloseBrace,
    InvalidAssignmentTarget,
    MissingPropertyIdentifier,
    MissingSuperDot,
    MissingSuperPropertyIdentifier,
    MissingGroupingCloseParen,
    MissingExpression(Token),
}

use ParseErrorType::*;

use super::tokens::{Token, TokenType};

#[derive(Debug, Clone, Copy)]
pub enum FunctionType {
    Function,
    Method,
}

impl FunctionType {
    pub fn to_string(&self) -> &str {
        match self {
            FunctionType::Function => "function",
            FunctionType::Method   => "method",
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Precidence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // == !=
    PrecComparison, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary
}

impl Precidence {
    fn next(&self) -> Precidence {
        use Precidence::*;

        match self {
            PrecNone       => PrecAssignment,
            PrecAssignment => PrecOr,
            PrecOr         => PrecAnd,
            PrecAnd        => PrecEquality,
            PrecEquality   => PrecComparison,
            PrecComparison => PrecTerm,
            PrecTerm       => PrecFactor,
            PrecFactor     => PrecUnary,
            PrecUnary      => PrecCall,
            PrecCall       => PrecPrimary,
            PrecPrimary    => panic!("Unknown precidence")
        }
    }
}


type ParseFunc = fn(&mut Parser, RuleArgs) -> ParseResult<Expr>;

// enum ParseFunc {
//     Stmt (fn(&mut Parser, RuleArgs) -> ParseResult<Stmt>),
//     Expr (fn(&mut Parser, RuleArgs) -> ParseResult<Expr>),
// }

#[derive(Clone, Copy)]
struct ParseRule {
    prefix:     Option<ParseFunc>,
    infix:      Option<ParseFunc>,
    precidence: Precidence
}

struct RuleArgs {
    can_assign: bool,
    target:     Option<Expr>
}

impl ParseRule {
    fn new(
        prefix:     Option<ParseFunc>,
        infix:      Option<ParseFunc>,
        precidence: Precidence
    ) -> Self {
        Self {
            prefix,
            infix,
            precidence,
        }
    }
}

impl Parser {

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            parse_table: Self::parse_table(),

            // Debug info
            parse_stack:  vec![],
            depth:        0,
        }
    }


    fn parse_table() -> HashMap<TokenType, ParseRule> {
        use Precidence::*;

        HashMap::from([
            (TokenLeftParen,    ParseRule::new(Some(Self::parse_grouping_expr), Some(Self::parse_call_expr) ,   PrecCall)),
            (TokenDot,          ParseRule::new(None,                            Some(Self::parse_dot_expr),     PrecCall)),
            (TokenMinus,        ParseRule::new(Some(Self::parse_unary_expr),    Some(Self::parse_binary_expr),  PrecTerm)),
            (TokenPlus,         ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecTerm)),
            (TokenSlash,        ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecFactor)),
            (TokenStar,         ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecFactor)),
            (TokenBang,         ParseRule::new(Some(Self::parse_unary_expr),    None,                           PrecNone)),
            (TokenBangEqual,    ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecEquality)),
            (TokenEqualEqual,   ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecEquality)),
            (TokenGreater,      ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecComparison)),
            (TokenGreaterEqual, ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecComparison)),
            (TokenLess,         ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecComparison)),
            (TokenLessEqual,    ParseRule::new(None,                            Some(Self::parse_binary_expr),  PrecComparison)),
            (TokenIdentifier,   ParseRule::new(Some(Self::parse_variable_expr), None,                           PrecNone)),
            (TokenString,       ParseRule::new(Some(Self::parse_literal_expr),  None,                           PrecNone)),
            (TokenNumber,       ParseRule::new(Some(Self::parse_literal_expr),  None,                           PrecNone)),
            (TokenAnd,          ParseRule::new(None,                            Some(Self::parse_and_expr),     PrecAnd)),
            (TokenOr,           ParseRule::new(None,                            Some(Self::parse_or_expr),      PrecOr)),
            (TokenTrue,         ParseRule::new(Some(Self::parse_literal_expr),  None,                           PrecNone)),
            (TokenFalse,        ParseRule::new(Some(Self::parse_literal_expr),  None,                           PrecNone)),
            (TokenNil,          ParseRule::new(Some(Self::parse_literal_expr),  None,                           PrecNone)),
            (TokenSuper,        ParseRule::new(Some(Self::parse_super_expr),    None,                           PrecNone)),
            (TokenThis,         ParseRule::new(Some(Self::parse_this_expr),     None,                           PrecNone)),
            (TokenSemicolon,    ParseRule::new(None,                            None,                           PrecNone)),
            (TokenRightBrace,   ParseRule::new(None,                            None,                           PrecNone)),
            (TokenRightParen,   ParseRule::new(None,                            None,                           PrecNone)),
        ])
    }

    //#region declarations


    fn parse_declaration(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_declaration");

        let result: ParseResult<Stmt> = match self.advance().type_ {

            TokenClass => self.parse_class_decl(),
            TokenFun   => self.parse_function_decl(FunctionType::Function).map(|f| Stmt::Function(f)),
            TokenVar   => self.parse_var_decl(),

            _ => {
                self.roll_back();
                self.parse_statement()
            }
        };

        if result.is_err() {
            self.synchronize();
        }

        self.debug_parse_end();
        result
    }

    fn parse_class_decl(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_class_decl");

        let name = self.consume(TokenIdentifier, MissingClassIdentifier)?;

        let mut superclass = None;
        if self.match_(&[TokenLess]) {

            self.consume(TokenIdentifier, MissingSuperclassIdentifier)?;
            let name = self.previous();
            superclass = Some(Variable { name, });
        }

        self.consume(TokenLeftBrace, MissingClassOpenCurly)?;

        let mut methods = vec![];
        while !self.check(TokenRightBrace) && !self.is_eof() {
            methods.push(self.parse_function_decl(FunctionType::Method)?);
        }

        self.consume(TokenRightBrace, MissingClassCloseCurly)?;

        self.debug_parse_end();
        Ok(Class::new(name, superclass, methods))
    }

    fn parse_function_decl(&mut self, type_: FunctionType) -> ParseResult<FunctionStmt> {
        self.debug_parse_start("parse_function_decl");

        let name = self.consume(TokenIdentifier, MissingFunctionIdentifier(type_))?;

        self.consume(TokenLeftParen, MissingFunctionOpenParen(type_))?;

        let mut params = vec![];
        if !self.check(TokenRightParen) {

            params.push(self.consume(TokenIdentifier, MissingParameterIdentifier)?);

            while self.match_(&[TokenComma]) {
                if params.len() > 255 {
                    return Err(self.error(FunctionTooManyParameters))
                }
                params.push(self.consume(TokenIdentifier, MissingParameterIdentifier)?);
            }
        }

        self.consume(TokenRightParen, MissingFunctionCloseParen)?;
        self.consume(TokenLeftBrace,  MissingFunctionOpenBrace(type_))?;

        let body = self.parse_block_statement()?;

        self.debug_parse_end();
        Ok(FunctionStmt::new(name, params, *body.stmts))
    }

    fn parse_var_decl(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_var_decl");

        let name = self.consume(TokenIdentifier, MissingVariableIdentifier)?;

        let initializer = if self.match_(&[TokenEqual]) {

            Some(self.parse_expression(None)?)
        } else {
            None
        };

        self.consume(TokenSemicolon, MissingVariableSemicolon)?;

        self.debug_parse_end();
        Ok(VarStmt::new(name, initializer))
    }

    //#endregion

    //#region Statements

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_statement");


        let result = match self.advance().type_ {
            TokenFor       => self.parse_for_statement(),
            TokenIf        => self.parse_if_statement(),
            TokenPrint     => self.parse_print_statement(),
            TokenReturn    => self.parse_return_statement(),
            TokenWhile     => self.parse_while_statement(),
            TokenLeftBrace => self.parse_block_statement().map(|block| Stmt::Block(block)),
            _ => {
                self.roll_back();
                self.parse_expression_statement()
            },
        };

        self.debug_parse_end();
        result
    }

    fn parse_block_statement(&mut self) -> ParseResult<Block> {
        self.debug_parse_start("parse_block_statement");


        let mut statements = vec![];

        while !self.check(TokenRightBrace) && !self.is_eof() {
            statements.push(self.parse_declaration()?);
        }

        self.consume(TokenRightBrace, MissingBlockCloseBrace)?;

        self.debug_parse_end();
        Ok(Block { stmts: Box::new(statements), })
    }

    fn parse_for_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_for_statement");

        self.consume(TokenLeftParen, MissingForOpenParen)?;

        let initializer = match self.peek().type_ {
            TokenSemicolon => None,
            TokenVar       => Some(self.parse_var_decl()?),
            _              => Some(self.parse_expression_statement()?),
        };

        let condition = if self.check(TokenSemicolon) {
            Some(self.parse_expression(None)?)
        } else {
            None
        };
        self.consume(TokenSemicolon, MissingForConditionDelimiter)?;

        let increment = if self.check(TokenRightParen) {
            Some(self.parse_expression(None)?)
        } else {
            None
        };
        self.consume(TokenRightParen, MissingForCloseParen)?;

        let mut body = self.parse_statement()?;

        if let Some(increment) = increment {
            body = Block::new(vec![
                body,
                ExpressionStmt::new(increment)
            ]);
        }

        let condition = condition.unwrap_or(
            Literal::new(Token::new_true())
        );

        let body = WhileStmt::new(condition, body);

        let body = match initializer {
            None       => body,
            Some(init) => Block::new(vec![init, body]),
        };

        self.debug_parse_end();
        Ok(body)
    }


    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_if_statement");


        self.consume(TokenLeftParen, MissingIfOpenParen)?;
        let condition = self.parse_expression(None)?;
        self.consume(TokenRightParen, MissingIfCloseParen)?;

        let then_branch = self.parse_statement()?;
        let else_branch = if self.match_(&[TokenElse]) {
            Some(self.parse_statement()?)
        } else {
            None
        };

        self.debug_parse_end();
        Ok(IfStmt::new(condition, then_branch, else_branch))
    }

    fn parse_print_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_print_statement");

        let value = self.parse_expression(None)?;

        self.consume(TokenSemicolon, MissingPrintSemicolon)?;

        self.debug_parse_end();
        Ok(PrintStmt::new(value))
    }

    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_return_statement");


        let keyword = self.previous();
        let value = if !self.check(TokenSemicolon) {
            Some(self.parse_expression(None)?)
        } else {
            None
        };

        self.consume(TokenSemicolon, MissingReturnSemicolon)?;

        self.debug_parse_end();
        Ok(ReturnStmt::new(keyword, value))
    }

    fn parse_while_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_while_statement");


        self.consume(TokenLeftParen, MissingWhileOpenParen)?;
        let condition = self.parse_expression(None)?;
        self.consume(TokenRightParen, MissingWhileCloseParen)?;

        let body = self.parse_statement()?;

        self.debug_parse_end();
        Ok(WhileStmt::new(condition, body))
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Stmt> {
        self.debug_parse_start("parse_expression_statement");


        let expr = self.parse_expression(None)?;
        self.consume(TokenSemicolon, MissingExpressionStmtSemicolon)?;

        self.debug_parse_end();
        Ok(ExpressionStmt::new(expr))

    }

    fn parse_expression(&mut self, target: Option<Expr>) -> ParseResult<Expr> {
        self.debug_parse_start("parse_expression");

        let result = self.parse_precedence(Precidence::PrecAssignment, target);

        self.debug_parse_end();
        result
    }

    fn parse_precedence(&mut self, prec: Precidence, target: Option<Expr>) -> ParseResult<Expr> {
        self.debug_parse_start("parse_precedence");


        let op   = self.advance();
        let rule = self.get_rule(op.type_)?;

        let can_assign = prec <= Precidence::PrecAssignment;

        
        let prefix = rule.prefix.ok_or_else(|| self.panic("Missing Prefix Rule"))?;
        let mut target = prefix(self, RuleArgs {
            can_assign,
            target,
        })?;


        while prec <= self.get_rule(self.peek().type_)?.precidence {
            let op   = self.advance();
            let rule = self.get_rule(op.type_)?;

            let infix = rule.infix.ok_or_else(|| self.panic("Missing Infix Rule"))?;

            target = infix(self, RuleArgs {
                can_assign,
                target: Some(target),
            })?;
        }

        if can_assign && self.match_(&[TokenEqual]) {
            return Err(self.error(InvalidAssignmentTarget))
        }


        self.debug_parse_end();
        Ok(target)
    }

    // Rules

    fn parse_grouping_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_grouping_expr");


        let expr = self.parse_expression(None)?;

        self.consume(TokenRightParen, MissingGroupingCloseParen)?;

        self.debug_parse_end();
        Ok(expr)
    }

    fn parse_call_expr(&mut self, rule_args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_call_expr");


        let callee = rule_args.target.ok_or_else(|| self.panic("Missing Callee for call expression"))?;

        let callee = match callee {
            Expr::Get(get) => *get.expr,
            _              => callee,
        };

        let (paren, arguments) = self.parse_argument_list()?;

        self.debug_parse_end();
        Ok(Call::new(callee, paren, arguments))
    }

    fn parse_dot_expr(&mut self, rule_args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_dot_expr");


        let target = rule_args.target.ok_or_else(|| self.panic("Missing target for dot expression"))?;
        let name   = self.consume(TokenIdentifier, MissingPropertyIdentifier)?;

        let result = if rule_args.can_assign && self.match_(&[TokenEqual]) {
            let value = self.parse_expression(Some(target.clone()))?;

            match value {
                Expr::Variable(_)   => Ok(Assign::new(name, value)),
                Expr::Get     (get) => Ok(Set   ::new(target, name, *get.expr)),
                _                   => Err(self.error(InvalidAssignmentTarget))
            }

        }
        else if self.match_(&[TokenLeftParen]) {
            let (paren, arguments) = self.parse_argument_list()?;
            Ok(Call::new(target, paren, arguments))
        }
        else {
            match target {
                Expr::Get(v) => Ok(Get::new(*v.expr, name)),
                _            => Ok(Get::new(target,  name)),
            }
        };

        self.debug_parse_end();
        result

    }

    fn parse_unary_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_unary_expr");


        let operator = self.previous();
        let operand  = self.parse_precedence(Precidence::PrecUnary, args.target);

        self.debug_parse_end();
        Ok(Unary::new(operator, operand?))
    }

    fn parse_binary_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_binary_expr");


        let op   = self.previous();
        let rule = self.get_rule(op.type_)?;

        let left  = args.target.ok_or_else(|| self.panic("Missing left operand for binary expression"))?;
        let right = self.parse_precedence(rule.precidence.next(), None)?;

        self.debug_parse_end();
        Ok(Binary::new(left, op, right))
    }


    fn parse_argument_list(&mut self) -> ParseResult<(Token, Vec<Expr>)> {
        self.debug_parse_start("parse_argument_list");


        let mut args = vec![];

        if !self.check(TokenRightParen) {

            args.push(self.parse_expression(None)?);

            while self.match_(&[TokenComma]) {

                if args.len() > 255 {
                    return Err(self.error(FunctionTooManyParameters));
                }
                
                args.push(self.parse_expression(None)?);
            }
        }

        let paren = self.consume(TokenRightParen, MissingFunctionCloseParen)?;

        self.debug_parse_end();
        Ok((paren, args))
    }

    fn parse_variable_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_variable_expr");

        let name   = self.previous();
        let target = Variable::new(name.clone());

        let result = if args.can_assign && self.match_(&[TokenEqual]) {
            let value  = self.parse_expression(None)?;
            Ok(Set::new(target, name, value))
        }
        else {
            Ok(Get::new(target, name))
        };

        self.debug_parse_end();
        result
    }

    fn parse_literal_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_log("parse_literal_expr");
        Ok(Literal::new(self.previous()))
    }

    fn parse_and_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_and_expr");
        let result = self.parse_logical_expr(args, Precidence::PrecAnd);

        self.debug_parse_end();
        result
    }

    fn parse_or_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_or_expr");
        let result = self.parse_logical_expr(args, Precidence::PrecOr);
        self.debug_parse_end();
        result
    }

    fn parse_logical_expr(&mut self, args: RuleArgs, prec: Precidence) -> ParseResult<Expr> {
        self.debug_parse_start("parse_logical_expr");
        let operator = self.previous();
        let left     = args.target.ok_or_else(|| self.panic("Missing target for logical expression"))?;

        let right = self.parse_precedence(prec, None)?;

        self.debug_parse_end();
        Ok(Logical::new(left, operator, right))

    }

    fn parse_super_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_start("parse_super_expr");
        let keyword = self.previous();
        self.consume(TokenDot, MissingSuperDot)?;
        let method = self.consume(TokenIdentifier, MissingSuperPropertyIdentifier)?;

        self.debug_parse_end();
        Ok(Super::new(keyword, method))
    }

    fn parse_this_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        self.debug_parse_log("parse_this_expr");
        Ok(This::new(self.previous()))
    }


    //#endregion

    //#region Expressions
    //#endregion

    // Utility functions

    // todo: try to rustify this

    pub fn match_(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_eof() {
            self.current += 1;
        }

        self.previous()
    }

    pub fn roll_back(&mut self) {
        self.current -= 1;
    }

    pub fn check(&self, type_: TokenType) -> bool {
        if self.is_eof() {
            return false;
        }
        self.peek().type_ == type_
    }

    pub fn is_eof(&self) -> bool {
        self.peek().type_ == TokenEOF || self.current >= self.tokens.len()
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.current -1].to_owned()
    }

    pub fn consume(&mut self, token_type: TokenType, error_type: ParseErrorType) -> ParseResult<Token> {
        if self.check(token_type) {
            return Ok(self.advance().to_owned());
        }

        Err(self.error(error_type))
    }

    pub fn error(&self, type_: ParseErrorType) -> ParseError {
        ParseError {
            type_,
            token: self.previous(),
        }
    }

    pub fn panic(&self, msg: &str) -> ! {
        eprintln!("Error while parsing Token: {}\n> {}", self.previous(), msg);

        panic!()
    }

    fn get_rule(&self, op: TokenType) -> ParseResult<ParseRule> {
        Ok(*self.parse_table
            .get(&op)
            .ok_or(self.error(MissingExpression(self.previous())))?
        )
    }

    fn synchronize(&mut self) {

        while !self.is_eof() {
            if self.previous().type_ == TokenSemicolon {
                return;
            }

            match self.peek().type_ {
                  TokenClass
                | TokenFun
                | TokenVar
                | TokenFor
                | TokenIf
                | TokenWhile
                | TokenPrint
                | TokenReturn => {
                    return;
                }

                _ => {}
            }

            self.advance();
        }

    }

    fn debug_parse_log(&mut self, name: &str) {
        let ind = self.parse_stack.len();

        print_ind(ind, name);
    }


    fn debug_parse_start(&mut self, name: &str) {
        let ind = self.parse_stack.len();

        print_ind(ind, &format!("{} ({}) {{", name, self.peek()));

        self.parse_stack.push(name.to_owned());
    }

    fn debug_parse_end(&mut self) {

        self.parse_stack.pop();

        let ind = self.parse_stack.len();

        print_ind(ind, "}");
    }

}


fn print_ind(ind: usize, msg: &str) {
    println!("{}{}", " ".repeat(ind * 4), msg);
}