use crate::script::{ast::{Ast, BinaryOpType, BinaryOperator, Expr, ExpressionStmt, Get, Literal, LiteralType, Set, Stmt, UnaryOpType, UnaryOperator, VarStmt, Variable}, tokens::Token};

use super::{chunk::{Chunk, OpCode}, object::ObjString, value::Value};

type Op = OpCode;


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

        compiler.write_op(Op::Return);

        compiler.chunks

    }

    // Statements

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Block      (_stmt) => todo!(),
            Stmt::Class      (_stmt) => todo!(),
            Stmt::Expression ( stmt) => self.compile_expr_stmt (stmt),
            Stmt::Function   (_stmt) => todo!(),
            Stmt::If         (_stmt) => todo!(),
            Stmt::Print      ( stmt) => self.compile_print_stmt(stmt.expr),
            Stmt::Return     (_stmt) => todo!(),
            Stmt::Var        ( stmt) => self.compile_var_stmt  (stmt),
            Stmt::While      (_stmt) => todo!(),
        }
    }

    fn compile_expr_stmt(&mut self, expr_stmt: ExpressionStmt) {
        self.compile_expr(expr_stmt.expr);
        self.write_op(Op::Pop);
    }


    fn compile_print_stmt(&mut self, expr: Expr) {
        self.compile_expr(expr);
        self.write_op(Op::Print);
        // Print pops
    }

    fn compile_var_stmt(&mut self, stmt: VarStmt) {

        self.line = stmt.name.line;
        let name_index = self.make_identifier_constant(stmt.name);

        match stmt.initializer {
            Some(expr) =>   self.compile_expr(expr),
            None       => { self.write_op(Op::Nil); }
        }

        self.define_variable(name_index);

    }

    // Expressions

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Assign   (_expr) => todo!(),
            Expr::Binary   ( expr) => self.compile_binary_expr (expr),
            Expr::Call     (_expr) => todo!(),
            Expr::Get      ( expr) => self.compile_get_expr    (expr),
            Expr::Grouping ( expr) => self.compile_expr        (*expr.expr),
            Expr::Literal  ( expr) => self.compile_literal_expr(expr),
            Expr::Logical  (_expr) => todo!(),
            Expr::Set      ( expr) => self.compile_set_expr    (expr),
            Expr::Super    (_expr) => todo!(),
            Expr::This     (_expr) => todo!(),
            Expr::Unary    ( expr) => self.compile_unary_expr  (expr),
            Expr::Variable ( expr) => self.compile_var_expr    (expr),
        };
    }

    fn compile_literal_expr(&mut self, literal: Literal) {
        let value = literal.value;
        self.line = value.line;

        match literal.type_ {
            LiteralType::Number => {
                let value = Value::Number(to_number(&value.lexeme));
                self.make_constant(value);
            },
            LiteralType::String => {
                let value = Value::new_string(value.lexeme);
                self.make_constant(value);
            },
            LiteralType::True   => { self.write_op(Op::True);  },
            LiteralType::False  => { self.write_op(Op::False); },
            LiteralType::Nil    => { self.write_op(Op::Nil);   },
        };
    }

    fn compile_set_expr(&mut self, set: Set) {
        self.line = set.name.line;

        let index = self.make_identifier_constant(set.name);
        self.compile_expr(*set.value);

        self.write_op(Op::SetGlobal { index, });
    }

    fn make_constant(&mut self, value: Value) -> usize {

        let chunk = self.current_chunk_mut();

        let mut index = None;

        // TODO: this is dumb, should be using string interning instead
        for (i, constant) in chunk.constants.iter().enumerate() {
            if value.is_string() && *constant == value {
                index = Some(i)
            }
        }

        let index = match index {
            Some(index) => index,
            None        => {
                let index = self.current_chunk_mut().add_constant(value);
                self.write_op(Op::Constant { index, });
                index
            },
        };


        index
    }


    fn make_identifier_constant(&mut self, name: Token) -> usize {
        self.make_constant(str_to_val(name.lexeme))
    }

    fn define_variable(&mut self, index: usize ) {
        self.write_op(Op::DefGlobal { index, });
    }

    // fn make_global(&mut self, name: Token) {
    //     let index = self.make_constant(str_to_val(name.lexeme));

    //     self.write_op(OpCode::DefGlobal { index, });
    // }



    fn compile_binary_expr(&mut self, binary: BinaryOperator) {
        self.line = binary.operator.line;

        self.compile_expr(*binary.left);
        self.compile_expr(*binary.right);

        type B = BinaryOpType;
        match binary.type_ {
            B::NotEqual     => self.write_ops(Op::Equal,   Op::Not),
            B::Equal        => self.write_op (Op::Equal),
            B::Greater      => self.write_op (Op::Greater),
            B::GreaterEqual => self.write_ops(Op::Less,    Op::Not),
            B::Less         => self.write_op (Op::Less),
            B::LessEqual    => self.write_ops(Op::Greater, Op::Not),

            B::Add          => self.write_op (Op::Add),
            B::Subtract     => self.write_op (Op::Subtract),
            B::Multiply     => self.write_op (Op::Multiply),
            B::Divide       => self.write_op (Op::Divide),
        };
    }

    fn compile_unary_expr(&mut self, unary: UnaryOperator) {
        self.line = unary.operator.line;

        self.compile_expr(*unary.right);

        type U = UnaryOpType;
        match unary.type_ {
            U::Negate     => self.write_op(Op::Negate),
            U::LogicalNot => self.write_op(Op::Not),
        };
    }

    fn compile_var_expr(&mut self, var: Variable) {
        self.line = var.name.line;

        let index = self.make_identifier_constant(var.name);
        self.write_op(Op::GetGlobal { index, });
    }

    fn compile_get_expr(&mut self, get: Get) {
        self.line = get.name.line;

        let index = self.make_identifier_constant(get.name);
        self.write_op(Op::GetGlobal { index, });
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

fn str_to_val(string: String) -> Value {
    Value::Obj(Box::new(ObjString::new(string)))
}
