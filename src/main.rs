use std::io::{stdin, stdout};
mod bf;


fn main() {
    match bf::Interpreter::new(
        "test_prog",
        stdin(),
        stdout(),
    ) {
        Ok(mut interpreter) => {
            if let Err(e) = interpreter.execute() {
                eprintln!("{}", e);                
            }
        },
        Err(e) => eprintln!("{}", e),
    };
}
