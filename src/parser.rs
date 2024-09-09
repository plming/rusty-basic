use crate::ast::{self, Program};
use crate::lexer::{Error as LexerError, Lexer};
use crate::token::Token;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken { expected: Token, found: Token },
    LexerError(LexerError),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            current_token: Token::EndOfFile,
        }
    }

    fn consume_token(&mut self) -> Result<(), Error> {
        match self.lexer.next_token() {
            Ok(token) => {
                self.current_token = token;
                Ok(())
            }
            Err(err) => Err(Error::LexerError(err)),
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

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program::new();

        loop {
            self.consume_token()?;

            if self.current_token == Token::EndOfFile {
                break;
            }

            let statement = self.parse_statement()?;
            program.add_statement(statement);
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        let statement = match self.current_token {
            Token::Print => {
                self.consume_token()?;
                let expression_list = self.parse_expression_list()?;
                ast::Statement::Print(expression_list)
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
                ast::Statement::Goto(expression)
            }
            _ => todo!(),
        };

        Ok(statement)
    }

    fn parse_expression_list(&mut self) -> Result<ast::ExpressionList, Error> {
        let mut expression_list = ast::ExpressionList::new();
        loop {
            match &self.current_token {
                Token::StringLiteral { value } => {
                    let element = ast::ExpressionListElement::String(value.clone());
                    expression_list.push(element);
                    self.consume_token()?;
                }
                _ => {
                    let expression = self.parse_expression()?;
                    let element = ast::ExpressionListElement::Expression(expression);
                    expression_list.push(element);
                }
            }

            if self.current_token == Token::Comma {
                self.consume_token()?;
            } else {
                break;
            }
        }

        Ok(expression_list)
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Error> {
        let mut expression = ast::Expression {
            terms: Vec::new(),
            operators: Vec::new(),
        };

        match self.current_token {
            Token::Plus => {
                self.consume_token()?;
                expression.operators.push(ast::TermOperator::Add);
            }
            Token::Minus => {
                self.consume_token()?;
                expression.operators.push(ast::TermOperator::Subtract);
            }
            _ => {
                // No operator, so we assume it's a positive number
                expression.operators.push(ast::TermOperator::Add);
            }
        }

        expression.terms.push(self.parse_term()?);

        loop {
            match self.current_token {
                Token::Plus => {
                    self.consume_token()?;
                    expression.operators.push(ast::TermOperator::Add);
                    let term = self.parse_term()?;
                    expression.terms.push(term);
                }
                Token::Minus => {
                    self.consume_token()?;
                    expression.operators.push(ast::TermOperator::Subtract);
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
                    term.operators.push(ast::FactorOperator::Multiply);
                    let factor = self.parse_factor()?;
                    term.factors.push(factor);
                }
                Token::Divide => {
                    self.consume_token()?;
                    term.operators.push(ast::FactorOperator::Divide);
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
                Ok(ast::Factor::Variable(ast::Variable::new(identifier)))
            }
            Token::NumberLiteral { value } => {
                self.consume_token()?;
                Ok(ast::Factor::Number(value))
            }
            _ => {
                self.expect(Token::OpeningParenthesis)?;
                let expression = self.parse_expression()?;
                self.expect(Token::ClosingParenthesis)?;
                Ok(ast::Factor::Expression(Box::new(expression)))
            }
        }
    }
}
