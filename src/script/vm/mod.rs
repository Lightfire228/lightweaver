use std::cell::{RefMut};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap};
use std::fmt::Write;

use chunk::{Chunk, OpCode};
use gc_arena::lock::RefLock;
use gc_arena::{Arena, Collect, Gc, Mutation, Rootable};
use value::Value;

use crate::script::vm::chunk::{StackOffset, UpvalueIndex};
use crate::script::vm::debug::DisassembleData;
use crate::script::vm::object::*;
use crate::script::vm::{
        chunk::{
            BytecodeIndex, ConstIndex, Offset, StackIndex
        },
        debug ::print_stack,
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


// static DEBUG_TRACE_EXECUTION: bool = true;
static DEBUG_TRACE_EXECUTION: bool = false;

static STACK_FRAMES_MAX:       usize = 10000; // ¯\_(ツ)_/¯
static INITIAL_STACK_CAPACITY: usize = 10000; // ¯\_(ツ)_/¯

pub fn interpret(root: ArenaRoot) -> RuntimeResult<()> {

    let mut vm  = Vm::new(root);

    vm.run()
}

// TODO: String interning
pub struct Vm {
    root: ArenaRoot,
}

#[derive(Collect)]
#[collect(no_drop)]
pub struct Root<'gc> {
    pub call_stack:  Vec    <Gc<'gc, RefLock<CallFrame<'gc>>>>,

    pub functions:   Vec    <Gc<'gc, ObjFunction<'gc>>>,

    pub stack:       Vec    <Value<'gc>>,
    pub constants:   Vec    <Value<'gc>>,

    pub globals:     HashMap<String, Value<'gc>>,

    pub ip:   BytecodeIndex,
}

pub type ArenaRoot = Arena::<Rootable![Root<'_>]>;

#[derive(Collect)]
#[collect(no_drop)]
struct CallFrame<'gc> {
    pub stack_len:    StackIndex,
    pub ret_ip:       BytecodeIndex,
    pub closure:      Gc<'gc, RefLock<Obj<'gc>>>,
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
    pub fn new(mut root: ArenaRoot) -> Self {

        root.mutate_root(|ctx, root| {

            let script_func = root.functions.pop().expect("expect top level script function");

            let script_closure = Gc::new(
                ctx,
                RefLock::new(
                    Obj::new_closure(script_func, 0, vec![])
                )
            );

            let call_frame = CallFrame {
                stack_len:    StackIndex   (0),
                ret_ip:       BytecodeIndex(0),
                closure:      script_closure,
                arity:        0,
            };
            let call_frame = Gc::new(ctx, RefLock::new(call_frame));

            root.call_stack.push(call_frame);


            def_natives(&mut root.globals, ctx);


            let mut stack = Vec::with_capacity(INITIAL_STACK_CAPACITY);
            stack.push(Value::new_obj_mut(script_closure));

            root.stack = stack;
        });


        Self {
            root,
        }
    }

    fn run<'gc>(&mut self) -> RuntimeResult<()> {

        loop {

            if DEBUG_TRACE_EXECUTION {
                self.root.mutate(|_ctx, root| {

                    let chunk = root.get_chunk();
                    let data  = DisassembleData {
                        name:      "",
                        lines:     &chunk.lines,
                        stack:     &root.stack,
                        constants: &root.constants,
                    };

                    chunk.code[*root.ip].disassemble(&data, *root.ip);
                    print_stack(&data);
                    println!();
                })
            }

            self.root.collect_all();

            let done = self.root.mutate_root(|ctx, root| {
                root.run_instruction(ctx)
            });

            if done? {
                break Ok(())
            }
        }
    }
}

impl<'gc> Root<'gc> {

