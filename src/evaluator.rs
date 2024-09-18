use crate::ast::{self, Program};

const NUM_VARIABLES: usize = 26;

pub struct Evaluator {
    variables: [i16; NUM_VARIABLES],
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            variables: [0; NUM_VARIABLES],
        }
    }

    pub fn run(&mut self, program: Program) {
        let statements = program.statements();
        for statement in statements {
            match statement {
                ast::Statement::Print { expression_list } => {
                    for element in expression_list {
                        match element {
                            ast::ExpressionListElement::StringLiteral(string_literal) => {
                                println!("{}", String::from_utf8_lossy(string_literal.value()));
                            }
                            ast::ExpressionListElement::Expression(expression) => {
                                let result = self.evaluate_expression(expression);
                                println!("{}", result);
                            }
                        }
                    }
                }
                ast::Statement::Let {
                    variable,
                    expression,
                } => {
                    let value = self.evaluate_expression(expression);
                    self.store_variable(variable.identifier(), value);
                }
                _ => todo!("{statement:?}"),
            }
        }
    }

    fn evaluate_expression(&self, expression: &ast::Expression) -> i16 {
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

    fn evaluate_term(&self, term: &ast::Term) -> i16 {
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

    fn evaluate_factor(&self, factor: &ast::Factor) -> i16 {
        match factor {
            ast::Factor::Variable(variable) => {
                let identifier = variable.identifier();
                self.load_variable(identifier)
            }
            ast::Factor::NumberLiteral(number) => number.value(),
            ast::Factor::Expression(expression) => self.evaluate_expression(expression),
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
