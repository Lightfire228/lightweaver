use std::{fmt::Display, marker::PhantomData, ops::{Deref, DerefMut}};

use ast_macro::derive_all;
use gc_arena::Collect;

#[derive(Debug, Clone, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub enum OpCode {
    GetConstant { index:     ConstIndex },

    DefGlobal   { name_idx:  ConstIndex },
    GetGlobal   { name_idx:  ConstIndex },
    SetGlobal   { name_idx:  ConstIndex },

    GetProperty { name_idx:  ConstIndex },
    SetProperty { name_idx:  ConstIndex },

    GetLocal    { offset:    StackOffset },
    SetLocal    { offset:    StackOffset },

    GetUpvalue  { index:     UpvalueIndex },
    SetUpvalue  { index:     UpvalueIndex },
    PushUpvalue { index:     StackOffset },

    JumpIfFalse { offset:    Offset },
    JumpIfTrue  { offset:    Offset },
    Jump        { offset:    Offset },

    Loop        { offset:    Offset },

    Call        { arg_count: usize },
    Class       { name_idx:  ConstIndex },
    Closure     { func_idx:  ConstIndex },


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

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct Chunk<'gc> {
    pub name:      String,
    pub code:      Vec<OpCode>,
    pub lines:     Vec<usize>,
    p: PhantomData<&'gc ()>
}

impl<'gc> Chunk<'gc> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            code:  vec![],
            lines: vec![],
            p: PhantomData,
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
            OpCode::GetConstant { index }                      => format!("Constant {}",      **index   ),
            OpCode::DefGlobal   { name_idx }                   => format!("DefGlobal {}",     **name_idx),
            OpCode::GetGlobal   { name_idx }                   => format!("GetGlobal {}",     **name_idx),
            OpCode::SetGlobal   { name_idx }                   => format!("SetGlobal {}",     **name_idx),
            OpCode::GetProperty { name_idx }                   => format!("GetProperty {}",   **name_idx),
            OpCode::SetProperty { name_idx }                   => format!("SetProperty {}",   **name_idx),
            OpCode::GetLocal    { offset }                     => format!("GetLocal {}",      **offset),
            OpCode::SetLocal    { offset }                     => format!("SetLocal {}",      **offset),
            OpCode::GetUpvalue  { index }                      => format!("GetUpvalue {}",    **index   ),
            OpCode::SetUpvalue  { index }                      => format!("SetUpvalue {}",    **index   ),
            OpCode::PushUpvalue { index }                      => format!("PushUpvalue {}",   **index),
            OpCode::JumpIfFalse { offset }                     => format!("JumpIfFalse {}",   **offset  ),
            OpCode::JumpIfTrue  { offset }                     => format!("JumpIfTrue {}",    **offset  ),
            OpCode::Jump        { offset }                     => format!("Jump {}",          **offset  ),
            OpCode::Loop        { offset }                     => format!("Loop {}",          **offset  ),
            OpCode::Call        { arg_count }                  => format!("Call (args: {})",  arg_count ),
            OpCode::Class       { name_idx }                   => format!("Class {}",         **name_idx),
            OpCode::Closure     { func_idx }                   => format!("Closure {}",       **func_idx),
            OpCode::Nil                                        => format!("Nil"),
            OpCode::True                                       => format!("True"),
            OpCode::False                                      => format!("False"),
            OpCode::Pop                                        => format!("Pop"),
            OpCode::Equal                                      => format!("Equal"),
            OpCode::Greater                                    => format!("Greater"),
            OpCode::Less                                       => format!("Less"),
            OpCode::Add                                        => format!("Add"),
            OpCode::Subtract                                   => format!("Subtract"),
            OpCode::Multiply                                   => format!("Multiply"),
            OpCode::Divide                                     => format!("Divide"),
            OpCode::Not                                        => format!("Not"),
            OpCode::Negate                                     => format!("Negate"),
            OpCode::Print                                      => format!("Print"),
            OpCode::Return                                     => format!("Return"),
        })
    }
}

#[derive_all]
pub struct GlobalIndex  (pub usize);

#[derive_all]
pub struct ConstIndex   (pub usize);

#[derive_all]
pub struct StackIndex   (pub usize);

#[derive_all]
pub struct StackOffset  (pub usize);

#[derive_all]
pub struct UpvalueIndex (pub usize);

#[derive_all]
pub struct Offset       (pub usize);

#[derive_all]
pub struct BytecodeIndex(pub usize);


// TODO: macro this
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

impl Deref for StackOffset {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StackOffset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for UpvalueIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UpvalueIndex {
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