    fn run_instruction(&'gc mut self, ctx: &'gc Mutation<'gc>) -> RuntimeResult<bool>{

        match self.get_instruction() {
            OpCode::GetConstant { index }            => self.op_constant    (index),

            OpCode::DefGlobal   { name_idx }         => self.op_def_global  (name_idx),
            OpCode::GetGlobal   { name_idx }         => self.op_get_global  (name_idx)?,
            OpCode::SetGlobal   { name_idx }         => self.op_set_global  (name_idx)?,

            OpCode::GetProperty { name_idx }         => self.op_get_property(name_idx)?,
            OpCode::SetProperty { name_idx }         => self.op_set_property(name_idx, ctx),

            OpCode::GetLocal    { offset }           => self.op_get_local   (offset),
            OpCode::SetLocal    { offset }           => self.op_set_local   (offset),

            OpCode::GetUpvalue  { index }            => self.op_get_upvalue (index),
            OpCode::SetUpvalue  { index }            => self.op_set_upvalue (index),
            OpCode::PushUpvalue { index }            => self.op_push_upvalue(index),

            OpCode::JumpIfFalse { offset }           => self.op_jump_if     (JumpType::IfFalsey, offset),
            OpCode::JumpIfTrue  { offset }           => self.op_jump_if     (JumpType::IfTruthy, offset),
            OpCode::Jump        { offset }           => self.op_jump        (offset),

            OpCode::Loop        { offset }           => self.op_loop        (offset),

            OpCode::Call        { arg_count }        => self.op_call        (arg_count, ctx)?,
            OpCode::Class       { name_idx }         => self.op_class       (name_idx, ctx),
            OpCode::Closure     { func }             => self.op_closure     (func),

            OpCode::Nil                              => self.push_stack     (Value::Nil),
            OpCode::True                             => self.push_stack     (Value::Bool(true)),
            OpCode::False                            => self.push_stack     (Value::Bool(false)),

            OpCode::Pop                              => self.op_pop         (),

            OpCode::Equal                            => self.op_equal       (),
            OpCode::Greater                          => self.op_binary      (BinaryOp::Greater)?,
            OpCode::Less                             => self.op_binary      (BinaryOp::Less)?,

            OpCode::Add                              => self.op_add         (ctx)?,
            OpCode::Subtract                         => self.op_binary      (BinaryOp::Sub)?,
            OpCode::Multiply                         => self.op_binary      (BinaryOp::Mul)?,
            OpCode::Divide                           => self.op_binary      (BinaryOp::Div)?,

            OpCode::Not                              => self.op_not         (),

            OpCode::Print                            => self.op_print       (),
            OpCode::Negate                           => self.op_negate      ()?,
            OpCode::Return                           => {
                let result = self.pop_stack();
                let frame  = self.pop_call_stack().borrow();

                let is_empty = self.call_stack.is_empty();

                if is_empty {
                    return Ok(true)
                }

                let diff = self.stack.len() - *frame.stack_len;

                for _ in 0..diff {
                    self.stack.pop();
                }

                self.push_stack(result);
                self.ip = frame.ret_ip;

            }
        };

        Ok(false)
    }

    fn get_instruction(&mut self) -> OpCode<'gc> {
        let ip = *self.ip;

        *self.ip += 1;

