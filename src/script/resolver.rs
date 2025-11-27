
use std::usize;

use crate::script::{ast::*, vm::{chunk::{OpCode, StackIndex, UpvalueIndex}, gc::Context}};


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
    scopes:      Vec<Scope<'a>>,
}

#[derive(Debug)]
struct Local<'a> {
    pub scope_depth: usize,
    pub name:        String,
    pub type_:       LocalType<'a>,
}

#[derive(Debug)]
enum LocalType<'a> {
    Local(&'a mut VarType),
    Call (VarType)
}

#[derive(Debug)]
struct Upvalue {
    index: UpvalueIndex,
}

struct Scope<'a> {
    pub depth:     usize,
    pub locals:    Vec<Local<'a>>,
}


impl<'a> Resolver<'a> {

    fn new(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            scopes:     vec![],
            upvalues:   vec![vec![]],
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

        block.locals = self.end_scope();
    }

    fn resolve_class_decl(&mut self, class: &'a mut Class) {
        self.begin_scope();

        // TODO:

        self.end_scope();

        if !self.is_global_scope() {
            class.var_type = VarType  ::Local(self.stack_index());
            let type_      = LocalType::Local(&mut class.var_type);

            self.push_local(class.name.lexeme.to_owned(), type_);
        }

    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &'a mut ExpressionStmt) {
        self.resolve_expr(&mut expr_stmt.expr);
    }

