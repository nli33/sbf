use std::env;
use std::io::{stdin, stdout};
use std::process;

mod bf;

fn main() {
    let mut args = env::args();
    
    let program = args.next().unwrap();
    let file = match args.next() {
        Some(f) => f,
        None => {
            eprintln!("usage: {} <program.bf>", program);
            process::exit(1);
        }
    };

    // one arg only
    if args.next().is_some() {
        eprintln!("usage: {} <program.bf>", program);
        process::exit(1);
    }

    let mut interpreter = match bf::Interpreter::new(
        &file,
        stdin(),
        stdout(),
    ) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error loading program: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = interpreter.execute() {
        eprintln!("runtime error: {}", e);
        process::exit(1);
    }
}