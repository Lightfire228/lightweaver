use std::{collections::HashMap};

use chunk::{Chunk, OpCode};
use value::Value;
use gc::Context;

use crate::script::vm::{chunk::{ConstIndex, Offset, StackIndex}, debug::print_stack, object::ObjString};

pub mod chunk;
pub mod debug;
pub mod value;
pub mod compiler;
pub mod object;
pub mod gc;


static DEBUG_TRACE_EXECUTION: bool = true;

// TODO: String interning
pub struct Vm {
    chunk:   Chunk,
    ip:      usize,
    stack:   Vec<Value>,
    globals: HashMap<String, Value>,
    ctx:     Context,
}

pub struct RuntimeError {
    pub msg:  String,
    pub line: usize,
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

enum BinaryOp {
    Greater,
    Less,
    Sub,
    Mul,
    Div
}

enum JumpType {
    IfFalsey,
    IfTruthy,
}

impl Vm {
    pub fn new(ctx: Context) -> Self {
        Self {
            chunk:   Chunk::new("null chunk".to_owned()),
            ip:      0,
            stack:   vec![],
            globals: HashMap::new(),
            ctx,
        }
    }

    pub fn interpret(&mut self, mut chunks: Vec<Chunk>) -> RuntimeResult<()> {


        let str = self.ctx.add_string("<script>");
        let val = Value::Obj(str);

        self.push_stack(val);
        self.chunk = chunks.remove(0);

        self.chunk.disassemble(&[], &self.ctx);
        println!();

        self.run()
    }

