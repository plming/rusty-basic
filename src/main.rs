mod lexer;

use lexer::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::new(b"PRINT 2+3");

    while let Ok(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}