        self.get_chunk().code[ip]
    }

    fn get_constant(&self, index: ConstIndex) -> Value<'gc> {
        self.constants[*index]
    }

    fn pop_stack(&mut self) -> Value<'gc> {
        self.stack.pop().expect("Stack cannot be empty")
    }

    fn pop_call_stack(&mut self) -> Gc<'gc, RefLock<CallFrame<'gc>>> {
        self.call_stack.pop().expect("Call stack cannot be empty")
    }

    fn push_stack(&mut self, val: Value<'gc>) {
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

    fn peek_stack(&self, index: usize) -> Value<'gc> {
        let index = self.from_stack_top(index);
        self.stack[index]
    }

    fn stack_swap(&mut self, index: StackIndex, value: Value<'gc>) -> Value<'gc>  {
        let top   = self.stack.len();

        self.stack.push(value);
        self.stack.swap(top, *index);

        self.pop_stack()
    }

    fn from_stack_top(&self, index: usize) -> usize {
        self.stack.len() - index -1
    }

    // op codes

    fn op_constant(&'gc mut self, index: ConstIndex) {
        let constant = self.get_constant(index);

        self.push_stack(constant);
    }

    fn op_def_global(&mut self, name: ConstIndex) {
        let name = self.get_constant_as_str(name);
        let val  = self.pop_stack();

        self.globals.insert(name, val);
    }

    fn op_get_global(&mut self, index: ConstIndex) -> RuntimeResult<()> {
        let name  = self.get_constant_as_str(index);

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
        let name = self.get_constant_as_str(index);
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
        let name     = self.get_constant_as_str(index);
        let val      = self.pop_stack();


        let obj = match val {
            Value::Obj   (gc) => gc.as_ref(),
            Value::ObjMut(gc) => &*gc.borrow(),
            x                 => panic!("Value not an object: '{x}'"),
        };

        let ObjType::Instance(instance) = &obj.type_ else {
            panic!("Object not an instance: '{}'", obj);
        };

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

    fn op_set_property(&mut self, index: ConstIndex, ctx: &Mutation<'gc>) {
        let name = self.get_constant_as_str(index);
        let val  = self.pop_stack();
        let obj  = self.pop_stack();

        let mut obj = match obj {
            Value::ObjMut(gc) => gc.borrow_mut(ctx),
            Value::Obj   (gc) => panic!("Value not a mutable object: '{gc}'"),
            x                 => panic!("Value not an object: '{x}'"),
        };

        let ObjType::Instance(instance) = &mut obj.type_ else {
            panic!("Object not an instance: '{}'", obj);
        };

        instance.fields
            .insert(name, val)
        ;

        self.push_stack(val);
    }


    fn op_get_local(&mut self, index: StackOffset) {
        let index = self.from_stack_top(*index);
        let val   = self.get_local(StackIndex(index));

        self.push_stack(val);
    }

    fn op_set_local(&mut self, index: StackOffset) {
        let value = self.peek_stack(0);
        let index = self.from_stack_top(*index);

        self.stack[index] = value;
    }

    fn op_get_upvalue(&mut self, _index: UpvalueIndex) {
        // TODO:
        // let upvalue = self.upvalues[*index];
        // self.push_stack(Value::Obj(upvalue));
    }

    fn op_set_upvalue(&mut self, _index: UpvalueIndex) {
        // TODO:
        // let value = self.peek_stack(0);
        // let obj   = self.upvalues[*index];

        // let obj: &mut ObjValue = self.ctx.get_mut(obj).try_into().unwrap();

        // obj.value = value;
    }

    fn op_push_upvalue(&mut self, _index: StackOffset) {
        // TODO:
        // let index = StackIndex(self.from_stack_top(*index));

        // let val = self.stack_swap(index, Value::Nil);

        // let obj = self.ctx.new_obj(ObjValue::new(val).into());
        // self.upvalues.push(obj);

        // self.stack_swap(index, Value::Obj(obj));
    }

    fn op_jump_if(&mut self, jump_type: JumpType, offset: Offset) {
        let is_falsey = self.peek_stack(0).is_falsey();

        let jump_on_false = match jump_type {
            JumpType::IfFalsey => true,
            JumpType::IfTruthy => false,
        };

        if is_falsey == jump_on_false {
            *self.ip += *offset;
        }
    }

    fn op_jump(&mut self, offset: Offset) {
        *self.ip += *offset;
    }

    fn op_loop(&mut self, offset: Offset) {
        *self.ip -= *offset;
    }

    fn op_call(&mut self, arg_count: usize, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {
        let val = self.peek_stack(arg_count);
        self.call_value(val, arg_count, ctx)
    }

    fn op_class(&mut self, name_idx: ConstIndex, ctx: &Mutation<'gc>) {
        let name = self.get_constant_as_str(name_idx);
        let obj  = Obj::new_class(name);
        let obj  = Gc ::new(ctx, obj);

        self.push_stack(Value::Obj(obj));
    }

    fn op_closure(&mut self, _func: Gc<'gc, ObjFunction>) {
        // TODO:
        // self.pop_stack();

        // let func_val           = self.get_constant(func);
        // let obj                = func_val.as_obj(&self.ctx).unwrap();
        // let func: &ObjFunction = obj.try_into().unwrap();

        // let closed_objs = self.stack.iter()
        //     .filter_map(|v| match v {
        //         Value::Closed(_) => Some(v.clone()),
        //         _                => None,
        //     })
        //     .collect()
        // ;

        // let closure = ObjClosure::new(obj.id, func.arity, closed_objs);

        // let id = self.ctx.new_obj(closure.into());

        // self.push_stack(Value::Obj(id));
    }

    fn op_close_var(&mut self, _index: StackIndex) {
        // TODO:
        // let val = self.stack_swap(index, Value::Nil);

        // let obj = self.ctx.new_obj(ObjValue::new(val).into());

        // let val = Value::Closed(obj);

        // self.stack_swap(index, val);
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

    fn op_add(&mut self, ctx: &Mutation<'gc>) -> RuntimeResult<()> {

        let b = self.pop_stack();
        let a = self.pop_stack();

        if let (Some(a), Some(b)) = (a.to_str(), b.to_str()) {
            let result = format!("{a}{b}");

            let obj = Obj::new_string(result);
            let obj = Gc ::new(ctx, obj);

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

        println!("{}", val)
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

        println!("{}", constant);
    }

    fn concatenate(&mut self, val1: &str, val2: &str, ctx: &'gc Mutation<'gc>) {
        let val = concatenate(val1, val2, ctx);
        self.push_stack(val);
    }

    // utils

    fn runtime_error(&self, msg: String) -> RuntimeError {

        RuntimeError {
            msg:         msg,
            stack_trace: self.stack_trace(),
            line:        self.get_chunk().lines[*self.ip -1],
        }
    }

    fn get_constant_as_str(&self, index: ConstIndex) -> String {
        self.constants[*index]
            .to_str()
            .expect("Expect constant value to be of type ObjString")
            .to_owned()
    }

    fn get_chunk(&self) -> Gc<'gc, Chunk<'gc>> {
        let frame     = self.call_frame();
        let frame_ref = frame.borrow();

        let obj     = frame_ref.closure;
        let obj_ref = obj.borrow();

        let ObjType::Closure(closure) = &obj_ref.type_ else {
            panic!("Object not a closure type: '{}'", obj_ref);
        };

        let chunk = &closure.function.chunk;

        *chunk
    }

    fn call_frame(&self) -> Gc<'gc, RefLock<CallFrame<'gc>>> {
        *self.call_stack.last().expect("Call stack must not be empty")
    }

    fn call_frame_mut(&mut self, ctx: &'gc Mutation<'gc>) -> RefMut<CallFrame<'gc>> {
        let frame = self.call_frame();
        frame.borrow_mut(ctx)
    }

    // fn stack_index(&self, index: StackIndex) -> usize {

    //     self.stack.len() -1 -*index
    // }

    fn get_local(&self, index: StackIndex) -> Value<'gc> {
        self.stack[*index]
    }

    fn call_value(&mut self, value: Value, arg_count: usize, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {

        let obj = match value {
            Value::Obj   (obj)   => obj.as_ref(),
            Value::ObjMut(obj)   => &*obj.borrow(),

            _ => Err(self.runtime_error(
                format!("Value of type '{}' is not callable", value.display_type())
            ))?
        };

        match &obj.type_ {

            // ObjType::Function(func)    => self.call       (func,           func.arity, arg_count, ctx)?,
            ObjType::NativeFn(func)    => self.call_native(arg_count,      func.func),
            ObjType::Class   (class)   => self.call_class (class,          arg_count)?,
            ObjType::Closure (cls)     => self.call       (cls, arg_count, arg_count,  ctx)?,

            _ => Err(self.runtime_error(
                format!("Object of type '{:?}' is not callable", obj.type_)
            ))?
        }

        Ok(())
    }

    fn call(&mut self, closure: &ObjClosure, func_arity: usize, arg_count: usize, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {

        if func_arity != arg_count {
            Err(self.runtime_error(format!("Expected {} arguments but got {}", func_arity, arg_count)))?
        }

        if self.call_stack.len() > STACK_FRAMES_MAX {
            Err(self.runtime_error("Stack overflow".to_owned()))?
        }

        let closure = Gc::new(ctx, closure);

        let frame = CallFrame {
            stack_len:    StackIndex   (self.stack.len() - func_arity -1),
            ret_ip:       BytecodeIndex(*self.ip +1),
            closure,
            arity:        func_arity,
        };



        self.call_stack.push();

        Ok(())
    }

    fn call_native(&mut self, func_arity: usize, func: NativeFn<'gc>) {
        let stack_top = self.stack.len() - func_arity;
        let result    = func.0(&self.stack[stack_top..]);

        for _ in 0..func_arity {
            self.stack.pop();
        }

        self.stack.pop(); // remove the callee temporary
        self.push_stack(result);
    }


    fn call_class(&mut self, class: Gc<'gc, ObjClass>, _arg_count: usize, ctx: &Mutation<'gc>) -> RuntimeResult<()> {

            let obj = Obj::new_instance(class);
            let obj = Gc::new(ctx, obj);

            self.stack.pop();
            self.stack.push(Value::Obj(obj));


        Ok(())
    }

    fn stack_trace(&self) -> String {

        let mut results = String::new();

        for frame in self.call_stack.iter().rev() {

            let frame = frame.borrow();


            // let closure: &ObjClosure  = self.ctx.get(frame  .closure) .try_into().unwrap();
            let func = frame.closure.borrow();

            let ObjType::Function(func) = &func.type_ else {
                panic!("Object not a function: '{}'", func);
            };

            let line = func.chunk.lines[*frame.ret_ip -1];

            writeln!(results, "  [line {}] in {}", line, func.name).unwrap();
        }

        results
    }
}



fn concatenate<'gc>(val1: &str, val2: &str, ctx: &'gc Mutation<'gc>) -> Value<'gc> {

    let str = format!("{val1}{val2}");

    let obj = Obj::new_string(str);
    let obj = Gc ::new(ctx, obj);

    Value::new_obj(obj)
}

fn def_natives<'gc>(globals: &'gc mut HashMap<String, Value<'gc>>, ctx: &'gc Mutation<'gc>) {

    let mut make_global = |name: &str, func| {

        let obj = Obj::new_native_fn(name.to_owned(), func);
        let obj = Gc::new(ctx, obj);

        globals.insert(name.to_owned(), Value::new_obj(obj))
    };

    make_global("clock", NativeFn(clock_native));
}

fn clock_native<'gc>(_: &[Value<'gc>]) -> Value<'gc> {
    let start = SystemTime::now();

    let time_since = start.duration_since(UNIX_EPOCH).unwrap();

    Value::Number(time_since.as_millis() as f64 / 1000.0)
}


