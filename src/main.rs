mod script;
mod shapes;
mod render;

use std::{env, process::exit};

use script::runtime::ScriptRuntime;


fn main() {
    // main_script();

    render::test();

}

#[allow(dead_code)]
fn main_script() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        println!("too many args");
        exit(64);
    }

    let mut runtime = ScriptRuntime::new();

    if args.len() == 1 {
        runtime.run_file(&args[0]);
    }
    else {
        runtime.run_prompt();
    }
    
}
