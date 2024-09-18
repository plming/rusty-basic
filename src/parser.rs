use std::collections::VecDeque;

use crate::ast;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedToken { expected: Token, found: Token },
    VariableNotFound,
    NoMoreToken,
    RelationalOperatorNotFound,
    KeywordNotFound,
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn peek_token(&self) -> Option<Token> {
        self.tokens.front().cloned()
    }

    fn expect(&mut self, expected: Token) -> Result<(), Error> {
        match self.consume_token() {
            Some(token) => {
                if token == expected {
                    Ok(())
                } else {
                    Err(Error::UnexpectedToken {
                        expected,
                        found: token,
                    })
                }
            }
            None => Err(Error::NoMoreToken),
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Error> {
        let mut statements = Vec::new();

        while !self.tokens.is_empty() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(ast::Program::new(statements))
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        let statement = match self.consume_token() {
            Some(Token::Print) => {
                let mut expression_list = Vec::new();

                loop {
                    match self.peek_token() {
                        Some(Token::StringLiteral { value }) => {
                            self.consume_token();
                            let element = ast::ExpressionListElement::StringLiteral(
                                ast::StringLiteral::new(value),
                            );
                            expression_list.push(element);
                        }
                        _ => {
                            let expression = self.parse_expression()?;
                            let element = ast::ExpressionListElement::Expression(expression);
                            expression_list.push(element);
                        }
                    }

                    if let Some(Token::Comma) = self.peek_token() {
                        self.consume_token();
                    } else {
                        break;
                    }
                }

                ast::Statement::Print { expression_list }
            }
            Some(Token::If) => {
                let left = self.parse_expression()?;
                let operator = match self.consume_token() {
                    Some(Token::Equal) => ast::RelationalOperator::Equal,
                    Some(Token::NotEqual) => ast::RelationalOperator::NotEqual,
                    Some(Token::LessThan) => ast::RelationalOperator::LessThan,
                    Some(Token::LessThanOrEqual) => ast::RelationalOperator::LessThanOrEqual,
                    Some(Token::GreaterThan) => ast::RelationalOperator::GreaterThan,
                    Some(Token::GreaterThanOrEqual) => ast::RelationalOperator::GreaterThanOrEqual,
                    _ => Err(Error::RelationalOperatorNotFound)?,
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
            Some(Token::Goto) => {
                let expression = self.parse_expression()?;
                ast::Statement::Goto { expression }
            }
            Some(Token::Input) => {
                let mut variable_list = Vec::new();
                loop {
                    match self.consume_token() {
                        Some(Token::Variable { identifier }) => {
                            let variable = ast::Variable::new(identifier);
                            variable_list.push(variable);
                        }
                        _ => Err(Error::VariableNotFound)?,
                    }

                    if let Some(Token::Comma) = self.peek_token() {
                        self.consume_token();
                    } else {
                        break;
                    }
                }
                ast::Statement::Input { variable_list }
            }
            Some(Token::Let) => {
                let variable = match self.consume_token() {
                    Some(Token::Variable { identifier }) => ast::Variable::new(identifier),
                    _ => Err(Error::VariableNotFound)?,
                };
                self.expect(Token::Equal)?;
                let expression = self.parse_expression()?;
                ast::Statement::Let {
                    variable,
                    expression,
                }
            }
            Some(Token::GoSub) => {
                let expression = self.parse_expression()?;
                ast::Statement::GoSub { expression }
            }
            Some(Token::Return) => ast::Statement::Return,
            Some(Token::Clear) => ast::Statement::Clear,
            Some(Token::List) => ast::Statement::List,
            Some(Token::Run) => ast::Statement::Run,
            Some(Token::End) => ast::Statement::End,
            None => Err(Error::NoMoreToken)?,
            _ => Err(Error::KeywordNotFound)?,
        };

        Ok(statement)
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Error> {
        let mut expression = ast::Expression {
            terms: Vec::new(),
            operators: Vec::new(),
        };

        match self.peek_token() {
            Some(Token::Plus) => {
                self.consume_token();
                expression.operators.push(ast::AdditiveOperator::Addition);
            }
            Some(Token::Minus) => {
                self.consume_token();
                expression
                    .operators
                    .push(ast::AdditiveOperator::Subtraction);
            }
            _ => {
                // No operator, so we assume it's a positive number
                expression.operators.push(ast::AdditiveOperator::Addition);
            }
        }

        let term = self.parse_term()?;
        expression.terms.push(term);

        loop {
            match self.peek_token() {
                Some(Token::Plus) => {
                    self.consume_token();
                    expression.operators.push(ast::AdditiveOperator::Addition);
                    let term = self.parse_term()?;
                    expression.terms.push(term);
                }
                Some(Token::Minus) => {
                    self.consume_token();
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
            match self.peek_token() {
                Some(Token::Multiply) => {
                    self.consume_token();
                    term.operators
                        .push(ast::MultiplicativeOperator::Multiplication);
                    let factor = self.parse_factor()?;
                    term.factors.push(factor);
                }
                Some(Token::Divide) => {
                    self.consume_token();
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
        match self.peek_token() {
            Some(Token::Variable { identifier }) => {
                self.consume_token();
                let variable = ast::Variable::new(identifier);
                Ok(ast::Factor::Variable(variable))
            }
            Some(Token::NumberLiteral { value }) => {
                self.consume_token();
                Ok(ast::Factor::NumberLiteral(ast::NumberLiteral::new(value)))
            }
            _ => {
                self.expect(Token::OpeningParenthesis)?;
                let expression = Box::new(self.parse_expression()?);
                self.expect(Token::ClosingParenthesis)?;
                Ok(ast::Factor::Expression(expression))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hello_world_returns_ast() {
        let tokens = VecDeque::from([
            Token::Print,
            Token::StringLiteral {
                value: b"Hello, World!".to_vec(),
            },
        ]);
        let mut parser = Parser::new(tokens);
        let expected = ast::Program::new(vec![ast::Statement::Print {
            expression_list: vec![ast::ExpressionListElement::StringLiteral(
                ast::StringLiteral::new(b"Hello, World!".to_vec()),
            )],
        }]);

        let actual = parser.parse_program();

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn parse_print_expression_returns_ast() {
        let tokens = VecDeque::from([
            Token::Print,
            Token::NumberLiteral { value: 2 },
            Token::Plus,
            Token::NumberLiteral { value: 3 },
        ]);
        let expected = ast::Program::new(vec![ast::Statement::Print {
            expression_list: vec![ast::ExpressionListElement::Expression(ast::Expression {
                terms: vec![
                    ast::Term {
                        factors: vec![ast::Factor::NumberLiteral(ast::NumberLiteral::new(2))],
                        operators: vec![],
                    },
                    ast::Term {
                        factors: vec![ast::Factor::NumberLiteral(ast::NumberLiteral::new(3))],
                        operators: vec![],
                    },
                ],
                operators: vec![
                    ast::AdditiveOperator::Addition,
                    ast::AdditiveOperator::Addition,
                ],
            })],
        }]);
        let mut parser = Parser::new(tokens);

        let actual = parser.parse_program();

        assert_eq!(Ok(expected), actual);
    }
}