// #[cfg(test)]
// mod tests {
//     use std::fs;

//     use crate::script::{parser, resolver::resolve, scanner, vm::compiler::{CompilerOut, compile}};

//     use super::*;

//     fn source(file: &str) -> String {
//         let path = format!("./test_scripts/unit_tests/vm/{file}");
//         fs::read_to_string(&path).unwrap()
//     }

//     fn init(source: String) -> Vm {

//         let mut ctx    = Context::new();
//         let     tokens = scanner::scan_tokens(&source).unwrap();
//         let mut ast    = parser ::parse_ast  (tokens) .unwrap();
//         resolve(&mut ast, &mut ctx);

//         let out = compile(ast, &mut ctx).unwrap();

//         let func = out.function_ids.first().unwrap();

//         dbg_funcs(&out, &ctx);

//         Vm::new(ctx, *func, out.constants)

//     }

//     fn assert_stack_empty(code: String) {
//         let mut vm = init(code);

//         vm.run().unwrap();

//         assert!(vm.stack.is_empty());

//     }

//     fn dbg_funcs(out: &CompilerOut, ctx: &Context) {
//         for f_id in &out.function_ids {
//             let obj = ctx.get(*f_id);

//             let func: &ObjFunction = obj.try_into().unwrap();

//             func.chunk.disassemble(&DisassembleData {
//                 ctx:       &ctx,
//                 lines:     &func.chunk.lines,
//                 stack:     &[],
//                 constants: &out.constants,
//             });
//         }
//     }

//     #[test]
//     fn test_stack_empty_base_syntax() {
//         assert_stack_empty(source("test_stack_base_syntax.lox"));
//     }

//     #[test]
//     fn test_stack_empty_functions_simple() {
//         assert_stack_empty(source("test_stack_functions_simple.lox"));
//     }

//     #[test]
//     fn test_stack_empty_functions_recursion() {
//         assert_stack_empty(source("test_stack_functions_recursion.lox"));
//     }

//     #[test]
//     fn test_stack_empty_functions_nested_2() {
//         assert_stack_empty(source("test_stack_functions_nested_2.lox"));
//     }

// }
