use crate::ast;
use crate::ast::{Expression, ExpressionListElement, Factor, Line, Statement, Term};
use std::collections::HashMap;

const NUM_VARIABLES: usize = 26;

#[derive(Debug)]
pub enum Error {
    LineNumberOutOfRange,
    UnknownLineNumber,
}

pub struct Evaluator {
    lines: Vec<Line>,
    label_to_index: HashMap<u8, usize>,
    /// Points to lines
    program_counter: usize,
    variables: [i16; NUM_VARIABLES],
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            label_to_index: HashMap::new(),
            program_counter: 0,
            variables: [0; NUM_VARIABLES],
        }
    }

    pub fn process_line(&mut self, line: Line) -> Result<(), Error> {
        match line.number().is_some() {
            true => self.load_line(line),
            false => self.run_direct(line.statement())?,
        }

        Ok(())
    }

    fn load_line(&mut self, line: Line) {
        debug_assert!(line.number().is_some());
        self.label_to_index
            .insert(line.number().unwrap(), self.lines.len());
        self.lines.push(line);
    }

    fn jump(&mut self, line_number: u8) -> Result<(), Error> {
        match self.label_to_index.get(&line_number) {
            Some(&index) => self.program_counter = index,
            None => Err(Error::UnknownLineNumber)?,
        };
        Ok(())
    }

    fn run_direct(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Print { expression_list } => {
                for element in expression_list {
                    match element {
                        ExpressionListElement::StringLiteral(string_literal) => {
                            println!("{}", String::from_utf8_lossy(string_literal.value()));
                        }
                        ExpressionListElement::Expression(expression) => {
                            let result = self.evaluate_expression(expression);
                            println!("{}", result);
                        }
                    }
                }
            }

            Statement::Let {
                variable,
                expression,
            } => {
                let value = self.evaluate_expression(expression);
                self.store_variable(variable.identifier(), value);
            }
            Statement::Goto { expression } => {
                let line_number = match u8::try_from(self.evaluate_expression(expression)) {
                    Ok(line_number) => line_number,
                    Err(_) => Err(Error::LineNumberOutOfRange)?,
                };

                self.jump(line_number)?;
            }
            Statement::Run => {
                self.program_counter = 0;
                self.run_indirect()?;
            }
            Statement::End => {
                self.program_counter = self.lines.len();
            }
            _ => todo!("{:?}", statement),
        }

        Ok(())
    }

    fn run_indirect(&mut self) -> Result<(), Error> {
        while self.program_counter < self.lines.len() {
            let line = &self.lines[self.program_counter];
            self.run_direct(&line.statement().clone())?;
            self.program_counter += 1;
        }

        Ok(())
    }

    fn evaluate_expression(&self, expression: &Expression) -> i16 {
        let terms = expression.terms();
        let operators = expression.operators();

        let mut result = 0;

        for i in 0..terms.len() {
            let operator = &operators[i];
            let term = &terms[i];

            let value = self.evaluate_term(term);

            match operator {
                ast::AdditiveOperator::Addition => result += value,
                ast::AdditiveOperator::Subtraction => result -= value,
            }
        }

        result
    }

    fn evaluate_term(&self, term: &Term) -> i16 {
        let factors = term.factors();
        let operators = term.operators();

        let mut result = self.evaluate_factor(&factors[0]);

        for i in 1..factors.len() {
            let operator = &operators[i - 1];
            let factor = &factors[i];

            let value = self.evaluate_factor(factor);

            match operator {
                ast::MultiplicativeOperator::Multiplication => result *= value,
                ast::MultiplicativeOperator::Division => result /= value,
            }
        }

        result
    }

    fn evaluate_factor(&self, factor: &Factor) -> i16 {
        match factor {
            Factor::Variable(variable) => {
                let identifier = variable.identifier();
                self.load_variable(identifier)
            }
            Factor::NumberLiteral(number) => number.value(),
            Factor::Expression(expression) => self.evaluate_expression(expression),
        }
    }

    fn load_variable(&self, identifier: u8) -> i16 {
        let offset = (identifier - b'A') as usize;
        self.variables[offset]
    }

    fn store_variable(&mut self, identifier: u8, value: i16) {
        let offset = (identifier - b'A') as usize;
        self.variables[offset] = value;
    }
}
