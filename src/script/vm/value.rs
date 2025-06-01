use std::fmt::Display;

use super::{RuntimeError, RuntimeResult};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool  (bool),
    Nil,
}

impl Value {
    pub fn expect_number<F>(&self, err: F) -> RuntimeResult<f64>
        where F: Fn() -> RuntimeError
    {
        match self {
            Value::Number(x) => Ok(*x),
            _                => Err(err()),
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil       => true,
            Value::Number(_) => false,
            Value::Bool  (x) => !(*x),
        }
    }
}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_str(self))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool  (a), Value::Bool  (b)) => a == b,
            (Value::Nil,       Value::Nil)       => true,
            _                                    => false,
        }
    }
}

fn to_str(value: &Value) -> String {
    match value {
        Value::Number(x) => x.to_string(),
        Value::Bool  (x) => x.to_string(),
        Value::Nil       => "nil".to_owned(),
    }
}
