use std::cell::{RefMut};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap};
use std::fmt::Write;

use chunk::{Chunk, OpCode};
use gc_arena::lock::{GcRefLock, RefLock};
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
        }
    };

pub mod chunk;
pub mod debug;
pub mod value;
pub mod compiler;
pub mod object;


static DEBUG_TRACE_EXECUTION: bool = true;
// static DEBUG_TRACE_EXECUTION: bool = false;

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
    call_stack:  Vec    <GcRefLock<'gc, CallFrame<'gc>>>,

    functions:   Vec    <Gc<'gc, ObjFunction<'gc>>>,

    stack:       Vec    <Value<'gc>>,
    constants:   Vec    <Value<'gc>>,

    globals:     HashMap<String, Value<'gc>>,

    ip:          BytecodeIndex,

    capture_out: bool,
    out:         Vec<String>,
}

pub type ArenaRoot = Arena::<Rootable![Root<'_>]>;

#[derive(Collect)]
#[collect(no_drop)]
pub struct CallFrame<'gc> {
    pub stack_len:    StackIndex,
    pub ret_ip:       BytecodeIndex,
    pub closure:      GcRefLock<'gc, ObjClosure<'gc>>,
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
            let script_func = root.functions.first().expect("expect top level script function");

            let script_closure = ObjectMut::new_closure(*script_func, 0, vec![], ctx);
            let ObjectMut::Closure(cls) = script_closure else {
                unreachable!()
            };

            let call_frame = CallFrame {
                stack_len:    StackIndex   (0),
                ret_ip:       BytecodeIndex(0),
                closure:      cls,
                arity:        0,
            };
            let call_frame = Gc::new(ctx, RefLock::new(call_frame));

            root.call_stack.push(call_frame);


            def_natives(&mut root.globals, ctx);


            let mut stack = Vec::with_capacity(INITIAL_STACK_CAPACITY);
            stack.push(Value::new_obj(ObjPtr::ObjMut(script_closure)));

            root.stack = stack;
        });


        Self {
            root,
        }
    }

    fn run<'gc>(&mut self) -> RuntimeResult<()> {

        loop {
            self.root.collect_debt();

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

    pub fn new() -> Self {
        Self {
            call_stack:  vec![],
            stack:       vec![],

            functions:   vec![],
            constants:   vec![],

            globals:     HashMap::new(),

            ip:          BytecodeIndex(0),

            capture_out: false,
            out:         vec![],
        }
    }

    pub fn new_test() -> Self {
        let mut x = Self::new();

        x.capture_out = true;

        x
    }

    pub fn dbg_funcs(&self) {

        for func in &self.functions {

            let chunk = func.chunk.borrow();

            chunk.disassemble(&DisassembleData {
                name:      &func.name,
                lines:     &chunk.lines,
                stack:     &[],
                constants: &self.constants,
            });
        }
    }


    fn run_instruction(&'gc mut self, ctx: &'gc Mutation<'gc>) -> RuntimeResult<bool> {

        if DEBUG_TRACE_EXECUTION {

            let chunk = self.get_chunk();
            let chunk = chunk.borrow();

            let data  = DisassembleData {
                name:      "",
                lines:     &chunk.lines,
                stack:     &self.stack,
                constants: &self.constants,
            };

            chunk.code[*self.ip].disassemble(&data, *self.ip);
            print_stack(&data);
                println!();
        }

        match self.get_instruction() {
            OpCode::GetConstant { index }            => self.op_constant    (index),

            OpCode::DefGlobal   { name_idx }         => self.op_def_global  (name_idx),
            OpCode::GetGlobal   { name_idx }         => self.op_get_global  (name_idx)?,
            OpCode::SetGlobal   { name_idx }         => self.op_set_global  (name_idx)?,

            OpCode::GetProperty { name_idx }         => self.op_get_property(name_idx, ctx)?,
            OpCode::SetProperty { name_idx }         => self.op_set_property(name_idx, ctx),

            OpCode::GetLocal    { offset }           => self.op_get_local   (offset),
            OpCode::SetLocal    { offset }           => self.op_set_local   (offset),

            OpCode::GetUpvalue  { index }            => self.op_get_upvalue (index),
            OpCode::SetUpvalue  { index }            => self.op_set_upvalue (index, ctx),
            OpCode::PushUpvalue { index }            => self.op_push_upvalue(index, ctx),

            OpCode::JumpIfFalse { offset }           => self.op_jump_if     (JumpType::IfFalsey, offset),
            OpCode::JumpIfTrue  { offset }           => self.op_jump_if     (JumpType::IfTruthy, offset),
            OpCode::Jump        { offset }           => self.op_jump        (offset),

            OpCode::Loop        { offset }           => self.op_loop        (offset),

            OpCode::Call        { arg_count }        => self.op_call        (arg_count, ctx)?,
            OpCode::Class       { name_idx }         => self.op_class       (name_idx,  ctx),
            OpCode::Closure     { func }             => self.op_closure     (func,      ctx),

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
        let chunk = self.get_chunk();
        let chunk = chunk.borrow();

        chunk.code[ip]
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

    fn op_get_property(&mut self, index: ConstIndex, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {
        let name     = self.get_constant_as_str(index);
        let val      = self.pop_stack();

        let obj = val.to_obj().unwrap_or_else(|| {
            panic!("Value not an object: '{val}'")
        });

        let instance = obj.to_instance().unwrap_or_else(|| {
            panic!("Object not an instance: '{}'", obj);
        });

        let instance = instance.borrow_mut(ctx);

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

        let obj = obj.to_obj().unwrap_or_else(|| {
            panic!("Value not an object: '{obj}'");
        });

        let obj = obj.to_instance().unwrap_or_else(|| {
            panic!("Object not an instance: '{obj}'");
        });

        let mut instance = obj.borrow_mut(ctx);

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

    fn op_get_upvalue(&mut self, index: UpvalueIndex) {
        let closure = self.call_frame().borrow().closure;
        let upvalue = closure.borrow().closed_vals[*index];

        self.push_stack(upvalue);
    }

    fn op_set_upvalue(&self, index: UpvalueIndex, ctx: &'gc Mutation<'gc>) {
        let value = self.peek_stack(0);

        let closure = self.call_frame().borrow().closure;
        let upvalue = closure.borrow().closed_vals[*index];

        let obj   = upvalue.to_obj_mut().expect("expect mutable object");
        let obj   = obj.to_value().unwrap_or_else(|| {
            panic!("expect object of type Value: '{}'", obj)
        });

        let mut obj = obj.borrow_mut(ctx);

        obj.value = value;
    }

    fn op_push_upvalue(&mut self, index: StackOffset, ctx: &'gc Mutation<'gc>) {
        let index = StackIndex(self.from_stack_top(*index));

        let val = self.stack_swap(index, Value::Nil);
        let obj = ObjPtr::new_value(val, ctx);
        let val = Value::Obj(obj);

        let closure = self.call_frame().borrow().closure;
        let mut closure = closure.borrow_mut(ctx);

        closure.closed_vals.push(val);

        self.stack_swap(index, val);
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
        let obj  = ObjPtr::new_class(name, ctx);

        self.push_stack(Value::Obj(obj));
    }

    fn op_closure(&mut self, func: Gc<'gc, ObjFunction<'gc>>, ctx: &Mutation<'gc>) {
        self.pop_stack();

        let closed_vals = self.stack.iter()
            .filter_map(|v| match v {
                Value::Closed(_) => Some(v.clone()),
                _                => None,
            })
            .collect()
        ;

        let closure = ObjPtr::new_closure(func, func.arity, closed_vals, ctx);

        self.push_stack(Value::Obj(closure));
    }

    fn op_close_var(&mut self, index: StackIndex, ctx: &Mutation<'gc>) {
        let val = self.stack_swap(index, Value::Nil);
        let obj = ObjPtr::new_value(val, ctx);

        let val = Value::Obj(obj);

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

    fn op_add(&mut self, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {

        let b = self.pop_stack();
        let a = self.pop_stack();

        if let (Some(a), Some(b)) = (a.as_str(), b.as_str()) {
            let result = format!("{}{}", a.string, b.string);

            let obj = ObjPtr::new_string(result, ctx);

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

        let out = format!("{}", val);
        if self.capture_out {
            self.out.push(out.clone());
        }

        println!("{}", out)
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

        let chunk = self.get_chunk();
        let chunk = chunk.borrow();

        RuntimeError {
            msg:         msg,
            stack_trace: self.stack_trace(),
            line:        chunk.lines[*self.ip -1],
        }
    }

    fn get_constant_as_str(&self, index: ConstIndex) -> String {

        self.constants[*index]
            .as_str()
            .unwrap_or_else(|| {
                panic!("Expect constant value to be of type ObjString: {}", self.constants[*index])
            })
            .string
            .to_owned()
    }

    fn get_chunk(&self) -> GcRefLock<'gc, Chunk<'gc>> {
        let frame     = self.call_frame();
        let frame_ref = frame.borrow();

        let cls = frame_ref.closure;
        let cls = cls.borrow();

        cls.function.chunk
    }

    fn call_frame(&self) -> Gc<'gc, RefLock<CallFrame<'gc>>> {
        *self.call_stack.last().expect("Call stack must not be empty")
    }

    fn call_frame_mut(&mut self, ctx: &'gc Mutation<'gc>) -> RefMut<'gc, CallFrame<'gc>> {
        let frame = self.call_frame();
        frame.borrow_mut(ctx)
    }

    fn get_local(&self, index: StackIndex) -> Value<'gc> {
        self.stack[*index]
    }

    fn call_value(&mut self, value: Value<'gc>, arg_count: usize, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {

        let obj = value.to_obj().ok_or_else(|| {
            self.runtime_error(
                format!("Value of type '{}' is not callable", value.display_type())
            )
        })?;

        match obj {
            ObjPtr::Obj   (Object   ::NativeFn(func))  => self.call_native(       arg_count, func.func.clone()),
            ObjPtr::ObjMut(ObjectMut::Closure (cls))   => self.call       (cls,   arg_count, ctx)?,
            ObjPtr::ObjMut(ObjectMut::Class   (class)) => self.call_class (class, arg_count, ctx)?,
            _                                          => Err(self.runtime_error(
                format!("Object of type '{:?}' is not callable", obj)
            ))?

        };

        Ok(())
    }

    fn call(&mut self, closure: GcRefLock<'gc, ObjClosure<'gc>>, arg_count: usize, ctx: &'gc Mutation<'gc>) -> RuntimeResult<()> {

        let func       = closure.borrow();
        let func       = func.function;
        let func_arity = func.arity;

        if func_arity != arg_count {
            Err(self.runtime_error(format!("Expected {} arguments but got {}", func_arity, arg_count)))?
        }

        if self.call_stack.len() > STACK_FRAMES_MAX {
            Err(self.runtime_error("Stack overflow".to_owned()))?
        }

        let frame = CallFrame {
            stack_len:    StackIndex   (self.stack.len() - func_arity -1),
            ret_ip:       self.ip,
            closure,
            arity:        func_arity,
        };


        self.call_stack.push(Gc::new(ctx, RefLock::new(frame)));
        *self.ip = 0;

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


    fn call_class(&mut self, class: Gc<'gc, RefLock<ObjClass<'gc>>>, _arg_count: usize, ctx: &Mutation<'gc>) -> RuntimeResult<()> {


        let obj = ObjPtr::new_instance(class, ctx);

        self.stack.pop();
        self.stack.push(Value::Obj(obj));

        Ok(())
    }

    fn stack_trace(&self) -> String {

        let mut results = String::new();

        let mut prv_ip  = self.ip;

        for frame in self.call_stack.iter().rev() {

            let frame = frame.borrow();

            let func  = frame.closure.borrow();
            let func  = func .function;
            let chunk = func .chunk  .borrow();
            let line  = chunk.lines[*prv_ip];

            writeln!(results, "  [line {}] in {}", line, func.name).unwrap();

            prv_ip = frame.ret_ip;
        }

        results
    }
}



fn concatenate<'gc>(val1: &str, val2: &str, ctx: &'gc Mutation<'gc>) -> Value<'gc> {

    let str = format!("{val1}{val2}");

    let obj = ObjPtr::new_string(str, ctx);

    Value::new_obj(obj)
}

fn def_natives<'gc>(globals: &'gc mut HashMap<String, Value<'gc>>, ctx: &'gc Mutation<'gc>) {

    let mut make_global = |name: &str, func| {

        let obj = ObjPtr::new_native_fn(name.to_owned(), func, ctx);

        globals.insert(name.to_owned(), Value::new_obj(obj))
    };

    make_global("clock", NativeFn(clock_native));
}

fn clock_native<'gc>(_: &[Value<'gc>]) -> Value<'gc> {
    let start = SystemTime::now();

    let time_since = start.duration_since(UNIX_EPOCH).unwrap();

    Value::Number(time_since.as_millis() as f64 / 1000.0)
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::script::{parser, resolver::resolve, scanner, vm::compiler::{compile}};

    use super::*;

    fn source(file: &str) -> String {
        let path = format!("./test_scripts/unit_tests/vm/{file}");
        fs::read_to_string(&path).unwrap()
    }

    fn init(source: String) -> Vm {

        let     tokens = scanner::scan_tokens(&source).unwrap();
        let mut ast    = parser ::parse_ast  (tokens) .unwrap();
        resolve(&mut ast);

        let mut root = ArenaRoot::new(|_ctx| Root::new_test());

        root.mutate_root(|ctx, root| {
            compile(ast, root, ctx).unwrap();
        });

        root.mutate(|_ctx, root| {
            root.dbg_funcs();
        });


        Vm::new(root)

    }

    fn assert_stack_empty(code: String) {
        let mut vm = init(code);

        vm.run().unwrap();

        vm.root.mutate(|_ctx, root| {
            assert!(root.stack.is_empty());
        });

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
    fn test_stack_empty_functions_recursion() {
        assert_stack_empty(source("test_stack_functions_recursion.lox"));
    }

    #[test]
    fn test_stack_empty_functions_nested_2() {
        assert_stack_empty(source("test_stack_functions_nested_2.lox"));
    }

    #[test]
    fn test_recursion_fib() {
        let mut vm = init(source("test_recursion_fib.lox"));

        vm.run().unwrap();

        vm.root.mutate(|_ctx, root| {
            assert!(root.stack.is_empty());

            assert_eq!(&root.out[0],  "1");
            assert_eq!(&root.out[1],  "1");
            assert_eq!(&root.out[2],  "2");
            assert_eq!(&root.out[3],  "3");
            assert_eq!(&root.out[4],  "5");
            assert_eq!(&root.out[5],  "8");
            assert_eq!(&root.out[6], "13");
            assert_eq!(&root.out[7], "21");
        });
    }

    #[test]
    fn test_closure_mutation_1() {
        let mut vm = init(source("test_closure_mutation_1.lox"));

        vm.run().unwrap();

        vm.root.mutate(|_ctx, root| {
            assert!(root.stack.is_empty());

            assert_eq!(&root.out[0],  "closed");
            assert_eq!(&root.out[1],  "1");

            assert_eq!(&root.out[2],  "changed");
            assert_eq!(&root.out[3],  "2");
        });
    }

}
