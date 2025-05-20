use std::io::stdin;
use std::process::exit;

use crate::ast::{
    AdditiveOperator, Expression, ExpressionListElement, Factor, Line, MultiplicativeOperator,
    RelationalOperator, Statement, Term,
};

const STORAGE_SIZE: usize = 256;
const NUM_VARIABLES: usize = 26;

#[derive(Debug)]
pub enum Error {
    LineNumberOutOfRange,
    UnknownLineNumber,
    WrongUserInput,
    CannotParseNumber,
}

pub struct Evaluator {
    storage: [Option<Line>; STORAGE_SIZE],
    stack: Vec<usize>,
    program_counter: usize,
    variables: [i16; NUM_VARIABLES],
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            storage: [const { None }; STORAGE_SIZE],
            stack: Vec::new(),
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
                for element in expression_list {
                    match element {
                        ExpressionListElement::Expression(expression) => {
                            let value = self.evaluate_expression(expression);
                            print!("{value} ");
                        }
                        ExpressionListElement::StringLiteral(string_literal) => {
                            let literal = String::from_utf8_lossy(string_literal.value());
                            print!("{literal} ",);
                        }
                    }
                }
                println!();
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
                    RelationalOperator::LessThan => left_value < right_value,
                    RelationalOperator::LessThanOrEqual => left_value <= right_value,
                    RelationalOperator::GreaterThan => left_value > right_value,
                    RelationalOperator::GreaterThanOrEqual => left_value >= right_value,
                    RelationalOperator::Equal => left_value == right_value,
                    RelationalOperator::NotEqual => left_value != right_value,
                };

                if condition {
                    self.run_direct(then)?;
                }
            }
            Statement::Goto { expression } => {
                let line_number = Self::to_line_number(self.evaluate_expression(expression))?;

                self.jump(line_number)?;
            }
            Statement::Input { variable_list } => {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer).unwrap();

                let mut nums = Vec::new();
                for num in buffer.trim().split(" ") {
                    let num = num.parse::<i16>().map_err(|_| Error::CannotParseNumber)?;
                    nums.push(num);
                }

                if nums.len() != variable_list.len() {
                    Err(Error::WrongUserInput)?;
                }

                for i in 0..nums.len() {
                    self.store_variable(variable_list[i].identifier(), nums[i]);
                }
            }
            Statement::Let {
                variable,
                expression,
            } => {
                let value = self.evaluate_expression(expression);
                self.store_variable(variable.identifier(), value);
            }
            Statement::GoSub { expression } => {
                let line_number = Self::to_line_number(self.evaluate_expression(expression))?;

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
                self.storage = [const { None }; STORAGE_SIZE];
            }
            Statement::List => {
                self.storage.iter().for_each(|line| {
                    if let Some(line) = line {
                        println!("{line}");
                    }
                });
            }
            Statement::Run => {
                self.program_counter = 0;
                self.run_indirect()?;
            }
            Statement::End => {
                exit(0);
            }
        }

        Ok(())
    }

    fn to_line_number(value: i16) -> Result<u8, Error> {
        match u8::try_from(value) {
            Ok(line_number) => Ok(line_number),
            Err(_) => Err(Error::LineNumberOutOfRange)?,
        }
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
        let term = expression.term();
        let mut result = self.evaluate_term(term);

        if let Some(AdditiveOperator::Subtraction) = expression.unary_operator() {
            result = -result;
        }

        for (operator, term) in expression.others() {
            let value = self.evaluate_term(term);

            match operator {
                AdditiveOperator::Addition => result += value,
                AdditiveOperator::Subtraction => result -= value,
            }
        }

        result
    }

    fn evaluate_term(&self, term: &Term) -> i16 {
        let factor = term.factor();
        let mut result = self.evaluate_factor(factor);

        for (operator, factor) in term.operations() {
            let value = self.evaluate_factor(factor);

            match operator {
                MultiplicativeOperator::Multiplication => result *= value,
                MultiplicativeOperator::Division => result /= value,
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
