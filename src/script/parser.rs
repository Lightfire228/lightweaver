use crate::script::ast::*;

use crate::script::tokens::TokenType::*;

type ParseResult<T> = Result<T, ParseError>;

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

struct Parser {
    tokens:  Vec<Token>,
    current: usize,
}


pub struct ParseError {
    pub type_: ParseErrorType,
    pub token: Token,

}

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
    MissingExpression,
}

use ParseErrorType::*;

use super::tokens::{Token, TokenType};

#[derive(Clone, Copy)]
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

impl Parser {

    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    //#region declarations

    fn parse_declaration(&mut self) -> ParseResult<Stmt> {
        let result: ParseResult<Stmt> = match self.peek().type_ {

            TokenClass => { self.advance(); self.parse_class_decl() },
            TokenFun   => { self.advance(); self.parse_function_decl(FunctionType::Function).map(|x| Ok(Stmt::Function(x)))?},
            TokenVar   => { self.advance(); self.parse_var_decl() },

            _ => self.parse_statement(),
        };

        if result.is_err() {
            self.synchronize();
        }

        result

    }

    fn parse_class_decl(&mut self) -> ParseResult<Stmt> {
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

        Ok(Class::new(name, superclass, methods))
    }

    fn parse_function_decl(&mut self, type_: FunctionType) -> ParseResult<FunctionStmt> {
        let name = self.consume(TokenIdentifier, MissingFunctionIdentifier(type_))?;

        self.consume(TokenLeftParen, MissingFunctionOpenParen(type_))?;

        let mut params = vec![];
        if !self.check(TokenRightParen) {

            params.push(self.consume(TokenIdentifier, MissingParameterIdentifier)?);

            while self.match_(&[TokenComma]) {
                if params.len() > 255 {
                    return self.error(FunctionTooManyParameters)
                }
                params.push(self.consume(TokenIdentifier, MissingParameterIdentifier)?);
            }
        }

        self.consume(TokenRightParen, MissingFunctionCloseParen)?;
        self.consume(TokenLeftBrace,  MissingFunctionOpenBrace(type_))?;

        let body = self.parse_block_statement()?;

        Ok(FunctionStmt::new(name, params, *body.stmts))

    }

    fn parse_var_decl(&mut self) -> ParseResult<Stmt> {

        let name = self.consume(TokenIdentifier, MissingVariableIdentifier)?;

        let mut initializer = None;

        if self.match_(&[TokenEqual]) {
            initializer = Some(self.parse_expression()?);
        }

        self.consume(TokenSemicolon, MissingVariableSemicolon)?;

        Ok(VarStmt::new(name, initializer))
    }

    //#endregion


