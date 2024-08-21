use crate::lexer::Lexer;

pub struct Evaluator<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Evaluator<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Evaluator { lexer }
    }

    pub fn run(&mut self) {
        todo!();
    }
}
