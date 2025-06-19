use std::fmt::Display;

use super::value::Value;


pub enum OpCode {
    Constant    { index: usize },

    DefGlobal   { index: usize },
    GetGlobal   { index: usize },
    SetGlobal   { index: usize },

    GetLocal    { index: usize },
    SetLocal    { index: usize },

    JumpIfFalse { offset: usize },
    JumpIfTrue  { offset: usize },
    Jump        { offset: usize },

    Loop        { offset: usize },

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

impl Display for OpCode {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OpCode::Constant    { index }  => format!("Constant {}",    index),
            OpCode::DefGlobal   { index }  => format!("DefGlobal {}",   index),
            OpCode::GetGlobal   { index }  => format!("GetGlobal {}",   index),
            OpCode::SetGlobal   { index }  => format!("SetGlobal {}",   index),
            OpCode::GetLocal    { index }  => format!("GetLocal {}",    index),
            OpCode::SetLocal    { index }  => format!("SetLocal {}",    index),
            OpCode::JumpIfFalse { offset } => format!("JumpIfFalse {}", offset),
            OpCode::JumpIfTrue  { offset } => format!("JumpIfTrue {}",  offset),
            OpCode::Jump        { offset } => format!("Jump {}",        offset),
            OpCode::Loop        { offset } => format!("Loop {}",        offset),
            OpCode::Nil                    => format!("Nil"),
            OpCode::True                   => format!("True"),
            OpCode::False                  => format!("False"),
            OpCode::Pop                    => format!("Pop"),
            OpCode::Equal                  => format!("Equal"),
            OpCode::Greater                => format!("Greater"),
            OpCode::Less                   => format!("Less"),
            OpCode::Add                    => format!("Add"),
            OpCode::Subtract               => format!("Subtract"),
            OpCode::Multiply               => format!("Multiply"),
            OpCode::Divide                 => format!("Divide"),
            OpCode::Not                    => format!("Not"),
            OpCode::Negate                 => format!("Negate"),
            OpCode::Print                  => format!("Print"),
            OpCode::Return                 => format!("Return"),
        })
    }
}
