mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = b"PRINT 2+3";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex().unwrap();
    let mut parser = Parser::new(tokens);

    let program = parser.parse_program().unwrap();

    let evaluator = Evaluator::new(program);
    evaluator.run();
}
