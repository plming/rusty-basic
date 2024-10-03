use std::collections::VecDeque;

use crate::ast::{
    AdditiveOperator, Expression, ExpressionListElement, Factor, Line, MultiplicativeOperator,
    NumberLiteral, RelationalOperator, Statement, StringLiteral, Term, Variable,
};
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedToken { expected: Token, found: Token },
    VariableNotFound,
    NoMoreToken,
    RelationalOperatorNotFound,
    KeywordNotFound,
    LineNumberOutOfRange,
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse_line(&mut self) -> Result<Line, Error> {
        let line_number = match self.peek_token() {
            Some(Token::NumberLiteral(value)) => match u8::try_from(value) {
                Ok(line_number) => {
                    self.consume_token();
                    Some(line_number)
                }
                Err(_) => Err(Error::LineNumberOutOfRange)?,
            },
            _ => Option::None,
        };

        let statement = self.parse_statement()?;

        Ok(Line::new(line_number, statement))
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

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        let statement = match self.consume_token() {
            Some(Token::Print) => {
                let mut expression_list = Vec::new();

                loop {
                    match self.peek_token() {
                        Some(Token::StringLiteral { value }) => {
                            self.consume_token();
                            let element =
                                ExpressionListElement::StringLiteral(StringLiteral::new(value));
                            expression_list.push(element);
                        }
                        _ => {
                            let expression = self.parse_expression()?;
                            let element = ExpressionListElement::Expression(expression);
                            expression_list.push(element);
                        }
                    }

                    if let Some(Token::Comma) = self.peek_token() {
                        self.consume_token();
                    } else {
                        break;
                    }
                }

                Statement::Print { expression_list }
            }
            Some(Token::If) => {
                let left = self.parse_expression()?;
                let operator = match self.consume_token() {
                    Some(Token::Equal) => RelationalOperator::Equal,
                    Some(Token::NotEqual) => RelationalOperator::NotEqual,
                    Some(Token::LessThan) => RelationalOperator::LessThan,
                    Some(Token::LessThanOrEqual) => RelationalOperator::LessThanOrEqual,
                    Some(Token::GreaterThan) => RelationalOperator::GreaterThan,
                    Some(Token::GreaterThanOrEqual) => RelationalOperator::GreaterThanOrEqual,
                    _ => Err(Error::RelationalOperatorNotFound)?,
                };
                let right = self.parse_expression()?;
                self.expect(Token::Then)?;
                let then = Box::new(self.parse_statement()?);
                Statement::If {
                    left,
                    operator,
                    right,
                    then,
                }
            }
            Some(Token::Goto) => {
                let expression = self.parse_expression()?;
                Statement::Goto { expression }
            }
            Some(Token::Input) => {
                let mut variable_list = Vec::new();
                loop {
                    match self.consume_token() {
                        Some(Token::Variable { identifier }) => {
                            let variable = Variable::new(identifier);
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
                Statement::Input { variable_list }
            }
            Some(Token::Let) => {
                let variable = match self.consume_token() {
                    Some(Token::Variable { identifier }) => Variable::new(identifier),
                    _ => Err(Error::VariableNotFound)?,
                };
                self.expect(Token::Equal)?;
                let expression = self.parse_expression()?;
                Statement::Let {
                    variable,
                    expression,
                }
            }
            Some(Token::GoSub) => {
                let expression = self.parse_expression()?;
                Statement::GoSub { expression }
            }
            Some(Token::Return) => Statement::Return,
            Some(Token::Clear) => Statement::Clear,
            Some(Token::List) => Statement::List,
            Some(Token::Run) => Statement::Run,
            Some(Token::End) => Statement::End,
            None => Err(Error::NoMoreToken)?,
            _ => Err(Error::KeywordNotFound)?,
        };

        Ok(statement)
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        let unary_operator = match self.peek_token() {
            Some(Token::Plus) => {
                self.consume_token();
                Some(AdditiveOperator::Addition)
            }
            Some(Token::Minus) => {
                self.consume_token();
                Some(AdditiveOperator::Subtraction)
            }
            _ => None,
        };
        let term = self.parse_term()?;
        let mut others = Vec::new();

        loop {
            match self.peek_token() {
                Some(Token::Plus) => {
                    self.consume_token();
                    let term = self.parse_term()?;
                    others.push((AdditiveOperator::Addition, term));
                }
                Some(Token::Minus) => {
                    self.consume_token();
                    let term = self.parse_term()?;
                    others.push((AdditiveOperator::Subtraction, term));
                }
                _ => break,
            }
        }

        Ok(Expression::new(unary_operator, term, others))
    }

    fn parse_term(&mut self) -> Result<Term, Error> {
        let factor = self.parse_factor()?;
        let mut operations = Vec::new();

        loop {
            match self.peek_token() {
                Some(Token::Multiply) => {
                    self.consume_token();
                    let factor = self.parse_factor()?;
                    operations.push((MultiplicativeOperator::Multiplication, factor));
                }
                Some(Token::Divide) => {
                    self.consume_token();
                    let factor = self.parse_factor()?;
                    operations.push((MultiplicativeOperator::Division, factor));
                }
                _ => break,
            }
        }

        Ok(Term::new(factor, operations))
    }

    fn parse_factor(&mut self) -> Result<Factor, Error> {
        match self.peek_token() {
            Some(Token::Variable { identifier }) => {
                self.consume_token();
                let variable = Variable::new(identifier);
                Ok(Factor::Variable(variable))
            }
            Some(Token::NumberLiteral(value)) => {
                self.consume_token();
                Ok(Factor::NumberLiteral(NumberLiteral::new(value)))
            }
            _ => {
                self.expect(Token::OpeningParenthesis)?;
                let expression = Box::new(self.parse_expression()?);
                self.expect(Token::ClosingParenthesis)?;
                Ok(Factor::Expression(expression))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_hello_world_returns_ast() {
        let tokens = VecDeque::from([
            Token::NumberLiteral(10),
            Token::Print,
            Token::StringLiteral {
                value: b"Hello, World!".to_vec(),
            },
        ]);
        let expected = Line::new(
            Some(10),
            Statement::Print {
                expression_list: vec![ExpressionListElement::StringLiteral(StringLiteral::new(
                    b"Hello, World!".to_vec(),
                ))],
            },
        );
        let mut parser = Parser::new(tokens);

        let actual = parser.parse_line();

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn parse_line_terms_returns_ast() {
        let tokens = VecDeque::from([
            Token::NumberLiteral(10),
            Token::Print,
            Token::NumberLiteral(2),
            Token::Plus,
            Token::NumberLiteral(3),
        ]);

        let expected = Line::new(
            Some(10),
            Statement::Print {
                expression_list: vec![ExpressionListElement::Expression(Expression::new(
                    None,
                    Term::new(Factor::NumberLiteral(NumberLiteral::new(2)), vec![]),
                    vec![(
                        AdditiveOperator::Addition,
                        Term::new(Factor::NumberLiteral(NumberLiteral::new(3)), vec![]),
                    )],
                ))],
            },
        );
        let mut parser = Parser::new(tokens);

        let actual = parser.parse_line();

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn parse_line_with_unary_operator_returns_ast() {
        let tokens = VecDeque::from([
            Token::NumberLiteral(10),
            Token::Print,
            Token::Minus,
            Token::NumberLiteral(2),
            Token::Plus,
            Token::NumberLiteral(3),
        ]);
        let expected = Line::new(
            Some(10),
            Statement::Print {
                expression_list: vec![ExpressionListElement::Expression(Expression::new(
                    Some(AdditiveOperator::Subtraction),
                    Term::new(Factor::NumberLiteral(NumberLiteral::new(2)), vec![]),
                    vec![(
                        AdditiveOperator::Addition,
                        Term::new(Factor::NumberLiteral(NumberLiteral::new(3)), vec![]),
                    )],
                ))],
            },
        );
        let mut parser = Parser::new(tokens);

        let actual = parser.parse_line();

        assert_eq!(Ok(expected), actual);
    }
}
