use std::io::Write;

use crate::ast;
use crate::ast::{Expression, ExpressionListElement, Factor, Line, Statement, Term};

const STORAGE_SIZE: usize = 256;
const NUM_VARIABLES: usize = 26;

#[derive(Debug)]
pub enum Error {
    LineNumberOutOfRange,
    UnknownLineNumber,
}

pub struct Evaluator<'a> {
    storage: Vec<Option<Line>>,
    stack: Vec<usize>,
    program_counter: usize,
    variables: [i16; NUM_VARIABLES],
    output: &'a mut dyn Write,
}

impl<'a> Evaluator<'a> {
    pub fn new(output: &'a mut dyn Write) -> Self {
        Self {
            storage: vec![None; STORAGE_SIZE],
            stack: Vec::new(),
            program_counter: 0,
            variables: [0; NUM_VARIABLES],
            output,
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

        let label = line.number().unwrap();
        self.storage[label as usize] = Some(line);
    }

    fn jump(&mut self, line_number: u8) -> Result<(), Error> {
        match self.storage.get(line_number as usize) {
            Some(_) => {
                self.program_counter = line_number as usize;
            }
            None => Err(Error::UnknownLineNumber)?,
        };

        Ok(())
    }

    fn run_direct(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Print { expression_list } => {
                expression_list.iter().for_each(|element| {
                    writeln!(self.output, "{element}").unwrap();
                });
            }
            Statement::If {
                left,
                operator,
                right,
                then,
            } => {
                let left_value = self.evaluate_expression(left);
                let right_value = self.evaluate_expression(right);

                let condition = match operator {
                    ast::RelationalOperator::LessThan => left_value < right_value,
                    ast::RelationalOperator::LessThanOrEqual => left_value <= right_value,
                    ast::RelationalOperator::GreaterThan => left_value > right_value,
                    ast::RelationalOperator::GreaterThanOrEqual => left_value >= right_value,
                    ast::RelationalOperator::Equal => left_value == right_value,
                    ast::RelationalOperator::NotEqual => left_value != right_value,
                };

                if condition {
                    self.run_direct(then)?;
                }
            }
            Statement::Goto { expression } => {
                let line_number = match u8::try_from(self.evaluate_expression(expression)) {
                    Ok(line_number) => line_number,
                    Err(_) => Err(Error::LineNumberOutOfRange)?,
                };

                self.jump(line_number)?;
            }
            Statement::Input { variable_list: _ } => {
                todo!("implement input statement");
            }
            Statement::Let {
                variable,
                expression,
            } => {
                let value = self.evaluate_expression(expression);
                self.store_variable(variable.identifier(), value);
            }
            Statement::GoSub { expression } => {
                let line_number = match u8::try_from(self.evaluate_expression(expression)) {
                    Ok(line_number) => line_number,
                    Err(_) => Err(Error::LineNumberOutOfRange)?,
                };

                self.stack.push(self.program_counter);
                self.jump(line_number)?;
            }
            Statement::Return => match self.stack.pop() {
                Some(line_number) => self.program_counter = line_number,
                None => {
                    self.program_counter = self.storage.len();
                }
            },
            Statement::Clear => {
                self.storage = vec![None; STORAGE_SIZE];
            }
            Statement::List => {
                self.storage.iter().for_each(|line| {
                    if let Some(line) = line {
                        writeln!(self.output, "{}", line).unwrap();
                    }
                });
            }
            Statement::Run => {
                self.program_counter = 0;
                self.run_indirect()?;
            }
            Statement::End => {
                self.program_counter = self.storage.len();
            }
        }

        Ok(())
    }

    fn run_indirect(&mut self) -> Result<(), Error> {
        while self.program_counter < self.storage.len() {
            let line = match &self.storage[self.program_counter] {
                Some(line) => line,
                None => {
                    self.program_counter += 1;
                    continue;
                }
            };

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
