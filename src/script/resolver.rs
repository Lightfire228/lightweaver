
use std::usize;

use crate::script::{ast::*, vm::{chunk::{OpCode, StackIndex, StackOffset, UpvalueIndex}, gc::Context}};


type Op = OpCode;

// type Func = (FuncType, ObjectId);

pub fn resolve(ast: &mut Ast, ctx: &mut Context) {
    let mut resolver = Resolver::new(ctx);

    for stmt in ast.stmts.iter_mut() {
        resolver.resolve_stmt(stmt);
    }
}



struct Resolver<'a> {
    ctx:            &'a mut Context,
    scopes:         Vec<Scope<'a>>,
    funcs:          Vec<Func>,
    temporaries:    usize,
}

#[derive(Debug)]
struct Local<'a> {
    pub scope_depth:    usize,
    pub function_depth: Option<usize>,
    pub name:           String,
    pub type_:          LocalType<'a>,
}

#[derive(Debug)]
enum LocalType<'a> {
    Local(&'a mut VarDeclType),
    Call (VarDeclType)
}

#[derive(Debug)]
struct Upvalue {
    index: UpvalueIndex,
}

struct Scope<'a> {
    pub depth:     usize,
    pub locals:    Vec<Local<'a>>,
}

struct Func {
    pub depth:    usize,
    pub upvalues: Vec<Upvalue>,
}


impl<'a> Resolver<'a> {

