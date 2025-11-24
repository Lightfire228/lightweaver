
use std::usize;

use crate::script::{ast::*, vm::{chunk::{StackIndex, UpvalueIndex}, gc::Context}};

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
    ctx:         &'a mut Context,
    upvalues:    Vec<Vec<Upvalue>>,
    var_types:   Vec<&'a mut VarType>,
    scopes:      Vec<Scope>,
}

#[derive(Debug)]
struct Local {
    pub scope_depth: usize,
    pub name:        String,
}


#[derive(Debug)]
struct Upvalue {
    index: UpvalueIndex,
}

struct Scope {
    pub depth:     usize,
    pub locals:    Vec<Local>,
}


impl<'a> Resolver<'a> {

    fn new(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            scopes:    vec![],
            upvalues:  vec![vec![]],
            var_types: vec![],
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

    fn resolve_class_decl(&mut self, class: &'a mut Class) {
        self.begin_scope();

        // TODO:

        self.end_scope();

        if !self.is_global_scope() {
            class.var_type = VarType::Local(StackIndex(0));
            self.push_local(class.name.lexeme.to_owned(), &mut class.var_type);
        }

    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &'a mut ExpressionStmt) {
        self.resolve_expr(&mut expr_stmt.expr);
    }

    fn resolve_func_decl(&mut self, func: &'a mut FunctionStmt) {

        if !self.is_global_scope() {
            func.var_type = VarType::Local(StackIndex(0));
            self.push_local(func.name.lexeme.to_string(), &mut func.var_type);
        }

        self.upvalues.push(vec![]);
        self.begin_scope();

        for arg in func.params.iter_mut() {
            self.push_local(arg.name.lexeme.to_owned(), &mut arg.var_type);
        }

        for stmt in func.body.iter_mut() {
            self.resolve_stmt(stmt);
        }

        self.end_scope();
        self.upvalues.pop();

    }

    fn resolve_if_stmt(&mut self, if_stmt: &'a mut IfStmt) {

        self.resolve_expr(&mut if_stmt.condition);

        self.resolve_stmt(&mut if_stmt.then_branch);

        if let Some(stmt) = &mut if_stmt.else_branch {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_print_stmt(&mut self, expr: &'a mut Expr) {
        self.resolve_expr(expr);
    }

    fn resolve_return_stmt(&mut self, return_: &'a mut ReturnStmt) {
        if let Some(val) = &mut return_.value {
            self.resolve_expr(val);
        }
    }

    fn resolve_var_decl(&mut self, stmt: &'a mut VarStmt) {

        if !self.is_global_scope() {
            stmt.var_type = VarType::Local(StackIndex(0));

            self.push_local(stmt.name.lexeme.to_owned(), &mut stmt.var_type);
        }

        if let Some(val) = &mut stmt.initializer {
            self.resolve_expr(val);
        }
    }

    fn resolve_while_stmt(&mut self, while_: &'a mut WhileStmt) {
        self.resolve_expr(&mut while_.condition);

        self.resolve_stmt(&mut while_.body);
    }


    // Expressions

    fn resolve_expr(&mut self, expr: &'a mut Expr) {
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

    fn resolve_logical_expr(&mut self, logical: &'a mut Logical) {
        self.resolve_expr(&mut logical.left);
    }

    fn resolve_logical_jump(&mut self, right: &'a mut Expr) {
        self.resolve_expr(right);
    }

    fn resolve_assign_expr(&mut self, assign: &'a mut Assign) {
        self.resolve_expr(&mut assign.value);
    }


    fn resolve_binary_expr(&mut self, binary: &'a mut BinaryOperator) {
        self.resolve_expr(&mut binary.left);
        self.resolve_expr(&mut binary.right);
    }

    fn resolve_call_expr(&mut self, call: &'a mut Call) {
        self.resolve_expr(&mut call.callee);

        for arg in call.args.iter_mut() {
            self.resolve_expr(arg);
        }
    }

    fn resolve_unary_expr(&mut self, unary: &'a mut UnaryOperator) {
        self.resolve_expr(&mut unary.right);
    }

    fn resolve_var_expr(&mut self, var: &'a mut Variable) {

        if self.is_global_scope() {
            return;
        }

        let current_scope_depth = self.scopes.len() -1;
        let locals              = self.var_types.len();

        let mut count = 0;
        for scope in self.scopes.iter_mut().rev() {

            for (local_idx, local) in scope.locals.iter().rev().enumerate() {

                dbg!("{} {}", &var.name.lexeme, &local.name);
                if local.name != var.name.lexeme {
                    count += 1;
                    continue;
                }

                let type_ = if local.scope_depth == current_scope_depth {
                    println!("match local");
                    VarType::Local(StackIndex(local_idx))
                }
                else {
                    let func  = self.upvalues.last_mut().unwrap();
                    let index = UpvalueIndex(func.len());
                    func.push(Upvalue { index });
                    println!("match upvalue");

                    VarType::Upvalue(index)
                };

                let idx = locals - count -1;
                *self.var_types[idx] = type_;
                var.var_type         = type_;
                return;
            }
        }
    }

    fn resolve_set_expr(&mut self, set: &'a mut Set) {
        self.resolve_expr(&mut set.value);
    }

    fn resolve_super_expr(&mut self, _super: &mut Super) {
        todo!()
    }

    fn resolve_this_expr(&mut self, _this: &mut This) {
        todo!()
    }

    fn push_local(&mut self, name: String, var_type: &'a mut VarType) {

        let last        = self.scopes.last_mut().expect("cannot push local in global scope");
        let scope_depth = last.depth;

        last.locals.push(Local {
            scope_depth,
            name,
        });

        self.var_types.push(var_type);
    }

    fn begin_scope(&mut self) {
        let depth = self.scopes.len();
        self.scopes.push(Scope {
            depth,
            locals: vec![],

        });

    }

    fn end_scope(&mut self) {
        let scope = self.scopes.pop().expect("Cannot pop global scope");

        for _ in scope.locals {
            self.var_types.pop();
        }
    }

    fn is_global_scope(&self) -> bool {
        self.scopes.is_empty()
    }

}

#[cfg(test)]
mod tests {
    use crate::{script::{parser, scanner}};
    use std::fs;

    use super::*;

    fn get_ast(file: &str) -> Ast {

        let path = format!("./test_scripts/unit_tests/resolver/{file}");
        let text = fs::read_to_string(&path).unwrap();

        let tokens = scanner::scan_tokens(&text).unwrap();
        let ast    = parser::parse_ast(tokens).unwrap();

        ast

    }

    #[test]
    fn base() {

        let mut ast = get_ast("base_01.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let var_decl_a: &VarStmt      = (&ast.stmts[0]).try_into().unwrap();
        let var_decl_b: &VarStmt      = (&ast.stmts[1]).try_into().unwrap();
        let fn_decl_1:  &FunctionStmt = (&ast.stmts[2]).try_into().unwrap();

        let var_decl_c: &VarStmt      = (&fn_decl_1.body[0]).try_into().unwrap();
        let var_decl_d: &VarStmt      = (&fn_decl_1.body[1]).try_into().unwrap();

        let fn_decl_2:  &FunctionStmt = (&fn_decl_1.body[2]).try_into().unwrap();
        let var_decl_e: &VarStmt      = (&fn_decl_2.body[0]).try_into().unwrap();

        dbg!("{}", var_decl_a.var_type);
        dbg!("{}", var_decl_b.var_type);
        dbg!("{}", fn_decl_1 .var_type);
        dbg!("{}", var_decl_c.var_type);
        dbg!("{}", var_decl_d.var_type);
        dbg!("{}", fn_decl_2 .var_type);
        dbg!("{}", var_decl_e.var_type);


        assert_eq!(var_decl_a.var_type, VarType::Global);
        assert_eq!(var_decl_b.var_type, VarType::Global);
        assert_eq!(fn_decl_1 .var_type, VarType::Global);

        // Variable declarations don't have a defined index
        assert!(matches!(var_decl_c.var_type, VarType::Local  (_)));
        assert!(matches!(var_decl_d.var_type, VarType::Upvalue(_)));

        assert!(matches!(fn_decl_2 .var_type, VarType::Local(_)));
        assert!(matches!(var_decl_e.var_type, VarType::Local(_)));


        let print:        &PrintStmt = (&ast.stmts[3]).try_into().unwrap();
        let print_target: &Variable  = (&print.expr)  .try_into().unwrap();
        assert_eq!(print_target.var_type, VarType::Global);

        let print:        &PrintStmt = (&fn_decl_2.body[1]).try_into().unwrap();
        let print_target: &Variable  = (&print.expr)           .try_into().unwrap();
        assert!(matches!(print_target.var_type, VarType::Upvalue(UpvalueIndex(0))));

        let print:        &PrintStmt = (&fn_decl_2.body[2]).try_into().unwrap();
        let print_target: &Variable  = (&print.expr)           .try_into().unwrap();
        assert!(matches!(print_target.var_type, VarType::Local(StackIndex(0))));
    }

    #[test]
    fn base_01() {

        let mut ast = get_ast("base_02.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let fn_decl_outer:      &FunctionStmt = (&ast           .stmts[0]).try_into().unwrap();
        let var_decl_mid:       &VarStmt      = (&ast           .stmts[1]).try_into().unwrap();
        let var_decl_in:        &VarStmt      = (&ast           .stmts[2]).try_into().unwrap();

        let var_decl_x:         &VarStmt      = (&fn_decl_outer .body[0]) .try_into().unwrap();
        let fn_decl_middle:     &FunctionStmt = (&fn_decl_outer .body[1]) .try_into().unwrap();
        let return_outer:       &ReturnStmt   = (&fn_decl_outer .body[3]) .try_into().unwrap();

        let fn_decl_inner:      &FunctionStmt = (&fn_decl_middle.body[0]) .try_into().unwrap();
        let return_middle:      &ReturnStmt   = (&fn_decl_middle.body[2]) .try_into().unwrap();
        let print_inner:        &PrintStmt    = (&fn_decl_inner .body[0]) .try_into().unwrap();
        let print_inner_target: &Variable     = (&print_inner.expr)       .try_into().unwrap();

        let ret_outer_target:   &Variable     = return_outer .value.as_ref().unwrap().try_into().unwrap();
        let ret_middle_target:  &Variable     = return_middle.value.as_ref().unwrap().try_into().unwrap();


        // Global
        assert_eq!(fn_decl_outer.var_type, VarType::Global);
        assert_eq!(var_decl_mid .var_type, VarType::Global);
        assert_eq!(var_decl_in  .var_type, VarType::Global);

        // Outer
        dbg!("----- {}", var_decl_x);
        assert!(matches!(var_decl_x       .var_type, VarType::Upvalue(_)));
        assert!(matches!(fn_decl_middle   .var_type, VarType::Local  (_)));

        // Middle
        assert!(matches!(fn_decl_inner    .var_type, VarType::Local  (_)));

        // Return target expr
        assert_eq!(ret_outer_target .var_type,  VarType::Local  (StackIndex  (0)));
        assert_eq!(ret_middle_target.var_type,  VarType::Local  (StackIndex  (0)));

        // Print target expr
        assert_eq!(print_inner_target.var_type, VarType::Upvalue(UpvalueIndex(0)));
    }


}
