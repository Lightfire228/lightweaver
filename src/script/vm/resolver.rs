
use std::usize;

use crate::script::{ast::*, vm::{chunk::StackIndex, gc::Context}};

use super::{chunk::{OpCode}};

type Op = OpCode;

// type Func = (FuncType, ObjectId);

pub fn resolve(ast: &mut Ast, ctx: &mut Context) {
    let mut resolver = Resolver::new(ctx);

    for stmt in ast.stmts.iter_mut() {
        resolver.resolve_stmt(stmt);
    }
}



struct Resolver<'a> {
    scope_depth: usize,
    ctx:         &'a mut Context,
    locals:      Vec<Local>,

    bools:       Vec<&'a mut bool>
}

struct Local {
    pub scope_depth: usize,
    pub name:        String,
}

impl<'a> Resolver<'a> {

    fn new(ctx: &'a mut Context) -> Self {
        Self {
            scope_depth: 0,
            ctx,
            locals: Vec::new(),
            bools:  Vec::new(),
        }
    }

    // Statements

    fn resolve_stmt(&mut self, stmt: &'a mut Stmt) {
        match stmt {
            Stmt::Block      (stmt) => self.resolve_block_stmt (stmt),
            Stmt::Class      (stmt) => self.resolve_class_decl (stmt),
            Stmt::Expression (stmt) => self.resolve_expr_stmt  (stmt),
            Stmt::Function   (stmt) => self.resolve_func_decl  (stmt),
            Stmt::If         (stmt) => self.resolve_if_stmt    (stmt),
            Stmt::Print      (stmt) => self.resolve_print_stmt (&mut stmt.expr),
            Stmt::Return     (stmt) => self.resolve_return_stmt(stmt),
            Stmt::Var        (stmt) => self.resolve_var_decl   (stmt),
            Stmt::While      (stmt) => self.resolve_while_stmt (stmt),
        };
    }

    fn resolve_block_stmt(&mut self, block: &'a mut Block) {
        self.begin_scope();

        for stmt in block.stmts.iter_mut() {
            self.resolve_stmt(stmt);
        }

        self.end_scope();
    }

    fn resolve_class_decl(&mut self, _class: &mut Class) {
        self.begin_scope();

        // TODO:

        self.end_scope();
    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &mut ExpressionStmt) {
        self.resolve_expr(&mut expr_stmt.expr);
    }

    fn resolve_func_decl(&mut self, func: &'a mut FunctionStmt) {
        self.begin_scope();

        for arg in func.params.iter_mut() {
            self.push_local(arg.name.lexeme.to_owned());
            self.bools.push(&mut arg.closed);
        }

        for stmt in func.body.iter_mut() {
            self.resolve_stmt(stmt);
        }

        self.end_scope();

    }

    fn resolve_if_stmt(&mut self, if_stmt: &'a mut IfStmt) {

        self.resolve_expr(&mut if_stmt.condition);

        self.resolve_stmt(&mut if_stmt.then_branch);

        if let Some(stmt) = &mut if_stmt.else_branch {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_print_stmt(&mut self, expr: &mut Expr) {
        self.resolve_expr(expr);
    }

    fn resolve_return_stmt(&mut self, return_: &mut ReturnStmt) {
        if let Some(val) = &mut return_.value {
            self.resolve_expr(val);
        }
    }

    fn resolve_var_decl(&mut self, stmt: &'a mut VarStmt) {
        self.push_local(stmt.name.lexeme.to_owned());
        self.bools.push(&mut stmt.is_closed);

        if let Some(val) = &mut stmt.initializer {
            self.resolve_expr(val);
        }
    }

    fn resolve_while_stmt(&mut self, while_: &'a mut WhileStmt) {
        self.resolve_expr(&mut while_.condition);

        self.resolve_stmt(&mut while_.body);
    }


    // Expressions

    fn resolve_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Assign   (expr) => self.resolve_assign_expr (expr),
            Expr::Binary   (expr) => self.resolve_binary_expr (expr),
            Expr::Call     (expr) => self.resolve_call_expr   (expr),
            Expr::Get      (_)    => {},
            Expr::Grouping (expr) => self.resolve_expr        (&mut expr.expr),
            Expr::Literal  (_)    => {},
            Expr::Logical  (expr) => self.resolve_logical_expr(expr),
            Expr::Set      (expr) => self.resolve_set_expr    (expr),
            Expr::Super    (expr) => self.resolve_super_expr  (expr),
            Expr::This     (expr) => self.resolve_this_expr   (expr),
            Expr::Unary    (expr) => self.resolve_unary_expr  (expr),
            Expr::Variable (expr) => self.resolve_var_expr    (expr),
        };
    }

    fn resolve_logical_expr(&mut self, logical: &mut Logical) {
        self.resolve_expr(&mut logical.left);
    }

    fn resolve_logical_jump(&mut self, right: &mut Expr) {
        self.resolve_expr(right);
    }

    fn resolve_assign_expr(&mut self, assign: &mut Assign) {
        self.resolve_expr(&mut assign.value);
    }


    fn resolve_binary_expr(&mut self, binary: &mut BinaryOperator) {
        self.resolve_expr(&mut binary.left);
        self.resolve_expr(&mut binary.right);
    }

    fn resolve_call_expr(&mut self, call: &mut Call) {
        self.resolve_expr(&mut call.callee);

        for arg in call.args.iter_mut() {
            self.resolve_expr(arg);
        }
    }

    fn resolve_unary_expr(&mut self, unary: &mut UnaryOperator) {
        self.resolve_expr(&mut unary.right);
    }

    fn resolve_var_expr(&mut self, var: &mut Variable) {

        let stack_len = self.locals.len();

        for (i, local) in self.locals.iter().enumerate().rev() {

            if local.name == var.name.lexeme {
                *self.bools[i] = local.scope_depth != self.scope_depth;
                var.decl       = StackIndex(stack_len - i);
                break;
            }
        };

    }

    fn resolve_set_expr(&mut self, set: &mut Set) {
        self.resolve_expr(&mut set.value);
    }

    fn resolve_super_expr(&mut self, _super: &mut Super) {
        todo!()
    }

    fn resolve_this_expr(&mut self, _this: &mut This) {
        todo!()
    }

    fn push_local(&mut self, name: String) {
        self.locals.push(Local {
            scope_depth: self.scope_depth,
            name,
        });
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        loop {
            let Some(last) = self.locals.last() else {
                break;
            };

            if last.scope_depth <= self.scope_depth {
                break;
            }

            self.locals.pop();
            self.bools .pop();
        }
    }

}
