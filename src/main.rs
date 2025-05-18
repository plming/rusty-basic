use evaluator::Evaluator;
use lexer::lex;
use parser::Parser;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut evaluator = Evaluator::new();

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("RustyBASIC - TinyBASIC written in Rust.");
    println!("Ver {VERSION}");
    println!("Type 'end' to quit program.");

    loop {
        // Print a prompt
        print!("> ");
        stdout().flush()?;

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let tokens = match lex(buffer.as_bytes()) {
            Ok(tokens) => tokens,
            Err(error) => {
                eprintln!("Lexer error: {error:?}");
                continue;
            }
        };

        let mut parser = Parser::new(VecDeque::from(tokens));
        let line = match parser.parse_line() {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Parser error: {error:?}");
                continue;
            }
        };

        let result = evaluator.process_line(line);
        if let Err(error) = result {
            eprintln!("Runtime error: {error:?}");
            continue;
        }
    }
}
