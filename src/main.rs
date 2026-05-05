#![allow(dead_code)]
#![allow(unused_imports)]

use std::{path::Path};

mod script;
mod macros;
mod utils;
mod rendering;
mod shapes;


pub fn main() {

    // lox();
    rendering::main().unwrap()

}


fn lox() {
    let path = Path::new("./test_scripts/test.lox");
    script::run_file(path);
}
