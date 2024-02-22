use std::fs::{File, read_to_string};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Interpreter {
    file: File,
    tokens: Vec<char>,
    pointer: usize,
}

impl Interpreter {
    pub(crate) fn init() -> Self {
        println!("Enter the file path: ");
        let file_path = io::stdin().lock().lines().next().unwrap().expect("Failed to read in file name!");
        let last_three_chars: String = file_path.chars().rev().take(3).collect::<String>().chars().rev().collect();
        if last_three_chars != ".bf" {
            panic!("File must end with .bf extension!");
        }
        Interpreter::default(Path::new(&file_path))
    }
    fn default(file_path: &Path) -> Self {
        Interpreter {
            file: File::open(file_path).unwrap(),
            tokens: vec!['>', '<', '+', '-', '.', ',', '[', ']'],
            pointer: 0,
        }
    }

    pub fn process_file(&mut self) {
        let reader = BufReader::new(&self.file);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            for token in line.chars() {
                self.process_token(&token.to_string());
            }
        }
    }

    fn process_token(&self, token: &str) {

        println!("{}", token);
    }
}