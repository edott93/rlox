use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::{env, io};

#[derive(Debug)]
pub struct Interpreter {
    pub args: Vec<String>,
    had_error: bool,
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            args: env::args().collect(),
            had_error: false,
        }
    }
    pub fn run_file(&self) {
        let mut f = File::open(&self.args[0]).expect("Unable to open file {:?}");
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)
            .expect("Unable to read file to string");
        self.run(buffer);
        if self.had_error == true {
            std::process::exit(65)
        }
    }
    pub fn run_prompt(&mut self) {
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!("> ");
            io::stdout().flush().unwrap();
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();
            if buffer.is_empty() {
                break;
            }
            self.run(buffer.to_string());
            self.had_error = true;
        }
    }
    fn run(&self, _buffer: String) {
        todo!();
    }
    fn error(&mut self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }
    pub fn report(&mut self, line: i32, location: String, message: String) {
        eprintln!("[line {} ] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
