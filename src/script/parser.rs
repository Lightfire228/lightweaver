use std::collections::HashMap;

use crate::script::ast::*;

use super::tokens::{Token, TokenType};

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

type Prec = Precidence;

type Pe = ParseErrorType;
type Tt = TokenType;

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
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary
}

impl Precidence {
    fn next(&self) -> Precidence {
        match self {
            Prec::None       => Prec::Assignment,
            Prec::Assignment => Prec::Or,
            Prec::Or         => Prec::And,
            Prec::And        => Prec::Equality,
            Prec::Equality   => Prec::Comparison,
            Prec::Comparison => Prec::Term,
            Prec::Term       => Prec::Factor,
            Prec::Factor     => Prec::Unary,
            Prec::Unary      => Prec::Call,
            Prec::Call       => Prec::Primary,
            Prec::Primary    => panic!("Unknown precidence")
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

        HashMap::from([
            (Tt::LeftParen,    ParseRule::new(Some(Self::parse_grouping_expr), Some(Self::parse_call_expr) ,   Prec::Call)),
            (Tt::Dot,          ParseRule::new(None,                            Some(Self::parse_dot_expr),     Prec::Call)),
            (Tt::Minus,        ParseRule::new(Some(Self::parse_unary_expr),    Some(Self::parse_binary_expr),  Prec::Term)),
            (Tt::Plus,         ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Term)),
            (Tt::Slash,        ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Factor)),
            (Tt::Star,         ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Factor)),
            (Tt::Bang,         ParseRule::new(Some(Self::parse_unary_expr),    None,                           Prec::None)),
            (Tt::BangEqual,    ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Equality)),
            (Tt::EqualEqual,   ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Equality)),
            (Tt::Greater,      ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Comparison)),
            (Tt::GreaterEqual, ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Comparison)),
            (Tt::Less,         ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Comparison)),
            (Tt::LessEqual,    ParseRule::new(None,                            Some(Self::parse_binary_expr),  Prec::Comparison)),
            (Tt::Identifier,   ParseRule::new(Some(Self::parse_variable_expr), None,                           Prec::None)),
            (Tt::String,       ParseRule::new(Some(Self::parse_literal_expr),  None,                           Prec::None)),
            (Tt::Number,       ParseRule::new(Some(Self::parse_literal_expr),  None,                           Prec::None)),
            (Tt::And,          ParseRule::new(None,                            Some(Self::parse_and_expr),     Prec::And)),
            (Tt::Or,           ParseRule::new(None,                            Some(Self::parse_or_expr),      Prec::Or)),
            (Tt::True,         ParseRule::new(Some(Self::parse_literal_expr),  None,                           Prec::None)),
            (Tt::False,        ParseRule::new(Some(Self::parse_literal_expr),  None,                           Prec::None)),
            (Tt::Nil,          ParseRule::new(Some(Self::parse_literal_expr),  None,                           Prec::None)),
            (Tt::Super,        ParseRule::new(Some(Self::parse_super_expr),    None,                           Prec::None)),
            (Tt::This,         ParseRule::new(Some(Self::parse_this_expr),     None,                           Prec::None)),
            (Tt::Semicolon,    ParseRule::new(None,                            None,                           Prec::None)),
            (Tt::RightBrace,   ParseRule::new(None,                            None,                           Prec::None)),
            (Tt::RightParen,   ParseRule::new(None,                            None,                           Prec::None)),
        ])
    }

    //#region declarations


    fn parse_declaration(&mut self) -> ParseResult<Stmt> {
        let result: ParseResult<Stmt> = match self.advance().type_ {

            Tt::Class => self.parse_class_decl(),
            Tt::Fun   => self.parse_function_decl(FunctionType::Function).map(|f| Stmt::Function(f)),
            Tt::Var   => self.parse_var_decl(),

            _ => {
                self.roll_back();
                self.parse_statement()
            }
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn parse_class_decl(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(Tt::Identifier, Pe::MissingClassIdentifier)?;

        let mut superclass = None;
        if self.match_(&[Tt::Less]) {

            self.consume(Tt::Identifier, Pe::MissingSuperclassIdentifier)?;
            let name = self.previous();
            superclass = Some(Variable { name, });
        }

        self.consume(Tt::LeftBrace, Pe::MissingClassOpenCurly)?;

        let mut methods = vec![];
        while !self.check(Tt::RightBrace) && !self.is_eof() {
            methods.push(self.parse_function_decl(FunctionType::Method)?);
        }

        self.consume(Tt::RightBrace, Pe::MissingClassCloseCurly)?;

        Ok(Class::new(name, superclass, methods))
    }

    fn parse_function_decl(&mut self, type_: FunctionType) -> ParseResult<FunctionStmt> {

        let name = self.consume(Tt::Identifier, Pe::MissingFunctionIdentifier(type_))?;

        self.consume(Tt::LeftParen, Pe::MissingFunctionOpenParen(type_))?;

        let mut params = vec![];
        if !self.check(Tt::RightParen) {

            params.push(self.consume(Tt::Identifier, Pe::MissingParameterIdentifier)?);

            while self.match_(&[Tt::Comma]) {
                if params.len() > 255 {
                    return Err(self.error(Pe::FunctionTooManyParameters))
                }
                params.push(self.consume(Tt::Identifier, Pe::MissingParameterIdentifier)?);
            }
        }

        self.consume(Tt::RightParen, Pe::MissingFunctionCloseParen)?;
        self.consume(Tt::LeftBrace,  Pe::MissingFunctionOpenBrace(type_))?;

        let body = self.parse_block_statement()?;

        Ok(FunctionStmt::new(name, params, *body.stmts))
    }

    fn parse_var_decl(&mut self) -> ParseResult<Stmt> {

        let name = self.consume(Tt::Identifier, Pe::MissingVariableIdentifier)?;

        let initializer = if self.match_(&[Tt::Equal]) {

            Some(self.parse_expression(None)?)
        } else {
            None
        };

        self.consume(Tt::Semicolon, Pe::MissingVariableSemicolon)?;

        Ok(VarStmt::new(name, initializer))
    }

    //#endregion

    //#region Statements

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        match self.advance().type_ {
            Tt::For       => self.parse_for_statement(),
            Tt::If        => self.parse_if_statement(),
            Tt::Print     => self.parse_print_statement(),
            Tt::Return    => self.parse_return_statement(),
            Tt::While     => self.parse_while_statement(),
            Tt::LeftBrace => self.parse_block_statement().map(|block| Stmt::Block(block)),
            _ => {
                self.roll_back();
                self.parse_expression_statement()
            },
        }
    }

    fn parse_block_statement(&mut self) -> ParseResult<Block> {
        let mut statements = vec![];

        while !self.check(Tt::RightBrace) && !self.is_eof() {
            statements.push(self.parse_declaration()?);
        }

        self.consume(Tt::RightBrace, Pe::MissingBlockCloseBrace)?;

        Ok(Block { stmts: Box::new(statements), })
    }

    fn parse_for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(Tt::LeftParen, Pe::MissingForOpenParen)?;

        let initializer = match self.peek().type_ {
            Tt::Semicolon => None,
            Tt::Var       => Some(self.parse_var_decl()?),
            _              => Some(self.parse_expression_statement()?),
        };

        let condition = if self.check(Tt::Semicolon) {
            Some(self.parse_expression(None)?)
        } else {
            None
        };
        self.consume(Tt::Semicolon, Pe::MissingForConditionDelimiter)?;

        let increment = if self.check(Tt::RightParen) {
            Some(self.parse_expression(None)?)
        } else {
            None
        };
        self.consume(Tt::RightParen, Pe::MissingForCloseParen)?;

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

        Ok(body)
    }


    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {

        self.consume(Tt::LeftParen, Pe::MissingIfOpenParen)?;
        let condition = self.parse_expression(None)?;
        self.consume(Tt::RightParen, Pe::MissingIfCloseParen)?;

        let then_branch = self.parse_statement()?;
        let else_branch = if self.match_(&[Tt::Else]) {
            Some(self.parse_statement()?)
        } else {
            None
        };

        Ok(IfStmt::new(condition, then_branch, else_branch))
    }

    fn parse_print_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.parse_expression(None)?;

        self.consume(Tt::Semicolon, Pe::MissingPrintSemicolon)?;

        Ok(PrintStmt::new(value))
    }

    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {

        let keyword = self.previous();
        let value = if !self.check(Tt::Semicolon) {
            Some(self.parse_expression(None)?)
        } else {
            None
        };

        self.consume(Tt::Semicolon, Pe::MissingReturnSemicolon)?;

        Ok(ReturnStmt::new(keyword, value))
    }

    fn parse_while_statement(&mut self) -> ParseResult<Stmt> {

        self.consume(Tt::LeftParen, Pe::MissingWhileOpenParen)?;
        let condition = self.parse_expression(None)?;
        self.consume(Tt::RightParen, Pe::MissingWhileCloseParen)?;

        let body = self.parse_statement()?;

        Ok(WhileStmt::new(condition, body))
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Stmt> {

        let expr = self.parse_expression(None)?;
        self.consume(Tt::Semicolon, Pe::MissingExpressionStmtSemicolon)?;

        Ok(ExpressionStmt::new(expr))

    }

    fn parse_expression(&mut self, target: Option<Expr>) -> ParseResult<Expr> {
        self.parse_precedence(Prec::Assignment, target)
    }

    fn parse_precedence(&mut self, prec: Precidence, target: Option<Expr>) -> ParseResult<Expr> {

        let op   = self.advance();
        let rule = self.get_rule(op.type_)?;

        let can_assign = prec <= Prec::Assignment;


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

        if can_assign && self.match_(&[Tt::Equal]) {
            return Err(self.error(Pe::InvalidAssignmentTarget))
        }

        Ok(target)
    }

    // Rules

    fn parse_grouping_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        let expr = self.parse_expression(None)?;

        self.consume(Tt::RightParen, Pe::MissingGroupingCloseParen)?;

        Ok(expr)
    }

    fn parse_call_expr(&mut self, rule_args: RuleArgs) -> ParseResult<Expr> {

        let callee = rule_args.target.ok_or_else(|| self.panic("Missing Callee for call expression"))?;
        let callee = collapse_get(callee);

        let (paren, arguments) = self.parse_argument_list()?;

        Ok(Call::new(callee, paren, arguments))
    }

    fn parse_dot_expr(&mut self, rule_args: RuleArgs) -> ParseResult<Expr> {

        let target = rule_args.target.ok_or_else(|| self.panic("Missing target for dot expression"))?;
        let name   = self.consume(Tt::Identifier, Pe::MissingPropertyIdentifier)?;

        let result = if rule_args.can_assign && self.match_(&[Tt::Equal]) {
            let value = self.parse_expression(Some(target.clone()))?;

            match value {
                Expr::Variable(_)   => Ok(Assign::new(Variable::new(name), value)),
                Expr::Get     (get) => Ok(Set   ::new(target, name, *get.expr)),
                _                   => Err(self.error(Pe::InvalidAssignmentTarget))
            }

        }
        else if self.match_(&[Tt::LeftParen]) {
            let (paren, arguments) = self.parse_argument_list()?;
            Ok(Call::new(target, paren, arguments))
        }
        else {
            match target {
                Expr::Get(v) => Ok(Get::new(*v.expr, name)),
                _            => Ok(Get::new(target,  name)),
            }
        };

        result

    }

    fn parse_unary_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {

        let operator = self.previous();
        let operand  = self.parse_precedence(Prec::Unary, args.target);

        Ok(UnaryOperator::new(operator, operand?))
    }

    fn parse_binary_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {

        let op   = self.previous();
        let rule = self.get_rule(op.type_)?;

        let left  = args.target.ok_or_else(|| self.panic("Missing left operand for binary expression"))?;
        let right = self.parse_precedence(rule.precidence.next(), None)?;

        Ok(BinaryOperator::new(left, op, right))
    }


    fn parse_argument_list(&mut self) -> ParseResult<(Token, Vec<Expr>)> {
        let mut args = vec![];

        if !self.check(Tt::RightParen) {

            args.push(self.parse_expression(None)?);

            while self.match_(&[Tt::Comma]) {

                if args.len() > 255 {
                    return Err(self.error(Pe::FunctionTooManyParameters));
                }

                args.push(self.parse_expression(None)?);
            }
        }

        let paren = self.consume(Tt::RightParen, Pe::MissingFunctionCloseParen)?;

        Ok((paren, args))
    }

    fn parse_variable_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        let name = self.previous();
        let target = Variable::new(name.clone());

        let result = if args.can_assign && self.match_(&[Tt::Equal]) {
            let value  = self.parse_expression(None)?;
            Ok(Assign::new(target, value))
        }
        else {
            Ok(Get::new(target.as_expr(), name))
        };

        result
    }

    fn parse_literal_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        Ok(Literal::new(self.previous()))
    }

    fn parse_and_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.parse_logical_expr(args, Prec::And)
    }

    fn parse_or_expr(&mut self, args: RuleArgs) -> ParseResult<Expr> {
        self.parse_logical_expr(args, Prec::Or)
    }

    fn parse_logical_expr(&mut self, args: RuleArgs, prec: Precidence) -> ParseResult<Expr> {
        let operator = self.previous();
        let left     = args.target.ok_or_else(|| self.panic("Missing target for logical expression"))?;

        let right = self.parse_precedence(prec, None)?;

        Ok(Logical::new(left, operator, right))

    }

    fn parse_super_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
        let keyword = self.previous();
        self.consume(Tt::Dot, Pe::MissingSuperDot)?;
        let method = self.consume(Tt::Identifier, Pe::MissingSuperPropertyIdentifier)?;

        Ok(Super::new(keyword, method))
    }

    fn parse_this_expr(&mut self, _: RuleArgs) -> ParseResult<Expr> {
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
        self.peek().type_ == Tt::EOF || self.current >= self.tokens.len()
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
            .ok_or(self.error(Pe::MissingExpression(self.previous())))?
        )
    }

    fn synchronize(&mut self) {

        while !self.is_eof() {
            if self.previous().type_ == Tt::Semicolon {
                return;
            }

            match self.peek().type_ {
                  Tt::Class
                  | Tt::Fun
                  | Tt::Var
                  | Tt::For
                  | Tt::If
                  | Tt::While
                  | Tt::Print
                  | Tt::Return => {
                    return;
                }

                _ => {}
            }

            self.advance();
        }

    }

}


fn print_ind(ind: usize, msg: &str) {
    println!("{}{}", " ".repeat(ind * 4), msg);
}


fn collapse_get(mut expr: Expr) -> Expr {

    loop {
        expr = match expr {
            Expr::Get(get) => *get.expr,
            _              => return expr,
        }

    }
}
