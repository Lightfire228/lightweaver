use chunk::{Chunk, OpCode};
use value::Value;

pub mod chunk;
pub mod debug;
pub mod value;

static DEBUG_TRACE_EXECUTION: bool = true;

pub struct Vm {
    chunk: Chunk,
    stack: Vec<Value>,
    ip:    usize,
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new("null chunk".to_owned()),
            stack: vec![],
            ip:    0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult<()> {

        self.chunk = chunk;

        self.run()
    }

    fn run(&mut self) -> InterpretResult<()> {
        use OpCode::*;

        loop {

            if DEBUG_TRACE_EXECUTION {
                self.chunk.code[self.ip].disassemble(&self.chunk, self.ip);
            }

            match *self.get_instruction() {
                OpConstant { index } => self.op_constant(index),
                OpAdd                => self.op_binary(BinaryOp::Add),
                OpSubtract           => self.op_binary(BinaryOp::Sub),
                OpMultiply           => self.op_binary(BinaryOp::Mul),
                OpDivide             => self.op_binary(BinaryOp::Div),

                OpNegate             => self.op_negate  (),
                OpReturn             => {
                    self.op_return();
                    return Ok(())
                }
            }
        }
    }

    fn get_instruction(&mut self) -> &OpCode {
        self.ip += 1;

        &self.chunk.code[self.ip -1]
    }

    fn get_constant(&self, index: usize) -> &Value {
        &self.chunk.constants[index]
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().expect("Stack cannot be empty")
    }


    fn op_constant(&mut self, index: usize) {
        let constant = self.get_constant(index);

        self.stack.push(constant.clone());
    }

    fn op_binary(&mut self, op: BinaryOp) {
        use BinaryOp::*;

        let b = self.pop_stack().as_number();
        let a = self.pop_stack().as_number();


        let val = match op {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
        };

        self.stack.push(Value::Number(val));
    }

    fn op_negate(&mut self) {
        let val = self.pop_stack().as_number();

        self.stack.push(Value::Number(-val));
    }

    fn op_return(&mut self) {
        let constant = self.pop_stack();

        println!("{}", constant);
    }


}

pub type InterpretResult<T> = Result<T, InterpretErrorType>;

pub enum InterpretErrorType {
    RuntimeError,
}
