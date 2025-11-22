use crate::script::vm::{chunk::{ConstIndex, Offset}, gc::Context, value::Value};

use super::chunk::{Chunk, OpCode};

pub struct DisassembleData<'a> {
    pub ctx:       &'a Context,
    pub lines:     &'a [usize],
    pub stack:     &'a [Value],
    pub constants: &'a [Value],
}

impl Chunk {
    pub fn disassemble(&self, data: &DisassembleData) {
        println!("== {} ==", &self.name);

        println!("addr line instruction");

        for (i, op) in self.code.iter().enumerate() {
            op.disassemble(data, i);
            print_stack(data);

            println!("");
        }
    }
}


impl OpCode {
    pub fn disassemble(&self, data: &DisassembleData, ip: usize) {
        print!("{:04} ", ip);

        print_line_info(data, ip);

        type O = OpCode;
        match &self {
            O::GetConstant  { index }     => constant_instruction("OP_CONSTANT",      data, index),

            O::DefGlobal    { name_idx }  => constant_instruction("OP_DEF_GLOBAL",    data, name_idx),
            O::GetGlobal    { name_idx }  => constant_instruction("OP_GET_GLOBAL",    data, name_idx),
            O::SetGlobal    { name_idx }  => constant_instruction("OP_SET_GLOBAL",    data, name_idx),

            O::GetProperty  { name_idx }  => constant_instruction("OP_GET_PROPERTY",  data, name_idx),
            O::SetProperty  { name_idx }  => constant_instruction("OP_SET_PROPERTY",  data, name_idx),

            O::GetLocal     { index }     => byte_instruction    ("OP_GET_LOCAL",     **index),
            O::SetLocal     { index }     => byte_instruction    ("OP_SET_LOCAL",     **index),

            O::JumpIfFalse  { offset }    => jump_instruction    ("OP_JUMP_IF_FALSE", ip, offset,  1),
            O::JumpIfTrue   { offset }    => jump_instruction    ("OP_JUMP_IF_TRUE",  ip, offset,  1),
            O::Jump         { offset }    => jump_instruction    ("OP_JUMP",          ip, offset,  1),
            O::Loop         { offset }    => jump_instruction    ("OP_LOOP",          ip, offset, -1),

            O::Call         { arg_count } => byte_instruction    ("OP_CALL",          *arg_count),
            O::Class        { name_idx }  => constant_instruction("OP_CLASS",         data, name_idx),
            O::Closure      { func_idx }  => closure_instruction ("OP_CLOSURE",       data, func_idx),

            O::CloseVar     { index }     => byte_instruction    ("OP_CLOSE_VAR",     **index),

            O::Nil                        => simple_instruction  ("OP_NIL"),
            O::True                       => simple_instruction  ("OP_TRUE"),
            O::False                      => simple_instruction  ("OP_FALSE"),
            O::Pop                        => simple_instruction  ("OP_POP"),

            O::Equal                      => simple_instruction  ("OP_EQUAL"),
            O::Greater                    => simple_instruction  ("OP_GREATER"),
            O::Less                       => simple_instruction  ("OP_LESS"),

            O::Add                        => simple_instruction  ("OP_ADD"),
            O::Subtract                   => simple_instruction  ("OP_SUBTRACT"),
            O::Multiply                   => simple_instruction  ("OP_MULTIPLY"),
            O::Divide                     => simple_instruction  ("OP_DIVIDE"),
            O::Not                        => simple_instruction  ("OP_NOT"),

            O::Print                      => simple_instruction  ("OP_PRINT"),
            O::Negate                     => simple_instruction  ("OP_NEGATE"),
            O::Return                     => simple_instruction  ("OP_RETURN"),
        }
    }
}

fn print_line_info(data: &DisassembleData, offset: usize) {
    if offset > 0 && data.lines[offset] == data.lines[offset -1] {
        print!("   | ");
    }
    else {
        print!("{:4} ", data.lines[offset]);
    }
}

pub fn print_stack(data: &DisassembleData) {
    for x in data.stack {
        print!("[ {} ]", x.display(data.ctx))
    }
}


fn simple_instruction(name: &str) {
    let msg = format!("{:16}    _ _", name);
    let msg = right_adjust(&msg);
    print!("{msg}")
}

fn constant_instruction(name: &str, data: &DisassembleData, index: &ConstIndex) {
    let msg = format!("{:16} {:4} {:30} ", name, **index, &data.constants[**index].display(data.ctx));
    let msg = right_adjust(&msg);
    print!("{msg}");
}

fn byte_instruction(name: &str, index: usize) {
    let msg = format!("{:16} {:4}", name, index);
    let msg = right_adjust(&msg);
    print!("{msg}");
}

fn jump_instruction(name: &str, ip: usize, offset: &Offset, sign: isize) {
    let delta = **offset as isize * sign;
    let dest  = (ip as isize + delta) as usize;

    let msg = format!("{:16} {:4} -> {}", name, **offset, dest);
    let msg = right_adjust(&msg);
    print!("{msg}");
}

fn closure_instruction(name: &str, data: &DisassembleData, index: &ConstIndex) {
    let msg = format!("{:16} {:4} {:30}", name, **index, &data.constants[**index].display(data.ctx));
    let msg = right_adjust(&msg);
    print!("{msg}");
}

fn right_adjust(msg: &str) -> String {
    let col = 60;
    assert!(msg.len() < col);

    let spaces = col - msg.len();
    format!("{} {}| ", msg, " ".repeat(spaces))
}
