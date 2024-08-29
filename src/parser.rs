use crate::lexer::{Error as LexerError, Lexer};
use crate::{ast, token};

#[derive(Debug)]
pub struct Error {
    expected: token::Token,
    found: token::Token,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Result<token::Token, LexerError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            current_token: Err(LexerError::EndOfCode),
        }
    }

    fn expect(&mut self, token: token::Token) -> Result<(), Error> {
        if let Ok(found) = &self.current_token {
            if *found == token {
                self.consume_token();
                return Ok(());
            }
        }

        Err(Error {
            expected: token,
            found: todo!(),
        })
    }

    pub fn parse_program(&mut self) -> Result<Vec<ast::Statement>, Error> {
        let mut program = Vec::new();

        loop {
            self.consume_token();

            if let Err(LexerError::EndOfCode) = self.current_token {
                break;
            }

            let statement = self.parse_statement()?;
            program.push(statement);
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        let statement = match self.current_token {
            Ok(token::Token::Print) => {
                self.consume_token();
                let expression_list = self.parse_expression_list()?;
                ast::Statement::Print(expression_list)
            }
            Ok(token::Token::If) => {
                self.consume_token();
                let left = self.parse_expression()?;
                let operator = match self.current_token {
                    Ok(token::Token::Equal) => ast::RelationalOperator::Equal,
                    Ok(token::Token::NotEqual) => ast::RelationalOperator::NotEqual,
                    Ok(token::Token::LessThan) => ast::RelationalOperator::LessThan,
                    Ok(token::Token::LessThanOrEqual) => ast::RelationalOperator::LessThanOrEqual,
                    Ok(token::Token::GreaterThan) => ast::RelationalOperator::GreaterThan,
                    Ok(token::Token::GreaterThanOrEqual) => {
                        ast::RelationalOperator::GreaterThanOrEqual
                    }
                    _ => Err(Error {
                        expected: todo!(),
                        found: todo!(),
                    })?,
                };
                let right = self.parse_expression()?;
                self.expect(token::Token::Then)?;
                let then = Box::new(self.parse_statement()?);
                ast::Statement::If {
                    left,
                    operator,
                    right,
                    then,
                }
            }
            Ok(token::Token::Goto) => {
                self.consume_token();
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
                Ok(token::Token::StringLiteral(s)) => {
                    let element = ast::ExpressionListElement::String(s.to_vec());
                    expression_list.push(element);
                    self.consume_token();
                }
                _ => {
                    let expression = self.parse_expression()?;
                    let element = ast::ExpressionListElement::Expression(expression);
                    expression_list.push(element);
                }
            }

            if let Ok(token::Token::Comma) = self.current_token {
                self.consume_token();
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
            Ok(token::Token::Plus) => {
                self.consume_token();
                expression.operators.push(ast::TermOperator::Add);
            }
            Ok(token::Token::Minus) => {
                self.consume_token();
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
                Ok(token::Token::Plus) => {
                    self.consume_token();
                    expression.operators.push(ast::TermOperator::Add);
                    let term = self.parse_term()?;
                    expression.terms.push(term);
                }
                Ok(token::Token::Minus) => {
                    self.consume_token();
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
                Ok(token::Token::Multiply) => {
                    self.consume_token();
                    term.operators.push(ast::FactorOperator::Multiply);
                    let factor = self.parse_factor()?;
                    term.factors.push(factor);
                }
                Ok(token::Token::Divide) => {
                    self.consume_token();
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
            Ok(token::Token::Variable { identifier }) => {
                self.consume_token();
                Ok(ast::Factor::Variable(ast::Variable::new(identifier)))
            }
            Ok(token::Token::NumberLiteral(literal)) => {
                self.consume_token();
                Ok(ast::Factor::Number(literal))
            }
            _ => {
                self.expect(token::Token::OpeningParenthesis)?;
                let expression = self.parse_expression()?;
                self.expect(token::Token::ClosingParenthesis)?;
                Ok(ast::Factor::Expression(Box::new(expression)))
            }
        }
    }

    fn consume_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}
