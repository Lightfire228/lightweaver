use super::value::Value;


pub enum OpCode {
    Constant  { index: usize },
    DefGlobal { index: usize },
    GetGlobal { index: usize },
    SetGlobal { index: usize },
    Nil,
    True,
    False,
    Pop,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,
    Return,
}

pub struct Chunk {
    pub name:      String,
    pub code:      Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines:     Vec<usize>,
}

impl Chunk {
    pub fn new(name: String) -> Self {
        Self {
            name,
            code:      vec![],
            constants: vec![],
            lines:     vec![],
        }
    }

    pub fn write_op(&mut self, op: OpCode, line: usize) -> usize {
        self.code .push(op);
        self.lines.push(line);

        self.code.len() -1
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);

        self.constants.len() -1
    }
}
