use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl Value {
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(x) => *x,
            _                => panic!("Not a number type"),
        }
    }
}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_str(self))
    }
}

fn to_str(value: &Value) -> String {
    match value {
        Value::Number(x) => x.to_string()
    }
}
