use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Found an invalid character like b'@', b'$'
    InvalidCharacter,
    /// Reached the end of the code
    EndOfCode,
    /// Lexed identifier is not keyword or variable
    InvalidIdentifier,
    /// Invalid string literal like "Hello, World!
    InvalidStringLiteral,
}

pub struct Lexer<'a> {
    /// the code to lex
    code: &'a [u8],
    /// the offset of the character not yet consumed
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a [u8]) -> Lexer {
        Lexer { code, position: 0 }
    }

    fn peek_next_char(&self) -> Option<u8> {
        self.code.get(self.position).copied()
    }

    fn read_next_char(&mut self) -> Option<u8> {
        let next_char = self.peek_next_char();
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

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespaces();

        match self.peek_next_char() {
            Some(b',') => {
                self.read_next_char();
                Ok(Token::Comma)
            }
            Some(b'(') => {
                self.read_next_char();
                Ok(Token::OpeningParenthesis)
            }
            Some(b')') => {
                self.read_next_char();
                Ok(Token::ClosingParenthesis)
            }
            Some(b'=') => {
                self.read_next_char();
                Ok(Token::Equal)
            }
            Some(b'<') => {
                self.read_next_char();
                match self.peek_next_char() {
                    Some(b'=') => {
                        self.read_next_char();
                        Ok(Token::LessThanOrEqual)
                    }
                    Some(b'>') => {
                        self.read_next_char();
                        Ok(Token::NotEqual)
                    }
                    _ => Ok(Token::LessThan),
                }
            }
            Some(b'>') => {
                self.read_next_char();
                match self.peek_next_char() {
                    Some(b'=') => {
                        self.read_next_char();
                        Ok(Token::GreaterThanOrEqual)
                    }
                    Some(b'<') => {
                        self.read_next_char();
                        Ok(Token::NotEqual)
                    }
                    _ => Ok(Token::GreaterThan),
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
                let mut value: i16 = 0;
                while let Some(digit @ b'0'..=b'9') = self.peek_next_char() {
                    value *= 10;
                    value += (digit - b'0') as i16;
                    self.read_next_char();
                }
                Ok(Token::NumberLiteral { value })
            }
            Some(ch) if ch.is_ascii_alphabetic() => {
                let mut identifier: Vec<u8> = Vec::new();

                while let Some(ch) = self.peek_next_char() {
                    if !ch.is_ascii_alphanumeric() {
                        break;
                    }

                    identifier.push(ch.to_ascii_uppercase());
                    self.read_next_char();
                }

                debug_assert_eq!(identifier, identifier.to_ascii_uppercase());

                // handle variable identifier
                if identifier.len() == 1 && identifier[0].is_ascii_alphabetic() {
                    return Ok(Token::Variable {
                        identifier: identifier[0],
                    });
                }

                match identifier.as_slice() {
                    b"PRINT" => Ok(Token::Print),
                    b"IF" => Ok(Token::If),
                    b"THEN" => Ok(Token::Then),
                    b"GOTO" => Ok(Token::Goto),
                    b"INPUT" => Ok(Token::Input),
                    b"LET" => Ok(Token::Let),
                    b"GOSUB" => Ok(Token::GoSub),
                    b"RETURN" => Ok(Token::Return),
                    b"CLEAR" => Ok(Token::Clear),
                    b"LIST" => Ok(Token::List),
                    b"RUN" => Ok(Token::Run),
                    b"END" => Ok(Token::End),
                    _ => Err(Error::InvalidIdentifier),
                }
            }
            Some(b'"') => {
                let mut value: Vec<u8> = Vec::new();
                self.read_next_char();

                while let Some(ch) = self.read_next_char() {
                    if ch == b'"' {
                        self.read_next_char();
                        return Ok(Token::StringLiteral { value });
                    }

                    value.push(ch);
                }

                Err(Error::InvalidStringLiteral)
            }
            Some(_) => Err(Error::InvalidCharacter),
            None => Err(Error::EndOfCode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_hello_world_returns_tokens() {
        let code = b"PRINT \"Hello, World!\"";
        let mut lexer = Lexer::new(code);
        let expected = vec![
            Token::Print,
            Token::StringLiteral {
                value: b"Hello, World!".to_vec(),
            },
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }

    #[test]
    fn next_token_expression_returns_tokens() {
        let expression = b"1 + 2 * 3 / 4 - 5";
        let mut lexer = Lexer::new(expression);
        let expected = vec![
            Token::NumberLiteral { value: 1 },
            Token::Plus,
            Token::NumberLiteral { value: 2 },
            Token::Multiply,
            Token::NumberLiteral { value: 3 },
            Token::Divide,
            Token::NumberLiteral { value: 4 },
            Token::Minus,
            Token::NumberLiteral { value: 5 },
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }

    #[test]
    fn next_token_keywords_returns_tokens() {
        let code = b"PRINT IF THEN GOTO INPUT LET GOSUB RETURN CLEAR LIST RUN END";
        let mut lexer = Lexer::new(code);
        let expected = vec![
            Token::Print,
            Token::If,
            Token::Then,
            Token::Goto,
            Token::Input,
            Token::Let,
            Token::GoSub,
            Token::Return,
            Token::Clear,
            Token::List,
            Token::Run,
            Token::End,
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }

    #[test]
    fn next_token_variable_returns_token() {
        let code = b"IF A < B THEN PRINT Z";
        let mut lexer = Lexer::new(code);
        let expected = vec![
            Token::If,
            Token::Variable { identifier: b'A' },
            Token::LessThan,
            Token::Variable { identifier: b'B' },
            Token::Then,
            Token::Print,
            Token::Variable { identifier: b'Z' },
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }

    #[test]
    fn next_token_lowercase_variable_returns_token() {
        let code = b"IF a < b THEN PRINT z";
        let mut lexer = Lexer::new(code);
        let expected = vec![
            Token::If,
            Token::Variable { identifier: b'A' },
            Token::LessThan,
            Token::Variable { identifier: b'B' },
            Token::Then,
            Token::Print,
            Token::Variable { identifier: b'Z' },
        ];

        for token in expected {
            assert_eq!(lexer.next_token().unwrap(), token);
        }
    }

    #[test]
    fn next_token_unknown_identifier_returns_error() {
        let invalid_code = b"PRINT HELLO";
        let mut lexer = Lexer::new(invalid_code);

        assert_eq!(lexer.next_token().unwrap(), Token::Print);
        assert_eq!(lexer.next_token(), Err(Error::InvalidIdentifier));
    }

    #[test]
    fn next_token_non_terminated_string_returns_error() {
        let invalid_code = b"PRINT \"Hello, World!";
        let mut lexer = Lexer::new(invalid_code);

        assert_eq!(lexer.next_token().unwrap(), Token::Print);
        assert_eq!(lexer.next_token(), Err(Error::InvalidStringLiteral));
    }
}
