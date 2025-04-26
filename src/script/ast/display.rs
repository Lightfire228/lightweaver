
use super::expr::*;
use super::stmt::*;
use super::Ast;

#[derive(Clone)]
pub struct AstDisplayOpts {
    pub display_expr_nodes: bool,
    pub explicit_names:     bool,
}
type Opts = AstDisplayOpts;

struct AstDisplay {
    indent: usize,
    string: String,
}

type DisplayList = Vec<AstDisplay>;


impl AstDisplayOpts {
    pub fn new() -> Self {
        Self {
            display_expr_nodes: false,
            explicit_names:     false,
        }
    }
}

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

trait AstFormat {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList;
}

impl Ast {
    pub fn to_display_str(&self, opts: &Opts) -> String {

        let ind = 0;

        let (prefix, suffix) = format_name(ind, "ast", "(", ")");

        let segments = wrap(prefix, suffix, || {
            self.stmts.iter().flat_map(|stmt| {
                stmt.to_display_str(ind +1, &opts)
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

impl AstFormat for Stmt {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {

        let (prefix, suffix) = format_name(ind, "stmt", "(", ")");

        wrap(prefix, suffix, || {
            match self {
                Stmt::Block     (block)      => block     .to_display_str(ind +1, opts),
                Stmt::Expression(expression) => expression.to_display_str(ind +1, opts),
                Stmt::VarDecl   (var_decl)   => var_decl  .to_display_str(ind +1, opts),
            }
        })
    }
}
impl Stmt {
    fn name(_opts: Opts) -> &'static str {
        "stmt"
    }
}

impl AstFormat for Block {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {

        let prefix = AstDisplay::from(ind, "{");
        let suffix = AstDisplay::from(ind, "}");

        wrap(prefix, suffix, || {
            self.statements.iter().flat_map(|stmt| {
                stmt.to_display_str(ind +1, opts)
            })
            .collect()
        })
    }
}

impl AstFormat for ExpressionStmt {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {

        let (prefix, suffix) = format_name(ind, "exprStmt", "(", ")");

        wrap(prefix, suffix, || {
            self.expression.to_display_str(ind +1, opts)
        })
    }
}

impl AstFormat for VarDecl {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {

        match &self.initializer {
            None              => vec![AstDisplay::new(ind, format!("varDecl {}", self.name.lexeme))],
            Some(initializer) => {

                let name = format!("varDecl {}", self.name.lexeme);
                let (prefix, suffix) = format_name(ind, &name, "= (", ")");
                
                wrap(prefix, suffix, || {
                    initializer.to_display_str(ind +1, opts)
                })
            }
        }

    }
}


impl AstFormat for Expr {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {
        match self {
            Expr::Assign       (assign)        => assign       .to_display_str(ind, opts),
            Expr::Instantiation(instantiation) => instantiation.to_display_str(ind, opts),
            Expr::Connection   (connection)    => connection   .to_display_str(ind, opts),
            Expr::Variable     (variable)      => variable     .to_display_str(ind, opts),
        }
    }
}

impl AstFormat for Assign {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {

        let (prefix, suffix) = format_name(ind, &self.name.lexeme, "= (", ") =");

        wrap(prefix, suffix, || {
            self.value.to_display_str(ind +1, &opts)
        })
    }
}

impl AstFormat for Instantiation {
    fn to_display_str(&self, ind: usize, _opts: &Opts) -> DisplayList {
        vec![AstDisplay::new(ind, format!("instantiation ({})", self.type_.lexeme))]
    }
}

impl AstFormat for Connection {
    fn to_display_str(&self, ind: usize, opts: &Opts) -> DisplayList {
        let left  = self.left .to_display_str(ind, opts);
        let right = self.right.to_display_str(ind, opts);
        let op    = AstDisplay::from(ind, &self.operator.lexeme);

        operator(left, op, right)
    }
}

impl AstFormat for Variable {
    fn to_display_str(&self, ind: usize, _opts: &Opts) -> DisplayList {
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

fn format_name(ind: usize, name: &str, left_bracket: &str, right_bracket: &str) -> (AstDisplay, AstDisplay) {
    (
        AstDisplay::new(ind, format!("{} {}", name,          left_bracket)),
        AstDisplay::new(ind, format!("{} {}", right_bracket, name)),
    )
}