#![allow(unused_variables)] // TODO:

use crate::script::{ast::{Ast, BinaryOpType, BinaryOperator, Expr, ExpressionStmt, Literal, LiteralType, LogicalType, Stmt, UnaryOpType, UnaryOperator}, tokens::TokenType};

use super::{chunk::{Chunk, OpCode}, value::Value};

use OpCode   ::*;
use TokenType::*;


pub struct Compiler {
    chunks: Vec<Chunk>,
    line:   usize,
}

impl Compiler {

    fn new() -> Self {
        let name = "script".to_owned();

        Self {
            chunks: vec![Chunk::new(name)],
            line:   0,
        }
    }

    pub fn compile(ast: Ast) -> Vec<Chunk> {

        let mut compiler = Self::new();

        for stmt in ast.stmts {
            compiler.compile_stmt(stmt);
        }

        compiler.write_op(OpReturn);

        compiler.chunks

    }

    // Statements

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Block      (stmt) => todo!(),
            Stmt::Class      (stmt) => todo!(),
            Stmt::Expression (stmt) => self.compile_expr(stmt.expr),
            Stmt::Function   (stmt) => todo!(),
            Stmt::If         (stmt) => todo!(),
            Stmt::Print      (stmt) => todo!(),
            Stmt::Return     (stmt) => todo!(),
            Stmt::Var        (stmt) => todo!(),
            Stmt::While      (stmt) => todo!(),
        }
    }

    fn compile_expr_stmt(&mut self, expr_stmt: ExpressionStmt) {
        self.compile_expr(expr_stmt.expr);
    }

    // Expressions

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Assign   (expr) => todo!(),
            Expr::Binary   (expr) => self.compile_binary (expr),
            Expr::Call     (expr) => todo!(),
            Expr::Get      (expr) => todo!(),
            Expr::Grouping (expr) => self.compile_expr   (*expr.expr),
            Expr::Literal  (expr) => self.compile_literal(expr),
            Expr::Logical  (expr) => todo!(),
            Expr::Set      (expr) => todo!(),
            Expr::Super    (expr) => todo!(),
            Expr::This     (expr) => todo!(),
            Expr::Unary    (expr) => self.compile_unary  (expr),
            Expr::Variable (expr) => todo!(),
        };
    }

    fn compile_literal(&mut self, literal: Literal) {
        let value = literal.value;

        match literal.type_ {
            LiteralType::Number => {
                let value = Value::Number(to_number(&value.lexeme));
                self.compile_constant(value);
            },
            LiteralType::String => todo!(),
            LiteralType::True   => { self.write_op(OpTrue);  },
            LiteralType::False  => { self.write_op(OpFalse); },
            LiteralType::Nil    => { self.write_op(OpNil);   },
        };
    }

    fn compile_constant(&mut self, value: Value) {
        let index = self.current_chunk_mut().add_constant(value);

        self.write_op(OpConstant { index, });
    }

    fn compile_binary(&mut self, binary: BinaryOperator) {
        use BinaryOpType::*;
        let value = binary.operator.lexeme;

        self.compile_expr(*binary.left);
        self.compile_expr(*binary.right);

        let value = match binary.type_ {

            NotEqual     => self.write_ops(OpEqual,   OpNot),
            Equal        => self.write_op (OpEqual),
            Greater      => self.write_op (OpGreater),
            GreaterEqual => self.write_ops(OpLess,    OpNot),
            Less         => self.write_op (OpLess),
            LessEqual    => self.write_ops(OpGreater, OpNot),

            Add          => self.write_op (OpAdd),
            Subtract     => self.write_op (OpSubtract),
            Multiply     => self.write_op (OpMultiply),
            Divide       => self.write_op (OpDivide),
        };
    }

    fn compile_unary(&mut self, unary: UnaryOperator) {
        use UnaryOpType::*;

        self.compile_expr(*unary.right);

        match unary.type_ {
            Negate     => self.write_op(OpNegate),
            LogicalNot => self.write_op(OpNot),
        };
    }

    // utils

    fn current_chunk(&self) -> &Chunk {
        self.chunks.last().expect("Chunk list cannot be empty")
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.chunks.last_mut().expect("Chunk list cannot be empty")
    }

    fn write_op(&mut self, op: OpCode) -> usize {
        let line = self.line;
        self.current_chunk_mut().write_op(op, line)
    }
    fn write_ops(&mut self, op1: OpCode, op2: OpCode) -> usize {
        let line = self.line;
        self.current_chunk_mut().write_op(op1, line);
        self.current_chunk_mut().write_op(op2, line)
    }

}


fn to_number(lexeme: &str) -> f64 {
    lexeme.parse().expect("Unable lexeme to convert to f64")
}
