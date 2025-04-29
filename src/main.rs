use std::collections::VecDeque;

use rustyline::DefaultEditor;

use evaluator::Evaluator;
use lexer::lex;
use parser::Parser;

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let mut evaluator = Evaluator::new(&mut stdout);
    let mut editor = DefaultEditor::new()?;

    loop {
        // Print a prompt
        let command = editor.readline("> ")?;
        editor.add_history_entry(&command)?;

        let tokens = match lex(command.as_bytes()) {
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
