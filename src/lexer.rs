mod error;
mod token;

pub use error::Error;
pub use token::Token;

type Result<Token> = std::result::Result<Token, Error>;

pub struct Lexer<'a> {
    code: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Lexer { code, position: 0 }
    }

    fn peek_next_char(&self) -> Option<&u8> {
        self.code.get(self.position)
    }

    fn read_next_char(&mut self) -> Option<&u8> {
        let next_char = self.code.get(self.position);
        if next_char.is_some() {
            self.position += 1;
        }
        next_char
    }

    fn skip_whitespaces(&mut self) {
        loop {
            if let Some(ch) = self.peek_next_char() {
                if ch.is_ascii_whitespace() {
                    self.read_next_char();
                    continue;
                }
            }

            break;
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespaces();

        match self.peek_next_char() {
            Some(b'=') => {
                self.read_next_char();
                Ok(Token::Equal)
            }
            Some(b'<') => {
                self.read_next_char();
                if let Some(b'=') = self.peek_next_char() {
                    self.read_next_char();
                    Ok(Token::LessThanOrEqual)
                } else if let Some(b'>') = self.peek_next_char() {
                    self.read_next_char();
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::LessThan)
                }
            }
            Some(b'>') => {
                self.read_next_char();
                if let Some(b'=') = self.peek_next_char() {
                    self.read_next_char();
                    Ok(Token::GreaterThanOrEqual)
                } else if let Some(b'<') = self.peek_next_char() {
                    self.read_next_char();
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            Some(b'+') => {
                self.read_next_char();
                Ok(Token::Plus)
            }
            Some(b'-') => {
                self.read_next_char();
                Ok(Token::Minus)
            }
            Some(b'*') => {
                self.read_next_char();
                Ok(Token::Multiply)
            }
            Some(b'/') => {
                self.read_next_char();
                Ok(Token::Divide)
            }
            Some(b'0'..=b'9') => {
                let mut number: i16 = 0;
                while let Some(digit @ b'0'..=b'9') = self.peek_next_char() {
                    number *= 10;
                    number += (digit - b'0') as i16;
                    self.read_next_char();
                }
                Ok(Token::NumberLiteral(number))
            }
            Some(b'A'..=b'Z') | Some(b'a'..=b'z') => {
                let mut identifier = String::new();
                while let Some(ch @ b'A'..=b'Z') | Some(ch @ b'a'..=b'z') = self.peek_next_char() {
                    identifier.push(*ch as char);
                    self.read_next_char();
                }

                match identifier.as_str() {
                    "PRINT" => Ok(Token::Print),
                    "IF" => Ok(Token::If),
                    "THEN" => Ok(Token::Then),
                    "GOTO" => Ok(Token::Goto),
                    "INPUT" => Ok(Token::Input),
                    "LET" => Ok(Token::Let),
                    "GOSUB" => Ok(Token::Gosub),
                    "RETURN" => Ok(Token::Return),
                    "CLEAR" => Ok(Token::Clear),
                    "LIST" => Ok(Token::List),
                    "RUN" => Ok(Token::Run),
                    "END" => Ok(Token::End),
                    _ => Err(Error::InvalidIdentifier),
                }
            }
            Some(b'\"') => todo!(),
            Some(_) => Err(Error::UnexpectedCharacter),
            None => Err(Error::UnexpectedEndOfInput),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_expression() {
        let code = b"PRINT 1 + 2";
        let mut lexer = Lexer::new(code);
        let expected = vec![
            Token::Print,
            Token::NumberLiteral(1),
            Token::Plus,
            Token::NumberLiteral(2),
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }

    #[test]
    fn lex_complex_expression() {
        let code = b"IF 1 < 2 THEN GOTO 10";
        let mut lexer = Lexer::new(code);
        let expected = vec![
            Token::If,
            Token::NumberLiteral(1),
            Token::LessThan,
            Token::NumberLiteral(2),
            Token::Then,
            Token::Goto,
            Token::NumberLiteral(10),
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }
}