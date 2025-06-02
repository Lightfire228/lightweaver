use chunk::{Chunk, OpCode};
use value::Value;

pub mod chunk;
pub mod debug;
pub mod value;
pub mod compiler;
pub mod object;

static DEBUG_TRACE_EXECUTION: bool = true;

pub struct Vm {
    chunk:   Chunk,
    ip:      usize,
    stack:   Vec<Value>,
    strings: Vec<String>,
}

pub struct RuntimeError {
    pub msg:  String,
    pub line: usize,
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

enum BinaryOp {
    Greater,
    Less,
    // Add,
    Sub,
    Mul,
    Div
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk:   Chunk::new("null chunk".to_owned()),
            ip:      0,
            stack:   vec![],
            strings: vec![],
        }
    }

    pub fn interpret(&mut self, mut chunks: Vec<Chunk>) -> RuntimeResult<()> {

        self.chunk = chunks.remove(0);

        self.run()
    }

    fn run(&mut self) -> RuntimeResult<()> {

        loop {

            if DEBUG_TRACE_EXECUTION {
                self.chunk.code[self.ip].disassemble(&self.chunk, self.ip);
            }

            type O = OpCode;
            match *self.get_instruction() {
                O::OpConstant { index } => self.op_constant(index),

                O::OpNil                => self.stack.push(Value::Nil),
                O::OpTrue               => self.stack.push(Value::Bool(true)),
                O::OpFalse              => self.stack.push(Value::Bool(false)),

                O::OpEqual              => self.op_equal(),
                O::OpGreater            => self.op_binary(BinaryOp::Greater)?,
                O::OpLess               => self.op_binary(BinaryOp::Less)?,

                O::OpAdd                => self.op_add()?,
                O::OpSubtract           => self.op_binary(BinaryOp::Sub)?,
                O::OpMultiply           => self.op_binary(BinaryOp::Mul)?,
                O::OpDivide             => self.op_binary(BinaryOp::Div)?,

                O::OpNot                => self.op_not    (),

                O::OpNegate             => self.op_negate ()?,
                O::OpReturn             => {
                    self.op_return();
                    return Ok(());
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

    fn pop_number(&mut self) -> RuntimeResult<f64> {
        Ok(self
            .pop_stack()
            .as_number()
            .ok_or_else(||
                self.runtime_error("Operand must be a number")
            )?
        )
    }

    fn peek_stack(&self, index: usize) -> &Value {
        let index = self.stack.len() - index -1;
        &self.stack[index]
    }

    // op codes

    fn op_constant(&mut self, index: usize) {
        let constant = self.get_constant(index);

        self.stack.push(constant.clone());
    }

    fn op_binary(&mut self, op: BinaryOp) -> RuntimeResult<()> {
        let b = self.pop_number()?;
        let a = self.pop_number()?;

        type B = BinaryOp;
        let val = match op {
            B::Greater => Value::Bool  (a > b),
            B::Less    => Value::Bool  (a < b),

            B::Sub     => Value::Number(a - b),
            B::Mul     => Value::Number(a * b),
            B::Div     => Value::Number(a / b),
        };

        self.stack.push(val);

        Ok(())
    }

    fn op_add(&mut self) -> RuntimeResult<()> {

        let b = self.peek_stack(0);
        let a = self.peek_stack(1);

        if a.is_string() && b.is_string() {
            self.concatenate();
        }
        else if let (Some(a), Some(b)) = (a.as_number(), b.as_number()) {
            self.stack.push(Value::Number(a + b));
        }
        else {
            return Err(self.runtime_error("Operands must be two numbers or two strings"))
        }

        Ok(())
    }

    fn op_negate(&mut self) -> RuntimeResult<()> {
        let val = self.pop_number()?;

        self.stack.push(Value::Number(-val));

        Ok(())
    }

    fn op_not(&mut self) {
        let val = self.pop_stack().is_falsey();

        self.stack.push(Value::Bool(val));
    }

    fn op_equal(&mut self) {
        let a = self.pop_stack();
        let b = self.pop_stack();

        self.stack.push(Value::Bool(a == b));
    }

    fn op_return(&mut self) {
        let constant = self.pop_stack();

        println!("{}", constant);
    }

    fn concatenate(&mut self) {
        let b = self.pop_stack().as_string().unwrap();
        let a = self.pop_stack().as_string().unwrap();

        self.stack.push(Value::new_string(format!("{}{}", a.string, b.string)));
    }

    // utils

    fn runtime_error(&self, msg: &str) -> RuntimeError {
        RuntimeError {
            msg:  msg.to_owned(),
            line: self.chunk.lines[self.ip -1],
        }
    }

}
