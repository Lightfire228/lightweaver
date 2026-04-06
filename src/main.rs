#![allow(dead_code)]

// use script::{ast::display::AstDisplayOpts, parser::Parser, scanner::Scanner};
use std::{fs::File, io::BufWriter, path::Path};

mod script;
mod macros;
mod utils;


pub fn main() {

    lox();

}


fn lox() {
    let path = Path::new("./test_scripts/test.lox");
    script::run_file(path);
}
