pub mod interpreter;
fn main() {
    let mut interpreter: interpreter::Interpreter = interpreter::Interpreter::new();
    if interpreter.args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if interpreter.args.len() == 2 {
        interpreter.run_file();
    } else {
        interpreter.run_prompt();
    }
}
