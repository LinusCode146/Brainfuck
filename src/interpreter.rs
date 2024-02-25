use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Interpreter {
    pub(crate) file: File,
    pub(crate) pointer: i64,
    pub(crate) tape: HashMap<i64, i64>,
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
            pointer: 0,
            tape: HashMap::new(),
        }
    }

    pub fn process_file(&mut self) -> String {
        let reader = BufReader::new(&self.file);
        let mut tokens: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            for token in line.chars() {
                tokens.push(token.to_string());
            }
        }

        self.process_tokens(&tokens)
    }

    fn process_tokens(&mut self, tokens: &[String]) -> String {
        let mut loop_stack: Vec<usize> = Vec::new();
        let mut output: String = String::new();

        let mut i = 0; // Use a separate index variable to control loop iteration

        while i < tokens.len() {
            let token = &tokens[i];
            match token.as_str() {
                ">" => self.pointer += 1,
                "<" => self.pointer -= 1,
                "+" => {
                    let value = self.tape.entry(self.pointer).or_insert(0);
                    *value += 1;
                }
                "-" => {
                    let value = self.tape.entry(self.pointer).or_insert(0);
                    *value -= 1;
                }
                "." => {
                    let value = self.tape.entry(self.pointer).or_insert(0);
                    output.push(char::from(*value as u8));
                }
                "," => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read a line from stdin");
                    let number: i64 = input.trim().parse().expect("Failed to parse number from stdin");
                    self.tape.insert(self.pointer, number);
                }
                "[" => {
                    loop_stack.push(i);
                }
                "]" => {
                    if let Some(open_bracket_position) = loop_stack.pop() {
                        if *self.tape.get(&self.pointer).unwrap_or(&0) != 0 {
                            i = open_bracket_position;
                            continue;
                        }
                    } else {
                        panic!("Unmatched ']' bracket");
                    }
                }
                _ => { /* Ignore token as a comment */}
            }
            i += 1;
        }

        output
    }
}