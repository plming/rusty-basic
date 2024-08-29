mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

fn main() {
    let code = b"PRINT 2+3";
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    let evaluator = Evaluator::new(program);
    evaluator.run();
}
