use crate::scanner::Scanner;
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

    pub fn run_file(&mut self) {
        let mut f = File::open(&self.args[1]).expect("Unable to open file");
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)
            .expect("Unable to read file to string");
        self.run(buffer);
        if self.had_error {
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
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        // For now, just print the tokens
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[allow(dead_code)]
    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    pub fn report(&mut self, line: usize, location: String, message: String) {
        eprintln!("[line {} ] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    // Method for testing that doesn't use args
    pub fn new_with_args(args: Vec<String>) -> Self {
        Interpreter {
            args,
            had_error: false,
        }
    }

    // Method to check if interpreter has errors (useful for testing)
    pub fn has_error(&self) -> bool {
        self.had_error
    }

    // Method to run source and return if it had errors (useful for testing)
    pub fn run_source(&mut self, source: String) -> bool {
        self.had_error = false;
        self.run(source);
        self.had_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_creation() {
        let interpreter = Interpreter::new();
        assert!(!interpreter.args.is_empty());
        assert!(!interpreter.has_error());
    }

    #[test]
    fn test_interpreter_with_custom_args() {
        let args = vec!["rlox".to_string(), "test.lox".to_string()];
        let interpreter = Interpreter::new_with_args(args.clone());
        assert_eq!(interpreter.args, args);
        assert!(!interpreter.has_error());
    }

    #[test]
    fn test_error_reporting() {
        let mut interpreter = Interpreter::new();
        assert!(!interpreter.has_error());
        
        interpreter.report(1, "".to_string(), "Test error".to_string());
        assert!(interpreter.has_error());
    }

    #[test]
    fn test_run_simple_source() {
        let mut interpreter = Interpreter::new();
        let had_error = interpreter.run_source("var x = 42;".to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_run_empty_source() {
        let mut interpreter = Interpreter::new();
        let had_error = interpreter.run_source("".to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_run_source_with_comments() {
        let mut interpreter = Interpreter::new();
        let had_error = interpreter.run_source("// This is a comment\nvar x;".to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_run_source_with_string_literals() {
        let mut interpreter = Interpreter::new();
        let had_error = interpreter.run_source("print \"Hello, World!\";".to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_run_source_with_numbers() {
        let mut interpreter = Interpreter::new();
        let had_error = interpreter.run_source("var x = 123.456;".to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_run_source_with_keywords() {
        let mut interpreter = Interpreter::new();
        let source = "if (true) { print \"yes\"; } else { print \"no\"; }";
        let had_error = interpreter.run_source(source.to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_run_source_with_operators() {
        let mut interpreter = Interpreter::new();
        let source = "var result = (1 + 2) * 3 != 4;";
        let had_error = interpreter.run_source(source.to_string());
        assert!(!had_error);
    }

    #[test]
    fn test_multiple_runs_reset_error_state() {
        let mut interpreter = Interpreter::new();
        
        // First run should succeed
        let had_error1 = interpreter.run_source("var x;".to_string());
        assert!(!had_error1);
        
        // Manually set error state
        interpreter.report(1, "".to_string(), "Test error".to_string());
        assert!(interpreter.has_error());
        
        // Second run should reset error state
        let had_error2 = interpreter.run_source("var y;".to_string());
        assert!(!had_error2);
    }

    #[test]
    fn test_complex_program() {
        let mut interpreter = Interpreter::new();
        let source = r#"
            // Variable declarations
            var a = 10;
            var b = 20.5;
            var message = "Hello, World!";
            
            // Conditional logic
            if (a < b) {
                print "a is less than b";
            } else {
                print "a is greater than or equal to b";
            }
            
            // Loop
            for (var i = 0; i < 5; i = i + 1) {
                print i;
            }
        "#;
        
        let had_error = interpreter.run_source(source.to_string());
        assert!(!had_error);
    }
}