    fn resolve_func_decl(&mut self, func: &'a mut FunctionStmt) {

        let arity = func.params.len();

        if !self.is_global_scope() {
            func.var_type = VarType  ::Local(self.stack_index());
            let type_     = LocalType::Local(&mut func.var_type);

            self.push_local(func.name.lexeme.to_string(), type_);
        }

        self.upvalues.push(vec![]);
        self.begin_scope();

        let type_ = LocalType::Call(VarType::Local(self.stack_index()));
        self.push_local(func.name.lexeme.clone(), type_);

        for arg in func.params.iter_mut() {
            let type_ = LocalType::Local(&mut arg.var_type);
            self.push_local(arg.name.lexeme.to_owned(), type_);
        }

        for stmt in func.body.iter_mut() {
            self.resolve_stmt(stmt);
        }

        func.locals = self.end_scope() - arity;
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
            stmt.var_type = VarType  ::Local(self.stack_index());
            let type_     = LocalType::Local(&mut stmt.var_type);

            self.push_local(stmt.name.lexeme.to_owned(), type_);
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

        let mut locals: Vec<_> = self.scopes.iter_mut()
            .flat_map(|scope| &mut scope.locals)
            .collect()
        ;

        let locals = locals.iter_mut().enumerate().rev();

        for (i, local) in locals {

            // +1 for the top script stack object
            let i = i +1;

            if local.name != var.name.lexeme {
                continue;
            }

            var.var_type = if local.scope_depth == current_scope_depth {
                VarType::Local(StackIndex(i))
            }
            else {
                let func  = self.upvalues.last_mut().unwrap();
                let index = UpvalueIndex(func.len());
                func.push(Upvalue { index });


                let type_ = VarType::Upvalue(index);

                let LocalType::Local(decl_type) = &mut local.type_ else {
                    panic!("The resolve stack got done borked");
                };

                **decl_type = type_;

                type_
            };

            return;
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

    fn push_local(&mut self, name: String, var_type: LocalType<'a>) {

        let last        = self.scopes.last_mut().expect("cannot push local in global scope");
        let scope_depth = last.depth;

        last.locals.push(Local {
            scope_depth,
            name,
            type_: var_type,
        });
    }

    fn begin_scope(&mut self) {
        let depth = self.scopes.len();
        self.scopes.push(Scope {
            depth,
            locals: vec![],
        });

    }

    fn end_scope(&mut self) -> usize {
        let scope = self.scopes.pop().expect("Cannot pop global scope");

        scope.locals.len()
    }

    fn is_global_scope(&self) -> bool {
        self.scopes.is_empty()
    }

    fn local_count(&self) -> usize {
        // +1 to account for global script stack slot
        self.scopes.iter().flat_map(|s| &s.locals).count() +1
    }

    fn stack_index(&self) -> StackIndex {
        StackIndex(self.local_count())
    }

}

#[cfg(test)]
mod tests {
    use crate::script::{parser, scanner};
    use std::fs;

    use super::*;

    macro_rules! get {
        ($arg: expr) => {
            $arg.try_into().unwrap()
        };
    }

    fn get_ast(file: &str) -> Ast {

        let path = format!("./test_scripts/unit_tests/resolver/{file}");
        let text = fs::read_to_string(&path).unwrap();

        let tokens = scanner::scan_tokens(&text) .unwrap();
        let ast    = parser ::parse_ast  (tokens).unwrap();

        ast

    }

    #[test]
    fn test_variable_resolution_1() {
        let mut ast = get_ast("test_variable_resolution_1.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let var_decl_a:        &VarStmt      = get!(&ast       .stmts[0]);
        let var_decl_b:        &VarStmt      = get!(&ast       .stmts[1]);
        let fn_decl_1:         &FunctionStmt = get!(&ast       .stmts[2]);
        let print_fn_1:        &PrintStmt    = get!(&ast       .stmts[3]);

        let var_decl_c:        &VarStmt      = get!(&fn_decl_1 .body [0]);
        let var_decl_d:        &VarStmt      = get!(&fn_decl_1 .body [1]);
        let fn_decl_2:         &FunctionStmt = get!(&fn_decl_1 .body [2]);

        let var_decl_e:        &VarStmt      = get!(&fn_decl_2 .body [0]);
        let var_decl_f:        &VarStmt      = get!(&fn_decl_2 .body [1]);
        let print_d:           &PrintStmt    = get!(&fn_decl_2 .body [2]);
        let print_e:           &PrintStmt    = get!(&fn_decl_2 .body [3]);
        let print_f:           &PrintStmt    = get!(&fn_decl_2 .body [4]);

        let print_fn_1_target: &Variable     = get!(&print_fn_1.expr);
        let print_d_target:    &Variable     = get!(&print_d   .expr);
        let print_e_target:    &Variable     = get!(&print_e   .expr);
        let print_f_target:    &Variable     = get!(&print_f   .expr);


        assert_eq!(var_decl_a       .var_type, VarType::Global);
        assert_eq!(var_decl_b       .var_type, VarType::Global);
        assert_eq!(fn_decl_1        .var_type, VarType::Global);

        // The first slot in a function call is reserved
        assert_eq!(var_decl_c       .var_type, VarType::Local  (StackIndex  (2)));
        assert_eq!(var_decl_d       .var_type, VarType::Upvalue(UpvalueIndex(0)));

        assert_eq!(fn_decl_2        .var_type, VarType::Local  (StackIndex(4)));
        assert_eq!(var_decl_e       .var_type, VarType::Local  (StackIndex(6)));
        assert_eq!(var_decl_f       .var_type, VarType::Local  (StackIndex(7)));
        assert_eq!(fn_decl_1        .locals,   4);

        assert_eq!(print_fn_1_target.var_type, VarType::Global);
        assert_eq!(print_d_target   .var_type, VarType::Upvalue(UpvalueIndex(0)));
        assert_eq!(print_e_target   .var_type, VarType::Local  (StackIndex  (6)));
        assert_eq!(print_f_target   .var_type, VarType::Local  (StackIndex  (7)));
        assert_eq!(fn_decl_2        .locals,   3);

    }

    #[test]
    fn test_variable_resolution_2() {

        let mut ast = get_ast("test_variable_resolution_2.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let fn_decl_outer:      &FunctionStmt = get!(&ast           .stmts[0]);
        let var_decl_mid:       &VarStmt      = get!(&ast           .stmts[1]);
        let var_decl_in:        &VarStmt      = get!(&ast           .stmts[2]);

        let var_decl_x:         &VarStmt      = get!(&fn_decl_outer .body [0]);
        let fn_decl_middle:     &FunctionStmt = get!(&fn_decl_outer .body [1]);
        let return_outer:       &ReturnStmt   = get!(&fn_decl_outer .body [3]);

        let fn_decl_inner:      &FunctionStmt = get!(&fn_decl_middle.body [0]);
        let return_middle:      &ReturnStmt   = get!(&fn_decl_middle.body [2]);
        let print_inner:        &PrintStmt    = get!(&fn_decl_inner .body [0]);
        let print_inner_target: &Variable     = get!(&print_inner   .expr);

        let ret_outer_target:   &Variable     = get!(return_outer .value.as_ref().unwrap());
        let ret_middle_target:  &Variable     = get!(return_middle.value.as_ref().unwrap());


        // Global
        assert_eq!(fn_decl_outer     .var_type, VarType::Global);
        assert_eq!(var_decl_mid      .var_type, VarType::Global);
        assert_eq!(var_decl_in       .var_type, VarType::Global);

        // Outer
        assert_eq!(var_decl_x        .var_type, VarType::Upvalue(UpvalueIndex(0)));
        assert_eq!(fn_decl_middle    .var_type, VarType::Local  (StackIndex  (3)));
        assert_eq!(fn_decl_outer     .locals,   3);

        // Middle
        assert_eq!(fn_decl_inner     .var_type, VarType::Local  (StackIndex(5)));
        assert_eq!(fn_decl_middle    .locals,   2);

        // Inner
        assert_eq!(fn_decl_inner     .locals,   1);

        // Return target expr
        assert_eq!(ret_outer_target  .var_type, VarType::Local  (StackIndex  (3)));
        assert_eq!(ret_middle_target .var_type, VarType::Local  (StackIndex  (5)));

        // Print target expr
        assert_eq!(print_inner_target.var_type, VarType::Upvalue(UpvalueIndex(0)));
    }


}
