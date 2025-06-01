use super::chunk::{Chunk, OpCode};


impl Chunk {
    pub fn disassemble(&self) {
        println!("== {} ==", &self.name);

        println!("addr line instruction");

        for i in 0..self.code.len() {
            self.code[i].disassemble(self, i);
        }
    }
}


impl OpCode {
    pub fn disassemble(&self, chunk: &Chunk, offset: usize) {
        use OpCode::*;

        print!("{:04} ", offset);

        print_line(chunk, offset);

        match &self {
            OpConstant { index } => constant_instruction("OP_CONSTANT", chunk, *index),

            OpNil                => simple_instruction  ("OP_NIL"),
            OpTrue               => simple_instruction  ("OP_TRUE"),
            OpFalse              => simple_instruction  ("OP_FALSE"),

            OpEqual              => simple_instruction  ("OP_EQUAL"),
            OpGreater            => simple_instruction  ("OP_GREATER"),
            OpLess               => simple_instruction  ("OP_LESS"),

            OpAdd                => simple_instruction  ("OP_ADD"),
            OpSubtract           => simple_instruction  ("OP_SUBTRACT"),
            OpMultiply           => simple_instruction  ("OP_MULTIPLY"),
            OpDivide             => simple_instruction  ("OP_DIVIDE"),
            OpNot                => simple_instruction  ("OP_NOT"),

            OpNegate             => simple_instruction  ("OP_NEGATE"),
            OpReturn             => simple_instruction  ("OP_RETURN"),

        }
    }
}

fn print_line(chunk: &Chunk, offset: usize) {

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset -1] {
        print!("   | ");
    }
    else {
        print!("{:4} ", chunk.lines[offset]);
    }
}

fn simple_instruction(name: &str) {
    println!("{name}")
}

fn constant_instruction(name: &str, chunk: &Chunk, index: usize) {
    println!("{:16} {:4} {}", name, index, &chunk.constants[index])
}
