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
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    let evaluator = Evaluator::new(program);
    evaluator.run();
}
