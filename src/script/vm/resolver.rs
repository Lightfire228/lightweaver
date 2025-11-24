
use std::usize;

use crate::script::{ast::*, tokens::Token, vm::{chunk::{StackIndex, UpvalueIndex}, gc::Context}};

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
    func_depth:  usize,
    ctx:         &'a mut Context,
    upvalues:    Vec<Vec<Upvalue>>,
    locals:      Vec<Local>,

    var_types:   Vec<&'a mut VarType>,
}

struct Local {
    pub func_depth: usize,
    pub name:       String,
}


#[derive(Debug)]
struct Upvalue {
    index: UpvalueIndex,
}


impl<'a> Resolver<'a> {

    fn new(ctx: &'a mut Context) -> Self {
        Self {
            func_depth: 0,
            ctx,
            locals:    vec![],
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

        self.push_local(class.name.lexeme.to_owned(), &mut class.var_type);
    }

    fn resolve_expr_stmt(&mut self, expr_stmt: &'a mut ExpressionStmt) {
        self.resolve_expr(&mut expr_stmt.expr);
    }

    fn resolve_func_decl(&mut self, func: &'a mut FunctionStmt) {

        if self.func_depth > 0 {
            func.var_type = VarType::Local(StackIndex(0));
        }

        self.push_local(func.name.lexeme.to_string(), &mut func.var_type);
        self.begin_scope();

        for arg in func.params.iter_mut() {
            self.push_local(arg.name.lexeme.to_owned(), &mut arg.var_type);
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

    fn resolve_print_stmt(&mut self, expr: &'a mut Expr) {
        self.resolve_expr(expr);
    }

    fn resolve_return_stmt(&mut self, return_: &'a mut ReturnStmt) {
        if let Some(val) = &mut return_.value {
            self.resolve_expr(val);
        }
    }

    fn resolve_var_decl(&mut self, stmt: &'a mut VarStmt) {

        if self.func_depth > 0 {
            stmt.var_type = VarType::Local(StackIndex(0))
        }

        self.push_local(stmt.name.lexeme.to_owned(), &mut stmt.var_type);

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
        println!("############################################\n");

        for (i, local) in self.locals.iter().rev().enumerate() {
            println!(">> {} {}", i, local.name)
        }


        for (i, local) in self.locals.iter().rev().enumerate() {

            if local.name != var.name.lexeme {
                continue;
            }

            let type_ = if local.func_depth == self.func_depth {
            // +1 to account for the bottom stack slot taken up by the script local object
                println!(">>> type local {}", local.name);
                VarType::Local(StackIndex(i +1))
            }
            else {
                let func  = self.upvalues.last_mut().unwrap();
                let index = UpvalueIndex(func.len());
                func.push(Upvalue { index });

                println!(">>> type upvalue {}", local.name);
                VarType::Upvalue(index)
            };

            let index = self.var_types.len() - i -1;
            *self.var_types[index] = type_;
            var.var_type           = type_;
        };

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
        self.locals.push(Local {
            func_depth: self.func_depth,
            name,
        });

        self.var_types.push(var_type);
    }

    fn begin_scope(&mut self) {
        self.func_depth += 1;
    }

    fn end_scope(&mut self) {
        self.func_depth -= 1;

        loop {
            let Some(last) = self.locals.last() else {
                break;
            };

            if self.func_depth > last.func_depth {
                break;
            }

            self.locals   .pop();
            self.var_types.pop();
        }
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

        let var_decl_a:    &VarStmt      = (&ast.stmts[0]).try_into().unwrap();
        let var_decl_b:    &VarStmt      = (&ast.stmts[1]).try_into().unwrap();
        let var_decl_fn_1: &FunctionStmt = (&ast.stmts[2]).try_into().unwrap();
        // let print:         &PrintStmt    = (&ast.stmts[3]).try_into().unwrap();

        let var_decl_c:    &VarStmt      = (&var_decl_fn_1.body[0]).try_into().unwrap();
        let var_decl_d:    &VarStmt      = (&var_decl_fn_1.body[1]).try_into().unwrap();


        assert_eq!(var_decl_a.var_type, VarType::Global);
        assert_eq!(var_decl_b.var_type, VarType::Global);

        assert!(matches!(var_decl_c.var_type, VarType::Local(_)));
        assert!(matches!(var_decl_d.var_type, VarType::Local(_)));



    }

}
