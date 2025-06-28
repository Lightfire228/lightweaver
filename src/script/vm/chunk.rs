use std::fmt::Display;

use super::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct GlobalIndex  (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct ConstIndex   (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct StackIndex   (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct Offset       (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct BytecodeIndex(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    Constant    { index:  ConstIndex },

    DefGlobal   { name:   ConstIndex },
    GetGlobal   { name:   ConstIndex },
    SetGlobal   { name:   ConstIndex },

    GetLocal    { index:  StackIndex },
    SetLocal    { index:  StackIndex },

    JumpIfFalse { offset: Offset },
    JumpIfTrue  { offset: Offset },
    Jump        { offset: Offset },

    Loop        { offset: Offset },

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

#[derive(Debug, Clone)]
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

    pub fn write_op(&mut self, op: OpCode, line: usize) -> BytecodeIndex {
        let index = self.code.len();

        self.code .push(op);
        self.lines.push(line);

        BytecodeIndex(index)
    }

    pub fn add_constant(&mut self, value: Value) -> ConstIndex {
        let index = self.constants.len();

        self.constants.push(value);

        ConstIndex(index)
    }
}

impl Display for OpCode {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OpCode::Constant    { index }        => format!("Constant {}",      index .0),
            OpCode::DefGlobal   { name }         => format!("DefGlobal {}",     name  .0),
            OpCode::GetGlobal   { name }         => format!("GetGlobal {}",     name  .0),
            OpCode::SetGlobal   { name }         => format!("SetGlobal {}",     name  .0),
            OpCode::GetLocal    { index }        => format!("GetLocal {}",      index .0),
            OpCode::SetLocal    { index }        => format!("SetLocal {}",      index .0),
            OpCode::JumpIfFalse { offset }       => format!("JumpIfFalse {}",   offset.0),
            OpCode::JumpIfTrue  { offset }       => format!("JumpIfTrue {}",    offset.0),
            OpCode::Jump        { offset }       => format!("Jump {}",          offset.0),
            OpCode::Loop        { offset }       => format!("Loop {}",          offset.0),
            OpCode::Nil                          => format!("Nil"),
            OpCode::True                         => format!("True"),
            OpCode::False                        => format!("False"),
            OpCode::Pop                          => format!("Pop"),
            OpCode::Equal                        => format!("Equal"),
            OpCode::Greater                      => format!("Greater"),
            OpCode::Less                         => format!("Less"),
            OpCode::Add                          => format!("Add"),
            OpCode::Subtract                     => format!("Subtract"),
            OpCode::Multiply                     => format!("Multiply"),
            OpCode::Divide                       => format!("Divide"),
            OpCode::Not                          => format!("Not"),
            OpCode::Negate                       => format!("Negate"),
            OpCode::Print                        => format!("Print"),
            OpCode::Return                       => format!("Return"),
        })
    }
}
