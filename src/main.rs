mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

use evaluator::Evaluator;
use lexer::lex;
use parser::Parser;

fn main() {
    let mut evaluator = Evaluator::new();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("RustyBASIC - TinyBASIC written in Rust.");
    println!("Ver {VERSION}");
    println!("Type 'end' to quit program.");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
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
