use std::io;
use std::io::BufRead;
use interpreter::Interpreter;

pub mod interpreter;

fn main() {
    let mut interpreter = Interpreter::init();
    interpreter.process_file();
}
