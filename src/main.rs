mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use std::error::Error;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use rustyline::DefaultEditor;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    let mut evaluator = Evaluator::new(&mut stdout);
    let mut editor = DefaultEditor::new()?;

    loop {
        // Print a prompt
        let command = editor.readline("> ")?;
        editor.add_history_entry(command.as_str())?;

        let mut lexer = Lexer::new(command.as_bytes());
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
        if let Err(error) = result {
            eprintln!("Runtime error: {error:?}");
            continue;
        }
    }
}
