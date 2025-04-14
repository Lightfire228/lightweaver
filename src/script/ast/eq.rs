
use super::expr::*;
use super::stmt::*;
use super::Ast;



impl PartialEq for Ast {
    fn eq(&self, other: &Self) -> bool {

        if self.stmts.len() != other.stmts.len() {
            return false;
        }

        let me    = self .stmts.iter();
        let other = other.stmts.iter();
        
        me.zip(other).map(|x| x.0 == x.1).all(|x| x)
    }
}

impl Eq        for Stmt {}

impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::_Block    (l0), Self::_Block    (r0)) => l0 == r0,
            (Self::Expression(l0), Self::Expression(r0)) => l0 == r0,
            (Self::Let       (l0), Self::Let       (r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Assign       (l0), Self::Assign       (r0)) => l0 == r0,
            (Self::Instantiation(l0), Self::Instantiation(r0)) => l0 == r0,
            (Self::Connection   (l0), Self::Connection   (r0)) => l0 == r0,
            (Self::Variable     (l0), Self::Variable     (r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {

        if self.statements.len() != other.statements.len() {
            return false;
        }

        let me    = self .statements.iter();
        let other = other.statements.iter();
        
        me.zip(other).map(|x| x.0 == x.1).all(|x| x)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}

impl PartialEq for Let {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.initializer == other.initializer
    }
}


impl PartialEq for Assign {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl PartialEq for Instantiation {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ // && self.body == other.body
    }
}

// impl PartialEq for Body {
//     fn eq(&self, other: &Self) -> bool {
//         self.properties == other.properties
//     }
// }

// impl PartialEq for Property {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.initializer == other.initializer
//     }
// }

impl PartialEq for Literal {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.operator == other.operator && self.right == other.right
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