    fn parse_statement(&mut self) -> ParseResult<Stmt> {

        match self.peek().type_ {
            TokenFor       => { self.advance(); self.parse_for_statement() },
            TokenIf        => { self.advance(); self.parse_if_statement() },
            TokenPrint     => { self.advance(); self.parse_print_statement() },
            TokenReturn    => { self.advance(); self.parse_return_statement() },
            TokenWhile     => { self.advance(); self.parse_while_statement() },
            TokenLeftBrace => { self.advance(); self.parse_block_statement().map(|block| Stmt::Block(block)) }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenLeftParen, MissingForOpenParen)?;

        let initializer = match self.peek().type_ {
            TokenSemicolon => None,
            TokenVar       => Some(self.parse_var_decl()?),
            _              => Some(self.parse_expression_statement()?),
        };

        let condition = if self.check(TokenSemicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume(TokenSemicolon, MissingForConditionDelimiter)?;

        let increment = if self.check(TokenRightParen) {
            Some(self.parse_expression()?)
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

        Ok(body)
    }

    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {

        self.consume(TokenLeftParen, MissingIfOpenParen)?;
        let condition = self.parse_expression()?;
        self.consume(TokenRightParen, MissingIfCloseParen)?;

        let then_branch = self.parse_statement()?;
        let else_branch = if self.match_(&[TokenElse]) {
            Some(self.parse_statement()?)
        } else {
            None
        };

        Ok(IfStmt::new(condition, then_branch, else_branch))
    }

    fn parse_print_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.parse_expression()?;

        self.consume(TokenSemicolon, MissingPrintSemicolon)?;

        Ok(PrintStmt::new(value))
    }

    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {

        let keyword = self.previous();
        let value = if !self.check(TokenSemicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenSemicolon, MissingReturnSemicolon)?;

        Ok(ReturnStmt::new(keyword, value))
    }

    fn parse_while_statement(&mut self) -> ParseResult<Stmt> {

        self.consume(TokenLeftParen, MissingWhileOpenParen)?;
        let condition = self.parse_expression()?;
        self.consume(TokenRightParen, MissingWhileCloseParen)?;

        let body = self.parse_statement()?;

        Ok(WhileStmt::new(condition, body))
    }

    fn parse_block_statement(&mut self) -> ParseResult<Block> {

        let mut statements = vec![];

        while !self.check(TokenRightBrace) && !self.is_eof() {
            statements.push(self.parse_declaration()?);
        }

        self.consume(TokenRightBrace, MissingBlockCloseBrace)?;

        Ok(Block { stmts: Box::new(statements), })
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Stmt> {

        let expr = self.parse_expression()?;
        self.consume(TokenSemicolon, MissingExpressionStmtSemicolon)?;

        Ok(ExpressionStmt::new(expr))

    }

    //#region Statements

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> ParseResult<Expr> {

        let mut expr = self.parse_or_expr()?;

        if self.match_(&[TokenEqual]) {
            let value = self.parse_assignment_expr()?;

            expr = match expr {
                Expr::Variable(var)  => Assign::new(var.name, value),
                Expr::Get     (expr) => Set   ::new(*expr.expr, expr.name, value),
                _                    => self.error(InvalidAssignmentTarget)?
            };
        }

        Ok(expr)
    }

    fn parse_or_expr(&mut self) -> ParseResult<Expr> {

        let mut expr = self.parse_and_expr()?;

        while self.match_(&[TokenOr]) {
            let operator = self.previous();
            let right    = self.parse_and_expr()?;
            expr         = Logical::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_and_expr(&mut self) -> ParseResult<Expr> {

        let mut expr = self.parse_equality_expr()?;

        while self.match_(&[TokenOr]) {
            let operator = self.previous();
            let right    = self.parse_equality_expr()?;
            expr         = Logical::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_equality_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_comparison_expr()?;

        while self.match_(&[TokenBang, TokenBangEqual]) {
            let operator = self.previous();
            let right    = self.parse_comparison_expr()?;

            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_comparison_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_term_expr()?;

        while self.match_(&[TokenGreater, TokenGreaterEqual, TokenLess, TokenLessEqual]) {
            let operator = self.previous();
            let right    = self.parse_term_expr()?;

            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_term_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_factor_expr()?;

        while self.match_(&[TokenStar, TokenSlash]) {
            let operator = self.previous();
            let right    = self.parse_factor_expr()?;

            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_factor_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_unary_expr()?;

        while self.match_(&[TokenMinus, TokenPlus]) {
            let operator = self.previous();
            let right    = self.parse_unary_expr()?;

            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    fn parse_unary_expr(&mut self) -> ParseResult<Expr> {
        if self.match_(&[TokenBang, TokenMinus]) {
            let operator = self.previous();
            let right    = self.parse_unary_expr()?;

            return Ok(Unary::new(operator, right));
        }

        Ok(self.parse_call_expr()?)
    }

    fn parse_call_expr(&mut self) -> ParseResult<Expr> {

        let mut expr = self.parse_primary_expr()?;

        loop {
            match self.peek().type_ {
                TokenLeftParen => {
                    self.advance();
                    expr = self.finish_call(expr)?;
                }
                TokenDot       => {
                    self.advance();
                    let name = self.consume(TokenIdentifier, MissingPropertyIdentifier)?;
                    expr = Get::new(expr, name);
                }
                _              => { break; }
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParseResult<Expr> {

        let mut args = vec![];

        if !self.check(TokenRightParen) {

            args.push(self.parse_expression()?);

            while self.match_(&[TokenComma]) {
                if args.len() > 255 {
                    return self.error(FunctionTooManyParameters)?
                }

                args.push(self.parse_expression()?);
            }
        }

        let paren = self.consume(TokenRightParen, MissingFunctionCloseParen)?;

        Ok(Call::new(callee, paren, args))
    }

    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {

        let token = self.advance();

        Ok(match token.type_ {
              TokenFalse
            | TokenTrue
            | TokenNil      => Literal ::new(token),

              TokenNumber
            | TokenString   => Literal ::new(self.previous()),

            TokenIdentifier => Variable::new(self.previous()),

            TokenThis       => This    ::new(self.previous()),

            TokenSuper      => {
                let keyword = self.previous();
                self.consume(TokenDot, MissingSuperDot)?;
                let method = self.consume(TokenIdentifier, MissingSuperPropertyIdentifier)?;

                Super::new(keyword, method)
            }


            TokenLeftParen  => {
                let expr = self.parse_expression()?;
                self.consume(TokenRightParen, MissingGroupingCloseParen)?;

                Grouping::new(expr)
            }

            _               => self.error(MissingExpression)?
        })
    }




    //#endregion

    // Utility functions

    // todo: try to rustify this

    fn match_(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_eof() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_eof() {
            return false;
        }
        self.peek().type_ == type_
    }

    fn is_eof(&self) -> bool {
        self.peek().type_ == TokenEOF || self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current -1].to_owned()
    }

    fn consume(&mut self, token_type: TokenType, error_type: ParseErrorType) -> ParseResult<Token> {
        if self.check(token_type) {
            return Ok(self.advance().to_owned());
        }

        self.error(error_type)
    }

    fn error<T>(&mut self, type_: ParseErrorType) -> ParseResult<T> {
        Err(ParseError {
            type_,
            token: self.previous(),
        })

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
}
