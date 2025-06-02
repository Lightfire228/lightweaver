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
        print!("{:04} ", offset);

        print_line(chunk, offset);

        type O = OpCode;
        match &self {
            O::OpConstant { index } => constant_instruction("OP_CONSTANT", chunk, *index),

            O::OpNil                => simple_instruction  ("OP_NIL"),
            O::OpTrue               => simple_instruction  ("OP_TRUE"),
            O::OpFalse              => simple_instruction  ("OP_FALSE"),

            O::OpEqual              => simple_instruction  ("OP_EQUAL"),
            O::OpGreater            => simple_instruction  ("OP_GREATER"),
            O::OpLess               => simple_instruction  ("OP_LESS"),

            O::OpAdd                => simple_instruction  ("OP_ADD"),
            O::OpSubtract           => simple_instruction  ("OP_SUBTRACT"),
            O::OpMultiply           => simple_instruction  ("OP_MULTIPLY"),
            O::OpDivide             => simple_instruction  ("OP_DIVIDE"),
            O::OpNot                => simple_instruction  ("OP_NOT"),

            O::OpNegate             => simple_instruction  ("OP_NEGATE"),
            O::OpReturn             => simple_instruction  ("OP_RETURN"),

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
