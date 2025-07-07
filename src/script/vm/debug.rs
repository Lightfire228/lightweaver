use crate::script::vm::{gc::Context, value::Value};

use super::chunk::{Chunk, OpCode};


impl Chunk {
    pub fn disassemble(&self, stack: &[Value], ctx: &Context) {
        println!("== {} ==", &self.name);

        println!("addr line instruction");

        for i in 0..self.code.len() {
            self.code[i].disassemble(self, i, ctx);
            print_stack(stack, ctx);
            println!("");
        }
    }
}


impl OpCode {
    pub fn disassemble(&self, chunk: &Chunk, ip: usize, ctx: &Context) {
        print!("{:04} ", ip);

        print_line_info(chunk, ip);

        type O = OpCode;
        match &self {
            O::Constant  { index }       => constant_instruction("OP_CONSTANT",      chunk, index.0, ctx),

            O::DefGlobal { name }        => constant_instruction("OP_DEF_GLOBAL",    chunk, name.0, ctx),
            O::GetGlobal { name }        => constant_instruction("OP_GET_GLOBAL",    chunk, name.0, ctx),
            O::SetGlobal { name }        => constant_instruction("OP_SET_GLOBAL",    chunk, name.0, ctx),

            O::GetLocal  { index }       => byte_instruction    ("OP_GET_LOCAL",     index.0),
            O::SetLocal  { index }       => byte_instruction    ("OP_SET_LOCAL",     index.0),

            O::JumpIfFalse { offset }    => jump_instruction    ("OP_JUMP_IF_FALSE", ip, offset.0,  1),
            O::JumpIfTrue  { offset }    => jump_instruction    ("OP_JUMP_IF_TRUE",  ip, offset.0,  1),
            O::Jump        { offset }    => jump_instruction    ("OP_JUMP",          ip, offset.0,  1),
            O::Loop        { offset }    => jump_instruction    ("OP_LOOP",          ip, offset.0, -1),

            O::Call        { arg_count } => byte_instruction    ("OP_CALL",          *arg_count),

            O::Nil                       => simple_instruction  ("OP_NIL"),
            O::True                      => simple_instruction  ("OP_TRUE"),
            O::False                     => simple_instruction  ("OP_FALSE"),
            O::Pop                       => simple_instruction  ("OP_POP"),

            O::Equal                     => simple_instruction  ("OP_EQUAL"),
            O::Greater                   => simple_instruction  ("OP_GREATER"),
            O::Less                      => simple_instruction  ("OP_LESS"),

            O::Add                       => simple_instruction  ("OP_ADD"),
            O::Subtract                  => simple_instruction  ("OP_SUBTRACT"),
            O::Multiply                  => simple_instruction  ("OP_MULTIPLY"),
            O::Divide                    => simple_instruction  ("OP_DIVIDE"),
            O::Not                       => simple_instruction  ("OP_NOT"),

            O::Print                     => simple_instruction  ("OP_PRINT"),
            O::Negate                    => simple_instruction  ("OP_NEGATE"),
            O::Return                    => simple_instruction  ("OP_RETURN"),
        }
    }
}

fn print_line_info(chunk: &Chunk, offset: usize) {

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset -1] {
        print!("   | ");
    }
    else {
        print!("{:4} ", chunk.lines[offset]);
    }
}

pub fn print_stack(stack: &[Value], ctx: &Context) {
    for x in stack {
        print!("[ {} ]", x.display(ctx))
    }
}


fn simple_instruction(name: &str) {
    let msg = format!("{:16} {:3}_ _", name, "");
    let msg = right_adjust(&msg);
    print!("{msg}")
}

fn constant_instruction(name: &str, chunk: &Chunk, index: usize, ctx: &Context) {
    let msg = format!("{:16} {:4} {:30} ", name, index, &chunk.constants[index].display(ctx));
    // let msg = format!("{:16} {:4} {:30} ", name, index, "");
    let msg = right_adjust(&msg);
    print!("{msg}");
}

fn byte_instruction(name: &str, index: usize) {
    let msg = format!("{:16} {:4}", name, index);
    let msg = right_adjust(&msg);
    print!("{msg}");
}

fn jump_instruction(name: &str, ip: usize, offset: usize, sign: isize) {
    let delta = offset as isize * sign;
    let dest  = (ip as isize + delta) as usize;

    let msg = format!("{:16} {:4} -> {}", name, offset, dest);
    let msg = right_adjust(&msg);
    print!("{msg}")
}

fn right_adjust(msg: &str) -> String {
    let col = 60;
    assert!(msg.len() < col);

    let spaces = col - msg.len();
    format!("{} {}| ", msg, " ".repeat(spaces))
}