    fn new(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            scopes:      vec![],
            funcs:       vec![],
            temporaries: 0,
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

        self.temporaries = 0;
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
            class.var_type = VarDeclType::Local;
            let type_      = LocalType  ::Local(&mut class.var_type);

            self.push_local(class.name.lexeme.to_owned(), type_);
        }

    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &'a mut ExpressionStmt) {
        self.resolve_expr(&mut expr_stmt.expr);
    }

    fn resolve_func_decl(&mut self, func: &'a mut FunctionStmt) {

        let arity = func.params.len();

        if !self.is_global_scope() {
            func.var_type = VarDeclType::Local;
            let type_     = LocalType::Local(&mut func.var_type);

            self.push_local(func.name.lexeme.to_string(), type_);
        }

        self.begin_func();
        self.begin_scope();

        let type_ = LocalType::Call(VarDeclType::Local);
        self.push_local(func.name.lexeme.clone(), type_);

        for arg in func.params.iter_mut() {
            let type_ = LocalType::Local(&mut arg.var_type);
            self.push_local(arg.name.lexeme.to_owned(), type_);
        }

        for stmt in func.body.iter_mut() {
            self.resolve_stmt(stmt);
        }

        func.locals = self.end_scope() - arity;
        self.funcs.pop();

    }

    fn resolve_if_stmt(&mut self, if_stmt: &'a mut IfStmt) {

        let temps = self.temporaries;
        self.resolve_expr(&mut if_stmt.condition);
        self.temporaries = temps;

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
            stmt.var_type = VarDeclType::Local;
            let type_     = LocalType  ::Local(&mut stmt.var_type);

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
        let temps = self.temporaries;

        self.resolve_expr(&mut logical.left);
        self.temporaries = temps +1;

        self.resolve_expr(&mut logical.right);
        self.temporaries = temps +2;
    }


    fn resolve_assign_expr(&mut self, assign: &'a mut Assign) {
        let temps = self.temporaries;

        self.resolve_expr(&mut assign.value);
        self.temporaries = temps +1;
    }

    fn resolve_binary_expr(&mut self, binary: &'a mut BinaryOperator) {
        let temps = self.temporaries;

        self.resolve_expr(&mut binary.left);
        self.temporaries = temps +1;

        self.resolve_expr(&mut binary.right);
        self.temporaries = temps +2;
    }

    fn resolve_call_expr(&mut self, call: &'a mut Call) {
        let temps = self.temporaries;

        self.resolve_expr(&mut call.callee);
        self.temporaries = temps +1;

        for (i, arg) in call.args.iter_mut().enumerate() {
            self.resolve_expr(arg);
            self.temporaries = temps + i;
        }
    }

    fn resolve_unary_expr(&mut self, unary: &'a mut UnaryOperator) {
        let temps = self.temporaries;

        self.resolve_expr(&mut unary.right);
        self.temporaries = temps +1;
    }

    fn resolve_var_expr(&mut self, var: &'a mut Variable) {
        self.temporaries += 1;

        if self.is_global_scope() {
            return;
        }

        let locals = self.scopes.iter_mut()
            .flat_map(|scope|
                &mut scope.locals
            )
            .rev()
            .enumerate()
        ;

        let mut found = None;
        for l in locals {
            if l.1.name == var.name.lexeme {
                found = Some(l);
                break;
            }
        }

        let Some((i, local)) = found else {
            return;
        };


        let func = self.funcs.last();

        if func.is_none_or(|func| local.function_depth.unwrap() == func.depth) {
            var.var_type = VarType::Local(StackOffset(i + self.temporaries -1));
            return;
        }


        let func  = self.funcs.last_mut().unwrap();
        let index = UpvalueIndex(func.upvalues.len());
        func.upvalues.push(Upvalue { index });

        let LocalType::Local(decl_type) = &mut local.type_ else {
            panic!("The resolve stack got done borked");
        };

        **decl_type  = VarDeclType::Upvalue;
        var.var_type = VarType    ::Upvalue(index);

    }

    fn resolve_set_expr(&mut self, set: &'a mut Set) {
        let temps = self.temporaries;

        self.resolve_expr(&mut set.value);
        self.temporaries = temps +1;
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
            function_depth: self.funcs.last().map(|f| f.depth),
            type_:          var_type,

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

    fn begin_func(&mut self) {
        let depth = self.funcs.len();

        self.funcs.push(Func {
            depth,
            upvalues: vec![],
        });

    }

    fn end_func(&mut self) {
        self.funcs.pop().expect("Cannot pop global scope");
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
    use crate::script::{parser, scanner, vm::chunk::StackOffset};
    use std::fs;

    use super::*;

    macro_rules! get {
        ($arg: expr) => {
            ($arg).try_into().unwrap()
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
    fn test_1() {
        let mut ast = get_ast("test_1.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let var_decl_a:        &VarStmt      = get!(&ast       .stmts[0]);
        let var_decl_b:        &VarStmt      = get!(&ast       .stmts[1]);
        let fn_decl_1:         &FunctionStmt = get!(&ast       .stmts[2]);
        let print_fn_1:        &PrintStmt    = get!(&ast       .stmts[3]);

        let var_decl_c:        &VarStmt      = get!(&fn_decl_1 .body [0]);
        let var_decl_d:        &VarStmt      = get!(&fn_decl_1 .body [1]);
        let fn_decl_2:         &FunctionStmt = get!(&fn_decl_1 .body [2]);
        let print_c:           &PrintStmt    = get!(&fn_decl_1 .body [3]);

        let var_decl_e:        &VarStmt      = get!(&fn_decl_2 .body [0]);
        let var_decl_f:        &VarStmt      = get!(&fn_decl_2 .body [1]);
        let print_d:           &PrintStmt    = get!(&fn_decl_2 .body [2]);
        let print_e:           &PrintStmt    = get!(&fn_decl_2 .body [3]);
        let print_f:           &PrintStmt    = get!(&fn_decl_2 .body [4]);

        let print_fn_1_target: &Variable     = get!(&print_fn_1.expr);
        let print_d_target:    &Variable     = get!(&print_d   .expr);
        let print_e_target:    &Variable     = get!(&print_e   .expr);
        let print_f_target:    &Variable     = get!(&print_f   .expr);
        let print_c_target:    &Variable     = get!(&print_c   .expr);


        assert_eq!(var_decl_a       .var_type, VarDeclType::Global);
        assert_eq!(var_decl_b       .var_type, VarDeclType::Global);
        assert_eq!(fn_decl_1        .var_type, VarDeclType::Global);

        // The first slot in a function call is reserved
        assert_eq!(var_decl_c       .var_type, VarDeclType::Local);
        assert_eq!(var_decl_d       .var_type, VarDeclType::Upvalue);

        assert_eq!(fn_decl_2        .var_type, VarDeclType::Local);
        assert_eq!(var_decl_e       .var_type, VarDeclType::Local);
        assert_eq!(var_decl_f       .var_type, VarDeclType::Local);
        assert_eq!(fn_decl_1        .locals,   4);

        assert_eq!(print_fn_1_target.var_type, VarType::Global);
        assert_eq!(print_d_target   .var_type, VarType::Upvalue(UpvalueIndex(0)));
        assert_eq!(print_e_target   .var_type, VarType::Local  (StackOffset (1)));
        assert_eq!(print_f_target   .var_type, VarType::Local  (StackOffset (0)));
        assert_eq!(print_c_target   .var_type, VarType::Local  (StackOffset (2)));
        assert_eq!(fn_decl_2        .locals,   3);

    }

    #[test]
    fn test_2() {

        let mut ast = get_ast("test_2.lox");
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
        assert_eq!(fn_decl_outer     .var_type, VarDeclType::Global);
        assert_eq!(var_decl_mid      .var_type, VarDeclType::Global);
        assert_eq!(var_decl_in       .var_type, VarDeclType::Global);

        // Outer
        assert_eq!(var_decl_x        .var_type, VarDeclType::Upvalue);
        assert_eq!(fn_decl_middle    .var_type, VarDeclType::Local);
        assert_eq!(fn_decl_outer     .locals,   3);

        // Middle
        assert_eq!(fn_decl_inner     .var_type, VarDeclType::Local);
        assert_eq!(fn_decl_middle    .locals,   2);

        // Inner
        assert_eq!(fn_decl_inner     .locals,   1);

        // Return target expr
        assert_eq!(ret_outer_target  .var_type, VarType::Local  (StackOffset (0)));
        assert_eq!(ret_middle_target .var_type, VarType::Local  (StackOffset (0)));

        // Print target expr
        assert_eq!(print_inner_target.var_type, VarType::Upvalue(UpvalueIndex(0)));
    }

    #[test]
    fn test_recursion() {

        let mut ast = get_ast("recursion.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let rec_decl:    &FunctionStmt   = get!(& ast        .stmts [0]);

        let decl_n:      &FunctionParam  = get!(& rec_decl   .params[0]);
        let if_stmt:     &IfStmt         = get!(& rec_decl   .body  [0]);
        let print:       &PrintStmt      = get!(& rec_decl   .body  [2]);
        let return_stmt: &ReturnStmt     = get!(& rec_decl   .body  [3]);

        let print_n:     &Variable       = get!(& print      .expr);
        let ret_expr:    &BinaryOperator = get!(  return_stmt.value.as_ref().unwrap());
        let ret_n:       &Variable       = get!(&*ret_expr   .left);
        let ret_rec:     &Call           = get!(&*ret_expr   .right);

        let ret_rec_n:   &BinaryOperator = get!(& ret_rec    .args  [0]);
        let ret_rec_n:   &Variable       = get!(&*ret_rec_n  .left);

        let if_cond:     &BinaryOperator = get!(& if_stmt    .condition);
        let if_n:        &Variable       = get!(&*if_cond    .left);
        let if_ret:      &Block          = get!(&*if_stmt    .then_branch);
        let if_ret:      &ReturnStmt     = get!(& if_ret     .stmts [0]);
        let if_ret_n:    &Variable       = get!(  if_ret     .value.as_ref().unwrap());


        assert_eq!(decl_n   .var_type, VarDeclType::Local);


        assert_eq!(if_n     .var_type, VarType::Local(StackOffset(0)));
        assert_eq!(if_ret_n .var_type, VarType::Local(StackOffset(0)));

        // stack +1

        assert_eq!(print_n  .var_type, VarType::Local(StackOffset(1)));
        assert_eq!(ret_n    .var_type, VarType::Local(StackOffset(1)));
        assert_eq!(ret_rec_n.var_type, VarType::Local(StackOffset(3)));
    }

    #[test]
    fn test_recursive_fib() {

        let mut ast = get_ast("recursive_fib.lox");
        let mut ctx = Context::new();

        resolve(&mut ast, &mut ctx);

        let fib_decl:    &FunctionStmt   = get!(& ast        .stmts [0]);

        let decl_n:      &FunctionParam  = get!(& fib_decl   .params[0]);
        let return_stmt: &ReturnStmt     = get!(& fib_decl   .body  [1]);

        let ret_expr:    &BinaryOperator = get!(  return_stmt.value.as_ref().unwrap());
        let ret_fib_1:   &Call           = get!(&*ret_expr   .left);
        let ret_fib_2:   &Call           = get!(&*ret_expr   .right);

        let ret_fib_1_n: &BinaryOperator = get!(& ret_fib_1  .args  [0]);
        let ret_fib_2_n: &BinaryOperator = get!(& ret_fib_2  .args  [0]);
        let ret_fib_1_n: &Variable       = get!(&*ret_fib_1_n.left);
        let ret_fib_2_n: &Variable       = get!(&*ret_fib_2_n.left);

        assert_eq!(decl_n   .var_type, VarDeclType::Local);


        assert_eq!(ret_fib_1_n.var_type, VarType::Local(StackOffset(1)));
        assert_eq!(ret_fib_2_n.var_type, VarType::Local(StackOffset(2)));


    }


}
