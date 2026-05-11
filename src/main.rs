#![allow(dead_code)]
#![allow(unused_imports)]

use std::{path::Path};

use crate::{script::RunError, shapes::Shape};

mod script;
mod macros;
mod utils;
mod shapes;
mod app;


pub fn main() {

    let shapes = lox().unwrap();

    app::main(shapes).unwrap();
}


fn lox() -> Result<Vec<Shape>, RunError> {
    let path = Path::new("./test_scripts/test.lox");
    script::run_file(path)
}
