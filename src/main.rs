

use interpreter::Interpreter;

pub mod interpreter;
pub mod test;

fn main() {
    let mut interpreter = Interpreter::init();
    let output = interpreter.process_file();
    println!("{}", output);
}
