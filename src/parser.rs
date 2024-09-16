use std::collections::VecDeque;

use crate::ast;
use crate::token::Token;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken { expected: Token, found: Token },
    VariableNotFound,
}

pub struct Parser {
    tokens: VecDeque<Token>,
    current_token: Token,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens,
            current_token: Token::EndOfFile,
        }
    }

    fn consume_token(&mut self) -> Result<(), Error> {
        match self.tokens.pop_front() {
            Some(token) => {
                self.current_token = token;
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), Error> {
        if self.current_token == token {
            self.consume_token()?;
            Ok(())
        } else {
            Err(Error::UnexpectedToken {
                expected: token,
                found: self.current_token.clone(),
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Error> {
        let mut program = ast::Program::new();

        self.consume_token()?;
        while self.current_token != Token::EndOfFile {
            let statement = self.parse_statement()?;
            program.add_statement(statement);
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        let statement = match self.current_token {
            Token::Print => {
                self.consume_token()?;

                let mut expression_list = Vec::new();

                loop {
                    match &self.current_token {
                        Token::StringLiteral { value } => {
                            let element = ast::ExpressionListElement::String {
                                value: value.to_vec(),
                            };
                            expression_list.push(element);
                            self.consume_token()?;
                        }
                        _ => {
                            let expression = self.parse_expression()?;
                            let element = ast::ExpressionListElement::Expression { expression };
                            expression_list.push(element);
                        }
                    }

                    if self.current_token == Token::Comma {
                        self.consume_token()?;
                    } else {
                        break;
                    }
                }

                ast::Statement::Print { expression_list }
            }
            Token::If => {
                self.consume_token()?;
                let left = self.parse_expression()?;
                let operator = match self.current_token {
                    Token::Equal => ast::RelationalOperator::Equal,
                    Token::NotEqual => ast::RelationalOperator::NotEqual,
                    Token::LessThan => ast::RelationalOperator::LessThan,
                    Token::LessThanOrEqual => ast::RelationalOperator::LessThanOrEqual,
                    Token::GreaterThan => ast::RelationalOperator::GreaterThan,
                    Token::GreaterThanOrEqual => ast::RelationalOperator::GreaterThanOrEqual,
                    _ => Err(Error::UnexpectedToken {
                        expected: Token::Equal,
                        found: self.current_token.clone(),
                    })?,
                };
                let right = self.parse_expression()?;
                self.expect(Token::Then)?;
                let then = Box::new(self.parse_statement()?);
                ast::Statement::If {
                    left,
                    operator,
                    right,
                    then,
                }
            }
            Token::Goto => {
                self.consume_token()?;
                let expression = self.parse_expression()?;
                ast::Statement::Goto { expression }
            }
            Token::Input => {
                self.consume_token()?;
                let mut variable_list = Vec::new();
                loop {
                    match self.current_token {
                        Token::Variable { identifier } => {
                            let variable = ast::Variable::new(identifier);
                            variable_list.push(variable);
                            self.consume_token()?;
                        }
                        _ => break,
                    }

                    if self.current_token == Token::Comma {
                        self.consume_token()?;
                    } else {
                        break;
                    }
                }
                ast::Statement::Input { variable_list }
            }
            Token::Let => {
                self.consume_token()?;
                let variable = match self.current_token {
                    Token::Variable { identifier } => {
                        let variable = ast::Variable::new(identifier);
                        self.consume_token()?;
                        variable
                    }
                    _ => Err(Error::VariableNotFound)?,
                };
                self.expect(Token::Equal)?;
                let expression = self.parse_expression()?;
                ast::Statement::Let {
                    variable,
                    expression,
                }
            }
            _ => todo!(),
        };

        Ok(statement)
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Error> {
        let mut expression = ast::Expression {
            terms: Vec::new(),
            operators: Vec::new(),
        };

        match self.current_token {
            Token::Plus => {
                self.consume_token()?;
                expression.operators.push(ast::AdditiveOperator::Addition);
            }
            Token::Minus => {
                self.consume_token()?;
                expression
                    .operators
                    .push(ast::AdditiveOperator::Subtraction);
            }
            _ => {
                // No operator, so we assume it's a positive number
                expression.operators.push(ast::AdditiveOperator::Addition);
            }
        }

        expression.terms.push(self.parse_term()?);

        loop {
            match self.current_token {
                Token::Plus => {
                    self.consume_token()?;
                    expression.operators.push(ast::AdditiveOperator::Addition);
                    let term = self.parse_term()?;
                    expression.terms.push(term);
                }
                Token::Minus => {
                    self.consume_token()?;
                    expression
                        .operators
                        .push(ast::AdditiveOperator::Subtraction);
                    let term = self.parse_term()?;
                    expression.terms.push(term);
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<ast::Term, Error> {
        let mut term = ast::Term {
            factors: Vec::new(),
            operators: Vec::new(),
        };

        term.factors.push(self.parse_factor()?);

        loop {
            match self.current_token {
                Token::Multiply => {
                    self.consume_token()?;
                    term.operators
                        .push(ast::MultiplicativeOperator::Multiplication);
                    let factor = self.parse_factor()?;
                    term.factors.push(factor);
                }
                Token::Divide => {
                    self.consume_token()?;
                    term.operators.push(ast::MultiplicativeOperator::Division);
                    let factor = self.parse_factor()?;
                    term.factors.push(factor);
                }
                _ => break,
            }
        }

        Ok(term)
    }

    fn parse_factor(&mut self) -> Result<ast::Factor, Error> {
        match self.current_token {
            Token::Variable { identifier } => {
                self.consume_token()?;
                let variable = ast::Variable::new(identifier);
                Ok(ast::Factor::Variable { variable })
            }
            Token::NumberLiteral { value } => {
                self.consume_token()?;
                Ok(ast::Factor::Number { value })
            }
            _ => {
                self.expect(Token::OpeningParenthesis)?;
                let expression = Box::new(self.parse_expression()?);
                self.expect(Token::ClosingParenthesis)?;
                Ok(ast::Factor::Expression { expression })
            }
        }
    }
}
