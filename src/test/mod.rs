use std::collections::HashMap;
use std::fs::File;
#[cfg(test)]

use super::*;

#[test]
fn test_valid_execution_1() {
    let mut interpreter = Interpreter {
        file: File::open("src/test/resources/HelloWorld.bf").unwrap(),
        pointer: 0,
        tape: HashMap::new(),
    };

    assert_eq!(interpreter.process_file(), "Hello, World!");
}


#[test]
fn test_valid_execution_2() {
    let mut interpreter = Interpreter {
        file: File::open("src/test/resources/666.bf").unwrap(),
        pointer: 0,
        tape: HashMap::new(),
    };

    assert_eq!(interpreter.process_file(), "666\n");
}

