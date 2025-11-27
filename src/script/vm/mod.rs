use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap};
use std::fmt::Write;

use chunk::{Chunk, OpCode};
use value::Value;
use gc::Context;

use crate::script::vm::chunk::UpvalueIndex;
use crate::script::vm::debug::DisassembleData;
use crate::script::vm::object::{NativeFn, ObjClass, ObjClosure, ObjInstance, ObjNative, ObjValue};
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
// static DEBUG_TRACE_EXECUTION: bool = false;

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
    upvalues:   Vec<ObjectId>,
    globals:    HashMap<String, Value>,
    ctx:        Context,
}

struct CallFrame {
    pub stack_len:    StackIndex,
    pub ip:           BytecodeIndex,
    pub closure:      ObjectId,
    pub arity:        usize,
}

#[derive(Debug)]
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

        let closure = ObjClosure::new(script_func, 0, vec![]);
        let closure = ctx.new_obj(closure.into());

        let call_frame = CallFrame {
           stack_len:    StackIndex   (0),
           ip:           BytecodeIndex(0),
           closure,
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
            upvalues: vec![],

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
                O::GetConstant { index }            => self.op_constant    (index),

                O::DefGlobal   { name_idx }         => self.op_def_global  (name_idx),
                O::GetGlobal   { name_idx }         => self.op_get_global  (name_idx)?,
                O::SetGlobal   { name_idx }         => self.op_set_global  (name_idx)?,

                O::GetProperty { name_idx }         => self.op_get_property(name_idx)?,
                O::SetProperty { name_idx }         => self.op_set_property(name_idx),

                O::GetLocal    { index }            => self.op_get_local   (index),
                O::SetLocal    { index }            => self.op_set_local   (index),

                O::GetUpvalue  { index }            => self.op_get_upvalue (index),
                O::SetUpvalue  { index }            => self.op_set_upvalue (index),
                O::PushUpvalue { index }            => self.op_push_upvalue(index),

                O::JumpIfFalse { offset }           => self.op_jump_if     (JumpType::IfFalsey, offset),
                O::JumpIfTrue  { offset }           => self.op_jump_if     (JumpType::IfTruthy, offset),
                O::Jump        { offset }           => self.op_jump        (offset),

                O::Loop        { offset }           => self.op_loop        (offset),

                O::Call        { arg_count }        => self.op_call        (arg_count)?,
                O::Class       { name_idx }         => self.op_class       (name_idx),
                O::Closure     { func_idx }         => self.op_closure     (func_idx),

                O::Nil                              => self.push_stack     (Value::Nil),
                O::True                             => self.push_stack     (Value::Bool(true)),
                O::False                            => self.push_stack     (Value::Bool(false)),

                O::Pop                              => self.op_pop         (),

                O::Equal                            => self.op_equal       (),
                O::Greater                          => self.op_binary      (BinaryOp::Greater)?,
                O::Less                             => self.op_binary      (BinaryOp::Less)?,

                O::Add                              => self.op_add         ()?,
                O::Subtract                         => self.op_binary      (BinaryOp::Sub)?,
                O::Multiply                         => self.op_binary      (BinaryOp::Mul)?,
                O::Divide                           => self.op_binary      (BinaryOp::Div)?,

                O::Not                              => self.op_not         (),

                O::Print                            => self.op_print       (),
                O::Negate                           => self.op_negate      ()?,
                O::Return                           => {
                    let result = self.pop_stack();
                    let frame  = self.pop_call_stack();

                    if self.call_stack.len() == 0 {
                        return Ok(())
                    }

                    println!("stack len:   {}", self.stack.len());
                    println!("frame stack: {}", *frame.stack_len);
                    let diff = self.stack.len() - *frame.stack_len;

                    for _ in 0..diff {
                        self.stack.pop();
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
        let index = self.from_stack_top(index);
        self.stack[index]
    }

    fn stack_swap(&mut self, index: StackIndex, value: Value) -> Value {
        let top   = self.stack.len();
        let index = self.from_stack_top(*index);

        self.stack.push(value);
        self.stack.swap(top, index);

        self.pop_stack()
    }

    fn from_stack_top(&self, index: usize) -> usize {
        self.stack.len() - index -1
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

        let value = self.globals
            .get(&name)
            .ok_or_else(|| {
                self.runtime_error(format!("Undefined variable '{name}'"))
            })
            ?
        ;

        self.push_stack(*value);

        Ok(())
    }

    fn op_set_global(&mut self, index: ConstIndex) -> RuntimeResult<()> {
        let name = self.get_constant_as_str(index, &self.ctx);
        let val  = self.peek_stack(0);

        self.globals
            .get_mut ( &name )
            .and_then( |global| {
                *global = val;
                Some(())
            })
            .ok_or_else(|| {
                self.runtime_error(format!("Undefined variable '{name}'"))
            })
    }

    fn op_get_property(&mut self, index: ConstIndex) -> RuntimeResult<()> {
        let name     = self.get_constant_as_str(index, &self.ctx);
        let val      = self.pop_stack();

        let obj      = val.as_obj(&self.ctx).unwrap();
        let instance = obj.to_instance()    .unwrap();

        let value = instance.fields
            .get(&name)
            .ok_or_else(|| {
                self.runtime_error(format!("Undefined property '.{name}'"))
            })
            ?
        ;

        self.push_stack(*value);

        Ok(())
    }

    fn op_set_property(&mut self, index: ConstIndex) {
        let     name = self.get_constant_as_str(index, &self.ctx);
        let     val  = self.pop_stack();
        let mut obj  = self.pop_stack();
        let     obj  = obj.as_obj_mut(&mut self.ctx);

        let instance = obj.unwrap().to_instance_mut()        .unwrap();

        instance.fields
            .insert(name, val)
        ;

        self.push_stack(val);
    }


    fn op_get_local(&mut self, index: StackIndex) {
        self.push_stack(self.get_local(index));
    }

    fn op_set_local(&mut self, index: StackIndex) {
        let value = self.peek_stack(0);
        self.stack[*index] = value;
    }

    fn op_get_upvalue(&mut self, index: UpvalueIndex) {
        let upvalue = self.upvalues[*index];
        self.push_stack(Value::Obj(upvalue));
    }

    fn op_set_upvalue(&mut self, index: UpvalueIndex) {
        let value = self.peek_stack(0);
        let obj   = self.upvalues[*index];

        let obj: &mut ObjValue = self.ctx.get_mut(obj).try_into().unwrap();

        obj.value = value;
    }

    fn op_push_upvalue(&mut self, index: Offset) {

        let index = StackIndex(self.from_stack_top(*index));

        let val = self.stack_swap(index, Value::Nil);

        let obj = self.ctx.new_obj(ObjValue::new(val).into());
        self.upvalues.push(obj);

        self.stack_swap(index, Value::Obj(obj));
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

    fn op_closure(&mut self, func_idx: ConstIndex) {

        self.pop_stack();

        let func_val           = self.get_constant(func_idx);
        let obj                = func_val.as_obj(&self.ctx).unwrap();
        let func: &ObjFunction = obj.try_into().unwrap();

        let closed_objs = self.stack.iter()
            .filter_map(|v| match v {
                Value::Closed(_) => Some(v.clone()),
                _                => None,
            })
            .collect()
        ;

        let closure = ObjClosure::new(obj.id, func.arity, closed_objs);

        let id = self.ctx.new_obj(closure.into());

        self.push_stack(Value::Obj(id));
    }

    fn op_close_var(&mut self, index: StackIndex) {
        let val = self.stack_swap(index, Value::Nil);

        let obj = self.ctx.new_obj(ObjValue::new(val).into());

        let val = Value::Closed(obj);

        self.stack_swap(index, val);
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

        let obj = self.ctx.get(obj.closure);
        let obj: &ObjClosure  = obj.try_into().unwrap();

        let obj = self.ctx.get(obj.function);
        let obj: &ObjFunction = obj.try_into().unwrap();

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

    // fn stack_index(&self, index: StackIndex) -> usize {

    //     self.stack.len() -1 -*index
    // }

    fn get_local(&self, index: StackIndex) -> Value {
        self.stack[*index]
    }

    fn call_value(&mut self, value: Value, arg_count: usize) -> RuntimeResult<()> {

        let obj = match value {
            Value::Obj  (obj)   => obj,

            _ => Err(self.runtime_error(
                format!("Value of type '{}' is not callable", value.display_type())
            ))?
        };

        let obj = self.ctx.get(obj);
        match &obj.type_ {

            // ObjType::Function(func)    => self.call       (obj.id, func.arity, arg_count)?,
            ObjType::NativeFn(func)    => self.call_native(arg_count, func.func),
            ObjType::Class   (_)       => self.call_class (obj.id, arg_count)?,
            ObjType::Closure (_)       => self.call       (obj.id, arg_count, arg_count)?,

            _ => Err(self.runtime_error(
                format!("Object of type '{:?}' is not callable", obj.type_)
            ))?
        }

        Ok(())
    }

    fn call(&mut self, closure: ObjectId, func_arity: usize, arg_count: usize) -> RuntimeResult<()> {

        if func_arity != arg_count {
            Err(self.runtime_error(format!("Expected {} arguments but got {}", func_arity, arg_count)))?
        }

        if self.call_stack.len() > STACK_FRAMES_MAX {
            Err(self.runtime_error("Stack overflow".to_owned()))?
        }

        self.call_stack.push(CallFrame {
            stack_len:    StackIndex   (self.stack.len() - func_arity -1),
            ip:           BytecodeIndex(0),
            closure,
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


    fn call_class(&mut self, class_id: ObjectId, _arg_count: usize) -> RuntimeResult<()> {

            let obj = ObjInstance::new(class_id);
            let id  = self.ctx.new_obj(obj.into());

            self.stack.pop();
            self.stack.push(Value::Obj(id));


        Ok(())
    }

    fn stack_trace(&self) -> String {

        let mut results = String::new();

        for frame in self.call_stack.iter().rev() {

            let closure: &ObjClosure  = self.ctx.get(frame  .closure) .try_into().unwrap();
            let func:    &ObjFunction = self.ctx.get(closure.function).try_into().unwrap();

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


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::script::{parser, resolver::resolve, scanner, vm::compiler::{CompilerOut, compile}};

    use super::*;

    fn source(file: &str) -> String {
        let path = format!("./test_scripts/unit_tests/vm/{file}");
        fs::read_to_string(&path).unwrap()
    }

    fn init(source: String) -> Vm {

        let mut ctx    = Context::new();
        let     tokens = scanner::scan_tokens(&source).unwrap();
        let mut ast    = parser ::parse_ast  (tokens) .unwrap();
        resolve(&mut ast, &mut ctx);

        let out = compile(ast, &mut ctx).unwrap();

        let func = out.function_ids.first().unwrap();

        dbg_funcs(&out, &ctx);

        Vm::new(ctx, *func, out.constants)

    }

    fn assert_stack_empty(code: String) {
        let mut vm = init(code);

        vm.run().unwrap();

        assert!(vm.stack.is_empty());

    }

    fn dbg_funcs(out: &CompilerOut, ctx: &Context) {
        for f_id in &out.function_ids {
            let obj = ctx.get(*f_id);

            let func: &ObjFunction = obj.try_into().unwrap();

            func.chunk.disassemble(&DisassembleData {
                ctx:       &ctx,
                lines:     &func.chunk.lines,
                stack:     &[],
                constants: &out.constants,
            });
        }
    }

    #[test]
    fn test_stack_empty_base_syntax() {
        assert_stack_empty(source("test_stack_base_syntax.lox"));
    }

    #[test]
    fn test_stack_empty_functions_simple() {
        assert_stack_empty(source("test_stack_functions_simple.lox"));
    }

    #[test]
    fn test_stack_empty_functions_nested_1() {
        assert_stack_empty(source("test_stack_functions_nested_1.lox"));
    }

    #[test]
    fn test_stack_empty_functions_nested_2() {
        assert_stack_empty(source("test_stack_functions_nested_2.lox"));
    }

}
