use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap};
use std::fmt::Write;

use chunk::{Chunk, OpCode};
use value::Value;
use gc::Context;

use crate::script::vm::debug::DisassembleData;
use crate::script::vm::object::{NativeFn, ObjClass, ObjNative};
use crate::script::vm::{
        chunk::{
            BytecodeIndex, ConstIndex, Offset, StackIndex
        },
        debug ::print_stack,
        gc    ::ObjectId,
        object::{
            ObjFunction,
            ObjType,
        }
    };

pub mod chunk;
pub mod debug;
pub mod value;
pub mod compiler;
pub mod object;
pub mod gc;


static DEBUG_TRACE_EXECUTION: bool = true;

static STACK_FRAMES_MAX:       usize = 10000; // ¯\_(ツ)_/¯
static INITIAL_STACK_CAPACITY: usize = 10000; // ¯\_(ツ)_/¯

pub fn interpret(ctx: Context, script_func: ObjectId, constants: Vec<Value>) -> RuntimeResult<()> {

    let mut vm  = Vm::new(ctx, script_func, constants);

    vm.run()
}

// TODO: String interning
pub struct Vm {
    stack:      Vec<Value>,
    call_stack: Vec<CallFrame>,
    constants:  Vec<Value>,
    globals:    HashMap<String, Value>,
    ctx:        Context,
}

struct CallFrame {
    pub stack_offset: StackIndex,
    pub ip:           BytecodeIndex,
    pub func_obj:     ObjectId,
    pub arity:        usize,
}

pub struct RuntimeError {
    pub stack_trace: String,
    pub msg:         String,
    pub line:        usize,
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
    pub fn new(mut ctx: Context, script_func: ObjectId, constants: Vec<Value>) -> Self {
        let call_frame = CallFrame {
           stack_offset: StackIndex   (0),
           ip:           BytecodeIndex(0),
           func_obj:     script_func,
           arity:        0,
        };

        let mut globals = HashMap::new();
        def_natives(&mut globals, &mut ctx);

        let mut stack = Vec::with_capacity(INITIAL_STACK_CAPACITY);
        stack.push(Value::new_obj(script_func));

        Self {
            globals,
            ctx,

            constants,

            stack,

            call_stack: vec![
                call_frame
            ],
        }
    }

    fn run(&mut self) -> RuntimeResult<()> {

        println!(">>>>>>>>>>>>>>>>>>>>> ");

        loop {

            if DEBUG_TRACE_EXECUTION {
                let chunk = self.get_chunk();
                let data  = DisassembleData {
                    ctx:       &self.ctx,
                    lines:     &chunk.lines,
                    stack:     &self.stack,
                    constants: &self.constants,
                };

                chunk.code[**self.ip()].disassemble(&data, **self.ip());
                print_stack(&data);
                println!();
            }

            type O = OpCode;
            match *self.get_instruction() {
                O::GetConstant { index }        => self.op_constant  (index),

                O::DefGlobal   { name_idx }     => self.op_def_global(name_idx),
                O::GetGlobal   { name_idx }     => self.op_get_global(name_idx)?,
                O::SetGlobal   { name_idx }     => self.op_set_global(name_idx)?,

                O::GetLocal    { index }        => self.op_get_local (index),
                O::SetLocal    { index }        => self.op_set_local (index),

                O::JumpIfFalse { offset }       => self.op_jump_if(JumpType::IfFalsey, offset),
                O::JumpIfTrue  { offset }       => self.op_jump_if(JumpType::IfTruthy, offset),
                O::Jump        { offset }       => self.op_jump   (offset),

                O::Loop        { offset }       => self.op_loop   (offset),

                O::Call        { arg_count }    => self.op_call   (arg_count)?,
                O::Class       { name_idx }     => self.op_class  (name_idx),

                O::Nil                          => self.push_stack(Value::Nil),
                O::True                         => self.push_stack(Value::Bool(true)),
                O::False                        => self.push_stack(Value::Bool(false)),

                O::Pop                          => self.op_pop(),

                O::Equal                        => self.op_equal(),
                O::Greater                      => self.op_binary(BinaryOp::Greater)?,
                O::Less                         => self.op_binary(BinaryOp::Less)?,

                O::Add                          => self.op_add()?,
                O::Subtract                     => self.op_binary(BinaryOp::Sub)?,
                O::Multiply                     => self.op_binary(BinaryOp::Mul)?,
                O::Divide                       => self.op_binary(BinaryOp::Div)?,

                O::Not                          => self.op_not    (),

                O::Print                        => self.op_print(),
                O::Negate                       => self.op_negate ()?,
                O::Return                       => {
                    let result = self.pop_stack();
                    let frame  = self.pop_call_stack();

                    if self.call_stack.len() == 0 {
                        return Ok(())
                    }

                    for _ in 0..frame.arity +1 {
                        self.pop_stack();
                    }

                    self.push_stack(result);
                }
            }
        }
    }

