use crate::token::Token;

#[derive(Debug)]
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
