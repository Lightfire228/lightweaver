use super::value::Value;


pub enum OpCode {
    OpConstant { index: usize },
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
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
