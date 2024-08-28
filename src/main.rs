mod ast;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = b"PRINT 2+3";
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    println!("{:#?}", String::from_utf8_lossy(code));
    println!("{:#?}", program);
}