    fn run(&mut self) -> RuntimeResult<()> {

        println!(">>>>>>>>>>>>>>>>>>>>> ");

        loop {

            if DEBUG_TRACE_EXECUTION {
                self.chunk.code[self.ip].disassemble(&self.chunk, self.ip, &self.ctx);
                print_stack(&self.stack, &self.ctx);
                println!();
            }

            type O = OpCode;
            match *self.get_instruction() {
                O::Constant    { index }  => self.op_constant  (index),

                O::DefGlobal   { name }   => self.op_def_global(name),
                O::GetGlobal   { name }   => self.op_get_global(name)?,
                O::SetGlobal   { name }   => self.op_set_global(name)?,

                O::GetLocal    { index }  => self.op_get_local (index),
                O::SetLocal    { index }  => self.op_set_local (index),

                O::JumpIfFalse { offset } => self.op_jump_if(JumpType::IfFalsey, offset),
                O::JumpIfTrue  { offset } => self.op_jump_if(JumpType::IfTruthy, offset),
                O::Jump        { offset } => self.op_jump   (offset),

                O::Loop        { offset } => self.op_loop   (offset),

                O::Nil                    => self.push_stack(Value::Nil),
                O::True                   => self.push_stack(Value::Bool(true)),
                O::False                  => self.push_stack(Value::Bool(false)),

                O::Pop                    => self.op_pop(),

                O::Equal                  => self.op_equal(),
                O::Greater                => self.op_binary(BinaryOp::Greater)?,
                O::Less                   => self.op_binary(BinaryOp::Less)?,

                O::Add                    => self.op_add()?,
                O::Subtract               => self.op_binary(BinaryOp::Sub)?,
                O::Multiply               => self.op_binary(BinaryOp::Mul)?,
                O::Divide                 => self.op_binary(BinaryOp::Div)?,

                O::Not                    => self.op_not    (),

                O::Print                  => self.op_print(),
                O::Negate                 => self.op_negate ()?,
                O::Return                 => {
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

    fn get_constant(&self, index: ConstIndex) -> &Value {
        &self.chunk.constants[index.0]
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().expect("Stack cannot be empty")
    }

    fn push_stack(&mut self, val: Value) {
        self.stack.push(val);
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

    fn op_constant(&mut self, index: ConstIndex) {
        let constant = self.get_constant(index);

        self.push_stack(constant.clone());
    }

    fn op_def_global(&mut self, name: ConstIndex) {
        let name = self.get_constant_as_str(name, &self.ctx);
        let val  = self.pop_stack();

        self.globals.insert(name, val);
    }

    fn op_get_global(&mut self, index: ConstIndex) -> RuntimeResult<()> {
        let name  = self.get_constant_as_str(index, &self.ctx);

        let value = match self.globals.get(&name) {
            Some(value) => value,
            None        => {
                let msg = format!("Undefined variable '{name}'");
                Err(self.runtime_error(&msg))?
            },
        };

        self.push_stack(value.clone());

        Ok(())
    }

    fn op_set_global(&mut self, index: ConstIndex) -> RuntimeResult<()> {
        let name = self.get_constant_as_str(index, &self.ctx);

        if self.globals.get(&name).is_none() {
            let msg = format!("Undefined variable '{name}'");
            Err(self.runtime_error(&msg))?
        }

        *self.globals.get_mut(&name).unwrap() = self.peek_stack(0).clone();

        Ok(())
    }

    fn op_get_local(&mut self, index: StackIndex) {
        let index = index.0;

        self.push_stack(self.stack[index +1].clone());
    }

    fn op_set_local(&mut self, index: StackIndex) {
        let index = index.0;

        let value = self.peek_stack(0).clone();
        self.stack[index +1] = value;
    }

    fn op_jump_if(&mut self, jump_type: JumpType, offset: Offset) {
        let offset = offset.0;

        let is_falsey = self.peek_stack(0).is_falsey();

        let jump_on_false = match jump_type {
            JumpType::IfFalsey => true,
            JumpType::IfTruthy => false,
        };

        if is_falsey == jump_on_false {
            self.ip += offset;
        }
    }

    fn op_jump(&mut self, offset: Offset) {
        let offset = offset.0;

        self.ip += offset;
    }

    fn op_loop(&mut self, offset: Offset) {
        let offset = offset.0;

        self.ip -= offset;
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

        self.push_stack(val);

        Ok(())
    }

    fn op_add(&mut self) -> RuntimeResult<()> {

        let b = self.pop_stack();
        let a = self.pop_stack();

        if a.is_lw_string(&self.ctx) && b.is_lw_string(&self.ctx) {
            let result = {
                let a  = a.to_str(&self.ctx).unwrap();
                let b  = b.to_str(&self.ctx).unwrap();
                format!("{a}{b}")
            };

            let obj = self.ctx.add_string(&result);
            let val = Value::new_obj(obj);
            self.push_stack(val);
        }
        else if let (Some(a), Some(b)) = (a.as_number(), b.as_number()) {
            self.push_stack(Value::Number(a + b));
        }
        else {
            return Err(self.runtime_error("Operands must be two numbers or two strings"))
        }

        Ok(())
    }

    fn op_print(&mut self) {
        let val = self.pop_stack();

        println!("{}", val.display(&self.ctx))
    }

    fn op_negate(&mut self) -> RuntimeResult<()> {
        let val = self.pop_number()?;

        self.push_stack(Value::Number(-val));

        Ok(())
    }

    fn op_not(&mut self) {
        let val = self.pop_stack().is_falsey();

        self.push_stack(Value::Bool(val));
    }

    fn op_pop(&mut self) {
        self.pop_stack();
    }

    fn op_equal(&mut self) {
        let a = self.pop_stack();
        let b = self.pop_stack();

        self.push_stack(Value::Bool(a == b));
    }

    fn op_return(&mut self) {
        let constant = self.pop_stack();

        println!("{}", constant.display(&self.ctx));
    }

    fn concatenate(&mut self, val1: &str, val2: &str) {
        let val = concatenate(val1, val2, &mut self.ctx);
        self.push_stack(val);
    }

    // utils

    fn runtime_error(&self, msg: &str) -> RuntimeError {
        RuntimeError {
            msg:  msg.to_owned(),
            line: self.chunk.lines[self.ip -1],
        }
    }

    fn get_constant_as_str(&self, index: ConstIndex, ctx: &Context) -> String {
        self.chunk.constants[index.0]
            .to_str(ctx)
            .expect("Expect constant value to be of type ObjString")
            .to_owned()
    }

}

fn concatenate(val1: &str, val2: &str, ctx: &mut Context) -> Value {

    let str = format!("{val1}{val2}");

    let id  = ctx.add(ObjString::new(str).into());

    Value::new_obj(id)
}
