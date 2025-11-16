use std::{fmt::Display, ops::{Deref, DerefMut}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct GlobalIndex  (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct ConstIndex   (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct StackIndex   (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct Offset       (pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct BytecodeIndex(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    GetConstant { index:     ConstIndex },

    DefGlobal   { name_idx:  ConstIndex },
    GetGlobal   { name_idx:  ConstIndex },
    SetGlobal   { name_idx:  ConstIndex },

    GetLocal    { index:     StackIndex },
    SetLocal    { index:     StackIndex },

    JumpIfFalse { offset:    Offset },
    JumpIfTrue  { offset:    Offset },
    Jump        { offset:    Offset },

    Loop        { offset:    Offset },

    Call        { arg_count: usize },
    Class       { name_idx:  ConstIndex },

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
    pub lines:     Vec<usize>,
}

impl Chunk {
    pub fn new(name: String) -> Self {
        Self {
            name,
            code:  vec![],
            lines: vec![],
        }
    }

    pub fn write_op(&mut self, op: OpCode, line: usize) -> BytecodeIndex {
        let index = self.code.len();

        self.code .push(op);
        self.lines.push(line);

        BytecodeIndex(index)
    }

}

impl Display for OpCode {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OpCode::GetConstant { index }        => format!("Constant {}",      **index   ),
            OpCode::DefGlobal   { name_idx }     => format!("DefGlobal {}",     **name_idx),
            OpCode::GetGlobal   { name_idx }     => format!("GetGlobal {}",     **name_idx),
            OpCode::SetGlobal   { name_idx }     => format!("SetGlobal {}",     **name_idx),
            OpCode::GetLocal    { index }        => format!("GetLocal {}",      **index   ),
            OpCode::SetLocal    { index }        => format!("SetLocal {}",      **index   ),
            OpCode::JumpIfFalse { offset }       => format!("JumpIfFalse {}",   **offset  ),
            OpCode::JumpIfTrue  { offset }       => format!("JumpIfTrue {}",    **offset  ),
            OpCode::Jump        { offset }       => format!("Jump {}",          **offset  ),
            OpCode::Loop        { offset }       => format!("Loop {}",          **offset  ),
            OpCode::Call        { arg_count }    => format!("Call (args: {})",  arg_count ),
            OpCode::Class       { name_idx }     => format!("Class {}",         **name_idx),
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

impl Deref for BytecodeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BytecodeIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl From<usize> for BytecodeIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Deref for StackIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StackIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Offset {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ConstIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