    fn get_instruction(&mut self) -> &OpCode {
        **self.ip_mut() += 1;

        &self.get_chunk().code[**self.ip() -1]
    }

    fn get_constant(&self, index: ConstIndex) -> Value {
        self.constants[*index]
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().expect("Stack cannot be empty")
    }

    fn pop_call_stack(&mut self) -> CallFrame {
        self.call_stack.pop().expect("Call stack cannot be empty")
    }

    fn push_stack(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop_number(&mut self) -> RuntimeResult<f64> {
        Ok(self
            .pop_stack()
            .as_number()
            .ok_or_else(||
                self.runtime_error("Operand must be a number".to_owned())
            )?
        )
    }

    fn peek_stack(&self, index: usize) -> Value {
        let index = self.stack.len() - index -1;
        self.stack[index]
    }

    // op codes

    fn op_constant(&mut self, index: ConstIndex) {
        let constant = self.get_constant(index);

        self.push_stack(constant);
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
                Err(self.runtime_error(msg))?
            },
        };

        self.push_stack(*value);

        Ok(())
    }

    fn op_set_global(&mut self, index: ConstIndex) -> RuntimeResult<()> {
        let name = self.get_constant_as_str(index, &self.ctx);

        if self.globals.get(&name).is_none() {
            let msg = format!("Undefined variable '{name}'");
            Err(self.runtime_error(msg))?
        }

        *self.globals.get_mut(&name).unwrap() = self.peek_stack(0);

        Ok(())
    }

    fn op_get_local(&mut self, index: StackIndex) {
        self.push_stack(self.get_local(index));
    }

    fn op_set_local(&mut self, index: StackIndex) {
        let value = self.peek_stack(0);
        self.stack[*index] = value;
    }

    fn op_jump_if(&mut self, jump_type: JumpType, offset: Offset) {
        let is_falsey = self.peek_stack(0).is_falsey();

        let jump_on_false = match jump_type {
            JumpType::IfFalsey => true,
            JumpType::IfTruthy => false,
        };

        if is_falsey == jump_on_false {
            **self.ip_mut() += *offset;
        }
    }

    fn op_jump(&mut self, offset: Offset) {
        **self.ip_mut() += *offset;
    }

    fn op_loop(&mut self, offset: Offset) {
        **self.ip_mut() -= *offset;
    }

    fn op_call(&mut self, arg_count: usize) -> RuntimeResult<()> {
        let val = self.peek_stack(arg_count);
        self.call_value(val, arg_count)
    }

    fn op_class(&mut self, name_idx: ConstIndex) {
        let name = self.get_constant_as_str(name_idx, &self.ctx);
        let obj  = ObjClass::new(name);
        let id   = self.ctx.new_obj(obj.into());

        self.push_stack(Value::Obj(id));
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
            return Err(self.runtime_error("Operands must be two numbers or two strings".to_owned()))
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

    fn runtime_error(&self, msg: String) -> RuntimeError {

        RuntimeError {
            msg:         msg,
            stack_trace: self.stack_trace(),
            line:        self.get_chunk().lines[**self.ip() -1],
        }
    }

    fn get_constant_as_str(&self, index: ConstIndex, ctx: &Context) -> String {
        self.constants[*index]
            .to_str(ctx)
            .expect("Expect constant value to be of type ObjString")
            .to_owned()
    }

    fn get_chunk(&self) -> &Chunk {
        let obj = self.call_frame();

        let obj = self.ctx.get(obj.func_obj);

        let obj: &ObjFunction = obj.into();

        &obj.chunk
    }

    fn ip(&self) -> &BytecodeIndex {
        &self.call_frame().ip
    }

    fn ip_mut(&mut self) -> &mut BytecodeIndex {
        &mut self.call_frame_mut().ip
    }

    fn call_frame(&self) -> &CallFrame {
        self.call_stack.last().expect("Call stack must not be empty")
    }

    fn call_frame_mut(&mut self) -> &mut CallFrame {
        self.call_stack.last_mut().expect("Call stack must not be empty")
    }

    fn get_local(&self, index: StackIndex) -> Value {
        let frame = self.call_frame();

        self.stack[*frame.stack_offset + *index]
    }

    fn call_value(&mut self, value: Value, arg_count: usize) -> RuntimeResult<()> {

        let obj = match value {
            Value::Obj(obj) => obj,

            _ => Err(self.runtime_error(
                format!("Value of type '{}' is not callable", value.display_type())
            ))?
        };

        let obj = self.ctx.get(obj);
        match &obj.type_ {

            ObjType::Function(func) => self.call       (obj.id, func.arity, arg_count)?,
            ObjType::NativeFn(func) => self.call_native(arg_count, func.func),

            _ => Err(self.runtime_error(
                format!("Object of type '{:?}' is not callable", obj.type_)
            ))?
        }

        Ok(())
    }

    fn call(&mut self, func_obj: ObjectId, func_arity: usize, arg_count: usize) -> RuntimeResult<()> {

        if func_arity != arg_count {
            Err(self.runtime_error(format!("Expected {} arguments but got {}", func_arity, arg_count)))?
        }

        if self.call_stack.len() > STACK_FRAMES_MAX {
            Err(self.runtime_error("Stack overflow".to_owned()))?
        }

        let offset = self.stack.len() - (arg_count + 1);

        self.call_stack.push(CallFrame {
            stack_offset: StackIndex   (offset),
            ip:           BytecodeIndex(0),
            func_obj,
            arity:        func_arity,
        });

        Ok(())
    }

    fn call_native(&mut self, func_arity: usize, func: NativeFn) {
        let stack_top = self.stack.len() - func_arity;
        let result    = func(&self.stack[stack_top..]);

        for _ in 0..func_arity {
            self.stack.pop();
        }

        self.stack.pop(); // remove the callee temporary
        self.push_stack(result);
    }

    fn stack_trace(&self) -> String {

        let mut results = String::new();

        for frame in self.call_stack.iter().rev() {

            let func: &ObjFunction = self.ctx.get(frame.func_obj).into();
            let line = func.chunk.lines[*frame.ip -1];

            writeln!(results, "  [line {}] in {}", line, func.name).unwrap();
        }

        results
    }
}



fn concatenate(val1: &str, val2: &str, ctx: &mut Context) -> Value {

    let str = format!("{val1}{val2}");

    let id  = ctx.new_obj(str.into());

    Value::new_obj(id)
}

fn def_natives(globals: &mut HashMap<String, Value>, ctx: &mut Context) {

    let mut make_global = |name: &str, func| {
        let obj = ObjNative::new(name.to_owned(), func);
        let obj = ctx.new_obj(obj.into());

        globals.insert(name.to_owned(), Value::Obj(obj))
    };

    make_global("clock", clock_native);
}

fn clock_native(_: &[Value]) -> Value {
    let start = SystemTime::now();

    let time_since = start.duration_since(UNIX_EPOCH).unwrap();

    Value::Number(time_since.as_millis() as f64 / 1000.0)
}
