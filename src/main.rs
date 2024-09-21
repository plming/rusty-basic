mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use std::io::Write;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();
    let mut evaluator = Evaluator::new(&mut stdout);

    loop {
        // Print a prompt
        print!("> ");
        std::io::stdout().flush().unwrap();

        // Read a line from the user
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let code = buffer.as_bytes();
        let mut lexer = Lexer::new(code);
        let tokens = match lexer.lex() {
            Ok(tokens) => tokens,
            Err(error) => {
                eprintln!("Lexer error: {error:?}");
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let line = match parser.parse_line() {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Parser error: {error:?}");
                continue;
            }
        };

        let result = evaluator.process_line(line);
        match result {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Runtime error: {error:?}");
                continue;
            }
        };
    }
}
