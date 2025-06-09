use crate::script::{ast::{Assign, Ast, BinaryOpType, BinaryOperator, Block, Expr, ExpressionStmt, Get, Literal, LiteralType, Set, Stmt, UnaryOpType, UnaryOperator, VarStmt, Variable}, tokens::Token};

use super::{chunk::{Chunk, OpCode}, object::ObjString, value::Value};

type Op = OpCode;


pub struct Compiler {
    chunk:       Chunk,
    line:        usize,
    locals:      Vec<Local>,
    scope_depth: usize,
}

struct Local {
    name:        Token,
    depth:       usize,
    initialized: bool,
}

enum VariableType {
    Global,
    Local,
}

#[derive(Debug)]
pub struct CompileError {
    pub msg: String
}

pub type CompilerResult<T> = Result<T, CompileError>;

impl Compiler {

    fn new() -> Self {
        let name = "script".to_owned();

        Self {
            chunk:       Chunk::new(name),
            line:        0,
            locals:      vec![],
            scope_depth: 0,
        }
    }

    pub fn compile(ast: Ast) -> CompilerResult<Vec<Chunk>> {

        let mut compiler = Self::new();

        for stmt in ast.stmts {
            compiler.compile_stmt(stmt)?;
        }

        compiler.write_op(Op::Return);

        Ok(vec![compiler.chunk])

    }

    // Statements

    fn compile_stmt(&mut self, stmt: Stmt) -> CompilerResult<()> {
        match stmt {
            Stmt::Block      ( stmt) => self.compile_block_stmt(stmt)?,
            Stmt::Class      (_stmt) => todo!(),
            Stmt::Expression ( stmt) => self.compile_expr_stmt (stmt),
            Stmt::Function   (_stmt) => todo!(),
            Stmt::If         (_stmt) => todo!(),
            Stmt::Print      ( stmt) => self.compile_print_stmt(stmt.expr),
            Stmt::Return     (_stmt) => todo!(),
            Stmt::Var        ( stmt) => self.compile_var_stmt  (stmt)?,
            Stmt::While      (_stmt) => todo!(),
        };

        Ok(())
    }

    fn compile_block_stmt(&mut self, block: Block) -> CompilerResult<()> {
        self.begin_scope();

        for stmt in *block.stmts {
            self.compile_stmt(stmt)?;
        }

        self.end_scope();
        Ok(())
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

    fn compile_var_stmt(&mut self, stmt: VarStmt) -> CompilerResult<()> {
        self.line = stmt.name.line;

        let     local = self.scope_depth > 0;
        let mut index = 0;

        dbg!(local);
        dbg!(&stmt);

        if local {
            self.declare_local(&stmt.name);
        }
        else {
            index = self.make_identifier_constant(stmt.name.clone());
        }

        match stmt.initializer {
            Some(expr) =>   self.compile_expr(expr),
            None       => { self.write_op(Op::Nil); }
        }

        if !local {
            self.define_global(index);
        }


        Ok(())
    }

    // Expressions

    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Assign   ( expr) => self.compile_assign_expr (expr),
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
            // Expr::Variable ( expr) => self.compile_var_expr    (expr),
            Expr::Variable ( expr) => todo!(),
        };
    }

    fn compile_literal_expr(&mut self, literal: Literal) {
        let value = literal.value;
        self.line = value.line;

        match literal.type_ {
            LiteralType::Number => {
                let value = Value::Number(to_number(&value.lexeme));
                self.emit_constant(value);
            },
            LiteralType::String => {
                let value = Value::new_string(value.lexeme);
                self.emit_constant(value);
            },
            LiteralType::True   => { self.write_op(Op::True);  },
            LiteralType::False  => { self.write_op(Op::False); },
            LiteralType::Nil    => { self.write_op(Op::Nil);   },
        };
    }

    fn compile_assign_expr(&mut self, assign: Assign) {

        self.compile_expr(*assign.value);

        let set_op = match self.resolve_local(&assign.target.name) {
            Some(_) => panic!(),
            None    => {
                let index = self.make_identifier_constant(assign.target.name);
                Op::SetGlobal { index, }
            }
        };

        self.write_op(set_op);
    }


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



    }

    fn compile_get_expr(&mut self, get: Get) {
        self.line = get.name.line;

        let index = self.make_identifier_constant(get.name);
        self.write_op(Op::GetGlobal { index, });
    }

    fn compile_set_expr(&mut self, set: Set) {
        self.line = set.name.line;

        let index = self.make_identifier_constant(set.name);
        self.compile_expr(*set.value);

        self.write_op(Op::SetGlobal { index, });
    }


    // Variables

    fn make_constant(&mut self, value: Value) -> usize {

        let chunk = self.current_chunk_mut();

        let mut index = None;

        // TODO: this is dumb, should be using string interning instead
        for (i, constant) in chunk.constants.iter().enumerate() {
            if value.is_string() && *constant == value {
                index = Some(i)
            }
        }

        index.unwrap_or(
            self.current_chunk_mut().add_constant(value)
        )
    }

    fn emit_constant(&mut self, value: Value) -> usize {
        let index = self.current_chunk_mut().add_constant(value);
        self.write_op(Op::Constant { index, })
    }


    fn make_identifier_constant(&mut self, name: Token) -> usize {
        self.make_constant(str_to_val(name.lexeme))
    }

    fn declare_local(&mut self, name: &Token) {
        // TODO: allow locals to shadow each other?
        for local in self.locals.iter().rev() {
            if local.initialized && local.depth < self.scope_depth {
                break;
            }

            if local.name.lexeme == name.lexeme {
                panic!("todo, make this a compiler error")
            }
        }

        self.add_local(name.clone());
    }

    fn define_global(&mut self, index: usize) {
        self.write_op(Op::DefGlobal { index, });
        // self.write_op(Op::Pop);
    }

    fn add_local(&mut self, name: Token) {
        self.locals.push(Local {
            name,
            depth:       self.scope_depth,
            initialized: false,
        });
    }

    fn resolve_local(&self, name: &Token) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {

            if local.name.lexeme == name.lexeme {
                return Some(i);
            }
        }

        None
    }


    // utils

    fn current_chunk(&self) -> &Chunk {
        &self.chunk
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
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

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        let pop_count = self.locals.iter().filter(|l| l.depth > self.scope_depth).count();

        for _ in 0..pop_count {
            self.write_op(Op::Pop);
        }
    }

}


fn to_number(lexeme: &str) -> f64 {
    lexeme.parse().expect("Unable lexeme to convert to f64")
}

fn str_to_val(string: String) -> Value {
    Value::Obj(Box::new(ObjString::new(string)))
}
