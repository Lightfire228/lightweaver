#![allow(dead_code)]

use std::{path::Path};

mod script;
mod macros;
mod utils;
mod vulkan;
mod app;


pub fn main() {

    app::main().unwrap();

    // lox();

}


fn lox() {
    let path = Path::new("./test_scripts/test.lox");
    script::run_file(path);
}
