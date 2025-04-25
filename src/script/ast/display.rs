
use super::expr::*;
use super::stmt::*;
use super::Ast;


struct AstDisplay {
    indent: usize,
    string: String,
}

type DisplayList = Vec<AstDisplay>;

impl AstDisplay {
    pub fn new(indent: usize, string: String) -> Self {
        Self {
            indent,
            string,
        }
    }

    pub fn from(indent: usize, string: &str) -> Self {
        Self::new(indent, string.to_owned())
    }
}

impl Ast {
    pub fn to_display_str(&self) -> String {

        let ind = 0;

        let prefix = AstDisplay::from(ind, "ast (");
        let suffix = AstDisplay::from(ind, ") ast");

        let segments = wrap(prefix, suffix, || {
            self.stmts.iter().flat_map(|stmt| {
                stmt.to_display_str(ind +1)
            })
            .collect()
        });
        
        let mut result = String::new();
        for d in segments {
            result.push_str(&" ".repeat(d.indent * 4));
            result.push_str(&d.string);
            result.push('\n');
        }

        result
    }
}

impl Stmt {
    fn to_display_str(&self, ind: usize) -> DisplayList {

        let prefix = AstDisplay::from(ind, "stmt (");
        let suffix = AstDisplay::from(ind, ") stmt");

        wrap(prefix, suffix, || {
            match self {
                Stmt::Block     (block)      => block     .to_display_str(ind +1),
                Stmt::Expression(expression) => expression.to_display_str(ind +1),
                Stmt::VarDecl   (var_decl)   => var_decl  .to_display_str(ind +1),
            }
        })
    }
}

impl Block {
    fn to_display_str(&self, ind: usize) -> DisplayList {

        let prefix = AstDisplay::from(ind, "{");
        let suffix = AstDisplay::from(ind, "}");

        wrap(prefix, suffix, || {
            self.statements.iter().flat_map(|stmt| {
                stmt.to_display_str(ind +1)
            })
            .collect()
        })
    }
}

impl ExpressionStmt {
    fn to_display_str(&self, ind: usize) -> DisplayList {

        let prefix = AstDisplay::from(ind, "exprStmt (");
        let suffix = AstDisplay::from(ind, ") exprStmt");

        wrap(prefix, suffix, || {
            self.expression.to_display_str(ind +1)
        })
    }
}

impl VarDecl {
    fn to_display_str(&self, ind: usize) -> DisplayList {

        match &self.initializer {
            None              => vec![AstDisplay::new(ind, format!("let {}", self.name.lexeme))],
            Some(initializer) => {
                let prefix = AstDisplay::new(ind, format!("let {} = (", self.name.lexeme));
                let suffix = AstDisplay::from(ind, ") let");
    
                wrap(prefix, suffix, || {
                    initializer.to_display_str(ind +1)
                })
            }
        }

    }
}


impl Expr {
    fn to_display_str(&self, ind: usize) -> DisplayList {
        match self {
            Expr::Assign       (assign)        => assign       .to_display_str(ind),
            Expr::Instantiation(instantiation) => instantiation.to_display_str(ind),
            Expr::Connection   (connection)    => connection   .to_display_str(ind),
            Expr::Variable     (variable)      => variable     .to_display_str(ind),
        }
    }
}

impl Assign {
    fn to_display_str(&self, ind: usize) -> DisplayList {
        let prefix = AstDisplay::new(ind, format!("{} = (", self.name));
        let suffix = AstDisplay::new(ind, format!(") = {}", self.name));

        wrap(prefix, suffix, || {
            self.value.to_display_str(ind +1)
        })
    }
}

impl Instantiation {
    fn to_display_str(&self, ind: usize) -> DisplayList {
        vec![AstDisplay::new(ind, format!("instantiation ({})", self.type_.lexeme))]
    }
}

impl Connection {
    fn to_display_str(&self, ind: usize) -> DisplayList {
        let left  = self.left .to_display_str(ind);
        let right = self.right.to_display_str(ind);
        let op    = AstDisplay::from(ind, &self.operator.lexeme);

        operator(left, op, right)
    }
}

impl Variable {
    fn to_display_str(&self, ind: usize) -> DisplayList {
        vec![AstDisplay::from(ind, &self.name.lexeme)]
    }
}



fn wrap<T>(prefix: AstDisplay, suffix: AstDisplay, func: T) -> DisplayList 
    where T: Fn() -> DisplayList
{
    let mut results = Vec::new();

    results.push  (prefix);
    results.extend(func());
    results.push  (suffix);

    results
}

fn operator(left: DisplayList, op: AstDisplay, right: DisplayList) -> DisplayList {
    let mut results = Vec::new();

    results.extend(left);
    results.push  (op);
    results.extend(right);

    results
}