use crate::ast::{self, Program};

const NUM_VARIABLES: usize = 26;

pub struct Evaluator {
    program: Program,
    variables: [i16; NUM_VARIABLES],
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            variables: [0; NUM_VARIABLES],
        }
    }

    pub fn run(&self) {
        for statement in self.program.statements() {
            match statement {
                ast::Statement::Print { expression_list } => {
                    for element in expression_list {
                        match element {
                            ast::ExpressionListElement::String { value } => {
                                println!("{}", String::from_utf8_lossy(value));
                            }
                            ast::ExpressionListElement::Expression { expression } => {
                                let result = self.evaluate_expression(expression);
                                println!("{}", result);
                            }
                        }
                    }
                }
                _ => todo!(),
            }
        }
    }

    fn evaluate_expression(&self, expression: &ast::Expression) -> i16 {
        let mut result = 0;

        for i in 0..expression.terms.len() {
            let operator = &expression.operators[i];
            let term = &expression.terms[i];

            let value = self.evaluate_term(term);

            match operator {
                ast::TermOperator::Add => result += value,
                ast::TermOperator::Subtract => result -= value,
            }
        }

        result
    }

    fn evaluate_term(&self, term: &ast::Term) -> i16 {
        let mut result = self.evaluate_factor(&term.factors[0]);

        for i in 1..term.factors.len() {
            let operator = &term.operators[i - 1];
            let factor = &term.factors[i];

            let value = self.evaluate_factor(factor);

            match operator {
                ast::FactorOperator::Multiply => result *= value,
                ast::FactorOperator::Divide => result /= value,
            }
        }

        result
    }

    fn evaluate_factor(&self, factor: &ast::Factor) -> i16 {
        match factor {
            ast::Factor::Variable(variable) => {
                let identifier = variable.identifier();
                self.load_variable(identifier)
            }
            ast::Factor::Number(number) => *number,
            ast::Factor::Expression(expression) => self.evaluate_expression(&expression),
        }
    }

    fn load_variable(&self, identifier: u8) -> i16 {
        let offset = (identifier - b'A') as usize;
        self.variables[offset]
    }
}
